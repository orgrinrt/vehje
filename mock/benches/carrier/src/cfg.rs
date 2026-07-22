//! CFG interpreter throughput: control-flow-heavy per-instruction cost.
//!
//! Every other interp bench here is straight-line (a linear node stream). The
//! vehje runtime also has a CFG-of-blocks interpreter tier, whose cost profile
//! differs: a large fraction of executed steps are block terminators (branches,
//! loop back-edges), which are the hard-to-predict control transfers. This is
//! the audit's fifth standalone scaling bench, ported onto the carrier.
//!
//! A small register VM: blocks of register arithmetic ending in a terminator
//! (jump, conditional branch, or return). The workload is a nested-loop numeric
//! kernel, so a known fraction of executed steps are the loop-head branches. The
//! interpreted result is cross-validated against a direct Rust computation of the
//! same kernel (the oracle), so the measured per-instruction cost is honest.

/// Register-op opcodes: `regs[dst] = regs[a] <op> regs[b]`, except SET which
/// loads the immediate into `dst`.
pub mod op {
    pub const SET: u8 = 0; // regs[dst] = imm
    pub const ADD: u8 = 1;
    pub const SUB: u8 = 2;
    pub const MUL: u8 = 3;
}

#[derive(Clone, Copy)]
pub struct Instr {
    pub op: u8,
    pub dst: u8,
    pub a: u8,
    pub b: u8,
    pub imm: u64,
}

/// Block terminator: the control transfer at the end of a block.
#[derive(Clone, Copy)]
pub enum Term {
    Jmp(u32),
    /// Branch to `nz` if `regs[reg] != 0`, else to `z`.
    BrNz(u8, u32, u32),
    Ret(u8),
}

pub struct Block {
    pub instrs: Vec<Instr>,
    pub term: Term,
}

const NREG: usize = 8;

/// Interpret the CFG from block 0 with `seed` in r0. Returns
/// (result, instrs_executed, terminators_executed). `cap` bounds total steps as
/// a safety net; a well-formed kernel returns before hitting it.
pub fn interp(blocks: &[Block], seed: u64, cap: u64) -> (u64, u64, u64) {
    let mut regs = [0u64; NREG];
    regs[0] = seed;
    let mut pc = 0u32;
    let mut ninstr = 0u64;
    let mut nterm = 0u64;
    loop {
        let b = &blocks[pc as usize];
        for ins in &b.instrs {
            let v = match ins.op {
                op::SET => ins.imm,
                op::ADD => regs[ins.a as usize].wrapping_add(regs[ins.b as usize]),
                op::SUB => regs[ins.a as usize].wrapping_sub(regs[ins.b as usize]),
                _ => regs[ins.a as usize].wrapping_mul(regs[ins.b as usize]),
            };
            regs[ins.dst as usize] = v;
            ninstr += 1;
        }
        nterm += 1;
        if ninstr + nterm > cap {
            return (regs[0], ninstr, nterm); // safety
        }
        match b.term {
            Term::Jmp(t) => pc = t,
            Term::BrNz(r, nz, z) => pc = if regs[r as usize] != 0 { nz } else { z },
            Term::Ret(r) => return (regs[r as usize], ninstr, nterm),
        }
    }
}

/// Build the nested-loop kernel: `acc += r0 * inner_counter` over
/// `outer * inner` iterations, plus a decrement per iteration. The loop-head
/// branches are the control-flow steps.
/// Registers: r0 = seed (input), r1 = outer counter, r2 = acc, r3 = inner
/// counter, r4 = 1 (decrement), r5 = inner_bound, r6 = scratch.
pub fn build_nested_loop(outer: u64, inner: u64) -> Vec<Block> {
    let i = |op, dst, a, b, imm| Instr { op, dst, a, b, imm };
    vec![
        // block 0: entry. set constants.
        Block {
            instrs: vec![
                i(op::SET, 1, 0, 0, outer), // r1 = outer
                i(op::SET, 2, 0, 0, 0),     // r2 = 0 (acc)
                i(op::SET, 4, 0, 0, 1),     // r4 = 1
                i(op::SET, 5, 0, 0, inner), // r5 = inner bound
            ],
            term: Term::Jmp(1),
        },
        // block 1: outer loop head. if r1 == 0 -> exit(4), else r3 = inner; -> 2
        Block {
            instrs: vec![i(op::SET, 3, 0, 0, inner)], // r3 = inner (reset per outer iter)
            term: Term::BrNz(1, 2, 4),
        },
        // block 2: inner loop head. if r3 == 0 -> outer tail(3), else -> inner body is folded here + loop
        Block {
            instrs: vec![
                i(op::MUL, 6, 0, 3, 0), // r6 = r0 * r3
                i(op::ADD, 2, 2, 6, 0), // r2 += r6
                i(op::SUB, 3, 3, 4, 0), // r3 -= 1
            ],
            term: Term::BrNz(3, 2, 3), // if r3 != 0 loop back to 2, else -> 3
        },
        // block 3: outer body tail. r1 -= 1; -> 1
        Block {
            instrs: vec![i(op::SUB, 1, 1, 4, 0)],
            term: Term::Jmp(1),
        },
        // block 4: exit. ret r2.
        Block { instrs: vec![], term: Term::Ret(2) },
    ]
}

/// The oracle: compute the same kernel result directly, for cross-validation.
pub fn oracle_nested_loop(seed: u64, outer: u64, inner: u64) -> u64 {
    let mut acc = 0u64;
    let mut o = outer;
    while o != 0 {
        let mut i = inner;
        while i != 0 {
            acc = acc.wrapping_add(seed.wrapping_mul(i));
            i -= 1;
        }
        o -= 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfg_matches_oracle() {
        for &(outer, inner) in &[(3u64, 5u64), (10, 10), (7, 100), (50, 40)] {
            for seed in [1u64, 3, 42, 1000] {
                let blocks = build_nested_loop(outer, inner);
                let (r, _ni, _nt) = interp(&blocks, seed, u64::MAX);
                assert_eq!(r, oracle_nested_loop(seed, outer, inner), "cfg vs oracle at ({outer},{inner}) seed {seed}");
            }
        }
    }

    #[test]
    fn control_flow_fraction_is_high() {
        let blocks = build_nested_loop(20, 50);
        let (_r, ni, nt) = interp(&blocks, 7, u64::MAX);
        let frac = nt as f64 / (ni + nt) as f64;
        assert!(frac > 0.25, "control-flow fraction {frac} too low to be a CFG-heavy workload");
    }
}

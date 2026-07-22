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
    pub const AND: u8 = 4;
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

pub const NREG: usize = 8;

/// Interpret the CFG from block 0 with `seed` in r0. Returns
/// (result, instrs_executed, terminators_executed). `cap` bounds total steps as
/// a safety net; a well-formed kernel returns before hitting it.
///
/// Every register read and write goes through the shared `access::rload` /
/// `access::rstore` (unchecked, sound: `build_*` only emit register indices
/// below `NREG`), so this switch cell, the fntable cell, and the threaded cell
/// share identical operand access and the CFG dispatch axis varies dispatch
/// alone (no bounds-check-vs-no-bounds-check confound between the shapes).
pub fn interp(blocks: &[Block], seed: u64, cap: u64) -> (u64, u64, u64) {
    use crate::access::{rload, rstore};
    let mut regs = [0u64; NREG];
    let rp = regs.as_mut_ptr();
    unsafe { rstore(rp, 0, seed) };
    let mut pc = 0u32;
    let mut ninstr = 0u64;
    let mut nterm = 0u64;
    loop {
        let b = &blocks[pc as usize];
        for ins in &b.instrs {
            let v = unsafe {
                match ins.op {
                    op::SET => ins.imm,
                    op::ADD => rload(rp, ins.a as u32).wrapping_add(rload(rp, ins.b as u32)),
                    op::SUB => rload(rp, ins.a as u32).wrapping_sub(rload(rp, ins.b as u32)),
                    op::AND => rload(rp, ins.a as u32) & rload(rp, ins.b as u32),
                    _ => rload(rp, ins.a as u32).wrapping_mul(rload(rp, ins.b as u32)),
                }
            };
            unsafe { rstore(rp, ins.dst as usize, v) };
            ninstr += 1;
        }
        nterm += 1;
        if ninstr + nterm > cap {
            return (unsafe { rload(rp, 0) }, ninstr, nterm); // safety
        }
        match b.term {
            Term::Jmp(t) => pc = t,
            Term::BrNz(r, nz, z) => pc = if unsafe { rload(rp, r as u32) } != 0 { nz } else { z },
            Term::Ret(r) => return (unsafe { rload(rp, r as u32) }, ninstr, nterm),
        }
    }
}

/// Per-instruction op function for the function-pointer-table dispatch over the
/// CFG: `(ins, regs) -> value`. Order matches `op` (SET..AND).
///
/// Each op function does its own `rload`s, exactly like the straight-line
/// fntable's `f_*` functions (`interp.rs`). The earlier signature was
/// `fn(imm, regs[a], regs[b])`, which forced the fntable call site to load
/// `regs[a]` and `regs[b]` eagerly for EVERY instruction, including SET, which
/// ignores both. Because the call is through a runtime-resolved pointer the
/// compiler cannot prove the callee discards them, so those two loads executed
/// unconditionally on every SET, a cost the switch and threaded cells do not pay
/// (disassembly confirmed: two dead `ldr` before the `blr`). Passing `ins` and
/// `regs` and letting each op load only what it needs removes the asymmetry.
type CFn = fn(&Instr, *const u64) -> u64;
fn c_set(ins: &Instr, _rp: *const u64) -> u64 {
    ins.imm
}
fn c_add(ins: &Instr, rp: *const u64) -> u64 {
    unsafe { crate::access::rload(rp, ins.a as u32).wrapping_add(crate::access::rload(rp, ins.b as u32)) }
}
fn c_sub(ins: &Instr, rp: *const u64) -> u64 {
    unsafe { crate::access::rload(rp, ins.a as u32).wrapping_sub(crate::access::rload(rp, ins.b as u32)) }
}
fn c_mul(ins: &Instr, rp: *const u64) -> u64 {
    unsafe { crate::access::rload(rp, ins.a as u32).wrapping_mul(crate::access::rload(rp, ins.b as u32)) }
}
fn c_and(ins: &Instr, rp: *const u64) -> u64 {
    unsafe { crate::access::rload(rp, ins.a as u32) & crate::access::rload(rp, ins.b as u32) }
}
static CTABLE: [CFn; 5] = [c_set, c_add, c_sub, c_mul, c_and];

/// Function-pointer-table dispatch over the CFG: identical control flow to
/// [`interp`], the per-instruction op dispatched through a table instead of a
/// `match`. Measures dispatch shape under real control flow (loops, branches),
/// where a straight-line DAG cannot.
pub fn interp_fntable(blocks: &[Block], seed: u64, cap: u64) -> (u64, u64, u64) {
    use crate::access::{rload, rstore};
    let mut regs = [0u64; NREG];
    let rp = regs.as_mut_ptr();
    unsafe { rstore(rp, 0, seed) };
    let mut pc = 0u32;
    let mut ninstr = 0u64;
    let mut nterm = 0u64;
    loop {
        let b = &blocks[pc as usize];
        for ins in &b.instrs {
            // each op function loads only the operands it needs (see `CFn`), so
            // SET pays no register loads here, matching switch and threaded.
            let v = CTABLE[ins.op as usize](ins, rp);
            unsafe { rstore(rp, ins.dst as usize, v) };
            ninstr += 1;
        }
        nterm += 1;
        if ninstr + nterm > cap {
            return (unsafe { rload(rp, 0) }, ninstr, nterm);
        }
        match b.term {
            Term::Jmp(t) => pc = t,
            Term::BrNz(r, nz, z) => pc = if unsafe { rload(rp, r as u32) } != 0 { nz } else { z },
            Term::Ret(r) => return (unsafe { rload(rp, r as u32) }, ninstr, nterm),
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

const LCG_A: u64 = 6364136223846793005;
const LCG_C: u64 = 1442695040888963407;

/// Build a loop with one hot conditional branch per iteration whose predictability
/// is controlled. Both variants do the identical work per iteration (one MUL, one
/// ADD, one AND, then a branch on a parity bit); only the branch SOURCE differs:
/// `predictable` branches on the loop counter's parity (a learnable ABAB
/// alternation), `!predictable` on a per-iteration LCG value's parity (~50/50,
/// unlearnable). The gap between the two is the branch-misprediction cost per
/// terminator, which the predictable nested-loop kernel could not show.
/// Registers: r0 = advancing value, r1 = counter, r2 = acc, r3 = parity, r4 = 1,
/// r5 = LCG_A, r6 = LCG_C, r7 = dead scratch (to match instruction counts).
pub fn build_branchy(n: u64, predictable: bool) -> Vec<Block> {
    let i = |op, dst, a, b, imm| Instr { op, dst, a, b, imm };
    let advance = if predictable {
        // dead MUL (matches the unpredictable variant's MUL op) then r0 += 1.
        vec![i(op::MUL, 7, 5, 6, 0), i(op::ADD, 0, 0, 4, 0)]
    } else {
        // r0 = r0 * A + C (an LCG step; its parity is unpredictable).
        vec![i(op::MUL, 0, 0, 5, 0), i(op::ADD, 0, 0, 6, 0)]
    };
    let mut b2 = advance;
    b2.push(i(op::AND, 3, 0, 4, 0)); // r3 = r0 & 1 (parity)
    vec![
        // block 0: entry.
        Block {
            instrs: vec![
                i(op::SET, 1, 0, 0, n),
                i(op::SET, 2, 0, 0, 0),
                i(op::SET, 4, 0, 0, 1),
                i(op::SET, 5, 0, 0, LCG_A),
                i(op::SET, 6, 0, 0, LCG_C),
            ],
            term: Term::Jmp(1),
        },
        // block 1: loop head. if r1 == 0 -> exit(6), else -> 2.
        Block { instrs: vec![], term: Term::BrNz(1, 2, 6) },
        // block 2: advance + compute parity; branch on it.
        Block { instrs: b2, term: Term::BrNz(3, 3, 4) },
        // block 3: path A. r2 += r1. -> 5.
        Block { instrs: vec![i(op::ADD, 2, 2, 1, 0)], term: Term::Jmp(5) },
        // block 4: path B. r2 -= r1. -> 5.
        Block { instrs: vec![i(op::SUB, 2, 2, 1, 0)], term: Term::Jmp(5) },
        // block 5: tail. r1 -= 1. -> 1.
        Block { instrs: vec![i(op::SUB, 1, 1, 4, 0)], term: Term::Jmp(1) },
        // block 6: exit. ret r2.
        Block { instrs: vec![], term: Term::Ret(2) },
    ]
}

/// Oracle for [`build_branchy`], for cross-validation.
pub fn oracle_branchy(seed: u64, n: u64, predictable: bool) -> u64 {
    let mut r0 = seed;
    let mut acc = 0u64;
    let mut c = n;
    while c != 0 {
        if predictable {
            r0 = r0.wrapping_add(1);
        } else {
            r0 = r0.wrapping_mul(LCG_A).wrapping_add(LCG_C);
        }
        if r0 & 1 != 0 {
            acc = acc.wrapping_add(c);
        } else {
            acc = acc.wrapping_sub(c);
        }
        c -= 1;
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
    fn branchy_matches_oracle() {
        for pred in [true, false] {
            for &n in &[5u64, 50, 1000] {
                for seed in [1u64, 42, 999] {
                    let blocks = build_branchy(n, pred);
                    let (r, _, _) = interp(&blocks, seed, u64::MAX);
                    assert_eq!(r, oracle_branchy(seed, n, pred), "branchy vs oracle pred={pred} n={n} seed={seed}");
                }
            }
        }
    }

    #[test]
    fn fntable_matches_switch_cfg() {
        // the function-pointer-table CFG interpreter agrees with the switch CFG
        // interpreter on both kernels, so the control-flow dispatch axis measures
        // dispatch and nothing else.
        for &(outer, inner) in &[(3u64, 5u64), (10, 10), (7, 100)] {
            for seed in [1u64, 3, 42] {
                let blocks = build_nested_loop(outer, inner);
                assert_eq!(
                    interp(&blocks, seed, u64::MAX),
                    interp_fntable(&blocks, seed, u64::MAX),
                    "cfg fntable vs switch at ({outer},{inner}) seed {seed}"
                );
            }
        }
        for pred in [true, false] {
            for &n in &[5u64, 50, 1000] {
                for seed in [1u64, 42, 999] {
                    let blocks = build_branchy(n, pred);
                    assert_eq!(
                        interp(&blocks, seed, u64::MAX),
                        interp_fntable(&blocks, seed, u64::MAX),
                        "cfg fntable vs switch branchy pred={pred} n={n} seed={seed}"
                    );
                }
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

    #[cfg(feature = "threaded")]
    #[test]
    fn threaded_matches_switch_cfg() {
        // The preserve-none context-threaded CFG interpreter agrees with the
        // switch CFG interpreter (the full triple: result, instr count, term
        // count) on both kernels, so the threaded control-flow dispatch cell
        // measures dispatch and nothing else. The identical instr/term counts
        // also prove the flattened stream walks the same path as the block VM
        // (same loop back-edges taken, same branches).
        for &(outer, inner) in &[(3u64, 5u64), (10, 10), (7, 100), (20, 40)] {
            for seed in [1u64, 3, 42, 1000] {
                let blocks = build_nested_loop(outer, inner);
                assert_eq!(
                    interp(&blocks, seed, u64::MAX),
                    crate::cfg_threaded::interp_threaded(&blocks, seed, u64::MAX),
                    "cfg threaded vs switch at ({outer},{inner}) seed {seed}"
                );
            }
        }
        for pred in [true, false] {
            for &n in &[5u64, 50, 1000] {
                for seed in [1u64, 42, 999] {
                    let blocks = build_branchy(n, pred);
                    assert_eq!(
                        interp(&blocks, seed, u64::MAX),
                        crate::cfg_threaded::interp_threaded(&blocks, seed, u64::MAX),
                        "cfg threaded vs switch branchy pred={pred} n={n} seed={seed}"
                    );
                }
            }
        }
    }
}

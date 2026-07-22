//! Trace / superblock dispatch over the control-flow VM.
//!
//! A trace is a hot linear path through the CFG, recorded once and then run as a
//! specialized straight-line superblock guarded by the single branch that could
//! leave it. Where the base CFG interpreter pays a block-array index and a
//! terminator `match` on every inner-loop iteration, the trace runs the hot
//! body as a contiguous instruction slice in a tight loop with one guard check,
//! and only returns to the base interpreter on a side exit (the guard failing).
//! It is the pre-JIT half of a tracing runtime: the linearization win without
//! yet specializing the ops to machine code (that is the copy-and-patch cell).
//!
//! Scope, stated honestly. This builds the clean representative case: a self-loop
//! trace, where the hot block branches back to itself (the inner loop of the
//! nested-loop kernel). Multi-block traces with an internal data-dependent branch
//! (the branchy kernel) need multiple guards and a partial-path record; that is a
//! labelled refinement, not built here, and it is exactly the case where a trace
//! degrades (an unpredictable internal branch side-exits the trace half the time).
//! The nested-loop self-loop is the case a trace is meant to win, so it is the one
//! measured.
//!
//! Fairness: the trace and the base interpreter both reach the register file
//! through the shared `access::rload` / `access::rstore`, so only the dispatch
//! structure (linearized superblock vs block VM) differs. Cross-validated against
//! the base CFG interpreter on the full `(result, ninstr, nterm)` triple: the
//! trace accounts each iteration as one hot-block execution plus its terminator,
//! so identical counts prove the superblock walks exactly the base path.

use crate::access::{rload, rstore};
use crate::cfg::{op, Block, Term};

const NREG: usize = 8;

/// A selected self-loop trace: the anchor block (which branches back to itself),
/// its body instructions, the register the loop-back branch tests, and the block
/// to resume the base interpreter at on a side exit.
pub struct Trace {
    pub anchor: u32,
    pub body: Vec<crate::cfg::Instr>,
    pub guard_reg: u8,
    pub exit_block: u32,
}

/// Select the hottest self-loop trace by profiling one capped run. A self-loop is
/// a block whose conditional terminator branches back to itself on the taken side
/// (`BrNz(_, self, exit)`); the one executed most is the trace anchor. Returns
/// `None` if the CFG has no self-loop block (then the workload is not a trace
/// target and the caller falls back to the base interpreter).
pub fn select_trace(blocks: &[Block], seed: u64) -> Option<Trace> {
    let mut counts = vec![0u64; blocks.len()];
    // Profiling run: base interp with per-block execution counts, capped.
    let mut regs = [0u64; NREG];
    let rp = regs.as_mut_ptr();
    unsafe { rstore(rp, 0, seed) };
    let mut pc = 0u32;
    let mut steps = 0u64;
    let cap = 1_000_000u64;
    loop {
        counts[pc as usize] += 1;
        let b = &blocks[pc as usize];
        for ins in &b.instrs {
            let v = eval_instr(rp, ins);
            unsafe { rstore(rp, ins.dst as usize, v) };
        }
        steps += b.instrs.len() as u64 + 1;
        if steps > cap {
            break;
        }
        match b.term {
            Term::Jmp(t) => pc = t,
            Term::BrNz(r, nz, z) => pc = if unsafe { rload(rp, r as u32) } != 0 { nz } else { z },
            Term::Ret(_) => break,
        }
    }
    // Find the self-loop block with the highest execution count.
    let mut best: Option<(u32, u64)> = None;
    for (bi, b) in blocks.iter().enumerate() {
        if let Term::BrNz(_, nz, _) = b.term {
            if nz as usize == bi {
                let c = counts[bi];
                if best.map_or(true, |(_, bc)| c > bc) {
                    best = Some((bi as u32, c));
                }
            }
        }
    }
    let (anchor, _) = best?;
    let b = &blocks[anchor as usize];
    let (guard_reg, exit_block) = match b.term {
        Term::BrNz(r, _, z) => (r, z),
        _ => unreachable!("anchor is a BrNz self-loop by construction"),
    };
    Some(Trace {
        anchor,
        body: b.instrs.clone(),
        guard_reg,
        exit_block,
    })
}

/// Evaluate one register instruction against the register file (shared access).
#[inline(always)]
fn eval_instr(rp: *const u64, ins: &crate::cfg::Instr) -> u64 {
    use crate::ops::binop_body;
    unsafe {
        // the register VM's arithmetic is the same scalar semantics the carrier
        // ops define, so it defers to the same `binop_body!` (SET is CFG-specific).
        match ins.op {
            op::SET => ins.imm,
            op::ADD => binop_body!(ADD, rload(rp, ins.a as u32), rload(rp, ins.b as u32)),
            op::SUB => binop_body!(SUB, rload(rp, ins.a as u32), rload(rp, ins.b as u32)),
            op::AND => binop_body!(AND, rload(rp, ins.a as u32), rload(rp, ins.b as u32)),
            _ => binop_body!(MUL, rload(rp, ins.a as u32), rload(rp, ins.b as u32)),
        }
    }
}

/// Interpret the CFG with the hot self-loop run as a linearized trace superblock.
/// Cold blocks run in the base interpreter; when control reaches the trace anchor,
/// the trace body runs in a tight loop until the guard fails, then the base
/// interpreter resumes at the side-exit block. Returns the same
/// `(result, ninstr, nterm)` triple as the base CFG interpreter.
pub fn interp_traced(blocks: &[Block], trace: &Trace, seed: u64, cap: u64) -> (u64, u64, u64) {
    let mut regs = [0u64; NREG];
    let rp = regs.as_mut_ptr();
    unsafe { rstore(rp, 0, seed) };
    let mut pc = 0u32;
    let mut ninstr = 0u64;
    let mut nterm = 0u64;
    let body = trace.body.as_slice();
    loop {
        if pc == trace.anchor {
            // Hot path: run the superblock as a tight loop guarded by one branch.
            // Each iteration is one anchor-block execution plus its terminator, so
            // the counts match the base interpreter exactly.
            loop {
                for ins in body {
                    let v = eval_instr(rp, ins);
                    unsafe { rstore(rp, ins.dst as usize, v) };
                }
                ninstr += body.len() as u64;
                nterm += 1;
                if ninstr + nterm > cap {
                    return (unsafe { rload(rp, 0) }, ninstr, nterm);
                }
                if unsafe { rload(rp, trace.guard_reg as u32) } != 0 {
                    continue; // guard holds: stay on the trace
                }
                pc = trace.exit_block; // side exit: back to the base interpreter
                break;
            }
            continue;
        }
        // Cold path: one base-interpreter block step.
        let b = &blocks[pc as usize];
        for ins in &b.instrs {
            let v = eval_instr(rp, ins);
            unsafe { rstore(rp, ins.dst as usize, v) };
        }
        ninstr += b.instrs.len() as u64;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::{build_nested_loop, interp};

    #[test]
    fn trace_selects_inner_loop() {
        let blocks = build_nested_loop(20, 50);
        let trace = select_trace(&blocks, 7).expect("nested loop has a self-loop trace");
        // Block 2 is the inner-loop head (BrNz(3, 2, 3)): anchor 2, guard r3, exit 3.
        assert_eq!(trace.anchor, 2, "trace anchored on the inner-loop head");
        assert_eq!(trace.guard_reg, 3);
        assert_eq!(trace.exit_block, 3);
        assert_eq!(trace.body.len(), 3, "inner-loop body is MUL, ADD, SUB");
    }

    #[test]
    fn traced_matches_base_cfg() {
        // The traced interpreter agrees with the base CFG interpreter on the full
        // (result, ninstr, nterm) triple across kernel sizes and seeds, so the
        // superblock walks exactly the base path and measures dispatch structure
        // alone.
        for &(outer, inner) in &[(3u64, 5u64), (10, 10), (7, 100), (20, 40), (1, 200)] {
            for seed in [1u64, 3, 42, 1000] {
                let blocks = build_nested_loop(outer, inner);
                let trace = select_trace(&blocks, seed).expect("self-loop trace exists");
                assert_eq!(
                    interp(&blocks, seed, u64::MAX),
                    interp_traced(&blocks, &trace, seed, u64::MAX),
                    "traced vs base at ({outer},{inner}) seed {seed}"
                );
            }
        }
    }
}

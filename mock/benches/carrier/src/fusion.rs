//! The fusion run-half stage: superinstructions over the flat form.
//!
//! A production interpreter fuses the most frequent adjacent producer-consumer
//! node pairs into one handler that computes both and keeps the intermediate in a
//! register, skipping its arena round-trip. This attacks the memory-traffic term
//! directly. Here a pair `(i, i+1)` is fused when node `i+1` consumes node `i` as
//! a binary operand, node `i` is used by nothing else (single-use), and node `i`
//! is not a program output: then `i`'s value is produced into a register, `i+1`
//! reads it from the register, and `i`'s result is never stored.
//!
//! Because a fused-away intermediate is not materialised, the full results array
//! differs from the unfused run, so this cell cross-validates on the program's
//! live-out (sink) values (like the optimize stage), which are preserved: fusion
//! changes where values live, never what the outputs are.

use crate::access::{rload, rstore};
use crate::ir::op;
use crate::predecode::{PNode, Predecoded};

/// One fused-program step.
#[derive(Clone, Copy)]
pub enum Step {
    /// Evaluate node `idx` normally and store it.
    Single(u32),
    /// Evaluate producer `prod` into a register, then consumer `cons` (a binary
    /// op) using the register for operand `which` (0 = a, 1 = b); store only
    /// `cons`. `prod` is single-use and not a live-out, so its slot is never read.
    Fused(u32, u32, u8),
}

/// Build the fused step list for a predecoded program, greedily pairing adjacent
/// single-use producer-consumer nodes. `sink_nodes` are the live-outs, which are
/// never fused away.
pub fn fuse(p: &Predecoded) -> Vec<Step> {
    let n = p.nodes.len();
    // reference count over node operands (CONST operands are pool indices).
    let mut refcount = vec![0u32; n];
    for nd in &p.nodes {
        match nd.op {
            op::CONST | op::INPUT => {}
            op::SELECT => {
                refcount[nd.a as usize] += 1;
                refcount[nd.b as usize] += 1;
                refcount[nd.c as usize] += 1;
            }
            op::NEG | op::NOT => refcount[nd.a as usize] += 1,
            _ => {
                refcount[nd.a as usize] += 1;
                refcount[nd.b as usize] += 1;
            }
        }
    }
    let mut is_sink = vec![false; n];
    for &s in &sinks_from_pnodes(p) {
        is_sink[s as usize] = true;
    }

    let is_binary = |o: u8| matches!(
        o,
        op::ADD
            | op::SUB
            | op::MUL
            | op::AND
            | op::OR
            | op::XOR
            | op::SHL
            | op::SHR
            | op::MIN
            | op::MAX
            | op::EQ
            | op::LT
    );

    let mut steps = Vec::with_capacity(n);
    let mut i = 0usize;
    while i < n {
        if i + 1 < n {
            let cons = p.nodes[i + 1];
            if is_binary(cons.op)
                && refcount[i] == 1
                && !is_sink[i]
                && p.nodes[i].op != op::INPUT
            {
                if cons.a as usize == i {
                    steps.push(Step::Fused(i as u32, (i + 1) as u32, 0));
                    i += 2;
                    continue;
                } else if cons.b as usize == i {
                    steps.push(Step::Fused(i as u32, (i + 1) as u32, 1));
                    i += 2;
                    continue;
                }
            }
        }
        steps.push(Step::Single(i as u32));
        i += 1;
    }
    steps
}

// sinks of a predecoded program (nodes referenced by no later node).
fn sinks_from_pnodes(p: &Predecoded) -> Vec<u32> {
    let n = p.nodes.len();
    let mut referenced = vec![false; n];
    for nd in &p.nodes {
        match nd.op {
            op::CONST | op::INPUT => {}
            op::SELECT => {
                referenced[nd.a as usize] = true;
                referenced[nd.b as usize] = true;
                referenced[nd.c as usize] = true;
            }
            op::NEG | op::NOT => referenced[nd.a as usize] = true,
            _ => {
                referenced[nd.a as usize] = true;
                referenced[nd.b as usize] = true;
            }
        }
    }
    (0..n as u32).filter(|&i| !referenced[i as usize]).collect()
}

// evaluate one binary op given its two operand values.
#[inline(always)]
fn bin(opcode: u8, a: u64, b: u64) -> u64 {
    match opcode {
        op::ADD => a.wrapping_add(b),
        op::SUB => a.wrapping_sub(b),
        op::MUL => a.wrapping_mul(b),
        op::AND => a & b,
        op::OR => a | b,
        op::XOR => a ^ b,
        op::SHL => a.wrapping_shl(b as u32),
        op::SHR => a.wrapping_shr(b as u32),
        op::MIN => a.min(b),
        op::MAX => a.max(b),
        op::EQ => (a == b) as u64,
        op::LT => (a < b) as u64,
        _ => 0,
    }
}

// evaluate any node given the results read pointer and const pointer.
#[inline(always)]
unsafe fn eval_node(nd: PNode, rp: *const u64, cp: *const u64, seed: u64) -> u64 {
    match nd.op {
        op::INPUT => seed,
        op::CONST => *cp.add(nd.a as usize),
        op::SELECT => {
            if rload(rp, nd.a) != 0 {
                rload(rp, nd.b)
            } else {
                rload(rp, nd.c)
            }
        }
        op::NEG => rload(rp, nd.a).wrapping_neg(),
        op::NOT => !rload(rp, nd.a),
        other => bin(other, rload(rp, nd.a), rload(rp, nd.b)),
    }
}

/// Interpret the fused step list. Fills `results`, leaving fused-away producer
/// slots untouched (they are single-use and dead). The caller folds the live-out
/// checksum, not the full array.
#[inline]
pub fn interpret_fused(p: &Predecoded, steps: &[Step], input_seed: u64, results: &mut [u64]) {
    let wp = results.as_mut_ptr();
    let rp = wp as *const u64;
    let cp = p.consts.as_ptr();
    let np = p.nodes.as_ptr();
    for step in steps {
        match *step {
            Step::Single(i) => {
                let nd = unsafe { *np.add(i as usize) };
                let v = unsafe { eval_node(nd, rp, cp, input_seed) };
                unsafe { rstore(wp, i as usize, v) };
            }
            Step::Fused(prod, cons, which) => {
                let pnd = unsafe { *np.add(prod as usize) };
                let vprod = unsafe { eval_node(pnd, rp, cp, input_seed) };
                let cnd = unsafe { *np.add(cons as usize) };
                let a = if which == 0 { vprod } else { unsafe { rload(rp, cnd.a) } };
                let b = if which == 1 { vprod } else { unsafe { rload(rp, cnd.b) } };
                let v = bin(cnd.op, a, b);
                unsafe { rstore(wp, cons as usize, v) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::checksum_at;
    use crate::ir::{encode, Decoded, REC24};
    use crate::optimize::sinks;
    use crate::predecode::predecode;
    use crate::{generate, GenParams};

    #[test]
    fn fused_preserves_outputs() {
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 700;
            let prog = generate(&gp);
            let bytes = encode(&prog, &REC24);
            let d = Decoded::parse(&bytes, REC24).unwrap();
            let p = predecode(&d);
            let out = sinks(&prog);
            let steps = fuse(&p);
            let mut r1 = vec![0u64; prog.nodes.len()];
            let mut r2 = vec![0u64; prog.nodes.len()];
            for seed in [0u64, 1, 42, 12345, 999_999] {
                crate::interp::interpret(&d, seed, &mut r1);
                interpret_fused(&p, &steps, seed, &mut r2);
                assert_eq!(
                    checksum_at(&r1, &out),
                    checksum_at(&r2, &out),
                    "{name} fused output diverged at seed {seed}"
                );
            }
        }
    }
}

//! Predecoded interpretation: split decode from dispatch.
//!
//! The reference interpreter is zero-copy over the wire: every node, every
//! iteration, it recomputes `nodes_start + i*stride + header + k*4` and runs
//! `from_le_bytes` to pull each operand out of the byte stream. That wire-unpack
//! is paid once per node per run. A real interpreter that runs the same program
//! many times predecodes it once into a flat, dispatch-ready form and then runs
//! a tight loop with no wire arithmetic. This module measures whether that pays.
//!
//! The predecode is a legitimate front-loaded cost (it happens once, outside the
//! timed region, exactly like `Decoded::parse`), so the timed comparison is
//! per-iteration interpretation throughput: flat predecoded loop versus wire
//! decode. Semantics are byte-identical to `interp::interpret` (same operand
//! reads, same hash fold), cross-validated. The flat record is 16 bytes (op plus
//! three u32 operands) against the 24-byte wire record, so the predecoded form is
//! also a smaller working set, which matters once the program spills L1.

use crate::ir::{op, Decoded};

/// One predecoded node: the opcode and up to three operand slots already
/// resolved from the wire (node-result indices for ops, a const-pool index for
/// CONST). 16 bytes, four per cache line.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNode {
    pub op: u8,
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

/// A program predecoded into flat dispatch-ready records plus the const pool.
pub struct Predecoded {
    pub nodes: Vec<PNode>,
    pub consts: Vec<u64>,
}

/// Predecode a wire program into the flat form. Reads every operand once, at the
/// node's arity, so the hot loop never touches wire bytes again.
pub fn predecode(d: &Decoded) -> Predecoded {
    let n = d.node_count;
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        let opc = d.op_at(i);
        let (a, b, c) = match opc {
            op::INPUT => (0, 0, 0),
            op::CONST => (d.operand(i, 0, 1), 0, 0),
            op::SELECT => (d.operand(i, 0, 3), d.operand(i, 1, 3), d.operand(i, 2, 3)),
            op::NEG | op::NOT => (d.operand(i, 0, 1), 0, 0),
            _ => (d.operand(i, 0, 2), d.operand(i, 1, 2), 0),
        };
        nodes.push(PNode { op: opc, a, b, c });
    }
    let mut consts = Vec::with_capacity(d.const_count);
    for i in 0..d.const_count {
        consts.push(d.const_at(i));
    }
    Predecoded { nodes, consts }
}

/// Switch dispatch over the predecoded form. Identical semantics to
/// `interp::interpret`, no wire arithmetic in the loop.
#[inline]
pub fn interpret_predecoded(p: &Predecoded, input_seed: u64, results: &mut [u64]) -> u64 {
    let mut hash: u64 = 0;
    for i in 0..p.nodes.len() {
        let nd = p.nodes[i];
        let a = nd.a as usize;
        let b = nd.b as usize;
        let v = match nd.op {
            op::INPUT => input_seed,
            op::CONST => p.consts[a],
            op::ADD => results[a].wrapping_add(results[b]),
            op::SUB => results[a].wrapping_sub(results[b]),
            op::MUL => results[a].wrapping_mul(results[b]),
            op::AND => results[a] & results[b],
            op::OR => results[a] | results[b],
            op::XOR => results[a] ^ results[b],
            op::SHL => results[a].wrapping_shl(results[b] as u32),
            op::SHR => results[a].wrapping_shr(results[b] as u32),
            op::MIN => results[a].min(results[b]),
            op::MAX => results[a].max(results[b]),
            op::EQ => (results[a] == results[b]) as u64,
            op::LT => (results[a] < results[b]) as u64,
            op::SELECT => {
                if results[a] != 0 {
                    results[b]
                } else {
                    results[nd.c as usize]
                }
            }
            op::NEG => results[a].wrapping_neg(),
            op::NOT => !results[a],
            _ => 0,
        };
        results[i] = v;
        hash = hash.rotate_left(7) ^ v;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{encode, REC24};
    use crate::{generate, GenParams};

    #[test]
    fn predecoded_matches_switch() {
        let prog = generate(&GenParams {
            node_count: 500,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let p = predecode(&d);
        let mut r1 = vec![0u64; prog.nodes.len()];
        let mut r2 = vec![0u64; prog.nodes.len()];
        for seed in [0u64, 1, 42, 255, 1000, 999_999] {
            assert_eq!(
                crate::interp::interpret(&d, seed, &mut r1),
                interpret_predecoded(&p, seed, &mut r2),
                "predecoded diverged from switch at seed {seed}"
            );
        }
    }
}

//! The reference interpreter: the shipped-default configuration (switch
//! dispatch, u64 values, decode-from-wire at whatever layout the program was
//! encoded with). A bench that varies the record layout uses this interpreter
//! unchanged and only changes the encode layout; a bench that varies dispatch
//! or value representation swaps this function for an axis variant while keeping
//! everything else identical. Either way the program crosses into the routine
//! as wire bytes, so the interpretation can never be partially evaluated away.

use crate::checksum::Checksum;
use crate::ir::{op, Decoded};

/// Interpret the decoded program once with the given per-call input seed,
/// folding every node result into a checksum which is returned. `results` is a
/// caller-owned scratch buffer of length at least `d.node_count`, so the hot
/// loop never allocates. Switch dispatch on the opcode.
#[inline]
pub fn interpret(d: &Decoded, input_seed: u64, results: &mut [u64]) -> u64 {
    let mut cs = Checksum::new();
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        let v = match opcode {
            op::INPUT => input_seed,
            op::CONST => d.const_at(d.operand(i, 0, 1) as usize),
            op::NEG => results[d.operand(i, 0, 1) as usize].wrapping_neg(),
            op::NOT => !results[d.operand(i, 0, 1) as usize],
            op::SELECT => {
                let c = results[d.operand(i, 0, 3) as usize];
                let t = results[d.operand(i, 1, 3) as usize];
                let f = results[d.operand(i, 2, 3) as usize];
                if c != 0 {
                    t
                } else {
                    f
                }
            }
            _ => {
                let a = results[d.operand(i, 0, 2) as usize];
                let b = results[d.operand(i, 1, 2) as usize];
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
        };
        results[i] = v;
        cs.fold(v);
    }
    cs.0
}

/// Drive the interpreter over a stream of input bytes, folding each per-byte
/// checksum into one accumulator. This is the exact shape a variant's `timed!`
/// region runs: the program structure is fixed, the input bytes vary the eval.
#[inline]
pub fn run_over_input(d: &Decoded, input: &[u8], results: &mut [u64]) -> u64 {
    let mut acc: u64 = 0;
    for &byte in input {
        acc ^= interpret(d, byte as u64, results);
    }
    acc
}

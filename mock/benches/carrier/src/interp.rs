//! The reference interpreters over the wire form, one per dispatch shape.
//!
//! Every interpreter here fills the caller-owned `results` scratch and returns
//! nothing; the caller folds one `access::checksum(results)` after the pass. The
//! per-node rolling hash that used to live in the dispatch loop is gone, so the
//! inner loop is decode plus dispatch plus store and nothing else, and the
//! dispatch measurement is clean. Every operand read and result write goes
//! through `access::rload` / `access::rstore` (unchecked, sound post-validation),
//! identical across all shapes, so the only thing the dispatch axis varies is
//! dispatch. The program crosses into these routines as wire bytes, so the
//! interpretation can never be partially evaluated away.

use crate::access::{rload, rstore};
use crate::ir::{op, Decoded};

/// Switch dispatch: a flat single-level `match` over the opcode, which the
/// backend lowers to a jump table. Fills `results`; no in-loop hash.
#[inline]
pub fn interpret(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        macro_rules! bin {
            ($k:expr) => {
                unsafe { rload(rp, d.operand(i, $k, 2)) }
            };
        }
        let v = match opcode {
            op::INPUT => input_seed,
            op::CONST => d.const_at(d.operand(i, 0, 1) as usize),
            op::ADD => bin!(0).wrapping_add(bin!(1)),
            op::SUB => bin!(0).wrapping_sub(bin!(1)),
            op::MUL => bin!(0).wrapping_mul(bin!(1)),
            op::AND => bin!(0) & bin!(1),
            op::OR => bin!(0) | bin!(1),
            op::XOR => bin!(0) ^ bin!(1),
            op::SHL => bin!(0).wrapping_shl(bin!(1) as u32),
            op::SHR => bin!(0).wrapping_shr(bin!(1) as u32),
            op::MIN => bin!(0).min(bin!(1)),
            op::MAX => bin!(0).max(bin!(1)),
            op::EQ => (bin!(0) == bin!(1)) as u64,
            op::LT => (bin!(0) < bin!(1)) as u64,
            op::SELECT => {
                if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                    unsafe { rload(rp, d.operand(i, 1, 3)) }
                } else {
                    unsafe { rload(rp, d.operand(i, 2, 3)) }
                }
            }
            op::NEG => unsafe { rload(rp, d.operand(i, 0, 1)) }.wrapping_neg(),
            op::NOT => !unsafe { rload(rp, d.operand(i, 0, 1)) },
            _ => 0,
        };
        unsafe { rstore(p, i, v) };
    }
}

/// Function-pointer-table dispatch: one indirect call per node through a table,
/// from a returning loop. The fair indirect-threaded alternative to the switch.
type OpFn = fn(&Decoded, usize, *const u64, u64) -> u64;

fn f_const(d: &Decoded, i: usize, _r: *const u64, _s: u64) -> u64 {
    d.const_at(d.operand(i, 0, 1) as usize)
}
fn f_input(_d: &Decoded, _i: usize, _r: *const u64, s: u64) -> u64 {
    s
}
fn f_neg(d: &Decoded, i: usize, r: *const u64, _s: u64) -> u64 {
    unsafe { rload(r, d.operand(i, 0, 1)) }.wrapping_neg()
}
fn f_not(d: &Decoded, i: usize, r: *const u64, _s: u64) -> u64 {
    !unsafe { rload(r, d.operand(i, 0, 1)) }
}
fn f_select(d: &Decoded, i: usize, r: *const u64, _s: u64) -> u64 {
    unsafe {
        if rload(r, d.operand(i, 0, 3)) != 0 {
            rload(r, d.operand(i, 1, 3))
        } else {
            rload(r, d.operand(i, 2, 3))
        }
    }
}
macro_rules! binop {
    ($name:ident, $a:ident, $b:ident, $body:expr) => {
        fn $name(d: &Decoded, i: usize, r: *const u64, _s: u64) -> u64 {
            let $a = unsafe { rload(r, d.operand(i, 0, 2)) };
            let $b = unsafe { rload(r, d.operand(i, 1, 2)) };
            $body
        }
    };
}
binop!(f_add, a, b, a.wrapping_add(b));
binop!(f_sub, a, b, a.wrapping_sub(b));
binop!(f_mul, a, b, a.wrapping_mul(b));
binop!(f_and, a, b, a & b);
binop!(f_or, a, b, a | b);
binop!(f_xor, a, b, a ^ b);
binop!(f_shl, a, b, a.wrapping_shl(b as u32));
binop!(f_shr, a, b, a.wrapping_shr(b as u32));
binop!(f_min, a, b, a.min(b));
binop!(f_max, a, b, a.max(b));
binop!(f_eq, a, b, (a == b) as u64);
binop!(f_lt, a, b, (a < b) as u64);

/// Table indexed by opcode; order matches `ir::op`.
static DISPATCH: [OpFn; op::COUNT as usize] = [
    f_const, f_add, f_sub, f_mul, f_and, f_or, f_xor, f_shl, f_shr, f_min, f_max, f_eq, f_lt,
    f_select, f_neg, f_not, f_input,
];

/// Function-pointer-table variant of [`interpret`]. Fills `results`.
#[inline]
pub fn interpret_fntable(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        let opcode = d.op_at(i) as usize;
        let v = DISPATCH[opcode](d, i, rp, input_seed);
        unsafe { rstore(p, i, v) };
    }
}

/// If-chain dispatch: an explicit linear if-else cascade over the opcode,
/// FREQUENCY-ordered (hot ops first) so it is the honest strongest form of the
/// technique the match-lowering finding tested (an ascending-opcode order would
/// be a strawman that puts hot ops behind cold comparisons). The order below is
/// hot-first for a typical arithmetic IR; `interpret_ifchain_ascending` carries
/// the opcode-order variant as the labelled sub-axis.
#[inline]
pub fn interpret_ifchain(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        macro_rules! bin {
            ($k:expr) => {
                unsafe { rload(rp, d.operand(i, $k, 2)) }
            };
        }
        // frequency order: arithmetic binaries, then leaves, then the rest.
        let v = if opcode == op::ADD {
            bin!(0).wrapping_add(bin!(1))
        } else if opcode == op::MUL {
            bin!(0).wrapping_mul(bin!(1))
        } else if opcode == op::SUB {
            bin!(0).wrapping_sub(bin!(1))
        } else if opcode == op::CONST {
            d.const_at(d.operand(i, 0, 1) as usize)
        } else if opcode == op::INPUT {
            input_seed
        } else if opcode == op::AND {
            bin!(0) & bin!(1)
        } else if opcode == op::OR {
            bin!(0) | bin!(1)
        } else if opcode == op::XOR {
            bin!(0) ^ bin!(1)
        } else if opcode == op::LT {
            (bin!(0) < bin!(1)) as u64
        } else if opcode == op::EQ {
            (bin!(0) == bin!(1)) as u64
        } else if opcode == op::SHL {
            bin!(0).wrapping_shl(bin!(1) as u32)
        } else if opcode == op::SHR {
            bin!(0).wrapping_shr(bin!(1) as u32)
        } else if opcode == op::MIN {
            bin!(0).min(bin!(1))
        } else if opcode == op::MAX {
            bin!(0).max(bin!(1))
        } else if opcode == op::SELECT {
            if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                unsafe { rload(rp, d.operand(i, 1, 3)) }
            } else {
                unsafe { rload(rp, d.operand(i, 2, 3)) }
            }
        } else if opcode == op::NEG {
            unsafe { rload(rp, d.operand(i, 0, 1)) }.wrapping_neg()
        } else if opcode == op::NOT {
            !unsafe { rload(rp, d.operand(i, 0, 1)) }
        } else {
            0
        };
        unsafe { rstore(p, i, v) };
    }
}

/// Ascending-opcode-ordered if-chain: the labelled sub-axis for the ordering
/// question. Identical semantics to [`interpret_ifchain`], cold-first order.
#[inline]
pub fn interpret_ifchain_ascending(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        macro_rules! bin {
            ($k:expr) => {
                unsafe { rload(rp, d.operand(i, $k, 2)) }
            };
        }
        let v = if opcode == op::CONST {
            d.const_at(d.operand(i, 0, 1) as usize)
        } else if opcode == op::ADD {
            bin!(0).wrapping_add(bin!(1))
        } else if opcode == op::SUB {
            bin!(0).wrapping_sub(bin!(1))
        } else if opcode == op::MUL {
            bin!(0).wrapping_mul(bin!(1))
        } else if opcode == op::AND {
            bin!(0) & bin!(1)
        } else if opcode == op::OR {
            bin!(0) | bin!(1)
        } else if opcode == op::XOR {
            bin!(0) ^ bin!(1)
        } else if opcode == op::SHL {
            bin!(0).wrapping_shl(bin!(1) as u32)
        } else if opcode == op::SHR {
            bin!(0).wrapping_shr(bin!(1) as u32)
        } else if opcode == op::MIN {
            bin!(0).min(bin!(1))
        } else if opcode == op::MAX {
            bin!(0).max(bin!(1))
        } else if opcode == op::EQ {
            (bin!(0) == bin!(1)) as u64
        } else if opcode == op::LT {
            (bin!(0) < bin!(1)) as u64
        } else if opcode == op::SELECT {
            if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                unsafe { rload(rp, d.operand(i, 1, 3)) }
            } else {
                unsafe { rload(rp, d.operand(i, 2, 3)) }
            }
        } else if opcode == op::NEG {
            unsafe { rload(rp, d.operand(i, 0, 1)) }.wrapping_neg()
        } else if opcode == op::NOT {
            !unsafe { rload(rp, d.operand(i, 0, 1)) }
        } else if opcode == op::INPUT {
            input_seed
        } else {
            0
        };
        unsafe { rstore(p, i, v) };
    }
}

/// Bit-test tree dispatch: a balanced binary search over the opcode range
/// instead of a jump table or a linear cascade. Log-depth branches; the third
/// branch-lowering strategy the match-lowering bench did not compose. Identical
/// semantics. Split points chosen to halve the 0..16 opcode range.
#[inline]
pub fn interpret_bittree(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        macro_rules! bin {
            ($k:expr) => {
                unsafe { rload(rp, d.operand(i, $k, 2)) }
            };
        }
        let v = if opcode < 8 {
            if opcode < 4 {
                if opcode < 2 {
                    if opcode == op::CONST {
                        d.const_at(d.operand(i, 0, 1) as usize)
                    } else {
                        bin!(0).wrapping_add(bin!(1)) // ADD
                    }
                } else if opcode == op::SUB {
                    bin!(0).wrapping_sub(bin!(1))
                } else {
                    bin!(0).wrapping_mul(bin!(1)) // MUL
                }
            } else if opcode < 6 {
                if opcode == op::AND {
                    bin!(0) & bin!(1)
                } else {
                    bin!(0) | bin!(1) // OR
                }
            } else if opcode == op::XOR {
                bin!(0) ^ bin!(1)
            } else {
                bin!(0).wrapping_shl(bin!(1) as u32) // SHL
            }
        } else if opcode < 12 {
            if opcode < 10 {
                if opcode == op::SHR {
                    bin!(0).wrapping_shr(bin!(1) as u32)
                } else {
                    bin!(0).min(bin!(1)) // MIN
                }
            } else if opcode == op::MAX {
                bin!(0).max(bin!(1))
            } else {
                (bin!(0) == bin!(1)) as u64 // EQ
            }
        } else if opcode < 14 {
            if opcode == op::LT {
                (bin!(0) < bin!(1)) as u64
            } else {
                // SELECT
                if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                    unsafe { rload(rp, d.operand(i, 1, 3)) }
                } else {
                    unsafe { rload(rp, d.operand(i, 2, 3)) }
                }
            }
        } else if opcode == op::NEG {
            unsafe { rload(rp, d.operand(i, 0, 1)) }.wrapping_neg()
        } else if opcode == op::NOT {
            !unsafe { rload(rp, d.operand(i, 0, 1)) }
        } else {
            input_seed // INPUT (16)
        };
        unsafe { rstore(p, i, v) };
    }
}

/// Null-dispatch reference floor: the same operand decode, the same two loads,
/// the same store, but NO opcode dispatch. Every node is executed as one fixed
/// operation (a wrapping add of its first two arity-2 operands, or the input
/// seed for a leaf), so its per-node cost is the interpreter's memory and
/// bookkeeping structure with dispatch removed. The gap between a real
/// interpreter's slope and this floor's slope is the dispatch cost, isolated.
/// Not cross-validated against the real interpreters (it computes a different
/// result on purpose); it is a floor, folded with its own checksum.
#[inline]
pub fn interpret_nulldispatch(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        // read two operands the same way the real loop does (leaf nodes have
        // no node operands, so guard on arity to stay in-bounds under the
        // children-before-parents contract), then one fixed op, then store.
        let ar = d.arity_at(i);
        let v = if ar >= 2 {
            let a = unsafe { rload(rp, d.operand(i, 0, 2)) };
            let b = unsafe { rload(rp, d.operand(i, 1, 2)) };
            a.wrapping_add(b)
        } else if ar == 1 {
            unsafe { rload(rp, d.operand(i, 0, 1)) }
        } else {
            input_seed
        };
        unsafe { rstore(p, i, v) };
    }
}

/// Drive the switch interpreter over a stream of input bytes, folding one
/// post-pass checksum per evaluation. The exact shape a variant's timed region
/// runs, with the checksum out of the per-node loop.
#[inline]
pub fn run_over_input(d: &Decoded, input: &[u8], results: &mut [u64]) -> u64 {
    let mut acc: u64 = 0;
    for &byte in input {
        interpret(d, byte as u64, results);
        acc ^= crate::access::checksum(results);
    }
    acc
}

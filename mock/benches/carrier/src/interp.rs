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
use crate::ops::{binop_body, unop_body};

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
        // op bodies come from the single `ops::binop_body` / `unop_body`
        // definition; only the switch skeleton (this match) is this cell's.
        let v = match opcode {
            op::INPUT => input_seed,
            op::CONST => d.const_at(d.operand(i, 0, 1) as usize),
            op::ADD => binop_body!(ADD, bin!(0), bin!(1)),
            op::SUB => binop_body!(SUB, bin!(0), bin!(1)),
            op::MUL => binop_body!(MUL, bin!(0), bin!(1)),
            op::AND => binop_body!(AND, bin!(0), bin!(1)),
            op::OR => binop_body!(OR, bin!(0), bin!(1)),
            op::XOR => binop_body!(XOR, bin!(0), bin!(1)),
            op::SHL => binop_body!(SHL, bin!(0), bin!(1)),
            op::SHR => binop_body!(SHR, bin!(0), bin!(1)),
            op::MIN => binop_body!(MIN, bin!(0), bin!(1)),
            op::MAX => binop_body!(MAX, bin!(0), bin!(1)),
            op::EQ => binop_body!(EQ, bin!(0), bin!(1)),
            op::LT => binop_body!(LT, bin!(0), bin!(1)),
            op::SELECT => {
                if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                    unsafe { rload(rp, d.operand(i, 1, 3)) }
                } else {
                    unsafe { rload(rp, d.operand(i, 2, 3)) }
                }
            }
            op::NEG => unop_body!(NEG, unsafe { rload(rp, d.operand(i, 0, 1)) }),
            op::NOT => unop_body!(NOT, unsafe { rload(rp, d.operand(i, 0, 1)) }),
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
    unop_body!(NEG, unsafe { rload(r, d.operand(i, 0, 1)) })
}
fn f_not(d: &Decoded, i: usize, r: *const u64, _s: u64) -> u64 {
    unop_body!(NOT, unsafe { rload(r, d.operand(i, 0, 1)) })
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
// each f_* function reads its two operands, then defers to the single
// `binop_body!` definition for the op's semantics.
macro_rules! fbin {
    ($name:ident, $op:ident) => {
        fn $name(d: &Decoded, i: usize, r: *const u64, _s: u64) -> u64 {
            let a = unsafe { rload(r, d.operand(i, 0, 2)) };
            let b = unsafe { rload(r, d.operand(i, 1, 2)) };
            binop_body!($op, a, b)
        }
    };
}
fbin!(f_add, ADD);
fbin!(f_sub, SUB);
fbin!(f_mul, MUL);
fbin!(f_and, AND);
fbin!(f_or, OR);
fbin!(f_xor, XOR);
fbin!(f_shl, SHL);
fbin!(f_shr, SHR);
fbin!(f_min, MIN);
fbin!(f_max, MAX);
fbin!(f_eq, EQ);
fbin!(f_lt, LT);

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

/// If-chain dispatch: an explicit if-else cascade over the opcode, FREQUENCY-
/// ordered (hot ops first). Written as a source-level linear cascade, but note
/// the ISA reality: over a small dense opcode range LLVM's SimplifyCFG recognizes
/// this cascade of `opcode == k` tests and canonicalizes it into a computed jump
/// table (an `ldrb` of a byte-index table, a shifted `add`, and a `br`), the same
/// O(1) dispatch the switch cell lowers to. A jump table does not care what order
/// the source comparisons were written in, so on this variant the frequency
/// ordering is effectively moot and the measured if-chain-vs-switch delta is two
/// near-identical lowerings, not "linear scan vs jump table." This is the honest
/// natural-lowering answer: what a source-level if-chain actually costs once a
/// production compiler has had it. `interpret_ifchain_ascending` is the ordering
/// sub-axis (also jump-table-lowered), and `interpret_ifchain_linear` is the
/// barrier-forced true-linear-scan cell where the cascade survives and the
/// frequency ordering does carry weight. The ISA claim is checked by the
/// `di_ifchain` disasm probe.
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
            binop_body!(ADD, bin!(0), bin!(1))
        } else if opcode == op::MUL {
            binop_body!(MUL, bin!(0), bin!(1))
        } else if opcode == op::SUB {
            binop_body!(SUB, bin!(0), bin!(1))
        } else if opcode == op::CONST {
            d.const_at(d.operand(i, 0, 1) as usize)
        } else if opcode == op::INPUT {
            input_seed
        } else if opcode == op::AND {
            binop_body!(AND, bin!(0), bin!(1))
        } else if opcode == op::OR {
            binop_body!(OR, bin!(0), bin!(1))
        } else if opcode == op::XOR {
            binop_body!(XOR, bin!(0), bin!(1))
        } else if opcode == op::LT {
            binop_body!(LT, bin!(0), bin!(1))
        } else if opcode == op::EQ {
            binop_body!(EQ, bin!(0), bin!(1))
        } else if opcode == op::SHL {
            binop_body!(SHL, bin!(0), bin!(1))
        } else if opcode == op::SHR {
            binop_body!(SHR, bin!(0), bin!(1))
        } else if opcode == op::MIN {
            binop_body!(MIN, bin!(0), bin!(1))
        } else if opcode == op::MAX {
            binop_body!(MAX, bin!(0), bin!(1))
        } else if opcode == op::SELECT {
            if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                unsafe { rload(rp, d.operand(i, 1, 3)) }
            } else {
                unsafe { rload(rp, d.operand(i, 2, 3)) }
            }
        } else if opcode == op::NEG {
            unop_body!(NEG, unsafe { rload(rp, d.operand(i, 0, 1)) })
        } else if opcode == op::NOT {
            unop_body!(NOT, unsafe { rload(rp, d.operand(i, 0, 1)) })
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
            binop_body!(ADD, bin!(0), bin!(1))
        } else if opcode == op::SUB {
            binop_body!(SUB, bin!(0), bin!(1))
        } else if opcode == op::MUL {
            binop_body!(MUL, bin!(0), bin!(1))
        } else if opcode == op::AND {
            binop_body!(AND, bin!(0), bin!(1))
        } else if opcode == op::OR {
            binop_body!(OR, bin!(0), bin!(1))
        } else if opcode == op::XOR {
            binop_body!(XOR, bin!(0), bin!(1))
        } else if opcode == op::SHL {
            binop_body!(SHL, bin!(0), bin!(1))
        } else if opcode == op::SHR {
            binop_body!(SHR, bin!(0), bin!(1))
        } else if opcode == op::MIN {
            binop_body!(MIN, bin!(0), bin!(1))
        } else if opcode == op::MAX {
            binop_body!(MAX, bin!(0), bin!(1))
        } else if opcode == op::EQ {
            binop_body!(EQ, bin!(0), bin!(1))
        } else if opcode == op::LT {
            binop_body!(LT, bin!(0), bin!(1))
        } else if opcode == op::SELECT {
            if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                unsafe { rload(rp, d.operand(i, 1, 3)) }
            } else {
                unsafe { rload(rp, d.operand(i, 2, 3)) }
            }
        } else if opcode == op::NEG {
            unop_body!(NEG, unsafe { rload(rp, d.operand(i, 0, 1)) })
        } else if opcode == op::NOT {
            unop_body!(NOT, unsafe { rload(rp, d.operand(i, 0, 1)) })
        } else if opcode == op::INPUT {
            input_seed
        } else {
            0
        };
        unsafe { rstore(p, i, v) };
    }
}

/// Barrier-forced true-linear-scan if-chain: frequency-ordered, with each
/// `opcode == k` comparison wrapped in `core::hint::black_box` so SimplifyCFG
/// cannot recognize the dense-integer-switch pattern and canonicalize the cascade
/// into a jump table. The result genuinely stays a linear scan of comparisons
/// short-circuiting at the first match, which is the textbook technique some real
/// interpreters use and the shape `interpret_ifchain` was named for but does not
/// actually compile to. This is where frequency ordering carries real weight: a
/// hot op matches after few comparisons, a cold op after many. The tradeoff is
/// that the cell measures an artificially preserved technique (a real interpreter
/// on this compiler would get the jump-table lowering instead), so it answers "what
/// would linear-scan dispatch cost in isolation," while `interpret_ifchain` answers
/// "what does a source if-chain actually cost once compiled." The `di_ifchain_linear`
/// probe confirms the barrier kept the branches (no computed jump).
#[inline]
pub fn interpret_ifchain_linear(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    use core::hint::black_box;
    let p = results.as_mut_ptr();
    let rp = p as *const u64;
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        macro_rules! bin {
            ($k:expr) => {
                unsafe { rload(rp, d.operand(i, $k, 2)) }
            };
        }
        // black_box each comparison so the compiler must materialize and branch
        // on it individually, keeping the cascade a real linear scan.
        let v = if black_box(opcode == op::ADD) {
            binop_body!(ADD, bin!(0), bin!(1))
        } else if black_box(opcode == op::MUL) {
            binop_body!(MUL, bin!(0), bin!(1))
        } else if black_box(opcode == op::SUB) {
            binop_body!(SUB, bin!(0), bin!(1))
        } else if black_box(opcode == op::CONST) {
            d.const_at(d.operand(i, 0, 1) as usize)
        } else if black_box(opcode == op::INPUT) {
            input_seed
        } else if black_box(opcode == op::AND) {
            binop_body!(AND, bin!(0), bin!(1))
        } else if black_box(opcode == op::OR) {
            binop_body!(OR, bin!(0), bin!(1))
        } else if black_box(opcode == op::XOR) {
            binop_body!(XOR, bin!(0), bin!(1))
        } else if black_box(opcode == op::LT) {
            binop_body!(LT, bin!(0), bin!(1))
        } else if black_box(opcode == op::EQ) {
            binop_body!(EQ, bin!(0), bin!(1))
        } else if black_box(opcode == op::SHL) {
            binop_body!(SHL, bin!(0), bin!(1))
        } else if black_box(opcode == op::SHR) {
            binop_body!(SHR, bin!(0), bin!(1))
        } else if black_box(opcode == op::MIN) {
            binop_body!(MIN, bin!(0), bin!(1))
        } else if black_box(opcode == op::MAX) {
            binop_body!(MAX, bin!(0), bin!(1))
        } else if black_box(opcode == op::SELECT) {
            if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                unsafe { rload(rp, d.operand(i, 1, 3)) }
            } else {
                unsafe { rload(rp, d.operand(i, 2, 3)) }
            }
        } else if black_box(opcode == op::NEG) {
            unop_body!(NEG, unsafe { rload(rp, d.operand(i, 0, 1)) })
        } else if black_box(opcode == op::NOT) {
            unop_body!(NOT, unsafe { rload(rp, d.operand(i, 0, 1)) })
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
                        binop_body!(ADD, bin!(0), bin!(1)) // ADD
                    }
                } else if opcode == op::SUB {
                    binop_body!(SUB, bin!(0), bin!(1))
                } else {
                    binop_body!(MUL, bin!(0), bin!(1)) // MUL
                }
            } else if opcode < 6 {
                if opcode == op::AND {
                    binop_body!(AND, bin!(0), bin!(1))
                } else {
                    binop_body!(OR, bin!(0), bin!(1)) // OR
                }
            } else if opcode == op::XOR {
                binop_body!(XOR, bin!(0), bin!(1))
            } else {
                binop_body!(SHL, bin!(0), bin!(1)) // SHL
            }
        } else if opcode < 12 {
            if opcode < 10 {
                if opcode == op::SHR {
                    binop_body!(SHR, bin!(0), bin!(1))
                } else {
                    binop_body!(MIN, bin!(0), bin!(1)) // MIN
                }
            } else if opcode == op::MAX {
                binop_body!(MAX, bin!(0), bin!(1))
            } else {
                binop_body!(EQ, bin!(0), bin!(1)) // EQ
            }
        } else if opcode < 14 {
            if opcode == op::LT {
                binop_body!(LT, bin!(0), bin!(1))
            } else {
                // SELECT
                if unsafe { rload(rp, d.operand(i, 0, 3)) } != 0 {
                    unsafe { rload(rp, d.operand(i, 1, 3)) }
                } else {
                    unsafe { rload(rp, d.operand(i, 2, 3)) }
                }
            }
        } else if opcode == op::NEG {
            unop_body!(NEG, unsafe { rload(rp, d.operand(i, 0, 1)) })
        } else if opcode == op::NOT {
            unop_body!(NOT, unsafe { rload(rp, d.operand(i, 0, 1)) })
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

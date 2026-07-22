//! The reference interpreter: the shipped-default configuration (switch
//! dispatch, u64 values, decode-from-wire at whatever layout the program was
//! encoded with). A bench that varies the record layout uses this interpreter
//! unchanged and only changes the encode layout; a bench that varies dispatch
//! or value representation swaps this function for an axis variant while keeping
//! everything else identical. Either way the program crosses into the routine
//! as wire bytes, so the interpretation can never be partially evaluated away.

use crate::ir::{op, Decoded};

/// Interpret the decoded program once with the given per-call input seed,
/// returning a cheap order-sensitive rolling hash of every node result.
///
/// `results` is a caller-owned scratch buffer of length at least `d.node_count`,
/// so the hot loop never allocates. Switch dispatch on the opcode. The hash is a
/// two-op xor-rotate per node, deliberately cheap: its only jobs are to keep the
/// result array live (so the interpretation cannot be dead-code-eliminated) and
/// to let the harness cross-validate that variants agree. It must not dominate
/// the per-node decode-and-dispatch cost the bench is trying to measure, which a
/// full FNV fold per node would.
#[inline]
pub fn interpret(d: &Decoded, input_seed: u64, results: &mut [u64]) -> u64 {
    let mut hash: u64 = 0;
    for i in 0..d.node_count {
        // Flat single-level match over all opcodes so the backend can emit one
        // jump table; a nested match (binary ops behind a second dispatch)
        // would make the switch pay two dispatches and lose to the
        // function-pointer table for the wrong reason. Each arm reads exactly
        // the operands its arity encodes.
        let opcode = d.op_at(i);
        // A macro (not a closure) so the operand reads expand inline and hold no
        // borrow of `results` across the later `results[i] = v` write.
        macro_rules! bin {
            ($k:expr) => {
                results[d.operand(i, $k, 2) as usize]
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
                if results[d.operand(i, 0, 3) as usize] != 0 {
                    results[d.operand(i, 1, 3) as usize]
                } else {
                    results[d.operand(i, 2, 3) as usize]
                }
            }
            op::NEG => results[d.operand(i, 0, 1) as usize].wrapping_neg(),
            op::NOT => !results[d.operand(i, 0, 1) as usize],
            _ => 0,
        };
        results[i] = v;
        hash = hash.rotate_left(7) ^ v;
    }
    hash
}

/// The dispatch axis: an indirect-threaded interpreter that dispatches each
/// opcode through a function-pointer table instead of a `match`. One indirect
/// call per node. This is the fair Rust alternative to the switch; the
/// guaranteed-tail-call ("threaded") shape Deegen relies on is not expressible
/// in Rust and lives in a Zig variant that consumes identical program bytes.
type OpFn = fn(&Decoded, usize, &[u64], u64) -> u64;

fn f_const(d: &Decoded, i: usize, _r: &[u64], _s: u64) -> u64 {
    d.const_at(d.operand(i, 0, 1) as usize)
}
fn f_input(_d: &Decoded, _i: usize, _r: &[u64], s: u64) -> u64 {
    s
}
fn f_neg(d: &Decoded, i: usize, r: &[u64], _s: u64) -> u64 {
    r[d.operand(i, 0, 1) as usize].wrapping_neg()
}
fn f_not(d: &Decoded, i: usize, r: &[u64], _s: u64) -> u64 {
    !r[d.operand(i, 0, 1) as usize]
}
fn f_select(d: &Decoded, i: usize, r: &[u64], _s: u64) -> u64 {
    if r[d.operand(i, 0, 3) as usize] != 0 {
        r[d.operand(i, 1, 3) as usize]
    } else {
        r[d.operand(i, 2, 3) as usize]
    }
}
macro_rules! binop {
    ($name:ident, $a:ident, $b:ident, $body:expr) => {
        fn $name(d: &Decoded, i: usize, r: &[u64], _s: u64) -> u64 {
            let $a = r[d.operand(i, 0, 2) as usize];
            let $b = r[d.operand(i, 1, 2) as usize];
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

/// Indirect-threaded variant of [`interpret`]: identical semantics, dispatch
/// through the function-pointer table. Used by the dispatch-shape bench.
#[inline]
pub fn interpret_fntable(d: &Decoded, input_seed: u64, results: &mut [u64]) -> u64 {
    let mut hash: u64 = 0;
    for i in 0..d.node_count {
        let opcode = d.op_at(i) as usize;
        let v = DISPATCH[opcode](d, i, results, input_seed);
        results[i] = v;
        hash = hash.rotate_left(7) ^ v;
    }
    hash
}

/// If-chain dispatch: the opcode is matched by an explicit linear if-else
/// cascade rather than a `match` (which the backend lowers to a jump table).
/// The match-lowering bench found an if-chain beating the jump table on M1 for
/// the branch-prediction reasons; this tests the same hypothesis inside the
/// interpreter. Identical semantics to [`interpret`]; the cascade is ordered by
/// ascending opcode. Written with explicit `if`/`else if` so the backend keeps
/// it as a branch chain.
#[inline]
pub fn interpret_ifchain(d: &Decoded, input_seed: u64, results: &mut [u64]) -> u64 {
    let mut hash: u64 = 0;
    for i in 0..d.node_count {
        let opcode = d.op_at(i);
        macro_rules! bin {
            ($k:expr) => {
                results[d.operand(i, $k, 2) as usize]
            };
        }
        let v = if opcode == op::INPUT {
            input_seed
        } else if opcode == op::CONST {
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
            if results[d.operand(i, 0, 3) as usize] != 0 {
                results[d.operand(i, 1, 3) as usize]
            } else {
                results[d.operand(i, 2, 3) as usize]
            }
        } else if opcode == op::NEG {
            results[d.operand(i, 0, 1) as usize].wrapping_neg()
        } else if opcode == op::NOT {
            !results[d.operand(i, 0, 1) as usize]
        } else {
            0
        };
        results[i] = v;
        hash = hash.rotate_left(7) ^ v;
    }
    hash
}

/// Drive the switch interpreter over a stream of input bytes, folding each
/// per-byte hash into one accumulator. The exact shape a variant's `timed!`
/// region runs: program structure fixed, input bytes vary the eval.
#[inline]
pub fn run_over_input(d: &Decoded, input: &[u8], results: &mut [u64]) -> u64 {
    let mut acc: u64 = 0;
    for &byte in input {
        acc ^= interpret(d, byte as u64, results);
    }
    acc
}

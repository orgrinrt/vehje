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

/// Function-pointer-table dispatch over the flat predecoded form (flat +
/// fntable), so the composition matrix has all three dispatch shapes on the flat
/// form. Identical semantics to [`interpret_predecoded`].
type PFn = fn(&Predecoded, usize, &[u64], u64) -> u64;

fn pf_const(p: &Predecoded, i: usize, _r: &[u64], _s: u64) -> u64 {
    p.consts[p.nodes[i].a as usize]
}
fn pf_input(_p: &Predecoded, _i: usize, _r: &[u64], s: u64) -> u64 {
    s
}
fn pf_neg(p: &Predecoded, i: usize, r: &[u64], _s: u64) -> u64 {
    r[p.nodes[i].a as usize].wrapping_neg()
}
fn pf_not(p: &Predecoded, i: usize, r: &[u64], _s: u64) -> u64 {
    !r[p.nodes[i].a as usize]
}
fn pf_select(p: &Predecoded, i: usize, r: &[u64], _s: u64) -> u64 {
    let nd = p.nodes[i];
    if r[nd.a as usize] != 0 {
        r[nd.b as usize]
    } else {
        r[nd.c as usize]
    }
}
macro_rules! pbinop {
    ($name:ident, $a:ident, $b:ident, $body:expr) => {
        fn $name(p: &Predecoded, i: usize, r: &[u64], _s: u64) -> u64 {
            let nd = p.nodes[i];
            let $a = r[nd.a as usize];
            let $b = r[nd.b as usize];
            $body
        }
    };
}
pbinop!(pf_add, a, b, a.wrapping_add(b));
pbinop!(pf_sub, a, b, a.wrapping_sub(b));
pbinop!(pf_mul, a, b, a.wrapping_mul(b));
pbinop!(pf_and, a, b, a & b);
pbinop!(pf_or, a, b, a | b);
pbinop!(pf_xor, a, b, a ^ b);
pbinop!(pf_shl, a, b, a.wrapping_shl(b as u32));
pbinop!(pf_shr, a, b, a.wrapping_shr(b as u32));
pbinop!(pf_min, a, b, a.min(b));
pbinop!(pf_max, a, b, a.max(b));
pbinop!(pf_eq, a, b, (a == b) as u64);
pbinop!(pf_lt, a, b, (a < b) as u64);

static PDISPATCH: [PFn; op::COUNT as usize] = [
    pf_const, pf_add, pf_sub, pf_mul, pf_and, pf_or, pf_xor, pf_shl, pf_shr, pf_min, pf_max, pf_eq,
    pf_lt, pf_select, pf_neg, pf_not, pf_input,
];

/// Function-pointer-table dispatch over the flat form.
#[inline]
pub fn interpret_predecoded_fntable(p: &Predecoded, input_seed: u64, results: &mut [u64]) -> u64 {
    let mut hash: u64 = 0;
    for i in 0..p.nodes.len() {
        let opcode = p.nodes[i].op as usize;
        let v = PDISPATCH[opcode](p, i, results, input_seed);
        results[i] = v;
        hash = hash.rotate_left(7) ^ v;
    }
    hash
}

/// Preserve-none context-threaded dispatch over the flat predecoded form: the
/// A1 threaded shape and the A2 flat form combined. Tests whether the smaller
/// flat working set rescues the threaded shape at large n, where wire-threaded
/// lost. Only compiled with the `threaded` feature.
#[cfg(feature = "threaded")]
pub mod threaded_flat {
    use super::PNode;
    use crate::ir::op;

    // State threaded in registers: node index, flat node pointer, const pool
    // pointer, results pointer, rolling hash, input seed, node count. Seven
    // arguments, all register-resident under preserve-none.
    type H = extern "rust-preserve-none" fn(usize, *const PNode, *const u64, *mut u64, u64, u64, usize) -> u64;

    macro_rules! advance {
        ($i:expr, $p:expr, $c:expr, $r:expr, $hash:expr, $seed:expr, $n:expr, $v:expr) => {{
            let i = $i;
            let v = $v;
            unsafe {
                *$r.add(i) = v;
            }
            let hash = $hash.rotate_left(7) ^ v;
            let ni = i + 1;
            if ni >= $n {
                return hash;
            }
            let nop = unsafe { (*$p.add(ni)).op } as usize;
            become TABLE[nop](ni, $p, $c, $r, hash, $seed, $n)
        }};
    }

    macro_rules! bin_h {
        ($name:ident, $x:ident, $y:ident, $body:expr) => {
            extern "rust-preserve-none" fn $name(i: usize, p: *const PNode, c: *const u64, r: *mut u64, hash: u64, seed: u64, n: usize) -> u64 {
                let nd = unsafe { *p.add(i) };
                let $x = unsafe { *r.add(nd.a as usize) };
                let $y = unsafe { *r.add(nd.b as usize) };
                advance!(i, p, c, r, hash, seed, n, $body)
            }
        };
    }

    bin_h!(h_add, a, b, a.wrapping_add(b));
    bin_h!(h_sub, a, b, a.wrapping_sub(b));
    bin_h!(h_mul, a, b, a.wrapping_mul(b));
    bin_h!(h_and, a, b, a & b);
    bin_h!(h_or, a, b, a | b);
    bin_h!(h_xor, a, b, a ^ b);
    bin_h!(h_shl, a, b, a.wrapping_shl(b as u32));
    bin_h!(h_shr, a, b, a.wrapping_shr(b as u32));
    bin_h!(h_min, a, b, a.min(b));
    bin_h!(h_max, a, b, a.max(b));
    bin_h!(h_eq, a, b, (a == b) as u64);
    bin_h!(h_lt, a, b, (a < b) as u64);

    extern "rust-preserve-none" fn h_const(i: usize, p: *const PNode, c: *const u64, r: *mut u64, hash: u64, seed: u64, n: usize) -> u64 {
        let nd = unsafe { *p.add(i) };
        let v = unsafe { *c.add(nd.a as usize) };
        advance!(i, p, c, r, hash, seed, n, v)
    }
    extern "rust-preserve-none" fn h_input(i: usize, p: *const PNode, c: *const u64, r: *mut u64, hash: u64, seed: u64, n: usize) -> u64 {
        advance!(i, p, c, r, hash, seed, n, seed)
    }
    extern "rust-preserve-none" fn h_neg(i: usize, p: *const PNode, c: *const u64, r: *mut u64, hash: u64, seed: u64, n: usize) -> u64 {
        let nd = unsafe { *p.add(i) };
        let v = unsafe { *r.add(nd.a as usize) }.wrapping_neg();
        advance!(i, p, c, r, hash, seed, n, v)
    }
    extern "rust-preserve-none" fn h_not(i: usize, p: *const PNode, c: *const u64, r: *mut u64, hash: u64, seed: u64, n: usize) -> u64 {
        let nd = unsafe { *p.add(i) };
        let v = !unsafe { *r.add(nd.a as usize) };
        advance!(i, p, c, r, hash, seed, n, v)
    }
    extern "rust-preserve-none" fn h_select(i: usize, p: *const PNode, c: *const u64, r: *mut u64, hash: u64, seed: u64, n: usize) -> u64 {
        let nd = unsafe { *p.add(i) };
        let v = unsafe {
            if *r.add(nd.a as usize) != 0 {
                *r.add(nd.b as usize)
            } else {
                *r.add(nd.c as usize)
            }
        };
        advance!(i, p, c, r, hash, seed, n, v)
    }

    static TABLE: [H; op::COUNT as usize] = [
        h_const, h_add, h_sub, h_mul, h_and, h_or, h_xor, h_shl, h_shr, h_min, h_max, h_eq, h_lt,
        h_select, h_neg, h_not, h_input,
    ];

    /// Threaded dispatch over the flat form. Same signature as
    /// [`super::interpret_predecoded`].
    #[inline]
    pub fn interpret_predecoded_threaded(p: &super::Predecoded, input_seed: u64, results: &mut [u64]) -> u64 {
        let n = p.nodes.len();
        if n == 0 {
            return 0;
        }
        let pp = p.nodes.as_ptr();
        let cp = p.consts.as_ptr();
        let rp = results.as_mut_ptr();
        let op0 = unsafe { (*pp).op } as usize;
        (TABLE[op0])(0, pp, cp, rp, 0, input_seed, n)
    }
}

#[cfg(feature = "threaded")]
pub use threaded_flat::interpret_predecoded_threaded;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{encode, REC24};
    use crate::{generate, GenParams};

    #[cfg(feature = "threaded")]
    #[test]
    fn predecoded_threaded_matches_switch() {
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
                super::interpret_predecoded_threaded(&p, seed, &mut r2),
                "predecoded-threaded diverged at seed {seed}"
            );
        }
    }

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

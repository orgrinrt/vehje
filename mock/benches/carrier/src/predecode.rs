//! Predecoded interpretation over a flat 16-byte-record form, one per dispatch
//! shape. Split decode from dispatch: predecode the wire program once into a flat
//! `PNode` array (outside the timed region, an honest `S` term), then run a tight
//! loop with no wire arithmetic.
//!
//! Same contract as `interp`: every interpreter fills the caller-owned `results`
//! and returns nothing; the caller folds one `access::checksum(results)` after
//! the pass. Every operand and const read and every result write goes through the
//! shared `access` primitive (unchecked, sound post-validation), identical across
//! all shapes, so the dispatch axis varies dispatch alone. The flat record is 16
//! bytes against the 24-byte wire record, so the predecoded form is also a
//! smaller working set.

use crate::access::{rload, rstore};
use crate::ir::{op, Decoded};

/// One predecoded node: opcode plus three resolved operand slots (node-result
/// indices, or a const-pool index for CONST). 16 bytes, four per cache line.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNode {
    pub op: u8,
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

/// A program predecoded into flat records plus the const pool.
pub struct Predecoded {
    pub nodes: Vec<PNode>,
    pub consts: Vec<u64>,
}

/// Predecode a wire program into the flat form. Reads every operand once.
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

// unchecked const load, matching the operand-access discipline.
#[inline(always)]
unsafe fn cload(base: *const u64, idx: u32) -> u64 {
    *base.add(idx as usize)
}

/// Switch dispatch over the flat form. Fills `results`.
#[inline]
pub fn interpret_predecoded(p: &Predecoded, input_seed: u64, results: &mut [u64]) {
    let wp = results.as_mut_ptr();
    let rp = wp as *const u64;
    let np = p.nodes.as_ptr();
    let cp = p.consts.as_ptr();
    for i in 0..p.nodes.len() {
        let nd = unsafe { *np.add(i) };
        let v = match nd.op {
            op::INPUT => input_seed,
            op::CONST => unsafe { cload(cp, nd.a) },
            op::ADD => unsafe { rload(rp, nd.a).wrapping_add(rload(rp, nd.b)) },
            op::SUB => unsafe { rload(rp, nd.a).wrapping_sub(rload(rp, nd.b)) },
            op::MUL => unsafe { rload(rp, nd.a).wrapping_mul(rload(rp, nd.b)) },
            op::AND => unsafe { rload(rp, nd.a) & rload(rp, nd.b) },
            op::OR => unsafe { rload(rp, nd.a) | rload(rp, nd.b) },
            op::XOR => unsafe { rload(rp, nd.a) ^ rload(rp, nd.b) },
            op::SHL => unsafe { rload(rp, nd.a).wrapping_shl(rload(rp, nd.b) as u32) },
            op::SHR => unsafe { rload(rp, nd.a).wrapping_shr(rload(rp, nd.b) as u32) },
            op::MIN => unsafe { rload(rp, nd.a).min(rload(rp, nd.b)) },
            op::MAX => unsafe { rload(rp, nd.a).max(rload(rp, nd.b)) },
            op::EQ => unsafe { (rload(rp, nd.a) == rload(rp, nd.b)) as u64 },
            op::LT => unsafe { (rload(rp, nd.a) < rload(rp, nd.b)) as u64 },
            op::SELECT => unsafe {
                if rload(rp, nd.a) != 0 {
                    rload(rp, nd.b)
                } else {
                    rload(rp, nd.c)
                }
            },
            op::NEG => unsafe { rload(rp, nd.a).wrapping_neg() },
            op::NOT => unsafe { !rload(rp, nd.a) },
            _ => 0,
        };
        unsafe { rstore(wp, i, v) };
    }
}

/// Function-pointer-table dispatch over the flat form.
type PFn = fn(&Predecoded, PNode, *const u64, u64) -> u64;

fn pf_const(p: &Predecoded, nd: PNode, _r: *const u64, _s: u64) -> u64 {
    unsafe { cload(p.consts.as_ptr(), nd.a) }
}
fn pf_input(_p: &Predecoded, _nd: PNode, _r: *const u64, s: u64) -> u64 {
    s
}
fn pf_neg(_p: &Predecoded, nd: PNode, r: *const u64, _s: u64) -> u64 {
    unsafe { rload(r, nd.a) }.wrapping_neg()
}
fn pf_not(_p: &Predecoded, nd: PNode, r: *const u64, _s: u64) -> u64 {
    !unsafe { rload(r, nd.a) }
}
fn pf_select(_p: &Predecoded, nd: PNode, r: *const u64, _s: u64) -> u64 {
    unsafe {
        if rload(r, nd.a) != 0 {
            rload(r, nd.b)
        } else {
            rload(r, nd.c)
        }
    }
}
macro_rules! pbinop {
    ($name:ident, $x:ident, $y:ident, $body:expr) => {
        fn $name(_p: &Predecoded, nd: PNode, r: *const u64, _s: u64) -> u64 {
            let $x = unsafe { rload(r, nd.a) };
            let $y = unsafe { rload(r, nd.b) };
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

/// Function-pointer-table dispatch over the flat form. Fills `results`.
#[inline]
pub fn interpret_predecoded_fntable(p: &Predecoded, input_seed: u64, results: &mut [u64]) {
    let wp = results.as_mut_ptr();
    let rp = wp as *const u64;
    let np = p.nodes.as_ptr();
    for i in 0..p.nodes.len() {
        let nd = unsafe { *np.add(i) };
        let v = PDISPATCH[nd.op as usize](p, nd, rp, input_seed);
        unsafe { rstore(wp, i, v) };
    }
}

/// Null-dispatch reference floor over the flat form: same loads and store, one
/// fixed op, no dispatch. Not cross-validated (see `interp::interpret_nulldispatch`).
#[inline]
pub fn interpret_predecoded_nulldispatch(p: &Predecoded, input_seed: u64, results: &mut [u64]) {
    let wp = results.as_mut_ptr();
    let rp = wp as *const u64;
    let np = p.nodes.as_ptr();
    for i in 0..p.nodes.len() {
        let nd = unsafe { *np.add(i) };
        // leaves (INPUT/CONST) have no node operands; guard to stay in-bounds.
        let v = if nd.op == op::INPUT {
            input_seed
        } else if nd.op == op::CONST || nd.op == op::NEG || nd.op == op::NOT {
            unsafe { rload(rp, nd.a) }
        } else {
            unsafe { rload(rp, nd.a).wrapping_add(rload(rp, nd.b)) }
        };
        unsafe { rstore(wp, i, v) };
    }
}

/// Preserve-none context-threaded dispatch over the flat form. Only compiled
/// with the `threaded` feature.
#[cfg(feature = "threaded")]
pub mod threaded_flat {
    use super::{cload, PNode};
    use crate::access::{rload, rstore};
    use crate::ir::op;

    // State threaded in registers under preserve-none: node index, flat node
    // pointer, const pointer, results write pointer, results read pointer, input
    // seed, node count. No hash is threaded (the checksum is a post-pass fold),
    // so each handler fills its slot and dispatches the next; the terminal
    // handler returns.
    type H = extern "rust-preserve-none" fn(usize, *const PNode, *const u64, *mut u64, u64, usize);

    macro_rules! advance {
        ($i:expr, $np:expr, $cp:expr, $wp:expr, $seed:expr, $n:expr, $v:expr) => {{
            let i = $i;
            unsafe {
                rstore($wp, i, $v);
            }
            let ni = i + 1;
            if ni >= $n {
                return;
            }
            let nop = unsafe { (*$np.add(ni)).op } as usize;
            become TABLE[nop](ni, $np, $cp, $wp, $seed, $n)
        }};
    }

    macro_rules! bin_h {
        ($name:ident, $x:ident, $y:ident, $body:expr) => {
            extern "rust-preserve-none" fn $name(i: usize, np: *const PNode, cp: *const u64, wp: *mut u64, seed: u64, n: usize) {
                let nd = unsafe { *np.add(i) };
                let rp = wp as *const u64;
                let $x = unsafe { rload(rp, nd.a) };
                let $y = unsafe { rload(rp, nd.b) };
                advance!(i, np, cp, wp, seed, n, $body)
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

    extern "rust-preserve-none" fn h_const(i: usize, np: *const PNode, cp: *const u64, wp: *mut u64, seed: u64, n: usize) {
        let nd = unsafe { *np.add(i) };
        let v = unsafe { cload(cp, nd.a) };
        advance!(i, np, cp, wp, seed, n, v)
    }
    extern "rust-preserve-none" fn h_input(i: usize, np: *const PNode, cp: *const u64, wp: *mut u64, seed: u64, n: usize) {
        advance!(i, np, cp, wp, seed, n, seed)
    }
    extern "rust-preserve-none" fn h_neg(i: usize, np: *const PNode, cp: *const u64, wp: *mut u64, seed: u64, n: usize) {
        let nd = unsafe { *np.add(i) };
        let v = unsafe { rload(wp as *const u64, nd.a) }.wrapping_neg();
        advance!(i, np, cp, wp, seed, n, v)
    }
    extern "rust-preserve-none" fn h_not(i: usize, np: *const PNode, cp: *const u64, wp: *mut u64, seed: u64, n: usize) {
        let nd = unsafe { *np.add(i) };
        let v = !unsafe { rload(wp as *const u64, nd.a) };
        advance!(i, np, cp, wp, seed, n, v)
    }
    extern "rust-preserve-none" fn h_select(i: usize, np: *const PNode, cp: *const u64, wp: *mut u64, seed: u64, n: usize) {
        let nd = unsafe { *np.add(i) };
        let rp = wp as *const u64;
        let v = unsafe {
            if rload(rp, nd.a) != 0 {
                rload(rp, nd.b)
            } else {
                rload(rp, nd.c)
            }
        };
        advance!(i, np, cp, wp, seed, n, v)
    }

    static TABLE: [H; op::COUNT as usize] = [
        h_const, h_add, h_sub, h_mul, h_and, h_or, h_xor, h_shl, h_shr, h_min, h_max, h_eq, h_lt,
        h_select, h_neg, h_not, h_input,
    ];

    /// Threaded dispatch over the flat form. Fills `results`.
    #[inline]
    pub fn interpret_predecoded_threaded(p: &super::Predecoded, input_seed: u64, results: &mut [u64]) {
        let n = p.nodes.len();
        if n == 0 {
            return;
        }
        let np = p.nodes.as_ptr();
        let cp = p.consts.as_ptr();
        let wp = results.as_mut_ptr();
        let op0 = unsafe { (*np).op } as usize;
        (TABLE[op0])(0, np, cp, wp, input_seed, n);
    }
}

#[cfg(feature = "threaded")]
pub use threaded_flat::interpret_predecoded_threaded;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::checksum;
    use crate::ir::{encode, REC24};
    use crate::{generate, GenParams};

    fn switch_ck(d: &Decoded, seed: u64, r: &mut [u64]) -> u64 {
        crate::interp::interpret(d, seed, r);
        checksum(r)
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
            let sw = switch_ck(&d, seed, &mut r1);
            interpret_predecoded(&p, seed, &mut r2);
            assert_eq!(sw, checksum(&r2), "predecoded switch diverged at seed {seed}");
            interpret_predecoded_fntable(&p, seed, &mut r2);
            assert_eq!(sw, checksum(&r2), "predecoded fntable diverged at seed {seed}");
        }
    }

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
            let sw = switch_ck(&d, seed, &mut r1);
            super::interpret_predecoded_threaded(&p, seed, &mut r2);
            assert_eq!(sw, checksum(&r2), "predecoded-threaded diverged at seed {seed}");
        }
    }
}

//! Preserve-none context-threaded interpreter over the wire form.
//!
//! `extern "rust-preserve-none"` (no callee-saved registers across the dispatch)
//! plus `become` (a guaranteed tail call) give a real context-threaded
//! interpreter: one handler per opcode, each filling its result slot and
//! `become`-dispatching the next node's handler, the state (node index, decoded
//! pointer, results pointer, input seed) staying in argument registers with no
//! per-node spill. `become` makes each dispatch a jump, so the chain runs in
//! constant stack.
//!
//! Same contract as the other interpreters: it fills `results` through the shared
//! `access` primitive (unchecked, identical to every other shape so the dispatch
//! axis varies dispatch alone), returns nothing, and the caller folds one
//! `access::checksum(results)` after the pass. No hash is threaded through the
//! chain (the checksum is a post-pass fold).

use crate::access::{rload, rstore};
use crate::ir::{op, Decoded};

/// One threaded handler. State: node index, opaque decoded-program pointer,
/// results write pointer, input seed. All register-resident under preserve-none.
type Handler = extern "rust-preserve-none" fn(usize, *const (), *mut u64, u64);

#[inline(always)]
unsafe fn view<'a>(d: *const ()) -> &'a Decoded<'a> {
    &*(d as *const Decoded<'a>)
}

macro_rules! advance {
    ($i:expr, $d:expr, $wp:expr, $seed:expr, $v:expr) => {{
        let i = $i;
        unsafe {
            rstore($wp, i, $v);
        }
        let ni = i + 1;
        let dec = unsafe { view($d) };
        if ni >= dec.node_count {
            return;
        }
        let nop = dec.op_at(ni) as usize;
        become TABLE[nop](ni, $d, $wp, $seed)
    }};
}

// each handler reads its two operands, then defers to the single `binop_body!`
// definition for the op's semantics (only the preserve-none tail-threaded
// skeleton is this cell's).
macro_rules! bin_h {
    ($name:ident, $op:ident) => {
        extern "rust-preserve-none" fn $name(i: usize, d: *const (), wp: *mut u64, seed: u64) {
            let dec = unsafe { view(d) };
            let rp = wp as *const u64;
            let a = unsafe { rload(rp, dec.operand(i, 0, 2)) };
            let b = unsafe { rload(rp, dec.operand(i, 1, 2)) };
            advance!(i, d, wp, seed, crate::ops::binop_body!($op, a, b))
        }
    };
}

bin_h!(h_add, ADD);
bin_h!(h_sub, SUB);
bin_h!(h_mul, MUL);
bin_h!(h_and, AND);
bin_h!(h_or, OR);
bin_h!(h_xor, XOR);
bin_h!(h_shl, SHL);
bin_h!(h_shr, SHR);
bin_h!(h_min, MIN);
bin_h!(h_max, MAX);
bin_h!(h_eq, EQ);
bin_h!(h_lt, LT);

extern "rust-preserve-none" fn h_const(i: usize, d: *const (), wp: *mut u64, seed: u64) {
    let dec = unsafe { view(d) };
    let v = dec.const_at(dec.operand(i, 0, 1) as usize);
    advance!(i, d, wp, seed, v)
}
extern "rust-preserve-none" fn h_input(i: usize, d: *const (), wp: *mut u64, seed: u64) {
    advance!(i, d, wp, seed, seed)
}
extern "rust-preserve-none" fn h_neg(i: usize, d: *const (), wp: *mut u64, seed: u64) {
    let dec = unsafe { view(d) };
    let v = crate::ops::unop_body!(NEG, unsafe { rload(wp as *const u64, dec.operand(i, 0, 1)) });
    advance!(i, d, wp, seed, v)
}
extern "rust-preserve-none" fn h_not(i: usize, d: *const (), wp: *mut u64, seed: u64) {
    let dec = unsafe { view(d) };
    let v = crate::ops::unop_body!(NOT, unsafe { rload(wp as *const u64, dec.operand(i, 0, 1)) });
    advance!(i, d, wp, seed, v)
}
extern "rust-preserve-none" fn h_select(i: usize, d: *const (), wp: *mut u64, seed: u64) {
    let dec = unsafe { view(d) };
    let rp = wp as *const u64;
    let v = unsafe {
        if rload(rp, dec.operand(i, 0, 3)) != 0 {
            rload(rp, dec.operand(i, 1, 3))
        } else {
            rload(rp, dec.operand(i, 2, 3))
        }
    };
    advance!(i, d, wp, seed, v)
}

static TABLE: [Handler; op::COUNT as usize] = [
    h_const, h_add, h_sub, h_mul, h_and, h_or, h_xor, h_shl, h_shr, h_min, h_max, h_eq, h_lt,
    h_select, h_neg, h_not, h_input,
];

/// Threaded-dispatch interpreter over the wire form. Fills `results`.
#[inline]
pub fn interpret_threaded(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    if d.node_count == 0 {
        return;
    }
    let dp = d as *const Decoded as *const ();
    let wp = results.as_mut_ptr();
    let op0 = d.op_at(0) as usize;
    (TABLE[op0])(0, dp, wp, input_seed);
}

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
    fn threaded_matches_switch() {
        let prog = generate(&GenParams {
            node_count: 400,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r1 = vec![0u64; prog.nodes.len()];
        let mut r2 = vec![0u64; prog.nodes.len()];
        for seed in [0u64, 1, 42, 255, 1000, 999_999] {
            let sw = switch_ck(&d, seed, &mut r1);
            interpret_threaded(&d, seed, &mut r2);
            assert_eq!(sw, checksum(&r2), "threaded diverged from switch at seed {seed}");
        }
    }

    #[test]
    fn threaded_constant_stack_deep() {
        let prog = generate(&GenParams {
            node_count: 20_000,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r1 = vec![0u64; prog.nodes.len()];
        let mut r2 = vec![0u64; prog.nodes.len()];
        crate::interp::interpret(&d, 7, &mut r1);
        interpret_threaded(&d, 7, &mut r2);
        assert_eq!(checksum(&r1), checksum(&r2));
    }
}

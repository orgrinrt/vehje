//! Preserve-none context-threaded interpreter: the Deegen-shape dispatch that
//! the switch/fn-table bench declared "not expressible in Rust".
//!
//! It is expressible. `extern "rust-preserve-none"` (LLVM's preserve_none
//! calling convention, no callee-saved registers) plus `become` (a guaranteed
//! tail call) give a real context-threaded interpreter: one handler per opcode,
//! each computing its node result and `become`-dispatching the next node's
//! handler. Because no registers are callee-saved across the dispatch, the
//! interpreter state (node index, decoded-program pointer, results pointer,
//! rolling hash, input seed) stays in argument registers across the whole chain
//! with no per-node spill/reload, which is the mechanism behind the threaded
//! shape's advantage. `become` makes each dispatch a jump, so the chain runs in
//! constant stack regardless of node count.
//!
//! Semantics are byte-identical to `interp::interpret`: the same operand decode
//! per arity, the same two-op xor-rotate hash fold, the same result writes. The
//! carrier cross-validation asserts the threaded checksum equals the switch
//! checksum, so the dispatch bench measures dispatch shape and nothing else.

use crate::ir::{op, Decoded};

/// One threaded handler. State travels in the five arguments (all in registers
/// under preserve-none): node index `i`, an opaque pointer to the `Decoded`
/// view, the results scratch pointer, the rolling `hash`, and the per-call
/// input `seed`. The decoded pointer is opaque (`*const ()`) to sidestep the
/// lifetime on `Decoded` across a raw dispatch table; every handler recovers the
/// borrow for the duration of its own call.
type Handler = extern "rust-preserve-none" fn(usize, *const (), *mut u64, u64, u64) -> u64;

/// Recover the decoded-program borrow from the opaque pointer. Sound for the
/// duration of the call: the `Decoded` outlives the entire threaded chain (it is
/// held by the driver's stack frame, which does not return until the chain does).
#[inline(always)]
unsafe fn view<'a>(d: *const ()) -> &'a Decoded<'a> {
    &*(d as *const Decoded<'a>)
}

/// Common tail: write this node's value, fold the hash, and dispatch the next
/// node's handler (or return the hash when the program is exhausted). Kept as a
/// macro so the `become` stays in each handler's own tail position (a helper
/// function call could not be a guaranteed tail call from inside the handler and
/// then dispatch onward).
macro_rules! advance {
    ($i:expr, $d:expr, $r:expr, $hash:expr, $seed:expr, $v:expr) => {{
        let i = $i;
        let d = $d;
        let r = $r;
        let v = $v;
        unsafe {
            *r.add(i) = v;
        }
        let hash = $hash.rotate_left(7) ^ v;
        let ni = i + 1;
        let dec = unsafe { view(d) };
        if ni >= dec.node_count {
            return hash;
        }
        let nop = dec.op_at(ni) as usize;
        become TABLE[nop](ni, d, r, hash, $seed)
    }};
}

/// Read binary operand `k` of node `i` from the results scratch (arity 2),
/// matching `interpret`'s `bin!` macro exactly.
macro_rules! bin {
    ($dec:expr, $r:expr, $i:expr, $k:expr) => {
        unsafe { *$r.add($dec.operand($i, $k, 2) as usize) }
    };
}

macro_rules! binary_handler {
    ($name:ident, $a:ident, $b:ident, $body:expr) => {
        extern "rust-preserve-none" fn $name(i: usize, d: *const (), r: *mut u64, hash: u64, seed: u64) -> u64 {
            let dec = unsafe { view(d) };
            let $a = bin!(dec, r, i, 0);
            let $b = bin!(dec, r, i, 1);
            advance!(i, d, r, hash, seed, $body)
        }
    };
}

binary_handler!(h_add, a, b, a.wrapping_add(b));
binary_handler!(h_sub, a, b, a.wrapping_sub(b));
binary_handler!(h_mul, a, b, a.wrapping_mul(b));
binary_handler!(h_and, a, b, a & b);
binary_handler!(h_or, a, b, a | b);
binary_handler!(h_xor, a, b, a ^ b);
binary_handler!(h_shl, a, b, a.wrapping_shl(b as u32));
binary_handler!(h_shr, a, b, a.wrapping_shr(b as u32));
binary_handler!(h_min, a, b, a.min(b));
binary_handler!(h_max, a, b, a.max(b));
binary_handler!(h_eq, a, b, (a == b) as u64);
binary_handler!(h_lt, a, b, (a < b) as u64);

extern "rust-preserve-none" fn h_const(i: usize, d: *const (), r: *mut u64, hash: u64, seed: u64) -> u64 {
    let dec = unsafe { view(d) };
    let v = dec.const_at(dec.operand(i, 0, 1) as usize);
    advance!(i, d, r, hash, seed, v)
}

extern "rust-preserve-none" fn h_input(i: usize, d: *const (), r: *mut u64, hash: u64, seed: u64) -> u64 {
    advance!(i, d, r, hash, seed, seed)
}

extern "rust-preserve-none" fn h_neg(i: usize, d: *const (), r: *mut u64, hash: u64, seed: u64) -> u64 {
    let dec = unsafe { view(d) };
    let v = unsafe { *r.add(dec.operand(i, 0, 1) as usize) }.wrapping_neg();
    advance!(i, d, r, hash, seed, v)
}

extern "rust-preserve-none" fn h_not(i: usize, d: *const (), r: *mut u64, hash: u64, seed: u64) -> u64 {
    let dec = unsafe { view(d) };
    let v = !unsafe { *r.add(dec.operand(i, 0, 1) as usize) };
    advance!(i, d, r, hash, seed, v)
}

extern "rust-preserve-none" fn h_select(i: usize, d: *const (), r: *mut u64, hash: u64, seed: u64) -> u64 {
    let dec = unsafe { view(d) };
    let v = unsafe {
        if *r.add(dec.operand(i, 0, 3) as usize) != 0 {
            *r.add(dec.operand(i, 1, 3) as usize)
        } else {
            *r.add(dec.operand(i, 2, 3) as usize)
        }
    };
    advance!(i, d, r, hash, seed, v)
}

/// Dispatch table indexed by opcode; order matches `ir::op`.
static TABLE: [Handler; op::COUNT as usize] = [
    h_const, h_add, h_sub, h_mul, h_and, h_or, h_xor, h_shl, h_shr, h_min, h_max, h_eq, h_lt,
    h_select, h_neg, h_not, h_input,
];

/// Threaded-dispatch variant of [`crate::interp::interpret`]: identical
/// semantics, dispatch by preserve-none guaranteed-tail-call chain. Same
/// signature as the switch interpreter so it slots into the dispatch bench
/// unchanged.
#[inline]
pub fn interpret_threaded(d: &Decoded, input_seed: u64, results: &mut [u64]) -> u64 {
    if d.node_count == 0 {
        return 0;
    }
    let dp = d as *const Decoded as *const ();
    let rp = results.as_mut_ptr();
    let op0 = d.op_at(0) as usize;
    (TABLE[op0])(0, dp, rp, 0, input_seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::REC24;
    use crate::{generate, GenParams};
    use crate::ir::encode;

    #[test]
    fn threaded_matches_switch() {
        // The whole point: threaded dispatch computes the byte-identical checksum
        // as the switch interpreter, over the same wire bytes, at every seed.
        let prog = generate(&GenParams {
            node_count: 400,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r1 = vec![0u64; prog.nodes.len()];
        let mut r2 = vec![0u64; prog.nodes.len()];
        for seed in [0u64, 1, 42, 255, 1000, 999_999] {
            let switch = crate::interp::interpret(&d, seed, &mut r1);
            let threaded = interpret_threaded(&d, seed, &mut r2);
            assert_eq!(switch, threaded, "threaded diverged from switch at seed {seed}");
        }
    }

    #[test]
    fn threaded_constant_stack_deep() {
        // A large program exercises the guaranteed-tail-call chain; without real
        // TCO this would overflow the stack. It must return, matching the switch.
        let prog = generate(&GenParams {
            node_count: 20_000,
            ..GenParams::default_point()
        });
        let bytes = encode(&prog, &REC24);
        let d = Decoded::parse(&bytes, REC24).unwrap();
        let mut r1 = vec![0u64; prog.nodes.len()];
        let mut r2 = vec![0u64; prog.nodes.len()];
        assert_eq!(
            crate::interp::interpret(&d, 7, &mut r1),
            interpret_threaded(&d, 7, &mut r2)
        );
    }
}

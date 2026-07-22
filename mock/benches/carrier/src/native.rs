//! The native-tier ceiling, done honestly.
//!
//! The audit found the old interp-vs-native ceiling bench measured 1.2x because
//! its two-instruction program was a comptime-visible constant, so the optimizer
//! partially evaluated the interpreter into native code and compared native to
//! native. The corrected figure, on the same workload with the program made
//! opaque, was about 10x. This module reproduces the corrected measurement in
//! the carrier's own shape.
//!
//! The program is a multiply-add chain: a shape simple enough that a
//! "compiled" native version can be written by hand, yet real scalar
//! compute-bound work. Three executions run the identical program: the switch
//! interpreter, the function-pointer-table interpreter, and [`native_madd`], the
//! shape-specialized native loop that does no per-node opcode dispatch (it is
//! what a compiler would emit for this program). All three write their node
//! values to a `results` array and let the caller fold one post-pass checksum,
//! so they cross-validate byte-exact AND carry the identical checksum cost shape.
//! The program crosses into every variant as wire bytes, so no variant's
//! optimizer can see it and partially evaluate it; the interpreter/native ratio
//! is therefore the honest dispatch-and-decode overhead, not an artifact.
//!
//! The checksum discipline matters for fairness here specifically: [`native_madd`]
//! is the denominator every profile's interpretation slope is normalized against,
//! so it must carry EXACTLY the per-node cost shape the interpreters carry, no
//! more. An earlier version folded the rolling hash inline in the madd loop (two
//! rotate-xors per chain step) while every interpreter had already moved its fold
//! out of the dispatch loop (`access.rs`), which loaded the ceiling with diluting
//! overhead the numerators no longer paid and compressed the reported ratio. It
//! now writes `results[node] = value` with no inline fold, exactly like
//! `interp::interpret`, and [`native_madd_over_input`] folds once per pass via
//! `access::checksum`, exactly like `interp::run_over_input`.

use crate::gen::Rng;
use crate::ir::{encode, op, Decoded, Layout, Node, Program};

/// Build a multiply-add chain program of `steps` MUL/ADD pairs. Layout: node 0
/// is INPUT; nodes 1..=2*steps are the CONST operands (a mul const then an add
/// const per step); the remaining nodes are the chain, alternating
/// `MUL(acc, mul_const)` then `ADD(mul, add_const)`. Children-before-parents
/// holds by construction.
pub fn madd_program(steps: usize) -> Program {
    let mut rng = Rng::new(0x3a1d_c4a1_0e5e_0001);
    let nconst = 2 * steps;
    let consts: Vec<u64> = (0..nconst).map(|_| rng.next_u64() | 1).collect();
    let mut nodes: Vec<Node> = Vec::with_capacity(1 + 2 * nconst);

    nodes.push(Node { op: op::INPUT, operands: vec![] }); // node 0
    for c in 0..nconst {
        // nodes 1..=nconst: CONST c
        nodes.push(Node { op: op::CONST, operands: vec![c as u32] });
    }
    // The chain. `acc` starts at node 0 (INPUT). Const node for pool index c is
    // node index 1 + c.
    let mut acc: u32 = 0;
    for s in 0..steps {
        let mul_c = (1 + 2 * s) as u32;
        let add_c = (1 + 2 * s + 1) as u32;
        let mul = nodes.len() as u32;
        nodes.push(Node { op: op::MUL, operands: vec![acc, mul_c] });
        let add = nodes.len() as u32;
        nodes.push(Node { op: op::ADD, operands: vec![mul, add_c] });
        acc = add;
    }
    Program { consts, nodes }
}

/// Wire bytes for a madd chain of `steps` steps at `layout`.
pub fn madd_bytes(steps: usize, layout: Layout) -> Vec<u8> {
    encode(&madd_program(steps), &layout)
}

/// The "compiled" native execution of a madd-chain program: no opcode dispatch.
/// It writes each node's value to `results` (INPUT, then every CONST, then the
/// alternating MUL/ADD chain) exactly the way `interp::interpret` does, with no
/// inline checksum fold, so it cross-validates byte-exact against the interpreter
/// AND carries the identical per-node cost shape. It runs the chain as a straight
/// scalar loop, which is what native code for this program would be. `steps` is
/// derived from the node count.
#[inline]
pub fn native_madd(d: &Decoded, input_seed: u64, results: &mut [u64]) {
    // node_count = 1 + 2*nconst_pairs... layout: 1 INPUT + 2*steps CONST +
    // 2*steps chain = 1 + 4*steps. So steps = (node_count - 1) / 4.
    let steps = (d.node_count - 1) / 4;
    let nconst = 2 * steps;

    // node 0: INPUT
    results[0] = input_seed;

    // nodes 1..=nconst: CONST, written in node order (parity with the
    // interpreter, which stores every CONST node before any chain node). The
    // indexed load from the wire (`operand(i, 0, 1)` is the pool index) is exactly
    // what the interpreter's CONST handler does.
    for i in 1..=nconst {
        results[i] = d.const_at(d.operand(i, 0, 1) as usize);
    }

    // the chain, run as a straight native scalar loop with no dispatch, writing
    // each MUL and ADD result to its node slot. Chain node 0 is index nconst+1
    // (node 0 INPUT, then nconst CONST). No inline fold: the caller folds one
    // post-pass checksum over `results`, exactly like `run_over_input`, so the
    // ceiling carries the same checksum cost as the interpreters and no more. acc
    // starts at the runtime input_seed so the optimizer cannot fold the chain to a
    // constant, and the caller's observed checksum keeps the stores alive.
    let chain0 = nconst + 1;
    let mut acc = input_seed;
    for s in 0..steps {
        let mul_c = d.const_at(d.operand(1 + 2 * s, 0, 1) as usize);
        let add_c = d.const_at(d.operand(1 + 2 * s + 1, 0, 1) as usize);
        let mul = acc.wrapping_mul(mul_c);
        let add = mul.wrapping_add(add_c);
        results[chain0 + 2 * s] = mul;
        results[chain0 + 2 * s + 1] = add;
        acc = add;
    }
}

/// Drive the native madd loop over a stream of input bytes, folding one post-pass
/// checksum per evaluation. The native-ceiling analogue of
/// `interp::run_over_input`: identical checksum-out-of-loop cost shape, so the
/// interp-vs-native ratio is pure dispatch-and-decode overhead, not a difference
/// in checksum accounting.
#[inline]
pub fn native_madd_over_input(d: &Decoded, input: &[u8], results: &mut [u64]) -> u64 {
    let mut acc: u64 = 0;
    for &byte in input {
        native_madd(d, byte as u64, results);
        acc ^= crate::access::checksum(results);
    }
    acc
}

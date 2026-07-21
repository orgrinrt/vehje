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
//! what a compiler would emit for this program). All three fold the same rolling
//! hash and cross-validate byte-exact. The program crosses into every variant as
//! wire bytes, so no variant's optimizer can see it and partially evaluate it;
//! the interpreter/native ratio is therefore the honest dispatch-and-decode
//! overhead, not an artifact.

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
/// It reads the same const values from the wire and folds the same rolling hash
/// as the interpreter, so it cross-validates byte-exact, but it runs the chain
/// as a straight scalar loop, which is what native code for this program would
/// be. The `steps` is derived from the node count.
#[inline]
pub fn native_madd(d: &Decoded, input_seed: u64) -> u64 {
    // node_count = 1 + 2*nconst_pairs... layout: 1 INPUT + 2*steps CONST +
    // 2*steps chain = 1 + 4*steps. So steps = (node_count - 1) / 4.
    let steps = (d.node_count - 1) / 4;
    let nconst = 2 * steps;
    let mut hash: u64 = 0;

    // node 0: INPUT
    hash = hash.rotate_left(7) ^ input_seed;

    // nodes 1..=nconst: CONST. Fold each const value in node order (parity with
    // the interpreter, which folds every CONST node before any chain node). No
    // heap: a real compiled madd would hold these as immediates. Folding from the
    // wire directly is the strongest honest native shape, and it avoids the
    // per-call Vec alloc + nconst stores that were penalising the baseline and
    // understating the interp/native ratio.
    for i in 1..=nconst {
        let v = d.const_at(d.operand(i, 0, 1) as usize);
        hash = hash.rotate_left(7) ^ v;
    }

    // the chain, run as a straight native scalar loop with no dispatch. Each
    // step reads its two consts inline from the wire (const node 1+2s and
    // 1+2s+1; operand 0 is the pool index), exactly the indexed load the
    // interpreter's CONST handler does, so no register-vs-heap asymmetry remains.
    // black_box the seed so the optimizer cannot fold the whole chain to a
    // constant even if it somehow saw the consts.
    let mut acc = core::hint::black_box(input_seed);
    for s in 0..steps {
        let mul_c = d.const_at(d.operand(1 + 2 * s, 0, 1) as usize);
        let add_c = d.const_at(d.operand(1 + 2 * s + 1, 0, 1) as usize);
        let mul = acc.wrapping_mul(mul_c);
        hash = hash.rotate_left(7) ^ mul;
        let add = mul.wrapping_add(add_c);
        hash = hash.rotate_left(7) ^ add;
        acc = add;
    }
    hash
}

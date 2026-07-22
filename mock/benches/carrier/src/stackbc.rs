//! Stack-bytecode residual encoding.
//!
//! The carrier's default residual is a register/SSA value graph (a node reads its
//! operands by index). A stack machine is a genuinely different encoding: operands
//! are implicit on a value stack, ops pop and push. Which residual encoding
//! interprets fastest is an open question the runtime's baseline-form choice rests
//! on, so this cell measures the stack shape against the register shape on the
//! same program.
//!
//! Compilation is the straightforward every-value-gets-a-local scheme (like Wasm
//! or the JVM): each node computes onto the stack via loads of its operands' locals
//! and stores its result to its own local. This is representative of a stack
//! bytecode without a stack-scheduling optimizer (which would keep single-use
//! values on the stack); that optimization is a labelled refinement, not built here.
//! The bytecode is longer than the node stream (a load per operand plus a store per
//! node), so this cell measures whether the stack encoding's implicit-operand
//! dispatch pays for its higher instruction count. Cross-validated on the live-out
//! (local) values.

use crate::ir::{op, Program};

/// One stack-bytecode instruction.
#[derive(Clone, Copy)]
pub enum Bc {
    /// push consts[idx]
    Const(u32),
    /// push the input seed
    Input,
    /// push locals[slot]
    Load(u32),
    /// pop, store to locals[slot]
    Store(u32),
    /// pop b, pop a, push bin(op, a, b)
    Bin(u8),
    /// pop a, push -a
    Neg,
    /// pop a, push !a
    Not,
    /// pop c, pop b, pop a, push (a != 0 ? b : c)
    Select,
}

/// A stack-bytecode program: the instruction stream, the const pool, the number
/// of locals, and the local slots holding the live-out values.
pub struct StackProgram {
    pub code: Vec<Bc>,
    pub consts: Vec<u64>,
    pub num_locals: usize,
    pub out_locals: Vec<u32>,
}

/// Compile a value-graph program to stack bytecode. Each node's local is its
/// index; every node loads its operands' locals, computes, and stores its local.
pub fn compile(prog: &Program) -> StackProgram {
    let n = prog.nodes.len();
    let mut code = Vec::with_capacity(n * 3);
    for (i, node) in prog.nodes.iter().enumerate() {
        match node.op {
            op::INPUT => code.push(Bc::Input),
            op::CONST => code.push(Bc::Const(node.operands[0])),
            op::NEG => {
                code.push(Bc::Load(node.operands[0]));
                code.push(Bc::Neg);
            }
            op::NOT => {
                code.push(Bc::Load(node.operands[0]));
                code.push(Bc::Not);
            }
            op::SELECT => {
                code.push(Bc::Load(node.operands[0]));
                code.push(Bc::Load(node.operands[1]));
                code.push(Bc::Load(node.operands[2]));
                code.push(Bc::Select);
            }
            other => {
                code.push(Bc::Load(node.operands[0]));
                code.push(Bc::Load(node.operands[1]));
                code.push(Bc::Bin(other));
            }
        }
        code.push(Bc::Store(i as u32));
    }
    let out_locals = crate::optimize::sinks(prog);
    StackProgram {
        code,
        consts: prog.consts.clone(),
        num_locals: n.max(1),
        out_locals,
    }
}

#[inline(always)]
fn bin(opcode: u8, a: u64, b: u64) -> u64 {
    use crate::ops::binop_body;
    match opcode {
        op::ADD => binop_body!(ADD, a, b),
        op::SUB => binop_body!(SUB, a, b),
        op::MUL => binop_body!(MUL, a, b),
        op::AND => binop_body!(AND, a, b),
        op::OR => binop_body!(OR, a, b),
        op::XOR => binop_body!(XOR, a, b),
        op::SHL => binop_body!(SHL, a, b),
        op::SHR => binop_body!(SHR, a, b),
        op::MIN => binop_body!(MIN, a, b),
        op::MAX => binop_body!(MAX, a, b),
        op::EQ => binop_body!(EQ, a, b),
        op::LT => binop_body!(LT, a, b),
        _ => 0,
    }
}

/// Interpret the stack bytecode. `stack` and `locals` are caller-owned scratch;
/// `stack` must hold at least the program's max stack depth (3 is enough here,
/// but callers size it generously), `locals` at least `num_locals`. Fills
/// `locals`; the caller folds `out_locals`.
#[inline]
pub fn interpret_stack(sp: &StackProgram, input_seed: u64, stack: &mut [u64], locals: &mut [u64]) {
    let sbase = stack.as_mut_ptr();
    let lbase = locals.as_mut_ptr();
    let cp = sp.consts.as_ptr();
    let mut sp_top: usize = 0; // stack pointer (next free slot)
    for bc in &sp.code {
        match *bc {
            Bc::Const(idx) => unsafe {
                *sbase.add(sp_top) = *cp.add(idx as usize);
                sp_top += 1;
            },
            Bc::Input => unsafe {
                *sbase.add(sp_top) = input_seed;
                sp_top += 1;
            },
            Bc::Load(slot) => unsafe {
                *sbase.add(sp_top) = *lbase.add(slot as usize);
                sp_top += 1;
            },
            Bc::Store(slot) => unsafe {
                sp_top -= 1;
                *lbase.add(slot as usize) = *sbase.add(sp_top);
            },
            Bc::Bin(opcode) => unsafe {
                let b = *sbase.add(sp_top - 1);
                let a = *sbase.add(sp_top - 2);
                sp_top -= 1;
                *sbase.add(sp_top - 1) = bin(opcode, a, b);
            },
            Bc::Neg => unsafe {
                let a = *sbase.add(sp_top - 1);
                *sbase.add(sp_top - 1) = a.wrapping_neg();
            },
            Bc::Not => unsafe {
                let a = *sbase.add(sp_top - 1);
                *sbase.add(sp_top - 1) = !a;
            },
            Bc::Select => unsafe {
                let c = *sbase.add(sp_top - 1);
                let b = *sbase.add(sp_top - 2);
                let a = *sbase.add(sp_top - 3);
                sp_top -= 2;
                *sbase.add(sp_top - 1) = if a != 0 { b } else { c };
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::checksum_at;
    use crate::ir::{encode, Decoded, REC24};
    use crate::optimize::sinks;
    use crate::{generate, GenParams};

    #[test]
    fn stack_preserves_outputs() {
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 700;
            let prog = generate(&gp);
            let orig_sinks = sinks(&prog);
            let sp = compile(&prog);
            let bytes = encode(&prog, &REC24);
            let d = Decoded::parse(&bytes, REC24).unwrap();
            let mut refr = vec![0u64; prog.nodes.len()];
            let mut stack = vec![0u64; 8];
            let mut locals = vec![0u64; sp.num_locals];
            for seed in [0u64, 1, 42, 12345, 999_999] {
                crate::interp::interpret(&d, seed, &mut refr);
                interpret_stack(&sp, seed, &mut stack, &mut locals);
                assert_eq!(
                    checksum_at(&refr, &orig_sinks),
                    checksum_at(&locals, &sp.out_locals),
                    "{name} stack output diverged at seed {seed}"
                );
            }
        }
    }
}

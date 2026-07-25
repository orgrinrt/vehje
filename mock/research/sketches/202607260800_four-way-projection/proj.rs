//! The Rust half of the four-way projection sketch.
//!
//! The second read's admissibility test says a representation must be evaluable
//! by rustc-compiled Rust as well as by Zig, because the binding-time lattice has
//! discharge sites on the Rust side (`vehje-lower/src/lib.rs:126-128`, the
//! Bundler const-fold; `:92-94`, HostLoader load-time lowering). If Rust cannot
//! evaluate the encoding, the const-fold has to be authored separately from the
//! runtime, and the meaning is written twice.
//!
//! So the question here is narrow and sharp: does the SAME byte encoding fold at
//! rustc compile time. A `const` item is the proof, because a `const` that does
//! not fold is a compile error rather than a slow path.
//!
//! No allocation, no std beyond the prelude; the stack is a fixed array, matching
//! the Zig side and the no-alloc axiom.

const PUSH_ARG: u8 = 0x01;
const ADD: u8 = 0x10;
const LT: u8 = 0x11;
const MUL: u8 = 0x12;
const SUB: u8 = 0x13;

const STACK: usize = 16;

/// Bundler grade: the program is data, evaluated by rustc at compile time when
/// its inputs are known, and at runtime when they are not. One body serves both,
/// which is the property under test.
///
/// Not a dynamic code evaluator despite the name. The opcode set is closed and
/// fixed at five entries, the operand of `PUSH_ARG` indexes a caller-provided
/// slice, and an unrecognised byte panics, which in a `const` context is a
/// compile error. Nothing here can execute anything the vocabulary above does
/// not already name.
const fn eval(prog: &[u8], args: &[i64]) -> i64 {
    let mut stack = [0i64; STACK];
    let mut sp = 0usize;
    let mut pc = 0usize;
    while pc < prog.len() {
        match prog[pc] {
            PUSH_ARG => {
                stack[sp] = args[prog[pc + 1] as usize];
                sp += 1;
                pc += 2;
            }
            ADD => {
                stack[sp - 2] = stack[sp - 2] + stack[sp - 1];
                sp -= 1;
                pc += 1;
            }
            SUB => {
                stack[sp - 2] = stack[sp - 2] - stack[sp - 1];
                sp -= 1;
                pc += 1;
            }
            MUL => {
                stack[sp - 2] = stack[sp - 2] * stack[sp - 1];
                sp -= 1;
                pc += 1;
            }
            LT => {
                stack[sp - 2] = if stack[sp - 2] < stack[sp - 1] { 1 } else { 0 };
                sp -= 1;
                pc += 1;
            }
            _ => panic!("unknown opcode in operation program"),
        }
    }
    stack[0]
}

// The same bytes the Zig side consumes. In the real stage these are emitted by
// `vehje-runtime-gen` rather than written here; the sketch hardcodes them so the
// two sides are provably reading one definition and not two.
const OP_ADD: [u8; 5] = [PUSH_ARG, 0, PUSH_ARG, 1, ADD];
const OP_LT: [u8; 5] = [PUSH_ARG, 0, PUSH_ARG, 1, LT];
const OP_MUL: [u8; 5] = [PUSH_ARG, 0, PUSH_ARG, 1, MUL];
const OP_DIFFSQ: [u8; 11] = [
    PUSH_ARG, 0, PUSH_ARG, 1, SUB, PUSH_ARG, 0, PUSH_ARG, 1, ADD, MUL,
];

// The proof of Bundler-grade discharge: these are `const`, so if `eval` did not
// fold at compile time this file would not compile at all. A runtime evaluation
// that happened to return the right number would not satisfy this.
const FOLDED_ADD: i64 = eval(&OP_ADD, &[5, 3]);
const FOLDED_LT: i64 = eval(&OP_LT, &[5, 3]);
const FOLDED_MUL: i64 = eval(&OP_MUL, &[5, 3]);
const FOLDED_DIFFSQ: i64 = eval(&OP_DIFFSQ, &[5, 3]);

// And the const-eval is exact, not approximate: these assert at compile time.
const _: () = assert!(FOLDED_ADD == 8);
const _: () = assert!(FOLDED_LT == 0);
const _: () = assert!(FOLDED_MUL == 15);
const _: () = assert!(FOLDED_DIFFSQ == 16);

const CORPUS: [[i64; 2]; 10] = [
    [0, 0],
    [1, 0],
    [0, 1],
    [2, 3],
    [-1, 1],
    [7, 7],
    [-5, -9],
    [100, 3],
    [1 << 20, 3],
    [-3, 1 << 10],
];

fn main() {
    // Runtime discharge of the same body, over the same corpus, in the same
    // output shape the Zig side emits, so the two are diffed rather than eyeballed.
    for c in CORPUS.iter() {
        let args = [c[0], c[1]];
        println!(
            "{} {} {} {} {} {}",
            c[0],
            c[1],
            eval(&OP_ADD, &args),
            eval(&OP_LT, &args),
            eval(&OP_MUL, &args),
            eval(&OP_DIFFSQ, &args),
        );
    }
}

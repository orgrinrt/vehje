//! Direct native code generation (the near-native tier, template-compiler shape).
//!
//! Every other cell interprets. This one lowers the program to native aarch64
//! code and runs it directly, no dispatch at all. Codegen is a single linear pass
//! that HAND-SELECTS an instruction sequence per node (a `match node.op` that
//! emits the right encoders) and concatenates them into one executable function.
//! Interpretation's per-node dispatch vanishes; the price is a codegen `S` term
//! (an instruction-selection pass over every node) paid once before the run.
//!
//! This is a template / baseline native-code generator (in spirit like an early
//! full-codegen or a baseline JIT tier), NOT the stencil-extraction mechanism of
//! copy-and-patch (Xu and Kjolstad, OOPSLA 2021). Copy-and-patch precompiles a
//! fixed per-op stencil through a real backend and codegens by `memcpy`-ing that
//! template and patching its recorded holes, so the JIT author never selects
//! instructions per node. That mechanism lives in `stencil.rs` as its own cell
//! (`carrier_copypatch_stencil`); this cell is the direct-codegen comparator.
//! The distinction is a real cost-shape difference the matrix measures: this
//! cell's per-node codegen does instruction selection, the stencil cell's does a
//! copy plus a few immediate patches. Naming them apart keeps each number honest
//! about the mechanism it measures.
//!
//! The generated function has signature `fn(results: *mut u64, consts: *const
//! u64, seed: u64)`. For node `i`: arithmetic ops load their operands from
//! `results[operand]`, compute in registers, and store to `results[i]`; a CONST
//! loads from `consts[operand]`; an INPUT moves the seed. So the observable
//! output is byte-identical to the interpreters (same `results` array), which is
//! the cross-validation: the JIT checksum must equal the interpreter checksum.
//!
//! Addressing has two paths, chosen per index. Below 4096 (the aarch64 imm12
//! unsigned-offset window at 8-byte scale) a load/store uses the immediate form
//! directly, same code shape as before. At or above 4096 the index is
//! materialized into the scratch register x6 (`MOVZ`, plus `MOVK #16` for the
//! upper half when the index exceeds `0xFFFF`) and the access uses the
//! register-offset form `LDR/STR Xt, [Xn, X6, LSL #3]` instead. Both forms
//! compute the identical byte address; there is no size at which `emit` declines
//! (any `u32` node or const index is representable). The cell is gated to
//! aarch64 + macOS (the JIT path here is Apple-Silicon-specific: MAP_JIT plus
//! `pthread_jit_write_protect_np` plus `sys_icache_invalidate`); the technique is
//! platform-portable, this one realization is not, and that is marked, not hidden.
//! There is no fairness-vs-dispatch question here (there is no dispatch); the
//! fairness contract is semantic, the generated code computes the identical
//! results array, checked byte-exact against the interpreter.

#![cfg(all(feature = "jit", target_arch = "aarch64", target_os = "macos"))]

use crate::ir::{op, Program};
use core::ffi::c_void;

// aarch64 condition codes used by the comparison-producing ops.
const EQ: u32 = 0;
const NE: u32 = 1;
const HS: u32 = 2; // unsigned higher-or-same
const LO: u32 = 3; // unsigned lower
const LS: u32 = 9; // unsigned lower-or-same

// The scratch register used to materialize an out-of-imm12-range element
// index before a register-offset load/store. Free in every stencil here: x0
// = results, x1 = consts, x2 = seed, x3/x4/x5 = value scratch.
const SCRATCH_IDX: u32 = 6;

// Instruction encoders. Each returns one 32-bit aarch64 word. The register
// numbers are 0..31 (31 is XZR/SP depending on position). Verified against the
// ARM ARM encodings; the operand fields (Rd/Rn/Rm, imm12) are the "patch" slots.
#[inline]
fn ldr(rt: u32, rn: u32, off_words: u32) -> u32 {
    0xF940_0000 | ((off_words & 0xFFF) << 10) | (rn << 5) | rt
}
#[inline]
fn str_(rt: u32, rn: u32, off_words: u32) -> u32 {
    0xF900_0000 | ((off_words & 0xFFF) << 10) | (rn << 5) | rt
}
#[inline]
fn ldr_reg(rt: u32, rn: u32, rm: u32) -> u32 {
    // LDR Xt, [Xn, Xm, LSL #3]. Register-offset, 64-bit variant, S=1 (the
    // implicit shift by log2(8)=3), option=011 (LSL, plain register offset).
    0xF860_7800 | (rm << 16) | (rn << 5) | rt
}
#[inline]
fn str_reg(rt: u32, rn: u32, rm: u32) -> u32 {
    // STR Xt, [Xn, Xm, LSL #3]. Same shape as ldr_reg with opc=00 (store).
    0xF820_7800 | (rm << 16) | (rn << 5) | rt
}
#[inline]
fn movz(rd: u32, imm16: u32) -> u32 {
    // MOVZ Xd, #imm16 (hw=0, shift 0). Zeros the rest of the register.
    0xD280_0000 | ((imm16 & 0xFFFF) << 5) | rd
}
#[inline]
fn movk16(rd: u32, imm16: u32) -> u32 {
    // MOVK Xd, #imm16, LSL #16 (hw=1). Keeps the low 16 bits already placed
    // by a prior MOVZ and inserts this into bits [31:16].
    0xF2A0_0000 | ((imm16 & 0xFFFF) << 5) | rd
}
#[inline]
fn add(rd: u32, rn: u32, rm: u32) -> u32 {
    0x8B00_0000 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn sub(rd: u32, rn: u32, rm: u32) -> u32 {
    0xCB00_0000 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn mul(rd: u32, rn: u32, rm: u32) -> u32 {
    // MADD Xd, Xn, Xm, XZR
    0x9B00_0000 | (rm << 16) | (31 << 10) | (rn << 5) | rd
}
#[inline]
fn and_(rd: u32, rn: u32, rm: u32) -> u32 {
    0x8A00_0000 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn orr(rd: u32, rn: u32, rm: u32) -> u32 {
    0xAA00_0000 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn eor(rd: u32, rn: u32, rm: u32) -> u32 {
    0xCA00_0000 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn lslv(rd: u32, rn: u32, rm: u32) -> u32 {
    0x9AC0_2000 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn lsrv(rd: u32, rn: u32, rm: u32) -> u32 {
    0x9AC0_2400 | (rm << 16) | (rn << 5) | rd
}
#[inline]
fn neg(rd: u32, rm: u32) -> u32 {
    sub(rd, 31, rm) // SUB Xd, XZR, Xm
}
#[inline]
fn mvn(rd: u32, rm: u32) -> u32 {
    0xAA20_0000 | (rm << 16) | (31 << 5) | rd // ORN Xd, XZR, Xm
}
#[inline]
fn cmp_reg(rn: u32, rm: u32) -> u32 {
    0xEB00_0000 | (rm << 16) | (rn << 5) | 31 // SUBS XZR, Xn, Xm
}
#[inline]
fn cmp_zero(rn: u32) -> u32 {
    0xF100_001F | (rn << 5) // SUBS XZR, Xn, #0
}
#[inline]
fn csel(rd: u32, rn: u32, rm: u32, cond: u32) -> u32 {
    0x9A80_0000 | (rm << 16) | (cond << 12) | (rn << 5) | rd
}
#[inline]
fn cset(rd: u32, cond: u32) -> u32 {
    // CSINC Xd, XZR, XZR, invert(cond)
    0x9A80_0400 | (31 << 16) | ((cond ^ 1) << 12) | (31 << 5) | rd
}
#[inline]
fn mov_reg(rd: u32, rm: u32) -> u32 {
    orr(rd, 31, rm) // ORR Xd, XZR, Xm
}
#[inline]
fn ret() -> u32 {
    0xD65F_03C0
}

/// Emit a load of `results[idx]` (or `consts[idx]` when `rn` is the consts
/// base) into `rt`. Below the imm12 window this is a single immediate-offset
/// `LDR`; at or above it, `idx` is materialized into the scratch register
/// (`MOVZ`, plus `MOVK` for the upper half past `0xFFFF`) and the load uses
/// the register-offset form. Both paths read the identical byte address.
#[inline]
fn emit_ldr(code: &mut Vec<u32>, rt: u32, rn: u32, idx: u32) {
    if idx < 4096 {
        code.push(ldr(rt, rn, idx));
    } else {
        code.push(movz(SCRATCH_IDX, idx & 0xFFFF));
        if idx > 0xFFFF {
            code.push(movk16(SCRATCH_IDX, idx >> 16));
        }
        code.push(ldr_reg(rt, rn, SCRATCH_IDX));
    }
}

/// Emit a store of `rt` into `results[idx]`. Mirrors `emit_ldr`'s fast/slow
/// split; see there for the addressing rationale.
#[inline]
fn emit_str(code: &mut Vec<u32>, rt: u32, rn: u32, idx: u32) {
    if idx < 4096 {
        code.push(str_(rt, rn, idx));
    } else {
        code.push(movz(SCRATCH_IDX, idx & 0xFFFF));
        if idx > 0xFFFF {
            code.push(movk16(SCRATCH_IDX, idx >> 16));
        }
        code.push(str_reg(rt, rn, SCRATCH_IDX));
    }
}

/// Emit the machine-code words for a program. Registers: x0 = results, x1 =
/// consts, x2 = seed; x3/x4/x5 = value scratch, x6 = index-materialization
/// scratch (used only when a node or const index is at or above the imm12
/// window; see `emit_ldr` / `emit_str`).
pub fn emit(prog: &Program) -> Option<Vec<u32>> {
    let mut code: Vec<u32> = Vec::with_capacity(prog.nodes.len() * 5 + 1);
    for (i, node) in prog.nodes.iter().enumerate() {
        let i = i as u32;
        let ops = &node.operands;
        match node.op {
            op::CONST => emit_ldr(&mut code, 3, 1, ops[0]),
            op::INPUT => code.push(mov_reg(3, 2)),
            op::NEG => {
                emit_ldr(&mut code, 3, 0, ops[0]);
                code.push(neg(3, 3));
            }
            op::NOT => {
                emit_ldr(&mut code, 3, 0, ops[0]);
                code.push(mvn(3, 3));
            }
            op::SELECT => {
                emit_ldr(&mut code, 3, 0, ops[0]);
                emit_ldr(&mut code, 4, 0, ops[1]);
                emit_ldr(&mut code, 5, 0, ops[2]);
                code.push(cmp_zero(3));
                code.push(csel(3, 4, 5, NE)); // ops[0] != 0 ? ops[1] : ops[2]
            }
            other => {
                // binary ops: load both operands into x3, x4.
                emit_ldr(&mut code, 3, 0, ops[0]);
                emit_ldr(&mut code, 4, 0, ops[1]);
                match other {
                    op::ADD => code.push(add(3, 3, 4)),
                    op::SUB => code.push(sub(3, 3, 4)),
                    op::MUL => code.push(mul(3, 3, 4)),
                    op::AND => code.push(and_(3, 3, 4)),
                    op::OR => code.push(orr(3, 3, 4)),
                    op::XOR => code.push(eor(3, 3, 4)),
                    op::SHL => code.push(lslv(3, 3, 4)), // mod-64, matches wrapping_shl(u64)
                    op::SHR => code.push(lsrv(3, 3, 4)),
                    op::MIN => {
                        code.push(cmp_reg(3, 4));
                        code.push(csel(3, 3, 4, LS)); // a <= b ? a : b (unsigned)
                    }
                    op::MAX => {
                        code.push(cmp_reg(3, 4));
                        code.push(csel(3, 3, 4, HS)); // a >= b ? a : b (unsigned)
                    }
                    op::EQ => {
                        code.push(cmp_reg(3, 4));
                        code.push(cset(3, EQ));
                    }
                    op::LT => {
                        code.push(cmp_reg(3, 4));
                        code.push(cset(3, LO)); // unsigned less-than
                    }
                    _ => code.push(mov_reg(3, 31)), // unreachable ops -> 0 (matches interp `_ => 0`)
                }
            }
        }
        emit_str(&mut code, 3, 0, i); // results[i] = x3
    }
    code.push(ret());
    Some(code)
}

// MAP_JIT executable memory on Apple Silicon. Declared directly to keep the
// carrier dependency-free.
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const PROT_EXEC: i32 = 0x4;
const MAP_PRIVATE: i32 = 0x0002;
const MAP_ANON: i32 = 0x1000;
const MAP_JIT: i32 = 0x0800;

extern "C" {
    fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> i32;
    fn pthread_jit_write_protect_np(enabled: i32);
    fn sys_icache_invalidate(start: *mut c_void, len: usize);
}

type JitFn = extern "C" fn(*mut u64, *const u64, u64);

/// An executable JIT function plus the mapping it lives in. Dropping it unmaps
/// the executable page (clean teardown, no leaked executable memory).
pub struct JitCode {
    ptr: *mut c_void,
    len: usize,
    consts: Vec<u64>,
}

impl JitCode {
    /// Codegen a program into executable memory. Returns `None` if the mmap
    /// fails; `emit` itself always succeeds (any `u32` index is addressable).
    pub fn new(prog: &Program) -> Option<JitCode> {
        let code = emit(prog)?;
        let byte_len = code.len() * 4;
        let page = 16384usize; // apple silicon page
        let len = (byte_len + page - 1) & !(page - 1);
        let ptr = unsafe {
            mmap(
                core::ptr::null_mut(),
                len,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANON | MAP_JIT,
                -1,
                0,
            )
        };
        if ptr as isize == -1 {
            return None;
        }
        unsafe {
            // W^X: make the page writable for this thread, copy, then executable.
            pthread_jit_write_protect_np(0);
            core::ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u32, code.len());
            pthread_jit_write_protect_np(1);
            // The D-cache write is not visible to the I-cache until invalidated.
            sys_icache_invalidate(ptr, len);
        }
        Some(JitCode {
            ptr,
            len,
            consts: prog.consts.clone(),
        })
    }

    /// Run the generated function, filling `results`. The caller folds one
    /// `access::checksum(results)` after, exactly as for the interpreters.
    pub fn run(&self, seed: u64, results: &mut [u64]) {
        let f: JitFn = unsafe { core::mem::transmute(self.ptr) };
        f(results.as_mut_ptr(), self.consts.as_ptr(), seed);
    }
}

impl Drop for JitCode {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr, self.len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::access::checksum;
    use crate::ir::{encode, Decoded, REC24};
    use crate::{generate, GenParams};

    #[test]
    fn jit_matches_interp() {
        // The generated native code fills the same results array as the switch
        // interpreter, so their checksums agree across every profile and seed.
        // This exercises every opcode's stencil (the profiles draw from the full
        // vocabulary), so a wrong encoding on any op would diverge here.
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 800;
            let prog = generate(&gp);
            let jit = JitCode::new(&prog).expect("program codegens");
            let bytes = encode(&prog, &REC24);
            let d = Decoded::parse(&bytes, REC24).unwrap();
            let mut ri = vec![0u64; prog.nodes.len()];
            let mut rj = vec![0u64; prog.nodes.len()];
            for seed in [0u64, 1, 42, 12345, 999_999] {
                crate::interp::interpret(&d, seed, &mut ri);
                jit.run(seed, &mut rj);
                assert_eq!(
                    checksum(&ri),
                    checksum(&rj),
                    "{name}: JIT diverged from interp at seed {seed}"
                );
            }
        }
    }

    #[test]
    fn jit_matches_interp_large_n() {
        // Above 4095 elements the imm12 unsigned-offset window is exceeded and
        // `emit` must switch a load/store to the register-offset form (the
        // index materialized into x6 via MOVZ/MOVK). N=4096 sits exactly at
        // the boundary (every node index from 4095 up needs the slow path,
        // while operands below it still take the fast path); N=16384 is an
        // order of magnitude past it. Bumping const_count to 5000 forces the
        // CONST hole's own register-offset path too (the default 32-entry
        // pool never reaches it), so the const-base (x1) addressing is
        // exercised, not just the results-base (x0) addressing.
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            for &node_count in &[4096usize, 16384usize] {
                let mut gp = GenParams::profile(name).unwrap();
                gp.node_count = node_count;
                gp.const_count = 5000;
                let prog = generate(&gp);
                let jit = JitCode::new(&prog).expect("large program must still codegen");
                let bytes = encode(&prog, &REC24);
                let d = Decoded::parse(&bytes, REC24).unwrap();
                let mut ri = vec![0u64; prog.nodes.len()];
                let mut rj = vec![0u64; prog.nodes.len()];
                for seed in [0u64, 1, 42, 12345, 999_999] {
                    crate::interp::interpret(&d, seed, &mut ri);
                    jit.run(seed, &mut rj);
                    assert_eq!(
                        checksum(&ri),
                        checksum(&rj),
                        "{name} n={node_count}: JIT diverged from interp at seed {seed}"
                    );
                }
            }
        }
    }

    #[test]
    fn very_large_program_no_longer_declines() {
        // Historically `emit` capped out at the imm12 window (4095 elements)
        // and returned None above it. Register-offset addressing removed
        // that cap; any u32-representable index now codegens.
        let mut gp = GenParams::profile("real").unwrap();
        gp.node_count = 20_000;
        gp.const_count = 6000;
        let prog = generate(&gp);
        assert!(
            emit(&prog).is_some(),
            "register-offset addressing must handle programs past the old imm12 cap"
        );
    }
}

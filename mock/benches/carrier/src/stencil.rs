//! Copy-and-patch stencil code generation (the copy-and-patch tier, proper).
//!
//! This is the copy-and-patch mechanism of Xu and Kjolstad (PLDI 2021), as
//! distinct from the direct instruction-selecting codegen in `copypatch.rs`. Each
//! IR op has a fixed machine-code STENCIL that the toolchain assembles once at
//! build time (the `global_asm!` block below); codegen for a program is then a
//! linear pass that, per node, `memcpy`s the op's stencil words and PATCHES their
//! holes (the operand and destination offsets baked into the load/store imm12
//! fields) with this node's actual indices. There is no per-node instruction
//! selection: the JIT never decides which instructions to emit, it copies a
//! precompiled template and fills its holes. That is the defining property of
//! copy-and-patch and the cost-shape difference the matrix measures against the
//! direct-codegen cell (copy + a few patches per node, versus an instruction-
//! selection pass per node).
//!
//! Stencil derivation, honestly. The published technique extracts each stencil's
//! bytes and its relocations from a compiled object file. This self-contained PoC
//! extracts the stencil bytes directly from the toolchain-assembled code via
//! linker labels (`_cp_<op>_start` / `_cp_<op>_end` bracket each stencil, and
//! their addresses bound the words to copy), and records the holes by
//! construction (the load/store word offsets are known from how each stencil is
//! written) rather than parsing relocation entries. The stencils are real
//! assembled machine code, and codegen is a real copy-plus-patch; the one honest
//! simplification versus the paper is where the hole offsets come from (known by
//! construction here, recorded relocations there). No per-op stencil is
//! hand-encoded instruction by instruction the way `copypatch::emit` selects.
//!
//! Calling convention (shared with the direct-codegen cell so the two are
//! comparable): `fn(results: *mut u64 in x0, consts: *const u64 in x1, seed: u64
//! in x2)`, scratch x3/x4/x5. Each stencil computes one node into `results[dst]`
//! and falls through to the next; the generator appends one `ret`. So the output
//! `results` array is byte-identical to the interpreters', the cross-validation.
//!
//! Scope: the imm12 unsigned-offset load/store form scales by 8, so node and
//! const indices must be below 4096; `emit_stencil` returns `None` above that,
//! declining rather than miscompiling (the same window the direct cell has).
//! aarch64 + macOS only (MAP_JIT / W^X / icache), marked, not hidden.

#![cfg(all(feature = "jit", target_arch = "aarch64", target_os = "macos"))]

use crate::ir::{op, Program};
use core::ffi::c_void;

// The assembled stencils. One `.text` block, each op's code bracketed by a
// start/end label pair so the addresses bound the words to copy. No `ret` inside
// a stencil: each falls through, and the generator appends a single `ret`. The
// load/store imm offsets are all `#0` placeholders (the holes), patched at
// codegen. x0 = results, x1 = consts, x2 = seed; x3/x4/x5 scratch.
core::arch::global_asm!(
    r#"
    .p2align 2
    .global _cp_const_start
_cp_const_start:
    ldr x3, [x1, #0]
    str x3, [x0, #0]
    .global _cp_const_end
_cp_const_end:

    .global _cp_input_start
_cp_input_start:
    mov x3, x2
    str x3, [x0, #0]
    .global _cp_input_end
_cp_input_end:

    .global _cp_add_start
_cp_add_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    add x3, x3, x4
    str x3, [x0, #0]
    .global _cp_add_end
_cp_add_end:

    .global _cp_sub_start
_cp_sub_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    sub x3, x3, x4
    str x3, [x0, #0]
    .global _cp_sub_end
_cp_sub_end:

    .global _cp_mul_start
_cp_mul_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    mul x3, x3, x4
    str x3, [x0, #0]
    .global _cp_mul_end
_cp_mul_end:

    .global _cp_and_start
_cp_and_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    and x3, x3, x4
    str x3, [x0, #0]
    .global _cp_and_end
_cp_and_end:

    .global _cp_or_start
_cp_or_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    orr x3, x3, x4
    str x3, [x0, #0]
    .global _cp_or_end
_cp_or_end:

    .global _cp_xor_start
_cp_xor_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    eor x3, x3, x4
    str x3, [x0, #0]
    .global _cp_xor_end
_cp_xor_end:

    .global _cp_shl_start
_cp_shl_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    lsl x3, x3, x4
    str x3, [x0, #0]
    .global _cp_shl_end
_cp_shl_end:

    .global _cp_shr_start
_cp_shr_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    lsr x3, x3, x4
    str x3, [x0, #0]
    .global _cp_shr_end
_cp_shr_end:

    .global _cp_min_start
_cp_min_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    cmp x3, x4
    csel x3, x3, x4, ls
    str x3, [x0, #0]
    .global _cp_min_end
_cp_min_end:

    .global _cp_max_start
_cp_max_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    cmp x3, x4
    csel x3, x3, x4, hs
    str x3, [x0, #0]
    .global _cp_max_end
_cp_max_end:

    .global _cp_eq_start
_cp_eq_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    cmp x3, x4
    cset x3, eq
    str x3, [x0, #0]
    .global _cp_eq_end
_cp_eq_end:

    .global _cp_lt_start
_cp_lt_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    cmp x3, x4
    cset x3, lo
    str x3, [x0, #0]
    .global _cp_lt_end
_cp_lt_end:

    .global _cp_select_start
_cp_select_start:
    ldr x3, [x0, #0]
    ldr x4, [x0, #0]
    ldr x5, [x0, #0]
    cmp x3, #0
    csel x3, x4, x5, ne
    str x3, [x0, #0]
    .global _cp_select_end
_cp_select_end:

    .global _cp_neg_start
_cp_neg_start:
    ldr x3, [x0, #0]
    neg x3, x3
    str x3, [x0, #0]
    .global _cp_neg_end
_cp_neg_end:

    .global _cp_not_start
_cp_not_start:
    ldr x3, [x0, #0]
    mvn x3, x3
    str x3, [x0, #0]
    .global _cp_not_end
_cp_not_end:
"#
);

extern "C" {
    static cp_const_start: u8;
    static cp_const_end: u8;
    static cp_input_start: u8;
    static cp_input_end: u8;
    static cp_add_start: u8;
    static cp_add_end: u8;
    static cp_sub_start: u8;
    static cp_sub_end: u8;
    static cp_mul_start: u8;
    static cp_mul_end: u8;
    static cp_and_start: u8;
    static cp_and_end: u8;
    static cp_or_start: u8;
    static cp_or_end: u8;
    static cp_xor_start: u8;
    static cp_xor_end: u8;
    static cp_shl_start: u8;
    static cp_shl_end: u8;
    static cp_shr_start: u8;
    static cp_shr_end: u8;
    static cp_min_start: u8;
    static cp_min_end: u8;
    static cp_max_start: u8;
    static cp_max_end: u8;
    static cp_eq_start: u8;
    static cp_eq_end: u8;
    static cp_lt_start: u8;
    static cp_lt_end: u8;
    static cp_select_start: u8;
    static cp_select_end: u8;
    static cp_neg_start: u8;
    static cp_neg_end: u8;
    static cp_not_start: u8;
    static cp_not_end: u8;
}

/// Which of a node's indices a hole is patched with.
#[derive(Clone, Copy)]
enum Hole {
    A,     // operands[0] (a node index into results)
    B,     // operands[1]
    C,     // operands[2]
    Const, // operands[0] as a const-pool index (base is x1)
    Dst,   // the node's own index i (the store destination)
}

/// The assembled words of a stencil, bounded by its two labels.
fn words(start: &'static u8, end: &'static u8) -> &'static [u32] {
    let s = start as *const u8 as usize;
    let e = end as *const u8 as usize;
    // labels bracket whole instructions, so the span is a multiple of 4.
    unsafe { core::slice::from_raw_parts(s as *const u32, (e - s) / 4) }
}

/// The stencil words and the (word-index, hole) list for one opcode. The word
/// indices are known by construction from the asm above (each `ldr`/`str` with a
/// `#0` placeholder is a hole).
fn stencil(opcode: u8) -> Option<(&'static [u32], &'static [(usize, Hole)])> {
    // SAFETY: the statics are `.text` labels defined in the global_asm block; the
    // references are addresses only (never dereferenced as u8 values).
    unsafe {
        Some(match opcode {
            op::CONST => (words(&cp_const_start, &cp_const_end), &[(0, Hole::Const), (1, Hole::Dst)]),
            op::INPUT => (words(&cp_input_start, &cp_input_end), &[(1, Hole::Dst)]),
            op::ADD => (words(&cp_add_start, &cp_add_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::SUB => (words(&cp_sub_start, &cp_sub_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::MUL => (words(&cp_mul_start, &cp_mul_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::AND => (words(&cp_and_start, &cp_and_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::OR => (words(&cp_or_start, &cp_or_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::XOR => (words(&cp_xor_start, &cp_xor_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::SHL => (words(&cp_shl_start, &cp_shl_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::SHR => (words(&cp_shr_start, &cp_shr_end), &[(0, Hole::A), (1, Hole::B), (3, Hole::Dst)]),
            op::MIN => (words(&cp_min_start, &cp_min_end), &[(0, Hole::A), (1, Hole::B), (4, Hole::Dst)]),
            op::MAX => (words(&cp_max_start, &cp_max_end), &[(0, Hole::A), (1, Hole::B), (4, Hole::Dst)]),
            op::EQ => (words(&cp_eq_start, &cp_eq_end), &[(0, Hole::A), (1, Hole::B), (4, Hole::Dst)]),
            op::LT => (words(&cp_lt_start, &cp_lt_end), &[(0, Hole::A), (1, Hole::B), (4, Hole::Dst)]),
            op::SELECT => (
                words(&cp_select_start, &cp_select_end),
                &[(0, Hole::A), (1, Hole::B), (2, Hole::C), (5, Hole::Dst)],
            ),
            op::NEG => (words(&cp_neg_start, &cp_neg_end), &[(0, Hole::A), (2, Hole::Dst)]),
            op::NOT => (words(&cp_not_start, &cp_not_end), &[(0, Hole::A), (2, Hole::Dst)]),
            _ => return None,
        })
    }
}

/// Patch the imm12 unsigned-offset field (bits [21:10]) of a load/store word with
/// `idx` (which is already the offset in units of the 8-byte access size, i.e. the
/// element index).
#[inline]
fn patch_imm12(word: u32, idx: u32) -> u32 {
    (word & !(0xFFF << 10)) | ((idx & 0xFFF) << 10)
}

/// Copy-and-patch codegen: for each node, copy its opcode's stencil words and
/// patch each hole with the node's actual index. Returns `None` if any index
/// exceeds the imm12 window (4095), declining rather than miscompiling.
pub fn emit_stencil(prog: &Program) -> Option<Vec<u32>> {
    if prog.nodes.len() >= 4096 || prog.consts.len() >= 4096 {
        return None;
    }
    let mut code: Vec<u32> = Vec::with_capacity(prog.nodes.len() * 6 + 1);
    for (i, node) in prog.nodes.iter().enumerate() {
        let (stencil_words, holes) = stencil(node.op)?;
        let base = code.len();
        code.extend_from_slice(stencil_words); // copy
        for &(w, hole) in holes {
            let idx = match hole {
                Hole::A => node.operands[0],
                Hole::B => node.operands[1],
                Hole::C => node.operands[2],
                Hole::Const => node.operands[0],
                Hole::Dst => i as u32,
            };
            code[base + w] = patch_imm12(code[base + w], idx); // patch
        }
    }
    code.push(0xD65F_03C0); // ret
    Some(code)
}

// MAP_JIT executable memory on Apple Silicon (same libc surface the direct cell
// declares; kept local so the carrier stays dependency-free).
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

/// An executable copy-and-patch function plus its mapping. Dropping it unmaps the
/// page (clean teardown, no leaked executable memory).
pub struct StencilCode {
    ptr: *mut c_void,
    len: usize,
    consts: Vec<u64>,
}

impl StencilCode {
    /// Copy-and-patch a program into executable memory. `None` if `emit_stencil`
    /// declines (too large) or the mmap fails.
    pub fn new(prog: &Program) -> Option<StencilCode> {
        let code = emit_stencil(prog)?;
        let byte_len = code.len() * 4;
        let page = 16384usize;
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
            pthread_jit_write_protect_np(0);
            core::ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u32, code.len());
            pthread_jit_write_protect_np(1);
            sys_icache_invalidate(ptr, len);
        }
        Some(StencilCode { ptr, len, consts: prog.consts.clone() })
    }

    /// Run the generated function, filling `results`. The caller folds one
    /// `access::checksum(results)` after, exactly as for the interpreters.
    pub fn run(&self, seed: u64, results: &mut [u64]) {
        let f: JitFn = unsafe { core::mem::transmute(self.ptr) };
        f(results.as_mut_ptr(), self.consts.as_ptr(), seed);
    }
}

impl Drop for StencilCode {
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
    fn stencil_matches_interp() {
        // The copy-and-patch code fills the same results array as the switch
        // interpreter, so their checksums agree across every profile and seed.
        // The profiles draw the full vocabulary, so a wrong stencil or a
        // mispatched hole on any opcode would diverge here.
        for name in ["real", "madd", "tight", "scatter", "wideselect", "leaf"] {
            let mut gp = GenParams::profile(name).unwrap();
            gp.node_count = 800;
            let prog = generate(&gp);
            let jit = StencilCode::new(&prog).expect("program fits the imm12 window");
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
                    "{name}: stencil copy-and-patch diverged from interp at seed {seed}"
                );
            }
        }
    }

    #[test]
    fn stencil_agrees_with_direct_codegen() {
        // The copy-and-patch cell and the direct-codegen cell compute the same
        // program (they differ only in how the machine code is produced, not in
        // what it computes), so their output checksums are identical.
        let mut gp = GenParams::profile("real").unwrap();
        gp.node_count = 500;
        let prog = generate(&gp);
        let sten = StencilCode::new(&prog).unwrap();
        let direct = crate::copypatch::JitCode::new(&prog).unwrap();
        let mut rs = vec![0u64; prog.nodes.len()];
        let mut rd = vec![0u64; prog.nodes.len()];
        for seed in [1u64, 7, 100, 65535] {
            sten.run(seed, &mut rs);
            direct.run(seed, &mut rd);
            assert_eq!(checksum(&rs), checksum(&rd), "stencil vs direct diverged at seed {seed}");
        }
    }

    #[test]
    fn oversize_program_declines() {
        let mut gp = GenParams::profile("real").unwrap();
        gp.node_count = 5000;
        let prog = generate(&gp);
        assert!(emit_stencil(&prog).is_none(), "oversize program must decline, not miscompile");
    }
}

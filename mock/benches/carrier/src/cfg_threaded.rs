//! Preserve-none context-threaded dispatch over the CFG (extracted).
//!
//! This is the threaded-CFG dispatch cell. It lives in its own file, loaded only
//! under the `threaded` feature (`#[cfg(feature = "threaded")] pub mod
//! cfg_threaded;` in lib.rs), because `become` is parse-gated: if the code lived
//! inside always-loaded `cfg.rs`, its tokens would have to parse even when the
//! module is cfg-stripped, forcing `explicit_tail_calls` on for every build. In a
//! feature-gated file the tokens are never seen without the feature, so the "only
//! threaded pulls the nightly feature" property holds.
//!
//! The straight-line threaded cells understate token threading, whose advantage
//! lives in hot loops with real control transfers. This threads the CFG: block
//! instructions and terminators flatten into one handler stream, op handlers
//! tail-call their successor, and terminator handlers compute the next flat index
//! (a jump target or branch) and tail-call into the target block's first handler.
//! A loop back-edge is a `become` into an earlier flat index; the branch-heavy
//! kernels exercise the indirect tail transfer that is the whole point.
//!
//! Same fairness contract as the switch and fntable CFG cells: every register
//! read and write goes through the shared `access::rload` / `access::rstore`, so
//! only the dispatch shape differs. The handler stream is built once (hoisted out
//! of the timed region by `flatten`), then the run is pure threaded dispatch.

use crate::cfg::{op, Block, Term, NREG};
use crate::access::{rload, rstore};

/// The run outputs, written by the terminal `h_ret` handler. `#[repr(C)]` so
/// the pointer passed through the preserve-none handler chain has a stable
/// layout.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Out {
    pub result: u64,
    pub ninstr: u64,
    pub nterm: u64,
}

/// One flattened threaded instruction: a handler plus the fields it reads.
/// Op instructions use `dst/a/b/imm`; terminators use `reg` (the tested
/// register) and `t0/t1` (flat successor indices). 32 bytes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TInstr {
    pub handler: H,
    pub dst: u8,
    pub a: u8,
    pub b: u8,
    pub reg: u8,
    pub imm: u64,
    pub t0: u32,
    pub t1: u32,
}

// The handler's `code` argument is typed `*const ()` rather than
// `*const TInstr` to break the otherwise-recursive type alias (`H` names
// `TInstr` which names `H`); it is cast back inside every handler. Same
// device the straight-line `threaded_direct` cell uses.
pub type H = extern "rust-preserve-none" fn(usize, *const (), *mut u64, u64, u64, u64, *mut Out);

#[inline(always)]
unsafe fn finish(result: u64, ninstr: u64, nterm: u64, out: *mut Out) {
    (*out).result = result;
    (*out).ninstr = ninstr;
    (*out).nterm = nterm;
}

macro_rules! op_handler {
    ($name:ident, $combine:expr) => {
        extern "rust-preserve-none" fn $name(
            idx: usize,
            code: *const (),
            regs: *mut u64,
            ninstr: u64,
            nterm: u64,
            cap: u64,
            out: *mut Out,
        ) {
            unsafe {
                let c = code as *const TInstr;
                let ins = *c.add(idx);
                let a = rload(regs, ins.a as u32);
                let b = rload(regs, ins.b as u32);
                let combine: fn(u64, u64, u64) -> u64 = $combine;
                let v = combine(ins.imm, a, b);
                rstore(regs, ins.dst as usize, v);
                let next = idx + 1;
                become ((*c.add(next)).handler)(next, code, regs, ninstr + 1, nterm, cap, out);
            }
        }
    };
}

// the register-VM arithmetic defers to the single ops::binop_body definition
// (SET is CFG-specific); the four ops are a subset of it.
op_handler!(h_set, |imm, _a, _b| imm);
op_handler!(h_add, |_i, a, b| crate::ops::binop_body!(ADD, a, b));
op_handler!(h_sub, |_i, a, b| crate::ops::binop_body!(SUB, a, b));
op_handler!(h_mul, |_i, a, b| crate::ops::binop_body!(MUL, a, b));
op_handler!(h_and, |_i, a, b| crate::ops::binop_body!(AND, a, b));

extern "rust-preserve-none" fn h_jmp(
    idx: usize,
    code: *const (),
    regs: *mut u64,
    ninstr: u64,
    nterm: u64,
    cap: u64,
    out: *mut Out,
) {
    unsafe {
        let c = code as *const TInstr;
        let nterm = nterm + 1;
        if ninstr + nterm > cap {
            return finish(rload(regs, 0), ninstr, nterm, out);
        }
        let ins = *c.add(idx);
        let next = ins.t0 as usize;
        become ((*c.add(next)).handler)(next, code, regs, ninstr, nterm, cap, out);
    }
}

extern "rust-preserve-none" fn h_brnz(
    idx: usize,
    code: *const (),
    regs: *mut u64,
    ninstr: u64,
    nterm: u64,
    cap: u64,
    out: *mut Out,
) {
    unsafe {
        let c = code as *const TInstr;
        let nterm = nterm + 1;
        if ninstr + nterm > cap {
            return finish(rload(regs, 0), ninstr, nterm, out);
        }
        let ins = *c.add(idx);
        let next = if rload(regs, ins.reg as u32) != 0 { ins.t0 } else { ins.t1 } as usize;
        become ((*c.add(next)).handler)(next, code, regs, ninstr, nterm, cap, out);
    }
}

extern "rust-preserve-none" fn h_ret(
    idx: usize,
    code: *const (),
    regs: *mut u64,
    ninstr: u64,
    nterm: u64,
    cap: u64,
    out: *mut Out,
) {
    let _ = cap;
    unsafe {
        let c = code as *const TInstr;
        let nterm = nterm + 1;
        let ins = *c.add(idx);
        finish(rload(regs, ins.reg as u32), ninstr, nterm, out);
    }
}

fn op_handler(o: u8) -> H {
    match o {
        op::SET => h_set,
        op::ADD => h_add,
        op::SUB => h_sub,
        op::MUL => h_mul,
        _ => h_and,
    }
}

/// Flatten the block CFG into a single threaded instruction stream, resolving
/// each block's jump targets to flat indices. Setup work, hoisted out of the
/// timed region.
pub fn flatten(blocks: &[Block]) -> Vec<TInstr> {
    let mut starts = vec![0u32; blocks.len()];
    let mut pos = 0u32;
    for (bi, b) in blocks.iter().enumerate() {
        starts[bi] = pos;
        pos += b.instrs.len() as u32 + 1; // +1 for the terminator slot
    }
    let mut code = Vec::with_capacity(pos as usize);
    for b in blocks {
        for ins in &b.instrs {
            code.push(TInstr {
                handler: op_handler(ins.op),
                dst: ins.dst,
                a: ins.a,
                b: ins.b,
                reg: 0,
                imm: ins.imm,
                t0: 0,
                t1: 0,
            });
        }
        let term = match b.term {
            Term::Jmp(t) => TInstr {
                handler: h_jmp,
                dst: 0,
                a: 0,
                b: 0,
                reg: 0,
                imm: 0,
                t0: starts[t as usize],
                t1: 0,
            },
            Term::BrNz(r, nz, z) => TInstr {
                handler: h_brnz,
                dst: 0,
                a: 0,
                b: 0,
                reg: r,
                imm: 0,
                t0: starts[nz as usize],
                t1: starts[z as usize],
            },
            Term::Ret(r) => TInstr {
                handler: h_ret,
                dst: 0,
                a: 0,
                b: 0,
                reg: r,
                imm: 0,
                t0: 0,
                t1: 0,
            },
        };
        code.push(term);
    }
    code
}

/// Run a flattened threaded CFG from flat index 0 with `seed` in r0. The
/// register file is caller-independent (a fresh `[u64; NREG]`); returns the
/// same `(result, ninstr, nterm)` triple as the switch and fntable cells.
pub fn interp_flat(code: &[TInstr], seed: u64, cap: u64) -> (u64, u64, u64) {
    let mut regs = [0u64; NREG];
    let rp = regs.as_mut_ptr();
    unsafe { rstore(rp, 0, seed) };
    let mut out = Out::default();
    unsafe {
        let c = code.as_ptr();
        ((*c).handler)(0, c as *const (), rp, 0, 0, cap, &mut out);
    }
    (out.result, out.ninstr, out.nterm)
}

/// Convenience: flatten and run. The flatten cost is included, so a bench
/// hoists `flatten` out and times `interp_flat`; tests use this.
pub fn interp_threaded(blocks: &[Block], seed: u64, cap: u64) -> (u64, u64, u64) {
    let code = flatten(blocks);
    interp_flat(&code, seed, cap)
}

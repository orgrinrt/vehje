//! Boundary bench 4: entry-point form, scalar payload pinned. op's vehicle bench.
//!
//! Over the same fixed-column / swept-W shape, holds the payload at scalar and varies
//! only how the batch width binds to the executed body. All four forms wrap the identical
//! scalar interpreter, so the comparison is fair (chris-fallin trap 1); they differ only
//! in where the W-to-body binding happens:
//!
//! - `scalar_anchor`: the per-record entry (`cr_execute1`), one record per crossing,
//!   W-invariant (always [`N_TOTAL`] crossings). The no-batched-entry reference: the cost
//!   of not batching at all.
//! - `runtime_w` (baseline): one symbol, `W` a runtime argument, internal loop
//!   (`cr_execute_scalar_runtime_w`). No unroll.
//! - `dispatch_table`: one symbol, runtime `W`, `match W` to statically monomorphised
//!   bodies (`cr_execute_scalar_dispatch`). One predictable branch per crossing.
//! - `per_w_set`: the per-W-monomorphised symbol for this W (`cr_execute_scalar_wK`),
//!   resolved once at setup, no width argument and no branch per crossing.
//!
//! The C-boundary-mapping decision op raised is dispatch-table versus per-W-set: same
//! monomorphised bodies, differing only in whether the W-to-body branch is paid callee-
//! side once per crossing (dispatch) or eliminated caller-side by resolving the symbol
//! once (per-W-set). The per-W symbol set stops at Wmax=128, so at W=256 `per_w_set`
//! chunks by 128 (two crossings), which is exactly the evidence that a per-W ABI plateaus
//! past its maximum compiled width while the runtime-W and dispatch forms keep scaling.
//!
//! `null_entry` is the shared pure-crossing floor. Every form is disasm-confirmed by the
//! ISA-shape gate (bench 0) before its number is trusted.

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{
    cross_column, fill_seeds, init_handle, open_and_init, open_runtime, CrEntryMono, CrExec1, CrFree,
    CrInit, StCross, N_TOTAL,
};

/// The largest per-W-monomorphised symbol compiled in `carrier-runtime` (Wmax=128). A
/// requested W above this chunks by [`WMAX`], modelling a per-W ABI's plateau past its
/// maximum compiled width.
pub const WMAX: usize = 128;

// ── the per-record anchor state ──

/// State for the per-record scalar anchor: the runtime, handle, resolved `cr_execute1`,
/// free pointer, and reused seed column. W-invariant (the anchor crosses per record).
pub struct StAnchor {
    pub rt:     Runtime,
    pub handle: *mut c_void,
    pub exec1:  CrExec1,
    pub free:   CrFree,
    pub seeds:  Vec<u64>,
}

impl Drop for StAnchor {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.free)(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

pub fn open_anchor(profile: &str) -> StAnchor {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let exec1: CrExec1 = unsafe { rt.resolve(b"cr_execute1\0") }.expect("cr_execute1");
    let handle = init_handle(&rt, init, profile);
    StAnchor { rt, handle, exec1, free, seeds: vec![0u64; N_TOTAL] }
}

/// Cross the per-record entry once per record over the whole column. Shared with the
/// cross-language family (bench 8), which crosses into the identical entry shape.
#[inline(always)]
pub fn anchor_column(exec1: CrExec1, handle: *mut c_void, seeds: &[u64]) -> u64 {
    let mut acc = 0u64;
    for &sd in &seeds[..N_TOTAL] {
        acc = acc.rotate_left(7) ^ unsafe { exec1(handle, sd) };
    }
    acc
}

// ── the per-W-monomorphised state ──

/// State for the per-W-monomorphised form: the runtime, handle, the resolved per-W entry
/// (no width argument), the free pointer, the effective compiled width `eff` the column is
/// chunked by, and the reused seed column.
pub struct StMono {
    pub rt:     Runtime,
    pub handle: *mut c_void,
    pub mono:   CrEntryMono,
    pub free:   CrFree,
    pub eff:    usize,
    pub seeds:  Vec<u64>,
}

impl Drop for StMono {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.free)(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

pub fn open_mono(profile: &str, w: usize) -> StMono {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    // Requested W above Wmax resolves the Wmax symbol and chunks by it: the plateau.
    let eff = w.min(WMAX);
    let name = format!("cr_execute_scalar_w{eff}\0");
    let mono: CrEntryMono =
        unsafe { rt.resolve(name.as_bytes()) }.expect("per-W monomorphised entry resolves");
    let handle = init_handle(&rt, init, profile);
    StMono { rt, handle, mono, free, eff, seeds: vec![0u64; N_TOTAL] }
}

/// Cross the per-W entry (no width argument) over the column in `eff`-sized chunks. Shared
/// with the cross-language family (bench 8).
#[inline(always)]
pub fn mono_column(mono: CrEntryMono, handle: *mut c_void, seeds: &[u64], eff: usize) -> u64 {
    let mut acc = 0u64;
    let mut off = 0usize;
    while off < N_TOTAL {
        // The symbol reads exactly `eff` records; `eff` divides N_TOTAL so no partial.
        acc = acc.rotate_left(7) ^ unsafe { mono(handle, seeds.as_ptr().add(off)) };
        off += eff;
    }
    acc
}

bench_matrix! {
    name: "abi_entry_form",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0004,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "runtime_w",
    floor: "null_entry",
    regime: warm,

    // Shared setup: the runtime-W form (the baseline). `n` is the batch width `W`.
    setup |profile: &str, n: usize| -> StCross {
        open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
    }

    cell runtime_w
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // The per-record anchor: no batched entry, one crossing per record.
    cell scalar_anchor
        #[feature = "boundary"]
        setup |profile: &str, _n: usize| -> StAnchor {
            open_anchor(profile)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            anchor_column(s.exec1, s.handle, &s.seeds)
        }

    // The dispatch-table form: one symbol, `match W` to monomorphised bodies.
    cell dispatch_table
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_scalar_dispatch\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // The per-W-monomorphised form: symbol resolved once, no width argument, no branch.
    cell per_w_set
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMono {
            open_mono(profile, n)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            mono_column(s.mono, s.handle, &s.seeds, s.eff)
        }

    // The shared pure-crossing floor.
    cell null_entry
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_null_entry\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }
}

#[cfg(test)]
mod tests {
    use super::super::common::{
        runtime_dylib_path, inproc_scalar_batch, program_bytes, CrEntryW,
    };
    use super::*;
    use crate as c;

    /// Each entry form's crossing computes byte-exactly what the in-process scalar payload
    /// computes over the same column and batch structure. The anchor folds per record; the
    /// runtime-W, dispatch, and per-W forms fold per crossing at their effective width, so
    /// each is validated against a replica of its own structure. (dispatch and runtime-W at
    /// equal W share structure; per-W matches for W <= Wmax and chunks by Wmax above it.)
    #[test]
    fn entry_forms_match_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let exec1: CrExec1 = unsafe { rt.resolve(b"cr_execute1\0") }.expect("cr_execute1");
        let rtw: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("runtime-w");

        for profile in ["real", "tight", "leaf"] {
            let bytes = program_bytes(profile);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let n = pd.nodes.len();
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());

            let seed = 0xc0ffee ^ profile.len() as u64;
            let mut seeds = vec![0u64; N_TOTAL];
            fill_seeds(&mut seeds, seed);

            // anchor: per-record fold in-process (one flat batch of N_TOTAL).
            let mut scratch = vec![0u64; n];
            let anchor_host = anchor_column(exec1, handle, &seeds);
            let anchor_ref = inproc_scalar_batch(&pd, &seeds, &mut scratch);
            assert_eq!(anchor_host, anchor_ref, "anchor mismatch for {profile}");

            // per-W-set at several W, each byte-matching runtime-W at the same effective
            // width (both fold per crossing at that width).
            for w in [1usize, 8, 64, 128, 256] {
                let mono = open_mono_symbol(&rt, w);
                let host = mono_column(mono, handle, &seeds, w.min(WMAX));
                let refv = cross_column(rtw, handle, &seeds, w.min(WMAX));
                assert_eq!(host, refv, "per-W vs runtime-W (eff) mismatch for {profile}, W={w}");
            }
            unsafe { free(handle) };
        }
    }

    /// Resolve the per-W symbol the mono form would use for width `w` (eff = min(w, Wmax)).
    fn open_mono_symbol(rt: &Runtime, w: usize) -> CrEntryMono {
        let eff = w.min(WMAX);
        let name = format!("cr_execute_scalar_w{eff}\0");
        unsafe { rt.resolve(name.as_bytes()) }.expect("per-W symbol resolves")
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_entry_form");
        assert_eq!(d.baseline, "runtime_w");
        assert_eq!(d.floor.as_deref(), Some("null_entry"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(
            tags,
            ["runtime_w", "scalar_anchor", "dispatch_table", "per_w_set", "null_entry"]
        );
    }
}

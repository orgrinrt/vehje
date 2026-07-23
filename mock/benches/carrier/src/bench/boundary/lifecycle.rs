//! Boundary bench 6: instance lifecycle (call-scoped vs session-scoped runtime).
//!
//! At the ABI layer "cold" is not the interpreter arc's program-identity cold: it is a
//! fresh runtime instance (residual load, predecode, region reservation) versus a
//! long-lived instance taking many batched calls. Over the same fixed-column / swept-W
//! shape, scalar payload, this varies how often the runtime handle is (re-)established.
//! It answers whether `execute` must take a persistent runtime handle, or whether a
//! per-call instance is acceptable.
//!
//! Cells: `held_handle` (baseline, the handle built once at setup and reused, the
//! session-scoped case), `fresh_per_column` (one `cr_init`/`cr_free` around the whole
//! column crossing, per pass), `fresh_per_batch` (a fresh instance per crossing, the
//! extreme: `k` inits and frees per pass), `null_entry` (the shared pure-crossing floor).
//! The `cr_init` re-parses and re-predecodes the residual, so the fresh cells charge that
//! re-establishment, which is exactly the session-versus-call-scope cost.

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{
    cross_column, fill_seeds, open_and_init, open_runtime, program_bytes, CrEntryW, CrFree, CrInit,
    StCross, N_TOTAL,
};

/// State for the per-call-instance cells: the runtime, the resolved init/free/entry
/// pointers, the residual wire bytes to re-init from, and the reused seed column. No
/// persistent handle: each timed call builds and frees its own.
pub struct StLifecycle {
    pub rt:    Runtime,
    pub init:  CrInit,
    pub free:  CrFree,
    pub entry: CrEntryW,
    pub bytes: Vec<u8>,
    pub seeds: Vec<u64>,
    pub w:     usize,
}

fn open_lifecycle(profile: &str, w: usize) -> StLifecycle {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let entry: CrEntryW =
        unsafe { rt.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("scalar runtime-w");
    StLifecycle { rt, init, free, entry, bytes: program_bytes(profile), seeds: vec![0u64; N_TOTAL], w }
}

/// Build a fresh handle from the residual bytes, asserting non-null.
#[inline(always)]
fn fresh_handle(init: CrInit, bytes: &[u8]) -> *mut c_void {
    let h = unsafe { init(bytes.as_ptr(), bytes.len()) };
    assert!(!h.is_null(), "cr_init must build a handle in the timed cell");
    h
}

bench_matrix! {
    name: "abi_lifecycle",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0006,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "held_handle",
    floor: "null_entry",
    regime: warm,

    // The session-scoped handle: built once at setup, reused across every crossing.
    setup |profile: &str, n: usize| -> StCross {
        open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
    }

    cell held_handle
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // A fresh instance per pass: one init/free around the whole column crossing.
    cell fresh_per_column
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StLifecycle {
            open_lifecycle(profile, n)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            let h = fresh_handle(s.init, &s.bytes);
            let acc = cross_column(s.entry, h, &s.seeds, s.w);
            unsafe { (s.free)(h) };
            acc
        }

    // A fresh instance per crossing: k inits and frees per pass (the extreme).
    cell fresh_per_batch
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StLifecycle {
            open_lifecycle(profile, n)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            let mut acc = 0u64;
            let mut off = 0usize;
            while off < N_TOTAL {
                let batch = s.w.min(N_TOTAL - off);
                let h = fresh_handle(s.init, &s.bytes);
                acc = acc.rotate_left(7)
                    ^ unsafe { (s.entry)(h, s.seeds.as_ptr().add(off), batch) };
                unsafe { (s.free)(h) };
                off += s.w;
            }
            acc
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
    /// A fresh-per-column handle computes the identical crossing fold as the held handle
    /// (the instance is stateless across calls beyond its scratch). Guards the shape;
    /// crossing correctness is the shared `cross.rs` cross-validation.
    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_lifecycle");
        assert_eq!(d.baseline, "held_handle");
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["held_handle", "fresh_per_column", "fresh_per_batch", "null_entry"]);
    }
}

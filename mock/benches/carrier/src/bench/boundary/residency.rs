//! Boundary bench 7: input buffer provenance (reused vs freshly allocated per call).
//!
//! Over the same fixed-column / swept-W shape, scalar payload, holds the crossing and
//! handle constant and varies where the input column comes from: a host-lent column
//! reused across calls versus a column freshly allocated and filled per call. Both cells
//! cross identically into `cr_execute_scalar_runtime_w`; they differ only in whether the
//! column crossed is the reused buffer or a freshly allocated one.
//!
//! ## Honest scope (panel finding)
//!
//! This measures the per-call ALLOCATION-plus-fill overhead of a fresh column, NOT genuine
//! cold-page first-touch. The system allocator typically hands back the just-freed block,
//! so the "fresh" buffer is usually the same warm physical pages; the measured delta over
//! the reused buffer is the allocator round-trip, not a cold-residency cost. A true
//! registered-buffer / residency measurement (the io_uring registered-buffers question)
//! needs a non-recycled or cache-evicted region and a regime that defeats the calibration
//! loop's warmth; that is a follow-up, not this cell. The cell is named and documented for
//! what it measures so a reader does not over-read it.
//!
//! Cells: `reused_buffer` (baseline, the host-lent column), `fresh_alloc` (allocate and fill
//! a new column per call), `null_entry` (the shared pure-crossing floor).

use mockspace_bench_matrix::bench_matrix;

use super::common::{cross_column, fill_seeds, open_and_init, StCross, N_TOTAL};

bench_matrix! {
    name: "abi_residency",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0007,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "reused_buffer",
    floor: "null_entry",
    regime: warm,

    setup |profile: &str, n: usize| -> StCross {
        open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
    }

    // The reused, host-lent input column.
    cell reused_buffer
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // A fresh column allocated and filled per call. Measures the allocation-plus-fill
    // overhead (see the module scope note: the allocator usually recycles, so this is not
    // genuine cold first-touch).
    cell fresh_alloc
        #[feature = "boundary"]
        |s, seed| {
            let mut fresh = vec![0u64; N_TOTAL];
            fill_seeds(&mut fresh, seed);
            cross_column(s.entry, s.handle, &fresh, s.w)
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
    use super::super::common::{program_bytes, runtime_dylib_path, CrEntryW, CrFree, CrInit};
    use super::*;
    use crate as c;
    use mockspace_bench_matrix::boundary::Runtime;

    /// The reused and fresh columns, filled from the same seed, cross to the identical fold:
    /// the cells differ only in buffer provenance, not in the computation crossed. Verifies
    /// the property the family rests on (rather than only asserting the declared shape).
    #[test]
    fn reused_and_fresh_columns_cross_to_the_same_fold() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let entry: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("scalar runtime-w");

        for profile in ["real", "tight"] {
            let bytes = program_bytes(profile);
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            let _ = c::ir::REC24;
            assert!(!handle.is_null());
            for w in [1usize, 8, 256] {
                let seed = 0x5151 ^ (w as u64);
                let mut reused = vec![0u64; N_TOTAL];
                fill_seeds(&mut reused, seed);
                let a = cross_column(entry, handle, &reused, w);
                // a freshly allocated column with the same content crosses identically.
                let mut fresh = vec![0u64; N_TOTAL];
                fill_seeds(&mut fresh, seed);
                let b = cross_column(entry, handle, &fresh, w);
                assert_eq!(a, b, "provenance must not change the fold, {profile} W={w}");
            }
            unsafe { free(handle) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_residency");
        assert_eq!(d.baseline, "reused_buffer");
        assert_eq!(d.floor.as_deref(), Some("null_entry"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["reused_buffer", "fresh_alloc", "null_entry"]);
    }
}

//! Boundary bench 7: input buffer residency and ownership.
//!
//! Over the same fixed-column / swept-W shape, scalar payload, holds the crossing and
//! handle constant and varies the input buffer's residency: a host-lent column registered
//! once and reused across calls (warm, pre-pinned) versus a fresh region allocated per
//! call (cold, re-pinned). The io_uring registered-buffers / JNI critical / CPython
//! buffer-protocol question, applied to the runtime ABI: does lending a persistent input
//! region matter, or is a fresh column per call acceptable.
//!
//! Both cells cross identically into `cr_execute_scalar_runtime_w`; they differ only in
//! whether the column crossed is the reused buffer or a freshly allocated one, so the
//! delta is the allocation plus first-touch cost of a cold input region per call.
//!
//! Cells: `reused_buffer` (baseline, the warm reused column), `fresh_buffer` (a new column
//! allocated and filled per call), `null_entry` (the shared pure-crossing floor).

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

    // The reused, warm, pre-pinned input column (the registered-buffer case).
    cell reused_buffer
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // A fresh input column allocated and filled per call (the cold, re-pinned case). The
    // allocation and first-touch of the cold region are charged to the timed pass.
    cell fresh_buffer
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
    /// The residency cells differ only in buffer provenance, not computation: reused and
    /// fresh columns filled from the same seed compute the identical crossing fold. Proven
    /// against the shared crossing cross-validation in `cross.rs`; here we guard the
    /// declared shape.
    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_residency");
        assert_eq!(d.baseline, "reused_buffer");
        assert_eq!(d.floor.as_deref(), Some("null_entry"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["reused_buffer", "fresh_buffer", "null_entry"]);
    }
}

//! The composition matrix: `abi_boundary_w`, crossing the two decisive axes.
//!
//! After the individual benches isolate each axis, this curated family crosses the two the
//! ABI decision actually turns on, entry-point form and payload, over the same W-sweep, so
//! the selector-regret analysis can answer the real question: does one fixed ABI choice (one
//! entry form, one payload) win across the six profiles and the W range, or does the optimum
//! move, which would make a runtime-selectable batch width or entry form a real design
//! consequence. `MatrixDecl` carries one swept axis (W), so the cross is a curated flat cell
//! list (the `entgrid` precedent), pruned to the reachable combinations.
//!
//! Cells (payload x entry form, plus the cross-language and pure-crossing floors):
//!
//! - `scalar_runtime_w` (baseline) / `soa_runtime_w`: runtime-W entry, scalar / SoA-8 payload.
//! - `scalar_dispatch` / `soa_dispatch`: dispatch-table entry, each payload.
//! - `scalar_per_w` / `soa_per_w`: per-W-monomorphised entry (chunking by Wmax=128 above it).
//! - `scalar_anchor`: the per-record no-batch reference (scalar only; SoA at W=1 is scalar).
//! - `zig_runtime_w`: the cross-language comparator (the Rust cells are a same-toolchain
//!   lower bound; this is the real target-language cost).
//! - `null_entry`: the shared pure-crossing floor.
//!
//! Every cell reuses the exact machinery its individual bench validated, so the composition
//! is the same measured operations recombined, not a reimplementation. The `abi_sink` and
//! `abi_residency` W-swept families already cross their axis over W and stand as the sink and
//! residency composition families; a further cross pinned to the winning entry/payload is a
//! post-run refinement once the selector names the winner.

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{
    cross_column, fill_seeds, init_handle, open_and_init, open_runtime, CrEntryMono, CrFree, CrInit,
    StCross, N_TOTAL,
};
use super::entry::{anchor_column, mono_column, open_anchor, open_mono, StAnchor, StMono, WMAX};
use super::zig::open_zig_cross;

/// Open the runtime and resolve the SoA per-W-monomorphised entry for width `w` (eff =
/// min(w, Wmax)), the SoA counterpart to [`open_mono`].
fn open_soa_mono(profile: &str, w: usize) -> StMono {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let eff = w.min(WMAX);
    let name = format!("cr_execute_soa_w{eff}\0");
    let mono: CrEntryMono =
        unsafe { rt.resolve(name.as_bytes()) }.expect("soa per-W entry resolves");
    let handle = init_handle(&rt, init, profile);
    StMono { rt, handle, mono, free, eff, seeds: vec![0u64; N_TOTAL] }
}

/// A width-taking cell body: fill the column from `seed`, cross `k = N/w` times.
#[inline(always)]
fn cross_cell(entry: super::common::CrEntryW, handle: *mut c_void, seeds: &mut [u64], w: usize, seed: u64) -> u64 {
    fill_seeds(seeds, seed);
    cross_column(entry, handle, seeds, w)
}

bench_matrix! {
    name: "abi_boundary_w",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_00a0,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "scalar_runtime_w",
    floor: "null_entry",
    regime: warm,

    setup |profile: &str, n: usize| -> StCross {
        open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
    }

    cell scalar_runtime_w
        #[feature = "boundary"]
        |s, seed| { cross_cell(s.entry, s.handle, &mut s.seeds, s.w, seed) }

    cell soa_runtime_w
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_soa_runtime_w\0")
        }
        |s, seed| { cross_cell(s.entry, s.handle, &mut s.seeds, s.w, seed) }

    cell scalar_dispatch
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_scalar_dispatch\0")
        }
        |s, seed| { cross_cell(s.entry, s.handle, &mut s.seeds, s.w, seed) }

    cell soa_dispatch
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_soa_dispatch\0")
        }
        |s, seed| { cross_cell(s.entry, s.handle, &mut s.seeds, s.w, seed) }

    cell scalar_per_w
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMono {
            open_mono(profile, n)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            mono_column(s.mono, s.handle, &s.seeds, s.eff)
        }

    cell soa_per_w
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMono {
            open_soa_mono(profile, n)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            mono_column(s.mono, s.handle, &s.seeds, s.eff)
        }

    cell scalar_anchor
        #[feature = "boundary"]
        setup |profile: &str, _n: usize| -> StAnchor {
            open_anchor(profile)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            anchor_column(s.exec1, s.handle, &s.seeds)
        }

    cell zig_runtime_w
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_zig_cross(profile, n, b"zr_execute_scalar_runtime_w\0")
        }
        |s, seed| { cross_cell(s.entry, s.handle, &mut s.seeds, s.w, seed) }

    cell null_entry
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_null_entry\0")
        }
        |s, seed| { cross_cell(s.entry, s.handle, &mut s.seeds, s.w, seed) }
}

#[cfg(test)]
mod tests {
    /// The composition family declares the curated entry-form x payload cross plus the
    /// cross-language and pure-crossing floors. Its cells reuse the individual benches'
    /// validated machinery, so their crossing correctness is those benches' cross-validation;
    /// here we guard the composed shape.
    #[test]
    fn matrix_decls_declare_the_composition_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_boundary_w");
        assert_eq!(d.baseline, "scalar_runtime_w");
        assert_eq!(d.floor.as_deref(), Some("null_entry"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(
            tags,
            [
                "scalar_runtime_w",
                "soa_runtime_w",
                "scalar_dispatch",
                "soa_dispatch",
                "scalar_per_w",
                "soa_per_w",
                "scalar_anchor",
                "zig_runtime_w",
                "null_entry",
            ]
        );
    }
}

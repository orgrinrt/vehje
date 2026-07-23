//! Payload-cost sweep: where the crossing actually matters (the decisive axis).
//!
//! The other families pin the residual at a heavy ~256-node tree-walk, so a ~9 ns crossing
//! is invisible against a ~microsecond-per-record payload and every axis reads as "the
//! crossing does not matter". The panel's sharpest finding is that the ABI decision lives at
//! the OTHER end: where the per-record payload is cheap (the native / copy-and-patch tier),
//! the crossing is a large fraction of the work and batching (fewer crossings, and the
//! vectorisation a column enables) is what pays. This family measures that directly.
//!
//! The swept size is the RESIDUAL NODE COUNT (the payload cost), from a 1-node residual (the
//! cheapest interpret, closest to the native-tier regime) to 1024 nodes (heavy). The batch
//! width is fixed at [`W_FIX`]; each cell crosses the whole [`N_TOTAL`]-record column at that
//! width, so only the per-record payload changes across the sweep. The crossing floor
//! (`null_entry`) is constant; its fraction of the scalar payload goes from large (cheap
//! residual) to negligible (heavy residual), which is the `C_cross / I_payload` law measured,
//! and the SoA-vs-scalar gap shows where vectorisation is worth the column entry.

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{
    cross_column, fill_seeds, program_bytes_n, CrEntryW, CrFree, CrInit, StCross, N_TOTAL,
};

/// The fixed batch width for the payload-cost sweep. A mid-column width where the SoA path is
/// fully vectorising (>= the SIMD width) and the crossing count is small, so the swept axis is
/// purely the payload cost.
const W_FIX: usize = 64;

/// Open the runtime, resolve the named entry, and build a handle from `profile`'s residual at
/// `nodes` node count (the swept payload cost). Batch width is fixed at [`W_FIX`].
fn open_payload(profile: &str, nodes: usize, entry_name: &[u8]) -> StCross {
    let rt = super::common::open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let entry: CrEntryW = unsafe { rt.resolve(entry_name) }.expect("entry resolves");
    let bytes = program_bytes_n(profile, nodes);
    let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
    assert!(!handle.is_null(), "cr_init must build a handle for a {nodes}-node residual");
    StCross { rt, handle, entry, free, seeds: vec![0u64; N_TOTAL], w: W_FIX }
}

bench_matrix! {
    name: "abi_payload_cost",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_00b0,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 4, 16, 64, 256, 1024],
    baseline: "scalar_payload",
    floor: "null_entry",
    regime: warm,

    // `n` (the swept size) is the residual node count, not the batch width (fixed at W_FIX).
    setup |profile: &str, n: usize| -> StCross {
        open_payload(profile, n, b"cr_execute_scalar_runtime_w\0")
    }

    cell scalar_payload
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    cell soa_payload
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_payload(profile, n, b"cr_execute_soa_runtime_w\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // the crossing floor: constant across the payload sweep, so its ratio to the scalar
    // payload IS the C_cross / I_payload curve.
    cell null_entry
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_payload(profile, n, b"cr_null_entry\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }
}

#[cfg(test)]
mod tests {
    use super::super::common::{program_bytes_n, runtime_dylib_path, CrFree, CrInit};
    use super::*;
    use crate as c;

    /// The scalar payload crossing matches in-process for residuals across the size sweep, so
    /// the sweep varies only the payload cost, not correctness.
    #[test]
    fn payload_sweep_matches_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let entry: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("scalar runtime-w");

        for nodes in [1usize, 16, 256, 1024] {
            let bytes = program_bytes_n("real", nodes);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());
            let seed = 0xba5e ^ nodes as u64;
            let mut seeds = vec![0u64; N_TOTAL];
            fill_seeds(&mut seeds, seed);
            let host = cross_column(entry, handle, &seeds, W_FIX);

            let mut scratch = vec![0u64; pd.nodes.len()];
            let mut acc = 0u64;
            let mut off = 0usize;
            while off < N_TOTAL {
                let batch = W_FIX.min(N_TOTAL - off);
                let mut inner = 0u64;
                for i in off..off + batch {
                    c::interpret_predecoded(&pd, seeds[i], &mut scratch);
                    inner = inner.rotate_left(7) ^ c::checksum(&scratch);
                }
                acc = acc.rotate_left(7) ^ inner;
                off += W_FIX;
            }
            assert_eq!(host, acc, "payload sweep must match in-process at {nodes} nodes");
            unsafe { free(handle) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_payload_cost");
        assert_eq!(d.sizes, vec![1, 4, 16, 64, 256, 1024], "the residual-size sweep");
        assert_eq!(d.baseline, "scalar_payload");
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["scalar_payload", "soa_payload", "null_entry"]);
    }
}

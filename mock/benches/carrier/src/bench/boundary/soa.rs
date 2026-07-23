//! Boundary bench 3: column-SoA vectorisation win across the boundary.
//!
//! The headline bench. Over the same fixed-column / swept-W shape as bench 1, it varies
//! only the payload: the scalar interpreter versus the SoA-8 vertical interpreter
//! (`carrier-runtime`'s `cr_execute_soa_runtime_w`, which processes a `W`-record batch in
//! `W / 8` SoA-8 passes plus a scalar remainder). Both cells cross identically (runtime-W
//! entry, same `C_cross`), so the delta between them on top of bench 1's shared FFI term
//! isolates the vectorisation win, un-aliased from the crossing cost.
//!
//! ## The batch width is not the SIMD width
//!
//! chris-fallin's constraint (`Simd<u64, W>` is a distinct type per W, so a runtime-W
//! entry could only wrap the scalar interpreter) holds only when the batch width equals
//! the SIMD lane width. `carrier-runtime` decouples them: the SIMD width is a fixed const
//! 8, and the batch width `W` is a runtime argument the internal loop chunks by 8. So the
//! SoA payload is reachable through the same runtime-W entry shape as scalar, and the two
//! payloads are a fair head-to-head at every W. The vectorisation win is the fixed SoA-8
//! speedup; the crossing amortisation is the separate 1/W term bench 1 measures.
//!
//! Cells: `scalar_payload` (baseline, the scalar crossing), `soa_payload` (the SoA-8
//! crossing), `null_entry` (the shared pure-crossing floor). The scalar and SoA folds
//! differ (per-record versus all-lanes, matching the carrier's own vertical bench), so
//! the two are not byte-comparable to each other; each cross-validates against its own
//! in-process replica.

use mockspace_bench_matrix::bench_matrix;

use super::common::{cross_column, fill_seeds, open_and_init, StCross};

bench_matrix! {
    name: "abi_soa_win",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0003,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "scalar_payload",
    floor: "null_entry",
    regime: warm,

    // The scalar crossing (identical to bench 1's ffi cell), the head-to-head baseline.
    setup |profile: &str, n: usize| -> StCross {
        open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
    }

    cell scalar_payload
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // The SoA-8 crossing: same runtime-W entry shape, vertical payload. The win over
    // `scalar_payload` at equal crossing cost is the vectorisation delta.
    cell soa_payload
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_soa_runtime_w\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // The shared pure-crossing floor (empty payload).
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

#[cfg(all(test, feature = "vertical"))]
mod tests {
    use super::super::common::{
        runtime_dylib_path, program_bytes, CrEntryW, CrFree, CrInit, N_TOTAL,
    };
    use super::*;
    use crate as c;
    use mockspace_bench_matrix::boundary::Runtime;

    /// The in-process SoA replica, matching `carrier-runtime`'s `soa_batch` exactly: SoA-8
    /// passes over full 8-chunks, then a scalar remainder, folded per pass. Byte-identical
    /// to the runtime body so the host crossing can be validated against it.
    fn inproc_soa_batch(
        pd: &c::Predecoded,
        seeds: &[u64],
        soa: &mut [core::simd::Simd<u64, 8>],
        scalar: &mut [u64],
        w: usize,
    ) -> u64 {
        let mut acc = 0u64;
        let mut i = 0usize;
        while i + 8 <= w {
            let chunk: &[u64; 8] = seeds[i..i + 8].try_into().unwrap();
            acc = acc.rotate_left(7)
                ^ c::vertical::interpret_vertical_checksum_into::<8>(pd, chunk, soa);
            i += 8;
        }
        while i < w {
            c::interpret_predecoded(pd, seeds[i], scalar);
            acc = acc.rotate_left(7) ^ c::checksum(scalar);
            i += 1;
        }
        acc
    }

    /// The SoA crossing computes byte-exactly what the in-process SoA payload computes,
    /// across the W-sweep. The SoA family's cross-validation gate.
    #[test]
    fn soa_crossing_matches_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let entry: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_soa_runtime_w\0") }.expect("soa runtime-w entry");

        for profile in ["real", "madd", "tight", "leaf"] {
            let bytes = program_bytes(profile);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let n = pd.nodes.len();
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());

            for w in [1usize, 8, 16, 64, 256] {
                let seed = 0xf00d ^ (w as u64).wrapping_mul(0x9e37);
                let mut seeds = vec![0u64; N_TOTAL];
                fill_seeds(&mut seeds, seed);
                let host = cross_column(entry, handle, &seeds, w);

                let mut soa = c::vertical::make_scratch::<8>(n);
                let mut scalar = vec![0u64; n];
                let mut acc = 0u64;
                let mut off = 0usize;
                while off < N_TOTAL {
                    let batch = w.min(N_TOTAL - off);
                    acc = acc.rotate_left(7)
                        ^ inproc_soa_batch(&pd, &seeds[off..off + batch], &mut soa, &mut scalar, batch);
                    off += w;
                }
                assert_eq!(host, acc, "SoA crossing must equal in-process for {profile}, W={w}");
            }
            unsafe { free(handle) };
        }
    }
}

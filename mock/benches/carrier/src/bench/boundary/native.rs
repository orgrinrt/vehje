//! Native-tier crossing amortisation: the payload cheap enough for the crossing to matter.
//!
//! Every interpret payload costs tens of ns to us per record, so the ~9 ns crossing is
//! invisible and batching pays only through vectorisation. The copy-and-patch native tier
//! (compiled machine code, ~ns per record) is the one regime where the crossing is a live
//! fraction: at W=1 the column pays 256 crossings, at W=256 it pays one, so the W-sweep of the
//! native crossing shows whether batching amortises the crossing itself. This is the question
//! the design named and the interpret families could not reach.
//!
//! Cells, over the fixed-column / swept-W shape:
//!
//! - `native_ffi_w` (baseline): the native payload across the real boundary
//!   (`cr_execute_native_runtime_w`), swept W. Its fall from W=1 to W=256 toward `inproc_native`
//!   is the crossing amortisation.
//! - `inproc_native`: the identical native payload run in-process (no crossing), the W-invariant
//!   payload floor. Requires the carrier's copy-and-patch backend (the `jit` feature).
//! - `null_entry`: the crossing floor (empty payload).
//!
//! aarch64 + macOS only (the copy-and-patch backend); the family is empty elsewhere.

use mockspace_bench_matrix::bench_matrix;

use super::common::{
    cross_column, fill_seeds, open_and_init, open_runtime, program_bytes_n, CrEntryW, CrFree,
    CrInit, StCross, N_TOTAL,
};

/// The native residual is deliberately TINY (a few nodes), so the compiled machine code is
/// ~ns per record and the ~7-9 ns crossing is a live fraction. At a realistic residual size
/// even native-compiled code costs microseconds per record (dominated by per-node work and
/// the result reduction), which the payload-cost family already shows drowns the crossing;
/// this family isolates the one regime where crossing amortisation over W is the effect.
const NATIVE_NODES: usize = 4;

/// Open the runtime, resolve the native init/free and the native entry, and compile the
/// residual to machine code. Reuses [`StCross`] with `free` bound to `cr_native_free`, so the
/// native handle is freed correctly on drop.
fn open_native(profile: &str, w: usize) -> StCross {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_native_init\0") }.expect("cr_native_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_native_free\0") }.expect("cr_native_free");
    let entry: CrEntryW =
        unsafe { rt.resolve(b"cr_execute_native_runtime_w\0") }.expect("native entry resolves");
    let bytes = program_bytes_n(profile, NATIVE_NODES);
    let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
    assert!(!handle.is_null(), "cr_native_init must compile the residual");
    StCross { rt, handle, entry, free, seeds: vec![0u64; N_TOTAL], w }
}

/// In-process native state: the compiled machine code and its scratch, run per record with no
/// crossing (the payload floor). The `jit` feature makes the carrier's copy-and-patch backend
/// available.
#[cfg(feature = "jit")]
pub struct StInprocNative {
    pub jit:     crate::copypatch::JitCode,
    pub scratch: Vec<u64>,
    pub seeds:   Vec<u64>,
    pub w:       usize,
}

bench_matrix! {
    name: "abi_native_cross",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_00d0,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "native_ffi_w",
    floor: "null_entry",
    regime: warm,

    // the native payload across the real boundary, swept W (the crossing-amortisation curve).
    setup |profile: &str, n: usize| -> StCross {
        open_native(profile, n)
    }

    cell native_ffi_w
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // the identical native payload in-process (no crossing), the W-invariant floor.
    cell inproc_native
        #[feature = "boundary"]
        #[feature = "jit"]
        setup |profile: &str, n: usize| -> StInprocNative {
            let bytes = program_bytes_n(profile, NATIVE_NODES);
            let d = crate::ir::Decoded::parse(&bytes, crate::ir::REC24).expect("residual parses");
            // reconstruct the program the same way the runtime does, then compile it.
            let consts: Vec<u64> = (0..d.const_count).map(|i| d.const_at(i)).collect();
            let nodes: Vec<crate::ir::Node> = (0..d.node_count)
                .map(|i| {
                    let arity = d.arity_at(i);
                    let operands = (0..arity).map(|k| d.operand(i, k, arity)).collect();
                    crate::ir::Node { op: d.op_at(i), operands }
                })
                .collect();
            let prog = crate::ir::Program { consts, nodes };
            let jit = crate::copypatch::JitCode::new(&prog).expect("jit compiles");
            StInprocNative { jit, scratch: vec![0u64; d.node_count], seeds: vec![0u64; N_TOTAL], w: n }
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            let mut acc = 0u64;
            for i in 0..N_TOTAL {
                s.jit.run(s.seeds[i], &mut s.scratch);
                acc = acc.rotate_left(7) ^ crate::checksum(&s.scratch);
            }
            let _ = s.w;
            acc
        }

    // the crossing floor (empty payload, regular handle which the entry ignores).
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

#[cfg(all(test, feature = "jit"))]
mod tests {
    use super::super::common::{program_bytes_n, runtime_dylib_path, CrFree, CrInit};
    use super::*;
    use crate as c;
    use mockspace_bench_matrix::boundary::Runtime;

    /// The native crossing folds byte-exactly to the in-process interpreter over the same
    /// column: the compiled machine code and the interpreter agree, so the native family
    /// measures cost, not a divergent computation.
    #[test]
    fn native_crossing_matches_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let ninit: CrInit = unsafe { rt.resolve(b"cr_native_init\0") }.expect("cr_native_init");
        let nfree: CrFree = unsafe { rt.resolve(b"cr_native_free\0") }.expect("cr_native_free");
        let entry: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_native_runtime_w\0") }.expect("native entry");

        for profile in ["real", "tight"] {
            let bytes = program_bytes_n(profile, NATIVE_NODES);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let handle = unsafe { ninit(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());
            for w in [1usize, 8, 256] {
                let seed = 0xf17 ^ (w as u64);
                let mut seeds = vec![0u64; N_TOTAL];
                fill_seeds(&mut seeds, seed);
                let host = cross_column(entry, handle, &seeds, w);
                // in-process interpreter reference (same fold structure).
                let mut scratch = vec![0u64; pd.nodes.len()];
                let mut acc = 0u64;
                let mut off = 0usize;
                while off < N_TOTAL {
                    let batch = w.min(N_TOTAL - off);
                    let mut inner = 0u64;
                    for i in off..off + batch {
                        c::interpret_predecoded(&pd, seeds[i], &mut scratch);
                        inner = inner.rotate_left(7) ^ c::checksum(&scratch);
                    }
                    acc = acc.rotate_left(7) ^ inner;
                    off += w;
                }
                assert_eq!(host, acc, "native crossing must match interpreter, {profile} W={w}");
            }
            unsafe { nfree(handle) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_native_cross");
        assert_eq!(d.baseline, "native_ffi_w");
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["native_ffi_w", "inproc_native", "null_entry"]);
    }
}

//! Boundary bench 1: call-crossing amortisation over W, scalar payload pinned.
//!
//! Measures the cost of the host-to-runtime C ABI call itself: feeding a column of
//! records across the boundary a batch of `W` at a time. The one non-deferrable ABI
//! decision is whether the boundary should expose a batched/column entry so the runtime
//! can amortise the crossing over many records, versus a scalar per-record entry that
//! forecloses it. Pinning the payload to scalar here makes it the FFI isolator, so a
//! later SoA bench's vectorisation term separates by construction.
//!
//! ## The measurement shape
//!
//! Total record count [`N_TOTAL`] is held fixed; the swept size axis is the batch width
//! `W`. Each pass processes all [`N_TOTAL`] records in `k = N_TOTAL / W` crossings.
//! Payload work per pass is constant, so fitting `total(k) = S + k * C_cross` over the
//! W-sweep reads the amortisation curve and isolates the per-crossing cost in the slope,
//! while calibration reps stay comparable across W (sidestepping reps-starvation). The
//! C_cross fit is an analysis-time step over the raw `(W, time)` points (`k = N_TOTAL /
//! W` is the fit x-axis, not `W`).
//!
//! ## The floor ladder (rungs a, b, c)
//!
//! - `inproc_direct` (baseline, rung a): the payload in-process, no crossing, direct
//!   inlinable call. The true zero the crossing cost is measured above.
//! - `inproc_fnptr` (rung b): the same payload through a black-boxed `fn` pointer,
//!   isolating indirect-call cost with no object boundary.
//! - `null_entry` (floor, rung c): cross into `cr_null_entry`, empty payload; its
//!   k-slope is C_cross with no payload masking it.
//! - `ffi_batched_scalar`: the real batched crossing into `cr_execute_scalar_runtime_w`.
//!
//! Rungs d (native ceiling) and e (null-dispatch) live in their own families.

use mockspace_bench_matrix::bench_matrix;

use super::common::{
    cross_column, fill_seeds, inproc_scalar_batch, open_and_init, program_bytes, StCross, N_TOTAL,
};
use crate as c;

/// In-process state for the payload floors: the predecoded residual, a reused scratch,
/// the reused seed column, and the batch width.
pub struct StInproc {
    pub pd:      c::Predecoded,
    pub scratch: Vec<u64>,
    pub seeds:   Vec<u64>,
    pub w:       usize,
}

/// Rung-b state: [`StInproc`] plus the black-boxed payload pointer, so the payload is
/// reached through a genuine indirect call rather than an inlined one.
pub struct StFnptr {
    pub pd:      c::Predecoded,
    pub scratch: Vec<u64>,
    pub seeds:   Vec<u64>,
    pub w:       usize,
    pub f:       fn(&c::Predecoded, &[u64], &mut [u64]) -> u64,
}

/// Build the in-process floor predecoded residual and scratch for `profile`.
fn inproc_state(profile: &str) -> (c::Predecoded, Vec<u64>) {
    let bytes = program_bytes(profile);
    let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).expect("residual parses");
    let pd = c::predecode::predecode(&d);
    let scratch = vec![0u64; pd.nodes.len()];
    (pd, scratch)
}

/// The in-process crossing loop shape (rungs a and b): process the column in `w`-sized
/// batches, calling `payload` per batch. Mirrors [`cross_column`] so the crossing cell's
/// excess over rung a is exactly the object-boundary cost.
#[inline(always)]
fn inproc_column(
    pd: &c::Predecoded,
    seeds: &[u64],
    scratch: &mut [u64],
    w: usize,
    payload: impl Fn(&c::Predecoded, &[u64], &mut [u64]) -> u64,
) -> u64 {
    let mut acc = 0u64;
    let mut off = 0usize;
    while off < N_TOTAL {
        let batch = w.min(N_TOTAL - off);
        acc = acc.rotate_left(7) ^ payload(pd, &seeds[off..off + batch], scratch);
        off += w;
    }
    acc
}

bench_matrix! {
    name: "abi_cross_scalar",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0001,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "inproc_direct",
    floor: "null_entry",
    regime: warm,

    // Shared setup: the in-process payload floor state, used by `inproc_direct`. `n` is
    // the swept size, reinterpreted as the batch width `W` (the program size is fixed).
    setup |profile: &str, n: usize| -> StInproc {
        let (pd, scratch) = inproc_state(profile);
        StInproc { pd, scratch, seeds: vec![0u64; N_TOTAL], w: n }
    }

    // Rung a: the payload in-process, no crossing, direct inlinable call.
    cell inproc_direct
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            inproc_column(&s.pd, &s.seeds, &mut s.scratch, s.w, inproc_scalar_batch)
        }

    // Rung b: the same payload through a black-boxed `fn` pointer (genuine indirect
    // dispatch, no object boundary), isolating indirect-call cost.
    cell inproc_fnptr
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StFnptr {
            let (pd, scratch) = inproc_state(profile);
            StFnptr { pd, scratch, seeds: vec![0u64; N_TOTAL], w: n, f: inproc_scalar_batch }
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            // black_box the pointer so the optimiser cannot prove which fn it is and
            // must issue an indirect call (disasm-confirmed by the ISA-shape gate).
            let f = core::hint::black_box(s.f);
            inproc_column(&s.pd, &s.seeds, &mut s.scratch, s.w, f)
        }

    // The real batched crossing into the runtime: k crossings of W records each.
    cell ffi_batched_scalar
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // Rung c: cross into the empty-payload entry. Pure crossing plus marshalling.
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
    use super::super::common::{runtime_dylib_path, CrEntryW, CrFree, CrInit};
    use super::*;
    use mockspace_bench_matrix::boundary::Runtime;

    /// The crossing path computes byte-exactly what the in-process payload computes: the
    /// batched `ffi_batched_scalar` fold equals a faithful in-process replica over the
    /// same column and batch structure. The boundary family's cross-validation gate (the
    /// floors are exempt; only the real crossing needs it).
    #[test]
    fn ffi_crossing_matches_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }
            .expect("built carrier-runtime dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let entry: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("scalar runtime-w entry");

        for profile in ["real", "madd", "tight", "leaf"] {
            let bytes = program_bytes(profile);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());

            for w in [1usize, 2, 8, 64, 256] {
                let seed = 0xdead_beef ^ (w as u64).wrapping_mul(0x1234_5678);
                let mut seeds = vec![0u64; N_TOTAL];
                fill_seeds(&mut seeds, seed);
                let host = cross_column(entry, handle, &seeds, w);

                let mut scratch = vec![0u64; pd.nodes.len()];
                let acc = inproc_column(&pd, &seeds, &mut scratch, w, inproc_scalar_batch);
                assert_eq!(host, acc, "crossing must equal in-process for {profile}, W={w}");
            }
            unsafe { free(handle) };
        }
    }

    /// The generated matrix declaration is the shape the generator consumes.
    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_cross_scalar");
        assert_eq!(d.sweep.values.len(), 6);
        assert_eq!(d.sizes, vec![1, 2, 4, 8, 16, 32, 64, 128, 256]);
        assert_eq!(d.baseline, "inproc_direct");
        assert_eq!(d.floor.as_deref(), Some("null_entry"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["inproc_direct", "inproc_fnptr", "ffi_batched_scalar", "null_entry"]);
        for &sz in &d.sizes {
            assert_eq!(N_TOTAL % (sz as usize), 0, "W={sz} must divide N_TOTAL={N_TOTAL}");
        }
    }
}

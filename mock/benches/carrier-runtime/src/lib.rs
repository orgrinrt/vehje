//! The shared runtime cdylib a boundary bench crosses into.
//!
//! Every entry point wraps the UNMODIFIED carrier bodies (`interpret_predecoded`
//! scalar, `interpret_vertical` SoA), so a boundary number is the real C ABI
//! crossing plus the same payload the in-process carrier benches measure, never a
//! second divergent interpreter. The host bench variants dlopen this ONE object
//! and call a resolved function pointer per batch, so each crossing is a genuine
//! cross-object `blr` the optimizer cannot inline away (the whole point of the
//! two-object split: it makes the per-batch call COUNT a real, un-elidable cost).
//!
//! ## The two widths, kept distinct
//!
//! The ABI **batch width** `W` (records per call, swept 1..256, Wmax 128) is not the
//! **SIMD lane width** ([`LANES`] = 8, the carrier's vert8, the measured column-eval
//! win). A batch of `W` records is interpreted in `W / LANES` SoA passes plus a
//! scalar remainder, so the crossing amortizes over the batch `W` while the
//! vectorization win is the fixed SoA-8 speedup, and the two 1/W knees separate by
//! construction rather than aliasing.
//!
//! ## The two axes the entry points span
//!
//! **Payload**: `scalar_*` runs the scalar interpreter once per record; `soa_*` runs
//! the SoA-8 vertical interpreter over each chunk. **Entry-point form** (the
//! monomorphization vehicle): `*_w{K}` bakes the batch width into the symbol (const
//! `K`, so the per-record / chunk loop unrolls); `*_runtime_w` takes `w` as a runtime
//! argument (register loop bound, no unroll); `*_dispatch` is one symbol that matches
//! `w` to the const bodies. Because the SIMD width is a fixed const and not `W`
//! itself, even the SoA runtime-W form is expressible. `execute1` is the scalar W=1
//! anchor; `null_entry` is the empty-payload floor (crosses and marshals, no
//! interpret). Each family folds its own reps-invariant keep-alive; cross-payload
//! fidelity (SoA lane l equals scalar seed l) is a `cargo test`, matching the
//! carrier's own vertical bench, which likewise folds scalar and SoA differently.

#![feature(portable_simd)]

use core::ffi::c_void;
use core::simd::Simd;

use vehje_bench_carrier as c;

/// The SIMD lane width the SoA payload uses. Fixed at the carrier's vert8 (the
/// measured column-eval win), independent of the ABI batch width.
const LANES: usize = 8;

/// An opaque runtime handle: the predecoded program plus reusable scratch.
///
/// Built once from wire bytes at [`cr_init`] (the residual crossing into the
/// runtime), then invoked many times over batches of records, modelling a compiled
/// unit loaded once and called per batch. The scratch is reused across calls (each
/// call overwrites it), so no allocation is charged to the timed region, matching
/// the carrier's own reuse discipline.
pub struct Handle {
    pd:             c::predecode::Predecoded,
    scalar_scratch: Vec<u64>,
    soa_scratch:    Vec<Simd<u64, LANES>>,
}

/// Build a runtime handle from a program's wire bytes (REC24). Returns null if the
/// bytes do not parse. The host calls this in its bench `setup` (timed once as S),
/// never in the timed cell.
///
/// # Safety
/// `bytes` must point to `len` valid bytes of a REC24 carrier program.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_init(bytes: *const u8, len: usize) -> *mut Handle {
    let slice = unsafe { core::slice::from_raw_parts(bytes, len) };
    let Some(d) = c::ir::Decoded::parse(slice, c::ir::REC24) else {
        return core::ptr::null_mut();
    };
    let pd = c::predecode::predecode(&d);
    let n = pd.nodes.len();
    let handle = Box::new(Handle {
        scalar_scratch: vec![0u64; n],
        soa_scratch: c::vertical::make_scratch::<LANES>(n),
        pd,
    });
    Box::into_raw(handle)
}

/// Free a runtime handle. Null is a no-op.
///
/// # Safety
/// `h` must be a pointer returned by [`cr_init`] and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_free(h: *mut Handle) {
    if !h.is_null() {
        drop(unsafe { Box::from_raw(h) });
    }
}

// ── the payload helpers, inlined so a const batch width unrolls ──

/// Scalar payload: interpret each of `w` records once, folding a per-record
/// keep-alive checksum. `#[inline(always)]` so a const `w` (the `scalar_w{K}`
/// entries) unrolls the loop while a runtime `w` keeps a register bound.
///
/// # Safety
/// `seeds` has length `w`; `h`'s scratch is sized to the program.
#[inline(always)]
unsafe fn scalar_batch(h: &mut Handle, seeds: &[u64], w: usize) -> u64 {
    let mut acc = 0u64;
    let mut i = 0usize;
    while i < w {
        c::interpret_predecoded(&h.pd, seeds[i], &mut h.scalar_scratch);
        acc = acc.rotate_left(7) ^ c::checksum(&h.scalar_scratch);
        i += 1;
    }
    acc
}

/// SoA payload: interpret `w` records in `w / LANES` SoA-8 passes plus a scalar
/// remainder, folding one keep-alive. `#[inline(always)]` for the same const-vs-
/// runtime `w` unroll distinction as [`scalar_batch`].
///
/// # Safety
/// `seeds` has length `w`; `h`'s scratch is sized to the program.
#[inline(always)]
unsafe fn soa_batch(h: &mut Handle, seeds: &[u64], w: usize) -> u64 {
    let mut acc = 0u64;
    let mut i = 0usize;
    while i + LANES <= w {
        // >= LANES elements remain here, so the fixed-array cast is in bounds.
        let chunk: &[u64; LANES] = unsafe { &*(seeds.as_ptr().add(i) as *const [u64; LANES]) };
        acc = acc.rotate_left(7)
            ^ c::vertical::interpret_vertical_checksum_into::<LANES>(&h.pd, chunk, &mut h.soa_scratch);
        i += LANES;
    }
    while i < w {
        c::interpret_predecoded(&h.pd, seeds[i], &mut h.scalar_scratch);
        acc = acc.rotate_left(7) ^ c::checksum(&h.scalar_scratch);
        i += 1;
    }
    acc
}

// ── W=1 anchor and the empty-payload floor ──

/// Scalar, one record. The W=1 anchor every batched form is measured against.
///
/// # Safety
/// `h` is a live handle from [`cr_init`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute1(h: *mut Handle, seed: u64) -> u64 {
    let h = unsafe { &mut *h };
    c::interpret_predecoded(&h.pd, seed, &mut h.scalar_scratch);
    c::checksum(&h.scalar_scratch)
}

/// The empty-payload floor: cross the boundary, marshal `w` seeds, do NO interpret
/// work, return a keep-alive folded from the inputs so the call is not hoistable.
/// `null_entry(W)` alone is the pure crossing plus marshalling; any real entry minus
/// it is the payload.
///
/// # Safety
/// `seeds` points to `w` valid `u64`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_null_entry(_h: *mut Handle, seeds: *const u64, w: usize) -> u64 {
    let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
    let mut acc = 0u64;
    for &s in seeds {
        acc = acc.rotate_left(7) ^ s;
    }
    acc
}

// ── runtime-W entries (register loop bound, no unroll) ──

/// Scalar payload, runtime batch width. The W-invariant scalar isolator: a bench
/// pinned to this separates the crossing's 1/W term from the SoA payload's.
///
/// # Safety
/// `seeds` points to `w` valid `u64`s; `h` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_scalar_runtime_w(h: *mut Handle, seeds: *const u64, w: usize) -> u64 {
    let h = unsafe { &mut *h };
    let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
    unsafe { scalar_batch(h, seeds, w) }
}

/// SoA payload, runtime batch width. The comparator for the monomorphized SoA
/// entries.
///
/// # Safety
/// `seeds` points to `w` valid `u64`s; `h` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_soa_runtime_w(h: *mut Handle, seeds: *const u64, w: usize) -> u64 {
    let h = unsafe { &mut *h };
    let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
    unsafe { soa_batch(h, seeds, w) }
}

// ── per-W-monomorphized entries (const batch width bakes into the symbol) ──

/// Emit one per-W-monomorphized entry over a payload helper: the batch width `W` is
/// a constant, so the helper's loop bound is constant and unrolls.
macro_rules! mono {
    ($name:ident, $helper:ident, $w:literal) => {
        /// Per-W-monomorphized entry (const batch width, unrolled loop).
        ///
        /// # Safety
        /// `seeds` points to `$w` valid `u64`s; `h` is live.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(h: *mut Handle, seeds: *const u64) -> u64 {
            let h = unsafe { &mut *h };
            let seeds = unsafe { core::slice::from_raw_parts(seeds, $w) };
            unsafe { $helper(h, seeds, $w) }
        }
    };
}

mono!(cr_execute_scalar_w1, scalar_batch, 1);
mono!(cr_execute_scalar_w2, scalar_batch, 2);
mono!(cr_execute_scalar_w4, scalar_batch, 4);
mono!(cr_execute_scalar_w8, scalar_batch, 8);
mono!(cr_execute_scalar_w16, scalar_batch, 16);
mono!(cr_execute_scalar_w32, scalar_batch, 32);
mono!(cr_execute_scalar_w64, scalar_batch, 64);
mono!(cr_execute_scalar_w128, scalar_batch, 128);

mono!(cr_execute_soa_w1, soa_batch, 1);
mono!(cr_execute_soa_w2, soa_batch, 2);
mono!(cr_execute_soa_w4, soa_batch, 4);
mono!(cr_execute_soa_w8, soa_batch, 8);
mono!(cr_execute_soa_w16, soa_batch, 16);
mono!(cr_execute_soa_w32, soa_batch, 32);
mono!(cr_execute_soa_w64, soa_batch, 64);
mono!(cr_execute_soa_w128, soa_batch, 128);

// ── boundary-cold call targets (distinct addresses, identical work) ──
//
// Sixteen entries with the identical scalar_batch body but distinct addresses, so a
// boundary-cold cell can cycle its crossings across many call targets and defeat the CPU's
// indirect-branch target predictor (the warm cells always cross one resolved pointer, a
// perfectly predicted `blr`, so they measure the best-case crossing; cycling distinct
// targets measures the mispredicted crossing the real varying-call-target case pays). A
// `black_box` of a distinct per-target constant (discarded) keeps identical-code-folding
// from merging the sixteen bodies back into one address under fat LTO, without changing the
// result: every target computes the identical fold, so a cold cell cross-validates against
// the warm scalar entry byte-exact.

/// Emit one cold call target: the scalar batch body, made a distinct address by a discarded
/// `black_box` of `$tag`.
macro_rules! cold_target {
    ($name:ident, $tag:literal) => {
        /// A boundary-cold call target: identical work, distinct address.
        ///
        /// # Safety
        /// `seeds` points to `w` valid `u64`s; `h` is live.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(h: *mut Handle, seeds: *const u64, w: usize) -> u64 {
            // distinct, discarded: prevents ICF from folding the 16 bodies to one address.
            let _ = core::hint::black_box($tag as u64);
            let h = unsafe { &mut *h };
            let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
            unsafe { scalar_batch(h, seeds, w) }
        }
    };
}

cold_target!(cr_cold_target_0, 0);
cold_target!(cr_cold_target_1, 1);
cold_target!(cr_cold_target_2, 2);
cold_target!(cr_cold_target_3, 3);
cold_target!(cr_cold_target_4, 4);
cold_target!(cr_cold_target_5, 5);
cold_target!(cr_cold_target_6, 6);
cold_target!(cr_cold_target_7, 7);
cold_target!(cr_cold_target_8, 8);
cold_target!(cr_cold_target_9, 9);
cold_target!(cr_cold_target_10, 10);
cold_target!(cr_cold_target_11, 11);
cold_target!(cr_cold_target_12, 12);
cold_target!(cr_cold_target_13, 13);
cold_target!(cr_cold_target_14, 14);
cold_target!(cr_cold_target_15, 15);

/// Emit one empty-payload cold target: the `cr_null_entry` fold (cross plus marshal, no
/// interpret) at a distinct address, so a cold cell isolates the crossing-misprediction
/// penalty with no payload masking it.
macro_rules! cold_null {
    ($name:ident, $tag:literal) => {
        /// A boundary-cold empty-payload target: identical fold, distinct address.
        ///
        /// # Safety
        /// `seeds` points to `w` valid `u64`s.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(_h: *mut Handle, seeds: *const u64, w: usize) -> u64 {
            let _ = core::hint::black_box($tag as u64);
            let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
            let mut acc = 0u64;
            for &s in seeds {
                acc = acc.rotate_left(7) ^ s;
            }
            acc
        }
    };
}

cold_null!(cr_cold_null_0, 100);
cold_null!(cr_cold_null_1, 101);
cold_null!(cr_cold_null_2, 102);
cold_null!(cr_cold_null_3, 103);
cold_null!(cr_cold_null_4, 104);
cold_null!(cr_cold_null_5, 105);
cold_null!(cr_cold_null_6, 106);
cold_null!(cr_cold_null_7, 107);
cold_null!(cr_cold_null_8, 108);
cold_null!(cr_cold_null_9, 109);
cold_null!(cr_cold_null_10, 110);
cold_null!(cr_cold_null_11, 111);
cold_null!(cr_cold_null_12, 112);
cold_null!(cr_cold_null_13, 113);
cold_null!(cr_cold_null_14, 114);
cold_null!(cr_cold_null_15, 115);

// ── dispatch-table entries (one symbol, match w to the const bodies) ──

/// Emit a dispatch-table entry: one exported symbol, `match w` to the const-W
/// bodies, falling back to the runtime-W body past Wmax (128). Each arm calls the
/// `#[inline(always)]` payload helper with a LITERAL width, so the arm is per-W
/// monomorphized (unrolled) via inlining, without a distinct exported symbol. The
/// shape a real ABI most plausibly ships (bounded symbol surface, still per-arm
/// monomorphized), and the caller crosses once regardless of which arm fires.
macro_rules! dispatch {
    ($name:ident, $helper:ident) => {
        /// Dispatch-table entry: one symbol, per-arm monomorphized, one crossing.
        ///
        /// # Safety
        /// `seeds` points to `w` valid `u64`s; `h` is live.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(h: *mut Handle, seeds: *const u64, w: usize) -> u64 {
            let hr = unsafe { &mut *h };
            unsafe {
                match w {
                    1 => $helper(hr, core::slice::from_raw_parts(seeds, 1), 1),
                    2 => $helper(hr, core::slice::from_raw_parts(seeds, 2), 2),
                    4 => $helper(hr, core::slice::from_raw_parts(seeds, 4), 4),
                    8 => $helper(hr, core::slice::from_raw_parts(seeds, 8), 8),
                    16 => $helper(hr, core::slice::from_raw_parts(seeds, 16), 16),
                    32 => $helper(hr, core::slice::from_raw_parts(seeds, 32), 32),
                    64 => $helper(hr, core::slice::from_raw_parts(seeds, 64), 64),
                    128 => $helper(hr, core::slice::from_raw_parts(seeds, 128), 128),
                    _ => $helper(hr, core::slice::from_raw_parts(seeds, w), w),
                }
            }
        }
    };
}

dispatch!(cr_execute_scalar_dispatch, scalar_batch);
dispatch!(cr_execute_soa_dispatch, soa_batch);

// ── the output sink: the settled reserve/commit two-function-pointer struct ──

/// The value-arena output sink, the settled ABI return contract's mechanism: results
/// flow out of the runtime through a host-supplied `reserve`/`commit` pair rather than a
/// return value. `reserve(userdata, hint)` yields a buffer of `hint` `u64` slots the
/// runtime writes into; `commit(userdata, n)` publishes `n` written slots. Both pointers
/// point into the HOST object, so every reserve/commit is a genuine reverse crossing (the
/// sink's cost, two crossings per batch or per record on top of the execute crossing).
///
/// `#[repr(C)]` so the host declares a layout-identical struct; passed by pointer and
/// called indirect, never inlined. The host owns the arena `userdata` points at.
#[repr(C)]
pub struct CrSink {
    /// Reserve `hint` `u64` output slots, returning a writable pointer to them.
    pub reserve:  unsafe extern "C" fn(*mut c_void, usize) -> *mut u64,
    /// Publish `n` written slots (advancing the host arena).
    pub commit:   unsafe extern "C" fn(*mut c_void, usize),
    /// Opaque host state (the output arena) the pointers operate on.
    pub userdata: *mut c_void,
}

/// Batched sink: interpret the whole `w`-record batch, reserve `w` slots ONCE, write the
/// `w` per-record checksums, commit ONCE. Two reverse crossings per batch (the batched
/// columnar sink shape). No return value; the results are in the committed arena.
///
/// # Safety
/// `seeds` points to `w` valid `u64`s; `h` is live; `sink`'s pointers are valid and its
/// `reserve(w)` yields at least `w` writable `u64` slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_sink_batched(
    h: *mut Handle,
    seeds: *const u64,
    w: usize,
    sink: *const CrSink,
) {
    let h = unsafe { &mut *h };
    let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
    let sink = unsafe { &*sink };
    let out = unsafe { (sink.reserve)(sink.userdata, w) };
    let slots = unsafe { core::slice::from_raw_parts_mut(out, w) };
    for i in 0..w {
        c::interpret_predecoded(&h.pd, seeds[i], &mut h.scalar_scratch);
        slots[i] = c::checksum(&h.scalar_scratch);
    }
    unsafe { (sink.commit)(sink.userdata, w) };
}

/// Per-record sink: for each of the `w` records, reserve ONE slot, interpret, write, and
/// commit ONE. Two reverse crossings PER RECORD (the per-record reserve/commit shape, the
/// `w`-fold sink penalty a batched entry avoids).
///
/// # Safety
/// `seeds` points to `w` valid `u64`s; `h` is live; `sink`'s pointers are valid and each
/// `reserve(1)` yields one writable `u64` slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_sink_per_record(
    h: *mut Handle,
    seeds: *const u64,
    w: usize,
    sink: *const CrSink,
) {
    let h = unsafe { &mut *h };
    let seeds = unsafe { core::slice::from_raw_parts(seeds, w) };
    let sink = unsafe { &*sink };
    for i in 0..w {
        let out = unsafe { (sink.reserve)(sink.userdata, 1) };
        c::interpret_predecoded(&h.pd, seeds[i], &mut h.scalar_scratch);
        unsafe { *out = c::checksum(&h.scalar_scratch) };
        unsafe { (sink.commit)(sink.userdata, 1) };
    }
}

// ── the marshalling entries: multi-field records in AoS vs SoA layout ──
//
// A record carries `f` fields (u64 each); the runtime combines them to a seed (XOR fold,
// layout-independent) and runs the scalar interpreter, so bench 2 measures the marshalling
// access pattern of AoS versus SoA BEFORE any downstream vectorisation, with the payload
// held scalar. AoS is record-major (`recs[i*f + k]` = field k of record i, a contiguous
// per-record read at stride f); SoA is field-major (`fields[k*n + i]`, f separate streams).
// The XOR combine is commutative, so AoS and SoA fold the identical checksum for the same
// logical records, which is the cross-validation the layouts rest on.

/// Combine a record's `f` fields to a seed from an AoS (record-major) buffer.
#[inline(always)]
unsafe fn combine_aos(recs: *const u64, i: usize, f: usize) -> u64 {
    let mut seed = 0u64;
    let mut k = 0usize;
    while k < f {
        seed ^= unsafe { *recs.add(i * f + k) };
        k += 1;
    }
    seed
}

/// Combine a record's `f` fields to a seed from a SoA (field-major) buffer of `n` records.
#[inline(always)]
unsafe fn combine_soa(fields: *const u64, i: usize, n: usize, f: usize) -> u64 {
    let mut seed = 0u64;
    let mut k = 0usize;
    while k < f {
        seed ^= unsafe { *fields.add(k * n + i) };
        k += 1;
    }
    seed
}

/// AoS marshalling: read each of `n` records' `f` fields record-major, combine, interpret,
/// fold. `recs` points to `n * f` `u64`s.
///
/// # Safety
/// `recs` points to `n * f` valid `u64`s; `h` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_marshal_aos(h: *mut Handle, recs: *const u64, n: usize, f: usize) -> u64 {
    let h = unsafe { &mut *h };
    let mut acc = 0u64;
    let mut i = 0usize;
    while i < n {
        let seed = unsafe { combine_aos(recs, i, f) };
        c::interpret_predecoded(&h.pd, seed, &mut h.scalar_scratch);
        acc = acc.rotate_left(7) ^ c::checksum(&h.scalar_scratch);
        i += 1;
    }
    acc
}

/// SoA marshalling: read each of `n` records' `f` fields field-major, combine, interpret,
/// fold. `fields` points to `f * n` `u64`s. Folds identically to [`cr_execute_marshal_aos`]
/// for the same logical records.
///
/// # Safety
/// `fields` points to `f * n` valid `u64`s; `h` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_marshal_soa(h: *mut Handle, fields: *const u64, n: usize, f: usize) -> u64 {
    let h = unsafe { &mut *h };
    let mut acc = 0u64;
    let mut i = 0usize;
    while i < n {
        let seed = unsafe { combine_soa(fields, i, n, f) };
        c::interpret_predecoded(&h.pd, seed, &mut h.scalar_scratch);
        acc = acc.rotate_left(7) ^ c::checksum(&h.scalar_scratch);
        i += 1;
    }
    acc
}

/// The marshalling floor: read and combine each record's `f` fields (AoS), fold the seeds,
/// NO interpret. Isolates the pure AoS marshalling cost; a marshalling cell minus this is
/// the interpret payload.
///
/// # Safety
/// `recs` points to `n * f` valid `u64`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cr_execute_marshal_null_aos(_h: *mut Handle, recs: *const u64, n: usize, f: usize) -> u64 {
    let mut acc = 0u64;
    let mut i = 0usize;
    while i < n {
        let seed = unsafe { combine_aos(recs, i, f) };
        acc = acc.rotate_left(7) ^ seed;
        i += 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program_bytes() -> Vec<u8> {
        // a real balanced program, the same shape the benches interpret.
        c::program_at(256, c::ir::REC24)
    }

    unsafe fn handle() -> *mut Handle {
        let bytes = program_bytes();
        let h = unsafe { cr_init(bytes.as_ptr(), bytes.len()) };
        assert!(!h.is_null(), "a valid REC24 program must build a handle");
        h
    }

    #[test]
    fn init_rejects_garbage_without_crashing() {
        let junk = [0xffu8; 3];
        let h = unsafe { cr_init(junk.as_ptr(), junk.len()) };
        assert!(h.is_null(), "unparseable bytes must return null, not crash");
    }

    #[test]
    fn every_entry_is_deterministic_and_nonzero() {
        let h = unsafe { handle() };
        let seeds: Vec<u64> = (0..128u64).map(|i| i.wrapping_mul(0x9e37_79b9_7f4a_7c15)).collect();
        // each entry, called twice on the same inputs, gives the same nonzero fold:
        // the handle's scratch is overwritten per call (idempotent), the contract the
        // calibration relies on.
        macro_rules! det {
            ($call:expr) => {{
                let a = unsafe { $call };
                let b = unsafe { $call };
                assert_eq!(a, b, "entry must be idempotent across calls");
                assert_ne!(a, 0, "entry must fold a nonzero keep-alive");
            }};
        }
        det!(cr_execute1(h, seeds[0]));
        det!(cr_execute_scalar_runtime_w(h, seeds.as_ptr(), 64));
        det!(cr_execute_soa_runtime_w(h, seeds.as_ptr(), 64));
        det!(cr_execute_scalar_w64(h, seeds.as_ptr()));
        det!(cr_execute_soa_w64(h, seeds.as_ptr()));
        det!(cr_execute_scalar_dispatch(h, seeds.as_ptr(), 64));
        det!(cr_execute_soa_dispatch(h, seeds.as_ptr(), 128));
        det!(cr_null_entry(h, seeds.as_ptr(), 64));
        unsafe { cr_free(h) };
    }

    #[test]
    fn entry_forms_agree_within_a_payload() {
        // the three entry forms of one payload compute the identical fold on the same
        // batch: they differ only in where the W-to-body binding happens, not in what
        // they compute. This is the per-payload cross-validation the harness relies on.
        let h = unsafe { handle() };
        let seeds: Vec<u64> = (0..64u64).map(|i| 0xabcd ^ i.wrapping_mul(0x100_0001)).collect();
        let sr = unsafe { cr_execute_scalar_runtime_w(h, seeds.as_ptr(), 64) };
        let sm = unsafe { cr_execute_scalar_w64(h, seeds.as_ptr()) };
        let sd = unsafe { cr_execute_scalar_dispatch(h, seeds.as_ptr(), 64) };
        assert_eq!(sr, sm, "scalar runtime-W and monomorphized must agree");
        assert_eq!(sr, sd, "scalar runtime-W and dispatch-table must agree");
        let vr = unsafe { cr_execute_soa_runtime_w(h, seeds.as_ptr(), 64) };
        let vm = unsafe { cr_execute_soa_w64(h, seeds.as_ptr()) };
        let vd = unsafe { cr_execute_soa_dispatch(h, seeds.as_ptr(), 64) };
        assert_eq!(vr, vm, "SoA runtime-W and monomorphized must agree");
        assert_eq!(vr, vd, "SoA runtime-W and dispatch-table must agree");
        unsafe { cr_free(h) };
    }

    /// A test host sink: an arena the reserve/commit pair writes into, so the sink path
    /// can be validated against the in-process per-record checksums.
    struct TestArena {
        buf: Vec<u64>,
        off: usize,
    }

    unsafe extern "C" fn test_reserve(ud: *mut c_void, _hint: usize) -> *mut u64 {
        let a = unsafe { &mut *(ud as *mut TestArena) };
        unsafe { a.buf.as_mut_ptr().add(a.off) }
    }

    unsafe extern "C" fn test_commit(ud: *mut c_void, n: usize) {
        let a = unsafe { &mut *(ud as *mut TestArena) };
        a.off += n;
    }

    #[test]
    fn sink_writes_per_record_checksums_batched_equals_per_record() {
        let h = unsafe { handle() };
        let seeds: Vec<u64> = (0..64u64).map(|i| 0x1357 ^ i.wrapping_mul(0x9e37_79b9)).collect();
        let w = seeds.len();

        // in-process reference: the per-record checksums the sink must write.
        let hr = unsafe { &mut *h };
        let mut scratch = vec![0u64; hr.pd.nodes.len()];
        let want: Vec<u64> = seeds
            .iter()
            .map(|&sd| {
                c::interpret_predecoded(&hr.pd, sd, &mut scratch);
                c::checksum(&scratch)
            })
            .collect();

        // batched sink: one reserve/commit for the whole batch.
        let mut arena = TestArena { buf: vec![0u64; w], off: 0 };
        let sink = CrSink {
            reserve: test_reserve,
            commit: test_commit,
            userdata: &mut arena as *mut _ as *mut c_void,
        };
        unsafe { cr_execute_sink_batched(h, seeds.as_ptr(), w, &sink) };
        assert_eq!(arena.off, w, "batched sink commits the whole batch");
        assert_eq!(arena.buf, want, "batched sink writes the per-record checksums");

        // per-record sink: reserve/commit per record, same committed contents.
        let mut arena2 = TestArena { buf: vec![0u64; w], off: 0 };
        let sink2 = CrSink {
            reserve: test_reserve,
            commit: test_commit,
            userdata: &mut arena2 as *mut _ as *mut c_void,
        };
        unsafe { cr_execute_sink_per_record(h, seeds.as_ptr(), w, &sink2) };
        assert_eq!(arena2.off, w, "per-record sink commits every record");
        assert_eq!(arena2.buf, want, "per-record sink writes the same checksums as batched");

        unsafe { cr_free(h) };
    }

    #[test]
    fn cold_targets_are_distinct_addresses_with_identical_fold() {
        let h = unsafe { handle() };
        let seeds: Vec<u64> = (0..64u64).map(|i| 0x2222 ^ i.wrapping_mul(0x9e37_79b9)).collect();
        let w = seeds.len();
        // distinct addresses: ICF must not have folded the 16 bodies to one.
        let targets: [unsafe extern "C" fn(*mut Handle, *const u64, usize) -> u64; 16] = [
            cr_cold_target_0, cr_cold_target_1, cr_cold_target_2, cr_cold_target_3,
            cr_cold_target_4, cr_cold_target_5, cr_cold_target_6, cr_cold_target_7,
            cr_cold_target_8, cr_cold_target_9, cr_cold_target_10, cr_cold_target_11,
            cr_cold_target_12, cr_cold_target_13, cr_cold_target_14, cr_cold_target_15,
        ];
        let addrs: std::collections::HashSet<usize> =
            targets.iter().map(|f| *f as usize).collect();
        assert_eq!(addrs.len(), 16, "the 16 cold targets must have distinct addresses (no ICF)");
        // identical fold: every cold target equals the warm scalar entry.
        let want = unsafe { cr_execute_scalar_runtime_w(h, seeds.as_ptr(), w) };
        for (i, f) in targets.iter().enumerate() {
            let got = unsafe { f(h, seeds.as_ptr(), w) };
            assert_eq!(got, want, "cold target {i} must fold identically to the warm entry");
        }
        unsafe { cr_free(h) };
    }

    #[test]
    fn marshal_aos_equals_soa_for_the_same_records() {
        let h = unsafe { handle() };
        let n = 48usize;
        for f in [1usize, 2, 4, 8] {
            // logical records: field k of record i is a deterministic function of (i, k).
            let val = |i: usize, k: usize| (i as u64).wrapping_mul(0x1000_0001) ^ (k as u64).wrapping_mul(0x9e37);
            let mut aos = vec![0u64; n * f];
            let mut soa = vec![0u64; f * n];
            for i in 0..n {
                for k in 0..f {
                    aos[i * f + k] = val(i, k);
                    soa[k * n + i] = val(i, k);
                }
            }
            let a = unsafe { cr_execute_marshal_aos(h, aos.as_ptr(), n, f) };
            let s = unsafe { cr_execute_marshal_soa(h, soa.as_ptr(), n, f) };
            assert_eq!(a, s, "AoS and SoA must fold identically for f={f}");
            assert_ne!(a, 0, "marshalling must fold a nonzero keep-alive for f={f}");
        }
        unsafe { cr_free(h) };
    }

    #[test]
    fn soa_lane_l_equals_scalar_seed_l_fidelity() {
        // the fidelity anchor the cross-payload comparison rests on: the SoA payload's
        // lane l computes the identical per-node results the scalar payload computes
        // for seed l. Checked through the carrier the runtime wraps, so the wrappers
        // are proven to preserve it.
        let bytes = program_bytes();
        let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
        let pd = c::predecode::predecode(&d);
        let n = pd.nodes.len();
        let seeds: [u64; LANES] = core::array::from_fn(|l| 0x55 ^ (l as u64).wrapping_mul(0x9e37_79b9));
        let mut soa = c::vertical::make_scratch::<LANES>(n);
        c::vertical::interpret_vertical::<LANES>(&pd, &seeds, &mut soa);
        let mut scalar = vec![0u64; n];
        for (l, &seed) in seeds.iter().enumerate() {
            c::interpret_predecoded(&pd, seed, &mut scalar);
            for (node, sv) in scalar.iter().enumerate() {
                assert_eq!(*sv, soa[node].as_array()[l], "SoA lane {l} must equal scalar seed {l} at node {node}");
            }
        }
    }
}

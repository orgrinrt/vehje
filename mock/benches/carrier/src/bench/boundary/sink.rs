//! Boundary bench 5: output-sink shape.
//!
//! The other half of the ABI: how results flow BACK. Over the same fixed-column /
//! swept-W shape, holds the execute crossing constant and varies the sink. Per the
//! settled return contract (fallibility is a value property; batch completion is a
//! stream property on the reserve/commit sink), the mechanism is the real `#[repr(C)]`
//! two-function-pointer sink struct, passed by pointer and called indirect (never an
//! inlinable closure), so the reverse crossings the sink costs are genuine.
//!
//! Cells (all scalar payload):
//!
//! - `null_sink` (baseline): the internal-fold scalar entry, no sink. The execute-only
//!   reference the sink cost is measured above.
//! - `batched_sink`: `cr_execute_sink_batched`, one reserve plus one commit per batch
//!   (two reverse crossings per batch). The host arena receives W results; the cell reads
//!   one, so the read-back cost is excluded.
//! - `per_record_sink`: `cr_execute_sink_per_record`, one reserve plus one commit per
//!   record (two reverse crossings per record). The W-fold sink penalty a batched sink
//!   avoids.
//! - `batched_sink_decode`: the batched sink plus a host-side structural decode of the
//!   whole committed W-output arena, so the read-back is charged.
//!
//! The host owns the output arena; `host_reserve`/`host_commit` are the sink pointers the
//! runtime calls back into. The arena is reused across the calibrated loop and never
//! reallocates (sized once to [`N_TOTAL`]), so no allocation is charged to the timed cell.

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{
    cross_column, fill_seeds, init_handle, open_and_init, open_runtime, CrEntrySink, CrFree, CrInit,
    CrSink, StCross, N_TOTAL,
};

/// The host output arena the sink writes into: a fixed buffer and a commit offset. Reset
/// to `off = 0` before each timed crossing of the column.
pub struct SinkArena {
    pub buf: Vec<u64>,
    pub off: usize,
}

/// Reserve: return a writable pointer at the current commit offset. `hint` is advisory
/// (the buffer is pre-sized to the column), so the arena never grows in the timed region.
///
/// # Safety
/// `ud` points to a live [`SinkArena`] whose `buf` has room for the runtime's writes.
unsafe extern "C" fn host_reserve(ud: *mut c_void, _hint: usize) -> *mut u64 {
    let a = unsafe { &mut *(ud as *mut SinkArena) };
    unsafe { a.buf.as_mut_ptr().add(a.off) }
}

/// Commit: advance the arena by `n` published slots.
///
/// # Safety
/// `ud` points to a live [`SinkArena`].
unsafe extern "C" fn host_commit(ud: *mut c_void, n: usize) {
    let a = unsafe { &mut *(ud as *mut SinkArena) };
    a.off += n;
}

/// State for a sink-driven cell: the runtime, handle, resolved sink entry, free pointer,
/// the reused seed column and batch width, and the host output arena.
pub struct StSink {
    pub rt:     Runtime,
    pub handle: *mut c_void,
    pub entry:  CrEntrySink,
    pub free:   CrFree,
    pub seeds:  Vec<u64>,
    pub w:      usize,
    pub arena:  SinkArena,
}

impl Drop for StSink {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.free)(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

fn open_sink(profile: &str, w: usize, entry_name: &[u8]) -> StSink {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let entry: CrEntrySink = unsafe { rt.resolve(entry_name) }.expect("sink entry resolves");
    let handle = init_handle(&rt, init, profile);
    StSink { rt, handle, entry, free, seeds: vec![0u64; N_TOTAL], w, arena: SinkArena { buf: vec![0u64; N_TOTAL], off: 0 } }
}

/// Cross a sink entry over the whole column in `k = N_TOTAL / w` crossings, resetting the
/// arena first and building the `#[repr(C)]` sink over the host pointers. Returns a
/// keep-alive derived from the committed arena so the crossings are not hoistable, without
/// reading the whole arena (the decode variant does that separately).
#[inline(always)]
fn sink_column(s: &mut StSink) -> u64 {
    s.arena.off = 0;
    let sink = CrSink {
        reserve: host_reserve,
        commit: host_commit,
        userdata: &mut s.arena as *mut SinkArena as *mut c_void,
    };
    let mut off = 0usize;
    while off < N_TOTAL {
        let batch = s.w.min(N_TOTAL - off);
        unsafe { (s.entry)(s.handle, s.seeds.as_ptr().add(off), batch, &sink) };
        off += s.w;
    }
    // one committed value plus the commit offset: reps-variant, no full read-back.
    s.arena.buf[0].wrapping_add(s.arena.off as u64)
}

bench_matrix! {
    name: "abi_sink",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0005,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "null_sink",
    floor: "null_sink",
    regime: warm,

    // The batched sink is the shared setup (used by `batched_sink` and, with the same
    // state, `batched_sink_decode`). `n` is the batch width `W`.
    setup |profile: &str, n: usize| -> StSink {
        open_sink(profile, n, b"cr_execute_sink_batched\0")
    }

    // One reserve/commit per batch; the cell reads one committed value (read-back excluded).
    cell batched_sink
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            sink_column(s)
        }

    // The batched sink plus a full host-side decode of the committed W-output arena.
    cell batched_sink_decode
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            let _ = sink_column(s);
            // charge the read-back: fold the whole committed arena.
            let mut acc = 0u64;
            for &v in &s.arena.buf[..N_TOTAL] {
                acc = acc.rotate_left(7) ^ v;
            }
            acc
        }

    // One reserve/commit per RECORD (two reverse crossings per record).
    cell per_record_sink
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StSink {
            open_sink(profile, n, b"cr_execute_sink_per_record\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            sink_column(s)
        }

    // The execute-only baseline: internal fold, no sink. The sink cost is measured above.
    cell null_sink
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_execute_scalar_runtime_w\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }
}

#[cfg(test)]
mod tests {
    use super::super::common::{program_bytes, runtime_dylib_path, CrFree, CrInit};
    use super::*;
    use crate as c;

    /// The sink path writes the correct per-record checksums into the host arena, byte-exact
    /// versus in-process, for both sink shapes across the W-sweep. The sink family's
    /// cross-validation gate.
    #[test]
    fn sink_arena_matches_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let batched: CrEntrySink =
            unsafe { rt.resolve(b"cr_execute_sink_batched\0") }.expect("batched sink");
        let per_record: CrEntrySink =
            unsafe { rt.resolve(b"cr_execute_sink_per_record\0") }.expect("per-record sink");

        for profile in ["real", "tight", "leaf"] {
            let bytes = program_bytes(profile);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());

            let seed = 0x5115 ^ profile.len() as u64;
            let mut seeds = vec![0u64; N_TOTAL];
            fill_seeds(&mut seeds, seed);

            // in-process reference: the per-record checksums the sink must write.
            let mut scratch = vec![0u64; pd.nodes.len()];
            let want: Vec<u64> = seeds
                .iter()
                .map(|&sd| {
                    c::interpret_predecoded(&pd, sd, &mut scratch);
                    c::checksum(&scratch)
                })
                .collect();

            for w in [1usize, 8, 64, 256] {
                for &entry in &[batched, per_record] {
                    let mut arena = SinkArena { buf: vec![0u64; N_TOTAL], off: 0 };
                    let sink = CrSink {
                        reserve: host_reserve,
                        commit: host_commit,
                        userdata: &mut arena as *mut SinkArena as *mut c_void,
                    };
                    let mut off = 0usize;
                    while off < N_TOTAL {
                        let batch = w.min(N_TOTAL - off);
                        unsafe { entry(handle, seeds.as_ptr().add(off), batch, &sink) };
                        off += w;
                    }
                    assert_eq!(arena.off, N_TOTAL, "sink commits the whole column, {profile} W={w}");
                    assert_eq!(arena.buf, want, "sink arena must match in-process, {profile} W={w}");
                }
            }
            unsafe { free(handle) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_sink");
        assert_eq!(d.baseline, "null_sink");
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(
            tags,
            ["batched_sink", "batched_sink_decode", "per_record_sink", "null_sink"]
        );
    }
}

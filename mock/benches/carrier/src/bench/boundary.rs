//! Boundary bench family: the runtime C ABI batched-execute crossing.
//!
//! This is bench 1 of the runtime-ABI arc (crossing amortisation, scalar payload
//! pinned). It measures the cost of the host-to-runtime C ABI call itself: feeding
//! a column of records across the boundary a batch of `W` at a time, and reading a
//! keep-alive back. The one non-deferrable ABI decision is whether the boundary
//! should expose a batched/column entry so the runtime can amortise the crossing
//! (and later vectorise) over many records, versus a scalar per-record entry that
//! forecloses it. This family produces the evidence.
//!
//! ## Why a separate cdylib, and why `warm` needs no new regime
//!
//! The crossing must be a genuine cross-object call the optimiser cannot inline. If
//! the runtime lived in this crate, a variant's fat-LTO link would fold "W separate
//! crossings" and "one crossing over W records" into indistinguishable code and the
//! axis under test would vanish. So the entry points live in the sibling
//! `carrier-runtime` cdylib (built once, its own fat-LTO), and each cell `dlopen`s
//! it and calls a resolved function pointer per batch. The `boundary::Runtime` helper
//! in `mockspace-bench-matrix` keeps that library loaded for the pointers' lifetime.
//!
//! No new scaffold regime is needed. `warm`'s calibrated loop already repeats the
//! cell, so a boundary bench is an ordinary `warm` cell whose state holds the opened
//! runtime plus the resolved entry pointer, and whose body issues the cross-object
//! calls. The S-vs-I split, the reps-invariant digest, and the anti-hoist fold all
//! apply unchanged.
//!
//! ## The measurement shape: fixed column, swept batch width
//!
//! Total record count [`N_TOTAL`] is held fixed; the swept size axis is the batch
//! width `W` (records per crossing). Each pass processes all [`N_TOTAL`] records in
//! `k = N_TOTAL / W` crossings of `W` each. Payload work per pass is therefore
//! constant (every pass interprets [`N_TOTAL`] records); only the crossing count `k`
//! changes with `W`. Fitting `total(k) = S + k * C_cross` over the W-sweep reads the
//! amortisation curve from crossing-dominated (W=1, k=N) to payload-dominated
//! (W=N, k=1), and isolates the per-crossing cost `C_cross` in the slope. Because
//! payload work is constant across the sweep, calibration reps stay comparable across
//! W, which sidesteps the reps-starvation confound a fixed-batch-per-pass shape hits.
//!
//! The program (the residual under test) is fixed at [`PROG_NODES`] per profile and
//! crosses as opaque wire bytes, so no cell specialises the interpreter over it.
//!
//! ## The floor ladder (rungs a, b, c)
//!
//! Four cells, all scalar payload (the FFI isolator, so a later SoA bench's
//! vectorisation term separates by construction):
//!
//! - `inproc_direct` (baseline, rung a): the payload in-process, no crossing, direct
//!   inlinable call. The true zero the crossing cost is measured above.
//! - `inproc_fnptr` (rung b): the same payload in-process through a black-boxed `fn`
//!   pointer, isolating indirect-call cost with no object boundary.
//! - `null_entry` (floor, rung c): cross into `cr_null_entry`, empty payload. Pure
//!   crossing plus marshalling; its k-slope is `C_cross` with no payload masking it.
//! - `ffi_batched_scalar`: the real batched crossing into `cr_execute_scalar_runtime_w`.
//!   Its excess over `inproc_direct` is the crossing cost paid to reach a real runtime.
//!
//! Rungs d (native ceiling) and e (null-dispatch) live in their own families
//! (`native_ceiling`, `dispatch`) and are crossed in the composition matrix.
//!
//! The C_cross fit is an analysis-time step over the raw `(W, time)` points this
//! family produces (`k = N_TOTAL / W` is the fit's x-axis, not `W` directly). The
//! cells here produce honest per-(profile, W) timings; the fit and the ratio headline
//! are computed downstream.

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use crate as c;

/// Records per pass, held fixed while the batch width `W` sweeps. Every value in the
/// `sizes` sweep divides it, so `k = N_TOTAL / W` is exact and no batch is partial.
/// This is the column height a single residual is evaluated over; `W` batches it.
const N_TOTAL: usize = 256;

/// Program (residual) node count, fixed per profile. The residual under test does not
/// change across the W-sweep; only how many records cross per call does.
const PROG_NODES: usize = 256;

/// The environment variable a run sets to the built `carrier-runtime` shared object,
/// so the path is not baked at compile time. A run stages the `.dylib` and sets this;
/// the cross-validation test opens the artifact directly and does not read it.
const RT_ENV: &str = "VEHJE_CARRIER_RUNTIME";

// ── the resolved C ABI signatures of the carrier-runtime exports ──

/// `cr_init(bytes, len) -> *mut Handle`. The handle is opaque to the host.
type CrInit = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
/// `cr_free(handle)`.
type CrFree = unsafe extern "C" fn(*mut c_void);
/// A batched entry: `(handle, seeds, w) -> keep_alive`. Both `cr_execute_scalar_runtime_w`
/// and `cr_null_entry` have this shape.
type CrEntryW = unsafe extern "C" fn(*mut c_void, *const u64, usize) -> u64;

// ── shared helpers ──

/// Fill `buf` deterministically from the per-iteration `seed`. Used identically by the
/// cells and the cross-validation test, so a host crossing and its in-process replica
/// marshal the same column. The `seed` dependence is what keeps the digest reps-variant
/// (anti-hoist); the fill cost is constant across `W`, so it sits in the fit intercept,
/// not the crossing slope.
pub fn fill_seeds(buf: &mut [u64], seed: u64) {
    for (i, x) in buf.iter_mut().enumerate() {
        *x = seed ^ (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
}

/// Build the wire bytes for `profile`'s residual at [`PROG_NODES`]. The bytes cross
/// into the runtime opaque, matching how a real residual reaches the ABI.
fn program_bytes(profile: &str) -> Vec<u8> {
    let mut gp = c::GenParams::profile(profile).expect("boundary sweep names a real profile");
    gp.node_count = PROG_NODES;
    c::ir::encode(&c::generate(&gp), &c::ir::REC24)
}

/// The in-process scalar payload: interpret each record once, folding a per-record
/// keep-alive that matches `carrier-runtime`'s `scalar_batch` fold recipe (so a host
/// crossing and this replica agree byte-exact). Called directly by `inproc_direct`
/// (rung a, inlinable) and through a black-boxed pointer by `inproc_fnptr` (rung b).
fn inproc_scalar_batch(pd: &c::Predecoded, seeds: &[u64], scratch: &mut [u64]) -> u64 {
    let mut acc = 0u64;
    for &sd in seeds {
        c::interpret_predecoded(pd, sd, scratch);
        acc = acc.rotate_left(7) ^ c::checksum(scratch);
    }
    acc
}

/// Open the runtime shared object named by [`RT_ENV`]. Panics with a clear message if
/// the variable is unset or the library does not open: a run must stage the artifact
/// and set the variable before the timed pass. Called once per cell setup (the S term),
/// never in the timed cell.
fn open_runtime() -> Runtime {
    let path = std::env::var(RT_ENV).unwrap_or_else(|_| {
        panic!(
            "boundary bench requires {RT_ENV} set to the built carrier-runtime shared object; \
             build `mock/benches/carrier-runtime` --release and export its .dylib/.so path"
        )
    });
    // The path names a trusted, freshly built sibling runtime cdylib.
    unsafe { Runtime::open(&path) }.unwrap_or_else(|e| panic!("opening {RT_ENV}={path}: {e}"))
}

// ── cell state ──

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

/// Cross state: the opened runtime (kept alive for the resolved pointers), the runtime
/// handle, the resolved batched entry and free pointers, the reused seed column, and
/// the batch width. Shared by `ffi_batched_scalar` and `null_entry` (which resolve
/// different entry symbols of the same [`CrEntryW`] shape).
pub struct StCross {
    // Kept alive so `entry`/`free` stay valid; dropped last (after `Drop` frees the
    // handle), because struct fields drop in declaration order after `Drop::drop` runs.
    pub rt:     Runtime,
    pub handle: *mut c_void,
    pub entry:  CrEntryW,
    pub free:   CrFree,
    pub seeds:  Vec<u64>,
    pub w:      usize,
}

impl Drop for StCross {
    fn drop(&mut self) {
        // Free the runtime handle while `rt` is still loaded (fields drop after this
        // returns), then null it so a double drop is a no-op.
        if !self.handle.is_null() {
            unsafe { (self.free)(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

/// Open the runtime, resolve `cr_init`/`cr_free` and the named batched `entry`, build
/// the handle from `profile`'s residual, and size the seed column. `w` is the batch
/// width (the swept size). One constructor for both cross cells.
fn open_and_init(profile: &str, w: usize, entry_name: &[u8]) -> StCross {
    let rt = open_runtime();
    // Each symbol has the signature named by its resolved type; the cross-validation
    // test proves the crossing computes what the in-process payload computes.
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init resolves");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free resolves");
    let entry: CrEntryW = unsafe { rt.resolve(entry_name) }.expect("batched entry resolves");
    let bytes = program_bytes(profile);
    let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
    assert!(!handle.is_null(), "cr_init must build a handle from a valid residual");
    StCross { rt, handle, entry, free, seeds: vec![0u64; N_TOTAL], w }
}

/// Cross the batched `entry` over the whole [`N_TOTAL`] column in `k = N_TOTAL / w`
/// crossings of `w` records each, folding one keep-alive per crossing. Shared by the
/// two cross cells; the `seed` fill happened in the caller so the fold is reps-variant.
#[inline(always)]
fn cross_column(entry: CrEntryW, handle: *mut c_void, seeds: &[u64], w: usize) -> u64 {
    let mut acc = 0u64;
    let mut off = 0usize;
    while off < N_TOTAL {
        let batch = w.min(N_TOTAL - off);
        // The runtime reads `batch` records at `seeds + off` and returns its own fold.
        acc = acc.rotate_left(7) ^ unsafe { entry(handle, seeds.as_ptr().add(off), batch) };
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

    // Shared setup: the in-process payload floor state, used by `inproc_direct`. `n`
    // is the swept size, reinterpreted here as the batch width `W` (the program size
    // is fixed at PROG_NODES, not swept).
    setup |profile: &str, n: usize| -> StInproc {
        let bytes = program_bytes(profile);
        let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).expect("residual parses");
        let pd = c::predecode::predecode(&d);
        let scratch = vec![0u64; pd.nodes.len()];
        StInproc { pd, scratch, seeds: vec![0u64; N_TOTAL], w: n }
    }

    // Rung a: the payload in-process, no crossing, direct inlinable call. The baseline
    // the crossing cost is measured above.
    cell inproc_direct
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            let mut acc = 0u64;
            let mut off = 0usize;
            while off < N_TOTAL {
                let batch = s.w.min(N_TOTAL - off);
                acc = acc.rotate_left(7)
                    ^ inproc_scalar_batch(&s.pd, &s.seeds[off..off + batch], &mut s.scratch);
                off += s.w;
            }
            acc
        }

    // Rung b: the same payload through a black-boxed `fn` pointer, so the call is a
    // genuine indirect dispatch with no object boundary. Isolates indirect-call cost.
    cell inproc_fnptr
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StFnptr {
            let bytes = program_bytes(profile);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).expect("residual parses");
            let pd = c::predecode::predecode(&d);
            let scratch = vec![0u64; pd.nodes.len()];
            StFnptr { pd, scratch, seeds: vec![0u64; N_TOTAL], w: n, f: inproc_scalar_batch }
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            // black_box the pointer so the optimiser cannot prove which fn it is and
            // must issue an indirect call (rung b, disasm-confirmed by bench 0).
            let f = core::hint::black_box(s.f);
            let mut acc = 0u64;
            let mut off = 0usize;
            while off < N_TOTAL {
                let batch = s.w.min(N_TOTAL - off);
                acc = acc.rotate_left(7) ^ f(&s.pd, &s.seeds[off..off + batch], &mut s.scratch);
                off += s.w;
            }
            acc
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

    // Rung c: cross into the empty-payload entry. Pure crossing plus marshalling; its
    // k-slope is C_cross with no payload masking it.
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
    use super::*;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn dylib_name() -> &'static str {
        #[cfg(target_os = "macos")]
        { "libcarrier_runtime.dylib" }
        #[cfg(all(unix, not(target_os = "macos")))]
        { "libcarrier_runtime.so" }
        #[cfg(windows)]
        { "carrier_runtime.dll" }
    }

    /// Build the sibling `carrier-runtime` cdylib and return its artifact path. It is a
    /// standalone workspace, so building it does not contend with the carrier's own build.
    fn build_runtime_dylib() -> PathBuf {
        let rt_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../carrier-runtime");
        let status = Command::new(env!("CARGO"))
            .args(["build", "--release"])
            .current_dir(&rt_dir)
            .status()
            .expect("cargo build of carrier-runtime spawns");
        assert!(status.success(), "carrier-runtime must build");
        rt_dir.join("target/release").join(dylib_name())
    }

    /// The crossing path computes byte-exactly what the in-process payload computes:
    /// the batched `ffi_batched_scalar` fold equals a faithful in-process replica over
    /// the same column and batch structure. This is the boundary family's cross-validation
    /// gate (the floors are exempt; only the real crossing needs it).
    #[test]
    fn ffi_crossing_matches_in_process_byte_exact() {
        let dylib = build_runtime_dylib();
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

                // host crossing path (exactly what the cell issues).
                let host = cross_column(entry, handle, &seeds, w);

                // in-process replica: same batch structure, same fold recipe.
                let mut scratch = vec![0u64; pd.nodes.len()];
                let mut acc = 0u64;
                let mut off = 0usize;
                while off < N_TOTAL {
                    let batch = w.min(N_TOTAL - off);
                    acc = acc.rotate_left(7)
                        ^ inproc_scalar_batch(&pd, &seeds[off..off + batch], &mut scratch);
                    off += w;
                }
                assert_eq!(
                    host, acc,
                    "crossing must equal in-process for profile {profile}, W={w}"
                );
            }
            unsafe { free(handle) };
        }
    }

    /// The generated matrix declaration is the shape the generator consumes: one
    /// family, the four floor-ladder cells, the full W-sweep, and the named baseline
    /// and floor. Guards against a typo in the macro arguments that would only surface
    /// at generation time.
    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1, "one boundary family");
        let d = &decls[0];
        assert_eq!(d.name, "abi_cross_scalar");
        assert_eq!(d.sweep.values.len(), 6, "six program profiles");
        assert_eq!(d.sizes, vec![1, 2, 4, 8, 16, 32, 64, 128, 256], "the W-sweep");
        assert_eq!(d.baseline, "inproc_direct");
        assert_eq!(d.floor.as_deref(), Some("null_entry"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["inproc_direct", "inproc_fnptr", "ffi_batched_scalar", "null_entry"]);
        // every size divides N_TOTAL, so no batch is partial.
        for &sz in &d.sizes {
            assert_eq!(N_TOTAL % (sz as usize), 0, "W={sz} must divide N_TOTAL={N_TOTAL}");
        }
    }
}

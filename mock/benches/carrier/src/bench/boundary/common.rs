//! Shared machinery for the runtime C ABI boundary bench families.
//!
//! The three boundary families (crossing amortisation, SoA-win, entry-form) all
//! dlopen the sibling `carrier-runtime` cdylib and cross into it a batch of records
//! per call. The constants, the resolved-signature types, the runtime-handle state,
//! the seed marshalling, and the column-crossing loop live here once so each family
//! module holds only its own `bench_matrix!` invocation.
//!
//! The measurement shape (fixed column [`N_TOTAL`], swept batch width `W`, program
//! fixed per profile at [`PROG_NODES`]) and the two-object rationale (a genuine
//! cross-object `blr` the optimiser cannot inline) are documented in the family
//! modules that use them.

use core::ffi::c_void;

use mockspace_bench_matrix::boundary::Runtime;

use crate as c;

/// Records per pass, held fixed while the batch width `W` sweeps. Every value in a
/// family's `sizes` sweep divides it, so `k = N_TOTAL / W` is exact and no batch is
/// partial. This is the column height a single residual is evaluated over.
pub const N_TOTAL: usize = 256;

/// Program (residual) node count, fixed per profile. The residual under test does not
/// change across the W-sweep; only how many records cross per call does.
pub const PROG_NODES: usize = 256;

/// The environment variable a run sets to the built `carrier-runtime` shared object,
/// so the path is not baked at compile time. A run stages the artifact and sets this;
/// the cross-validation tests open the built artifact directly and do not read it.
pub const RT_ENV: &str = "VEHJE_CARRIER_RUNTIME";

/// The environment variable a run sets to the built `carrier-zig` shared object (bench 8,
/// the cross-language entry-form floor). Same role as [`RT_ENV`] for the Zig object.
pub const ZIG_ENV: &str = "VEHJE_CARRIER_ZIG";

/// `cr_init(bytes, len) -> *mut Handle`. The handle is opaque to the host.
pub type CrInit = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
/// `cr_free(handle)`.
pub type CrFree = unsafe extern "C" fn(*mut c_void);
/// A batched entry taking a runtime width: `(handle, seeds, w) -> keep_alive`. The
/// runtime-W, dispatch-table, and null entries all share this shape.
pub type CrEntryW = unsafe extern "C" fn(*mut c_void, *const u64, usize) -> u64;
/// A per-W-monomorphised entry: `(handle, seeds) -> keep_alive`, the width baked into
/// the symbol identity so the caller resolves once and passes no width.
pub type CrEntryMono = unsafe extern "C" fn(*mut c_void, *const u64) -> u64;
/// The per-record scalar anchor: `(handle, seed) -> keep_alive`, one record per call.
pub type CrExec1 = unsafe extern "C" fn(*mut c_void, u64) -> u64;
/// A sink-driven entry: `(handle, seeds, w, sink)`, no return; results flow out through
/// the sink. `cr_execute_sink_batched` and `cr_execute_sink_per_record` share this shape.
pub type CrEntrySink = unsafe extern "C" fn(*mut c_void, *const u64, usize, *const CrSink);

/// The value-arena output sink, layout-identical to `carrier-runtime`'s `CrSink`: the
/// settled reserve/commit two-function-pointer return mechanism. The runtime calls
/// `reserve(userdata, hint)` for `hint` writable `u64` slots and `commit(userdata, n)` to
/// publish `n`. Both pointers point into the host object, so each call is a reverse
/// crossing (the sink's cost). Passed by pointer, called indirect, never inlined.
#[repr(C)]
pub struct CrSink {
    /// Reserve `hint` `u64` output slots, returning a writable pointer to them.
    pub reserve:  unsafe extern "C" fn(*mut c_void, usize) -> *mut u64,
    /// Publish `n` written slots (advancing the host arena).
    pub commit:   unsafe extern "C" fn(*mut c_void, usize),
    /// Opaque host state (the output arena) the pointers operate on.
    pub userdata: *mut c_void,
}

/// Fill `buf` deterministically from the per-iteration `seed`. Used identically by the
/// cells and the cross-validation tests, so a host crossing and its in-process replica
/// marshal the same column. The `seed` dependence keeps the digest reps-variant
/// (anti-hoist); the fill cost is constant across `W`, so it sits in the fit intercept,
/// not the crossing slope.
pub fn fill_seeds(buf: &mut [u64], seed: u64) {
    for (i, x) in buf.iter_mut().enumerate() {
        *x = seed ^ (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
}

/// Build the wire bytes for `profile`'s residual at [`PROG_NODES`]. The bytes cross
/// into the runtime opaque, matching how a real residual reaches the ABI.
pub fn program_bytes(profile: &str) -> Vec<u8> {
    let mut gp = c::GenParams::profile(profile).expect("boundary sweep names a real profile");
    gp.node_count = PROG_NODES;
    c::ir::encode(&c::generate(&gp), &c::ir::REC24)
}

/// Open the runtime shared object named by [`RT_ENV`]. Panics with a clear message if
/// the variable is unset or the library does not open: a run must stage the artifact
/// and set the variable before the timed pass. Called once per cell setup (the S term).
pub fn open_runtime() -> Runtime {
    open_runtime_at(RT_ENV)
}

/// Open the runtime shared object named by the environment variable `env_var`. The Rust
/// (`RT_ENV`) and Zig (`ZIG_ENV`) families differ only in which object they open; the
/// resolved-symbol surface is otherwise identical.
pub fn open_runtime_at(env_var: &str) -> Runtime {
    let path = std::env::var(env_var).unwrap_or_else(|_| {
        panic!(
            "boundary bench requires {env_var} set to the built runtime shared object; \
             build the sibling runtime and export its .dylib/.so path"
        )
    });
    // The path names a trusted, freshly built sibling runtime object.
    unsafe { Runtime::open(&path) }.unwrap_or_else(|e| panic!("opening {env_var}={path}: {e}"))
}

/// Cross state for a width-taking entry: the opened runtime (kept alive for the
/// resolved pointers), the runtime handle, the resolved entry and free pointers, the
/// reused seed column, and the batch width `W`. Shared by every family cell whose entry
/// is a [`CrEntryW`] (runtime-W, dispatch-table, null, and the SoA runtime-W entry).
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

/// Open the runtime, resolve `cr_init`/`cr_free` and the named width-taking `entry`,
/// build the handle from `profile`'s residual, and size the seed column. `w` is the
/// batch width (the swept size).
pub fn open_and_init(profile: &str, w: usize, entry_name: &[u8]) -> StCross {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init resolves");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free resolves");
    let entry: CrEntryW = unsafe { rt.resolve(entry_name) }.expect("batched entry resolves");
    let handle = init_handle(&rt, init, profile);
    StCross { rt, handle, entry, free, seeds: vec![0u64; N_TOTAL], w }
}

/// Build a runtime handle from `profile`'s residual, asserting it is non-null. Factored
/// so the entry-form family (which resolves per-W-monomorphised or per-record entries
/// of other shapes) reuses the init path.
pub fn init_handle(_rt: &Runtime, init: CrInit, profile: &str) -> *mut c_void {
    let bytes = program_bytes(profile);
    let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
    assert!(!handle.is_null(), "cr_init must build a handle from a valid residual");
    handle
}

/// Cross a width-taking `entry` over the whole [`N_TOTAL`] column in `k = N_TOTAL / w`
/// crossings of `w` records each, folding one keep-alive per crossing. The `seed` fill
/// happens in the caller so the fold is reps-variant.
#[inline(always)]
pub fn cross_column(entry: CrEntryW, handle: *mut c_void, seeds: &[u64], w: usize) -> u64 {
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

/// The in-process scalar payload: interpret each record once, folding a per-record
/// keep-alive that matches `carrier-runtime`'s `scalar_batch` fold recipe (so a host
/// crossing and an in-process replica agree byte-exact). Used by the crossing family's
/// floors and every scalar cross-validation replica.
pub fn inproc_scalar_batch(pd: &c::Predecoded, seeds: &[u64], scratch: &mut [u64]) -> u64 {
    let mut acc = 0u64;
    for &sd in seeds {
        c::interpret_predecoded(pd, sd, scratch);
        acc = acc.rotate_left(7) ^ c::checksum(scratch);
    }
    acc
}

/// The name of the built `carrier-runtime` shared object on this platform. Used by the
/// cross-validation tests to locate the artifact after building it.
pub fn dylib_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "libcarrier_runtime.dylib"
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        "libcarrier_runtime.so"
    }
    #[cfg(windows)]
    {
        "carrier_runtime.dll"
    }
}

/// Resolve the `carrier-runtime` shared object path for the cross-validation tests.
///
/// Prefers an already-staged path from [`RT_ENV`] (what a run sets), so no nested cargo
/// runs. Otherwise builds the sibling cdylib at most once per test process, behind a
/// [`OnceLock`]: the cross-validation tests run in parallel, and a separate nested
/// `cargo build` per test deadlocks on cargo's package-cache lock, so exactly one build
/// happens and every test reuses its path. Test-only; a real run stages the artifact.
///
/// [`OnceLock`]: std::sync::OnceLock
#[cfg(test)]
pub fn runtime_dylib_path() -> std::path::PathBuf {
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::OnceLock;

    if let Ok(p) = std::env::var(RT_ENV) {
        return PathBuf::from(p);
    }
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    BUILT
        .get_or_init(|| {
            let rt_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../carrier-runtime");
            let status = Command::new(env!("CARGO"))
                .args(["build", "--release"])
                .current_dir(&rt_dir)
                .status()
                .expect("cargo build of carrier-runtime spawns");
            assert!(status.success(), "carrier-runtime must build");
            rt_dir.join("target/release").join(dylib_name())
        })
        .clone()
}

/// Resolve the `carrier-zig` shared object path for the cross-language cross-validation.
/// Prefers [`ZIG_ENV`]; otherwise runs `carrier-zig/build.sh` at most once per process
/// (behind a `OnceLock`, same parallel-test rationale as [`runtime_dylib_path`]).
#[cfg(test)]
pub fn zig_dylib_path() -> std::path::PathBuf {
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::OnceLock;

    if let Ok(p) = std::env::var(ZIG_ENV) {
        return PathBuf::from(p);
    }
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    BUILT
        .get_or_init(|| {
            let zig_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../carrier-zig");
            let status = Command::new("bash")
                .arg("build.sh")
                .current_dir(&zig_dir)
                .status()
                .expect("carrier-zig build.sh spawns");
            assert!(status.success(), "carrier-zig must build");
            zig_dir.join("libcarrier_zig.dylib")
        })
        .clone()
}

//! Boundary-cold regime: the crossing under a mispredicted indirect branch.
//!
//! The `warm` families cross one resolved pointer every iteration, so the indirect `blr` is
//! perfectly predicted and C_cross is the best-case, lowest crossing cost (the panel's
//! "biggest hole"). This family crosses a rotating set of sixteen distinct call targets in a
//! scattered order, so the CPU's indirect-branch target predictor mispredicts, measuring the
//! crossing the real varying-call-target case pays. The sixteen targets do the identical work
//! at distinct addresses (ICF defeated by a discarded `black_box`), so a cold cell folds the
//! identical result as its warm counterpart and cross-validates byte-exact.
//!
//! Cells, over the same fixed-column / swept-W shape:
//!
//! - `warm_null` (baseline / floor): the crossing, one target, empty payload (predicted).
//! - `cold_null`: the crossing, sixteen empty-payload targets cycled (mispredicted). The delta
//!   over `warm_null` is the crossing-misprediction penalty in isolation, no payload masking.
//! - `warm_scalar` / `cold_scalar`: the same pair with the scalar payload, showing the penalty
//!   under a real payload (where the crossing, predicted or not, is a small fraction).

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{
    cross_column, fill_seeds, init_handle, open_and_init, open_runtime, CrEntryW, CrFree, CrInit,
    StCross, N_TOTAL,
};

/// The number of distinct cold call targets. Enough to thrash a per-site indirect-branch
/// target predictor when cycled in a scattered order.
const COLD_TARGETS: usize = 16;

/// State for a cold cell: the runtime, handle, the sixteen resolved targets, the free
/// pointer, the reused seed column, and the batch width.
pub struct StColdCross {
    pub rt:      Runtime,
    pub handle:  *mut c_void,
    pub targets: [CrEntryW; COLD_TARGETS],
    pub free:    CrFree,
    pub seeds:   Vec<u64>,
    pub w:       usize,
}

impl Drop for StColdCross {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.free)(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

/// Resolve `cr_<prefix>_{0..15}` into the target array and build a handle from `profile`.
fn open_cold(profile: &str, w: usize, prefix: &str) -> StColdCross {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let targets: [CrEntryW; COLD_TARGETS] = core::array::from_fn(|i| {
        let name = format!("{prefix}{i}\0");
        unsafe { rt.resolve::<CrEntryW>(name.as_bytes()) }.expect("cold target resolves")
    });
    let handle = init_handle(&rt, init, profile);
    StColdCross { rt, handle, targets, free, seeds: vec![0u64; N_TOTAL], w }
}

/// Cross the column in `k = N/w` crossings, each into a DATA-dependent target so the indirect
/// branch mispredicts. The index is derived from the batch's own seed data (which the CPU has
/// not seen at branch time and which changes every calibrated iteration via `fill_seeds`), so
/// a history-based predictor (BTB/ITTAGE) cannot learn the sequence the way it learns a fixed
/// counter-derived rotation. Panel fix (agner): a loop-counter index is a deterministic
/// sequence a predictor learns after warmup, measuring near-warm cost; a data-dependent index
/// is what actually forces the mispredict.
#[inline(always)]
fn cold_column(targets: &[CrEntryW; COLD_TARGETS], handle: *mut c_void, seeds: &[u64], w: usize) -> u64 {
    let mut acc = 0u64;
    let mut off = 0usize;
    while off < N_TOTAL {
        let batch = w.min(N_TOTAL - off);
        // index from this batch's first seed: unknown to the predictor at branch time, and
        // re-randomised every iteration by the anti-hoist seed, so the target sequence is not
        // a learnable fixed rotation.
        let idx = (seeds[off].wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 60) as usize & (COLD_TARGETS - 1);
        acc = acc.rotate_left(7) ^ unsafe { targets[idx](handle, seeds.as_ptr().add(off), batch) };
        off += w;
    }
    acc
}

bench_matrix! {
    name: "abi_cross_cold",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0c01,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "warm_null",
    floor: "warm_null",
    regime: warm,

    // one empty-payload target, predicted crossing (baseline + floor). Uses cold_null_0 (a
    // single cold address) rather than cr_null_entry, so it carries the identical per-call
    // black_box the cold cells' targets carry: the only difference between warm_null and
    // cold_null is then one predicted target vs sixteen mispredicted ones, the misprediction
    // penalty in isolation (panel fix: the warm baseline must match the cold body).
    setup |profile: &str, n: usize| -> StCross {
        open_and_init(profile, n, b"cr_cold_null_0\0")
    }

    cell warm_null
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // sixteen empty-payload targets cycled: the crossing-misprediction penalty, isolated.
    cell cold_null
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StColdCross {
            open_cold(profile, n, "cr_cold_null_")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cold_column(&s.targets, s.handle, &s.seeds, s.w)
        }

    // one scalar-payload target, predicted crossing (cold_target_0, same body as the cold
    // cells so the delta is pure misprediction).
    cell warm_scalar
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_and_init(profile, n, b"cr_cold_target_0\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    // sixteen scalar-payload targets cycled: the penalty under a real payload.
    cell cold_scalar
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StColdCross {
            open_cold(profile, n, "cr_cold_target_")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cold_column(&s.targets, s.handle, &s.seeds, s.w)
        }
}

#[cfg(test)]
mod tests {
    use super::super::common::{program_bytes, runtime_dylib_path, CrFree, CrInit};
    use super::*;
    use crate as c;

    /// A cold (cycled sixteen-target) crossing folds byte-exactly to the warm (single-target)
    /// crossing: the targets do identical work, so only the branch-prediction cost differs, not
    /// the result. The cold family's cross-validation gate.
    #[test]
    fn cold_and_warm_crossings_fold_identically() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let warm: CrEntryW =
            unsafe { rt.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("warm scalar");
        let cold: [CrEntryW; COLD_TARGETS] = core::array::from_fn(|i| {
            let name = format!("cr_cold_target_{i}\0");
            unsafe { rt.resolve::<CrEntryW>(name.as_bytes()) }.expect("cold target")
        });

        for profile in ["real", "tight"] {
            let bytes = program_bytes(profile);
            let _ = c::ir::REC24;
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());
            for w in [1usize, 8, 256] {
                let seed = 0xc01d ^ (w as u64);
                let mut seeds = vec![0u64; N_TOTAL];
                fill_seeds(&mut seeds, seed);
                let a = cross_column(warm, handle, &seeds, w);
                let b = cold_column(&cold, handle, &seeds, w);
                assert_eq!(a, b, "cold and warm must fold identically, {profile} W={w}");
            }
            unsafe { free(handle) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_cross_cold");
        assert_eq!(d.baseline, "warm_null");
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["warm_null", "cold_null", "warm_scalar", "cold_scalar"]);
    }
}

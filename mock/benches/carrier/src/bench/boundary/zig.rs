//! Boundary bench 8: cross-language entry-form floor (Zig).
//!
//! The entry-form conclusion of bench 4 rests on a Rust cdylib, which answers the
//! language-agnostic ISA-level crossing cost but NOT the callee-side codegen of the real
//! target language. The shipped vehje runtime is Zig, so this family crosses into the
//! sibling `carrier-zig` object's batched entries (`zr_*`, the same symbol surface as the
//! Rust `cr_*` scalar family) and re-runs the entry-form comparison there. It reports the
//! entry-form question on the real target language, with a stub-fidelity caveat until the
//! actual M3 runtime entry points land.
//!
//! Cells mirror bench 4, scalar payload pinned: `zig_runtime_w` (baseline), `zig_anchor`
//! (per-record), `zig_dispatch` (match-W), `zig_per_w_set` (per-W-monomorphised, chunking
//! by Wmax=128 above it), `zig_null` (the pure-crossing floor). The Zig entries fold the
//! identical keep-alive as the Rust ones, so a Zig cell cross-validates byte-exact against
//! Rust (proven in [`tests`]); the Rust/Rust number is the same-toolchain lower bound and
//! this Zig cell is the real comparator (fairness: the Rust cdylib is labelled a lower
//! bound, the Zig cell the true cross-language cost).

use mockspace_bench_matrix::bench_matrix;

use super::common::{
    cross_column, fill_seeds, init_handle, open_runtime_at, program_bytes, CrEntryMono, CrEntryW,
    CrExec1, CrFree, CrInit, StCross, ZIG_ENV, N_TOTAL,
};
use super::entry::{anchor_column, mono_column, StAnchor, StMono};

/// The largest per-W-monomorphised Zig symbol (`zr_execute_scalar_w128`); a requested W
/// above it chunks by [`WMAX`], the same plateau the Rust per-W set has.
const WMAX: usize = 128;

/// Open the Zig object and resolve `zr_init`/`zr_free` and the named width-taking entry,
/// building a handle from `profile`'s residual. Reuses [`StCross`] (the Zig entries have
/// the identical resolved signatures as the Rust ones).
fn open_zig_cross(profile: &str, w: usize, entry_name: &[u8]) -> StCross {
    let rt = open_runtime_at(ZIG_ENV);
    let init: CrInit = unsafe { rt.resolve(b"zr_init\0") }.expect("zr_init resolves");
    let free: CrFree = unsafe { rt.resolve(b"zr_free\0") }.expect("zr_free resolves");
    let entry: CrEntryW = unsafe { rt.resolve(entry_name) }.expect("zig batched entry resolves");
    let handle = init_handle(&rt, init, profile);
    StCross { rt, handle, entry, free, seeds: vec![0u64; N_TOTAL], w }
}

fn open_zig_anchor(profile: &str) -> StAnchor {
    let rt = open_runtime_at(ZIG_ENV);
    let init: CrInit = unsafe { rt.resolve(b"zr_init\0") }.expect("zr_init");
    let free: CrFree = unsafe { rt.resolve(b"zr_free\0") }.expect("zr_free");
    let exec1: CrExec1 = unsafe { rt.resolve(b"zr_execute1\0") }.expect("zr_execute1");
    let handle = init_handle(&rt, init, profile);
    StAnchor { rt, handle, exec1, free, seeds: vec![0u64; N_TOTAL] }
}

fn open_zig_mono(profile: &str, w: usize) -> StMono {
    let rt = open_runtime_at(ZIG_ENV);
    let init: CrInit = unsafe { rt.resolve(b"zr_init\0") }.expect("zr_init");
    let free: CrFree = unsafe { rt.resolve(b"zr_free\0") }.expect("zr_free");
    let eff = w.min(WMAX);
    let name = format!("zr_execute_scalar_w{eff}\0");
    let mono: CrEntryMono =
        unsafe { rt.resolve(name.as_bytes()) }.expect("zig per-W entry resolves");
    let handle = init_handle(&rt, init, profile);
    StMono { rt, handle, mono, free, eff, seeds: vec![0u64; N_TOTAL] }
}

bench_matrix! {
    name: "abi_zig_entry",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0008,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16, 32, 64, 128, 256],
    baseline: "zig_runtime_w",
    floor: "zig_null",
    regime: warm,

    setup |profile: &str, n: usize| -> StCross {
        open_zig_cross(profile, n, b"zr_execute_scalar_runtime_w\0")
    }

    cell zig_runtime_w
        #[feature = "boundary"]
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    cell zig_anchor
        #[feature = "boundary"]
        setup |profile: &str, _n: usize| -> StAnchor {
            open_zig_anchor(profile)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            anchor_column(s.exec1, s.handle, &s.seeds)
        }

    cell zig_dispatch
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_zig_cross(profile, n, b"zr_execute_scalar_dispatch\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }

    cell zig_per_w_set
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMono {
            open_zig_mono(profile, n)
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            mono_column(s.mono, s.handle, &s.seeds, s.eff)
        }

    cell zig_null
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StCross {
            open_zig_cross(profile, n, b"zr_null_entry\0")
        }
        |s, seed| {
            fill_seeds(&mut s.seeds, seed);
            cross_column(s.entry, s.handle, &s.seeds, s.w)
        }
}

#[cfg(test)]
mod tests {
    use super::super::common::{runtime_dylib_path, zig_dylib_path, N_TOTAL as NT};
    use super::*;
    use mockspace_bench_matrix::boundary::Runtime;

    /// The Zig batched entry computes byte-exactly what the Rust runtime entry computes,
    /// across profiles and the W-sweep. The cross-language fidelity gate that lets the Zig
    /// entry-form numbers be compared against the Rust ones and against each other.
    #[test]
    fn zig_crossing_matches_rust_byte_exact() {
        let zig = unsafe { Runtime::open(zig_dylib_path().to_str().unwrap()) }.expect("zig opens");
        let rust =
            unsafe { Runtime::open(runtime_dylib_path().to_str().unwrap()) }.expect("rust opens");

        let z_init: CrInit = unsafe { zig.resolve(b"zr_init\0") }.expect("zr_init");
        let z_free: CrFree = unsafe { zig.resolve(b"zr_free\0") }.expect("zr_free");
        let z_rtw: CrEntryW =
            unsafe { zig.resolve(b"zr_execute_scalar_runtime_w\0") }.expect("zr runtime-w");
        let r_init: CrInit = unsafe { rust.resolve(b"cr_init\0") }.expect("cr_init");
        let r_free: CrFree = unsafe { rust.resolve(b"cr_free\0") }.expect("cr_free");
        let r_rtw: CrEntryW =
            unsafe { rust.resolve(b"cr_execute_scalar_runtime_w\0") }.expect("cr runtime-w");

        for profile in ["real", "madd", "tight", "leaf"] {
            let bytes = program_bytes(profile);
            let zh = unsafe { z_init(bytes.as_ptr(), bytes.len()) };
            let rh = unsafe { r_init(bytes.as_ptr(), bytes.len()) };
            assert!(!zh.is_null() && !rh.is_null());

            for w in [1usize, 2, 8, 64, 256] {
                let seed = 0x2468 ^ (w as u64).wrapping_mul(0xace);
                let mut seeds = vec![0u64; NT];
                fill_seeds(&mut seeds, seed);
                let z = cross_column(z_rtw, zh, &seeds, w);
                let r = cross_column(r_rtw, rh, &seeds, w);
                assert_eq!(z, r, "zig must equal rust for {profile}, W={w}");
            }
            unsafe { z_free(zh) };
            unsafe { r_free(rh) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_zig_entry");
        assert_eq!(d.baseline, "zig_runtime_w");
        assert_eq!(d.floor.as_deref(), Some("zig_null"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["zig_runtime_w", "zig_anchor", "zig_dispatch", "zig_per_w_set", "zig_null"]);
    }
}

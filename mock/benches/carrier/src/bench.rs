//! The carrier semantic-benchmark matrix, authored with the `bench_matrix!` macro
//! from `mockspace-bench-matrix`.
//!
//! Each family is one `pub mod` holding one `bench_matrix!` invocation (so the
//! per-family `setup` / `cell_<tag>` / `matrix_decls` functions do not collide),
//! and [`all_matrix_decls`] concatenates them for the generator binary. The cells
//! are real, type-checked `pub fn`s the variant cdylibs call by path; the
//! measurement discipline (anti-hoist, seed table, S-vs-I timing, digest) lives in
//! the scaffold, not here.
//!
//! One family, `native_ceiling`, is NOT here: it sweeps the harness `input` byte
//! stream as an O(N^2) throughput measurement, which is not the per-execution
//! shape the scaffold (and this macro) model. It stays hand-authored in
//! `src/bin/gen_matrix.rs`.
//!
//! Note on setup: unlike the old string-template matrix, `setup` does NOT cache
//! its build behind a `OnceLock`. The scaffold times `setup` on every call
//! precisely to measure the one-time cost S (the review panel's number-one
//! finding); caching would hide it. The per-iteration I measurement is unaffected,
//! because the calibrated inner loop runs many reps over the one built state.

use mockspace_bench_matrix::MatrixDecl;

/// The variant `Cargo.toml` dependency lines. Path deps to the local mockspace
/// checkout for cross-repo iteration (five levels up from `variants/<name>/`);
/// switch to the git rev before pushing. The variants need the scaffold + the FFI
/// struct + the bench_variant macro, never the harness transport, so bench-matrix
/// is `default-features = false`.
const EXTRA_DEPS: &[&str] = &[
    "mockspace-bench-core = { path = \"../../../../../mockspace/bench-core\", features = [\"std\"] }",
    "mockspace-bench-macro = { path = \"../../../../../mockspace/bench-macro\" }",
    "mockspace-bench-matrix = { path = \"../../../../../mockspace/bench-matrix\", default-features = false }",
];

fn extra_deps() -> Vec<String> {
    EXTRA_DEPS.iter().map(|s| s.to_string()).collect()
}

/// Dispatch shape over the wire `Decoded` form, one bench per profile.
pub mod dispatch {
    use crate as c;
    use mockspace_bench_matrix::bench_matrix;

    /// Shared state: the decoded wire program and a results scratch. `Decoded`
    /// borrows its bytes, so setup leaks them to `'static` (a bounded per-subprocess
    /// leak reclaimed at exit, the standard bench idiom for a borrowing decoded
    /// form; the old string matrix did the same via `OnceLock`'s static storage).
    pub struct St {
        pub d: c::ir::Decoded<'static>,
        pub r: Vec<u64>,
    }

    bench_matrix! {
        name: "carrier_dispatch",
        crate_path: vehje_bench_carrier,
        crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
        extra_deps: [ ],
        seed: 0x5eed_d15b_a7c4_0002,
        sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
        sizes: [64, 256, 1024, 4096, 16384],
        baseline: "switch",
        floor: "nullfloor",
        regime: warm,

        setup |profile: &str, n: usize| -> St {
            let mut gp = c::GenParams::profile(profile).unwrap();
            gp.node_count = n;
            let bytes: &'static [u8] = Vec::leak(c::ir::encode(&c::generate(&gp), &c::ir::REC24));
            let d = c::ir::Decoded::parse(bytes, c::ir::REC24).unwrap();
            let r = vec![0u64; d.node_count];
            St { d, r }
        }

        // every cell folds `checksum(&r)` as its single keep-alive; the scaffold
        // folds one per iteration, so the fidelity fold is symmetric by construction.
        cell switch     |s, seed| { c::interpret(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell fntable    |s, seed| { c::interpret_fntable(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell ifchain    |s, seed| { c::interpret_ifchain(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell ifchainasc |s, seed| { c::interpret_ifchain_ascending(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell ifchainlin |s, seed| { c::interpret_ifchain_linear(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell bittree    |s, seed| { c::interpret_bittree(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell threaded   #[feature = "threaded"]
                        |s, seed| { c::interpret_threaded(&s.d, seed, &mut s.r); c::checksum(&s.r) }
        cell nullfloor  |s, seed| { c::interpret_nulldispatch(&s.d, seed, &mut s.r); c::checksum(&s.r) }
    }
}

/// Every family's matrix declarations, for the generator binary. Each family's
/// `matrix_decls()` returns its per-profile benches; the extra_deps (the variant
/// dependency lines) are filled in here so a family module does not repeat them.
pub fn all_matrix_decls() -> Vec<MatrixDecl> {
    let mut all: Vec<MatrixDecl> = Vec::new();
    all.extend(dispatch::matrix_decls());
    for d in &mut all {
        d.extra_deps = extra_deps();
    }
    all
}

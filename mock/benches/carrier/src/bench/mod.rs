//! The carrier semantic-benchmark matrix, authored with the `bench_matrix!` macro
//! from `mockspace-bench-matrix`.
//!
//! Each family is one submodule holding one `bench_matrix!` invocation (so the
//! per-family `setup` / `cell_<tag>` / `matrix_decls` functions do not collide),
//! and [`all_matrix_decls`] concatenates them for the generator binary. The cells
//! are real, type-checked `pub fn`s the variant cdylibs call by path; the
//! measurement discipline (anti-hoist, seed table, S-vs-I timing, digest) lives in
//! the scaffold, not here.
//!
//! Every family expresses through the macro. `native_ceiling` sweeps the harness
//! `input` byte stream as an O(N^2) throughput measurement, which is not the
//! per-execution shape the warm/cold regimes model; it uses the macro's `stream`
//! regime rather than a hand-authored exception (the upstream gained that regime
//! for exactly this shape).
//!
//! Note on setup: unlike the old string-template matrix, `setup` does NOT cache
//! its build behind a `OnceLock`. The scaffold times `setup` on every call
//! precisely to measure the one-time cost S (the review panel's number-one
//! finding); caching would hide it. The per-iteration I measurement is unaffected,
//! because the calibrated inner loop runs many reps over the one built state.

use mockspace_bench_matrix::MatrixDecl;

pub mod cfg;
pub mod coldcycle;
pub mod dispatch;
pub mod entgrid;
pub mod layout;
pub mod native;
pub mod native_ceiling;
pub mod optimize;
pub mod predecode;
pub mod residual;
pub mod setup_cost;
pub mod valrepr;
pub mod vertical;

/// The mockspace dev rev that landed the semantic-matrix layer. The runner
/// (`mock/benches/Cargo.toml`), the carrier (`carrier/Cargo.toml`), and the variant
/// extra_deps below all pin this one rev, so the whole graph resolves one bench-core:
/// a variant's `FfiBenchCall` + `abi_hash` must match the runner's or it is rejected
/// at dlopen. Bump all four sites together when re-pinning.
const MOCKSPACE_REV: &str = "8c5f87ae1a916c45b40c6ea690dc42402f91d199";

/// The variant `Cargo.toml` dependency lines. The variants need the scaffold + the
/// FFI struct + the bench_variant macro, never the harness transport, so bench-matrix
/// is `default-features = false`.
fn extra_deps() -> Vec<String> {
    let git = "git = \"https://github.com/hiisi-digital/mockspace\"";
    vec![
        format!("mockspace-bench-core = {{ {git}, rev = \"{MOCKSPACE_REV}\", features = [\"std\"] }}"),
        format!("mockspace-bench-macro = {{ {git}, rev = \"{MOCKSPACE_REV}\" }}"),
        format!("mockspace-bench-matrix = {{ {git}, rev = \"{MOCKSPACE_REV}\", default-features = false }}"),
    ]
}

/// Every family's matrix declarations, for the generator binary. Each family's
/// `matrix_decls()` returns its per-sweep-value benches; the extra_deps (the
/// variant dependency lines) are filled in here so a family module does not
/// repeat them.
pub fn all_matrix_decls() -> Vec<MatrixDecl> {
    let mut all: Vec<MatrixDecl> = Vec::new();
    all.extend(dispatch::matrix_decls());
    all.extend(predecode::matrix_decls());
    all.extend(cfg::matrix_decls());
    all.extend(layout::matrix_decls());
    all.extend(valrepr::matrix_decls());
    all.extend(residual::matrix_decls());
    all.extend(optimize::matrix_decls());
    all.extend(native::matrix_decls());
    all.extend(vertical::matrix_decls());
    all.extend(native_ceiling::matrix_decls());
    all.extend(setup_cost::matrix_decls());
    all.extend(coldcycle::matrix_decls());
    all.extend(entgrid::matrix_decls());
    for d in &mut all {
        d.extra_deps = extra_deps();
    }
    all
}

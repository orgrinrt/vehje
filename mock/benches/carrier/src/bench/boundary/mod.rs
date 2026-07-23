//! The runtime C ABI batched-execute boundary bench families.
//!
//! Three families, each isolating one axis of the ABI's batched-execute decision, all
//! crossing into the sibling `carrier-runtime` cdylib through a resolved function pointer
//! so every crossing is a genuine cross-object call the optimiser cannot inline:
//!
//! - [`cross`] (bench 1): call-crossing amortisation over the batch width `W`, scalar
//!   payload pinned. Yields the per-crossing cost `C_cross` and the amortisation knee.
//! - [`soa`] (bench 3): the column-SoA vectorisation win across the boundary, scalar
//!   versus SoA-8 payload at equal crossing cost. The headline family.
//! - [`entry`] (bench 4): the entry-point form (scalar-anchor / runtime-W / dispatch-table
//!   / per-W-set), scalar payload pinned. op's vehicle: the C-boundary-mapping decision.
//! - [`sink`] (bench 5): the output-sink shape (null / batched / per-record / batched
//!   +decode), the real `#[repr(C)]` reserve/commit two-function-pointer struct.
//! - [`lifecycle`] (bench 6): instance lifecycle (held handle vs fresh per column vs fresh
//!   per batch), the session-versus-call-scoped runtime question.
//! - [`residency`] (bench 7): input buffer residency (reused warm column vs fresh cold
//!   region per call), the registered-buffer question.
//!
//! Shared machinery (constants, resolved-signature types, the runtime-handle state, the
//! column-crossing loop, seed marshalling, the cross-validation dylib build) lives in
//! [`common`]. Each family is its own module so its macro-generated `setup` / `cell_*` /
//! `matrix_decls` fns do not collide, matching the carrier's one-family-per-module
//! convention. [`matrix_decls`] concatenates the three for the generator.
//!
//! The whole module is behind the `boundary` feature: it pulls
//! `mockspace-bench-matrix::boundary::Runtime` (and its libloading dep), which the other
//! carrier families do not need.

pub mod common;
pub mod cross;
pub mod entry;
pub mod lifecycle;
pub mod residency;
pub mod sink;
pub mod soa;

use mockspace_bench_matrix::MatrixDecl;

/// The boundary families' declarations, concatenated for the generator. `bench/mod.rs`
/// calls this once; the extra_deps are filled in by the caller like every other family.
pub fn matrix_decls() -> Vec<MatrixDecl> {
    let mut all = cross::matrix_decls();
    all.extend(soa::matrix_decls());
    all.extend(entry::matrix_decls());
    all.extend(sink::matrix_decls());
    all.extend(lifecycle::matrix_decls());
    all.extend(residency::matrix_decls());
    all
}

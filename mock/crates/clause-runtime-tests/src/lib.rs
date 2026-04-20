//! clause-runtime-tests — integration-test harness for the
//! clause runtime C ABI and the driver-side dispatcher.
//!
//! Skeleton round (2026-04-20): ships a single
//! `test_runtime_roundtrip` harness that exercises the
//! `RuntimeLoader` surface and verifies the returned error
//! is `LoaderError::NotImplemented`. Real dlopen
//! round-trip against a Zig stub dylib, struct-shape
//! assertions, enum-discriminant stability checks, and
//! ABI-version monotonicity guards are all BACKLOG.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub use clause_runtime_abi::{AbiSpan, ClauseDiagnostic, ClauseDiagnosticKind, ClauseResult};
pub use clause_runtime_driver::{
    DriverError, LoaderError, RuntimeDriver, RuntimeHandle, RuntimeLoader,
};

/// Attempt a full round-trip against a runtime and return
/// either the emitted bytes or a loader error.
///
/// Skeleton: the `RuntimeLoader::load` call always returns
/// `Err(LoaderError::NotImplemented)`, so this fn always
/// propagates that variant via the `?` operator. Callers
/// verify the specific error via `matches!`.
///
/// Once the follow-up round wires real dlopen integration,
/// this harness will attempt a real load + execute + result
/// check against the Zig stub dylib.
// lint:allow(bare_collection) — the diagnostic return surface across every compiler phase crate matches what clause-resolve already ships; storage-crate collection types target mockspace domain graphs not host-side compiler runtime buffers here
pub fn test_runtime_roundtrip() -> Result<Vec<u8>, LoaderError> {
    let path = std::path::Path::new("nonexistent-runtime");
    let _handle = RuntimeLoader::load(path)?;
    Ok(Vec::new())
}

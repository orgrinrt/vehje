//! Runtime round-trip harness.
//!
//! Houses `test_runtime_roundtrip`: the single skeleton-round
//! harness that exercises the `RuntimeLoader` surface and
//! verifies the returned error is `LoaderError::NotImplemented`.
//! Real dlopen round-trip against a Zig stub dylib, struct-shape
//! assertions, enum-discriminant stability checks, and
//! ABI-version monotonicity guards are all BACKLOG.

use notko::Outcome;

use vehje_runtime_driver::{LoaderError, RuntimeLoader};

/// Attempt a full round-trip against a runtime and return
/// `Outcome::Ok(())` on success or a loader error on failure.
///
/// Skeleton: the `RuntimeLoader::load` call always returns
/// `Outcome::Err(LoaderError::NotImplemented)`, so this fn
/// always propagates that variant. Callers verify the
/// specific error via `matches!`.
///
/// Once the follow-up round wires real dlopen integration,
/// this harness will attempt a real load + execute + result
/// check against the Zig stub dylib. The harness is ABI smoke;
/// the emitted bytes are not returned to the caller.
pub fn test_runtime_roundtrip() -> Outcome<(), LoaderError> {
    let path = std::path::Path::new("nonexistent-runtime");
    let _handle = match RuntimeLoader::load(path) {
        Outcome::Ok(h) => h,
        Outcome::Err(e) => return Outcome::Err(e),
    };
    Outcome::Ok(())
}

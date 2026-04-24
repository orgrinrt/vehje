//! vehje-runtime-tests — integration-test harness for the
//! vehje runtime C ABI and the driver-side dispatcher.
//!
//! Skeleton round (2026-04-20): ships a single
//! `test_runtime_roundtrip` harness that exercises the
//! `RuntimeLoader` surface and verifies the returned error
//! is `LoaderError::NotImplemented`. Real dlopen
//! round-trip against a Zig stub dylib, struct-shape
//! assertions, enum-discriminant stability checks, and
//! ABI-version monotonicity guards are all BACKLOG.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod roundtrip;

pub use vehje_runtime_abi::{AbiSpan, ClauseDiagnostic, ClauseDiagnosticKind, ClauseResult};
pub use vehje_runtime_driver::{
    DriverError, LoaderError, RuntimeDriver, RuntimeHandle, RuntimeLoader,
};
pub use roundtrip::test_runtime_roundtrip;

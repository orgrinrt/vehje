//! vehje-runtime-driver — compiler-side dispatcher.
//!
//! Owns the dylib handle; routes calls through the ABI
//! defined by `clause-runtime-abi`. Driver-side wrappers
//! convert `AbiDiagnostic` to `vehje_ir::Diagnostic` and
//! integrate with `clause-schedule`'s pass DAG (both
//! BACKLOG).
//!
//! Skeleton round (2026-04-20): ships `RuntimeDriver`
//! placeholder, `RuntimeLoader` with a stubbed `load`,
//! `RuntimeHandle` trait, and `LoaderError` / `DriverError`
//! enums. Real `libloading` integration, env-var convention,
//! per-session handle lifecycle, and clause-schedule
//! integration are BACKLOG.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod driver;
pub mod error;
pub mod handle;
pub mod loader;

pub use driver::RuntimeDriver;
pub use error::{DriverError, LoaderError};
pub use handle::RuntimeHandle;
pub use loader::RuntimeLoader;

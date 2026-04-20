//! clause-runtime-abi — the C-ABI shim between the Rust
//! compiler and any runtime backend (Zig today; possibly
//! others tomorrow).
//!
//! This crate holds the pure type surface + `extern "C"` entry
//! points. No runtime logic ships here — the Zig sibling tree
//! at `mock/runtime-zig/` owns the execution body; the
//! `clause-runtime-driver` crate owns the compiler-side
//! dispatch / dlopen integration.
//!
//! Skeleton round (2026-04-20): ships `ClauseResult`,
//! `ClauseRuntime` (opaque), `ClauseDiagnostic` + `AbiSpan` +
//! `ClauseDiagnosticKind`, and three stubbed `extern "C"`
//! entries. Full `AbiToken` / `AbiNode` mirrors, `From`
//! conversions, init / shutdown / invoke / scratch /
//! generative / last_error entry points, and panic-at-FFI
//! guards are all BACKLOG.
//!
//! Stays alloc-free and free of heap-backed collection
//! types across every ABI crossing: `ClauseResult`,
//! `ClauseRuntime`, `ClauseDiagnostic`, and `AbiSpan` carry
//! only bare integers, `[u8; 0]` sentinels, and raw
//! pointers. `std` is nonetheless imported because a
//! `cdylib` on stable Rust needs a panic runtime; the crate
//! never touches `std::collections`, `String`, `Vec`, or
//! `Box`. Treat it as "alloc-free FFI shim" rather than
//! strict `no_std` — the follow-up round that wires
//! panic-at-FFI guards and cross-compilation targets can
//! reopen the question of stripping `std` back out.

#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod diagnostic;
pub mod exports;
pub mod handle;
pub mod result;

pub use diagnostic::{AbiSpan, ClauseDiagnostic, ClauseDiagnosticKind};
pub use handle::ClauseRuntime;
pub use result::ClauseResult;

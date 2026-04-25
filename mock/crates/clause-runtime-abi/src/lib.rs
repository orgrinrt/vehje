//! vehje-runtime-abi, the C-ABI shim between the Rust
//! compiler and any runtime backend (Zig today; possibly
//! others tomorrow).
//!
//! This crate holds the pure type surface + `extern "C"` entry
//! points. No runtime logic ships here, the Zig sibling tree
//! at `mock/runtime-zig/` owns the execution body; the
//! `vehje-runtime-driver` crate owns the compiler-side
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
//! Stays `no_std` + `no_alloc` like every other vehje crate.
//! Every ABI crossing carries only bare integers, `[u8; 0]`
//! sentinels, and raw pointers: `ClauseResult`,
//! `ClauseRuntime`, `ClauseDiagnostic`, and `AbiSpan`. A
//! follow-up round wires panic-at-FFI guards via an explicit
//! abort path rather than `std::panic::catch_unwind` (which
//! would require `std`).
//!
//! The bare-primitive sites at the `extern "C"` boundary
//! (`i32` / `u32` / `u8` / `usize` scalars in `#[repr(C)]`
//! and `#[repr(i32)]` types) follow the two-tier FFI-wire
//! primitive policy declared in `DESIGN.md`: arvo newtype
//! where wire-identical, bare primitive with single-token
//! `lint:allow(...) tracked: #207` where no wire-stable arvo
//! newtype exists yet. A future `arvo::UWire<N>` flips the
//! bare sites when it lands.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

pub mod diagnostic;
pub mod exports;
pub mod handle;
pub mod result;

pub use diagnostic::{AbiSpan, ClauseDiagnostic, ClauseDiagnosticKind};
pub use handle::ClauseRuntime;
pub use result::ClauseResult;

// `no_std` cdylib panic handler lives behind `#[cfg(not(feature = "std"))]`.
// With the default `std` feature enabled the compiler's panic runtime is
// used; without it, we abort. Follow-up round wires a real panic-at-FFI
// guard that sets `ClauseResult::Err`, fills the last-error diagnostic,
// and then aborts if unwinding would still be attempted.
#[cfg(all(not(feature = "std"), not(test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

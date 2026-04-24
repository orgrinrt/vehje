//! `extern "C"` entry points exposed by the `cdylib` build
//! of `vehje-runtime-abi`.
//!
//! These are the symbols the Zig runtime (or any other backend)
//! implements on its side; the driver (`vehje-runtime-driver`)
//! resolves them at load time via `dlsym`. The Rust-side bodies
//! here are fall-backs / reference implementations, and every
//! body in the skeleton round is a stub.
//!
//! `#[unsafe(no_mangle)]` is the 2024-edition attribute form;
//! earlier editions used the plain `#[no_mangle]` form.

use crate::handle::ClauseRuntime;
use crate::result::ClauseResult;

/// Create a new runtime session.
///
/// Skeleton: returns `null_mut()`. The real body allocates a
/// session struct and returns a pointer the caller later
/// hands to `vehje_runtime_free`.
// SAFETY-GATE: no real body may land in this function before
//   1. catch_unwind wraps any Rust code that can panic (UB across
//      FFI otherwise).
//   2. null+nonzero-len input guards are present where applicable.
// Both are tracked in BACKLOG as hard gates before non-stub
// implementation.
#[unsafe(no_mangle)]
pub extern "C" fn vehje_runtime_new() -> *mut ClauseRuntime {
    core::ptr::null_mut()
}

/// Destroy a runtime session.
///
/// Skeleton: no-op. Accepts null safely. The real body
/// deallocates the session allocated by `vehje_runtime_new`.
// SAFETY-GATE: no real body may land in this function before
//   1. catch_unwind wraps any Rust code that can panic (UB across
//      FFI otherwise).
//   2. null+nonzero-len input guards are present where applicable.
// Both are tracked in BACKLOG as hard gates before non-stub
// implementation.
#[unsafe(no_mangle)]
pub extern "C" fn vehje_runtime_free(rt: *mut ClauseRuntime) {
    let _ = rt;
}

/// Execute an input buffer against a runtime session.
///
/// Skeleton: returns `ClauseResult::Err`. The real body hands
/// off to the runtime backend, which walks the IR-encoded
/// input and emits an output buffer (BACKLOG).
// SAFETY-GATE: no real body may land in this function before
//   1. catch_unwind wraps any Rust code that can panic (UB across
//      FFI otherwise).
//   2. null+nonzero-len input guards are present for the
//      `(input, len)` pair — a null `input` with `len > 0` is a
//      caller bug that must surface as `ClauseResult::InvalidInput`,
//      not a segfault.
// Both are tracked in BACKLOG as hard gates before non-stub
// implementation.
#[unsafe(no_mangle)]
pub extern "C" fn vehje_runtime_execute(
    rt: *mut ClauseRuntime,
    input: *const u8,
    len: usize,
) -> ClauseResult {
    let _ = (rt, input, len);
    ClauseResult::Err
}

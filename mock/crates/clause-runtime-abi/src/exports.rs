//! `extern "C"` entry points exposed by the `cdylib` build
//! of `clause-runtime-abi`.
//!
//! These are the symbols the Zig runtime (or any other backend)
//! implements on its side; the driver (`clause-runtime-driver`)
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
/// hands to `clause_runtime_free`.
#[unsafe(no_mangle)]
pub extern "C" fn clause_runtime_new() -> *mut ClauseRuntime {
    core::ptr::null_mut()
}

/// Destroy a runtime session.
///
/// Skeleton: no-op. Accepts null safely. The real body
/// deallocates the session allocated by `clause_runtime_new`.
#[unsafe(no_mangle)]
pub extern "C" fn clause_runtime_free(rt: *mut ClauseRuntime) {
    let _ = rt;
}

/// Execute an input buffer against a runtime session.
///
/// Skeleton: returns `ClauseResult::Err`. The real body hands
/// off to the runtime backend, which walks the IR-encoded
/// input and emits an output buffer (BACKLOG).
#[unsafe(no_mangle)]
pub extern "C" fn clause_runtime_execute(
    rt: *mut ClauseRuntime,
    input: *const u8,
    len: usize,
) -> ClauseResult {
    let _ = (rt, input, len);
    ClauseResult::Err
}

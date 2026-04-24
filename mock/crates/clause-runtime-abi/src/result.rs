//! `ClauseResult` — integer result code returned by every
//! `extern "C"` entry point in the Vehje runtime ABI.
//!
//! The discriminants follow a POSIX-ish convention: `Ok = 0`,
//! errors are negative. The Zig side mirrors the same layout
//! via `pub const ClauseResult = extern struct { code: i32 };`
//! — Zig uses a one-field struct to work around the lack of a
//! direct `extern enum` equivalent. The field width (`i32`) and
//! values match the Rust discriminants exactly.

/// Integer result code carried by FFI entry points.
///
/// `#[repr(i32)]` so the layout matches C's `int` on every
/// supported target.
///
/// Skeleton round: `NullHandle` and `InvalidInput` are declared
/// but never returned. They exist for the deferred real-body
/// round that surfaces those paths.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ClauseResult {
    /// Success. Any accompanying out-buffer is valid.
    Ok = 0,
    /// Generic failure. Use `vehje_runtime_last_error` (BACKLOG)
    /// to fetch the associated diagnostic.
    Err = -1,
    /// Caller passed a null `*mut ClauseRuntime` handle to an
    /// entry that requires a valid handle.
    NullHandle = -2,
    /// Caller passed a malformed input buffer (length 0 with a
    /// non-null pointer, or length non-zero with a null
    /// pointer).
    InvalidInput = -3,
}

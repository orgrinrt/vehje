//! `RuntimeHandle` — trait every loaded runtime backend
//! implements.
//!
//! The handle wraps an opaque runtime session plus whatever
//! resolved function pointers the loader gathered from the
//! dylib. Concrete impls are BACKLOG — the skeleton round
//! ships only the trait; the follow-up round that adds real
//! `libloading` integration lands a `DlopenHandle` impl.
//!
//! `Send + Sync` is required so a driver can share the
//! handle across threads (via `Arc` / `Mutex` wrappers if
//! needed) — future rayon-backed execution surfaces the
//! constraint.

use crate::error::DriverError;

/// Runtime-handle interface.
///
/// `execute` accepts an input buffer and returns either an
/// output buffer or a `DriverError`. Skeleton round has no
/// concrete impls; the first real impl (`DlopenHandle`) is
/// BACKLOG.
pub trait RuntimeHandle: Send + Sync {
    /// Hand `input` to the runtime and collect the output.
    // lint:allow(bare_collection) — the diagnostic return surface across every compiler phase crate matches what clause-resolve already ships; storage-crate collection types target mockspace domain graphs not host-side compiler runtime buffers here
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, DriverError>;
}

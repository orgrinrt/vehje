//! `ClauseRuntime` — opaque handle to a live runtime session.
//!
//! The type body is intentionally `[u8; 0]`: a zero-sized
//! private field that is never constructed in Rust and never
//! inspected in C. Consumers only see `*mut ClauseRuntime`,
//! which the runtime allocates on `clause_runtime_new` and
//! frees on `clause_runtime_free`.
//!
//! Skeleton round: `clause_runtime_new` always returns
//! `null_mut()`, so no real instance of this type ever exists
//! at runtime. The body becomes real once the runtime backend
//! (`mock/runtime-zig/`) ships a real allocator + state struct.

/// Opaque handle to a runtime session.
///
/// Never construct this type directly in Rust. It exists only
/// as a type-level marker for `*mut ClauseRuntime` pointers
/// crossing the FFI boundary.
#[repr(C)]
pub struct ClauseRuntime {
    _private: [u8; 0],
}

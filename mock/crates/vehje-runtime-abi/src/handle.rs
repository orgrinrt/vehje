//! `VehjeRuntime`, opaque handle to a live runtime session.
//!
//! The type body is intentionally `[u8; 0]`: a zero-sized
//! private field that is never constructed in Rust and never
//! inspected in C. Consumers only see `*mut VehjeRuntime`,
//! which the runtime allocates on `vehje_runtime_new` and
//! frees on `vehje_runtime_free`.
//!
//! Skeleton round: `vehje_runtime_new` always returns
//! `null_mut()`, so no real instance of this type ever exists
//! at runtime. The body becomes real once the runtime backend
//! (`mock/runtime-zig/`) ships a real allocator + state struct.

use core::marker::{PhantomData, PhantomPinned};

/// Opaque handle to a runtime session.
///
/// The handle points to foreign-owned memory (the Zig runtime's
/// session struct); only the Zig runtime may dereference it.
/// Rust treats it as an opaque token, never inspected, never
/// moved, never pinned on this side. Never construct this type
/// directly in Rust: it exists only as a type-level marker for
/// `*mut VehjeRuntime` pointers crossing the FFI boundary.
///
/// The `PhantomData<(*mut u8, PhantomPinned)>` marker makes the
/// handle `!Send`, `!Sync`, and `!Unpin`, blocking the default
/// auto-trait implementations that would otherwise let a caller
/// move the handle across threads or pin-project through it.
/// The raw-pointer component denies `Send`/`Sync`; the
/// `PhantomPinned` component denies `Unpin`.
#[repr(C)]
pub struct VehjeRuntime {
    _opaque: [u8; 0], // lint:allow(arvo-types-only) tracked: #207 lint:allow(no-bare-numeric) tracked: #207
    // PhantomData here makes the handle !Send, !Sync, !Unpin.
    // The handle points to foreign-owned memory; only the Zig
    // runtime may dereference it.
    _marker: PhantomData<(*mut u8, PhantomPinned)>, // lint:allow(arvo-types-only) tracked: #207 lint:allow(no-bare-numeric) tracked: #207
}

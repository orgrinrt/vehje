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

use hilavitkutin_api::ByteEmitter;
use notko::Outcome;

use crate::error::DriverError;

/// Runtime-handle interface.
///
/// `execute` accepts an input buffer and pushes the runtime's
/// output into `output`; returns `Outcome::Ok(())` on success or
/// `DriverError` on failure. Skeleton round has no concrete
/// impls; the first real impl (`DlopenHandle`) is BACKLOG.
pub trait RuntimeHandle: Send + Sync {
    /// Hand `input` to the runtime and push output bytes into
    /// `output`.
    ///
    /// `output` is `&mut dyn ByteEmitter` rather than `&mut impl`
    /// because `RuntimeHandle` is loaded via `dlopen` and crosses
    /// the `Box<dyn RuntimeHandle>` boundary in `loader.rs`; dyn
    /// methods are object-safe and must not carry `impl Trait`
    /// parameters.
    fn execute(
        &self,
        input: &[u8],  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
        output: &mut dyn ByteEmitter,
    ) -> Outcome<(), DriverError>;
}

//! vehje-runtime-driver, the framework's compile-side runtime dispatch.
//!
//! The compiler-side embedding and dispatch over the wire ABI, and the
//! in-process consumer of what crosses back: it serializes a checked residual
//! through `vehje-runtime-abi`, hands it to the embedded runtime, maps the
//! runtime's result and diagnostics back to `vehje-ir` diagnostics, and holds
//! the safe reader over the value-arena a produced value crosses back in. The
//! one framework crate with a `std` role, for the dlopen of the runtime
//! artifact.
//!
//! `#![no_std]` on the core paths, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use notko::Outcome;
use vehje_runtime_abi::{Residual, ValueArena};

/// A driver diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DriverError {
    /// The runtime rejected the residual.
    RuntimeRejected,
    /// The residual could not be serialized at the requested tier.
    SerializeFailed,
    /// A produced value failed the structural decode (a corrupt value-arena).
    CorruptValue,
}

/// Hand a checked residual to the runtime and read back its outcome.
///
/// Defines the dispatch entry; serialization through the tier tag, the runtime
/// linkage, and the result and diagnostic mapping are the next behavior gate.
// FIXME: serialize the residual at its tier through vehje-runtime-abi, call
// into the embedded runtime (statically linked, or dlopen'd behind the std
// feature), and map the wire diagnostics back to vehje-ir::Diagnostic. The
// surface ships; the embedding is owed.
pub fn dispatch(_residual: &Residual<'_>) -> Outcome<(), DriverError> {
    Outcome::Ok(())
}

/// The safe, bounds-checked reader over a produced value-arena.
///
/// The runtime output is a `vehje-runtime-abi` value-arena; the driver is the
/// in-process consumer, so it holds the safe reader and the validation pass a
/// produced value passes before the host reads it (the typed structural decode,
/// which the in-process path may skip and the untrusted path must run).
pub struct Reader<'a> {
    arena: ValueArena<'a>,
}

impl<'a> Reader<'a> {
    /// Wrap a produced value-arena for reading.
    pub fn new(arena: ValueArena<'a>) -> Self {
        Self { arena }
    }

    /// The wrapped value-arena, for the host to read its produced value.
    pub fn value(&self) -> &ValueArena<'a> {
        &self.arena
    }

    /// Validate the value-arena before the host reads it: a complete, linear,
    /// bounds-checked typed structural decode (children-before-parents makes
    /// acyclicity a single monotone index check).
    // FIXME: run the full typed structural decode (bounds, monotone child
    // indices, region-table integrity); the untrusted path must run it, the
    // in-process path may skip it. M-level accepts a well-formed arena.
    pub fn validate(&self) -> Outcome<(), DriverError> {
        Outcome::Ok(())
    }
}

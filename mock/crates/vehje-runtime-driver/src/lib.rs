//! vehje-runtime-driver, the framework's compile-side runtime dispatch.
//!
//! Takes a checked residual, serializes it at the chosen tier through
//! `vehje-runtime-abi`, hands it to the embedded Zig runtime, and maps the
//! runtime's result and diagnostics back to `vehje-ir` diagnostics.
//! Compile-time dispatch, no `dyn`, no plugin shared-object loading at the
//! compiler layer; the Zig runtime is statically linked.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use notko::Outcome;
use vehje_runtime_abi::Residual;

/// A driver diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DriverError {
    /// The runtime rejected the residual.
    RuntimeRejected,
    /// The residual could not be serialized at the requested tier.
    SerializeFailed,
}

/// Hand a checked residual to the runtime and read back its outcome.
///
/// M0 defines the dispatch entry; serialization through the tier tag,
/// the static Zig-runtime linkage, and the result and diagnostic mapping
/// are the next behavior gate.
// FIXME: serialize the residual at its tier through vehje-runtime-abi,
// call into the statically-linked Zig runtime, and map the wire
// diagnostics back to vehje-ir::Diagnostic. M0 ships the surface.
pub fn dispatch(_residual: &Residual) -> Outcome<(), DriverError> {
    Outcome::Ok(())
}

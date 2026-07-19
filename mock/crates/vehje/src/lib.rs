//! vehje, the framework's public API and composition.
//!
//! The two plug-in contracts a consumer builds against (the grammar
//! contract on the input side, the target contract on the output side),
//! the orchestration that runs a program from grammar through the Core
//! passes to a checked residual, and the distribution-composition entry
//! point that links a set of targets at compile time.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use notko::Outcome;
use vehje_ir::{Arena, Diagnostic, NodeRef};

/// The `Target` output contract, re-exported from `vehje-codegen`.
pub use vehje_codegen::{Checked, CodegenError, Target};

/// The input-side plug-in: a consumer's front-end produces well-formed
/// Core-plus-family IR.
///
/// The framework mandates the IR contract and offers, but does not
/// mandate, front-end scaffolding. A grammar lowers its surface syntax
/// into the Core forms plus its family node kinds, building into a
/// caller-provided arena and returning the program root.
pub trait Grammar {
    /// The grammar's error type.
    type Error;

    /// Lower a source into IR, building into `arena`, returning the
    /// program root.
    // FIXME: the source input type is a byte source the grammar reads;
    // M0 fixes the IR-producing shape, the source-side type lands with
    // the first consumer grammar.
    fn lower(&self, arena: &mut Arena<'_>) -> Outcome<NodeRef, Self::Error>;
}

/// Run a program from its IR through the Core passes to a checked
/// residual for a target.
///
/// M0 defines the orchestration entry; wiring resolve, check, the family
/// and effect inclusion checks, and the residual hand-off to the runtime
/// driver is the next behavior gate.
// FIXME: run vehje-resolve then vehje-typecheck (check) over the arena,
// run the inclusion checks to produce a Checked, and hand it to the
// runtime driver. M0 ships the surface.
pub fn run<T: Target>(_target: &T, _arena: &Arena<'_>, _root: NodeRef) -> Outcome<(), Diagnostic> {
    Outcome::Ok(())
}

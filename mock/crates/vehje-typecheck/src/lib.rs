//! vehje-typecheck, the framework's Core-level check pass.
//!
//! The `vehje-check` role (the crate directory keeps the
//! `vehje-typecheck` name until a later cosmetic rename). Checks the
//! resolved Core IR, generic over the family set, with family-extension
//! hooks for family-specific checks (a consumer's coherence, routing, or
//! fragment rules).
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::Outcome;

use vehje_ir::{Arena, NodeRef, Span};

/// A check diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CheckError {
    /// A Core form is malformed (bad arity, dangling child).
    Malformed { span: Span },
    /// A family node's own check failed.
    FamilyRule { span: Span },
}

/// The Core check pass over a program's IR.
///
/// Runs the shared, target-agnostic checks every consumer's IR must pass
/// (well-formed binders, arity, family and effect classifications
/// consistent with their nodes), and dispatches family nodes to their
/// family checks. M0 defines the entry; the shared checks and the
/// family-extension dispatch are the next behavior gate.
// FIXME: implement the shared Core checks (well-formedness, arity, the
// family/effect classification consistency) and the family-check dispatch
// for Raw nodes. M0 ships the surface; the check body is the behavior
// gate.
pub fn check(_arena: &Arena<'_>, _root: NodeRef) -> Outcome<(), CheckError> {
    Outcome::Ok(())
}

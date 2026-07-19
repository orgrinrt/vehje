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

use vehje_ir::{Arena, Node, NodeRef};

/// A check diagnostic.
///
/// M0 keys the diagnostic on the offending node. A source span requires a
/// per-node span table (see the same follow-up in vehje-resolve).
// FIXME: carry a Span once vehje-ir grows a per-node span side-table.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CheckError {
    /// A node refers to a child outside the arena (a corrupt IR).
    DanglingRef { at: NodeRef },
    /// A family node's own check failed.
    FamilyRule { at: NodeRef },
}

/// The Core check pass over a program's IR.
///
/// Walks the Core forms from `root`, validating that every child handle
/// is in bounds (a corrupt-IR integrity check), and dispatches family
/// nodes to their family checks. The type of `Node` already enforces the
/// per-form arity, so the Core-level checks are integrity plus the family
/// dispatch; a consumer's own type system is a family check plugged in
/// through the extension hook.
// FIXME: dispatch Raw to the family-check hook (M0 accepts it), and add
// the family/effect classification consistency checks once the classifier
// lands. M0 validates arena-integrity and traverses.
pub fn check(arena: &Arena<'_>, root: NodeRef) -> Outcome<(), CheckError> {
    walk(arena, root)
}

/// Recurse over one node, validating child handles are in bounds.
fn walk(arena: &Arena<'_>, at: NodeRef) -> Outcome<(), CheckError> {
    if at.index().0 >= arena.len().0 {
        return Outcome::Err(CheckError::DanglingRef { at });
    }
    match arena.get(at) {
        Node::Lit(_) | Node::Var(_) => Outcome::Ok(()),
        Node::Let { value, body, .. } => {
            walk(arena, value)?;
            walk(arena, body)
        }
        Node::Lambda { body, .. } => walk(arena, body),
        Node::Apply { callee, args } => {
            walk(arena, callee)?;
            for child in arena.list(args) {
                walk(arena, *child)?;
            }
            Outcome::Ok(())
        }
        Node::Project { base, .. } => walk(arena, base),
        Node::If { cond, then_branch, else_branch } => {
            walk(arena, cond)?;
            walk(arena, then_branch)?;
            walk(arena, else_branch)
        }
        Node::Match { scrutinee, arms } => {
            walk(arena, scrutinee)?;
            for child in arena.list(arms) {
                walk(arena, *child)?;
            }
            Outcome::Ok(())
        }
        Node::Iter { seq, body } => {
            walk(arena, seq)?;
            walk(arena, body)
        }
        Node::Interp { value } => walk(arena, value),
        // FIXME: dispatch Raw to the family-check hook. M0 accepts it.
        Node::Raw { .. } => Outcome::Ok(()),
    }
}

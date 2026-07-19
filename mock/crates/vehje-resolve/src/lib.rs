//! vehje-resolve, the framework's Core-level resolve pass.
//!
//! Name resolution over the shared Core binders (`Let`, `Lambda`,
//! `Var`), with a borrowed non-allocating scope chain: each scope frame
//! lives on the walk's own stack and chains to its parent by reference,
//! so no owned scope table is allocated. Generic over the family set,
//! with family-extension hooks for family-specific resolution.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::{Maybe, Outcome};

use hilavitkutin_str::Str;
use vehje_ir::{Arena, NodeRef, Span};

/// A borrowed scope frame: a single binding, chained to its parent.
///
/// A scope chain is a stack of these, each frame borrowed from the
/// resolve walk's own call stack. Resolution of a `Var` walks the chain
/// from the innermost frame outward. No allocation: the chain is the call
/// stack.
pub struct Scope<'p> {
    /// The name this frame binds.
    pub name: Str,
    /// The node that introduced the binding.
    pub binder: NodeRef,
    /// The enclosing scope, or `Isnt` at the root.
    pub parent: Maybe<&'p Scope<'p>>,
}

impl<'p> Scope<'p> {
    /// A root frame with no parent.
    pub fn root(name: Str, binder: NodeRef) -> Self {
        Self { name, binder, parent: Maybe::Isnt }
    }

    /// A child frame chained to `parent`.
    pub fn child(name: Str, binder: NodeRef, parent: &'p Scope<'p>) -> Self {
        Self { name, binder, parent: Maybe::Is(parent) }
    }

    /// Resolve `name` by walking the chain from this frame outward.
    pub fn resolve(&self, name: Str) -> Maybe<NodeRef> {
        if self.name == name {
            return Maybe::Is(self.binder);
        }
        match self.parent {
            Maybe::Is(p) => p.resolve(name),
            Maybe::Isnt => Maybe::Isnt,
        }
    }
}

/// A resolve diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ResolveError {
    /// A `Var` names no binding in scope.
    Unresolved { span: Span },
    /// Two bindings of the same name in one scope.
    Duplicate { span: Span },
}

/// The Core resolve pass over a program's IR.
///
/// Generic over the family set through the family-extension hooks (a
/// family's own binding forms extend the walk). M0 defines the entry and
/// the scope-chain machinery; the full walk over every Core binder, plus
/// the family-extension dispatch, is the next behavior gate.
// FIXME: implement the full resolve walk over the Core forms (Let/Lambda
// introduce frames; Var resolves against the chain; Apply/Project/If/etc.
// recurse) and the family-extension hook dispatch. M0 ships the scope
// chain and the entry; the walk body is the behavior gate.
pub fn resolve(_arena: &mut Arena<'_>, _root: NodeRef) -> Outcome<(), ResolveError> {
    Outcome::Ok(())
}

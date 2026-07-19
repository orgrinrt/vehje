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
// const_trait_impl: WATCH-allowed (unstable-features.md); required by
// hilavitkutin-str's `str_const!` for its static-context const construction.
#![feature(const_trait_impl)]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::{Maybe, Outcome};

use hilavitkutin_str::Str;
use vehje_ir::{Arena, Node, NodeRef};

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
///
/// M0 keys the diagnostic on the offending name. A source span requires a
/// per-node span table (nodes carry no span in the arena today); adding
/// it is a marked follow-up.
// FIXME: carry a Span once vehje-ir grows a per-node span side-table (a
// caller-provided region parallel to the node arena). M0 reports the name.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ResolveError {
    /// A `Var` names no binding in scope.
    Unresolved { name: Str },
}

/// The Core resolve pass over a program's IR.
///
/// Walks the Core forms from `root`: `Let` and `Lambda` introduce scope
/// frames (borrowed on the walk's own stack); `Var` resolves against the
/// chain; the compound forms recurse into their children. An unresolved
/// `Var` is refused with its name.
///
/// Generic over the family set through the family-extension hooks. The
/// hooks (a family's own binding forms) and a produced resolution
/// side-table are the next additions.
// FIXME: dispatch Raw nodes to the family-extension resolve hooks, and
// produce a resolution side-table (Var handle to binder handle) for the
// check and emit passes. M0 walks and validates against the scope chain.
pub fn resolve(arena: &mut Arena<'_>, root: NodeRef) -> Outcome<(), ResolveError> {
    walk(arena, root, Maybe::Isnt)
}

/// Recurse over one node, resolving `Var`s against `scope`.
fn walk(arena: &Arena<'_>, at: NodeRef, scope: Maybe<&Scope<'_>>) -> Outcome<(), ResolveError> {
    match arena.get(at) {
        Node::Lit(_) => Outcome::Ok(()),
        Node::Var(name) => match scope {
            Maybe::Is(s) => match s.resolve(name) {
                Maybe::Is(_) => Outcome::Ok(()),
                Maybe::Isnt => Outcome::Err(ResolveError::Unresolved { name }),
            },
            Maybe::Isnt => Outcome::Err(ResolveError::Unresolved { name }),
        },
        Node::Let { name, value, body, .. } => {
            walk(arena, value, scope)?;
            let frame = frame_for(name, at, scope);
            walk(arena, body, Maybe::Is(&frame))
        }
        Node::Lambda { param, body } => {
            let frame = frame_for(param, at, scope);
            walk(arena, body, Maybe::Is(&frame))
        }
        Node::Apply { callee, args } => {
            walk(arena, callee, scope)?;
            for child in arena.list(args) {
                walk(arena, *child, scope)?;
            }
            Outcome::Ok(())
        }
        Node::Project { base, .. } => walk(arena, base, scope),
        Node::If { cond, then_branch, else_branch } => {
            walk(arena, cond, scope)?;
            walk(arena, then_branch, scope)?;
            walk(arena, else_branch, scope)
        }
        Node::Match { scrutinee, arms } => {
            walk(arena, scrutinee, scope)?;
            for child in arena.list(arms) {
                walk(arena, *child, scope)?;
            }
            Outcome::Ok(())
        }
        Node::Iter { seq, body } => {
            walk(arena, seq, scope)?;
            walk(arena, body, scope)
        }
        Node::Interp { value } => walk(arena, value, scope),
        // FIXME: dispatch Raw to the family-extension resolve hook; M0
        // treats a family node as opaque (no Core-level names inside).
        Node::Raw { .. } => Outcome::Ok(()),
    }
}

/// A scope frame binding `name` at `binder`, chained to `parent`.
fn frame_for<'p>(name: Str, binder: NodeRef, parent: Maybe<&'p Scope<'p>>) -> Scope<'p> {
    match parent {
        Maybe::Is(p) => Scope::child(name, binder, p),
        Maybe::Isnt => Scope::root(name, binder),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use arvo::{Bool, Identity, USize};
    use hilavitkutin_str::str_const;
    use vehje_ir::{Builder, Literal, Node};

    fn expect(m: Maybe<NodeRef>) -> NodeRef {
        match m {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        }
    }

    #[test]
    fn resolves_a_bound_var() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut pool));

        // let x = () in x
        let x = str_const!("x");
        let unit = expect(b.lit(Literal::Unit));
        let var = expect(b.var(x));
        let root = expect(b.let_(Bool::FALSE, x, unit, var));

        let mut arena = b.into_arena();
        assert!(matches!(resolve(&mut arena, root), Outcome::Ok(())));
    }

    #[test]
    fn refuses_an_unbound_var() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut pool));

        // a bare, unbound reference to y
        let y = str_const!("y");
        let root = expect(b.var(y));

        let mut arena = b.into_arena();
        assert!(matches!(
            resolve(&mut arena, root),
            Outcome::Err(ResolveError::Unresolved { .. })
        ));
    }
}

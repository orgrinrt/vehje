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

use arvo::{Maybe, Outcome, USize};

use hilavitkutin_str::Str;
use vehje_ir::{Arena, Node, NodeRef, Span};

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
/// Keys the diagnostic on the offending name and the source span of the
/// `Var` node that failed to resolve, recovered from the arena's span
/// side-table.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ResolveError {
    /// A `Var` names no binding in scope.
    Unresolved { name: Str, span: Span },
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
pub fn resolve(arena: &Arena<'_>, root: NodeRef) -> Outcome<(), ResolveError> {
    walk(arena, root, Maybe::Isnt)
}

/// Recurse over one node, resolving `Var`s against `scope`.
fn walk(arena: &Arena<'_>, at: NodeRef, scope: Maybe<&Scope<'_>>) -> Outcome<(), ResolveError> {
    match arena.get(at) {
        Node::Lit(_) => Outcome::Ok(()),
        Node::Var(name) => match scope {
            Maybe::Is(s) => match s.resolve(name) {
                Maybe::Is(_) => Outcome::Ok(()),
                Maybe::Isnt => Outcome::Err(ResolveError::Unresolved { name, span: arena.span(at) }),
            },
            Maybe::Isnt => Outcome::Err(ResolveError::Unresolved { name, span: arena.span(at) }),
        },
        Node::Let { rec, name, value, body } => {
            let frame = frame_for(name, at, scope);
            // a recursive binding is in scope for its own value, so `let rec f =
            // ... f ...` can reference itself; a non-recursive `let` is not.
            let value_scope = if rec.0 { Maybe::Is(&frame) } else { scope };
            walk(arena, value, value_scope)?;
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
            // `Node::Iter` carries no loop-variable name in the current IR, so
            // it introduces no scope frame here. If a future IR gives `Iter` a
            // named loop variable, resolve pushes its frame around `body`, the
            // way `Let` and `Lambda` do.
            walk(arena, seq, scope)?;
            walk(arena, body, scope)
        }
        Node::Interp { value } => walk(arena, value, scope),
        // FIXME: dispatch Raw to the family-extension resolve hook; M0
        // treats a family node as opaque (no Core-level names inside).
        Node::Raw { .. } => Outcome::Ok(()),
        Node::Handle { body, clauses } => {
            // FIXME: a handler clause binds the operation and the resumption;
            // once the clause representation lands, push those binders before
            // walking the clause body. M-level walks the body and the clause
            // bodies without the operation/resumption binders.
            walk(arena, body, scope)?;
            for child in arena.list(clauses) {
                walk(arena, *child, scope)?;
            }
            Outcome::Ok(())
        }
    }
}

/// A scope frame binding `name` at `binder`, chained to `parent`.
fn frame_for<'p>(name: Str, binder: NodeRef, parent: Maybe<&'p Scope<'p>>) -> Scope<'p> {
    match parent {
        Maybe::Is(p) => Scope::child(name, binder, p),
        Maybe::Isnt => Scope::root(name, binder),
    }
}

/// The resolution side-table.
///
/// Each resolved `Var`'s arena index maps to the binder that introduced it,
/// over a caller-provided region parallel to the node arena. The check and
/// lower passes read it so they do not re-resolve. No allocation.
pub struct Resolution<'a> {
    binders: &'a mut [Maybe<NodeRef>],
}

impl<'a> Resolution<'a> {
    /// Wrap a caller-provided region sized like the node arena; every entry
    /// starts unresolved.
    pub fn new(binders: &'a mut [Maybe<NodeRef>]) -> Self {
        Self { binders }
    }

    /// The binder a `Var` at arena index `at` resolved to, or `Isnt`.
    pub fn binder(&self, at: USize) -> Maybe<NodeRef> {
        self.binders[at.0]
    }

    /// Record that the `Var` at arena index `at` resolves to `binder`.
    pub fn record(&mut self, at: USize, binder: NodeRef) {
        self.binders[at.0] = Maybe::Is(binder);
    }
}

/// Resolve a program, recording each `Var`'s binder into `resolution`.
///
/// Like [`resolve`], but populates the resolution side-table the later passes
/// read. An unresolved `Var` is still refused with its name.
pub fn resolve_into(
    arena: &Arena<'_>,
    root: NodeRef,
    resolution: &mut Resolution<'_>,
) -> Outcome<(), ResolveError> {
    walk_record(arena, root, Maybe::Isnt, resolution)
}

/// Recurse over one node, resolving and recording `Var` bindings.
fn walk_record(
    arena: &Arena<'_>,
    at: NodeRef,
    scope: Maybe<&Scope<'_>>,
    res: &mut Resolution<'_>,
) -> Outcome<(), ResolveError> {
    match arena.get(at) {
        Node::Lit(_) => Outcome::Ok(()),
        Node::Var(name) => match scope {
            Maybe::Is(s) => match s.resolve(name) {
                Maybe::Is(binder) => {
                    res.record(at.index(), binder);
                    Outcome::Ok(())
                }
                Maybe::Isnt => Outcome::Err(ResolveError::Unresolved { name, span: arena.span(at) }),
            },
            Maybe::Isnt => Outcome::Err(ResolveError::Unresolved { name, span: arena.span(at) }),
        },
        Node::Let { rec, name, value, body } => {
            let frame = frame_for(name, at, scope);
            // recursive binding is in scope for its own value; see `walk`.
            let value_scope = if rec.0 { Maybe::Is(&frame) } else { scope };
            walk_record(arena, value, value_scope, res)?;
            walk_record(arena, body, Maybe::Is(&frame), res)
        }
        Node::Lambda { param, body } => {
            let frame = frame_for(param, at, scope);
            walk_record(arena, body, Maybe::Is(&frame), res)
        }
        Node::Apply { callee, args } => {
            walk_record(arena, callee, scope, res)?;
            for child in arena.list(args) {
                walk_record(arena, *child, scope, res)?;
            }
            Outcome::Ok(())
        }
        Node::Project { base, .. } => walk_record(arena, base, scope, res),
        Node::If { cond, then_branch, else_branch } => {
            walk_record(arena, cond, scope, res)?;
            walk_record(arena, then_branch, scope, res)?;
            walk_record(arena, else_branch, scope, res)
        }
        Node::Match { scrutinee, arms } => {
            walk_record(arena, scrutinee, scope, res)?;
            for child in arena.list(arms) {
                walk_record(arena, *child, scope, res)?;
            }
            Outcome::Ok(())
        }
        Node::Iter { seq, body } => {
            walk_record(arena, seq, scope, res)?;
            walk_record(arena, body, scope, res)
        }
        Node::Interp { value } => walk_record(arena, value, scope, res),
        Node::Raw { .. } => Outcome::Ok(()),
        Node::Handle { body, clauses } => {
            walk_record(arena, body, scope, res)?;
            for child in arena.list(clauses) {
                walk_record(arena, *child, scope, res)?;
            }
            Outcome::Ok(())
        }
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
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // let x = () in x
        let x = str_const!("x");
        let unit = expect(b.lit(Literal::Unit, Span::default()));
        let var = expect(b.var(x, Span::default()));
        let root = expect(b.let_(Bool::FALSE, x, unit, var, Span::default()));

        let arena = b.into_arena();
        assert!(matches!(resolve(&arena, root), Outcome::Ok(())));
    }

    #[test]
    fn refuses_an_unbound_var() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // a bare, unbound reference to y
        let y = str_const!("y");
        let root = expect(b.var(y, Span::default()));

        let arena = b.into_arena();
        assert!(matches!(
            resolve(&arena, root),
            Outcome::Err(ResolveError::Unresolved { .. })
        ));
    }

    #[test]
    fn resolves_a_recursive_let() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // let rec f = f in f: the bound value references the binding itself,
        // which resolves only because `rec` puts the binder in scope for it.
        let f = str_const!("f");
        let value = expect(b.var(f, Span::default()));
        let body = expect(b.var(f, Span::default()));
        let root = expect(b.let_(Bool::TRUE, f, value, body, Span::default()));

        let arena = b.into_arena();
        assert!(matches!(resolve(&arena, root), Outcome::Ok(())));
    }

    #[test]
    fn non_recursive_let_does_not_bind_its_value() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // let f = f in f (non-recursive): the value's `f` is unbound, so the
        // rec flag is load-bearing: without it this same shape resolves, with
        // it (above) it does not.
        let f = str_const!("f");
        let value = expect(b.var(f, Span::default()));
        let body = expect(b.var(f, Span::default()));
        let root = expect(b.let_(Bool::FALSE, f, value, body, Span::default()));

        let arena = b.into_arena();
        assert!(matches!(
            resolve(&arena, root),
            Outcome::Err(ResolveError::Unresolved { .. })
        ));
    }
}

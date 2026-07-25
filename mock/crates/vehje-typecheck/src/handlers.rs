//! The handler scope and the escape promotion: the machinery that makes a
//! lexically handled operation invisible to the effect mask, and a latently
//! performing lambda visible to the lease axis.
//!
//! Split from the graded fold because it is a different concern with a
//! different shape: the fold is bottom-up over grades, and this is a scope
//! carried top-down plus a term walk that reads no grades at all.

use arvo::{Bool, Maybe, USize};
use hilavitkutin_sym::Sym;
use vehje_ir::{Arena, Node, NodeRef, ReachMask};
use vehje_resolve::Resolution;

use crate::slot_of;

/// A never-matching placeholder for the handler-scope buffer.
const ZERO_SYM: Sym = Sym::new(hilavitkutin_sym::SymKind::from_raw(0b111), arvo::Bits::from_raw(0)); // lint:allow(no-bare-numeric) reason: scratch placeholder; tracked: #207

/// The most handler frames one walk may nest.
const HANDLER_DEPTH: usize = 64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a lent walk bound; tracked: #207

/// The operations lexically in scope, each paired with the `Handle` that
/// services it.
///
/// Carried down the walk the way `resolve` carries binders, which is what lets
/// a handled `Perform` contribute nothing to the effect mask IN THE FIRST PLACE.
/// The alternative, subtracting a mask at the `Handle`, is unsound when two
/// operations share a slot; this formulation avoids the question rather than
/// answering it.
#[derive(Copy, Clone)]
pub(crate) struct HandlerScope {
    ops: [(Sym, NodeRef); HANDLER_DEPTH],
    len: USize,
}

impl HandlerScope {
    pub(crate) const fn empty() -> Self {
        Self { ops: [(ZERO_SYM, NodeRef::new(USize(0))); HANDLER_DEPTH], len: USize(0) }
    }

    /// The innermost `Handle` servicing `op`, if any.
    pub(crate) fn lookup(&self, op: Sym) -> Maybe<NodeRef> {
        let mut i = self.len.0;
        while i > 0 {
            i -= 1;
            if self.ops[i].0 == op {
                return Maybe::Is(self.ops[i].1);
            }
        }
        Maybe::Isnt
    }

    pub(crate) fn push(&mut self, op: Sym, owner: NodeRef) -> Bool {
        if self.len.0 >= HANDLER_DEPTH {
            return Bool(false);
        }
        self.ops[self.len.0] = (op, owner);
        self.len = USize(self.len.0 + 1);
        Bool(true)
    }
}

/// Walk `body` for operations a handler above services, putting that handler's
/// slot on `reach`.
///
/// This is the promotion half of the escape rule. It descends the term without
/// grading it, because the grades are already recorded by the time a `Lambda`
/// promotes; it is looking only for which enclosing handlers the body can still
/// reach through a performed operation.
pub(crate) fn promote_latent(
    arena: &Arena<'_>,
    at: NodeRef,
    res: &Resolution<'_>,
    handled: &HandlerScope,
    reach: &mut ReachMask,
) {
    if at.index().0 >= arena.len().0 {
        return;
    }
    match arena.get(at) {
        Node::Perform { op, args } => {
            if let Maybe::Is(owner) = handled.lookup(op) {
                reach.insert(slot_of(owner));
            }
            for a in arena.list(args) {
                promote_latent(arena, *a, res, handled, reach);
            }
        }
        Node::Handle { body, clauses } => {
            // an operation this handler services is discharged here, so it does
            // not reach past it; only the ones it does not service promote.
            let mut inner = *handled;
            for c in arena.clauses(clauses) {
                let _ = inner.push(c.op, at);
            }
            promote_latent(arena, body, res, &inner, reach);
            for c in arena.clauses(clauses) {
                promote_latent(arena, c.body, res, handled, reach);
            }
        }
        Node::Lit(_) | Node::Var(_) => {}
        Node::Let { value, body, .. } => {
            promote_latent(arena, value, res, handled, reach);
            promote_latent(arena, body, res, handled, reach);
        }
        Node::Lambda { body, .. } => promote_latent(arena, body, res, handled, reach),
        Node::Apply { callee, args } => {
            promote_latent(arena, callee, res, handled, reach);
            for a in arena.list(args) {
                promote_latent(arena, *a, res, handled, reach);
            }
        }
        Node::Project { base, .. } => promote_latent(arena, base, res, handled, reach),
        Node::If { cond, then_branch, else_branch } => {
            promote_latent(arena, cond, res, handled, reach);
            promote_latent(arena, then_branch, res, handled, reach);
            promote_latent(arena, else_branch, res, handled, reach);
        }
        Node::Match { scrutinee, arms } => {
            promote_latent(arena, scrutinee, res, handled, reach);
            for a in arena.list(arms) {
                promote_latent(arena, *a, res, handled, reach);
            }
        }
        Node::Iter { seq, body } => {
            promote_latent(arena, seq, res, handled, reach);
            promote_latent(arena, body, res, handled, reach);
        }
        Node::Interp { value } => promote_latent(arena, value, res, handled, reach),
        Node::Raw { payload, .. } => {
            for c in arena.list(payload) {
                promote_latent(arena, *c, res, handled, reach);
            }
        }
    }
}

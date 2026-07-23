//! vehje-check, the framework's graded check pass.
//!
//! The graded (co)modal judgment over the resolved Core IR: it infers the
//! effect, lease, and binding-time grades, computes assurance, checks
//! integrity, and mints the `Checked` witness. Where the
//! type-system-as-verification identity lives. The three axes discharge in one
//! bottom-up fold reusing one bitmask machinery; the reachability and effect
//! inference are (in the full design) fixpoint queries on `vehje-fixpoint`,
//! reading the per-family lease-rule schema from `vehje-signature`.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use core::marker::PhantomData;

use arvo::Outcome;

use vehje_ir::{
    Arena, Assurance, EffectMask, Grade, GradeTable, Knowledge, Lease, Node, NodeRef, ReachMask,
};

/// A check diagnostic.
///
/// Keys the diagnostic on the offending node. A source span is recovered from
/// the arena's span side-table by the renderer.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CheckError {
    /// A node refers to a child outside the arena (a corrupt IR).
    DanglingRef { at: NodeRef },
    /// An effect the target does not permit reaches emission.
    ForbiddenEffect { at: NodeRef },
    /// A lease that cannot be placed (a value escapes past what its type can
    /// express, at the avoidance boundary).
    UnplaceableLease { at: NodeRef },
    /// A family node's own check failed.
    FamilyRule { at: NodeRef },
}

/// A witness that a program has been graded and inclusion-checked against a
/// specific target `T`.
///
/// Minted here (the proof witness belongs with the prover) and consumed by
/// `vehje-codegen`'s `emit`, so the obligation to check is compile-time
/// unskippable. `T` is unbound here; a target binds it at the emit site.
pub struct Checked<'a, T> {
    arena: &'a Arena<'a>,
    root: NodeRef,
    _target: PhantomData<T>,
}

impl<'a, T> Checked<'a, T> {
    /// Construct the witness for a graded, inclusion-checked program. Called
    /// by the inclusion check (compile-time in `vehje-codegen`, or the runtime
    /// bitmask path) once the program is proven acceptable for `T`.
    pub fn new(arena: &'a Arena<'a>, root: NodeRef) -> Self {
        Self { arena, root, _target: PhantomData }
    }

    /// The checked program's arena.
    pub fn arena(&self) -> &'a Arena<'a> {
        self.arena
    }

    /// The checked program's root node.
    pub fn root(&self) -> NodeRef {
        self.root
    }
}

/// The graded check pass over a program's IR.
///
/// Walks the resolved Core forms from `root`, validating that every child
/// handle is in bounds (a corrupt-IR integrity check), and computes each
/// node's grade (effect, lease, binding time, assurance) into `grades`. The
/// type of `Node` already enforces the per-form arity, so the Core-level check
/// is integrity plus the graded inference plus the family dispatch.
// FIXME: dispatch Raw and Handle to the family-check and handler-discharge
// hooks, and route the reach and effect inference through vehje-fixpoint as
// relational queries; M-level runs the inference as a direct bottom-up fold.
pub fn check(
    arena: &Arena<'_>,
    root: NodeRef,
    grades: &mut GradeTable<'_>,
) -> Outcome<(), CheckError> {
    match infer(arena, root, grades) {
        Outcome::Ok(_) => Outcome::Ok(()),
        Outcome::Err(e) => Outcome::Err(e),
    }
}

/// Bottom-up graded inference over one node, recording its grade and returning
/// it. Integrity-checks child handles as it descends.
fn infer(
    arena: &Arena<'_>,
    at: NodeRef,
    grades: &mut GradeTable<'_>,
) -> Outcome<Grade, CheckError> {
    if at.index().0 >= arena.len().0 {
        return Outcome::Err(CheckError::DanglingRef { at });
    }
    let node = arena.get(at);
    // effect and lease are computed as the join of the children's grades; the
    // per-form rules (the binder rule dropping a bound variable's reach slot,
    // a family operation's own effect) refine this and are FIXME'd below.
    let mut effect = EffectMask::empty();
    let mut reach = ReachMask::empty();
    let mut child = |c: NodeRef, grades: &mut GradeTable<'_>| -> Outcome<(), CheckError> {
        match infer(arena, c, grades) {
            Outcome::Ok(g) => {
                effect = effect.join(g.effect);
                reach = reach.join(g.lease.0);
                Outcome::Ok(())
            }
            Outcome::Err(e) => Outcome::Err(e),
        }
    };
    match node {
        Node::Lit(_) | Node::Var(_) => {}
        Node::Let { value, body, .. } => {
            child(value, grades)?;
            child(body, grades)?;
            // FIXME: apply the reachability binder rule here (splice the bound
            // expression's reach where the body reaches the variable, then drop
            // the variable's slot); M-level unions without the drop.
        }
        Node::Lambda { body, .. } => {
            child(body, grades)?;
            // FIXME: the closure-escape reach obligation (the load-bearing
            // region-soundness case) drops the parameter slot here.
        }
        Node::Apply { callee, args } => {
            child(callee, grades)?;
            for a in arena.list(args) {
                child(*a, grades)?;
            }
        }
        Node::Project { base, .. } => child(base, grades)?,
        Node::If { cond, then_branch, else_branch } => {
            child(cond, grades)?;
            child(then_branch, grades)?;
            child(else_branch, grades)?;
        }
        Node::Match { scrutinee, arms } => {
            child(scrutinee, grades)?;
            for a in arena.list(arms) {
                child(*a, grades)?;
            }
        }
        Node::Iter { seq, body } => {
            child(seq, grades)?;
            child(body, grades)?;
        }
        Node::Interp { value } => child(value, grades)?,
        Node::Raw { .. } => {
            // FIXME: dispatch to the family-check hook and fold in the family
            // operation's own declared effect from the signature.
        }
        Node::Handle { body, clauses } => {
            child(body, grades)?;
            for c in arena.list(clauses) {
                child(*c, grades)?;
            }
            // FIXME: a handled operation drops out of the residual's effect the
            // way a reduced build-env effect does; subtract the handled ops.
        }
    }
    let grade = Grade {
        effect,
        lease: Lease(reach),
        // FIXME: binding-time discharge (the earliest binding time the inputs
        // are known) and the assurance projection from it; M-level defaults to
        // nothing-known and unclaimed until the discharge lands.
        binding: Knowledge::empty(),
        assurance: Assurance::Unclaimed,
    };
    grades.set(at.index(), grade);
    Outcome::Ok(grade)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo::{Identity, USize};
    use vehje_ir::{Builder, Literal, Span};

    fn at(m: arvo::Maybe<NodeRef>) -> NodeRef {
        match m {
            arvo::Maybe::Is(r) => r,
            arvo::Maybe::Isnt => panic!("arena full"),
        }
    }

    #[test]
    fn checks_and_grades_a_program() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
        let unit = at(b.lit(Literal::Unit, Span::default()));
        let root = at(b.if_(unit, unit, unit, Span::default()));
        let arena = b.into_arena();

        let mut grade_region = [Grade::default(); 8];
        let mut grades = GradeTable::new(&mut grade_region);
        assert!(matches!(check(&arena, root, &mut grades), Outcome::Ok(())));
        // the whole program is pure and reaches nothing at this stage.
        assert_eq!(grades.get(root.index()).effect, EffectMask::empty());
    }
}

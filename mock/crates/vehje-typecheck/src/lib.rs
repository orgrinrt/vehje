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
// const_trait_impl: WATCH-allowed (unstable-features.md); required by
// hilavitkutin-str's `str_const!` for the interned name in the binder-rule test.
#![feature(const_trait_impl)]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use core::marker::PhantomData;

use arvo::Outcome;

use arvo::USize;
use vehje_ir::{
    Arena, Assurance, ContainsAll, EffectMask, Grade, GradeTable, Knowledge, Lease, Node, NodeRef,
    ReachMask, TargetSets,
};
use vehje_resolve::Resolution;

/// The reach-mask slot a binder or its variable occupies: the binder node's
/// arena index folded into the 64-slot mask. Two binders more than 64 apart
/// share a slot, the degenerate depth-lease floor the design names; a
/// census-sized mask is the later refinement.
fn slot_of(binder: NodeRef) -> USize {
    USize(binder.index().0 % 64) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64 is the ReachMask slot count (Bits<64>); a fixed mask-width index; tracked: #207
}

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
    /// Construct the witness for a graded, inclusion-checked program.
    ///
    /// Crate-private so the witness cannot be minted outside the sanctioned
    /// paths ([`mint_checked`], the compile-time inclusion proof, and the future
    /// runtime-bitmask path). A consumer cannot fabricate a `Checked` and skip
    /// the inclusion check, which is what makes the check obligation
    /// compile-time unskippable.
    pub(crate) fn new(arena: &'a Arena<'a>, root: NodeRef) -> Self {
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

/// Evidence that `check`'s graded fold ran clean over an arena and root.
///
/// A zero-cost witness the mint requires, so a `Checked` cannot be minted
/// without a clean check having run. It binds the exact arena and root that
/// were checked, and the mint builds the `Checked` from them, so the witness a
/// target consumes is over the program the check validated. The constructor is
/// crate-private, so only `check` produces a `Graded`.
// FIXME: `Graded` proves integrity (no dangling child handle) today; the
// program-derived family and effect sets it will also carry (so inclusion no
// longer trusts caller-supplied `Families`/`Effects`) land with the
// runtime-bitmask path (tracked #29).
pub struct Graded<'a> {
    pub(crate) arena: &'a Arena<'a>,
    pub(crate) root: NodeRef,
}

impl<'a> Graded<'a> {
    /// Bind the checked arena and root. Crate-private, so only `check` mints it.
    pub(crate) fn new(arena: &'a Arena<'a>, root: NodeRef) -> Self {
        Self { arena, root }
    }

    /// The checked program's arena.
    pub fn arena(&self) -> &'a Arena<'a> {
        self.arena
    }

    /// The checked program's root.
    pub fn root(&self) -> NodeRef {
        self.root
    }
}

/// Mint a `Checked` witness for target `T`, gated on the inclusion proof.
///
/// The target's declared sets come from `T: TargetSets`, not from free
/// parameters, so a caller cannot decouple the checked sets from the target and
/// claim a support or permit set the target does not declare. The `where` bounds
/// are the static half of the two-stage proof: the target's `Supports` set
/// contains every family the program is claimed to use, and its `Permits` set
/// every effect. A mismatch is a compile error naming the missing family or
/// effect. The `Graded` evidence, minted only by `check`, is required, so the
/// witness cannot be produced without a clean check having run. This is the only
/// sanctioned mint path (`Checked::new` is crate-private), so `vehje-codegen`'s
/// `check_for` and the future runtime-bitmask path both route through it.
// FIXME: `Families`/`Effects` are still caller-supplied (the program's *claimed*
// used sets); deriving them from the program itself, so a caller cannot claim
// `Empty` and dodge the check, is the remaining inclusion half, carried by the
// runtime-bitmask path on the `Graded` witness (tracked #29).
pub fn mint_checked<'a, T, Families, Effects>(
    graded: Graded<'a>,
    _inclusion: PhantomData<(Families, Effects)>,
) -> Checked<'a, T>
where
    T: TargetSets,
    T::Supports: ContainsAll<Families>,
    T::Permits: ContainsAll<Effects>,
{
    Checked::new(graded.arena, graded.root)
}

/// The graded check pass over a program's IR.
///
/// Walks the resolved Core forms from `root`, validating that every child
/// handle is in bounds (a corrupt-IR integrity check), and computes each
/// node's grade (effect, lease, binding time, assurance) into `grades`. The
/// type of `Node` already enforces the per-form arity, so the Core-level check
/// is integrity plus the graded inference plus the family dispatch.
///
/// The graded judgment splits in two: `check` computes and records the grades
/// and, on a clean fold, returns a [`Graded`] evidence token (this pass); the
/// target-typed inclusion proof then consumes that token to mint the [`Checked`]
/// witness (`check_for`, and `vehje-codegen`'s wrapper over it). A caller runs
/// `check` to obtain the `Graded`, then `check_for` to obtain the emit-gating
/// `Checked`, so the witness cannot be minted without a clean check.
// FIXME: dispatch Raw and Handle to the family-check and handler-discharge
// hooks, and route the reach and effect inference through vehje-fixpoint as
// relational queries; M-level runs the inference as a direct bottom-up fold.
pub fn check<'a>(
    arena: &'a Arena<'a>,
    root: NodeRef,
    res: &Resolution<'_>,
    grades: &mut GradeTable<'_>,
) -> Outcome<Graded<'a>, CheckError> {
    match infer(arena, root, res, grades) {
        Outcome::Ok(_) => Outcome::Ok(Graded::new(arena, root)),
        Outcome::Err(e) => Outcome::Err(e),
    }
}

/// Bottom-up graded inference over one node, recording its grade and returning
/// it. Integrity-checks child handles as it descends, and computes the reach
/// set through the binder rule using the resolution side-table.
fn infer(
    arena: &Arena<'_>,
    at: NodeRef,
    res: &Resolution<'_>,
    grades: &mut GradeTable<'_>,
) -> Outcome<Grade, CheckError> {
    if at.index().0 >= arena.len().0 {
        return Outcome::Err(CheckError::DanglingRef { at });
    }
    let node = arena.get(at);
    // effect and lease are computed as the join of the children's grades; the
    // binder rule below drops a bound variable's reach slot, and a `Var`
    // contributes its resolved binder's slot.
    let mut effect = EffectMask::empty();
    let mut reach = ReachMask::empty();
    let mut child = |c: NodeRef, grades: &mut GradeTable<'_>| -> Outcome<(), CheckError> {
        match infer(arena, c, res, grades) {
            Outcome::Ok(g) => {
                effect = effect.join(g.effect);
                reach = reach.join(g.lease.0);
                Outcome::Ok(())
            }
            Outcome::Err(e) => Outcome::Err(e),
        }
    };
    match node {
        Node::Lit(_) => {}
        Node::Var(_) => {
            // a variable reaches the binder that introduced it: contribute the
            // binder's slot to this node's reach set.
            if let arvo::Maybe::Is(binder) = res.binder(at.index()) {
                reach.insert(slot_of(binder));
            }
        }
        Node::Let { value, body, .. } => {
            child(value, grades)?;
            child(body, grades)?;
            // the binder rule: the variable this `Let` introduces does not
            // escape its own binder, so drop its slot from the combined reach.
            reach.remove(slot_of(at));
        }
        Node::Lambda { body, .. } => {
            child(body, grades)?;
            // the closure-escape reach obligation: the parameter this `Lambda`
            // binds does not escape, so its slot is dropped here.
            reach.remove(slot_of(at));
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
    use arvo::{Bool, Identity, Maybe, USize};
    use hilavitkutin_str::str_const;
    use vehje_ir::{Builder, Literal, Span};
    use vehje_resolve::resolve_into;

    fn at(m: Maybe<NodeRef>) -> NodeRef {
        match m {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        }
    }

    fn grade_of(grades: &GradeTable<'_>, at: NodeRef) -> Grade {
        match grades.get(at.index()) {
            Maybe::Is(g) => g,
            Maybe::Isnt => panic!("grade out of range"),
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

        let mut binders = [Maybe::Isnt; 8];
        let mut res = Resolution::new(&mut binders);
        assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

        let mut grade_region = [Grade::default(); 8];
        let mut grades = GradeTable::new(&mut grade_region);
        assert!(matches!(check(&arena, root, &res, &mut grades), Outcome::Ok(_)));
        // the whole program is pure and reaches nothing at this stage.
        assert_eq!(grade_of(&grades, root).effect, EffectMask::empty());
    }

    #[test]
    fn binder_rule_drops_the_bound_slot() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // let x = () in x: the body reaches the binder, and the binder rule
        // drops the binder's own slot, so the Let's reach set is empty.
        let x = str_const!("x");
        let unit = at(b.lit(Literal::Unit, Span::default()));
        let var = at(b.var(x, Span::default()));
        let root = at(b.let_(Bool::FALSE, x, unit, var, Span::default()));
        let arena = b.into_arena();

        let mut binders = [Maybe::Isnt; 8];
        let mut res = Resolution::new(&mut binders);
        assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

        let mut grade_region = [Grade::default(); 8];
        let mut grades = GradeTable::new(&mut grade_region);
        assert!(matches!(check(&arena, root, &res, &mut grades), Outcome::Ok(_)));

        // the Let's own binder slot is dropped from its reach set.
        let slot = USize(root.index().0 % 64); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ReachMask slot-count index; tracked: #207
        assert!(!grade_of(&grades, root).lease.0.contains(slot).0);
    }

    #[test]
    #[ignore = "catalogue: slot_of folds the binder index mod 64, so two binders exactly 64 apart share a reach slot (the design's degenerate depth-lease floor); a census-sized mask removes the collision; tracked #30"]
    fn distant_binders_get_distinct_reach_slots() {
        // the intended behaviour once the reach mask is census-sized: binders far
        // apart do not share a slot. Today `slot_of` is index mod 64, so this
        // asserts the fixed behaviour and stays red until the mask widens.
        assert_ne!(slot_of(NodeRef::new(USize(0))), slot_of(NodeRef::new(USize(64))));
    }

    #[test]
    fn refuses_a_dangling_root() {
        let mut nodes = [Node::Lit(Literal::Unit); 4];
        let mut spans = [Span::default(); 4];
        let mut pool = [NodeRef::new(USize::ZERO); 4];
        let b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
        let arena = b.into_arena();

        // a root past the end of the (empty) arena: check must refuse it rather
        // than mint a Graded over a corrupt IR, so a dangling reference cannot
        // reach emit. This is the integrity half of the evidence-of-check gate.
        let dangling = NodeRef::new(USize(3));
        let mut binders = [Maybe::Isnt; 4];
        let res = Resolution::new(&mut binders);
        let mut grade_region = [Grade::default(); 4];
        let mut grades = GradeTable::new(&mut grade_region);
        assert!(matches!(
            check(&arena, dangling, &res, &mut grades),
            Outcome::Err(CheckError::DanglingRef { .. })
        ));
    }
}

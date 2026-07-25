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

use arvo::{Bool, Maybe, Outcome};
use hilavitkutin_sym::Sym;

use arvo::USize;
use vehje_ir::{
    Arena, Assurance, ContainsAll, EffectMask, FamilyId, Grade, GradeTable, Knowledge, Lease, Node,
    NodeRef, ReachMask, TargetSets,
};
use vehje_resolve::{CoreFamilies, Resolution};

/// The family-extension check hook.
///
/// A consumer implements this to declare a family operation's own effect. The
/// check pass descends into a `Raw` node's payload, joins its children's grades,
/// and joins the effect this hook returns for the family. The default is the
/// empty effect (a pure family operation), so the Core-only path is unchanged.
pub trait FamilyCheck {
    /// The effect a `Raw` node of family `family` contributes. Defaults to the
    /// empty effect.
    fn effect_of_raw(&self, _family: FamilyId) -> EffectMask {
        EffectMask::empty()
    }
}

// `CoreFamilies` (the shared Core-only defaults marker from `vehje-resolve`) is
// the no-family-effect hook `check` uses, so one marker names the Core-only case
// across both the resolve and check family seams.
impl FamilyCheck for CoreFamilies {}

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
    /// More nested handler frames than the walk's lent bound allows.
    HandlerDepthExceeded { at: NodeRef },
    /// A family node's own check failed.
    FamilyRule { at: NodeRef },
    /// The caller-lent grade region is smaller than the node arena, so a
    /// node's grade could not be recorded. A sizing error, not a program error.
    GradeRegionFull { at: NodeRef },
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
/// The Core-only entry, using the `CoreFamilies` hook (no family effects). A
/// payload-bearing family consumer uses [`check_with`].
// FIXME: dispatch Handle to the handler-discharge hook, and route the reach and
// effect inference through vehje-fixpoint as relational queries; M-level runs
// the inference as a direct bottom-up fold. Raw is now dispatched to the
// family-check hook and its payload descended.
pub fn check<'a>(
    arena: &'a Arena<'a>,
    root: NodeRef,
    res: &Resolution<'_>,
    grades: &mut GradeTable<'_>,
) -> Outcome<Graded<'a>, CheckError> {
    check_with(arena, root, res, grades, &CoreFamilies)
}

/// Like [`check`], dispatching each `Raw` node to `hook` for its family effect
/// while grading the payload's Core children.
pub fn check_with<'a, H: FamilyCheck>(
    arena: &'a Arena<'a>,
    root: NodeRef,
    res: &Resolution<'_>,
    grades: &mut GradeTable<'_>,
    hook: &H,
) -> Outcome<Graded<'a>, CheckError> {
    match infer(arena, root, res, grades, hook, &HandlerScope::empty()) {
        Outcome::Ok(_) => Outcome::Ok(Graded::new(arena, root)),
        Outcome::Err(e) => Outcome::Err(e),
    }
}

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
struct HandlerScope {
    ops: [(Sym, NodeRef); HANDLER_DEPTH],
    len: USize,
}

impl HandlerScope {
    const fn empty() -> Self {
        Self { ops: [(ZERO_SYM, NodeRef::new(USize(0))); HANDLER_DEPTH], len: USize(0) }
    }

    /// The innermost `Handle` servicing `op`, if any.
    fn lookup(&self, op: Sym) -> Maybe<NodeRef> {
        let mut i = self.len.0;
        while i > 0 {
            i -= 1;
            if self.ops[i].0 == op {
                return Maybe::Is(self.ops[i].1);
            }
        }
        Maybe::Isnt
    }

    fn push(&mut self, op: Sym, owner: NodeRef) -> Bool {
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
fn promote_latent(
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

/// Bottom-up graded inference over one node, recording its grade and returning
/// it. Integrity-checks child handles as it descends, and computes the reach
/// set through the binder rule using the resolution side-table.
fn infer<H: FamilyCheck>(
    arena: &Arena<'_>,
    at: NodeRef,
    res: &Resolution<'_>,
    grades: &mut GradeTable<'_>,
    hook: &H,
    handled: &HandlerScope,
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
    let child = |c: NodeRef,
                 grades: &mut GradeTable<'_>,
                 effect: &mut EffectMask,
                 reach: &mut ReachMask|
     -> Outcome<(), CheckError> {
        match infer(arena, c, res, grades, hook, handled) {
            Outcome::Ok(g) => {
                *effect = effect.join(g.effect);
                *reach = reach.join(g.lease.0);
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
            child(value, grades, &mut effect, &mut reach)?;
            child(body, grades, &mut effect, &mut reach)?;
            // the binder rule: the variable this `Let` introduces does not
            // escape its own binder, so drop its slot from the combined reach.
            reach.remove(slot_of(at));
        }
        Node::Lambda { body, .. } => {
            child(body, grades, &mut effect, &mut reach)?;
            // the closure-escape reach obligation: the parameter this `Lambda`
            // binds does not escape, so its slot is dropped here.
            reach.remove(slot_of(at));
            // THE LATENT-EFFECT PROMOTION. The effect axis records that the
            // BODY performs; the coeffect axis has to record that the lambda
            // VALUE can perform later. So every operation the body performs
            // that a handler above services puts that handler's slot on this
            // value's reach, and the `Handle` arm refuses a result that names
            // it. This is the first genuine effect-and-lease interaction rule
            // in the system; it is intended, not proven.
            promote_latent(arena, body, res, handled, &mut reach);
        }
        Node::Apply { callee, args } => {
            child(callee, grades, &mut effect, &mut reach)?;
            for a in arena.list(args) {
                child(*a, grades, &mut effect, &mut reach)?;
            }
        }
        Node::Project { base, .. } => child(base, grades, &mut effect, &mut reach)?,
        Node::If { cond, then_branch, else_branch } => {
            child(cond, grades, &mut effect, &mut reach)?;
            child(then_branch, grades, &mut effect, &mut reach)?;
            child(else_branch, grades, &mut effect, &mut reach)?;
        }
        Node::Match { scrutinee, arms } => {
            child(scrutinee, grades, &mut effect, &mut reach)?;
            for a in arena.list(arms) {
                child(*a, grades, &mut effect, &mut reach)?;
            }
        }
        Node::Iter { seq, body } => {
            child(seq, grades, &mut effect, &mut reach)?;
            child(body, grades, &mut effect, &mut reach)?;
        }
        Node::Interp { value } => child(value, grades, &mut effect, &mut reach)?,
        Node::Raw { family, payload } => {
            // descend into the family node's payload, joining each child's grade,
            // then join the family operation's own declared effect from the hook.
            for c in arena.list(payload) {
                child(*c, grades, &mut effect, &mut reach)?;
            }
            effect = effect.join(hook.effect_of_raw(family));
        }
        Node::Handle { body, clauses } => {
            // the clause operations are in scope for the BODY, so a `Perform`
            // under it never reaches the effect mask at all.
            let mut inner = *handled;
            for c in arena.clauses(clauses) {
                if !inner.push(c.op, at).0 {
                    return Outcome::Err(CheckError::HandlerDepthExceeded { at });
                }
            }
            let body_grade = match infer(arena, body, res, grades, hook, &inner) {
                Outcome::Ok(g) => g,
                Outcome::Err(e) => return Outcome::Err(e),
            };
            effect = effect.join(body_grade.effect);
            reach = reach.join(body_grade.lease.0);

            // THE ESCAPE CHECK. A `Perform` is not a value, so a direct escape
            // is impossible; the residual case is a lambda whose body performs,
            // returned out of here and called after the handler is gone. The
            // `Lambda` arm promotes that latent effect onto the lambda's reach,
            // so a body whose reach names this handler is exactly that case.
            if body_grade.lease.0.contains(slot_of(at)).0 {
                return Outcome::Err(CheckError::UnplaceableLease { at });
            }

            // a clause body is checked OUTSIDE its own handler, which is what
            // stops a clause from handling the operation it services.
            for c in arena.clauses(clauses) {
                child(c.body, grades, &mut effect, &mut reach)?;
            }
        }
        Node::Perform { op, args } => {
            for a in arena.list(args) {
                child(*a, grades, &mut effect, &mut reach)?;
            }
            // a lexically handled operation is discharged here and does not
            // escape, so it contributes nothing. An unhandled one is a real
            // effect of this program and populates the mask, where the target's
            // `Permits` set is what decides whether it may ship.
            if let Maybe::Isnt = handled.lookup(op) {
                effect.insert(USize(slot_of(at).0));
            }
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
    if !grades.set(at.index(), grade).0 {
        return Outcome::Err(CheckError::GradeRegionFull { at });
    }
    Outcome::Ok(grade)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo::{Bool, Maybe, USize};
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
        let mut pool = [NodeRef::new(USize(0)); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));
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
    fn refuses_a_too_small_grade_region() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize(0)); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));
        let unit = at(b.lit(Literal::Unit, Span::default()));
        let root = at(b.if_(unit, unit, unit, Span::default()));
        let arena = b.into_arena();

        let mut binders = [Maybe::Isnt; 8];
        let mut res = Resolution::new(&mut binders);
        assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

        // a grade region smaller than the node count: recording the If's grade
        // (node index 1) does not land, so check refuses rather than silently
        // passing an ungraded node.
        let mut grade_region = [Grade::default(); 1];
        let mut grades = GradeTable::new(&mut grade_region);
        assert!(matches!(
            check(&arena, root, &res, &mut grades),
            Outcome::Err(CheckError::GradeRegionFull { .. })
        ));
    }

    #[test]
    fn binder_rule_drops_the_bound_slot() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize(0)); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));

        // let x = () in x: the body reaches the binder, and the binder rule
        // drops the binder's own slot, so the Let's reach set is empty.
        let x = str_const!("x").as_sym();
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
        let mut pool = [NodeRef::new(USize(0)); 4];
        let b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));
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

    #[test]
    fn family_check_hook_effect_reaches_the_raw_grade() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize(0)); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));

        // Raw(family, [()]): the family declares an effect through the hook, and
        // the Raw node's grade must carry it on top of the children's.
        let unit = at(b.lit(Literal::Unit, Span::default()));
        let payload = match b.alloc_list(&[unit]) {
            Maybe::Is(l) => l,
            Maybe::Isnt => panic!("pool full"),
        };
        let raw = at(b.raw(FamilyId::default(), payload, Span::default()));
        let arena = b.into_arena();

        struct WithEffect;
        impl FamilyCheck for WithEffect {
            fn effect_of_raw(&self, _family: FamilyId) -> EffectMask {
                let mut e = EffectMask::empty();
                e.insert(USize(0));
                e
            }
        }

        let mut binders = [Maybe::Isnt; 8];
        let mut res = Resolution::new(&mut binders);
        assert!(matches!(resolve_into(&arena, raw, &mut res), Outcome::Ok(())));
        let mut grade_region = [Grade::default(); 8];
        let mut grades = GradeTable::new(&mut grade_region);
        assert!(matches!(
            check_with(&arena, raw, &res, &mut grades, &WithEffect),
            Outcome::Ok(_)
        ));
        // the family's declared effect (slot 0) reaches the Raw node's grade.
        assert!(grade_of(&grades, raw).effect.contains(USize(0)).0);
    }
}

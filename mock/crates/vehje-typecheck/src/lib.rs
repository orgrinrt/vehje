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

use arvo::{Maybe, Outcome};

use arvo::USize;
use vehje_ir::{
    Arena, Assurance, ContainsAll, EffectMask, FamilyId, Grade, GradeTable, Knowledge, Lease, Node,
    NodeRef, ReachMask, TargetSets,
};
use vehje_resolve::{CoreFamilies, Resolution};

mod handlers;
use handlers::{promote_latent, HandlerScope};

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
mod tests;

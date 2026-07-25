//! Tests for the graded check: the effect scope chain, the escape refusal, and
//! the region-exhaustion refusals.
//!
//! A sibling file rather than an inline module, so the crate root stays inside
//! the file-size gate.

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

/// A minted operation identity, distinct from any source name by kind.
fn op(id: u32) -> hilavitkutin_sym::Sym { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test operation id; tracked: #207
    hilavitkutin_sym::Sym::new(
        hilavitkutin_sym::SymKind::from_raw(0b010),
        arvo::Bits::from_raw(id),
    )
}

#[test]
fn a_handled_operation_never_reaches_the_effect_mask() {
    // handle (perform op) with op -> unit. The operation is discharged
    // lexically, so it is not an effect of this program at all: it never
    // enters the mask rather than being subtracted from it afterwards.
    let mut nodes = [Node::Lit(Literal::Unit); 16];
    let mut spans = [Span::default(); 16];
    let mut pool = [NodeRef::new(USize(0)); 16];
    let mut clauses = [vehje_ir::Clause {
        op: op(1),
        resume: op(0),
        arity: arvo::Uint::<8, arvo::strategy::Hot>::from_raw(0), // lint:allow(no-bare-numeric) reason: nullary test operation; tracked: #207
        body: NodeRef::new(USize(0)),
        span: Span::default(),
    }; 4];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut clauses));
    let unit = at(b.lit(Literal::Unit, Span::default()));
    let empty = match b.alloc_list(&[]) {
        Maybe::Is(l) => l,
        Maybe::Isnt => panic!("pool full"),
    };
    let p = at(b.perform(op(1), empty, Span::default()));
    let cl = match b.alloc_clauses(&[vehje_ir::Clause {
        op: op(1),
        resume: op(0),
        arity: arvo::Uint::<8, arvo::strategy::Hot>::from_raw(0), // lint:allow(no-bare-numeric) reason: nullary test operation; tracked: #207
        body: unit,
        span: Span::default(),
    }]) {
        Maybe::Is(l) => l,
        Maybe::Isnt => panic!("clause region full"),
    };
    let root = at(b.handle(p, cl, Span::default()));
    let arena = b.into_arena();

    let mut binders = [Maybe::Isnt; 16];
    let mut res = Resolution::new(&mut binders);
    assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

    let mut grade_region = [Grade::default(); 16];
    let mut grades = GradeTable::new(&mut grade_region);
    assert!(matches!(check(&arena, root, &res, &mut grades), Outcome::Ok(_)));
    assert_eq!(grade_of(&grades, root).effect, EffectMask::empty());
}

#[test]
fn an_unhandled_operation_is_an_effect_of_the_program() {
    // The complement of the test above: with no handler in scope, the
    // operation escapes, so it populates the mask and the target's
    // `Permits` set is what decides whether it may ship.
    let mut nodes = [Node::Lit(Literal::Unit); 16];
    let mut spans = [Span::default(); 16];
    let mut pool = [NodeRef::new(USize(0)); 16];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut []));
    let empty = match b.alloc_list(&[]) {
        Maybe::Is(l) => l,
        Maybe::Isnt => panic!("pool full"),
    };
    let root = at(b.perform(op(1), empty, Span::default()));
    let arena = b.into_arena();

    let mut binders = [Maybe::Isnt; 16];
    let mut res = Resolution::new(&mut binders);
    assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

    let mut grade_region = [Grade::default(); 16];
    let mut grades = GradeTable::new(&mut grade_region);
    assert!(matches!(check(&arena, root, &res, &mut grades), Outcome::Ok(_)));
    assert_ne!(grade_of(&grades, root).effect, EffectMask::empty());
}

#[test]
fn an_operation_escaping_inside_a_returned_lambda_is_refused() {
    // handle (\_. perform op) with op -> unit.
    //
    // The handled computation RETURNS a lambda whose body performs the
    // handled operation. Nothing performs it while the handler is live, so
    // the effect axis alone sees nothing wrong; the lambda VALUE, however,
    // can perform it after the handler is gone. The promotion at `Lambda`
    // puts the handler's slot on that value's reach and the check at
    // `Handle` refuses a result that names its own slot.
    //
    // This is the first genuine effect-and-lease interaction rule in the
    // system, which is why it gets a test of its own.
    let mut nodes = [Node::Lit(Literal::Unit); 16];
    let mut spans = [Span::default(); 16];
    let mut pool = [NodeRef::new(USize(0)); 16];
    let mut clause_region = [vehje_ir::Clause {
        op: op(1),
        resume: op(0),
        arity: arvo::Uint::<8, arvo::strategy::Hot>::from_raw(0), // lint:allow(no-bare-numeric) reason: nullary test operation; tracked: #207
        body: NodeRef::new(USize(0)),
        span: Span::default(),
    }; 4];
    let mut b =
        Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool, &mut clause_region));
    let unit = at(b.lit(Literal::Unit, Span::default()));
    let empty = match b.alloc_list(&[]) {
        Maybe::Is(l) => l,
        Maybe::Isnt => panic!("pool full"),
    };
    let p = at(b.perform(op(1), empty, Span::default()));
    let lam = at(b.lambda(str_const!("x").as_sym(), p, Span::default()));
    let cl = match b.alloc_clauses(&[vehje_ir::Clause {
        op: op(1),
        resume: op(0),
        arity: arvo::Uint::<8, arvo::strategy::Hot>::from_raw(0), // lint:allow(no-bare-numeric) reason: nullary test operation; tracked: #207
        body: unit,
        span: Span::default(),
    }]) {
        Maybe::Is(l) => l,
        Maybe::Isnt => panic!("clause region full"),
    };
    let root = at(b.handle(lam, cl, Span::default()));
    let arena = b.into_arena();

    let mut binders = [Maybe::Isnt; 16];
    let mut res = Resolution::new(&mut binders);
    assert!(matches!(resolve_into(&arena, root, &mut res), Outcome::Ok(())));

    let mut grade_region = [Grade::default(); 16];
    let mut grades = GradeTable::new(&mut grade_region);
    assert!(matches!(
        check(&arena, root, &res, &mut grades),
        Outcome::Err(CheckError::UnplaceableLease { .. })
    ));
}

#[test]
#[ignore = "catalogue: a clause whose body references its resumption must be a named refusal until the bounded-multi-shot budget machinery lands; the resumption binder is reserved and unbound today, so nothing refuses it yet; tracked #45"]
fn a_clause_referencing_its_resumption_is_refused() {
    // A green test over a handler that cannot actually resume would be a lie
    // about the cost, and the cost is the whole promise. So this asserts the
    // intended behaviour and stays catalogued red until CR1 lands.
    panic!("the resumption refusal is not built");
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

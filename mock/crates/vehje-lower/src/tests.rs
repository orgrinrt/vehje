//! Tests for the vehje-lower lowering pass.

use super::*;
use arvo::strategy::Hot;
use arvo::{Bool, Identity, Int, USize};
use hilavitkutin_str::str_const;
use vehje_ir::{Builder, Span};

fn at(m: Maybe<NodeRef>) -> NodeRef {
    match m {
        Maybe::Is(r) => r,
        Maybe::Isnt => panic!("arena full"),
    }
}

fn int(b: &mut Builder<'_>, v: Int<64, Hot>) -> NodeRef {
    at(b.lit(Literal::Int(v), Span::default()))
}

#[test]
fn const_fold_folds_if_true_to_the_taken_branch() {
    let mut nodes = [Node::Lit(Literal::Unit); 8];
    let mut spans = [Span::default(); 8];
    let mut pool = [NodeRef::new(USize::ZERO); 8];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

    // if true then 1 else 2: distinct branches make the fold observable.
    let cond = at(b.lit(Literal::Bool(Bool::TRUE), Span::default()));
    let a = int(&mut b, Int::<64, Hot>::from_raw(1));
    let bb = int(&mut b, Int::<64, Hot>::from_raw(2));
    let iff = at(b.if_(cond, a, bb, Span::default()));
    let arena = b.into_arena();

    let mut remap = [Maybe::Isnt; 8];
    let mut rw = Rewrite::new(&mut remap);
    ConstFold.apply(&arena, iff, &mut rw);
    assert_eq!(rw.resolve(iff), a);
}

#[test]
fn const_fold_folds_if_false_to_the_else_branch() {
    let mut nodes = [Node::Lit(Literal::Unit); 8];
    let mut spans = [Span::default(); 8];
    let mut pool = [NodeRef::new(USize::ZERO); 8];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

    let cond = at(b.lit(Literal::Bool(Bool::FALSE), Span::default()));
    let a = int(&mut b, Int::<64, Hot>::from_raw(1));
    let bb = int(&mut b, Int::<64, Hot>::from_raw(2));
    let iff = at(b.if_(cond, a, bb, Span::default()));
    let arena = b.into_arena();

    let mut remap = [Maybe::Isnt; 8];
    let mut rw = Rewrite::new(&mut remap);
    ConstFold.apply(&arena, iff, &mut rw);
    assert_eq!(rw.resolve(iff), bb);
}

#[test]
fn cse_does_not_share_variables_avoiding_capture() {
    let mut nodes = [Node::Lit(Literal::Unit); 8];
    let mut spans = [Span::default(); 8];
    let mut pool = [NodeRef::new(USize::ZERO); 8];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

    // two Var("x")s that (in a real program) could resolve to different
    // binders. CSE must NOT merge them, or it captures. Only variable-free
    // subtrees are shared, so a Var is never keyed.
    let x = str_const!("x").as_sym();
    let cond = at(b.lit(Literal::Unit, Span::default()));
    let v1 = at(b.var(x, Span::default()));
    let v2 = at(b.var(x, Span::default()));
    let parent = at(b.if_(cond, v1, v2, Span::default()));
    let arena = b.into_arena();

    let mut remap = [Maybe::Isnt; 8];
    let mut rw = Rewrite::new(&mut remap);
    let mut table = [Maybe::Isnt; 8];
    Cse.apply(&arena, parent, &mut rw, &mut table);
    // neither Var is redirected to the other: no capture.
    assert_eq!(rw.resolve(v1), v1);
    assert_eq!(rw.resolve(v2), v2);
}

#[test]
fn cse_shares_two_equal_subtrees() {
    let mut nodes = [Node::Lit(Literal::Unit); 8];
    let mut spans = [Span::default(); 8];
    let mut pool = [NodeRef::new(USize::ZERO); 8];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

    // two structurally equal Int(7)s under one parent: CSE redirects the
    // later to the earlier.
    let first = int(&mut b, Int::<64, Hot>::from_raw(7));
    let second = int(&mut b, Int::<64, Hot>::from_raw(7));
    let parent = at(b.if_(first, second, first, Span::default()));
    let arena = b.into_arena();

    let mut remap = [Maybe::Isnt; 8];
    let mut rw = Rewrite::new(&mut remap);
    let mut table = [Maybe::Isnt; 8];
    Cse.apply(&arena, parent, &mut rw, &mut table);
    assert_eq!(rw.resolve(second), first);
}

#[test]
fn cheap_lowering_preserves_a_trivial_program() {
    let mut nodes = [Node::Lit(Literal::Unit); 8];
    let mut spans = [Span::default(); 8];
    let mut pool = [NodeRef::new(USize::ZERO); 8];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let unit = at(b.lit(Literal::Unit, Span::default()));
    let arena = b.into_arena();

    let mut remap = [Maybe::Isnt; 8];
    let mut rw = Rewrite::new(&mut remap);
    let mut table = [Maybe::Isnt; 8];
    let out = Lower::cheap().lower(LowerStrategy::Cheap, &arena, unit, &mut rw, &mut table);
    assert_eq!(out, unit);
}


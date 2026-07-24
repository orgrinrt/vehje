//! Tests for the vehje-lower lowering pass.

use super::*;
use arvo::strategy::Hot;
use arvo::{Bool, Identity, Int, USize};
use hilavitkutin_str::str_const;
use hilavitkutin_sym::{Domain, Generator, Sym};
use vehje_ir::{BinderDomain, Builder, Span};
use crate::anf::ARG_CAP;

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

    let mut remap = [Maybe::Isnt; 8];
    let mut rw = Rewrite::new(&mut remap);
    let mut table = [Maybe::Isnt; 8];
    let mut binders = Generator::<BinderDomain>::new();
    let out = Lower::cheap().lower(LowerStrategy::Cheap, &mut b, unit, &mut rw, &mut table, &mut binders);
    // a trivial program is a single atom: ANF appends nothing and returns it.
    assert_eq!(out, Maybe::Is(unit));
}


// ── ANF catamorphism tests ───────────────────────────────────────────────

fn minted_kind() -> hilavitkutin_sym::SymKind {
    <BinderDomain as Domain>::KIND
}

#[test]
fn anf_leaves_atoms_untouched() {
    let mut nodes = [Node::Lit(Literal::Unit); 16];
    let mut spans = [Span::default(); 16];
    let mut pool = [NodeRef::new(USize::ZERO); 16];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let before = b.arena().len();
    let mut remap = [Maybe::Isnt; 16];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    let out = Anf.apply(&mut b, x, &rw, &mut binders);
    // an atom normalises to itself with no appended nodes.
    assert_eq!(out, Maybe::Is(x));
    assert_eq!(b.arena().len().0, before.0);
}

#[test]
fn anf_hoists_a_compound_apply_arg() {
    let mut nodes = [Node::Lit(Literal::Unit); 32];
    let mut spans = [Span::default(); 32];
    let mut pool = [NodeRef::new(USize::ZERO); 32];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let f = at(b.var(str_const!("f").as_sym(), Span::default()));
    let g = at(b.var(str_const!("g").as_sym(), Span::default()));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let gx = at(b.apply(g, &[x], Span::default()));
    let fgx = at(b.apply(f, &[gx], Span::default()));
    let mut remap = [Maybe::Isnt; 32];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    let out = at(Anf.apply(&mut b, fgx, &rw, &mut binders));
    // f(g(x)) normalises to `let t = g(x) in f(t)`.
    match b.arena().get(out) {
        Node::Let { name, value, body, .. } => {
            // the binder is minted (BinderDomain, kind 0b001), never a source name.
            assert_eq!(name.kind(), minted_kind());
            // its value is the g(x) application.
            assert!(matches!(b.arena().get(value), Node::Apply { .. }));
            // the body applies f to the fresh binder.
            match b.arena().get(body) {
                Node::Apply { args, .. } => {
                    let a0 = b.arena().list(args)[0];
                    assert_eq!(b.arena().get(a0), Node::Var(name));
                }
                other => panic!("body is not an apply: {other:?}"),
            }
        }
        other => panic!("root is not a let: {other:?}"),
    }
}

#[test]
fn anf_flattens_nested_compounds() {
    // f(g(h(x))) must flatten to a straight-line let-sequence: no let-bound
    // value is itself a let.
    let mut nodes = [Node::Lit(Literal::Unit); 48];
    let mut spans = [Span::default(); 48];
    let mut pool = [NodeRef::new(USize::ZERO); 48];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let f = at(b.var(str_const!("f").as_sym(), Span::default()));
    let g = at(b.var(str_const!("g").as_sym(), Span::default()));
    let h = at(b.var(str_const!("h").as_sym(), Span::default()));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let hx = at(b.apply(h, &[x], Span::default()));
    let ghx = at(b.apply(g, &[hx], Span::default()));
    let fghx = at(b.apply(f, &[ghx], Span::default()));
    let mut remap = [Maybe::Isnt; 48];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    let out = at(Anf.apply(&mut b, fghx, &rw, &mut binders));
    // walk the let-chain: every let-bound value must be an apply (never a let).
    let mut cur = out;
    let mut lets = 0;
    loop {
        match b.arena().get(cur) {
            Node::Let { value, body, .. } => {
                assert!(
                    matches!(b.arena().get(value), Node::Apply { .. }),
                    "a let-bound value is not flat (it is itself compound-nested)"
                );
                lets += 1;
                cur = body;
            }
            _ => break,
        }
    }
    // two hoists: h(x) and g(t_h); f(t_g) is the tail.
    assert_eq!(lets, 2);
}

#[test]
fn anf_flattens_let_chains() {
    // an input `let a = () in let b = () in a` stays a flat chain.
    let mut nodes = [Node::Lit(Literal::Unit); 32];
    let mut spans = [Span::default(); 32];
    let mut pool = [NodeRef::new(USize::ZERO); 32];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let a_name = str_const!("a").as_sym();
    let b_name = str_const!("b").as_sym();
    let u1 = at(b.lit(Literal::Unit, Span::default()));
    let a_ref = at(b.var(a_name, Span::default()));
    let u2 = at(b.lit(Literal::Unit, Span::default()));
    let inner = at(b.let_(Bool::FALSE, b_name, u2, a_ref, Span::default()));
    let outer = at(b.let_(Bool::FALSE, a_name, u1, inner, Span::default()));
    let mut remap = [Maybe::Isnt; 32];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    let out = at(Anf.apply(&mut b, outer, &rw, &mut binders));
    // the root is a let whose body is a let (the flat chain), source binders kept.
    match b.arena().get(out) {
        Node::Let { name, body, .. } => {
            assert_eq!(name, a_name);
            assert!(matches!(b.arena().get(body), Node::Let { .. }));
        }
        other => panic!("root is not a let: {other:?}"),
    }
}

#[test]
fn anf_branches_are_separate_contexts() {
    // `if c then g(x) else y`: the hoist of g(x) stays inside the then-branch,
    // it is not lifted above the If.
    let mut nodes = [Node::Lit(Literal::Unit); 48];
    let mut spans = [Span::default(); 48];
    let mut pool = [NodeRef::new(USize::ZERO); 48];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let c = at(b.var(str_const!("c").as_sym(), Span::default()));
    let f = at(b.var(str_const!("f").as_sym(), Span::default()));
    let g = at(b.var(str_const!("g").as_sym(), Span::default()));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let y = at(b.var(str_const!("y").as_sym(), Span::default()));
    let gx = at(b.apply(g, &[x], Span::default()));
    let fgx = at(b.apply(f, &[gx], Span::default()));
    let iff = at(b.if_(c, fgx, y, Span::default()));
    let mut remap = [Maybe::Isnt; 48];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    let out = at(Anf.apply(&mut b, iff, &rw, &mut binders));
    // the root stays an If (the branch hoist did not escape upward), and the
    // then-branch is the hoisted `let t = g(x) in f(t)`.
    match b.arena().get(out) {
        Node::If { then_branch, .. } => {
            assert!(matches!(b.arena().get(then_branch), Node::Let { .. }));
        }
        other => panic!("root is not an if: {other:?}"),
    }
}

#[test]
fn anf_minted_binder_cannot_capture() {
    // the fresh binder ANF mints has the BinderDomain tag (0b001), disjoint from
    // any source name (0b000), so a hoisted binding can never capture.
    let mut nodes = [Node::Lit(Literal::Unit); 32];
    let mut spans = [Span::default(); 32];
    let mut pool = [NodeRef::new(USize::ZERO); 32];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let f = at(b.var(str_const!("f").as_sym(), Span::default()));
    let g = at(b.var(str_const!("g").as_sym(), Span::default()));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let gx = at(b.apply(g, &[x], Span::default()));
    let fgx = at(b.apply(f, &[gx], Span::default()));
    let mut remap = [Maybe::Isnt; 32];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    let out = at(Anf.apply(&mut b, fgx, &rw, &mut binders));
    match b.arena().get(out) {
        Node::Let { name, .. } => {
            let source: Sym = str_const!("x").as_sym();
            assert_ne!(name.kind(), source.kind());
            assert_eq!(name.kind(), minted_kind());
        }
        other => panic!("root is not a let: {other:?}"),
    }
}

#[test]
fn anf_returns_isnt_on_arena_full() {
    // an arena sized exactly to the input has no room for the ANF appends.
    let mut nodes = [Node::Lit(Literal::Unit); 5];
    let mut spans = [Span::default(); 5];
    let mut pool = [NodeRef::new(USize::ZERO); 5];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let f = at(b.var(str_const!("f").as_sym(), Span::default()));
    let g = at(b.var(str_const!("g").as_sym(), Span::default()));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let gx = at(b.apply(g, &[x], Span::default()));
    let fgx = at(b.apply(f, &[gx], Span::default()));
    let mut remap = [Maybe::Isnt; 5];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    // the arena is full (5 nodes used, 5 cap), so the first append fails.
    assert_eq!(Anf.apply(&mut b, fgx, &rw, &mut binders), Maybe::Isnt);
}

#[test]
fn anf_returns_isnt_on_scratch_overflow() {
    // an apply whose arity exceeds ARG_CAP overflows the operand scratch.
    const N: usize = ARG_CAP + 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
    let mut nodes = [Node::Lit(Literal::Unit); 256];
    let mut spans = [Span::default(); 256];
    let mut pool = [NodeRef::new(USize::ZERO); 256];
    let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));
    let f = at(b.var(str_const!("f").as_sym(), Span::default()));
    let g = at(b.var(str_const!("g").as_sym(), Span::default()));
    let x = at(b.var(str_const!("x").as_sym(), Span::default()));
    let gx = at(b.apply(g, &[x], Span::default()));
    let args = [gx; N];
    let big = at(b.apply(f, &args, Span::default()));
    let mut remap = [Maybe::Isnt; 256];
    let rw = Rewrite::new(&mut remap);
    let mut binders = Generator::<BinderDomain>::new();
    assert_eq!(Anf.apply(&mut b, big, &rw, &mut binders), Maybe::Isnt);
}

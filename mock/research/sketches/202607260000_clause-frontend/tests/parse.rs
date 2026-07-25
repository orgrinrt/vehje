//! Parser tests: the productions that carry the grammar's disambiguations.

use arvo::USize;
use clause_frontend::ast::{Arena, AstRef, BinOp, Node, UnOp};
use clause_frontend::{lex, Token, TokenKind};
use notko::{Maybe, Outcome};

const N: usize = 512;

struct Fixture {
    nodes: [Node; N],
    spans: [clause_frontend::Span; N],
    pool: [AstRef; N],
    toks: [Token; N],
}

impl Fixture {
    fn new() -> Self {
        Self {
            nodes: [Node::Continue; N],
            spans: [Default::default(); N],
            pool: [AstRef(USize(0)); N],
            toks: [Token::new(TokenKind::Eof, Default::default()); N],
        }
    }
}

/// Parse `src` as an expression, returning the arena and the root.
fn expr<'f>(f: &'f mut Fixture, src: &str) -> (Arena<'f>, AstRef) {
    let n = match lex(src, &mut f.toks) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("lex: {e:?}"),
    };
    let arena = Arena::new(&mut f.nodes, &mut f.spans, &mut f.pool);
    let mut p = clause_frontend::Parser::new(&f.toks[..n], src, arena);
    let root = match p.parse_expr() {
        Outcome::Ok(r) => r,
        Outcome::Err(e) => panic!("parse: {e:?}"),
    };
    (p.into_arena(), root)
}

#[test]
fn the_operator_ladder_binds_by_precedence() {
    // 1 + 2 * 3 must group as 1 + (2 * 3)
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "1 + 2 * 3");
    let Node::Binary { op, rhs, .. } = a.get(root) else { panic!("not binary") };
    assert_eq!(op, BinOp::Add);
    let Node::Binary { op: inner, .. } = a.get(rhs) else { panic!("rhs not binary") };
    assert_eq!(inner, BinOp::Mul);
}

#[test]
fn comparison_binds_looser_than_arithmetic_and_tighter_than_logic() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "a + 1 < b && c");
    let Node::Binary { op, lhs, .. } = a.get(root) else { panic!() };
    assert_eq!(op, BinOp::And);
    let Node::Binary { op: cmp, lhs: sum, .. } = a.get(lhs) else { panic!() };
    assert_eq!(cmp, BinOp::Lt);
    let Node::Binary { op: add, .. } = a.get(sum) else { panic!() };
    assert_eq!(add, BinOp::Add);
}

#[test]
fn the_postfix_chain_composes_field_call_and_index() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "a.b(c)[0]?");
    let Node::Question { value } = a.get(root) else { panic!("not ?") };
    let Node::Index { base, .. } = a.get(value) else { panic!("not index") };
    let Node::MethodCall { base: recv, .. } = a.get(base) else { panic!("not method call") };
    assert!(matches!(a.get(recv), Node::ExprPath { .. }));
}

#[test]
fn a_struct_literal_is_refused_in_an_if_head_and_allowed_inside_the_block() {
    // the grammar's sharpest ambiguity: `if Name { }` is a path plus a block
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "if Name { Other { x: 1 } }");
    let Node::If { cond, then_block, .. } = a.get(root) else { panic!("not if") };
    assert!(matches!(a.get(cond), Node::ExprPath { .. }), "the head must not be a struct literal");
    let Node::Block { tail, .. } = a.get(then_block) else { panic!("not block") };
    let Maybe::Is(t) = tail else { panic!("block has no tail") };
    assert!(matches!(a.get(t), Node::StructLit { .. }), "inside the block it is a struct literal");
}

#[test]
fn a_parenthesised_expression_is_peeled_but_a_trailing_comma_makes_a_tuple() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "(1)");
    assert!(matches!(a.get(root), Node::Lit { .. }), "(1) is 1, not a one-tuple");

    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "(1,)");
    let Node::ExprTuple { elems } = a.get(root) else { panic!("not a tuple") };
    assert_eq!(a.list(elems).len(), 1);

    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "()");
    let Node::ExprTuple { elems } = a.get(root) else { panic!("not unit") };
    assert_eq!(a.list(elems).len(), 0);
}

#[test]
fn a_match_parses_its_arms_with_guards() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "match x { 0 => a, n if n > 1 => b, _ => c }");
    let Node::Match { arms, .. } = a.get(root) else { panic!("not match") };
    let arms = a.list(arms);
    assert_eq!(arms.len(), 3);
    let Node::Arm { guard, .. } = a.get(arms[1]) else { panic!() };
    assert!(matches!(guard, Maybe::Is(_)), "the second arm has a guard");
    let Node::Arm { pat, .. } = a.get(arms[2]) else { panic!() };
    assert!(matches!(a.get(pat), Node::PatWild), "the third arm is the wildcard");
}

#[test]
fn an_or_pattern_collects_its_alternatives() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "match x { 1 | 2 | 3 => y }");
    let Node::Match { arms, .. } = a.get(root) else { panic!() };
    let Node::Arm { pat, .. } = a.get(a.list(arms)[0]) else { panic!() };
    let Node::PatOr { alts } = a.get(pat) else { panic!("not an or-pattern") };
    assert_eq!(a.list(alts).len(), 3);
}

#[test]
fn a_closure_takes_parameters_and_a_body() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "move |x, y| x + y");
    let Node::Closure { moved, params, .. } = a.get(root) else { panic!("not a closure") };
    assert!(moved);
    assert_eq!(a.list(params).len(), 2);

    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "|| 1");
    let Node::Closure { params, .. } = a.get(root) else { panic!() };
    assert_eq!(a.list(params).len(), 0, "|| is the empty parameter list, not two pipes");
}

#[test]
fn a_block_distinguishes_a_tail_expression_from_a_statement() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "{ let x = 1; f(x); x }");
    let Node::Block { stmts, tail } = a.get(root) else { panic!("not a block") };
    assert_eq!(a.list(stmts).len(), 2);
    assert!(matches!(tail, Maybe::Is(_)), "the trailing `x` is the block's value");

    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "{ f(); }");
    let Node::Block { tail, .. } = a.get(root) else { panic!() };
    assert!(matches!(tail, Maybe::Isnt), "a trailing semicolon leaves no value");
}

#[test]
fn a_turbofish_carries_generic_arguments_in_expression_position() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "collect::<Vec>()");
    let Node::Call { callee, .. } = a.get(root) else { panic!("not a call") };
    let Node::ExprPath { path } = a.get(callee) else { panic!() };
    let Node::Path { generics, .. } = a.get(path) else { panic!() };
    assert_eq!(a.list(generics).len(), 1, "::<Vec> is one generic argument");
}

#[test]
fn a_range_may_be_open_at_either_end() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "1..=9");
    let Node::Range { inclusive, lo, hi } = a.get(root) else { panic!() };
    assert!(inclusive);
    assert!(matches!(lo, Maybe::Is(_)) && matches!(hi, Maybe::Is(_)));

    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "..n");
    let Node::Range { lo, hi, .. } = a.get(root) else { panic!() };
    assert!(matches!(lo, Maybe::Isnt) && matches!(hi, Maybe::Is(_)));
}

#[test]
fn a_compound_assignment_records_its_operator() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "x += 1");
    let Node::Assign { op, .. } = a.get(root) else { panic!("not an assignment") };
    assert!(matches!(op, Maybe::Is(BinOp::Add)));
}

#[test]
fn a_reference_operator_records_its_mutability() {
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "&mut x");
    let Node::Unary { op, .. } = a.get(root) else { panic!() };
    assert_eq!(op, UnOp::RefMut);
}

#[test]
fn a_macro_call_keeps_its_body_as_an_unparsed_span() {
    // a macro's tokens mean nothing until expansion, so the parser balances the
    // delimiters and records the extent rather than building a throwaway tree
    let mut f = Fixture::new();
    let (a, root) = expr(&mut f, "quote!( a { b } c )");
    assert!(matches!(a.get(root), Node::MacroCall { .. }));
}

#[test]
fn a_reserved_but_unproductive_word_is_refused_by_name() {
    let src = "dyn";
    let mut f = Fixture::new();
    let n = match lex(src, &mut f.toks) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("{e:?}"),
    };
    let arena = Arena::new(&mut f.nodes, &mut f.spans, &mut f.pool);
    let mut p = clause_frontend::Parser::new(&f.toks[..n], src, arena);
    assert!(matches!(
        p.parse_expr(),
        Outcome::Err(clause_frontend::ParseError::ReservedWord { .. })
    ));
}

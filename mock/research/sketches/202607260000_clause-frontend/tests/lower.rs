//! Lowering tests: a parsed Clause program becomes a Core term.

use arvo::USize;
use clause_frontend::ast::{Arena as AstArena, AstRef, Node as AstNode};
use clause_frontend::lower::{Lower, LowerError};
use clause_frontend::{lex, Token, TokenKind};
use hilavitkutin_str::{ArenaInterner, StringInterner};
use notko::Outcome;
use vehje_ir::{Arena as IrArena, Builder, Clause, Literal, Node, NodeRef, Span as IrSpan};

const N: usize = 4096;

/// A test interner over fixed storage: intern by first occurrence, resolve by
/// index. Enough for a test, and it keeps the front end's no-alloc discipline.
struct TestInterner {
    buf: core::cell::RefCell<([&'static str; 256], usize)>,
}

impl TestInterner {
    fn new() -> Self {
        Self { buf: core::cell::RefCell::new(([""; 256], 0)) }
    }
}

impl ArenaInterner for TestInterner {
    fn arena_intern(&self, s: &str) -> u32 {
        let mut b = self.buf.borrow_mut();
        let n = b.1;
        for i in 0..n {
            if b.0[i] == s {
                return i as u32;
            }
        }
        // leaked so the resolve borrow can outlive this call; a test may leak
        let leaked: &'static str = Box::leak(s.to_string().into_boxed_str());
        b.0[n] = leaked;
        b.1 = n + 1;
        n as u32
    }

    fn arena_resolve(&self, id: u32) -> &str {
        let b = self.buf.borrow();
        b.0[id as usize]
    }
}

/// A placeholder `Sym` for the clause scratch, never read.
fn zero_sym() -> hilavitkutin_sym::Sym {
    hilavitkutin_sym::Sym::new(hilavitkutin_sym::SymKind::from_raw(0b111), arvo::Bits::from_raw(0))
}

struct Fx {
    an: [AstNode; N],
    asp: [clause_frontend::Span; N],
    ap: [AstRef; N],
    toks: [Token; N],
    inodes: [Node; N],
    ispans: [IrSpan; N],
    ipool: [NodeRef; N],
    iclauses: [Clause; 8],
}

impl Fx {
    fn new() -> Self {
        Self {
            an: [AstNode::Continue; N],
            asp: [Default::default(); N],
            ap: [AstRef(USize(0)); N],
            toks: [Token::new(TokenKind::Eof, Default::default()); N],
            inodes: [Node::Lit(Literal::Unit); N],
            ispans: [IrSpan::default(); N],
            ipool: [NodeRef::new(USize(0)); N],
            iclauses: [Clause {
                op: zero_sym(),
                resume: zero_sym(),
                arity: arvo::Uint::<8, arvo::strategy::Hot>::from_raw(0),
                body: NodeRef::new(USize(0)),
                span: IrSpan::default(),
            }; 8],
        }
    }
}

/// Parse and lower `src`, returning the IR arena and the root.
fn lower<'f>(f: &'f mut Fx, src: &str, entry: &str) -> Result<(IrArena<'f>, NodeRef), LowerError> {
    let n = match lex(src, &mut f.toks) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("lex: {e:?}"),
    };
    let ast_arena = AstArena::new(&mut f.an, &mut f.asp, &mut f.ap);
    let mut p = clause_frontend::Parser::new(&f.toks[..n], src, ast_arena);
    let root = match p.parse_file() {
        Outcome::Ok(r) => r,
        Outcome::Err(e) => panic!("parse: {e:?}"),
    };
    let ast = p.into_arena();

    let ir = IrArena::new(&mut f.inodes, &mut f.ispans, &mut f.ipool, &mut f.iclauses);
    let mut b = Builder::new(ir);
    let mut interner = StringInterner::new(TestInterner::new());
    let out = {
        let mut lo = Lower { ast: &ast, src, b: &mut b, interner: &mut interner };
        lo.file(root, entry)
    };
    match out {
        Outcome::Ok(r) => Ok((b.into_arena(), r)),
        Outcome::Err(e) => Err(e),
    }
}

/// Count nodes of a shape in the whole arena.
fn count(a: &IrArena<'_>, pred: impl Fn(Node) -> bool) -> usize {
    (0..a.len().0).filter(|i| pred(a.get(NodeRef::new(USize(*i))))).count()
}

#[test]
fn a_function_becomes_a_recursive_binding_of_a_curried_lambda() {
    let mut f = Fx::new();
    let (a, root) = lower(&mut f, "fn add(x, y) { x + y }", "add").expect("lowers");

    // the entry call is the program's value
    assert!(matches!(a.get(root), Node::Let { rec, .. } if rec.0), "a fn binds recursively");

    // two parameters become two nested lambdas, because Core application takes
    // one argument at a time
    assert_eq!(count(&a, |n| matches!(n, Node::Lambda { .. })), 2);
    // `+` is a family operation, because the Core has no addition
    assert_eq!(count(&a, |n| matches!(n, Node::Raw { .. })), 1);
}

#[test]
fn a_block_becomes_nested_bindings_with_the_tail_as_the_value() {
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn main() { let x = 1; let y = 2; x + y }", "main").expect("lowers");
    // two `let`s in the block plus the recursive binding of `main`
    assert_eq!(count(&a, |n| matches!(n, Node::Let { .. })), 3);
}

#[test]
fn a_discarded_statement_still_binds_so_effect_order_stays_explicit() {
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn main() { f(1); 2 }", "main").expect("lowers");
    // the discarded call binds to a fresh name rather than vanishing
    assert!(count(&a, |n| matches!(n, Node::Let { .. })) >= 2);
}

#[test]
fn an_if_without_an_else_gets_a_unit_arm_because_if_is_total() {
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn main() { if c { 1 } }", "main").expect("lowers");
    assert_eq!(count(&a, |n| matches!(n, Node::If { .. })), 1);
    assert!(count(&a, |n| matches!(n, Node::Lit(Literal::Unit))) >= 1);
}

#[test]
fn every_operator_reaches_the_core_as_a_family_operation() {
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn main() { 1 + 2 * 3 < 4 }", "main").expect("lowers");
    // three operators, three family nodes, zero Core arithmetic
    assert_eq!(count(&a, |n| matches!(n, Node::Raw { .. })), 3);
}

#[test]
fn a_struct_literal_becomes_a_family_operation_over_alternating_keys_and_values() {
    // content-as-values: the Core is the eliminator algebra, so construction
    // belongs to a family
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn main() { P { x: 1, y: 2 } }", "main").expect("lowers");
    assert_eq!(count(&a, |n| matches!(n, Node::Raw { .. })), 1);
    // two string keys plus the op code and the two values
    assert!(count(&a, |n| matches!(n, Node::Lit(Literal::Str(_)))) >= 2);
}

#[test]
fn a_field_access_becomes_project() {
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn main() { p.x }", "main").expect("lowers");
    assert_eq!(count(&a, |n| matches!(n, Node::Project { .. })), 1);
}

#[test]
fn declaration_only_items_erase_rather_than_lowering() {
    // a struct, a trait, and a use inform the checker and the Core never sees
    // them; the program is still just the entry binding
    let src = "
        struct S { a: u32 }
        trait T { type Item; }
        use a::b;
        fn main() { 1 }
    ";
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, src, "main").expect("lowers");
    assert_eq!(count(&a, |n| matches!(n, Node::Let { .. })), 1, "only `main` binds");
}

#[test]
fn a_signature_only_function_binds_nothing() {
    let mut f = Fx::new();
    let (a, _) = lower(&mut f, "fn declared() -> u32; fn main() { 1 }", "main").expect("lowers");
    assert_eq!(count(&a, |n| matches!(n, Node::Let { .. })), 1);
}

#[test]
fn a_surface_form_with_no_lowering_yet_is_named_not_silently_dropped() {
    let mut f = Fx::new();
    let Err(e) = lower(&mut f, "fn main() { loop { 1 } }", "main") else { panic!("should refuse") };
    assert!(matches!(e, LowerError::Unsupported { what: "loop" }));
}

#[test]
fn a_multi_segment_path_is_refused_until_a_resolve_pass_exists() {
    // refusing beats guessing which segment wins
    let mut f = Fx::new();
    let Err(e) = lower(&mut f, "fn main() { a::b }", "main") else { panic!("should refuse") };
    assert!(matches!(e, LowerError::UnresolvedPath));
}

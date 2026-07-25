//! Item tests, centred on the features the language is FOR: generics, traits,
//! associated types, and the bounds machinery that carries them.

use arvo::USize;
use clause_frontend::ast::{Arena, AstRef, Node};
use clause_frontend::{lex, Token, TokenKind};
use notko::{Maybe, Outcome};

const N: usize = 2048;

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

fn file<'f>(f: &'f mut Fixture, src: &str) -> (Arena<'f>, AstRef) {
    let n = match lex(src, &mut f.toks) {
        Outcome::Ok(n) => n.0,
        Outcome::Err(e) => panic!("lex: {e:?}"),
    };
    let arena = Arena::new(&mut f.nodes, &mut f.spans, &mut f.pool);
    let mut p = clause_frontend::Parser::new(&f.toks[..n], src, arena);
    let root = match p.parse_file() {
        Outcome::Ok(r) => r,
        Outcome::Err(e) => panic!("parse: {e:?}"),
    };
    (p.into_arena(), root)
}

fn items<'a>(a: &'a Arena<'a>, root: AstRef) -> &'a [AstRef] {
    let Node::File { items, .. } = a.get(root) else { panic!("not a file") };
    a.list(items)
}

#[test]
fn a_trait_carries_generics_supertraits_and_an_associated_type() {
    // this is the headline feature set, so it gets the headline test
    let src = "
        pub trait Iterator<T>: Clone + Debug where T: Sized {
            type Item;
            type Output = T;
            const LIMIT: usize = 10;
            fn next(&mut self) -> Item;
            fn peek(&self) -> Item { self.next() }
        }
    ";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::ItemTrait { name: _, generics, supertraits, wheres, items: body, .. } =
        a.get(items(&a, root)[0])
    else {
        panic!("not a trait")
    };
    assert_eq!(a.list(generics).len(), 1, "one type parameter");
    assert_eq!(a.list(supertraits).len(), 2, "Clone + Debug");
    assert_eq!(a.list(wheres).len(), 1, "one where predicate");

    let body = a.list(body);
    assert_eq!(body.len(), 5);

    // an associated type DECLARATION has no `= Ty`; a defaulted one does
    let Node::ItemTypeAlias { ty, .. } = a.get(body[0]) else { panic!("not a type alias") };
    assert!(matches!(ty, Maybe::Isnt), "`type Item;` is a declaration");
    let Node::ItemTypeAlias { ty, .. } = a.get(body[1]) else { panic!() };
    assert!(matches!(ty, Maybe::Is(_)), "`type Output = T;` has a definition");

    // a signature-only fn has no body; a defaulted one does
    let Node::ItemFn { body: b, .. } = a.get(body[3]) else { panic!("not a fn") };
    assert!(matches!(b, Maybe::Isnt), "`fn next(...);` is signature-only");
    let Node::ItemFn { body: b, .. } = a.get(body[4]) else { panic!() };
    assert!(matches!(b, Maybe::Is(_)), "a defaulted trait method has a body");
}

#[test]
fn a_generic_function_carries_its_bounds_and_where_clause() {
    let src = "fn map<T: Clone + Debug, U, const N: usize>(x: T, f: U) -> U where U: Fn { f }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::ItemFn { generics, params, ret, wheres, .. } = a.get(items(&a, root)[0]) else {
        panic!("not a fn")
    };
    let g = a.list(generics);
    assert_eq!(g.len(), 3);
    let Node::GenericParam { bounds, .. } = a.get(g[0]) else { panic!() };
    assert_eq!(a.list(bounds).len(), 2, "T: Clone + Debug");
    assert!(matches!(a.get(g[2]), Node::ConstParam { .. }), "const N: usize");
    assert_eq!(a.list(params).len(), 2);
    assert!(matches!(ret, Maybe::Is(_)));
    assert_eq!(a.list(wheres).len(), 1);
}

#[test]
fn an_impl_block_distinguishes_inherent_from_trait() {
    let src = "impl<T> Foo<T> { fn a(self) {} } impl<T> Trait for Foo<T> { type Item = T; }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let it = items(&a, root);

    let Node::ItemImpl { for_ty, .. } = a.get(it[0]) else { panic!("not an impl") };
    assert!(matches!(for_ty, Maybe::Isnt), "an inherent impl has no `for`");

    let Node::ItemImpl { for_ty, items: body, .. } = a.get(it[1]) else { panic!() };
    assert!(matches!(for_ty, Maybe::Is(_)), "a trait impl has one");
    assert!(matches!(a.get(a.list(body)[0]), Node::ItemTypeAlias { .. }));
}

#[test]
fn a_struct_carries_visibility_field_modifiers_and_a_bind_target() {
    let src = "pub sealed struct S<T>: Bind where T: Copy { pub const a: u32 = 1, mut b: T }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::ItemStruct { sealed, generics, bind_target, wheres, fields, vis, .. } =
        a.get(items(&a, root)[0])
    else {
        panic!("not a struct")
    };
    assert!(sealed);
    assert!(matches!(vis, Maybe::Is(_)));
    assert_eq!(a.list(generics).len(), 1);
    assert!(matches!(bind_target, Maybe::Is(_)), "the colon tail is the bind target");
    assert_eq!(a.list(wheres).len(), 1);

    let fs = a.list(fields);
    assert_eq!(fs.len(), 2);
    let Node::Field2 { is_const, default, .. } = a.get(fs[0]) else { panic!() };
    assert!(is_const);
    assert!(matches!(default, Maybe::Is(_)));
    let Node::Field2 { is_mut, .. } = a.get(fs[1]) else { panic!() };
    assert!(is_mut);
}

#[test]
fn an_enum_carries_variants_with_tuple_payloads() {
    let src = "enum E<T> { None, Some(T), Pair(T, u32) }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::ItemEnum { variants, .. } = a.get(items(&a, root)[0]) else { panic!("not an enum") };
    let vs = a.list(variants);
    assert_eq!(vs.len(), 3);
    let Node::Variant { payload, .. } = a.get(vs[0]) else { panic!() };
    assert_eq!(a.list(payload).len(), 0);
    let Node::Variant { payload, .. } = a.get(vs[2]) else { panic!() };
    assert_eq!(a.list(payload).len(), 2);
}

#[test]
fn a_macro_declaration_requires_a_return_type() {
    // the mandatory return type is what makes a macro a typed function at a
    // compile stage rather than a token rewriter
    let src = "macro build<T>(x: T) -> Expr { x }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::ItemMacro { generics, params, .. } = a.get(items(&a, root)[0]) else {
        panic!("not a macro decl")
    };
    assert_eq!(a.list(generics).len(), 1);
    assert_eq!(a.list(params).len(), 1);
}

#[test]
fn use_trees_nest_rename_and_glob() {
    let src = "use a::b; use a::{c, d as e}; use a::*;";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let it = items(&a, root);
    assert_eq!(it.len(), 3);

    let Node::ItemUse { tree, .. } = a.get(it[1]) else { panic!() };
    let Node::UseTree { children, .. } = a.get(tree) else { panic!() };
    let kids = a.list(children);
    assert_eq!(kids.len(), 2);
    let Node::UseTree { alias, .. } = a.get(kids[1]) else { panic!() };
    assert!(matches!(alias, Maybe::Is(_)), "d as e is renamed");

    let Node::ItemUse { tree, .. } = a.get(it[2]) else { panic!() };
    let Node::UseTree { glob, .. } = a.get(tree) else { panic!() };
    assert!(glob);
}

#[test]
fn the_target_flavoured_items_parse_as_ordinary_items() {
    // `event`, `expect`, `actual`, and `extern` look like they need bespoke
    // machinery and do not: each is an item wrapping an item or a body
    let src = "
        event OnTick for World { do_it(); }
        expect fn provided() -> u32;
        actual fn provided() -> u32 { 1 }
        extern fn host(x: u32) -> u32;
    ";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let it = items(&a, root);
    assert_eq!(it.len(), 4);
    assert!(matches!(a.get(it[0]), Node::ItemEvent { .. }));
    assert!(matches!(a.get(it[1]), Node::ItemExpect { .. }));
    assert!(matches!(a.get(it[2]), Node::ItemActual { .. }));
    assert!(matches!(a.get(it[3]), Node::ItemExtern { .. }));
}

#[test]
fn attributes_attach_to_items_and_to_the_file() {
    let src = "#![cfg(all)] #[derive(Clone)] pub fn f() {}";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::File { attrs, items } = a.get(root) else { panic!() };
    assert_eq!(a.list(attrs).len(), 1, "the inner attribute is the file's");
    let Node::ItemFn { attrs, .. } = a.get(a.list(items)[0]) else { panic!() };
    assert_eq!(a.list(attrs).len(), 1, "the outer attribute is the item's");
}

#[test]
fn a_method_takes_self_in_its_several_spellings() {
    let src = "impl S { fn a(self) {} fn b(&self) {} fn c(&mut self, x: u32) {} }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let Node::ItemImpl { items: body, .. } = a.get(items(&a, root)[0]) else { panic!() };
    for (i, expect_params) in [(0usize, 1usize), (1, 1), (2, 2)] {
        let Node::ItemFn { params, .. } = a.get(a.list(body)[i]) else { panic!() };
        assert_eq!(a.list(params).len(), expect_params);
        let Node::Param { self_param, .. } = a.get(a.list(params)[0]) else { panic!() };
        assert!(self_param, "the first parameter is self");
    }
}

#[test]
fn a_module_may_be_a_declaration_or_a_body() {
    let src = "mod a; mod b { fn f() {} }";
    let mut f = Fixture::new();
    let (a, root) = file(&mut f, src);
    let it = items(&a, root);
    let Node::ItemMod { items: i0, .. } = a.get(it[0]) else { panic!() };
    assert!(matches!(i0, Maybe::Isnt), "`mod a;` has no body");
    let Node::ItemMod { items: i1, .. } = a.get(it[1]) else { panic!() };
    assert!(matches!(i1, Maybe::Is(_)));
}

//! Every Vehje keyword tokenises to its expected `TokenKind`.

use vehje_ir::{FileId, TokenKind};
use vehje_lex::Lexer;

fn first_kind(src: &str) -> TokenKind {
    let mut lx = Lexer::from_str(src, FileId(0));
    lx.next().expect("at least one token").kind
}

#[test]
fn declaration_keywords() {
    assert_eq!(first_kind("fn"), TokenKind::Fn);
    assert_eq!(first_kind("let"), TokenKind::Let);
    assert_eq!(first_kind("mut"), TokenKind::Mut);
    assert_eq!(first_kind("const"), TokenKind::Const);
    assert_eq!(first_kind("struct"), TokenKind::Struct);
    assert_eq!(first_kind("pub"), TokenKind::Pub);
    assert_eq!(first_kind("use"), TokenKind::Use);
    assert_eq!(first_kind("mod"), TokenKind::Mod);
    assert_eq!(first_kind("impl"), TokenKind::Impl);
    assert_eq!(first_kind("trait"), TokenKind::Trait);
    assert_eq!(first_kind("type"), TokenKind::Type);
    assert_eq!(first_kind("enum"), TokenKind::Enum);
    assert_eq!(first_kind("macro"), TokenKind::Macro);
    assert_eq!(first_kind("expect"), TokenKind::Expect);
    assert_eq!(first_kind("actual"), TokenKind::Actual);
    assert_eq!(first_kind("static"), TokenKind::Static);
    assert_eq!(first_kind("extern"), TokenKind::Extern);
}

#[test]
fn control_flow_keywords() {
    assert_eq!(first_kind("if"), TokenKind::If);
    assert_eq!(first_kind("else"), TokenKind::Else);
    assert_eq!(first_kind("match"), TokenKind::Match);
    assert_eq!(first_kind("for"), TokenKind::For);
    assert_eq!(first_kind("while"), TokenKind::While);
    assert_eq!(first_kind("loop"), TokenKind::Loop);
    assert_eq!(first_kind("return"), TokenKind::Return);
    assert_eq!(first_kind("break"), TokenKind::Break);
    assert_eq!(first_kind("continue"), TokenKind::Continue);
    assert_eq!(first_kind("in"), TokenKind::In);
}

#[test]
fn scope_and_reserved_keywords() {
    assert_eq!(first_kind("self"), TokenKind::Self_);
    assert_eq!(first_kind("super"), TokenKind::Super);
    assert_eq!(first_kind("crate"), TokenKind::Crate);
    assert_eq!(first_kind("true"), TokenKind::True);
    assert_eq!(first_kind("false"), TokenKind::False);
    assert_eq!(first_kind("as"), TokenKind::As);
    assert_eq!(first_kind("where"), TokenKind::Where);
    assert_eq!(first_kind("move"), TokenKind::Move);
    assert_eq!(first_kind("async"), TokenKind::Async);
    assert_eq!(first_kind("await"), TokenKind::Await);
    assert_eq!(first_kind("dyn"), TokenKind::Dyn);
    assert_eq!(first_kind("unsafe"), TokenKind::Unsafe);
    assert_eq!(first_kind("ref"), TokenKind::Ref);
}

#[test]
fn keyword_followed_by_ident_char_is_ident() {
    // `fno` is an ident, not `fn` + `o`. (The lexer reads the whole
    // run of ident-cont bytes before classifying.)
    assert_eq!(first_kind("fno"), TokenKind::Ident);
    assert_eq!(first_kind("let_me"), TokenKind::Ident);
    assert_eq!(first_kind("trueish"), TokenKind::Ident);
}

#[test]
fn case_sensitive() {
    // Uppercase variants should be idents (case-sensitive).
    assert_eq!(first_kind("Fn"), TokenKind::Ident);
    assert_eq!(first_kind("LET"), TokenKind::Ident);
    assert_eq!(first_kind("True"), TokenKind::Ident);
}

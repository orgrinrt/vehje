//! Non-keyword identifiers.

use clause_ir::{FileId, TokenKind};
use clause_lex::Lexer;

fn first(src: &str) -> (TokenKind, u32, u32) {
    let mut lx = Lexer::from_str(src, FileId(0));
    let t = lx.next().expect("token");
    (t.kind, t.span.start.0, t.span.end.0)
}

#[test]
fn simple_ident() {
    assert_eq!(first("foo"), (TokenKind::Ident, 0, 3));
}

#[test]
fn underscore_prefixed() {
    assert_eq!(first("_foo"), (TokenKind::Ident, 0, 4));
    assert_eq!(first("_"), (TokenKind::Ident, 0, 1));
}

#[test]
fn ident_with_digits() {
    assert_eq!(first("foo123"), (TokenKind::Ident, 0, 6));
}

#[test]
fn ident_mixed_case() {
    assert_eq!(first("FooBar"), (TokenKind::Ident, 0, 6));
    assert_eq!(first("snake_case"), (TokenKind::Ident, 0, 10));
}

#[test]
fn ident_stops_at_nonident_byte() {
    // `foo(` should produce `Ident(foo)` then `LParen(()`.
    let mut lx = Lexer::from_str("foo(", FileId(0));
    let a = lx.next().unwrap();
    let b = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::Ident);
    assert_eq!(a.span.start.0, 0);
    assert_eq!(a.span.end.0, 3);
    assert_eq!(b.kind, TokenKind::LParen);
}

#[test]
fn ident_cannot_start_with_digit() {
    let mut lx = Lexer::from_str("123abc", FileId(0));
    let a = lx.next().unwrap();
    let b = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::IntLit);
    assert_eq!(b.kind, TokenKind::Ident);
}

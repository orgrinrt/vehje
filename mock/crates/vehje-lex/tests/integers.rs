//! Decimal integer literals.

use vehje_ir::{FileId, TokenKind};
use vehje_lex::Lexer;

fn first(src: &str) -> (TokenKind, u32, u32) {
    let mut lx = Lexer::from_str(src, FileId(0));
    let t = lx.next().expect("token");
    (t.kind, t.span.start.0, t.span.end.0)
}

#[test]
fn single_digit() {
    assert_eq!(first("0"), (TokenKind::IntLit, 0, 1));
    assert_eq!(first("7"), (TokenKind::IntLit, 0, 1));
}

#[test]
fn multi_digit() {
    assert_eq!(first("123"), (TokenKind::IntLit, 0, 3));
    assert_eq!(first("9876543210"), (TokenKind::IntLit, 0, 10));
}

#[test]
fn int_then_whitespace_then_ident() {
    let mut lx = Lexer::from_str("42 foo", FileId(0));
    let a = lx.next().unwrap();
    let b = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::IntLit);
    assert_eq!(a.span.end.0, 2);
    assert_eq!(b.kind, TokenKind::Ident);
    assert_eq!(b.span.start.0, 3);
}

#[test]
fn int_then_dot_then_ident() {
    // This round does not recognise floats; `1.foo` lexes as
    // `IntLit(1) Dot Ident(foo)`.
    let mut lx = Lexer::from_str("1.foo", FileId(0));
    let a = lx.next().unwrap();
    let b = lx.next().unwrap();
    let c = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::IntLit);
    assert_eq!(b.kind, TokenKind::Dot);
    assert_eq!(c.kind, TokenKind::Ident);
}

#[test]
fn leading_sign_is_not_part_of_int() {
    // Unary `-` / `+` is a separate operator; literal sign handling
    // is a parser concern.
    let mut lx = Lexer::from_str("-5", FileId(0));
    let a = lx.next().unwrap();
    let b = lx.next().unwrap();
    assert_eq!(a.kind, TokenKind::Minus);
    assert_eq!(b.kind, TokenKind::IntLit);
}

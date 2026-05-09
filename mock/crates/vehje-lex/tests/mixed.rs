//! Lex a small program and verify the TokenKind sequence.

use notko::Maybe;
use vehje_ir::{FileId, TokenKind};
use vehje_lex::{Lexer, TokenStream};

fn collect(src: &str) -> Vec<TokenKind> {
    let mut lx = Lexer::from_str(src, FileId(0));
    let mut out = Vec::new();
    while let Maybe::Is(t) = lx.next() {
        out.push(t.kind);
    }
    out
}

#[test]
fn fn_declaration() {
    let kinds = collect("fn add(a, b) { a + b }");
    assert_eq!(
        kinds,
        vec![
            TokenKind::Fn,
            TokenKind::Ident,
            TokenKind::LParen,
            TokenKind::Ident,
            TokenKind::Comma,
            TokenKind::Ident,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Ident,
            TokenKind::Plus,
            TokenKind::Ident,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn let_statement() {
    let kinds = collect("let x = 42;");
    assert_eq!(
        kinds,
        vec![
            TokenKind::Let,
            TokenKind::Ident,
            TokenKind::Eq,
            TokenKind::IntLit,
            TokenKind::Semi,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn nested_parens_and_ops() {
    let kinds = collect("((a + b) * c) == 0");
    assert_eq!(
        kinds,
        vec![
            TokenKind::LParen,
            TokenKind::LParen,
            TokenKind::Ident,
            TokenKind::Plus,
            TokenKind::Ident,
            TokenKind::RParen,
            TokenKind::Star,
            TokenKind::Ident,
            TokenKind::RParen,
            TokenKind::EqEq,
            TokenKind::IntLit,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn path_with_colon_colon() {
    let kinds = collect("std::io::println");
    assert_eq!(
        kinds,
        vec![
            TokenKind::Ident,
            TokenKind::ColonColon,
            TokenKind::Ident,
            TokenKind::ColonColon,
            TokenKind::Ident,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn comments_and_whitespace_do_not_change_token_sequence() {
    let a = collect("fn foo() { 1 }");
    let b = collect("// header\nfn foo() /* inline */ { 1 /*trailing*/ }");
    assert_eq!(a, b);
}

#[test]
fn unknown_byte_emits_unknown_token() {
    // Backtick is not in the operator or punctuation set.
    let kinds = collect("`");
    assert_eq!(kinds, vec![TokenKind::Unknown, TokenKind::Eof]);
}

#[test]
fn token_stream_peek_does_not_consume() {
    let lx = Lexer::from_str("a b", FileId(0));
    let mut ts = TokenStream::new(lx);
    let k1 = ts.peek().map(|t| t.kind);
    let k2 = ts.peek().map(|t| t.kind);
    assert_eq!(k1, Maybe::Is(TokenKind::Ident));
    assert_eq!(k2, Maybe::Is(TokenKind::Ident));
    let n1 = ts.next().map(|t| t.kind);
    assert_eq!(n1, Maybe::Is(TokenKind::Ident));
    let n2 = ts.next().map(|t| t.kind);
    assert_eq!(n2, Maybe::Is(TokenKind::Ident));
}

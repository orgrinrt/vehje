//! Parsing trivial inputs: empty slice and a slice of only Eof.

use clause_ir::{ByteOffset, FileId, Span, TokenKind};
use clause_lex::Token;
use clause_syntax::parse;

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end))
}

fn eof_token(at: u32) -> Token {
    Token::bare(TokenKind::Eof, span(at, at))
}

#[test]
fn parse_empty_slice() {
    let ast = parse(&[]).expect("empty slice parses");
    assert!(ast.is_empty());
    assert_eq!(ast.root(), None);
}

#[test]
fn parse_slice_with_only_eof() {
    let tokens = [eof_token(0)];
    let ast = parse(&tokens).expect("lone eof parses");
    assert!(ast.is_empty());
    assert_eq!(ast.root(), None);
}

//! Parsing a single integer literal, with and without a trailing
//! Eof token. Both shapes produce the same one-node Ast.

use clause_ir::{AstNodeKind, ByteOffset, FileId, NodeId, Span, TokenKind};
use clause_lex::Token;
use clause_syntax::parse;

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end))
}

fn int_token(start: u32, end: u32) -> Token {
    Token::bare(TokenKind::IntLit, span(start, end))
}

fn eof_token(at: u32) -> Token {
    Token::bare(TokenKind::Eof, span(at, at))
}

#[test]
fn parse_int_literal_then_eof() {
    let tokens = [int_token(0, 3), eof_token(3)];
    let ast = parse(&tokens).expect("literal + eof parses");
    assert_eq!(ast.len(), 1);
    let root_id = ast.root().expect("root set");
    let root = ast.get(root_id).expect("root present");
    assert_eq!(root.kind, AstNodeKind::Expr);
    assert_eq!(root.span, span(0, 3));
    assert_eq!(root.child_count(), 0);
}

#[test]
fn parse_int_literal_at_end_of_slice() {
    let tokens = [int_token(0, 2)];
    let ast = parse(&tokens).expect("literal alone parses");
    assert_eq!(ast.len(), 1);
    let root = ast.get(ast.root().unwrap()).unwrap();
    assert_eq!(root.kind, AstNodeKind::Expr);
    assert_eq!(root.span, span(0, 2));
}

#[test]
fn parse_int_literal_sets_root() {
    let tokens = [int_token(0, 1), eof_token(1)];
    let ast = parse(&tokens).unwrap();
    assert_eq!(ast.root(), Some(NodeId(0)));
}

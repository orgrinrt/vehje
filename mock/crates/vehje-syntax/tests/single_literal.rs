//! Parsing a single integer literal, with and without a trailing
//! Eof token. Both shapes produce the same one-node Ast.

use arvo::USize;
use hilavitkutin_api::{Len, Push};
use notko::Maybe;
use vehje_ir::{AstNodeKind, ByteOffset, FileId, NodeId, Span, TokenKind};
use vehje_lex::Token;
use vehje_syntax::{SyntaxError, parse};

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end)) // lint:allow(no-bare-numeric) reason: FileId(0) sentinel for single-file tests; tracked: #412
}

fn int_token(start: u32, end: u32) -> Token {
    Token::bare(TokenKind::IntLit, span(start, end))
}

fn eof_token(at: u32) -> Token {
    Token::bare(TokenKind::Eof, span(at, at))
}

#[derive(Default)]
struct ErrSink {
    count: usize, // lint:allow(no-bare-numeric) reason: test-internal counter; tracked: #412
}

impl Push<SyntaxError> for ErrSink {
    fn push(&mut self, _e: SyntaxError) {
        self.count += 1; // lint:allow(no-bare-numeric) reason: test-internal counter; tracked: #412
    }
}

impl Len for ErrSink {
    fn len(&self) -> USize {
        USize(self.count)
    }
}

#[test]
fn parse_int_literal_then_eof() {
    let mut errs = ErrSink::default();
    let tokens = [int_token(0, 3), eof_token(3)]; // lint:allow(no-bare-numeric) reason: byte-offset literals; tracked: #412
    let ast = parse(&tokens, &mut errs).expect("literal + eof parses");
    assert_eq!(ast.len(), 1); // lint:allow(no-bare-numeric) reason: ast.len() returns bare usize per current source surface; tracked: #412
    let root_id = ast.root().expect("root set");
    let root = ast.get(root_id).expect("root present");
    assert_eq!(root.kind, AstNodeKind::Expr);
    assert_eq!(root.span, span(0, 3)); // lint:allow(no-bare-numeric) reason: byte-offset literals; tracked: #412
    assert_eq!(root.child_count(), 0); // lint:allow(no-bare-numeric) reason: child_count() returns bare usize per current source surface; tracked: #412
}

#[test]
fn parse_int_literal_at_end_of_slice() {
    let mut errs = ErrSink::default();
    let tokens = [int_token(0, 2)]; // lint:allow(no-bare-numeric) reason: byte-offset literal; tracked: #412
    let ast = parse(&tokens, &mut errs).expect("literal alone parses");
    assert_eq!(ast.len(), 1); // lint:allow(no-bare-numeric) reason: ast.len() returns bare usize per current source surface; tracked: #412
    let root = ast.get(ast.root().unwrap()).unwrap();
    assert_eq!(root.kind, AstNodeKind::Expr);
    assert_eq!(root.span, span(0, 2)); // lint:allow(no-bare-numeric) reason: byte-offset literal; tracked: #412
}

#[test]
fn parse_int_literal_sets_root() {
    let mut errs = ErrSink::default();
    let tokens = [int_token(0, 1), eof_token(1)]; // lint:allow(no-bare-numeric) reason: byte-offset literals; tracked: #412
    let ast = parse(&tokens, &mut errs).unwrap();
    assert_eq!(ast.root(), Maybe::Is(NodeId(0))); // lint:allow(no-bare-numeric) reason: NodeId(0) is first-node sentinel; tracked: #412
}

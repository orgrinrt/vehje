//! UnexpectedToken paths: any kind the skeleton does not accept at
//! the top level triggers the error, including kinds that will
//! become valid starts in follow-up rounds (Ident, Plus, …).

use clause_ir::{ByteOffset, FileId, Span, TokenKind};
use clause_lex::Token;
use clause_syntax::{parse, SyntaxErrorKind};

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end))
}

fn tok(kind: TokenKind, start: u32, end: u32) -> Token {
    Token::bare(kind, span(start, end))
}

#[test]
fn parse_plus_is_unexpected_token() {
    let tokens = [tok(TokenKind::Plus, 0, 1), tok(TokenKind::Eof, 1, 1)];
    let err = parse(&tokens).expect_err("leading + fails");
    assert_eq!(err.kind, SyntaxErrorKind::UnexpectedToken);
}

#[test]
fn parse_ident_is_unexpected_token_in_skeleton() {
    // Identifier starts are valid grammar in later rounds; the
    // skeleton only accepts IntLit at the top level today.
    let tokens = [tok(TokenKind::Ident, 0, 3), tok(TokenKind::Eof, 3, 3)];
    let err = parse(&tokens).expect_err("leading ident fails in skeleton");
    assert_eq!(err.kind, SyntaxErrorKind::UnexpectedToken);
}

#[test]
fn parse_literal_followed_by_non_eof_is_unexpected_token() {
    let tokens = [
        tok(TokenKind::IntLit, 0, 2),
        tok(TokenKind::Plus, 3, 4),
        tok(TokenKind::Eof, 4, 4),
    ];
    let err = parse(&tokens).expect_err("literal then + fails in skeleton");
    assert_eq!(err.kind, SyntaxErrorKind::UnexpectedToken);
}

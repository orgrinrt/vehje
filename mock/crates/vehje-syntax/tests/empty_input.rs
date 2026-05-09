//! Parsing trivial inputs: empty slice and a slice of only Eof.

use arvo::USize;
use hilavitkutin_api::{Len, Push};
use notko::Maybe;
use vehje_ir::{ByteOffset, FileId, Span, TokenKind};
use vehje_lex::Token;
use vehje_syntax::{parse, SyntaxError};

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end)) // lint:allow(no-bare-numeric) reason: FileId(0) sentinel for single-file tests; tracked: #412
}

fn eof_token(at: u32) -> Token {
    Token::bare(TokenKind::Eof, span(at, at))
}

/// Bounded test-side `DiagnosticSink<SyntaxError>`. Counts pushes; tests
/// that only need to confirm error-count or success/failure read `len()`.
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
fn parse_empty_slice() {
    let mut errs = ErrSink::default();
    let ast = parse(&[], &mut errs).expect("empty slice parses");
    assert!(ast.is_empty());
    assert_eq!(ast.root(), Maybe::Isnt);
}

#[test]
fn parse_slice_with_only_eof() {
    let mut errs = ErrSink::default();
    let tokens = [eof_token(0)]; // lint:allow(no-bare-numeric) reason: byte-offset literal; tracked: #412
    let ast = parse(&tokens, &mut errs).expect("lone eof parses");
    assert!(ast.is_empty());
    assert_eq!(ast.root(), Maybe::Isnt);
}

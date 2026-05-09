//! UnexpectedToken paths: any kind the skeleton does not accept at
//! the top level triggers the error, including kinds that will
//! become valid starts in follow-up rounds (Ident, Plus, ...).

use arvo::USize;
use hilavitkutin_api::{Len, Push};
use vehje_ir::{ByteOffset, FileId, Span, TokenKind};
use vehje_lex::Token;
use vehje_syntax::{parse, SyntaxError, SyntaxErrorKind};

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end)) // lint:allow(no-bare-numeric) reason: FileId(0) sentinel for single-file tests; tracked: #412
}

fn tok(kind: TokenKind, start: u32, end: u32) -> Token {
    Token::bare(kind, span(start, end))
}

/// Bounded sink with capacity 4: enough for skeleton tests that
/// expect at most one or two errors. The actual stored values let
/// the test inspect `kind`.
struct ErrSink<const N: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    errs: [core::mem::MaybeUninit<SyntaxError>; N],
    count: usize, // lint:allow(no-bare-numeric) reason: test-internal counter; tracked: #412
}

impl<const N: usize> ErrSink<N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const fn new() -> Self {
        Self {
            errs: [const { core::mem::MaybeUninit::uninit() }; N],
            count: 0, // lint:allow(no-bare-numeric) reason: counter init; tracked: #412
        }
    }

    fn at(&self, idx: usize) -> &SyntaxError { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: index into MaybeUninit array; tracked: #412
        assert!(idx < self.count, "index out of bounds");
        // SAFETY: idx < count, count slots are initialised in order.
        unsafe { self.errs[idx].assume_init_ref() }
    }
}

impl<const N: usize> Push<SyntaxError> for ErrSink<N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn push(&mut self, e: SyntaxError) {
        assert!(self.count < N, "ErrSink overflow");
        self.errs[self.count].write(e);
        self.count += 1; // lint:allow(no-bare-numeric) reason: counter increment; tracked: #412
    }
}

impl<const N: usize> Len for ErrSink<N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn len(&self) -> USize {
        USize(self.count)
    }
}

#[test]
fn parse_plus_is_unexpected_token() {
    let mut errs: ErrSink<4> = ErrSink::new(); // lint:allow(no-bare-numeric) reason: bounded sink capacity; tracked: #412
    let tokens = [tok(TokenKind::Plus, 0, 1), tok(TokenKind::Eof, 1, 1)]; // lint:allow(no-bare-numeric) reason: byte-offset literals; tracked: #412
    let _ = parse(&tokens, &mut errs).expect_err("leading + fails");
    assert_eq!(errs.len(), USize(1)); // lint:allow(no-bare-numeric) reason: error count; tracked: #412
    assert_eq!(errs.at(0).kind, SyntaxErrorKind::UnexpectedToken); // lint:allow(no-bare-numeric) reason: index into bounded sink; tracked: #412
}

#[test]
fn parse_ident_is_unexpected_token_in_skeleton() {
    // Identifier starts are valid grammar in later rounds; the
    // skeleton only accepts IntLit at the top level today.
    let mut errs: ErrSink<4> = ErrSink::new(); // lint:allow(no-bare-numeric) reason: bounded sink capacity; tracked: #412
    let tokens = [tok(TokenKind::Ident, 0, 3), tok(TokenKind::Eof, 3, 3)]; // lint:allow(no-bare-numeric) reason: byte-offset literals; tracked: #412
    let _ = parse(&tokens, &mut errs).expect_err("leading ident fails in skeleton");
    assert_eq!(errs.len(), USize(1)); // lint:allow(no-bare-numeric) reason: error count; tracked: #412
    assert_eq!(errs.at(0).kind, SyntaxErrorKind::UnexpectedToken); // lint:allow(no-bare-numeric) reason: index into bounded sink; tracked: #412
}

#[test]
fn parse_literal_followed_by_non_eof_is_unexpected_token() {
    let mut errs: ErrSink<4> = ErrSink::new(); // lint:allow(no-bare-numeric) reason: bounded sink capacity; tracked: #412
    let tokens = [
        tok(TokenKind::IntLit, 0, 2), // lint:allow(no-bare-numeric) reason: byte-offset literal; tracked: #412
        tok(TokenKind::Plus, 3, 4), // lint:allow(no-bare-numeric) reason: byte-offset literal; tracked: #412
        tok(TokenKind::Eof, 4, 4), // lint:allow(no-bare-numeric) reason: byte-offset literal; tracked: #412
    ];
    let _ = parse(&tokens, &mut errs).expect_err("literal then + fails in skeleton");
    assert_eq!(errs.len(), USize(1)); // lint:allow(no-bare-numeric) reason: error count; tracked: #412
    assert_eq!(errs.at(0).kind, SyntaxErrorKind::UnexpectedToken); // lint:allow(no-bare-numeric) reason: index into bounded sink; tracked: #412
}

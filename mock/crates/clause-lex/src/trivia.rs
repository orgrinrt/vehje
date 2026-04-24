//! Lex-level trivia (whitespace + comments).
//!
//! Trivia is tracked separately from real tokens so the lexer can
//! round-trip source text through a future formatter. Each token
//! carries a `TriviaSet` of up-to-eight leading and up-to-eight
//! trailing `Trivia` items; excess trivia is coalesced into the last
//! slot (the span's `end` extends, the `kind` becomes
//! `TriviaKind::Whitespace` if a mix of kinds collides).
//!
//! The const-sized representation keeps the lexer alloc-free. Eight
//! slots covers every realistic case — more than a couple of blank
//! lines of comments before a token is already unusual.

use vehje_ir::Span;

/// What a piece of trivia is.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
pub enum TriviaKind {
    Whitespace,
    LineComment,
    BlockComment,
}

impl Default for TriviaKind {
    fn default() -> Self {
        Self::Whitespace
    }
}

/// A single piece of lexical trivia.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Trivia {
    pub kind: TriviaKind,
    pub span: Span,
}

impl Trivia {
    pub const fn new(kind: TriviaKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Maximum trivia slots tracked per side. Excess coalesces into the
/// final slot; see `TriviaSet::push_leading` / `push_trailing`.
pub const TRIVIA_SLOTS: usize = 8;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207

/// Fixed-size leading-and-trailing trivia set attached to a token.
///
/// Slots are filled in insertion order. `leading_len` and
/// `trailing_len` are always `<= TRIVIA_SLOTS`. When a push would
/// exceed the slot count the incoming trivia's span is merged into
/// the tail slot — its span's `end` advances to the new trivia's
/// `end` — and the tail slot's kind is preserved unless the new
/// kind differs, in which case the tail slot becomes `Whitespace`
/// (the generic fallback).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct TriviaSet {
    leading: [Trivia; TRIVIA_SLOTS],
    trailing: [Trivia; TRIVIA_SLOTS],
    leading_len: u8,  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207 lint:allow(no-public-raw-field) tracked: #207
    trailing_len: u8,  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207 lint:allow(no-public-raw-field) tracked: #207
}

impl TriviaSet {
    /// Construct an empty trivia set.
    pub const fn new() -> Self {
        Self {
            leading: [Trivia {
                kind: TriviaKind::Whitespace,
                span: Span::new(
                    vehje_ir::FileId(0),
                    vehje_ir::ByteOffset(0),
                    vehje_ir::ByteOffset(0),
                ),
            }; TRIVIA_SLOTS],
            trailing: [Trivia {
                kind: TriviaKind::Whitespace,
                span: Span::new(
                    vehje_ir::FileId(0),
                    vehje_ir::ByteOffset(0),
                    vehje_ir::ByteOffset(0),
                ),
            }; TRIVIA_SLOTS],
            leading_len: 0,
            trailing_len: 0,
        }
    }

    pub fn leading(&self) -> &[Trivia] {
        &self.leading[..self.leading_len as usize]  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    }

    pub fn trailing(&self) -> &[Trivia] {
        &self.trailing[..self.trailing_len as usize]  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
    }

    /// Push a leading trivia. Coalesces into the last slot when full.
    pub fn push_leading(&mut self, t: Trivia) {
        Self::push_side(&mut self.leading, &mut self.leading_len, t);
    }

    /// Push a trailing trivia. Coalesces into the last slot when full.
    pub fn push_trailing(&mut self, t: Trivia) {
        Self::push_side(&mut self.trailing, &mut self.trailing_len, t);
    }

    fn push_side(buf: &mut [Trivia; TRIVIA_SLOTS], len: &mut u8, t: Trivia) {  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
        let n = *len as usize;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
        if n < TRIVIA_SLOTS {
            buf[n] = t;
            *len = n as u8 + 1;  // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
            return;
        }
        // Coalesce into the last slot.
        let tail = &mut buf[TRIVIA_SLOTS - 1];
        if tail.kind != t.kind {
            tail.kind = TriviaKind::Whitespace;
        }
        tail.span.end = t.span.end;
    }
}

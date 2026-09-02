//! `TokenStream`, single-token-lookahead wrapper over `Lexer`.
//!
//! Parsers typically want a `peek` method without consuming the
//! token. `TokenStream` buffers one token ahead; `peek` lazily
//! fetches it from the lexer, `next` returns the buffered token and
//! clears the buffer. EOF is reported by returning a token with
//! `kind == TokenKind::Eof`; subsequent calls after EOF return
//! `Maybe::Isnt`.

use notko::Maybe;
use vehje_ir::TokenKind;

use crate::lexer::Lexer;
use crate::token::Token;

/// Single-token-lookahead stream.
pub struct TokenStream<'a> {
    lexer:  Lexer<'a>,
    peeked: Maybe<Maybe<Token>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            peeked: Maybe::Isnt,
        }
    }

    /// Peek at the next token without consuming it.
    ///
    /// Returns `Maybe::Isnt` once the lexer is fully drained (past
    /// the terminating `Eof`).
    pub fn peek(&mut self) -> Maybe<&Token> {
        if self.peeked.isnt() {
            self.peeked = Maybe::Is(self.lexer.next());
        }
        match self.peeked {
            Maybe::Is(Maybe::Is(ref t)) => Maybe::Is(t),
            _ => Maybe::Isnt,
        }
    }

    /// Consume and return the next token.
    pub fn next(&mut self) -> Maybe<Token> {
        let slot = core::mem::replace(&mut self.peeked, Maybe::Isnt);
        match slot {
            Maybe::Is(inner) => inner,
            Maybe::Isnt => self.lexer.next(),
        }
    }

    /// `true` if the next token is `Eof` or the stream is drained.
    pub fn is_at_end(&mut self) -> bool {
        // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) tracked: #207
        match self.peek() {
            Maybe::Is(t) => t.kind == TokenKind::Eof,
            Maybe::Isnt => true,
        }
    }
}

//! `TokenStream` — single-token-lookahead wrapper over `Lexer`.
//!
//! Parsers typically want a `peek` method without consuming the
//! token. `TokenStream` buffers one token ahead; `peek` lazily
//! fetches it from the lexer, `next` returns the buffered token and
//! clears the buffer. EOF is reported by returning a token with
//! `kind == TokenKind::Eof`; subsequent calls after EOF return
//! `None`.

use clause_ir::TokenKind;

use crate::lexer::Lexer;
use crate::token::Token;

/// Single-token-lookahead stream.
pub struct TokenStream<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Option<Token>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer, peeked: None }
    }

    /// Peek at the next token without consuming it.
    ///
    /// Returns `None` once the lexer is fully drained (past the
    /// terminating `Eof`).
    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        match self.peeked {
            Some(Some(ref t)) => Some(t),
            _ => None,
        }
    }

    /// Consume and return the next token.
    pub fn next(&mut self) -> Option<Token> {
        match self.peeked.take() {
            Some(slot) => slot,
            None => self.lexer.next(),
        }
    }

    /// `true` if the next token is `Eof` or the stream is drained.
    pub fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Some(t) => t.kind == TokenKind::Eof,
            None => true,
        }
    }
}

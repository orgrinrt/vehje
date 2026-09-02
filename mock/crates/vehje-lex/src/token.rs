//! Lex-level `Token`, kind + span + attached trivia.
//!
//! A `Token` carries the discriminator the parser pattern-matches on,
//! the source span it covers, and the trivia (whitespace + comments)
//! attached to it. Trivia is attached to the next real token; the
//! final `Eof` token carries any trailing trivia after the last real
//! token.

use vehje_ir::{Span, TokenKind};

use crate::trivia::TriviaSet;

/// Lex-level token.
///
/// All three fields are `Copy`; tokens are trivially duplicated.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Token {
    pub kind:   TokenKind,
    pub span:   Span,
    pub trivia: TriviaSet,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span, trivia: TriviaSet) -> Self {
        Self {
            kind,
            span,
            trivia,
        }
    }

    /// Construct a token with an empty trivia set.
    pub const fn bare(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span,
            trivia: TriviaSet::new(),
        }
    }
}

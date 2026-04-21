//! Parser driver loop.
//!
//! `Parser<'a>` holds a borrowed token slice, a `TokenCursor`
//! newtype, and an `Ast` under construction. It exposes peek / bump
//! / matching helpers that follow-up rounds will use to implement
//! each grammar production.
//!
//! This round ships the skeleton: `parse` handles the two trivial
//! inputs (empty slice and single literal) and rejects everything
//! else with `UnexpectedToken`. Each deferred production (primary
//! path, binary, unary, call, block, let, if, match, fn, type,
//! struct, enum, module, pattern) lands as its own micro-round.

use clause_ir::TokenKind;
use clause_ir::{AstNodeKind, ByteOffset, FileId, Span};
use clause_lex::Token;
use hilavitkutin_api::DiagnosticSink;
use notko::{Maybe, Outcome};

use crate::ast::{Ast, AstNode};
use crate::error::SyntaxError;

/// Typed index into the parser's token slice.
///
/// `#[repr(transparent)]` over `u32`: zero runtime cost, but distinct
/// from any other `u32` the parser module might hold (byte offset,
/// span index, AST node id). `Copy` + `Ord` so the parser can
/// compare / advance freely.
#[repr(transparent)]
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
)]
pub struct TokenCursor(pub u32);

impl TokenCursor {
    /// Construct a cursor pointing at the token at index `idx`.
    pub const fn new(idx: u32) -> Self {
        Self(idx)
    }

    /// The cursor index as a `usize` for slice indexing.
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    /// A cursor advanced by one position.
    pub const fn advance(self) -> Self {
        Self(self.0 + 1)
    }
}

/// The parser.
///
/// Borrow-lived against the caller's token slice; no owned state
/// beyond the AST under construction.
pub struct Parser<'a> {
    tokens: &'a [Token],
    cursor: TokenCursor,
    ast: Ast,
}

impl<'a> Parser<'a> {
    /// Build a parser over `tokens`, starting at offset 0 with an
    /// empty AST.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, cursor: TokenCursor::new(0), ast: Ast::empty() }
    }

    /// Token at the current cursor, or `Maybe::Isnt` if at end.
    pub fn peek(&self) -> Maybe<&Token> {
        match self.tokens.get(self.cursor.as_usize()) {
            Some(t) => Maybe::Is(t),
            None => Maybe::Isnt,
        }
    }

    /// Kind of the token at the current cursor.
    pub fn peek_kind(&self) -> Maybe<TokenKind> {
        self.peek().map(|t| t.kind)
    }

    /// Token `offset` positions past the cursor, or `Maybe::Isnt`
    /// if out of range.
    pub fn peek_at(&self, offset: usize) -> Maybe<&Token> {
        match self.tokens.get(self.cursor.as_usize() + offset) {
            Some(t) => Maybe::Is(t),
            None => Maybe::Isnt,
        }
    }

    /// Advance the cursor by one, returning a reference to the
    /// consumed token or `Maybe::Isnt` at end of input.
    pub fn bump(&mut self) -> Maybe<&Token> {
        let idx = self.cursor.as_usize();
        let tok = match self.tokens.get(idx) {
            Some(t) => t,
            None => return Maybe::Isnt,
        };
        self.cursor = self.cursor.advance();
        Maybe::Is(tok)
    }

    /// `true` if the current token has the given kind.
    pub fn at(&self, kind: TokenKind) -> bool {
        self.peek_kind() == Maybe::Is(kind)
    }

    /// `true` if the cursor is past the end of the slice or the
    /// current token is `Eof`. Both conventions terminate parse.
    pub fn is_eof(&self) -> bool {
        match self.peek_kind() {
            Maybe::Isnt => true,
            Maybe::Is(TokenKind::Eof) => true,
            _ => false,
        }
    }

    /// Span of the current token. If past the end of input,
    /// returns a zero-length span at the last token's end (or
    /// `Span::default()` if the slice is empty).
    pub fn current_span(&self) -> Span {
        if let Maybe::Is(tok) = self.peek() {
            return tok.span;
        }
        if let Some(last) = self.tokens.last() {
            return Span::new(last.span.file, last.span.end, last.span.end);
        }
        Span::new(FileId(0), ByteOffset(0), ByteOffset(0))
    }

    /// Drive the parse to completion, pushing any errors into
    /// `errors`.
    ///
    /// Skeleton grammar:
    ///
    /// 1. Empty input / lone `Eof` → empty `Ast`.
    /// 2. `IntLit` (then optional `Eof` or end of slice) → an
    ///    `Ast` with one `AstNodeKind::Expr` root node spanning
    ///    the literal.
    /// 3. Anything else → a `SyntaxErrorKind::UnexpectedToken`
    ///    pushed into `errors`, return `Outcome::Err(())`.
    ///
    /// `errors` is `&mut dyn DiagnosticSink<SyntaxError>` so the
    /// parser can flow through the free `parse` fn (which also
    /// takes a dyn sink) without monomorphisation mismatches.
    pub fn parse(
        mut self,
        errors: &mut dyn DiagnosticSink<SyntaxError>,
    ) -> Outcome<Ast, ()> {
        if self.is_eof() {
            return Outcome::Ok(self.ast);
        }

        let first_kind = self.peek_kind().unwrap();
        if first_kind == TokenKind::IntLit {
            let span = self.current_span();
            let node = AstNode::leaf(AstNodeKind::Expr, span);
            let id = match self.ast.push(node) {
                Maybe::Is(id) => id,
                Maybe::Isnt => {
                    errors.push(SyntaxError::unexpected_token(
                        span,
                        "AST arena full",
                    ));
                    return Outcome::Err(());
                },
            };
            self.ast.set_root(id);
            self.bump();

            if !self.is_eof() {
                errors.push(SyntaxError::unexpected_token(
                    self.current_span(),
                    "expected end of input after literal",
                ));
                return Outcome::Err(());
            }
            return Outcome::Ok(self.ast);
        }

        errors.push(SyntaxError::unexpected_token(
            self.current_span(),
            "unexpected token at start of input",
        ));
        Outcome::Err(())
    }
}

/// Parse a token slice into an `Ast`, pushing soft errors into
/// `errors`.
///
/// The skeleton handles:
///
/// - Empty slice (or a slice of just `Eof`) → empty `Ast`.
/// - A single `IntLit` followed by optional `Eof` → an `Ast`
///   containing one `AstNodeKind::Expr` node spanning the
///   literal.
/// - Anything else → at least one `SyntaxError` pushed into
///   `errors`, return `Outcome::Err(())`.
///
/// `errors` is `&mut dyn DiagnosticSink<SyntaxError>` so any
/// caller that implements `Push<SyntaxError> + Len` (including
/// future multi-error recovery accumulators) can receive emitted
/// errors without a signature churn. The `Outcome::Err(())`
/// return carries the hard-failure shape; the detail lives in the
/// sink.
///
/// Every deferred production flips from `UnexpectedToken` to a
/// real parse in its own follow-up round.
pub fn parse(
    tokens: &[Token],
    errors: &mut dyn DiagnosticSink<SyntaxError>,
) -> Outcome<Ast, ()> {
    Parser::new(tokens).parse(errors)
}

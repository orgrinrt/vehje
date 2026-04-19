//! Parser driver loop.
//!
//! `Parser<'a>` holds a borrowed token slice, a cursor index, and
//! an `Ast` under construction. It exposes peek / bump / matching
//! helpers that follow-up rounds will use to implement each
//! grammar production.
//!
//! This round ships the skeleton: `parse` handles the two trivial
//! inputs (empty slice and single literal) and rejects everything
//! else with `UnexpectedToken`. Each deferred production (primary
//! path, binary, unary, call, block, let, if, match, fn, type,
//! struct, enum, module, pattern) lands as its own micro-round.

use clause_ir::{AstNodeKind, ByteOffset, FileId, Span};
use clause_lex::Token;
use clause_ir::TokenKind;

use crate::ast::{Ast, AstNode};
use crate::error::SyntaxError;

/// The parser.
///
/// Borrow-lived against the caller's token slice; no owned state
/// beyond the AST under construction.
pub struct Parser<'a> {
    tokens: &'a [Token],
    cursor: u32,
    ast: Ast,
}

impl<'a> Parser<'a> {
    /// Build a parser over `tokens`, starting at offset 0 with an
    /// empty AST.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, cursor: 0, ast: Ast::empty() }
    }

    /// Token at the current cursor, or `None` if at end.
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor as usize)
    }

    /// Kind of the token at the current cursor.
    pub fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|t| t.kind)
    }

    /// Token `offset` positions past the cursor, or `None` if out
    /// of range.
    pub fn peek_at(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.cursor as usize + offset)
    }

    /// Advance the cursor by one, returning a reference to the
    /// consumed token or `None` at end of input.
    pub fn bump(&mut self) -> Option<&Token> {
        let idx = self.cursor as usize;
        let tok = self.tokens.get(idx)?;
        self.cursor += 1;
        Some(tok)
    }

    /// `true` if the current token has the given kind.
    pub fn at(&self, kind: TokenKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    /// `true` if the cursor is past the end of the slice or the
    /// current token is `Eof`. Both conventions terminate parse.
    pub fn is_eof(&self) -> bool {
        match self.peek_kind() {
            None => true,
            Some(TokenKind::Eof) => true,
            _ => false,
        }
    }

    /// Span of the current token. If past the end of input,
    /// returns a zero-length span at the last token's end (or
    /// `Span::default()` if the slice is empty).
    pub fn current_span(&self) -> Span {
        if let Some(tok) = self.peek() {
            return tok.span;
        }
        if let Some(last) = self.tokens.last() {
            return Span::new(last.span.file, last.span.end, last.span.end);
        }
        Span::new(FileId(0), ByteOffset(0), ByteOffset(0))
    }

    /// Drive the parse to completion.
    ///
    /// Skeleton grammar:
    ///
    /// 1. Empty input / lone `Eof` → empty `Ast`.
    /// 2. `IntLit` (then optional `Eof` or end of slice) → an
    ///    `Ast` with one `AstNodeKind::Expr` root node spanning
    ///    the literal.
    /// 3. Anything else → `SyntaxErrorKind::UnexpectedToken`.
    pub fn parse(mut self) -> Result<Ast, SyntaxError> {
        if self.is_eof() {
            return Ok(self.ast);
        }

        let first_kind = self.peek_kind().expect("peek after is_eof false");
        if first_kind == TokenKind::IntLit {
            let span = self.current_span();
            let node = AstNode::leaf(AstNodeKind::Expr, span);
            let id = self.ast.push(node).ok_or_else(|| {
                SyntaxError::unexpected_token(span, "AST arena full")
            })?;
            self.ast.set_root(id);
            self.bump();

            if !self.is_eof() {
                return Err(SyntaxError::unexpected_token(
                    self.current_span(),
                    "expected end of input after literal",
                ));
            }
            return Ok(self.ast);
        }

        Err(SyntaxError::unexpected_token(
            self.current_span(),
            "unexpected token at start of input",
        ))
    }
}

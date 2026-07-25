//! Patterns.
//!
//! Patterns appear in far more places than `match`: `let`, `for`, closure
//! parameters, and function parameters all take one, which is why this is its
//! own production rather than a branch inside the match arm.

use notko::{Maybe, Outcome};

use crate::ast::{AstList, AstRef, LitKind, Name, Node};
use crate::token::{Keyword, TokenKind};

use super::{Gather, ParseError, Parser};

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    /// `Pattern ::= OrPattern`, the outermost level.
    pub fn parse_pat(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let first = self.parse_pat_single()?;
        if !self.is(TokenKind::Pipe) {
            return Outcome::Ok(first);
        }
        let mut alts = Gather::new();
        alts.push(first, start)?;
        while self.eat(TokenKind::Pipe) {
            let p = self.parse_pat_single()?;
            alts.push(p, start)?;
        }
        let list = self.list(alts.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::PatOr { alts: list }, span)
    }

    /// One alternative of an or-pattern.
    fn parse_pat_single(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;

        // `..` on its own is the rest pattern; with an operand it is a range.
        if self.is(TokenKind::DotDot) || self.is(TokenKind::DotDotEq) {
            let inclusive = self.at() == TokenKind::DotDotEq;
            self.bump();
            if self.pat_can_start() {
                let hi = self.parse_pat_single()?;
                let span = self.span_from(start);
                return self
                    .push(Node::PatRange { inclusive, lo: Maybe::Isnt, hi: Maybe::Is(hi) }, span);
            }
            let span = self.span_from(start);
            return self.push(Node::PatRest, span);
        }

        if self.eat(TokenKind::Amp) {
            let mutable = self.eat(TokenKind::Keyword(Keyword::Mut));
            let inner = self.parse_pat_single()?;
            let span = self.span_from(start);
            return self.push(Node::PatRef { mutable, inner }, span);
        }

        if self.eat(TokenKind::LParen) {
            if self.eat(TokenKind::RParen) {
                let span = self.span_from(start);
                return self.push(Node::PatTuple { elems: AstList::EMPTY }, span);
            }
            let mut elems = Gather::new();
            let mut tupled = false;
            loop {
                let p = self.parse_pat()?;
                elems.push(p, start)?;
                if self.eat(TokenKind::Comma) {
                    tupled = true;
                    if self.is(TokenKind::RParen) {
                        break;
                    }
                    continue;
                }
                break;
            }
            self.expect(TokenKind::RParen)?;
            // as with types, a parenthesised pattern with no trailing comma is
            // peeled rather than made a one-tuple
            if !tupled && elems.as_slice().len() == 1 {
                return Outcome::Ok(elems.as_slice()[0]);
            }
            let list = self.list(elems.as_slice(), start)?;
            let span = self.span_from(start);
            return self.push(Node::PatTuple { elems: list }, span);
        }

        // a literal, possibly the low end of a range
        if let Some(kind) = lit_kind(self.at()) {
            let t = self.bump();
            if self.is(TokenKind::DotDot) || self.is(TokenKind::DotDotEq) {
                let inclusive = self.at() == TokenKind::DotDotEq;
                self.bump();
                let hi = if self.pat_can_start() {
                    Maybe::Is(self.parse_pat_single()?)
                } else {
                    Maybe::Isnt
                };
                let lo = self.push(Node::PatLit { kind, span: t.span, range_end: Maybe::Isnt }, t.span)?;
                let span = self.span_from(start);
                return self.push(Node::PatRange { inclusive, lo: Maybe::Is(lo), hi }, span);
            }
            return self.push(Node::PatLit { kind, span: t.span, range_end: Maybe::Isnt }, t.span);
        }

        if self.is(TokenKind::Bang) {
            // `_` lexes as an identifier, so the wildcard is checked by text
            let t = self.peek();
            return Outcome::Err(ParseError::Unexpected { at: t.span, found: t.kind });
        }

        // `_`, a bare binding, or a path (possibly a tuple-struct pattern)
        if self.is(TokenKind::Ident) {
            let t = self.peek();
            if self.text(t.span) == "_" {
                self.bump();
                let span = self.span_from(start);
                return self.push(Node::PatWild, span);
            }
            // a lone identifier not followed by `::` or `(` is a binding
            let next = self.peek_at(1).kind;
            if next != TokenKind::ColonColon && next != TokenKind::LParen {
                self.bump();
                let span = self.span_from(start);
                return self.push(Node::PatIdent { name: Name(t.span) }, span);
            }
        }

        let path = self.parse_path(false)?;
        if self.eat(TokenKind::LParen) {
            let mut elems = Gather::new();
            while !self.is(TokenKind::RParen) {
                let p = self.parse_pat()?;
                elems.push(p, start)?;
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen)?;
            let list = self.list(elems.as_slice(), start)?;
            let span = self.span_from(start);
            return self.push(Node::PatTupleStruct { path, elems: list }, span);
        }
        let span = self.span_from(start);
        self.push(Node::PatPath { path }, span)
    }

    /// Whether a pattern can begin at the cursor, which is what decides whether
    /// a `..` has a high end.
    fn pat_can_start(&self) -> bool {
        matches!(
            self.at(),
            TokenKind::Ident
                | TokenKind::Int
                | TokenKind::Float
                | TokenKind::Str
                | TokenKind::RawStr
                | TokenKind::Char
                | TokenKind::LParen
                | TokenKind::Amp
                | TokenKind::ColonColon
        )
    }
}

/// The literal kind a token begins, if it begins one.
pub(crate) fn lit_kind(k: TokenKind) -> Option<LitKind> {
    Some(match k {
        TokenKind::Int => LitKind::Int,
        TokenKind::Float => LitKind::Float,
        TokenKind::Str => LitKind::Str,
        TokenKind::RawStr => LitKind::RawStr,
        TokenKind::Char => LitKind::Char,
        _ => return None,
    })
}

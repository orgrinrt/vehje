//! Paths and type expressions.
//!
//! Two productions that look small and carry the grammar's two sharpest
//! disambiguations: a parenthesised type with no trailing comma is NOT a
//! one-tuple, and generic arguments spell differently in type position (`<T>`)
//! and expression position (`::<T>`), because the second has to be
//! distinguishable from a less-than.

use notko::{Maybe, Outcome};

use crate::ast::{AstList, AstRef, Name, Node};
use crate::token::TokenKind;

use super::{Gather, ParseError, Parser};

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    /// `Path ::= [ "::" ] PathSeg { "::" PathSeg } [ PathGenerics ]`
    ///
    /// `turbofish` selects the expression-position spelling, where generic
    /// arguments must be introduced by `::` so they cannot be read as a
    /// less-than.
    pub fn parse_path(&mut self, turbofish: bool) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let leading_colon = self.eat(TokenKind::ColonColon);

        let mut segs = Gather::new();
        loop {
            let name = match self.at() {
                TokenKind::Ident => Name(self.bump().span),
                TokenKind::Keyword(k)
                    if matches!(
                        k,
                        crate::token::Keyword::LowerSelf
                            | crate::token::Keyword::UpperSelf
                            | crate::token::Keyword::Super
                            | crate::token::Keyword::Crate
                    ) =>
                {
                    Name(self.bump().span)
                }
                found => {
                    return Outcome::Err(ParseError::Unexpected { at: self.peek().span, found })
                }
            };
            let seg = self.push(Node::PathSeg { name }, name.0)?;
            segs.push(seg, name.0)?;

            // `::` continues the path only when a segment follows; `::<` is the
            // turbofish and belongs to the generics, not to the segment list.
            if self.is(TokenKind::ColonColon) && !matches!(self.peek_at(1).kind, TokenKind::Lt) {
                self.bump();
                continue;
            }
            break;
        }

        let generics = self.parse_generic_args(turbofish)?;
        let segs = self.list(segs.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::Path { leading_colon, segs, generics }, span)
    }

    /// The generic-argument list of a path, empty when there is none.
    fn parse_generic_args(&mut self, turbofish: bool) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        if turbofish {
            // expression position: only `::<` opens arguments
            if !(self.is(TokenKind::ColonColon) && self.peek_at(1).kind == TokenKind::Lt) {
                return Outcome::Ok(AstList::EMPTY);
            }
            self.bump();
        } else if !self.is(TokenKind::Lt) {
            return Outcome::Ok(AstList::EMPTY);
        }
        self.expect(TokenKind::Lt)?;

        let mut args = Gather::new();
        while !self.is(TokenKind::Gt) {
            let t = self.parse_ty()?;
            args.push(t, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Gt)?;
        self.list(args.as_slice(), start)
    }

    /// `TypeExpr ::= RefType | TupleType | PathType`
    pub fn parse_ty(&mut self) -> Outcome<AstRef, ParseError> {
        if let Maybe::Is(e) = self.refuse_reserved() {
            return Outcome::Err(e);
        }
        let start = self.peek().span;

        if self.eat(TokenKind::Amp) {
            let mutable = self.eat(TokenKind::Keyword(crate::token::Keyword::Mut));
            let inner = self.parse_ty()?;
            let span = self.span_from(start);
            return self.push(Node::TyRef { mutable, inner }, span);
        }

        if self.eat(TokenKind::LParen) {
            // `()` is unit, `(T)` is T with the parens dropped, `(T,)` is the
            // one-tuple. The trailing comma is the whole distinction.
            if self.eat(TokenKind::RParen) {
                let span = self.span_from(start);
                return self.push(Node::TyTuple { elems: AstList::EMPTY }, span);
            }
            let mut elems = Gather::new();
            let mut tupled = false;
            loop {
                let t = self.parse_ty()?;
                elems.push(t, start)?;
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
            if !tupled && elems.as_slice().len() == 1 {
                // a peeled type, not a tuple
                return Outcome::Ok(elems.as_slice()[0]);
            }
            let list = self.list(elems.as_slice(), start)?;
            let span = self.span_from(start);
            return self.push(Node::TyTuple { elems: list }, span);
        }

        let path = self.parse_path(false)?;
        let span = self.span_from(start);
        self.push(Node::TyPath { path }, span)
    }
}

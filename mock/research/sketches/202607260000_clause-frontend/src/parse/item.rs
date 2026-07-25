//! Items: the declaration forms, and the generics machinery they all share.
//!
//! Generics, bounds, and where clauses are the reason this file exists as its
//! own production rather than a branch per item: `fn`, `struct`, `enum`,
//! `trait`, `impl`, and `macro` all take the same three, so they are parsed
//! once here and threaded rather than repeated six times with six chances to
//! diverge.

use notko::{Maybe, Outcome};

use crate::ast::{AstList, AstRef, Node};
use crate::token::{Keyword, TokenKind};

use super::{Gather, ParseError, Parser};

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    /// `File ::= { InnerAttribute } { Item }`
    pub fn parse_file(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let mut attrs = Gather::new();
        while self.is(TokenKind::PoundBang) {
            let a = self.parse_attr()?;
            attrs.push(a, start)?;
        }
        let mut items = Gather::new();
        while !self.is(TokenKind::Eof) {
            let it = self.parse_item()?;
            items.push(it, start)?;
        }
        let attrs = self.list(attrs.as_slice(), start)?;
        let items = self.list(items.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::File { attrs, items }, span)
    }

    /// `#[...]` or `#![...]`, with the argument tokens kept as a span.
    pub(crate) fn parse_attr(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let inner = self.is(TokenKind::PoundBang);
        self.bump();
        self.expect(TokenKind::LBracket)?;
        let path = self.parse_path(false)?;
        let args = if self.is(TokenKind::LParen) {
            Maybe::Is(self.parse_delimited_tokens()?)
        } else {
            Maybe::Isnt
        };
        self.expect(TokenKind::RBracket)?;
        let span = self.span_from(start);
        self.push(Node::Attr { inner, path, args }, span)
    }

    /// `Item ::= { OuterAttribute } [ Visibility ] [ "sealed" ] ItemKind`
    pub fn parse_item(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let mut attrs = Gather::new();
        while self.is(TokenKind::Pound) {
            let a = self.parse_attr()?;
            attrs.push(a, start)?;
        }
        let attrs = self.list(attrs.as_slice(), start)?;

        // `sealed` may precede or follow `pub`; both orders are legal
        let mut sealed = self.eat(TokenKind::Keyword(Keyword::Sealed));
        let vis = self.parse_vis()?;
        if !sealed {
            sealed = self.eat(TokenKind::Keyword(Keyword::Sealed));
        }

        match self.at() {
            TokenKind::Keyword(Keyword::Expect) => {
                self.bump();
                let inner = self.parse_item()?;
                let span = self.span_from(start);
                self.push(Node::ItemExpect { inner }, span)
            }
            TokenKind::Keyword(Keyword::Actual) => {
                self.bump();
                let inner = self.parse_item()?;
                let span = self.span_from(start);
                self.push(Node::ItemActual { inner }, span)
            }
            TokenKind::Keyword(Keyword::Extern) => {
                self.bump();
                let inner = self.parse_item()?;
                let span = self.span_from(start);
                self.push(Node::ItemExtern { attrs, inner }, span)
            }
            TokenKind::Keyword(Keyword::Fn) => self.parse_fn(attrs, vis, start),
            TokenKind::Keyword(Keyword::Struct) => self.parse_struct(attrs, vis, sealed, start),
            TokenKind::Keyword(Keyword::Enum) => self.parse_enum(attrs, vis, start),
            TokenKind::Keyword(Keyword::Trait) => self.parse_trait(attrs, vis, sealed, start),
            TokenKind::Keyword(Keyword::Impl) => self.parse_impl(attrs, start),
            TokenKind::Keyword(Keyword::Type) => self.parse_type_alias(attrs, vis, start),
            TokenKind::Keyword(Keyword::Const) => self.parse_const(attrs, vis, start),
            TokenKind::Keyword(Keyword::Static) => self.parse_static(attrs, vis, start),
            TokenKind::Keyword(Keyword::Mod) => self.parse_mod(attrs, vis, start),
            TokenKind::Keyword(Keyword::Use) => self.parse_use(attrs, vis, start),
            TokenKind::Keyword(Keyword::Macro) => self.parse_macro_decl(attrs, vis, start),
            TokenKind::Keyword(Keyword::Event) => self.parse_event(attrs, start),
            TokenKind::Ident | TokenKind::ColonColon => {
                let path = self.parse_path(false)?;
                self.expect(TokenKind::Bang)?;
                let body = self.parse_delimited_tokens()?;
                let _ = self.eat(TokenKind::Semi);
                let span = self.span_from(start);
                self.push(Node::ItemMacroCall { attrs, path, body }, span)
            }
            found => Outcome::Err(ParseError::Unexpected { at: self.peek().span, found }),
        }
    }

    /// `[ "pub" [ "(" ( "crate" | "super" | "in" Path ) ")" ] ]`
    pub(crate) fn parse_vis(&mut self) -> Outcome<Maybe<AstRef>, ParseError> {
        let start = self.peek().span;
        if !self.eat(TokenKind::Keyword(Keyword::Pub)) {
            return Outcome::Ok(Maybe::Isnt);
        }
        let restriction = if self.eat(TokenKind::LParen) {
            let _ = self.eat(TokenKind::Keyword(Keyword::In));
            let p = self.parse_path(false)?;
            self.expect(TokenKind::RParen)?;
            Maybe::Is(p)
        } else {
            Maybe::Isnt
        };
        let span = self.span_from(start);
        match self.push(Node::Vis { restriction }, span) {
            Outcome::Ok(r) => Outcome::Ok(Maybe::Is(r)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    /// `Generics ::= "<" { GenericParam } ">"`, empty when absent.
    pub fn parse_generics(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        if !self.eat(TokenKind::Lt) {
            return Outcome::Ok(AstList::EMPTY);
        }
        let mut params = Gather::new();
        while !self.is(TokenKind::Gt) {
            let p_start = self.peek().span;
            if self.eat(TokenKind::Keyword(Keyword::Const)) {
                let name = self.expect_ident()?;
                self.expect(TokenKind::Colon)?;
                let ty = self.parse_ty()?;
                let span = self.span_from(p_start);
                let p = self.push(Node::ConstParam { name, ty }, span)?;
                params.push(p, start)?;
            } else {
                let name = self.expect_ident()?;
                let bounds = if self.eat(TokenKind::Colon) {
                    self.parse_bounds()?
                } else {
                    AstList::EMPTY
                };
                let span = self.span_from(p_start);
                let p = self.push(Node::GenericParam { name, bounds }, span)?;
                params.push(p, start)?;
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Gt)?;
        self.list(params.as_slice(), start)
    }

    /// `BoundList ::= TypeExpr { "+" TypeExpr }`
    pub fn parse_bounds(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        let mut bounds = Gather::new();
        loop {
            let t = self.parse_ty()?;
            bounds.push(t, start)?;
            if !self.eat(TokenKind::Plus) {
                break;
            }
        }
        self.list(bounds.as_slice(), start)
    }

    /// `WhereClause ::= "where" { WherePredicate }`, empty when absent.
    pub fn parse_where(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        if !self.eat(TokenKind::Keyword(Keyword::Where)) {
            return Outcome::Ok(AstList::EMPTY);
        }
        let mut preds = Gather::new();
        // a where clause runs until the body opens or the item ends
        while !self.is(TokenKind::LBrace) && !self.is(TokenKind::Semi) && !self.is(TokenKind::Eof) {
            let p_start = self.peek().span;
            let ty = self.parse_ty()?;
            self.expect(TokenKind::Colon)?;
            let bounds = self.parse_bounds()?;
            let span = self.span_from(p_start);
            let p = self.push(Node::WherePred { ty, bounds }, span)?;
            preds.push(p, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.list(preds.as_slice(), start)
    }

    /// The parenthesised parameter list, including `self` in its four spellings.
    pub(crate) fn parse_params(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::LParen)?;
        let mut params = Gather::new();
        while !self.is(TokenKind::RParen) {
            let p_start = self.peek().span;
            // `self`, `mut self`, `&self`, `&mut self`
            let is_self = matches!(self.at(), TokenKind::Keyword(Keyword::LowerSelf))
                || (self.is(TokenKind::Amp)
                    && matches!(
                        self.peek_at(1).kind,
                        TokenKind::Keyword(Keyword::LowerSelf)
                            | TokenKind::Keyword(Keyword::Mut)
                    ))
                || (self.is(TokenKind::Keyword(Keyword::Mut))
                    && matches!(self.peek_at(1).kind, TokenKind::Keyword(Keyword::LowerSelf)));
            if is_self {
                let _ = self.eat(TokenKind::Amp);
                let _ = self.eat(TokenKind::Keyword(Keyword::Mut));
                self.expect(TokenKind::Keyword(Keyword::LowerSelf))?;
                let span = self.span_from(p_start);
                let p = self.push(
                    Node::Param { self_param: true, name: Maybe::Isnt, ty: Maybe::Isnt },
                    span,
                )?;
                params.push(p, start)?;
            } else {
                let name = self.expect_ident()?;
                let ty =
                    if self.eat(TokenKind::Colon) { Maybe::Is(self.parse_ty()?) } else { Maybe::Isnt };
                let span = self.span_from(p_start);
                let p = self.push(
                    Node::Param { self_param: false, name: Maybe::Is(name), ty },
                    span,
                )?;
                params.push(p, start)?;
            }
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RParen)?;
        self.list(params.as_slice(), start)
    }

    /// A body that is either a block or a bare `;` signature.
    pub(crate) fn parse_body_or_semi(&mut self) -> Outcome<Maybe<AstRef>, ParseError> {
        if self.eat(TokenKind::Semi) {
            return Outcome::Ok(Maybe::Isnt);
        }
        match self.parse_block() {
            Outcome::Ok(b) => Outcome::Ok(Maybe::Is(b)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }
}

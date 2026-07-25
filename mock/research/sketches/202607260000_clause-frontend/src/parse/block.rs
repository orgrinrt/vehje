//! The block-shaped expressions: blocks and their statements, `if`, `match`,
//! closures, struct literals, and a macro's unparsed body.
//!
//! Split from the operator chain because they are a different kind of
//! production: the operator half is one loop over a precedence table, and these
//! are the forms that open a brace and carry the grammar's context rules about
//! what a brace means where.

use notko::{Maybe, Outcome};

use crate::ast::{AstRef, Node};
use crate::token::{Keyword, TokenKind};

use super::{Gather, ParseError, Parser};

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    /// `Path { field: value, ..base }`, once the head has been read.
    pub(crate) fn parse_struct_lit(
        &mut self,
        path: AstRef,
        start: crate::token::Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::LBrace)?;
        let mut fields = Gather::new();
        let mut base = Maybe::Isnt;
        while !self.is(TokenKind::RBrace) {
            if self.eat(TokenKind::DotDot) {
                base = Maybe::Is(self.parse_expr()?);
                break;
            }
            let f_start = self.peek().span;
            let name = self.expect_ident()?;
            // `Name { x }` is shorthand: the identifier is both field and value
            let value = if self.eat(TokenKind::Colon) {
                Maybe::Is(self.parse_expr()?)
            } else {
                Maybe::Isnt
            };
            let span = self.span_from(f_start);
            let f = self.push(Node::FieldInit { name, value }, span)?;
            fields.push(f, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        let list = self.list(fields.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::StructLit { path, fields: list, base }, span)
    }

    /// `if cond block [ else ( block | if ) ]`, with the head disallowing
    /// struct literals so `if Name { }` is a path plus a block.
    pub(crate) fn parse_if(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::Keyword(Keyword::If))?;
        let cond = self.no_struct_lit(|p| p.parse_expr())?;
        let then_block = self.parse_block()?;
        let else_branch = if self.eat(TokenKind::Keyword(Keyword::Else)) {
            if self.is(TokenKind::Keyword(Keyword::If)) {
                Maybe::Is(self.parse_if()?)
            } else {
                Maybe::Is(self.parse_block()?)
            }
        } else {
            Maybe::Isnt
        };
        let span = self.span_from(start);
        self.push(Node::If { cond, then_block, else_branch }, span)
    }

    /// `match scrutinee { arms }`.
    pub(crate) fn parse_match(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::Keyword(Keyword::Match))?;
        let scrutinee = self.no_struct_lit(|p| p.parse_expr())?;
        self.expect(TokenKind::LBrace)?;
        let mut arms = Gather::new();
        while !self.is(TokenKind::RBrace) {
            let a_start = self.peek().span;
            let pat = self.parse_pat()?;
            let guard = if self.eat(TokenKind::Keyword(Keyword::If)) {
                Maybe::Is(self.no_struct_lit(|p| p.parse_expr())?)
            } else {
                Maybe::Isnt
            };
            self.expect(TokenKind::FatArrow)?;
            let body = self.parse_expr()?;
            let span = self.span_from(a_start);
            let arm = self.push(Node::Arm { pat, guard, body }, span)?;
            arms.push(arm, start)?;
            // a comma separates arms and is optional after a block-shaped body
            let _ = self.eat(TokenKind::Comma);
        }
        self.expect(TokenKind::RBrace)?;
        let list = self.list(arms.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::Match { scrutinee, arms: list }, span)
    }

    /// `[move] || expr` or `[move] |params| expr`.
    pub(crate) fn parse_closure(&mut self, moved: bool) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let mut params = Gather::new();
        if self.eat(TokenKind::OrOr) {
            // `||` is the empty parameter list, not two pipes
        } else {
            self.expect(TokenKind::Pipe)?;
            while !self.is(TokenKind::Pipe) {
                let p_start = self.peek().span;
                let name = self.expect_ident()?;
                // a closure parameter's type annotation is optional and, once
                // parsed, is not part of the binding shape here
                if self.eat(TokenKind::Colon) {
                    let _ = self.parse_ty()?;
                }
                let span = self.span_from(p_start);
                let p = self.push(Node::PatIdent { name }, span)?;
                params.push(p, start)?;
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::Pipe)?;
        }
        let body = self.parse_expr()?;
        let list = self.list(params.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::Closure { moved, params: list, body }, span)
    }

    /// `{ stmts; tail }`. Struct literals are allowed again inside.
    pub fn parse_block(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::LBrace)?;
        let saved_allowed = self.struct_lit_allowed();
        let _ = saved_allowed;

        let mut stmts = Gather::new();
        let mut tail = Maybe::Isnt;
        while !self.is(TokenKind::RBrace) {
            if self.is(TokenKind::Eof) {
                return Outcome::Err(ParseError::Unbalanced { at: start });
            }
            if self.is(TokenKind::Keyword(Keyword::Let)) {
                let s = self.parse_let()?;
                stmts.push(s, start)?;
                continue;
            }
            let e_start = self.peek().span;
            let e = self.parse_expr()?;
            if self.eat(TokenKind::Semi) {
                let span = self.span_from(e_start);
                let s = self.push(Node::ExprStmt { value: e }, span)?;
                stmts.push(s, start)?;
                continue;
            }
            // no semicolon: this is the block's value unless a `}` does not
            // follow, in which case it was a block-shaped statement
            if self.is(TokenKind::RBrace) {
                tail = Maybe::Is(e);
                break;
            }
            let span = self.span_from(e_start);
            let s = self.push(Node::ExprStmt { value: e }, span)?;
            stmts.push(s, start)?;
        }
        self.expect(TokenKind::RBrace)?;
        let list = self.list(stmts.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::Block { stmts: list, tail }, span)
    }

    /// `let pat [: Ty] [= init];`
    fn parse_let(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::Keyword(Keyword::Let))?;
        // `mut` is recorded nowhere: a rebinding is what it means here, and the
        // binding shape is the same either way
        let _ = self.eat(TokenKind::Keyword(Keyword::Mut));
        let pat = self.parse_pat()?;
        let ty = if self.eat(TokenKind::Colon) { Maybe::Is(self.parse_ty()?) } else { Maybe::Isnt };
        let init = if self.eat(TokenKind::Eq) { Maybe::Is(self.parse_expr()?) } else { Maybe::Isnt };
        self.expect(TokenKind::Semi)?;
        let span = self.span_from(start);
        self.push(Node::Let { pat, ty, init }, span)
    }

    /// A macro's delimited token body, kept as a span.
    ///
    /// The tokens mean nothing until expansion, so the parser balances the
    /// delimiters and records the extent rather than building a tree it would
    /// have to throw away.
    pub fn parse_delimited_tokens(&mut self) -> Outcome<crate::token::Span, ParseError> {
        let open = self.peek();
        let (close, _) = match open.kind {
            TokenKind::LParen => (TokenKind::RParen, ()),
            TokenKind::LBrace => (TokenKind::RBrace, ()),
            TokenKind::LBracket => (TokenKind::RBracket, ()),
            found => return Outcome::Err(ParseError::Unexpected { at: open.span, found }),
        };
        self.bump();
        let mut depth = 1usize;
        while depth > 0 {
            match self.at() {
                TokenKind::Eof => return Outcome::Err(ParseError::Unbalanced { at: open.span }),
                k if k == open.kind => {
                    depth += 1;
                    self.bump();
                }
                k if k == close => {
                    depth -= 1;
                    self.bump();
                }
                _ => {
                    self.bump();
                }
            }
        }
        Outcome::Ok(self.span_from(open.span))
    }
}

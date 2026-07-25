//! The individual declaration forms.
//!
//! Split from `item.rs`, which keeps the dispatch and the three things every
//! declaration shares: generics, bounds, and where clauses. These are the
//! per-keyword bodies, and they read as a list because that is what they are.

use notko::{Maybe, Outcome};

use crate::ast::{AstList, AstRef, Node};
use crate::token::{Keyword, Span, TokenKind};

use super::{Gather, ParseError, Parser};

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    pub(crate) fn parse_fn(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Fn))?;
        let name = self.expect_ident()?;
        let generics = self.parse_generics()?;
        let params = self.parse_params()?;
        let ret = if self.eat(TokenKind::Arrow) { Maybe::Is(self.parse_ty()?) } else { Maybe::Isnt };
        let wheres = self.parse_where()?;
        let body = self.parse_body_or_semi()?;
        let span = self.span_from(start);
        self.push(Node::ItemFn { attrs, vis, name, generics, params, ret, wheres, body }, span)
    }

    pub(crate) fn parse_struct(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        sealed: bool,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Struct))?;
        let name = self.expect_ident()?;
        let generics = self.parse_generics()?;
        // the colon tail is the bind-target constraint, read and then erased
        let bind_target =
            if self.eat(TokenKind::Colon) { Maybe::Is(self.parse_ty()?) } else { Maybe::Isnt };
        let wheres = self.parse_where()?;
        self.expect(TokenKind::LBrace)?;
        let mut fields = Gather::new();
        while !self.is(TokenKind::RBrace) {
            let f_start = self.peek().span;
            let mut fattrs = Gather::new();
            while self.is(TokenKind::Pound) {
                let a = self.parse_attr()?;
                fattrs.push(a, f_start)?;
            }
            let fvis = self.parse_vis()?;
            let is_const = self.eat(TokenKind::Keyword(Keyword::Const));
            let is_mut = self.eat(TokenKind::Keyword(Keyword::Mut));
            let fname = self.expect_ident()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_ty()?;
            let default =
                if self.eat(TokenKind::Eq) { Maybe::Is(self.parse_expr()?) } else { Maybe::Isnt };
            let span = self.span_from(f_start);
            let f = self.push(
                Node::Field2 { vis: fvis, is_const, is_mut, name: fname, ty, default },
                span,
            )?;
            fields.push(f, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        let fields = self.list(fields.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(
            Node::ItemStruct { attrs, vis, sealed, name, generics, bind_target, wheres, fields },
            span,
        )
    }

    pub(crate) fn parse_enum(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Enum))?;
        let name = self.expect_ident()?;
        let generics = self.parse_generics()?;
        let wheres = self.parse_where()?;
        self.expect(TokenKind::LBrace)?;
        let mut variants = Gather::new();
        while !self.is(TokenKind::RBrace) {
            let v_start = self.peek().span;
            while self.is(TokenKind::Pound) {
                let _ = self.parse_attr()?;
            }
            let vname = self.expect_ident()?;
            let payload = if self.eat(TokenKind::LParen) {
                let mut tys = Gather::new();
                while !self.is(TokenKind::RParen) {
                    let t = self.parse_ty()?;
                    tys.push(t, v_start)?;
                    if !self.eat(TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(TokenKind::RParen)?;
                self.list(tys.as_slice(), v_start)?
            } else {
                AstList::EMPTY
            };
            let span = self.span_from(v_start);
            let v = self.push(Node::Variant { name: vname, payload }, span)?;
            variants.push(v, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        let variants = self.list(variants.as_slice(), start)?;
        let span = self.span_from(start);
        self.push(Node::ItemEnum { attrs, vis, name, generics, wheres, variants }, span)
    }

    pub(crate) fn parse_trait(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        sealed: bool,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Trait))?;
        let name = self.expect_ident()?;
        let generics = self.parse_generics()?;
        let supertraits =
            if self.eat(TokenKind::Colon) { self.parse_bounds()? } else { AstList::EMPTY };
        let wheres = self.parse_where()?;
        let items = self.parse_item_body()?;
        let span = self.span_from(start);
        self.push(
            Node::ItemTrait { attrs, vis, sealed, name, generics, supertraits, wheres, items },
            span,
        )
    }

    pub(crate) fn parse_impl(&mut self, attrs: AstList, start: Span) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Impl))?;
        let generics = self.parse_generics()?;
        let ty = self.parse_ty()?;
        let for_ty = if self.eat(TokenKind::Keyword(Keyword::For)) {
            Maybe::Is(self.parse_ty()?)
        } else {
            Maybe::Isnt
        };
        let wheres = self.parse_where()?;
        let items = self.parse_item_body()?;
        let span = self.span_from(start);
        self.push(Node::ItemImpl { attrs, generics, ty, for_ty, wheres, items }, span)
    }

    /// `{ { Item } }`, the shared body of a trait or an impl.
    pub(crate) fn parse_item_body(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::LBrace)?;
        let mut items = Gather::new();
        while !self.is(TokenKind::RBrace) {
            if self.is(TokenKind::Eof) {
                return Outcome::Err(ParseError::Unbalanced { at: start });
            }
            let it = self.parse_item()?;
            items.push(it, start)?;
        }
        self.expect(TokenKind::RBrace)?;
        self.list(items.as_slice(), start)
    }

    pub(crate) fn parse_type_alias(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Type))?;
        let name = self.expect_ident()?;
        // no `= Ty` makes this an associated-type DECLARATION, legal only in a
        // trait body; where it is legal is the checker's call, not the parser's
        let ty = if self.eat(TokenKind::Eq) { Maybe::Is(self.parse_ty()?) } else { Maybe::Isnt };
        self.expect(TokenKind::Semi)?;
        let span = self.span_from(start);
        self.push(Node::ItemTypeAlias { attrs, vis, name, ty }, span)
    }

    pub(crate) fn parse_const(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Const))?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_ty()?;
        let value = if self.eat(TokenKind::Eq) { Maybe::Is(self.parse_expr()?) } else { Maybe::Isnt };
        self.expect(TokenKind::Semi)?;
        let span = self.span_from(start);
        self.push(Node::ItemConst { attrs, vis, name, ty, value }, span)
    }

    pub(crate) fn parse_static(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Static))?;
        let mutable = self.eat(TokenKind::Keyword(Keyword::Mut));
        let name = self.expect_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_ty()?;
        self.expect(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semi)?;
        let span = self.span_from(start);
        self.push(Node::ItemStatic { attrs, vis, mutable, name, ty, value }, span)
    }

    pub(crate) fn parse_mod(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Mod))?;
        let path = self.parse_path(false)?;
        let items = if self.eat(TokenKind::Semi) {
            Maybe::Isnt
        } else {
            Maybe::Is(self.parse_item_body()?)
        };
        let span = self.span_from(start);
        self.push(Node::ItemMod { attrs, vis, path, items }, span)
    }

    pub(crate) fn parse_use(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Use))?;
        let tree = self.parse_use_tree()?;
        self.expect(TokenKind::Semi)?;
        let span = self.span_from(start);
        self.push(Node::ItemUse { attrs, vis, tree }, span)
    }

    /// One `use` tree: a path, then a rename, a glob, or a brace of children.
    pub(crate) fn parse_use_tree(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        if self.is(TokenKind::LBrace) {
            let children = self.parse_use_group()?;
            let span = self.span_from(start);
            return self.push(
                Node::UseTree { path: Maybe::Isnt, alias: Maybe::Isnt, glob: false, children },
                span,
            );
        }
        let path = self.parse_path(false)?;
        if self.eat(TokenKind::ColonColon) {
            if self.eat(TokenKind::Star) {
                let span = self.span_from(start);
                return self.push(
                    Node::UseTree {
                        path: Maybe::Is(path),
                        alias: Maybe::Isnt,
                        glob: true,
                        children: AstList::EMPTY,
                    },
                    span,
                );
            }
            let children = self.parse_use_group()?;
            let span = self.span_from(start);
            return self.push(
                Node::UseTree { path: Maybe::Is(path), alias: Maybe::Isnt, glob: false, children },
                span,
            );
        }
        let alias = if self.eat(TokenKind::Keyword(Keyword::As)) {
            Maybe::Is(self.expect_ident()?)
        } else {
            Maybe::Isnt
        };
        let span = self.span_from(start);
        self.push(
            Node::UseTree { path: Maybe::Is(path), alias, glob: false, children: AstList::EMPTY },
            span,
        )
    }

    pub(crate) fn parse_use_group(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::LBrace)?;
        let mut kids = Gather::new();
        while !self.is(TokenKind::RBrace) {
            let t = self.parse_use_tree()?;
            kids.push(t, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        self.list(kids.as_slice(), start)
    }

    pub(crate) fn parse_macro_decl(
        &mut self,
        attrs: AstList,
        vis: Maybe<AstRef>,
        start: Span,
    ) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Macro))?;
        let name = self.expect_ident()?;
        let generics = self.parse_generics()?;
        let params = self.parse_params()?;
        // the return type is mandatory, which is what makes a macro a typed
        // function at a compile stage rather than a token rewriter
        self.expect(TokenKind::Arrow)?;
        let ret = self.parse_ty()?;
        let _ = self.parse_where()?;
        let body = self.parse_block()?;
        let span = self.span_from(start);
        self.push(Node::ItemMacro { attrs, vis, name, generics, params, ret, body }, span)
    }

    pub(crate) fn parse_event(&mut self, attrs: AstList, start: Span) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Event))?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Keyword(Keyword::For))?;
        let for_ty = self.parse_ty()?;
        let body = self.parse_block()?;
        let span = self.span_from(start);
        self.push(Node::ItemEvent { attrs, name, for_ty, body }, span)
    }
}

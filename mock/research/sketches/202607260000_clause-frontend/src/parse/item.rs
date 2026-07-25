//! Items: the declaration forms, and the generics machinery they all share.
//!
//! Generics, bounds, and where clauses are the reason this file exists as its
//! own production rather than a branch per item: `fn`, `struct`, `enum`,
//! `trait`, `impl`, and `macro` all take the same three, so they are parsed
//! once here and threaded rather than repeated six times with six chances to
//! diverge.

use notko::{Maybe, Outcome};

use crate::ast::{AstList, AstRef, Node};
use crate::token::{Keyword, Span, TokenKind};

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
    fn parse_attr(&mut self) -> Outcome<AstRef, ParseError> {
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
    fn parse_vis(&mut self) -> Outcome<Maybe<AstRef>, ParseError> {
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
    fn parse_params(&mut self) -> Outcome<AstList, ParseError> {
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
    fn parse_body_or_semi(&mut self) -> Outcome<Maybe<AstRef>, ParseError> {
        if self.eat(TokenKind::Semi) {
            return Outcome::Ok(Maybe::Isnt);
        }
        match self.parse_block() {
            Outcome::Ok(b) => Outcome::Ok(Maybe::Is(b)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    fn parse_fn(
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

    fn parse_struct(
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

    fn parse_enum(
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

    fn parse_trait(
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

    fn parse_impl(&mut self, attrs: AstList, start: Span) -> Outcome<AstRef, ParseError> {
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
    fn parse_item_body(&mut self) -> Outcome<AstList, ParseError> {
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

    fn parse_type_alias(
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

    fn parse_const(
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

    fn parse_static(
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

    fn parse_mod(
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

    fn parse_use(
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
    fn parse_use_tree(&mut self) -> Outcome<AstRef, ParseError> {
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

    fn parse_use_group(&mut self) -> Outcome<AstList, ParseError> {
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

    fn parse_macro_decl(
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

    fn parse_event(&mut self, attrs: AstList, start: Span) -> Outcome<AstRef, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Event))?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Keyword(Keyword::For))?;
        let for_ty = self.parse_ty()?;
        let body = self.parse_block()?;
        let span = self.span_from(start);
        self.push(Node::ItemEvent { attrs, name, for_ty, body }, span)
    }
}

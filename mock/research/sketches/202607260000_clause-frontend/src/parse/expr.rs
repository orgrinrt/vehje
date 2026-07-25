//! Expressions: the precedence chain, the postfix chain, and blocks.
//!
//! The grammar writes the binary operators as a ladder of productions from
//! `MulExpr` up to `OrExpr`. Precedence climbing is the same grammar with the
//! ladder folded into a loop over a precedence table, which is what this does:
//! one function instead of nine, and the table is the specification.

use notko::{Maybe, Outcome};

use crate::ast::{AstList, AstRef, BinOp, Name, Node, UnOp};
use crate::token::{Keyword, TokenKind};

use super::pat::lit_kind;
use super::{Gather, ParseError, Parser};

/// The binding power of a binary operator, higher binding tighter.
///
/// Reads as the grammar's ladder: multiplicative at the top, then additive,
/// shifts, the three bitwise levels, comparison, and the two logical levels.
fn bin_op(k: TokenKind) -> Option<(BinOp, u8)> {
    Some(match k {
        TokenKind::Star => (BinOp::Mul, 10),
        TokenKind::Slash => (BinOp::Div, 10),
        TokenKind::Percent => (BinOp::Rem, 10),
        TokenKind::Plus => (BinOp::Add, 9),
        TokenKind::Minus => (BinOp::Sub, 9),
        TokenKind::Shl => (BinOp::Shl, 8),
        TokenKind::Shr => (BinOp::Shr, 8),
        TokenKind::Amp => (BinOp::BitAnd, 7),
        TokenKind::Caret => (BinOp::BitXor, 6),
        TokenKind::Pipe => (BinOp::BitOr, 5),
        TokenKind::EqEq => (BinOp::Eq, 4),
        TokenKind::Ne => (BinOp::Ne, 4),
        TokenKind::Lt => (BinOp::Lt, 4),
        TokenKind::Gt => (BinOp::Gt, 4),
        TokenKind::Le => (BinOp::Le, 4),
        TokenKind::Ge => (BinOp::Ge, 4),
        TokenKind::AndAnd => (BinOp::And, 3),
        TokenKind::OrOr => (BinOp::Or, 2),
        _ => return None,
    })
}

/// The operator a compound assignment applies, if the token is one.
fn assign_op(k: TokenKind) -> Option<Option<BinOp>> {
    Some(match k {
        TokenKind::Eq => None,
        TokenKind::PlusEq => Some(BinOp::Add),
        TokenKind::MinusEq => Some(BinOp::Sub),
        TokenKind::StarEq => Some(BinOp::Mul),
        TokenKind::SlashEq => Some(BinOp::Div),
        TokenKind::PercentEq => Some(BinOp::Rem),
        TokenKind::AmpEq => Some(BinOp::BitAnd),
        TokenKind::PipeEq => Some(BinOp::BitOr),
        TokenKind::CaretEq => Some(BinOp::BitXor),
        TokenKind::ShlEq => Some(BinOp::Shl),
        TokenKind::ShrEq => Some(BinOp::Shr),
        _ => return None,
    })
}

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    /// `Expr ::= AssignExpr`, the outermost level.
    pub fn parse_expr(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let lhs = self.parse_range()?;
        if let Some(op) = assign_op(self.at()) {
            self.bump();
            let value = self.parse_expr()?;
            let span = self.span_from(start);
            let op = match op {
                Some(o) => Maybe::Is(o),
                None => Maybe::Isnt,
            };
            return self.push(Node::Assign { op, place: lhs, value }, span);
        }
        Outcome::Ok(lhs)
    }

    /// `RangeExpr`, which sits between assignment and the operator ladder and
    /// may have either end missing.
    fn parse_range(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;

        if self.is(TokenKind::DotDot) || self.is(TokenKind::DotDotEq) {
            let inclusive = self.at() == TokenKind::DotDotEq;
            self.bump();
            let hi = if self.expr_can_start() {
                Maybe::Is(self.parse_binary(0)?)
            } else {
                Maybe::Isnt
            };
            let span = self.span_from(start);
            return self.push(Node::Range { inclusive, lo: Maybe::Isnt, hi }, span);
        }

        let lo = self.parse_binary(0)?;
        if self.is(TokenKind::DotDot) || self.is(TokenKind::DotDotEq) {
            let inclusive = self.at() == TokenKind::DotDotEq;
            self.bump();
            let hi = if self.expr_can_start() {
                Maybe::Is(self.parse_binary(0)?)
            } else {
                Maybe::Isnt
            };
            let span = self.span_from(start);
            return self.push(Node::Range { inclusive, lo: Maybe::Is(lo), hi }, span);
        }
        Outcome::Ok(lo)
    }

    /// The operator ladder, folded into precedence climbing.
    fn parse_binary(&mut self, min_bp: u8) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let mut lhs = self.parse_unary()?;
        loop {
            let (op, bp) = match bin_op(self.at()) {
                Some(x) => x,
                None => break,
            };
            if bp < min_bp {
                break;
            }
            self.bump();
            // every operator here is left-associative, so the right side binds
            // one level tighter
            let rhs = self.parse_binary(bp + 1)?;
            let span = self.span_from(start);
            lhs = self.push(Node::Binary { op, lhs, rhs }, span)?;
        }
        Outcome::Ok(lhs)
    }

    /// `Unary ::= ( "-" | "!" | "&" [ "mut" ] ) Unary | Postfix`
    fn parse_unary(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let op = match self.at() {
            TokenKind::Minus => UnOp::Neg,
            TokenKind::Bang => UnOp::Not,
            TokenKind::Amp => {
                self.bump();
                let mutable = self.eat(TokenKind::Keyword(Keyword::Mut));
                let operand = self.parse_unary()?;
                let span = self.span_from(start);
                let op = if mutable { UnOp::RefMut } else { UnOp::Ref };
                return self.push(Node::Unary { op, operand }, span);
            }
            _ => return self.parse_postfix(),
        };
        self.bump();
        let operand = self.parse_unary()?;
        let span = self.span_from(start);
        self.push(Node::Unary { op, operand }, span)
    }

    /// `Postfix ::= Primary { PostfixOp }`
    fn parse_postfix(&mut self) -> Outcome<AstRef, ParseError> {
        let start = self.peek().span;
        let mut base = self.parse_primary()?;
        loop {
            match self.at() {
                TokenKind::Dot => {
                    self.bump();
                    let name = self.expect_ident()?;
                    if self.is(TokenKind::LParen) {
                        let args = self.parse_args()?;
                        let span = self.span_from(start);
                        base = self.push(Node::MethodCall { base, name, args }, span)?;
                    } else {
                        let span = self.span_from(start);
                        base = self.push(Node::Field { base, name }, span)?;
                    }
                }
                TokenKind::LParen => {
                    let args = self.parse_args()?;
                    let span = self.span_from(start);
                    base = self.push(Node::Call { callee: base, args }, span)?;
                }
                TokenKind::LBracket => {
                    self.bump();
                    let index = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    let span = self.span_from(start);
                    base = self.push(Node::Index { base, index }, span)?;
                }
                TokenKind::Question => {
                    self.bump();
                    let span = self.span_from(start);
                    base = self.push(Node::Question { value: base }, span)?;
                }
                _ => break,
            }
        }
        Outcome::Ok(base)
    }

    /// A parenthesised argument list, with the grammar's keyword-argument form.
    fn parse_args(&mut self) -> Outcome<AstList, ParseError> {
        let start = self.peek().span;
        self.expect(TokenKind::LParen)?;
        let mut args = Gather::new();
        while !self.is(TokenKind::RParen) {
            let a_start = self.peek().span;
            // `name: value` is the keyword form; a lone `name` is an expression
            let name = if self.is(TokenKind::Ident) && self.peek_at(1).kind == TokenKind::Colon {
                let n = Name(self.bump().span);
                self.bump();
                Maybe::Is(n)
            } else {
                Maybe::Isnt
            };
            let value = self.parse_expr()?;
            let span = self.span_from(a_start);
            let arg = self.push(Node::Arg { name, value }, span)?;
            args.push(arg, start)?;
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RParen)?;
        self.list(args.as_slice(), start)
    }

    /// Whether an expression can begin at the cursor.
    fn expr_can_start(&self) -> bool {
        !matches!(
            self.at(),
            TokenKind::RParen
                | TokenKind::RBrace
                | TokenKind::RBracket
                | TokenKind::Comma
                | TokenKind::Semi
                | TokenKind::Eof
                | TokenKind::FatArrow
        )
    }

    /// `Primary`, including the block-shaped expressions.
    fn parse_primary(&mut self) -> Outcome<AstRef, ParseError> {
        if let Maybe::Is(e) = self.refuse_reserved() {
            return Outcome::Err(e);
        }
        let start = self.peek().span;

        if let Some(kind) = lit_kind(self.at()) {
            let t = self.bump();
            return self.push(Node::Lit { kind, span: t.span }, t.span);
        }

        match self.at() {
            TokenKind::LBrace => return self.parse_block(),
            TokenKind::LParen => {
                self.bump();
                if self.eat(TokenKind::RParen) {
                    let span = self.span_from(start);
                    return self.push(Node::ExprTuple { elems: AstList::EMPTY }, span);
                }
                // inside parens, struct literals are allowed again
                let mut elems = Gather::new();
                let mut tupled = false;
                loop {
                    let e = self.parse_expr()?;
                    elems.push(e, start)?;
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
                    return Outcome::Ok(elems.as_slice()[0]);
                }
                let list = self.list(elems.as_slice(), start)?;
                let span = self.span_from(start);
                return self.push(Node::ExprTuple { elems: list }, span);
            }
            TokenKind::Pipe | TokenKind::OrOr => return self.parse_closure(false),
            TokenKind::Keyword(Keyword::Move) => {
                self.bump();
                return self.parse_closure(true);
            }
            TokenKind::Keyword(Keyword::If) => return self.parse_if(),
            TokenKind::Keyword(Keyword::Match) => return self.parse_match(),
            TokenKind::Keyword(Keyword::While) => {
                self.bump();
                let cond = self.no_struct_lit(|p| p.parse_expr())?;
                let body = self.parse_block()?;
                let span = self.span_from(start);
                return self.push(Node::While { cond, body }, span);
            }
            TokenKind::Keyword(Keyword::Loop) => {
                self.bump();
                let body = self.parse_block()?;
                let span = self.span_from(start);
                return self.push(Node::Loop { body }, span);
            }
            TokenKind::Keyword(Keyword::For) => {
                self.bump();
                let pat = self.parse_pat()?;
                self.expect(TokenKind::Keyword(Keyword::In))?;
                let iter = self.no_struct_lit(|p| p.parse_expr())?;
                let body = self.parse_block()?;
                let span = self.span_from(start);
                return self.push(Node::For { pat, iter, body }, span);
            }
            TokenKind::Keyword(Keyword::Return) => {
                self.bump();
                let value =
                    if self.expr_can_start() { Maybe::Is(self.parse_expr()?) } else { Maybe::Isnt };
                let span = self.span_from(start);
                return self.push(Node::Return { value }, span);
            }
            TokenKind::Keyword(Keyword::Break) => {
                self.bump();
                let value =
                    if self.expr_can_start() { Maybe::Is(self.parse_expr()?) } else { Maybe::Isnt };
                let span = self.span_from(start);
                return self.push(Node::Break { value }, span);
            }
            TokenKind::Keyword(Keyword::Continue) => {
                self.bump();
                let span = self.span_from(start);
                return self.push(Node::Continue, span);
            }
            _ => {}
        }

        // a path, then possibly a macro call or a struct literal
        let path = self.parse_path(true)?;

        if self.is(TokenKind::Bang) {
            self.bump();
            let body = self.parse_delimited_tokens()?;
            let span = self.span_from(start);
            return self.push(Node::MacroCall { path, body }, span);
        }

        if self.is(TokenKind::LBrace) && self.struct_lit_allowed() {
            return self.parse_struct_lit(path, start);
        }

        let span = self.span_from(start);
        self.push(Node::ExprPath { path }, span)
    }
}

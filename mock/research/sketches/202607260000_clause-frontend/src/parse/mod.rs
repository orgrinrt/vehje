//! The parser: tokens to the AST arena.
//!
//! A recursive-descent parser with precedence climbing for the operator chain,
//! which is the shape the grammar is written in. No allocation: a production
//! that gathers a variable number of children collects into a fixed-capacity
//! stack buffer and refuses past it, the same lent-bound discipline the arenas
//! use.

pub mod block;
pub mod expr;
pub mod pat;
pub mod ty;

use arvo::USize;
use notko::{Maybe, Outcome};

use crate::ast::{Arena, AstList, AstRef, Name, Node};
use crate::token::{Keyword, Span, Token, TokenKind};

/// The most children one production may gather.
pub const LIST_CAP: usize = 64;

/// What can go wrong reading tokens.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ParseError {
    /// A token the grammar does not accept here.
    Unexpected { at: Span, found: TokenKind },
    /// A word the grammar reserves without accepting anywhere. Named apart from
    /// `Unexpected` so the message can say "reserved" rather than "unexpected",
    /// which is the difference between a typo and a deliberate refusal.
    ReservedWord { at: Span, word: Keyword },
    /// The AST arena or its pool is full.
    ArenaFull { at: Span },
    /// A production gathered more children than the lent buffer holds.
    TooManyChildren { at: Span },
    /// A bracket, brace, or paren with no partner.
    Unbalanced { at: Span },
}

/// The token cursor plus the arena being filled.
pub struct Parser<'t, 'a, 's> {
    toks: &'t [Token],
    pos: usize,
    src: &'s str,
    arena: Arena<'a>,
    /// Whether a `Path { ... }` here is a struct literal.
    ///
    /// False in the head of `if`, `while`, `for`, and `match`, and true again
    /// inside a nested block. This is the grammar's own disambiguation and the
    /// reason `if Name { }` parses as a path plus a block rather than as a
    /// struct literal with an empty body.
    allow_struct_lit: bool,
}

impl<'t, 'a, 's> Parser<'t, 'a, 's> {
    /// Start at the first token.
    pub fn new(toks: &'t [Token], src: &'s str, arena: Arena<'a>) -> Self {
        Self { toks, pos: 0, src, arena, allow_struct_lit: true }
    }

    /// Give the filled arena back.
    pub fn into_arena(self) -> Arena<'a> {
        self.arena
    }

    /// The token under the cursor.
    pub fn peek(&self) -> Token {
        self.toks[self.pos.min(self.toks.len() - 1)]
    }

    /// The token `n` ahead.
    pub fn peek_at(&self, n: usize) -> Token {
        self.toks[(self.pos + n).min(self.toks.len() - 1)]
    }

    /// The kind under the cursor.
    pub fn at(&self) -> TokenKind {
        self.peek().kind
    }

    /// Whether the cursor is on `kind`.
    pub fn is(&self, kind: TokenKind) -> bool {
        self.at() == kind
    }

    /// Step past the current token, returning it.
    pub fn bump(&mut self) -> Token {
        let t = self.peek();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    /// Step past `kind` if it is here, reporting whether it was.
    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.is(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Require `kind`, or refuse naming what was found instead.
    pub fn expect(&mut self, kind: TokenKind) -> Outcome<Token, ParseError> {
        if self.is(kind) {
            Outcome::Ok(self.bump())
        } else {
            let t = self.peek();
            Outcome::Err(ParseError::Unexpected { at: t.span, found: t.kind })
        }
    }

    /// Require an identifier, returning its name.
    pub fn expect_ident(&mut self) -> Outcome<Name, ParseError> {
        match self.expect(TokenKind::Ident) {
            Outcome::Ok(t) => Outcome::Ok(Name(t.span)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    /// The source text a span names.
    pub fn text(&self, span: Span) -> &'s str {
        span.of(self.src)
    }

    /// Append a node, turning arena exhaustion into a refusal.
    pub fn push(&mut self, node: Node, span: Span) -> Outcome<AstRef, ParseError> {
        match self.arena.push(node, span) {
            Maybe::Is(r) => Outcome::Ok(r),
            Maybe::Isnt => Outcome::Err(ParseError::ArenaFull { at: span }),
        }
    }

    /// Append a child list, likewise.
    pub fn list(&mut self, refs: &[AstRef], span: Span) -> Outcome<AstList, ParseError> {
        match self.arena.alloc_list(refs) {
            Maybe::Is(l) => Outcome::Ok(l),
            Maybe::Isnt => Outcome::Err(ParseError::ArenaFull { at: span }),
        }
    }

    /// Run `f` with struct literals disallowed, which is what the heads of
    /// `if`, `while`, `for`, and `match` need.
    pub fn no_struct_lit<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Outcome<T, ParseError>,
    ) -> Outcome<T, ParseError> {
        let saved = self.allow_struct_lit;
        self.allow_struct_lit = false;
        let out = f(self);
        self.allow_struct_lit = saved;
        out
    }

    /// Whether a `Path { ... }` here is a struct literal.
    pub fn struct_lit_allowed(&self) -> bool {
        self.allow_struct_lit
    }

    /// A span covering `from` through the previous token.
    pub fn span_from(&self, from: Span) -> Span {
        let end = if self.pos == 0 { from.end } else { self.toks[self.pos - 1].span.end };
        Span::new(from.start, end)
    }

    /// Refuse a reserved-but-unproductive word by name.
    pub fn refuse_reserved(&self) -> Maybe<ParseError> {
        if let TokenKind::Keyword(w) = self.at() {
            if w.is_reserved_unused() {
                return Maybe::Is(ParseError::ReservedWord { at: self.peek().span, word: w });
            }
        }
        Maybe::Isnt
    }
}

/// A fixed-capacity gather buffer for a production's children.
pub struct Gather {
    items: [AstRef; LIST_CAP],
    len: usize,
}

impl Gather {
    /// An empty buffer.
    pub fn new() -> Self {
        Self { items: [AstRef(USize(0)); LIST_CAP], len: 0 }
    }

    /// Append, refusing past the lent bound.
    pub fn push(&mut self, r: AstRef, at: Span) -> Outcome<(), ParseError> {
        if self.len >= LIST_CAP {
            return Outcome::Err(ParseError::TooManyChildren { at });
        }
        self.items[self.len] = r;
        self.len += 1;
        Outcome::Ok(())
    }

    /// What was gathered.
    pub fn as_slice(&self) -> &[AstRef] {
        &self.items[..self.len]
    }
}

impl Default for Gather {
    fn default() -> Self {
        Self::new()
    }
}

//! Lowering: the AST to the vehje Core.
//!
//! Most of the surface disappears here, which is the point. The Core has
//! thirteen evaluation forms and the grammar has far more syntax, so lowering
//! is mostly desugaring: a multi-parameter function becomes nested lambdas
//! because Core application takes one argument at a time, a block becomes
//! nested `Let`s, and every operator becomes a family operation because the
//! Core has no `+` and the framework owns no family.
//!
//! Names: the surface spells them, and the Core matches binders by `Sym`. The
//! front end interns a name to a `Str` and widens it, so two occurrences of the
//! same spelling agree by construction rather than by a lookup table.

pub mod expr;
pub mod item;

use arvo::{Bool, USize};
use hilavitkutin_str::{ArenaInterner, StringInterner};
use hilavitkutin_sym::Sym;
use notko::{Maybe, Outcome};
use vehje_ir::{Builder, FamilyId, NodeRef, Span as IrSpan};

use crate::ast::{Arena as AstArena, AstRef, Name, Node};

/// The family every operator and host service arrives through.
///
/// The framework defines no family, so the language declares its own. One id
/// serves all of them and the operation rides as the first operand, which keeps
/// the family surface a single number rather than one id per operator.
pub const ARITH: u16 = 1;

/// Operation codes within [`ARITH`], carried as the first operand.
pub mod op {
    pub const ADD: i64 = 1;
    pub const SUB: i64 = 2;
    pub const MUL: i64 = 3;
    pub const DIV: i64 = 4;
    pub const REM: i64 = 5;
    pub const EQ: i64 = 6;
    pub const NE: i64 = 7;
    pub const LT: i64 = 8;
    pub const GT: i64 = 9;
    pub const LE: i64 = 10;
    pub const GE: i64 = 11;
    pub const AND: i64 = 12;
    pub const OR: i64 = 13;
    pub const NEG: i64 = 14;
    pub const NOT: i64 = 15;
    pub const CONCAT: i64 = 16;
    /// Build a record from alternating key and value operands.
    pub const RECORD: i64 = 17;
    /// Build a sequence from its operands.
    pub const SEQ: i64 = 18;
}

/// What can go wrong lowering.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LowerError {
    /// The IR arena or one of its regions is full.
    ArenaFull,
    /// The interner cannot take another name.
    InternerFull,
    /// A surface form with no lowering yet, named rather than silently dropped.
    Unsupported { what: &'static str },
    /// A production gathered more children than the lent buffer holds.
    TooManyChildren,
    /// A path used as a value that names more than one segment.
    ///
    /// Multi-segment paths resolve to one binder in a real resolve pass; until
    /// that exists, lowering refuses rather than guessing which segment wins.
    UnresolvedPath,
}

/// Turn an arena refusal into a lowering refusal.
///
/// A free function rather than a method: as a method it would borrow `self`
/// immutably while the builder call inside it needs `self` mutably, which is a
/// conflict the shape avoids rather than works around.
pub fn ok(m: Maybe<NodeRef>) -> Outcome<NodeRef, LowerError> {
    match m {
        Maybe::Is(r) => Outcome::Ok(r),
        Maybe::Isnt => Outcome::Err(LowerError::ArenaFull),
    }
}

/// The most children one lowered form may gather.
pub const CAP: usize = 32;

/// The lowering context: the AST being read, the IR being built, and the
/// interner that gives a spelling its stable identity.
pub struct Lower<'ast, 'b, 'ir, 'i, A: ArenaInterner> {
    pub ast: &'ast AstArena<'ast>,
    pub src: &'ast str,
    /// The builder, borrowed for `'b` over an arena that lives for `'ir`.
    ///
    /// Two lifetimes rather than one: `&'b mut Builder<'b>` would force the
    /// borrow to last as long as the arena itself, which stops a caller from
    /// taking the arena back once lowering is done.
    pub b: &'b mut Builder<'ir>,
    pub interner: &'i mut StringInterner<A>,
}

impl<'ast, 'b, 'ir, 'i, A: ArenaInterner> Lower<'ast, 'b, 'ir, 'i, A> {
    /// The `Sym` a surface name lowers to.
    ///
    /// Interning is what makes two occurrences of one spelling the same binder
    /// without a scope table: the identity is the interned handle.
    pub fn sym(&mut self, name: Name) -> Outcome<Sym, LowerError> {
        let text = name.0.of(self.src);
        Outcome::Ok(self.interner.intern(text).as_sym())
    }

    /// A `Sym` for a name that never appears in the source, used where the Core
    /// needs a binder and the surface supplies none (a discarded statement, a
    /// lambda's ignored parameter).
    pub fn fresh(&mut self, tag: &str) -> Outcome<Sym, LowerError> {
        Outcome::Ok(self.interner.intern(tag).as_sym())
    }

    /// The AST node behind a handle.
    pub fn node(&self, at: AstRef) -> Node {
        self.ast.get(at)
    }

    /// A family operation over `args`, with `code` as the leading operand.
    pub fn family(&mut self, code: i64, args: &[NodeRef]) -> Outcome<NodeRef, LowerError> {
        let mut all: [NodeRef; CAP] = [NodeRef::new(USize(0)); CAP];
        if args.len() + 1 > CAP {
            return Outcome::Err(LowerError::TooManyChildren);
        }
        let head = ok(self.b.lit(
            vehje_ir::Literal::Int(arvo::Int::<64, arvo::strategy::Hot>::from_raw(code)),
            IrSpan::default(),
        ))?;
        all[0] = head;
        all[1..=args.len()].copy_from_slice(args);
        let list = match self.b.alloc_list(&all[..=args.len()]) {
            Maybe::Is(l) => l,
            Maybe::Isnt => return Outcome::Err(LowerError::ArenaFull),
        };
        let r = self.b.raw(FamilyId::new(arvo::Uint::<16, arvo::strategy::Hot>::from_raw(ARITH)), list, IrSpan::default());
        ok(r)
    }

    /// The unit value, which stands in wherever the surface has no value.
    pub fn unit(&mut self) -> Outcome<NodeRef, LowerError> {
        let r = self.b.lit(vehje_ir::Literal::Unit, IrSpan::default());
        ok(r)
    }

    /// A non-recursive binding.
    pub fn bind(
        &mut self,
        name: Sym,
        value: NodeRef,
        body: NodeRef,
    ) -> Outcome<NodeRef, LowerError> {
        let r = self.b.let_(Bool(false), name, value, body, IrSpan::default());
        ok(r)
    }

    /// A recursive binding, which is what a function definition is: its own
    /// name must be in scope for its body so it can call itself.
    pub fn bind_rec(
        &mut self,
        name: Sym,
        value: NodeRef,
        body: NodeRef,
    ) -> Outcome<NodeRef, LowerError> {
        let r = self.b.let_(Bool(true), name, value, body, IrSpan::default());
        ok(r)
    }
}

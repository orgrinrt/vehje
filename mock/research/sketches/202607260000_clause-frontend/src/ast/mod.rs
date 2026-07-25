//! The AST: one lent arena, one node enum, four syntactic categories.
//!
//! Items, types, expressions, and patterns share one arena and one node enum
//! rather than getting four of each. The IR the front end lowers into is shaped
//! that way, so the two halves read alike, and a parser that returns one handle
//! type has no conversions between categories to get wrong. The categories are
//! kept honest by the parser rather than by the type system, which is the trade
//! this makes deliberately: four arenas would encode the distinction and cost
//! four lending budgets, four cursors, and four sets of accessors.

use arvo::USize;
use notko::Maybe;

use crate::token::Span;

/// A handle into the node arena.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AstRef(pub USize);

/// A `{ start, len }` slice of the child-index pool.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AstList {
    /// First child.
    pub start: USize,
    /// How many.
    pub len: USize,
}

impl AstList {
    /// The empty list.
    pub const EMPTY: Self = Self { start: USize(0), len: USize(0) };
}

/// A name, by the span of the source text that spelled it.
///
/// Not interned here: interning wants a table the front end does not own, and
/// the span is enough for every comparison the parser makes.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Name(pub Span);

/// Binary operators, in the grammar's precedence order.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinOp {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Shl,
    Shr,
    BitAnd,
    BitXor,
    BitOr,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

/// Unary operators.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnOp {
    /// `-e`
    Neg,
    /// `!e`
    Not,
    /// `&e`, which erases to its pointee downstream.
    Ref,
    /// `&mut e`, likewise.
    RefMut,
}

/// The kind of a literal, with its text left in the source.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LitKind {
    Int,
    Float,
    Str,
    RawStr,
    Char,
    Bool,
}

pub mod node;
pub use node::Node;

/// The lent AST arena: nodes, a child-index pool, and a parallel span table.
pub struct Arena<'a> {
    nodes: &'a mut [Node],
    spans: &'a mut [Span],
    node_len: USize,
    pool: &'a mut [AstRef],
    pool_len: USize,
}

impl<'a> Arena<'a> {
    /// Wrap three caller-provided regions. All cursors start empty.
    pub fn new(nodes: &'a mut [Node], spans: &'a mut [Span], pool: &'a mut [AstRef]) -> Self {
        Self { nodes, spans, node_len: USize(0), pool, pool_len: USize(0) }
    }

    /// Append a node with its span. `Isnt` when either region is full.
    #[must_use]
    pub fn push(&mut self, node: Node, span: Span) -> Maybe<AstRef> {
        let at = self.node_len.0;
        if at >= self.nodes.len() || at >= self.spans.len() {
            return Maybe::Isnt;
        }
        self.nodes[at] = node;
        self.spans[at] = span;
        self.node_len = USize(at + 1);
        Maybe::Is(AstRef(USize(at)))
    }

    /// Copy `refs` into the pool. `Isnt` when it cannot fit them.
    #[must_use]
    pub fn alloc_list(&mut self, refs: &[AstRef]) -> Maybe<AstList> {
        let start = self.pool_len.0;
        let end = start + refs.len();
        if end > self.pool.len() {
            return Maybe::Isnt;
        }
        self.pool[start..end].copy_from_slice(refs);
        self.pool_len = USize(end);
        Maybe::Is(AstList { start: USize(start), len: USize(refs.len()) })
    }

    /// How many nodes exist.
    pub fn len(&self) -> USize {
        self.node_len
    }

    /// Whether no node exists.
    pub fn is_empty(&self) -> bool {
        self.node_len.0 == 0
    }

    /// Read a node by handle.
    pub fn get(&self, at: AstRef) -> Node {
        self.nodes[at.0 .0]
    }

    /// A node's span.
    pub fn span(&self, at: AstRef) -> Span {
        self.spans[at.0 .0]
    }

    /// The children a list addresses, clamped to the live pool so a malformed
    /// list is an empty slice rather than a panic.
    pub fn list(&self, list: AstList) -> &[AstRef] {
        let start = list.start.0.min(self.pool_len.0);
        let end = (start + list.len.0).min(self.pool_len.0);
        &self.pool[start..end]
    }
}

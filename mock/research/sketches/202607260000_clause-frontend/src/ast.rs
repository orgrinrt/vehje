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

/// One AST node.
///
/// Variants are grouped by syntactic category. A node's children are `AstRef`s
/// and `AstList`s into the same arena, so a subtree is a span of indices and
/// nothing is owned.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Node {
    // ── types ────────────────────────────────────────────────────────────
    /// `&T` or `&mut T`. References erase to their pointee downstream, which is
    /// why the mutability is recorded and then ignored.
    TyRef { mutable: bool, inner: AstRef },
    /// `()`, `(T,)`, or `(A, B, ...)`. A parenthesised type with no trailing
    /// comma is NOT a tuple; the parser peels it.
    TyTuple { elems: AstList },
    /// A path type, with optional generic arguments.
    TyPath { path: AstRef },

    // ── paths ────────────────────────────────────────────────────────────
    /// `a::b::<T>`: segments plus optional generic arguments.
    Path { leading_colon: bool, segs: AstList, generics: AstList },
    /// One path segment: an identifier or `self`/`Self`/`super`/`crate`.
    PathSeg { name: Name },

    // ── expressions ──────────────────────────────────────────────────────
    Lit { kind: LitKind, span: Span },
    /// A path used as a value.
    ExprPath { path: AstRef },
    /// `{ stmts; tail }`. `tail` is `Isnt` when the block ends in a statement.
    Block { stmts: AstList, tail: Maybe<AstRef> },
    /// `(a, b)`; `()` is the unit value.
    ExprTuple { elems: AstList },
    If { cond: AstRef, then_block: AstRef, else_branch: Maybe<AstRef> },
    Match { scrutinee: AstRef, arms: AstList },
    /// One `pat if guard => body` arm.
    Arm { pat: AstRef, guard: Maybe<AstRef>, body: AstRef },
    For { pat: AstRef, iter: AstRef, body: AstRef },
    While { cond: AstRef, body: AstRef },
    Loop { body: AstRef },
    Return { value: Maybe<AstRef> },
    Break { value: Maybe<AstRef> },
    Continue,
    /// `|a, b| body`, with `move` recorded and erased (values are immutable, so
    /// capture is by value either way).
    Closure { moved: bool, params: AstList, body: AstRef },
    /// `Path { field: value, ..base }`.
    StructLit { path: AstRef, fields: AstList, base: Maybe<AstRef> },
    /// One `name: value` field, or `name` shorthand.
    FieldInit { name: Name, value: Maybe<AstRef> },
    /// `base.name`.
    Field { base: AstRef, name: Name },
    /// `base.name(args)`.
    MethodCall { base: AstRef, name: Name, args: AstList },
    /// `callee(args)`.
    Call { callee: AstRef, args: AstList },
    /// One call argument, with the keyword name the grammar parses and later
    /// passes currently ignore.
    Arg { name: Maybe<Name>, value: AstRef },
    /// `base[index]`.
    Index { base: AstRef, index: AstRef },
    /// `e?`.
    Question { value: AstRef },
    Unary { op: UnOp, operand: AstRef },
    Binary { op: BinOp, lhs: AstRef, rhs: AstRef },
    /// `a..b` or `a..=b`, with either end optional.
    Range { inclusive: bool, lo: Maybe<AstRef>, hi: Maybe<AstRef> },
    /// `place = value` and the compound forms; `op` is `Isnt` for plain `=`.
    Assign { op: Maybe<BinOp>, place: AstRef, value: AstRef },
    /// `path!(tokens)` in expression or item position. The body is kept as a
    /// span because a macro's tokens mean nothing until expansion.
    MacroCall { path: AstRef, body: Span },

    // ── statements ───────────────────────────────────────────────────────
    /// `let pat: Ty = init;`, with both the type and the initialiser optional.
    Let { pat: AstRef, ty: Maybe<AstRef>, init: Maybe<AstRef> },
    /// An expression in statement position.
    ExprStmt { value: AstRef },

    // ── patterns ─────────────────────────────────────────────────────────
    /// `_`
    PatWild,
    /// A literal pattern, with an optional range end.
    PatLit { kind: LitKind, span: Span, range_end: Maybe<AstRef> },
    /// A bare binding.
    PatIdent { name: Name },
    /// A multi-segment or leading-colon path.
    PatPath { path: AstRef },
    /// `Path(a, b)`.
    PatTupleStruct { path: AstRef, elems: AstList },
    /// `(a, b)`; a parenthesised pattern with no trailing comma is peeled.
    PatTuple { elems: AstList },
    /// `a..b` / `a..=b`.
    PatRange { inclusive: bool, lo: Maybe<AstRef>, hi: Maybe<AstRef> },
    /// `&p` / `&mut p`, peeled downstream like `TyRef`.
    PatRef { mutable: bool, inner: AstRef },
    /// `..`
    PatRest,
    /// `a | b`.
    PatOr { alts: AstList },
}

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

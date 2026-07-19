//! The Core substrate node: the eleven ratified evaluation forms.
//!
//! One `Node` is one arena slot, addressed by `NodeRef`. Variable-arity
//! children live in the child-index pool, addressed by `NodeList`. The
//! forms are the shared evaluation primitives every grammar lowers into;
//! a language-specific construct is a family node, attached through `Raw`.

use arvo::strategy::Hot;
use arvo::{Bool, Identity, Int, USize, Uint};

use hilavitkutin_str::Str;

/// An index into the node arena.
///
/// A platform-width handle (arena indexing is platform-width); a compact
/// packed encoding is a later optimization (see BACKLOG).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NodeRef(USize);

impl NodeRef {
    /// Construct from an arena index.
    pub const fn new(index: USize) -> Self {
        Self(index)
    }

    /// The arena index.
    pub const fn index(self) -> USize {
        self.0
    }
}

/// A slice of the child-index pool: `len` consecutive `NodeRef` starting
/// at `start`. Gives a form its variable-arity children without a
/// per-node allocation.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NodeList {
    pub start: USize,
    pub len: USize,
}

impl NodeList {
    /// The empty child list.
    pub const EMPTY: Self = Self { start: USize::ZERO, len: USize::ZERO };
}

/// A family identifier for the `Raw` escape hatch.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct FamilyId(Uint<16, Hot>);

impl FamilyId {
    /// Construct from a raw id.
    pub const fn new(id: Uint<16, Hot>) -> Self {
        Self(id)
    }

    /// The underlying id.
    pub const fn get(self) -> Uint<16, Hot> {
        self.0
    }
}

/// A literal value carried by a `Lit` node.
///
/// Fixed scalars inline; string identity through the interner. Array and
/// blob literals via a byte arena are a later addition (see BACKLOG).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Literal {
    /// The unit value.
    Unit,
    /// A boolean.
    Bool(Bool),
    /// A 64-bit signed integer literal.
    Int(Int<64, Hot>),
    /// An interned string literal.
    Str(Str),
}

/// The Core node: one of the eleven ratified evaluation forms.
///
/// `Copy`; nodes live in a caller-provided arena as flat slots.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Node {
    /// A literal value.
    Lit(Literal),
    /// A reference to a binding, by name pre-resolution.
    Var(Str),
    /// Bind `name` to `value` in scope for `body`. `rec` marks a
    /// recursive binding. Multi-binding `let` desugars to nested `Let`.
    Let {
        rec: Bool,
        name: Str,
        value: NodeRef,
        body: NodeRef,
    },
    /// Abstraction over a single parameter. Multi-parameter lambdas
    /// desugar to nested `Lambda`.
    Lambda { param: Str, body: NodeRef },
    /// Application of `callee` to `args`.
    Apply { callee: NodeRef, args: NodeList },
    /// Field, member, or index access of `base` by `key`.
    Project { base: NodeRef, key: Str },
    /// Conditional. Both arms survive into residual code under an emit
    /// policy that preserves control flow.
    If {
        cond: NodeRef,
        then_branch: NodeRef,
        else_branch: NodeRef,
    },
    /// Pattern match over `scrutinee`.
    // FIXME: arms need pattern-plus-body pairs; M0 stores the arm bodies
    // as a NodeList and defers the pattern representation, since no M0
    // consumer matches. The pattern node kinds land with the first
    // matching consumer.
    Match {
        scrutinee: NodeRef,
        arms: NodeList,
    },
    /// Iteration producing a monoidal accumulation of `body` over `seq`.
    Iter { seq: NodeRef, body: NodeRef },
    /// Interpolation of a value into text or content. A distinct form
    /// because the staging rule attaches to it.
    Interp { value: NodeRef },
    /// The one typed escape hatch: a family's own construct, attached to
    /// the Core arena by family id plus a payload the family interprets.
    // FIXME: the payload is a family-interpreted handle; M0 carries only
    // the family id and a node-list placeholder until the first family
    // defines its payload encoding.
    Raw { family: FamilyId, payload: NodeList },
}

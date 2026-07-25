//! The Core substrate node: the twelve evaluation forms.
//!
//! One `Node` is one arena slot, addressed by `NodeRef`. Variable-arity
//! children live in the child-index pool, addressed by `NodeList`. The
//! forms are the shared evaluation primitives every grammar lowers into;
//! a language-specific construct is a family node, attached through `Raw`.

use arvo::strategy::Hot;
use arvo::{Bool, Identity, Int, USize, Uint};

use hilavitkutin_str::Str;
use hilavitkutin_sym::Sym;

use crate::span::Span;

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

/// The Core node: one of the twelve evaluation forms.
///
/// `Copy`; nodes live in a caller-provided arena as flat slots.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Node {
    /// A literal value.
    Lit(Literal),
    /// A reference to a binding, by its `Sym` binder identity.
    Var(Sym),
    /// Bind `name` to `value` in scope for `body`. `rec` marks a
    /// recursive binding. Multi-binding `let` desugars to nested `Let`.
    Let {
        rec: Bool,
        name: Sym,
        value: NodeRef,
        body: NodeRef,
    },
    /// Abstraction over a single parameter. Multi-parameter lambdas
    /// desugar to nested `Lambda`.
    Lambda { param: Sym, body: NodeRef },
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
    /// Handle a computation with user-defined resumable algebraic-effect
    /// handlers. `body` is the handled computation; `clauses` are the handler
    /// clauses, each an operation and a handler body over the resumption. The
    /// twelfth Core form: a general evaluation primitive that unifies the
    /// effect set, the host-call boundary, and macro expansion into one handler
    /// discipline, and supports bounded multi-shot resumption via a host-lent
    /// budget.
    ///
    /// A clause authors two things, not three. The discharge is which stage
    /// provides the handler, which is the binding-time coordinate the grade
    /// already owns, so authoring it here would author a derived quantity.
    Handle { body: NodeRef, clauses: ClauseList },
    /// Perform an operation, to be serviced by the innermost enclosing
    /// [`Node::Handle`] clause whose operation matches.
    ///
    /// The thirteenth Core form, admitted on the same ground as the twelfth: by
    /// algebra, not by census. `Handle` is an eliminator, and nothing among the
    /// other forms built a term it could eliminate, so the handler had nothing
    /// to handle. The free model is generated by the operation symbols and the
    /// handler is the homomorphism out of it, so shipping the homomorphism
    /// alone was half an algebra.
    ///
    /// Deliberately not folded into [`Node::Raw`], and the reason is typing
    /// rather than ergonomics: arithmetic lowers to `Raw`, so performing
    /// through `Raw` would give `1 + 2` an effect, and that is a grade the
    /// handler discipline would then have to subtract everywhere.
    ///
    /// `op` is a freshly minted `Sym` rather than an id from a fixed
    /// vocabulary, which is what makes labelled `break` work: two loops in one
    /// function mint their own operations and cannot catch each other's exits.
    Perform { op: Sym, args: NodeList },
}

/// One handler clause: an operation, its resumption binder, its operand arity,
/// its body, and its span.
///
/// Lives in the arena's clause region rather than the child pool, because the
/// pool's element type is [`NodeRef`] and an operation identity is a `Sym`, so
/// every scheme that fits one into the other is a pun. It is not a [`Node`]
/// variant either: a non-evaluating node in the term arena would give every
/// term-walking pass a skip rule.
///
/// Every field is authored because none is derivable. `arity` in particular
/// cannot be recovered by counting `Lambda` nesting in `body`, because a body
/// may return a lambda.
///
/// Operands curry through `body`'s lambda chain rather than taking binder slots
/// here, which reuses machinery that already works end to end. `resume` does
/// not curry, because the check needs a syntactic grip on whether a clause
/// resumes: a clause whose body references its resumption is refused until the
/// bounded-multi-shot work lands.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Clause {
    /// The operation this clause services.
    pub op: Sym,
    /// The resumption binder. Reserved: unbound until bounded multi-shot lands.
    pub resume: Sym,
    /// How many operands the operation carries.
    pub arity: Uint<8, Hot>,
    /// The handler body, a lambda chain over the operands.
    pub body: NodeRef,
    /// Where the clause was written.
    pub span: Span,
}

/// A slice of the clause region: `len` consecutive [`Clause`] from `start`.
///
/// What [`NodeList`] is to the child pool.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ClauseList {
    /// The first clause.
    pub start: USize,
    /// The number of clauses.
    pub len: USize,
}

impl ClauseList {
    /// The empty clause list.
    pub const EMPTY: Self = Self { start: USize::ZERO, len: USize::ZERO };
}

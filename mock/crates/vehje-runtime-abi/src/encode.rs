//! IR-preserving residual encoding: the format-agnostic node walk.
//!
//! An encoder materializes the IR *as the IR*, so a runtime can transport
//! and interpret it. That is distinct from codegen (`Target::emit`), which
//! abandons the IR by lowering it into a foreign representation (native
//! machine code, another language's source). The line is IR-preserving,
//! not low-level-or-not: native code is a codegen output on the emit axis,
//! never a residual encoder, even though a bytecode tier and a native
//! backend can both look "low level".
//!
//! [`encode`] walks a checked arena once, in node-index order, decomposing
//! each Core form into the primitive calls of a [`ResidualEncoder`]. A tier
//! (flat arena today, bytecode later) implements only how each primitive
//! materializes; the walk, and the twelve-form match, are written once
//! here and shared by every tier.

use arvo::strategy::Hot;
use arvo::{Bool, Int, Maybe, USize};

use hilavitkutin_str::{ArenaInterner, StringInterner};
use hilavitkutin_sym::Interner;
use vehje_ir::{Arena, FamilyId, Literal, Node, NodeList, NodeRef};

use crate::Tier;

/// A Core node kind, as a semantic tag the encoder maps to its own format.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NodeTag {
    Lit,
    Var,
    Let,
    Lambda,
    Apply,
    Project,
    If,
    Match,
    Iter,
    Interp,
    Raw,
    Handle,
}

/// A `Lit` node's literal kind, written before the literal's payload.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LitTag {
    Unit,
    Bool,
    Int,
    Str,
}

/// An IR-preserving residual encoder: one impl per encoding tier.
///
/// The shared [`encode`] walk drives these methods; an impl decides how
/// each primitive materializes (a fixed word record, a bytecode opcode).
/// Every method returns `Isnt` on failure (an exhausted output buffer, an
/// unresolvable name); the walk propagates it. `begin` opens the residual,
/// `begin_node` opens one node's record and names its kind, the payload
/// methods fill that node, `pool` writes the child-index backing, and
/// `finish` closes the residual and reports the byte length.
///
/// Native code is not an encoder: it abandons the IR and rides the
/// `Target::emit` codegen axis instead.
pub trait ResidualEncoder {
    /// Open the residual: the node and pool counts, the root, and the tier.
    fn begin(
        &mut self,
        node_count: USize,
        pool_count: USize,
        root: NodeRef,
        tier: Tier,
    ) -> Maybe<()>;

    /// Open node `index`'s record and name its kind.
    fn begin_node(&mut self, index: USize, tag: NodeTag) -> Maybe<()>;

    /// The literal kind of the current `Lit` node.
    fn lit_kind(&mut self, kind: LitTag) -> Maybe<()>;

    /// A boolean payload (a `rec` flag, a `Bool` literal).
    fn flag(&mut self, b: Bool) -> Maybe<()>;

    /// A 64-bit integer literal payload.
    fn int(&mut self, v: Int<64, Hot>) -> Maybe<()>;

    /// A resolved string payload (a name, a key, a string literal).
    fn text(&mut self, s: &str) -> Maybe<()>; // lint:allow(no-bare-string) reason: resolved name bytes crossing to the encoder; the interner already resolved it; tracked: #207

    /// A child-node reference payload.
    fn child(&mut self, r: NodeRef) -> Maybe<()>;

    /// A child-list payload (a slice of the child-index pool).
    fn list(&mut self, l: NodeList) -> Maybe<()>;

    /// A family id payload (the `Raw` escape hatch).
    fn family(&mut self, id: FamilyId) -> Maybe<()>;

    /// The whole child-index pool.
    fn pool(&mut self, pool: &[NodeRef]) -> Maybe<()>;

    /// Close the residual, returning the total byte length written.
    fn finish(self) -> Maybe<USize>;
}

/// Walk a checked program's arena once and drive `encoder`, returning the
/// byte length the encoder wrote.
///
/// Node-index order: node `i` is emitted before node `i + 1`, so an encoder
/// that lays fixed records keeps `i` as the record index. Strings are
/// resolved through `interner` before reaching the encoder, so the encoder
/// never sees an interner handle.
pub fn encode<A: ArenaInterner, E: ResidualEncoder>(
    arena: &Arena<'_>,
    interner: &StringInterner<A>,
    root: NodeRef,
    tier: Tier,
    mut encoder: E,
) -> Maybe<USize> {
    let pool = arena.pool();
    encoder.begin(arena.len(), USize(pool.len()), root, tier)?;

    let node_count = arena.len().0;
    let mut i = 0;
    while i < node_count {
        let index = USize(i);
        match arena.get(NodeRef::new(index)) {
            Node::Lit(lit) => {
                encoder.begin_node(index, NodeTag::Lit)?;
                match lit {
                    Literal::Unit => encoder.lit_kind(LitTag::Unit)?,
                    Literal::Bool(b) => {
                        encoder.lit_kind(LitTag::Bool)?;
                        encoder.flag(b)?;
                    }
                    Literal::Int(v) => {
                        encoder.lit_kind(LitTag::Int)?;
                        encoder.int(v)?;
                    }
                    Literal::Str(s) => {
                        encoder.lit_kind(LitTag::Str)?;
                        encoder.text(interner.resolve(s)?)?;
                    }
                }
            }
            // FIXME: a binder Sym of the string domain (kind 0b000, a widened
            // source name) resolves to its text here, as before the flip. A
            // MINTED binder (BinderDomain, kind 0b001) has no backing string,
            // so `Interner::resolve` returns Isnt and the `?` fails. No minting
            // exists yet (the Anf/MacroExpand catamorphism is Round B); when it
            // lands, the wire format must encode a minted binder by its Sym
            // bits, not by resolved text. Applies to Var, Let.name, Lambda.param.
            Node::Var(s) => {
                encoder.begin_node(index, NodeTag::Var)?;
                encoder.text(Interner::resolve(interner, s)?)?;
            }
            Node::Let { rec, name, value, body } => {
                encoder.begin_node(index, NodeTag::Let)?;
                encoder.flag(rec)?;
                encoder.text(Interner::resolve(interner, name)?)?;
                encoder.child(value)?;
                encoder.child(body)?;
            }
            Node::Lambda { param, body } => {
                encoder.begin_node(index, NodeTag::Lambda)?;
                encoder.text(Interner::resolve(interner, param)?)?;
                encoder.child(body)?;
            }
            Node::Apply { callee, args } => {
                encoder.begin_node(index, NodeTag::Apply)?;
                encoder.child(callee)?;
                encoder.list(args)?;
            }
            Node::Project { base, key } => {
                encoder.begin_node(index, NodeTag::Project)?;
                encoder.child(base)?;
                encoder.text(interner.resolve(key)?)?;
            }
            Node::If { cond, then_branch, else_branch } => {
                encoder.begin_node(index, NodeTag::If)?;
                encoder.child(cond)?;
                encoder.child(then_branch)?;
                encoder.child(else_branch)?;
            }
            Node::Match { scrutinee, arms } => {
                encoder.begin_node(index, NodeTag::Match)?;
                encoder.child(scrutinee)?;
                encoder.list(arms)?;
            }
            Node::Iter { seq, body } => {
                encoder.begin_node(index, NodeTag::Iter)?;
                encoder.child(seq)?;
                encoder.child(body)?;
            }
            Node::Interp { value } => {
                encoder.begin_node(index, NodeTag::Interp)?;
                encoder.child(value)?;
            }
            Node::Raw { family, payload } => {
                encoder.begin_node(index, NodeTag::Raw)?;
                encoder.family(family)?;
                encoder.list(payload)?;
            }
            Node::Handle { body, clauses } => {
                encoder.begin_node(index, NodeTag::Handle)?;
                encoder.child(body)?;
                encoder.list(clauses)?;
            }
        }
        i += 1;
    }

    encoder.pool(pool)?;
    encoder.finish()
}

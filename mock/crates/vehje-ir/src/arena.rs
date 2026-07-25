//! The no-alloc IR arena: caller-provided bump regions.
//!
//! The IR lives in three caller-provided regions: a node arena, a
//! child-index pool, and a span side-table parallel to the node arena.
//! Each carries a bump cursor; construction appends and returns a handle.
//! The host sizes and provides the regions. No framework library crate
//! allocates.
//!
//! The span side-table stores one `Span` per node at the node's own index,
//! so a diagnostic that has a `NodeRef` recovers the source location
//! through `span` without the node itself carrying its span inline.
//!
//! This is the hand-built equivalent of a cranelift-entity `EntityList` /
//! `ListPool`. The module is kept free of vehje-specific policy so it can
//! be lifted upstream into hilavitkutin (which owns the memory and storage
//! layer) once its shape is proven.

use arvo::{Identity, Maybe, USize};

use crate::node::{Clause, ClauseList, Node, NodeList, NodeRef};
use crate::span::Span;

/// A bump-allocated IR arena over caller-provided memory.
///
/// The lifetime `'a` ties the arena to the caller's regions; nothing is
/// owned or allocated here.
pub struct Arena<'a> {
    nodes: &'a mut [Node],
    spans: &'a mut [Span],
    node_len: USize,
    pool: &'a mut [NodeRef],
    pool_len: USize,
    clauses: &'a mut [Clause],
    clause_len: USize,
}

impl<'a> Arena<'a> {
    /// Wrap four caller-provided regions: the node arena, the span side-table
    /// (parallel to the node arena, so it is sized like it), the child-index
    /// pool, and the clause region. All cursors start empty.
    ///
    /// The clause region is a fourth region rather than a reuse of the pool
    /// because the pool's element type is [`NodeRef`] and a clause carries a
    /// `Sym`, so packing one into the other is a pun. The cost is that the host
    /// sizes and lends one more buffer, which is the fourth instance of a cost
    /// already paid three times.
    pub fn new(
        nodes: &'a mut [Node],
        spans: &'a mut [Span],
        pool: &'a mut [NodeRef],
        clauses: &'a mut [Clause],
    ) -> Self {
        Self {
            nodes,
            spans,
            node_len: USize::ZERO,
            pool,
            pool_len: USize::ZERO,
            clauses,
            clause_len: USize::ZERO,
        }
    }

    /// Copy `clauses` into the clause region, returning the list handle.
    /// `Isnt` if the region cannot fit them, which is a refusal rather than a
    /// growth point.
    #[must_use]
    pub fn alloc_clauses(&mut self, clauses: &[Clause]) -> Maybe<ClauseList> {
        let start = self.clause_len;
        let end = start.0 + clauses.len();
        if end > self.clauses.len() {
            return Maybe::Isnt;
        }
        self.clauses[start.0..end].copy_from_slice(clauses);
        self.clause_len = USize(end);
        Maybe::Is(ClauseList { start, len: USize(clauses.len()) })
    }

    /// The whole live clause region, for the encoder's clause section.
    pub fn all_clauses(&self) -> &[Clause] {
        &self.clauses[..self.clause_len.0]
    }

    /// The clauses a [`ClauseList`] addresses, clamped to the live region so a
    /// malformed list is an empty slice rather than a panic.
    pub fn clauses(&self, list: ClauseList) -> &[Clause] {
        let start = list.start.0.min(self.clause_len.0);
        let end = (start + list.len.0).min(self.clause_len.0);
        &self.clauses[start..end]
    }

    /// Append a node with its source span, returning its handle. `Isnt` if
    /// the node arena (or its parallel span table) is full.
    pub fn push(&mut self, node: Node, span: Span) -> Maybe<NodeRef> {
        let at = self.node_len;
        if at.0 >= self.nodes.len() || at.0 >= self.spans.len() {
            return Maybe::Isnt;
        }
        self.nodes[at.0] = node;
        self.spans[at.0] = span;
        self.node_len = USize(at.0 + 1);
        Maybe::Is(NodeRef::new(at))
    }

    /// Copy `refs` into the child-index pool, returning the list handle.
    /// `Isnt` if the pool cannot fit them.
    pub fn alloc_list(&mut self, refs: &[NodeRef]) -> Maybe<NodeList> {
        let start = self.pool_len;
        let end = start.0 + refs.len();
        if end > self.pool.len() {
            return Maybe::Isnt;
        }
        let mut i = start.0;
        for r in refs {
            self.pool[i] = *r;
            i += 1;
        }
        self.pool_len = USize(end);
        Maybe::Is(NodeList { start, len: USize(refs.len()) })
    }

    /// Read a node by handle.
    pub fn get(&self, at: NodeRef) -> Node {
        self.nodes[at.index().0]
    }

    /// The source span of a node by handle.
    pub fn span(&self, at: NodeRef) -> Span {
        self.spans[at.index().0]
    }

    /// The child refs a `NodeList` addresses.
    ///
    /// Clamped to the filled pool region (`pool_len`), so a malformed
    /// `NodeList` whose range runs past what has been allocated yields a
    /// truncated (or empty) slice rather than reading stale or out-of-bounds
    /// pool memory.
    pub fn list(&self, l: NodeList) -> &[NodeRef] {
        let live = self.pool_len.0;
        let start = l.start.0.min(live);
        // saturating: a malformed `NodeList` with a huge start or len is exactly
        // the case the clamp guards, so the `start + len` must not overflow (and
        // panic) before it is bounded.
        let end = l.start.0.saturating_add(l.len.0).min(live).max(start);
        &self.pool[start..end]
    }

    /// The number of nodes appended so far.
    pub fn len(&self) -> USize {
        self.node_len
    }

    /// The child-index pool filled so far.
    ///
    /// The flat backing every `NodeList` addresses. Serialization writes it
    /// out whole; `NodeList` start and len index directly into it.
    pub fn pool(&self) -> &[NodeRef] {
        &self.pool[..self.pool_len.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use arvo::strategy::Hot;
    use arvo::Uint;

    use crate::node::Literal;
    use crate::span::{ByteOffset, FileId, Span};

    fn at(m: Maybe<NodeRef>) -> NodeRef {
        match m {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        }
    }

    #[test]
    fn span_round_trips_per_node() {
        let mut nodes = [Node::Lit(Literal::Unit); 4];
        let mut spans = [Span::default(); 4];
        let mut pool = [NodeRef::new(USize::ZERO); 4];
        let mut a = Arena::new(&mut nodes, &mut spans, &mut pool, &mut []);

        // a node with the default (empty) span, then one with a distinct span.
        // the raw offsets are built through arvo's from_raw at the
        // span-construction boundary; no bare numeric type escapes.
        let s = Span::new(
            FileId::new(Uint::<32, Hot>::from_raw(2)),
            ByteOffset::new(Uint::<32, Hot>::from_raw(10)),
            ByteOffset::new(Uint::<32, Hot>::from_raw(15)),
        );
        let r0 = at(a.push(Node::Lit(Literal::Unit), Span::default()));
        let r1 = at(a.push(Node::Lit(Literal::Unit), s));

        // each node's span comes back exactly as stored, keyed by its own handle
        assert_eq!(a.span(r0), Span::default());
        assert_eq!(a.span(r1), s);
        assert_ne!(a.span(r0), a.span(r1));
    }
}

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

use crate::node::{Node, NodeList, NodeRef};
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
}

impl<'a> Arena<'a> {
    /// Wrap three caller-provided regions: the node arena, the child-index
    /// pool, and the span side-table (parallel to the node arena, so it is
    /// sized like it). All cursors start empty.
    pub fn new(nodes: &'a mut [Node], spans: &'a mut [Span], pool: &'a mut [NodeRef]) -> Self {
        Self { nodes, spans, node_len: USize::ZERO, pool, pool_len: USize::ZERO }
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
    pub fn list(&self, l: NodeList) -> &[NodeRef] {
        &self.pool[l.start.0..l.start.0 + l.len.0]
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
        let mut a = Arena::new(&mut nodes, &mut spans, &mut pool);

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

//! The no-alloc IR arena: caller-provided bump regions.
//!
//! The IR lives in two caller-provided regions: a node arena and a
//! child-index pool. Each carries a bump cursor; construction appends and
//! returns a handle. The host sizes and provides the regions. No
//! framework library crate allocates.
//!
//! This is the hand-built equivalent of a cranelift-entity `EntityList` /
//! `ListPool`. The module is kept free of vehje-specific policy so it can
//! be lifted upstream into hilavitkutin (which owns the memory and storage
//! layer) once its shape is proven.

use arvo::{Identity, Maybe, USize};

use crate::node::{Node, NodeList, NodeRef};

/// A bump-allocated IR arena over caller-provided memory.
///
/// The lifetime `'a` ties the arena to the caller's regions; nothing is
/// owned or allocated here.
pub struct Arena<'a> {
    nodes: &'a mut [Node],
    node_len: USize,
    pool: &'a mut [NodeRef],
    pool_len: USize,
}

impl<'a> Arena<'a> {
    /// Wrap two caller-provided regions: the node arena and the
    /// child-index pool. Both cursors start empty.
    pub fn new(nodes: &'a mut [Node], pool: &'a mut [NodeRef]) -> Self {
        Self { nodes, node_len: USize::ZERO, pool, pool_len: USize::ZERO }
    }

    /// Append a node, returning its handle. `Isnt` if the node arena is
    /// full.
    pub fn push(&mut self, node: Node) -> Maybe<NodeRef> {
        let at = self.node_len;
        if at.0 >= self.nodes.len() {
            return Maybe::Isnt;
        }
        self.nodes[at.0] = node;
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

    /// The child refs a `NodeList` addresses.
    pub fn list(&self, l: NodeList) -> &[NodeRef] {
        &self.pool[l.start.0..l.start.0 + l.len.0]
    }

    /// The number of nodes appended so far.
    pub fn len(&self) -> USize {
        self.node_len
    }
}

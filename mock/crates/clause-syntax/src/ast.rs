//! Abstract syntax tree arena.
//!
//! The AST is a flat arena of `AstNode`s addressed by `NodeId`. No
//! tree pointers, no boxing, no allocation: the whole tree is a
//! fixed-size array with a length, `Copy` all the way through. Each
//! node carries a kind discriminator (`vehje_ir::AstNodeKind`), a
//! source span (`vehje_ir::Span`), and up to `MAX_CHILDREN`
//! references to other nodes by `NodeId`.
//!
//! This round ships the skeleton: `MAX_CHILDREN` is small (8) and
//! `MAX_NODES` is a modest default (256). Productions that need
//! wider children or larger programs will either raise those
//! constants in their own round or ship a const-generic `Ast<const
//! N: usize>` once the parser exercises those limits. The skeleton
//! keeps the surface simple so downstream phases (resolve,
//! typecheck, codegen) can key off `NodeId` today without waiting
//! for the full grammar.

use vehje_ir::{AstNodeKind, NodeId, Span};
use notko::Maybe;

/// Maximum direct children any `AstNode` can hold in this round.
///
/// Calls, blocks, and parameter lists will exceed this; those
/// productions either raise the constant in their own round or
/// store child lists in a side table keyed by `NodeId`.
pub const MAX_CHILDREN: usize = 8;

/// Default arena capacity in `AstNode` slots.
///
/// Sized for the skeleton and small integration tests; replaced by
/// a const-generic `Ast<const N: usize>` once the real grammar
/// exercises larger programs.
pub const MAX_NODES: usize = 256;

/// A single node in the AST arena.
///
/// `AstNode` is `Copy`: all fields are `Copy` (kind, span, child
/// array, count). Unused child slots hold `NodeId::default()` but
/// callers must not look past `child_count`; use `children()` to
/// get a correctly-bounded slice.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub span: Span,
    children: [NodeId; MAX_CHILDREN],
    child_count: u8,
}

impl Default for AstNode {
    fn default() -> Self {
        Self {
            kind: AstNodeKind::default(),
            span: Span::default(),
            children: [NodeId::default(); MAX_CHILDREN],
            child_count: 0,
        }
    }
}

impl AstNode {
    /// Construct a fresh node with the given kind and span, no
    /// children attached.
    pub const fn new(kind: AstNodeKind, span: Span) -> Self {
        Self {
            kind,
            span,
            children: [NodeId(0); MAX_CHILDREN],
            child_count: 0,
        }
    }

    /// Convenience constructor for a leaf node (no children).
    ///
    /// Identical to `new` today but kept as a named constructor so
    /// leaf sites read intent-first.
    pub const fn leaf(kind: AstNodeKind, span: Span) -> Self {
        Self::new(kind, span)
    }

    /// Number of children attached to this node.
    pub const fn child_count(&self) -> usize {
        self.child_count as usize
    }

    /// Slice over the attached children, bounded by `child_count`.
    pub fn children(&self) -> &[NodeId] {
        &self.children[..self.child_count as usize]
    }

    /// Attach another child. Returns `false` if the node is already
    /// at `MAX_CHILDREN` capacity.
    pub fn push_child(&mut self, id: NodeId) -> bool {
        let idx = self.child_count as usize;
        if idx >= MAX_CHILDREN {
            return false;
        }
        self.children[idx] = id;
        self.child_count += 1;
        true
    }
}

/// Flat arena of `AstNode`s plus a root pointer.
///
/// `Ast` is `Copy` via its fixed-size array. `len` tracks how many
/// slots are populated; indices beyond `len` are valid memory but
/// uninitialised semantically and must not be read.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Ast {
    nodes: [AstNode; MAX_NODES],
    len: u32,
    root: Maybe<NodeId>,
}

impl Default for Ast {
    fn default() -> Self {
        Self::empty()
    }
}

impl Ast {
    /// Construct an empty AST with no root and zero nodes.
    pub const fn empty() -> Self {
        Self {
            nodes: [AstNode::new(AstNodeKind::Unknown, Span::new(
                vehje_ir::FileId(0),
                vehje_ir::ByteOffset(0),
                vehje_ir::ByteOffset(0),
            )); MAX_NODES],
            len: 0,
            root: Maybe::Isnt,
        }
    }

    /// `true` if no nodes have been pushed.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of nodes currently in the arena.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Root node id, or `Maybe::Isnt` if the AST is empty / unset.
    pub const fn root(&self) -> Maybe<NodeId> {
        self.root
    }

    /// Set the root node id. Callers must have pushed the node
    /// before calling this; no bounds check is performed.
    pub fn set_root(&mut self, id: NodeId) {
        self.root = Maybe::Is(id);
    }

    /// Push a node into the arena, returning its id. Returns
    /// `Maybe::Isnt` if the arena is full (`len == MAX_NODES`).
    pub fn push(&mut self, node: AstNode) -> Maybe<NodeId> {
        let idx = self.len as usize;
        if idx >= MAX_NODES {
            return Maybe::Isnt;
        }
        self.nodes[idx] = node;
        self.len += 1;
        Maybe::Is(NodeId(idx as u32))
    }

    /// Borrow a node by id, or `Maybe::Isnt` if the id is out of
    /// bounds.
    pub fn get(&self, id: NodeId) -> Maybe<&AstNode> {
        let idx = id.0 as usize;
        if idx >= self.len as usize {
            return Maybe::Isnt;
        }
        Maybe::Is(&self.nodes[idx])
    }

    /// Mutably borrow a node by id, or `Maybe::Isnt` if the id is
    /// out of bounds.
    pub fn get_mut(&mut self, id: NodeId) -> Maybe<&mut AstNode> {
        let idx = id.0 as usize;
        if idx >= self.len as usize {
            return Maybe::Isnt;
        }
        Maybe::Is(&mut self.nodes[idx])
    }
}

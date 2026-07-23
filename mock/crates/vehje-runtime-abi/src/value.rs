//! The value-arena: the form a produced value crosses back in.
//!
//! A produced value crosses from the runtime back to the host as a value-arena:
//! fixed-width value-node records, a flat child-index pool, a self-contained
//! byte blob, and relative index links, the same structural family as the
//! residual wire form reused for produced values rather than IR. It serialises
//! depth-first, children-before-parents, so a parent records already-known
//! child indices with no back-patching, and a typed structural decode over an
//! untrusted arena is then linear and acyclic by a single monotone index check.
//!
//! It is region-structured: each value node carries a region id through a
//! region-table indirection (the table finalises at chunk close, so no byte is
//! touched twice and there is no back-patch), and region open and close markers
//! let the runtime free a region's segment on stack discipline, with no
//! collector and no refcount.

use arvo::{Bool, Identity, USize};

/// A relative index link into the value-arena's node records.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ValueRef(pub USize);

impl ValueRef {
    /// Construct from a node index.
    pub const fn new(index: USize) -> Self {
        Self(index)
    }

    /// The node index.
    pub const fn index(self) -> USize {
        self.0
    }
}

/// A region identifier: an index into the value-arena's region table.
///
/// Region open and close markers bound a segment the runtime frees on stack
/// discipline; every value node names the region it belongs to.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RegionId(pub USize);

/// The kind of a value-node record.
///
/// A closed set: the framework owns both producer and reader, so the value
/// arena is versioned rather than vtable-dispatched. Fallibility crosses as an
/// `Outcome`-shaped node, so a failing record is a value, not a status flag.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ValueTag {
    /// The unit value.
    Unit,
    /// A boolean, in the record's inline payload.
    Bool,
    /// A 64-bit integer, in the record's inline payload.
    Int,
    /// A string, spanning the byte blob.
    Str,
    /// A sequence, its elements in the child pool.
    Seq,
    /// A record, its fields in the child pool.
    Record,
    /// An `Outcome`-shaped value: a produced result or a carried failure.
    Outcome,
}

/// A slice of the value-arena's flat child-index pool: `len` consecutive
/// [`ValueRef`] starting at `start`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ValueList {
    /// The first child index.
    pub start: USize,
    /// The number of children.
    pub len: USize,
}

impl ValueList {
    /// The empty child list.
    pub const EMPTY: Self = Self { start: USize::ZERO, len: USize::ZERO };
}

/// A span into the value-arena's self-contained byte blob: an offset and a
/// length, for a string or scalar payload.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlobSpan {
    /// The byte offset into the blob.
    pub offset: USize,
    /// The byte length.
    pub len: USize,
}

impl BlobSpan {
    /// The empty span: a record with no blob payload.
    pub const EMPTY: Self = Self { offset: USize::ZERO, len: USize::ZERO };
}

/// One value-node record: fixed-width, region-tagged, with relative links.
///
/// Children are addressed through the flat pool; a scalar or string payload
/// spans the byte blob. The record is fixed width so the runtime indexes node
/// `i` in O(1), the same discipline as the residual node arena.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ValueNode {
    /// The value kind.
    pub tag: ValueTag,
    /// The region this node belongs to.
    pub region: RegionId,
    /// The node's children in the flat pool.
    pub children: ValueList,
    /// The node's payload span in the byte blob.
    pub blob: BlobSpan,
}

/// One region-table record: the segment a region owns.
///
/// The table finalises at chunk close; a region's segment is a contiguous node
/// range the runtime frees on stack discipline.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Region {
    /// The first node of the region's segment.
    pub start: USize,
    /// The number of nodes in the segment.
    pub len: USize,
}

/// The value-arena: the produced value the runtime hands back to the host.
///
/// Fixed-width value-node records, a flat child-index pool, a self-contained
/// byte blob, and a region table, over host-lent backing. The runtime holds no
/// output memory after a call: the backing is lent up front, so the wire form
/// equals the in-process form.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ValueArena<'buf> {
    /// The fixed-width value-node records.
    pub nodes: &'buf [ValueNode],
    /// The flat child-index pool every [`ValueList`] addresses.
    pub pool: &'buf [ValueRef],
    /// The self-contained byte blob for string and scalar payloads.
    pub blob: &'buf [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: host-lent FFI value-arena blob bytes; tracked: #207
    /// The region-table indirection.
    pub regions: &'buf [Region],
    /// The produced value's root node.
    pub root: ValueRef,
}

impl<'buf> ValueArena<'buf> {
    /// Wrap host-lent value-arena backing.
    pub const fn new(
        nodes: &'buf [ValueNode],
        pool: &'buf [ValueRef],
        blob: &'buf [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: host-lent FFI value-arena blob bytes; tracked: #207
        regions: &'buf [Region],
        root: ValueRef,
    ) -> Self {
        Self { nodes, pool, blob, regions, root }
    }

    /// The number of value nodes.
    pub fn len(&self) -> USize {
        USize(self.nodes.len())
    }

    /// Whether the arena is empty.
    pub fn is_empty(&self) -> Bool {
        Bool(self.nodes.is_empty())
    }

    /// Read a value node by relative index.
    pub fn get(&self, at: ValueRef) -> ValueNode {
        self.nodes[at.index().0]
    }

    /// The child refs a [`ValueList`] addresses.
    pub fn children(&self, l: ValueList) -> &[ValueRef] {
        &self.pool[l.start.0..l.start.0 + l.len.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_arena_reads_back_a_children_first_record() {
        // outcome(seq[unit]) built children-first: unit (0), seq (1), outcome
        // (2). the parent records already-emitted child indices, no back-patch.
        let nodes = [
            ValueNode {
                tag: ValueTag::Unit,
                region: RegionId(USize::ZERO),
                children: ValueList::EMPTY,
                blob: BlobSpan::EMPTY,
            },
            ValueNode {
                tag: ValueTag::Seq,
                region: RegionId(USize::ZERO),
                children: ValueList { start: USize::ZERO, len: USize::ONE },
                blob: BlobSpan::EMPTY,
            },
            ValueNode {
                tag: ValueTag::Outcome,
                region: RegionId(USize::ZERO),
                children: ValueList { start: USize::ONE, len: USize::ONE },
                blob: BlobSpan::EMPTY,
            },
        ];
        // the pool: seq points at node 0, outcome points at node 1
        let pool = [ValueRef::new(USize::ZERO), ValueRef::new(USize::ONE)];
        let blob: [u8; 0] = []; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: empty test blob; bytes are the value-arena unit; tracked: #207
        let regions = [Region { start: USize::ZERO, len: USize(3) }];
        let arena = ValueArena::new(&nodes, &pool, &blob, &regions, ValueRef::new(USize(2)));

        assert_eq!(arena.len(), USize(3));
        assert!(!arena.is_empty().0);

        // the root is the outcome, and its single child is the seq (node 1),
        // whose single child is the unit (node 0). every link points backward.
        let root = arena.get(arena.root);
        assert_eq!(root.tag, ValueTag::Outcome);
        let root_kids = arena.children(root.children);
        assert_eq!(root_kids.len(), 1);
        assert_eq!(root_kids[0], ValueRef::new(USize::ONE));

        let seq = arena.get(root_kids[0]);
        assert_eq!(seq.tag, ValueTag::Seq);
        let seq_kids = arena.children(seq.children);
        assert_eq!(seq_kids[0], ValueRef::new(USize::ZERO));
        assert_eq!(arena.get(seq_kids[0]).tag, ValueTag::Unit);
    }
}

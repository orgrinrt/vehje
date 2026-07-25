//! The value image: the pinned byte layout a produced value crosses back in.
//!
//! `#[repr(C)]` fixes a Rust in-memory shape, not a cross-language one, because
//! `USize` is platform-width and the tag takes the C enum width. A Zig writer
//! and a Rust reader would be agreeing only by luck. The residual met this
//! problem first and pinned fixed little-endian words; the value image takes the
//! same answer rather than inventing a second convention.
//!
//! Layout, little-endian throughout:
//!
//! - header: `magic`, `version`, `node_count`, `pool_count`, `region_count`,
//!   `blob_len`, `root` (seven words).
//! - nodes: `node_count` records of six words each (`tag`, `region`,
//!   `children.start`, `children.len`, `blob.offset`, `blob.len`), which is
//!   [`ValueNode`]'s logical shape, so a decode is a direct field read.
//! - pool: `pool_count` words, the flat child-index backing.
//! - regions: `region_count` records of two words (`start`, `len`).
//! - blob: `blob_len` bytes, the self-contained payload backing.
//!
//! Records are fixed width so a reader indexes node `i` in constant time
//! without scanning, and the image is position-independent and self-contained,
//! which is what lets the wire form equal the in-process form.
//!
//! Nodes emit depth-first, children before parents, so a parent records child
//! indices it already knows and nothing is back-patched. A child index is
//! therefore always strictly below its parent's, and that single monotone check
//! is the whole acyclicity argument for an untrusted image: [`ValueImage::validate`]
//! is linear and needs no cycle search.
//!
//! This module is a `#[repr(C)]` / FFI wire boundary. Bare integer widths appear
//! here because the byte layout is the contract the Zig side mirrors (the
//! documented FFI exception); they are confined to the encoding helpers.

use arvo::strategy::Hot;
use arvo::{Bool, Int, Maybe, USize};

use crate::value::{BlobSpan, Region, RegionId, ValueArena, ValueList, ValueNode, ValueRef, ValueTag};

/// Wire magic: the ASCII bytes `VEV0`, little-endian. Distinct from the
/// residual's magic so a residual image is never mistaken for a value image.
const MAGIC: u32 = 0x3056_4556; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire magic word; the byte layout is the contract; tracked: #207
/// Value-image format version.
const VERSION: u32 = 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire version word; tracked: #207

/// Bytes per wire word.
const WORD: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire word size; tracked: #207
/// Header words: magic, version, node_count, pool_count, region_count, blob_len, root.
const HEADER_WORDS: usize = 7; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire header width; tracked: #207
/// Words per node record: tag, region, child start, child len, blob offset, blob len.
const NODE_WORDS: usize = 6; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire node-record width; tracked: #207
/// Words per region record: start, len.
const REGION_WORDS: usize = 2; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire region-record width; tracked: #207
/// Bytes in an encoded integer payload.
const INT_BYTES: usize = 8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire scalar payload width; tracked: #207

const fn tag_code(t: ValueTag) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
    match t {
        ValueTag::Unit => 0, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        ValueTag::Bool => 1, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        ValueTag::Int => 2, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        ValueTag::Str => 3, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        ValueTag::Seq => 4, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        ValueTag::Record => 5, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        ValueTag::Outcome => 6, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
    }
}

const fn tag_of(w: u32) -> Maybe<ValueTag> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
    match w {
        0 => Maybe::Is(ValueTag::Unit), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        1 => Maybe::Is(ValueTag::Bool), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        2 => Maybe::Is(ValueTag::Int), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        3 => Maybe::Is(ValueTag::Str), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        4 => Maybe::Is(ValueTag::Seq), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        5 => Maybe::Is(ValueTag::Record), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        6 => Maybe::Is(ValueTag::Outcome), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        _ => Maybe::Isnt,
    }
}

/// Write one little-endian `u32` at byte offset `at`.
fn put_u32(out: &mut [u8], at: usize, w: u32) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire byte-encoding boundary; tracked: #207
    out[at] = (w & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    out[at + 1] = ((w >> 8) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    out[at + 2] = ((w >> 16) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    out[at + 3] = ((w >> 24) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
}

/// Read one little-endian `u32` at byte offset `at`, unchecked: every caller
/// has already established that the word lies inside the image.
fn get_u32(inp: &[u8], at: usize) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire byte-decoding boundary; tracked: #207
    (inp[at] as u32) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte decode; tracked: #207
        | ((inp[at + 1] as u32) << 8) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte decode; tracked: #207
        | ((inp[at + 2] as u32) << 16) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte decode; tracked: #207
        | ((inp[at + 3] as u32) << 24) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte decode; tracked: #207
}

/// The byte length the image of `arena` occupies.
fn image_len(arena: &ValueArena<'_>) -> usize { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-length arithmetic at the FFI wire boundary; tracked: #207
    HEADER_WORDS * WORD
        + arena.nodes.len() * NODE_WORDS * WORD
        + arena.pool.len() * WORD
        + arena.regions.len() * REGION_WORDS * WORD
        + arena.blob.len()
}

/// Serialise a value-arena into `out` in the pinned layout.
///
/// Returns the number of bytes written, or `Isnt` when `out` cannot hold the
/// whole image. The size is computed and checked before any byte is written, so
/// a refusal never leaves a partial image behind for a reader to misinterpret.
pub fn write_value(arena: &ValueArena<'_>, out: &mut [u8]) -> Maybe<USize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: host-lent FFI byte buffer; tracked: #207
    let total = image_len(arena);
    if out.len() < total {
        return Maybe::Isnt;
    }

    put_u32(out, 0, MAGIC);
    put_u32(out, WORD, VERSION);
    put_u32(out, 2 * WORD, arena.nodes.len() as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
    put_u32(out, 3 * WORD, arena.pool.len() as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
    put_u32(out, 4 * WORD, arena.regions.len() as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
    put_u32(out, 5 * WORD, arena.blob.len() as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
    put_u32(out, 6 * WORD, arena.root.index().0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: index narrows to the wire width; tracked: #207

    let mut at = HEADER_WORDS * WORD;
    for n in arena.nodes {
        put_u32(out, at, tag_code(n.tag));
        put_u32(out, at + WORD, n.region.0 .0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: index narrows to the wire width; tracked: #207
        put_u32(out, at + 2 * WORD, n.children.start.0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: index narrows to the wire width; tracked: #207
        put_u32(out, at + 3 * WORD, n.children.len.0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
        put_u32(out, at + 4 * WORD, n.blob.offset.0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: offset narrows to the wire width; tracked: #207
        put_u32(out, at + 5 * WORD, n.blob.len.0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
        at += NODE_WORDS * WORD;
    }
    for r in arena.pool {
        put_u32(out, at, r.index().0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: index narrows to the wire width; tracked: #207
        at += WORD;
    }
    for reg in arena.regions {
        put_u32(out, at, reg.start.0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: index narrows to the wire width; tracked: #207
        put_u32(out, at + WORD, reg.len.0 as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the wire width; tracked: #207
        at += REGION_WORDS * WORD;
    }
    out[at..at + arena.blob.len()].copy_from_slice(arena.blob);

    Maybe::Is(USize(total))
}

/// A reader over an untrusted value image.
///
/// Every accessor bounds-checks against the image and returns `Maybe`, so a
/// malformed input is a refusal rather than a read past the end or a panic.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ValueImage<'img> {
    bytes: &'img [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the untrusted FFI image bytes themselves; tracked: #207
    /// Byte offset of the node section.
    nodes_at: USize,
    /// Declared node count.
    node_count: USize,
    /// Byte offset of the child pool.
    pool_at: USize,
    /// Declared pool count.
    pool_count: USize,
    /// Byte offset of the region table.
    regions_at: USize,
    /// Declared region count.
    region_count: USize,
    /// Byte offset of the blob.
    blob_at: USize,
    /// Declared blob length in bytes.
    blob_len: USize,
    /// Declared root node index.
    root: USize,
}

impl<'img> ValueImage<'img> {
    /// Parse an image header, checking the magic, the version, and that every
    /// declared section fits inside the byte slice.
    ///
    /// A header that claims more than it carries is refused here, so every later
    /// accessor may read within a section knowing the section exists.
    pub fn parse(bytes: &'img [u8]) -> Maybe<Self> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: untrusted FFI image bytes; tracked: #207
        if bytes.len() < HEADER_WORDS * WORD {
            return Maybe::Isnt;
        }
        if get_u32(bytes, 0) != MAGIC || get_u32(bytes, WORD) != VERSION {
            return Maybe::Isnt;
        }
        let node_count = get_u32(bytes, 2 * WORD) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens to an index; tracked: #207
        let pool_count = get_u32(bytes, 3 * WORD) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens to an index; tracked: #207
        let region_count = get_u32(bytes, 4 * WORD) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens to an index; tracked: #207
        let blob_len = get_u32(bytes, 5 * WORD) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens to an index; tracked: #207
        let root = get_u32(bytes, 6 * WORD) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire index widens; tracked: #207

        let nodes_at = HEADER_WORDS * WORD;
        let pool_at = nodes_at + node_count * NODE_WORDS * WORD;
        let regions_at = pool_at + pool_count * WORD;
        let blob_at = regions_at + region_count * REGION_WORDS * WORD;
        if blob_at + blob_len > bytes.len() {
            return Maybe::Isnt;
        }
        Maybe::Is(Self {
            bytes,
            nodes_at: USize(nodes_at),
            node_count: USize(node_count),
            pool_at: USize(pool_at),
            pool_count: USize(pool_count),
            regions_at: USize(regions_at),
            region_count: USize(region_count),
            blob_at: USize(blob_at),
            blob_len: USize(blob_len),
            root: USize(root),
        })
    }

    /// The number of value nodes.
    pub fn node_count(&self) -> USize {
        self.node_count
    }

    /// The produced value's root node.
    pub fn root(&self) -> ValueRef {
        ValueRef::new(self.root)
    }

    /// Read a value node by index, bounds-checked against the image.
    pub fn try_node(&self, at: ValueRef) -> Maybe<ValueNode> {
        let i = at.index().0;
        if i >= self.node_count.0 {
            return Maybe::Isnt;
        }
        let base = self.nodes_at.0 + i * NODE_WORDS * WORD;
        let tag = match tag_of(get_u32(self.bytes, base)) {
            Maybe::Is(t) => t,
            Maybe::Isnt => return Maybe::Isnt,
        };
        Maybe::Is(ValueNode {
            tag,
            region: RegionId(USize(get_u32(self.bytes, base + WORD) as usize)), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire index widens; tracked: #207
            children: ValueList {
                start: USize(get_u32(self.bytes, base + 2 * WORD) as usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire index widens; tracked: #207
                len: USize(get_u32(self.bytes, base + 3 * WORD) as usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens; tracked: #207
            },
            blob: BlobSpan {
                offset: USize(get_u32(self.bytes, base + 4 * WORD) as usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire offset widens; tracked: #207
                len: USize(get_u32(self.bytes, base + 5 * WORD) as usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens; tracked: #207
            },
        })
    }

    /// Read a region record by id, bounds-checked against the region table.
    pub fn try_region(&self, at: RegionId) -> Maybe<Region> {
        let i = at.0 .0;
        if i >= self.region_count.0 {
            return Maybe::Isnt;
        }
        let base = self.regions_at.0 + i * REGION_WORDS * WORD;
        Maybe::Is(Region {
            start: USize(get_u32(self.bytes, base) as usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire index widens; tracked: #207
            len: USize(get_u32(self.bytes, base + WORD) as usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count widens; tracked: #207
        })
    }

    /// The `i`th child a list addresses, bounds-checked against the pool.
    pub fn child(&self, list: ValueList, i: USize) -> Maybe<ValueRef> {
        if i.0 >= list.len.0 {
            return Maybe::Isnt;
        }
        let at = list.start.0 + i.0;
        if at >= self.pool_count.0 {
            return Maybe::Isnt;
        }
        Maybe::Is(ValueRef::new(USize(
            get_u32(self.bytes, self.pool_at.0 + at * WORD) as usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire index widens; tracked: #207
        )))
    }

    /// The bytes a blob span addresses, bounds-checked against the blob.
    pub fn blob(&self, span: BlobSpan) -> Maybe<&'img [u8]> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: blob payload bytes at the FFI boundary; tracked: #207
        let end = span.offset.0 + span.len.0;
        if end > self.blob_len.0 {
            return Maybe::Isnt;
        }
        Maybe::Is(&self.bytes[self.blob_at.0 + span.offset.0..self.blob_at.0 + end])
    }

    /// The boolean a `Bool` node carries, or `Isnt` if the node is another kind
    /// or its payload is not one byte.
    pub fn as_bool(&self, node: ValueNode) -> Maybe<Bool> {
        if node.tag != ValueTag::Bool || node.blob.len.0 != 1 {
            return Maybe::Isnt;
        }
        match self.blob(node.blob) {
            Maybe::Is(b) => Maybe::Is(Bool(b[0] != 0)), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 0/1 payload byte at the wire boundary; tracked: #207
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// The integer an `Int` node carries, or `Isnt` if the node is another kind
    /// or its payload is not eight bytes.
    pub fn as_int(&self, node: ValueNode) -> Maybe<Int<64, Hot>> {
        if node.tag != ValueTag::Int || node.blob.len.0 != INT_BYTES {
            return Maybe::Isnt;
        }
        match self.blob(node.blob) {
            Maybe::Is(b) => {
                let lo = get_u32(b, 0) as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE scalar decode at the wire boundary; tracked: #207
                let hi = get_u32(b, WORD) as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE scalar decode at the wire boundary; tracked: #207
                let bits = lo | (hi << 32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE scalar decode at the wire boundary; tracked: #207
                Maybe::Is(Int::<64, Hot>::from_raw(bits as i64)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit reinterpretation for the wire; tracked: #207
            }
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// The linear structural check over an untrusted image.
    ///
    /// For every node: its child span lies inside the pool, its blob span inside
    /// the blob, its region inside the region table, and every child index is
    /// strictly below the node's own index. The writer emits children before
    /// parents, so that last comparison is the whole acyclicity argument: a
    /// cycle would require a child at or above its parent, and no search is
    /// needed to rule one out.
    pub fn validate(&self) -> Bool {
        if self.root.0 >= self.node_count.0 {
            return Bool(false);
        }
        let mut i = 0;
        while i < self.node_count.0 {
            let node = match self.try_node(ValueRef::new(USize(i))) {
                Maybe::Is(n) => n,
                Maybe::Isnt => return Bool(false),
            };
            if node.children.start.0 + node.children.len.0 > self.pool_count.0 {
                return Bool(false);
            }
            if node.blob.offset.0 + node.blob.len.0 > self.blob_len.0 {
                return Bool(false);
            }
            if node.children.len.0 > 0 && node.region.0 .0 >= self.region_count.0 && self.region_count.0 > 0 {
                return Bool(false);
            }
            let mut k = 0;
            while k < node.children.len.0 {
                match self.child(node.children, USize(k)) {
                    Maybe::Is(c) if c.index().0 < i => {}
                    _ => return Bool(false),
                }
                k += 1;
            }
            i += 1;
        }
        Bool(true)
    }
}

#[cfg(test)]
mod tests;

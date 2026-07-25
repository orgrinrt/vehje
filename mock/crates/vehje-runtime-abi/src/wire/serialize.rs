//! Tier-0 flat-arena encoding: the baseline `ResidualEncoder` impl.
//!
//! The simplest IR-preserving tier. Each node becomes a fixed record of a
//! tag word plus six payload words, so a runtime indexes node `i` in O(1)
//! without scanning. Child lists index the flat pool; strings are copied
//! into a trailing blob as `(offset, len)`, so the residual is
//! self-contained.
//!
//! Layout, little-endian throughout so the Zig runtime reads the same
//! bytes on every target:
//!
//! - header: `magic`, `version`, `tier`, `node_count`, `pool_count`,
//!   `blob_len`, `root` (seven `u32` words).
//! - nodes: `node_count` records of seven words each.
//! - pool: `pool_count` `u32`, the flat child-index backing.
//! - blob: `blob_len` bytes, the concatenated string bytes.
//!
//! This module is the `#[repr(C)]` / FFI wire boundary. Bare integer
//! widths (`u8`, `u32`, `usize`) appear here because the byte layout is the
//! contract the Zig side mirrors (the documented FFI exception); they are
//! confined to the encoding helpers, and the walk (in `encode`) stays
//! format-agnostic over arvo types.

use arvo::strategy::Hot;
use arvo::{Bool, Identity, Int, Maybe, USize};

use hilavitkutin_str::{ArenaInterner, StringInterner};
use hilavitkutin_sym::Sym;
use vehje_ir::{Arena, FamilyId, NodeList, NodeRef};

use crate::encode::{encode, LitTag, NodeTag, ResidualEncoder};
use crate::wire::residual::Tier;

/// A byte (or word-slot) position into the wire image.
///
/// Semantic name for the encoder's cursors; zero-cost over `USize`. The
/// raw byte arithmetic reads the inner value where it must interface with
/// the `[u8]` buffer at the FFI boundary.
type Cursor = USize;

/// Wire magic: the ASCII bytes `VEH0`, little-endian.
const MAGIC: u32 = 0x3048_4556; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire magic word; the byte layout is the contract; tracked: #207
/// Wire format version.
const VERSION: u32 = 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire version word; tracked: #207

/// Bytes per wire word.
const WORD: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire word size; tracked: #207
/// Header words: magic, version, tier, node_count, pool_count, blob_len, root.
const HEADER_WORDS: usize = 7; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire header width; tracked: #207
/// Words per node record: a tag word plus six payload words.
const NODE_WORDS: usize = 7; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire node-record width; tracked: #207
/// Byte offset of the header's `blob_len` word (back-patched at finish).
const BLOB_LEN_AT: usize = 5 * 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire header slot; tracked: #207

const fn tag_code(tag: NodeTag) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
    match tag {
        NodeTag::Lit => 0, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Var => 1, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Let => 2, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Lambda => 3, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Apply => 4, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Project => 5, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::If => 6, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Match => 7, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Iter => 8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Interp => 9, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Raw => 10, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
        NodeTag::Handle => 11, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tag code; tracked: #207
    }
}

const fn lit_code(kind: LitTag) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire literal sub-tag code; tracked: #207
    match kind {
        LitTag::Unit => 0, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire literal sub-tag; tracked: #207
        LitTag::Bool => 1, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire literal sub-tag; tracked: #207
        LitTag::Int => 2, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire literal sub-tag; tracked: #207
        LitTag::Str => 3, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire literal sub-tag; tracked: #207
    }
}

const fn tier_code(tier: Tier) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tier code; tracked: #207
    match tier {
        Tier::Arena => 0, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tier code; tracked: #207
        Tier::Bytecode => 1, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire tier code; tracked: #207
    }
}

/// Write one little-endian `u32` at byte offset `at`.
fn put_u32(out: &mut [u8], at: usize, w: u32) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire byte-encoding boundary; tracked: #207
    out[at] = (w & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    out[at + 1] = ((w >> 8) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    out[at + 2] = ((w >> 16) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    out[at + 3] = ((w >> 24) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
}

/// Read one little-endian `u32` at byte offset `at`. The tier-0 decode
/// primitive; the Rust-side residual reader promotes it out of `cfg(test)`
/// when the in-proc bindings land.
#[cfg(test)]
fn get_u32(inp: &[u8], at: usize) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire byte-decoding boundary; tracked: #207
    (inp[at] as u32) | ((inp[at + 1] as u32) << 8) | ((inp[at + 2] as u32) << 16) | ((inp[at + 3] as u32) << 24) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte decode at the wire boundary; tracked: #207
}

/// Write a `usize` count as a wire word.
fn put_len(out: &mut [u8], at: usize, n: usize) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire count boundary; tracked: #207
    put_u32(out, at, n as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: count narrows to the 32-bit wire width; tracked: #207
}

/// Write an arena index (`USize` handle) as a wire word.
fn put_index(out: &mut [u8], at: usize, idx: USize) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI wire byte buffer + offset; tracked: #207
    put_len(out, at, idx.0);
}

/// The tier-0 flat-arena encoder: fixed seven-word records, a flat pool,
/// and a trailing string blob, over a caller-provided output buffer.
struct FlatArenaEncoder<'o> {
    out: &'o mut [u8],
    nodes_at: Cursor,
    node_base: Cursor,
    slot: Cursor,
    blob_start: Cursor,
    blob: Cursor,
}

impl<'o> FlatArenaEncoder<'o> {
    fn new(out: &'o mut [u8]) -> Self {
        let z = USize::ZERO;
        Self { out, nodes_at: z, node_base: z, slot: z, blob_start: z, blob: z }
    }

    /// The byte offset of the current record's next payload slot, advancing
    /// the slot cursor by `words`.
    fn take(&mut self, words: usize) -> usize { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte offset arithmetic at the FFI wire boundary; tracked: #207
        let at = self.node_base.0 + self.slot.0 * WORD;
        self.slot = USize(self.slot.0 + words);
        at
    }

    /// Append resolved string bytes to the blob, returning `(offset, len)`.
    fn append(&mut self, s: &[u8]) -> Maybe<(USize, USize)> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw string bytes at the FFI wire blob boundary; tracked: #207
        let off = self.blob.0 - self.blob_start.0;
        let end = self.blob.0 + s.len();
        if end > self.out.len() {
            return Maybe::Isnt;
        }
        let mut k = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte copy index; tracked: #207
        while k < s.len() {
            self.out[self.blob.0 + k] = s[k];
            k += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte copy step; tracked: #207
        }
        self.blob = USize(end);
        Maybe::Is((USize(off), USize(s.len())))
    }
}

impl ResidualEncoder for FlatArenaEncoder<'_> {
    fn begin(
        &mut self,
        node_count: USize,
        pool_count: USize,
        root: NodeRef,
        tier: Tier,
    ) -> Maybe<()> {
        self.nodes_at = USize(HEADER_WORDS * WORD);
        let pool_at = self.nodes_at.0 + node_count.0 * NODE_WORDS * WORD;
        let fixed = pool_at + pool_count.0 * WORD;
        if self.out.len() < fixed {
            return Maybe::Isnt;
        }
        self.blob_start = USize(fixed);
        self.blob = USize(fixed);

        put_u32(self.out, 0, MAGIC);
        put_u32(self.out, WORD, VERSION);
        put_u32(self.out, 2 * WORD, tier_code(tier));
        put_len(self.out, 3 * WORD, node_count.0);
        put_len(self.out, 4 * WORD, pool_count.0);
        // blob_len (word 5) is back-patched at finish
        put_index(self.out, 6 * WORD, root.index());

        // the pool lands between the nodes and the blob; `pool` recomputes
        // its base from `blob_start` and the pool length when it arrives
        let _ = pool_at;
        Maybe::Is(())
    }

    fn begin_node(&mut self, index: USize, tag: NodeTag) -> Maybe<()> {
        self.node_base = USize(self.nodes_at.0 + index.0 * NODE_WORDS * WORD);
        put_u32(self.out, self.node_base.0, tag_code(tag));
        self.slot = USize::ONE;
        Maybe::Is(())
    }

    fn lit_kind(&mut self, kind: LitTag) -> Maybe<()> {
        let at = self.take(1);
        put_u32(self.out, at, lit_code(kind));
        Maybe::Is(())
    }

    fn flag(&mut self, b: Bool) -> Maybe<()> {
        let at = self.take(1);
        put_len(self.out, at, if bool::from(b) { 1 } else { 0 }); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 0/1 flag byte at the wire boundary; tracked: #207
        Maybe::Is(())
    }

    fn int(&mut self, v: Int<64, Hot>) -> Maybe<()> {
        let at = self.take(2);
        let bits = v.to_raw() as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit reinterpretation for the wire; tracked: #207
        put_u32(self.out, at, bits as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: low word; tracked: #207
        put_u32(self.out, at + WORD, (bits >> 32) as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: high word; tracked: #207
        Maybe::Is(())
    }

    fn text(&mut self, s: &str) -> Maybe<()> { // lint:allow(no-bare-string) reason: resolved name bytes; the interner already resolved it; tracked: #207
        let (off, len) = self.append(s.as_bytes())?;
        let at = self.take(2);
        put_index(self.out, at, off);
        put_index(self.out, at + WORD, len);
        Maybe::Is(())
    }

    fn child(&mut self, r: NodeRef) -> Maybe<()> {
        let at = self.take(1);
        put_index(self.out, at, r.index());
        Maybe::Is(())
    }

    fn binder(&mut self, sym: Sym) -> Maybe<()> {
        let at = self.take(1);
        put_u32(self.out, at, sym.to_bits().to_raw()); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the 32-bit binder identity at the FFI wire boundary; tracked: #207
        Maybe::Is(())
    }

    fn list(&mut self, l: NodeList) -> Maybe<()> {
        let at = self.take(2);
        put_index(self.out, at, l.start);
        put_index(self.out, at + WORD, l.len);
        Maybe::Is(())
    }

    fn family(&mut self, id: FamilyId) -> Maybe<()> {
        let at = self.take(1);
        put_len(self.out, at, id.get().to_raw() as usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: family id widens to the wire word; tracked: #207
        Maybe::Is(())
    }

    fn pool(&mut self, pool: &[NodeRef]) -> Maybe<()> {
        // the pool sits immediately before the blob; its base is the blob
        // start minus the pool's own byte length
        let pool_at = self.blob_start.0 - pool.len() * WORD;
        let mut j = 0;
        while j < pool.len() {
            put_index(self.out, pool_at + j * WORD, pool[j].index());
            j += 1;
        }
        Maybe::Is(())
    }

    fn finish(self) -> Maybe<USize> {
        let blob_len = self.blob.0 - self.blob_start.0;
        put_len(self.out, BLOB_LEN_AT, blob_len);
        Maybe::Is(self.blob)
    }
}

/// Serialize a checked program into `out` as a tier-0 residual.
///
/// The convenience entry over [`encode`] with the tier-0
/// [`FlatArenaEncoder`]. Returns the byte length written, or `Isnt` if
/// `out` is too small or a string fails to resolve. `out` is
/// caller-provided; nothing allocates.
pub fn serialize<A: ArenaInterner>(
    arena: &Arena<'_>,
    interner: &StringInterner<A>,
    root: NodeRef,
    tier: Tier,
    out: &mut [u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: caller-provided FFI wire byte buffer; tracked: #207
) -> Maybe<USize> {
    encode(arena, interner, root, tier, FlatArenaEncoder::new(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    use arvo::Identity;
    use hilavitkutin_str::{ArenaInterner, StringInterner};
    use vehje_ir::{Builder, Literal, Node, Span};

    /// A no-op arena interner: the test program uses no runtime strings, so
    /// its arena methods are never called (const strings short-circuit).
    struct NoArena;

    impl ArenaInterner for NoArena {
        fn arena_intern(&self, _s: &str) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: foreign ArenaInterner signature; tracked: #72
            0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: unused stub id; tracked: #72
        }
        fn arena_resolve(&self, _id: u32) -> &str { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: foreign ArenaInterner signature; tracked: #72
            ""
        }
    }

    fn at(m: Maybe<NodeRef>) -> NodeRef {
        match m {
            Maybe::Is(r) => r,
            Maybe::Isnt => panic!("arena full"),
        }
    }

    #[test]
    fn tier0_round_trips_header_nodes_and_pool() {
        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // apply(u0, [u1, u2]) with unit atoms: exercises nodes + the pool,
        // no strings so the blob stays empty
        let u0 = at(b.lit(Literal::Unit, Span::default()));
        let u1 = at(b.lit(Literal::Unit, Span::default()));
        let u2 = at(b.lit(Literal::Unit, Span::default()));
        let root = at(b.apply(u0, &[u1, u2], Span::default()));
        let arena = b.into_arena();

        let interner = StringInterner::new(NoArena);
        let mut buf = [0u8; 512]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; bytes are the wire unit; tracked: #207
        let written = match serialize(&arena, &interner, root, Tier::Arena, &mut buf) {
            Maybe::Is(n) => n,
            Maybe::Isnt => panic!("serialize failed"),
        };

        // header reads back
        assert_eq!(get_u32(&buf, 0), MAGIC);
        assert_eq!(get_u32(&buf, WORD), VERSION);
        assert_eq!(get_u32(&buf, 3 * WORD), 4); // 3 lits + apply
        assert_eq!(get_u32(&buf, 4 * WORD), 2); // two pooled args
        assert_eq!(get_u32(&buf, BLOB_LEN_AT), 0); // empty blob
        assert_eq!(get_u32(&buf, 6 * WORD), 3); // root = apply index

        // the root record carries the Apply tag and its arg list
        let base = HEADER_WORDS * WORD + 3 * NODE_WORDS * WORD;
        assert_eq!(get_u32(&buf, base), tag_code(NodeTag::Apply));
        assert_eq!(get_u32(&buf, base + 2 * WORD), 0); // args start at pool 0
        assert_eq!(get_u32(&buf, base + 3 * WORD), 2); // two args

        // written size is header + nodes + pool, no blob
        let expect = HEADER_WORDS * WORD + 4 * NODE_WORDS * WORD + 2 * WORD;
        assert_eq!(written.0, expect);
    }

    #[test]
    fn tier0_encodes_binders_by_sym_bits_and_strings_in_the_blob() {
        use arvo::Bool;
        use hilavitkutin_str::str_const;

        let mut nodes = [Node::Lit(Literal::Unit); 8];
        let mut spans = [Span::default(); 8];
        let mut pool = [NodeRef::new(USize::ZERO); 8];
        let mut b = Builder::new(Arena::new(&mut nodes, &mut spans, &mut pool));

        // let x = "hi" in x: the Str literal's text goes to the blob; the binder
        // x (at the Var and the Let name) encodes by its Sym bits, not text, and
        // the same bits appear at both sites, which is the stable binder identity
        // the runtime resolves variables by.
        let x = str_const!("x").as_sym();
        let x_bits = x.to_bits().to_raw();
        let hi = at(b.lit(Literal::Str(str_const!("hi")), Span::default())); // node 0
        let var = at(b.var(x, Span::default())); // node 1
        let root = at(b.let_(Bool::FALSE, x, hi, var, Span::default())); // node 2
        let arena = b.into_arena();

        let interner = StringInterner::new(NoArena);
        let mut buf = [0u8; 512]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
        let written = match serialize(&arena, &interner, root, Tier::Arena, &mut buf) {
            Maybe::Is(n) => n,
            Maybe::Isnt => panic!("serialize failed"),
        };

        // three nodes, no pooled lists, a two-byte blob ("hi" from the Str
        // literal; the binder does not go to the blob).
        assert_eq!(get_u32(&buf, 3 * WORD), 3);
        assert_eq!(get_u32(&buf, 4 * WORD), 0);
        assert_eq!(get_u32(&buf, BLOB_LEN_AT), 2);

        let blob_start = HEADER_WORDS * WORD + 3 * NODE_WORDS * WORD;
        assert_eq!(&buf[blob_start..blob_start + 2], b"hi");
        assert_eq!(written.0, blob_start + 2);

        // node 0: a Str literal, its text at blob offset 0 len 2.
        let lit_at = HEADER_WORDS * WORD;
        assert_eq!(get_u32(&buf, lit_at), tag_code(NodeTag::Lit));
        assert_eq!(get_u32(&buf, lit_at + 2 * WORD), 0); // str offset
        assert_eq!(get_u32(&buf, lit_at + 3 * WORD), 2); // str len

        // Var (node 1): one binder word, x's Sym bits.
        let var_at = HEADER_WORDS * WORD + NODE_WORDS * WORD;
        assert_eq!(get_u32(&buf, var_at), tag_code(NodeTag::Var));
        assert_eq!(get_u32(&buf, var_at + WORD), x_bits);

        // Let (node 2): rec flag, then the SAME binder bits as the Var (stable
        // identity), then the value/body children (nodes 0 and 1), shifted by
        // the one-word binder slot.
        let let_at = HEADER_WORDS * WORD + 2 * NODE_WORDS * WORD;
        assert_eq!(get_u32(&buf, let_at), tag_code(NodeTag::Let));
        assert_eq!(get_u32(&buf, let_at + WORD), 0); // rec = false
        assert_eq!(get_u32(&buf, let_at + 2 * WORD), x_bits); // name bits, == Var's
        assert_eq!(get_u32(&buf, let_at + 3 * WORD), 0); // value = node 0
        assert_eq!(get_u32(&buf, let_at + 4 * WORD), 1); // body = node 1
    }
}

//! The language-neutral wire-format IR the whole carrier is built on.
//!
//! A `Program` is the canonical in-memory shape the generators produce. It is
//! children-before-parents by construction (a node's operands are always
//! earlier node indices), so the encoded form is acyclic by a single monotone
//! index check, matching the real value-arena invariant. `encode` serialises a
//! program to wire bytes at a chosen `Layout`; the interpreter decodes those
//! bytes at the same layout. Two layouts encoding the same program must produce
//! byte-identical checksums when run, which is the cross-validation contract.
//!
//! The record layout is a bench axis, not a fixed choice. The five layouts
//! below are the real contenders the record-width question needs, including the
//! true 16-byte three-operand layout that the earlier standalone bench never
//! measured. Every stride is asserted at compile time, so a layout can never
//! silently be a different size than its name claims.

/// The bounded op vocabulary. Small on purpose (the corpus measured ~25
/// primitives cover the real language); a lean set keeps the dispatch switch
/// well predicted, which is itself one of the things under test.
pub mod op {
    pub const CONST: u8 = 0;
    pub const ADD: u8 = 1;
    pub const SUB: u8 = 2;
    pub const MUL: u8 = 3;
    pub const AND: u8 = 4;
    pub const OR: u8 = 5;
    pub const XOR: u8 = 6;
    pub const SHL: u8 = 7;
    pub const SHR: u8 = 8;
    pub const MIN: u8 = 9;
    pub const MAX: u8 = 10;
    pub const EQ: u8 = 11;
    pub const LT: u8 = 12;
    pub const SELECT: u8 = 13;
    pub const NEG: u8 = 14;
    pub const NOT: u8 = 15;
    /// The per-call input value (the byte the harness feeds this iteration).
    /// Nullary; returns the runtime seed, so the eval genuinely depends on the
    /// FFI-borne input and cannot be hoisted or constant-folded.
    pub const INPUT: u8 = 16;

    /// Number of distinct ops.
    pub const COUNT: u8 = 17;

    /// Arity of each op, indexed by opcode. CONST carries one operand (a const-
    /// pool index, not a node reference) and INPUT carries none; both are leaves
    /// the generator special-cases. SELECT is ternary; NEG/NOT unary; INPUT
    /// nullary; the rest binary.
    pub const ARITY: [u8; COUNT as usize] =
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 1, 1, 0];
}

/// One node in the canonical program. `operands` are earlier node indices for
/// every op except `CONST`, whose single operand is an index into the const
/// pool.
#[derive(Clone, Debug)]
pub struct Node {
    pub op: u8,
    pub operands: Vec<u32>,
}

/// A canonical program: a const pool plus a children-before-parents node list.
#[derive(Clone, Debug, Default)]
pub struct Program {
    pub consts: Vec<u64>,
    pub nodes: Vec<Node>,
}

impl Program {
    /// Cheap structural check that the children-before-parents invariant holds
    /// and every operand is in range. The generators uphold it; this is the
    /// belt-and-suspenders the untrusted-load path would run for real.
    pub fn is_well_formed(&self) -> bool {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.op >= op::COUNT {
                return false;
            }
            if node.op == op::INPUT {
                if !node.operands.is_empty() {
                    return false;
                }
            } else if node.op == op::CONST {
                if node.operands.len() != 1 || node.operands[0] as usize >= self.consts.len() {
                    return false;
                }
            } else {
                if node.operands.len() != op::ARITY[node.op as usize] as usize {
                    return false;
                }
                for &c in &node.operands {
                    if c as usize >= i {
                        return false; // forward or self reference
                    }
                }
            }
        }
        true
    }
}

/// A record layout: how one node is packed into the wire byte stream. Operands
/// up to `inline_operands` live in the record; any beyond spill to the pool,
/// with the record's first slot holding the pool offset. `stride` is the real
/// on-wire size of one record and may exceed `header_bytes + inline*4` when the
/// layout carries padding, which is exactly the thing the width question tests.
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub name: &'static str,
    pub header_bytes: usize,
    pub inline_operands: usize,
    pub stride: usize,
}

impl Layout {
    const fn check(self) -> Self {
        assert!(self.stride >= self.header_bytes + self.inline_operands * 4);
        assert!(self.header_bytes == 4 || self.header_bytes == 8);
        self
    }
}

/// 12 bytes: 4-byte header, two inline operands. The narrow inline-2 baseline;
/// arity-3 nodes spill.
pub const REC12: Layout = Layout {
    name: "rec12",
    header_bytes: 4,
    inline_operands: 2,
    stride: 12,
}
.check();

/// 16 bytes: 4-byte header, three inline operands, no padding. The contender
/// the earlier bench never built: three inline operands at a true 16-byte
/// stride, four records per cache line and no pool.
pub const REC16: Layout = Layout {
    name: "rec16",
    header_bytes: 4,
    inline_operands: 3,
    stride: 16,
}
.check();

/// 20 bytes: 8-byte header (carrying a u32 shape_id), three inline operands, no
/// padding. This is the layout the "24-byte" standalone bench actually
/// measured.
pub const REC20: Layout = Layout {
    name: "rec20",
    header_bytes: 8,
    inline_operands: 3,
    stride: 20,
}
.check();

/// 24 bytes: 8-byte header, three inline operands, 4 bytes of padding. Tests
/// whether the padding to a 24-byte stride costs anything over REC20.
pub const REC24: Layout = Layout {
    name: "rec24",
    header_bytes: 8,
    inline_operands: 3,
    stride: 24,
}
.check();

/// 32 bytes: 8-byte header, four inline operands, 8 bytes of padding. The wide
/// end; carries a fourth inline operand almost no node uses.
pub const REC32: Layout = Layout {
    name: "rec32",
    header_bytes: 8,
    inline_operands: 4,
    stride: 32,
}
.check();

/// Every layout, for sweeps.
pub const ALL_LAYOUTS: [Layout; 5] = [REC12, REC16, REC20, REC24, REC32];

/// Compile-time proof that each named layout is exactly its named size. If a
/// header or operand count is ever edited without fixing the stride, this fails
/// the build rather than silently mismeasuring, which is the discipline the
/// audit found missing.
const _: () = {
    assert!(REC12.stride == 12);
    assert!(REC16.stride == 16);
    assert!(REC20.stride == 20);
    assert!(REC24.stride == 24);
    assert!(REC32.stride == 32);
};

/// Wire header: version, node count, const count, pool count, each u32 LE.
pub const WIRE_HEADER_BYTES: usize = 16;
const WIRE_VERSION: u32 = 1;

/// Serialise a program to wire bytes at `layout`.
///
/// Layout: `[version, node_count, const_count, pool_count]` (16 bytes), then
/// the consts (u64 LE each), then `node_count` records at `layout.stride`, then
/// the spilled-operand pool (u32 LE each). A record packs `[op, arity, flags:u16
/// (, shape_id:u32)]` then its inline operand slots then padding. A node whose
/// arity exceeds `inline_operands` stores a pool offset in slot 0 and appends
/// its operands to the pool.
pub fn encode(prog: &Program, layout: &Layout) -> Vec<u8> {
    let mut pool: Vec<u32> = Vec::new();
    let node_bytes_start = WIRE_HEADER_BYTES + prog.consts.len() * 8;
    let pool_start = node_bytes_start + prog.nodes.len() * layout.stride;
    let mut buf = vec![0u8; pool_start];

    buf[0..4].copy_from_slice(&WIRE_VERSION.to_le_bytes());
    buf[4..8].copy_from_slice(&(prog.nodes.len() as u32).to_le_bytes());
    buf[8..12].copy_from_slice(&(prog.consts.len() as u32).to_le_bytes());
    // pool_count backfilled after we know it.

    for (i, &c) in prog.consts.iter().enumerate() {
        let o = WIRE_HEADER_BYTES + i * 8;
        buf[o..o + 8].copy_from_slice(&c.to_le_bytes());
    }

    for (i, node) in prog.nodes.iter().enumerate() {
        let base = node_bytes_start + i * layout.stride;
        buf[base] = node.op;
        buf[base + 1] = node.operands.len() as u8;
        // flags u16 at base+2..+4 left zero; shape_id u32 at base+4..+8 for
        // 8-byte headers left zero (both are real bytes the decoder strides
        // over, which is the point of measuring the header size).
        let slot0 = base + layout.header_bytes;
        if node.operands.len() <= layout.inline_operands {
            for (k, &operand) in node.operands.iter().enumerate() {
                let o = slot0 + k * 4;
                buf[o..o + 4].copy_from_slice(&operand.to_le_bytes());
            }
        } else {
            let pool_off = pool.len() as u32;
            buf[slot0..slot0 + 4].copy_from_slice(&pool_off.to_le_bytes());
            pool.extend_from_slice(&node.operands);
        }
    }

    buf[12..16].copy_from_slice(&(pool.len() as u32).to_le_bytes());
    for &p in &pool {
        buf.extend_from_slice(&p.to_le_bytes());
    }
    buf
}

/// A decoded view over wire bytes at a layout. Zero-copy: it indexes into the
/// byte slice on demand, which is what the interpreter's hot loop does.
pub struct Decoded<'a> {
    pub bytes: &'a [u8],
    pub layout: Layout,
    pub node_count: usize,
    pub const_count: usize,
    nodes_start: usize,
    pool_start: usize,
}

impl<'a> Decoded<'a> {
    pub fn parse(bytes: &'a [u8], layout: Layout) -> Option<Decoded<'a>> {
        if bytes.len() < WIRE_HEADER_BYTES {
            return None;
        }
        let rd = |o: usize| u32::from_le_bytes(bytes[o..o + 4].try_into().unwrap()) as usize;
        if rd(0) != WIRE_VERSION as usize {
            return None;
        }
        let node_count = rd(4);
        let const_count = rd(8);
        let pool_count = rd(12);
        let nodes_start = WIRE_HEADER_BYTES + const_count * 8;
        let pool_start = nodes_start + node_count * layout.stride;
        if pool_start + pool_count * 4 > bytes.len() {
            return None;
        }
        Some(Decoded {
            bytes,
            layout,
            node_count,
            const_count,
            nodes_start,
            pool_start,
        })
    }

    #[inline]
    pub fn const_at(&self, i: usize) -> u64 {
        let o = WIRE_HEADER_BYTES + i * 8;
        u64::from_le_bytes(self.bytes[o..o + 8].try_into().unwrap())
    }

    #[inline]
    pub fn op_at(&self, i: usize) -> u8 {
        self.bytes[self.nodes_start + i * self.layout.stride]
    }

    #[inline]
    pub fn arity_at(&self, i: usize) -> usize {
        self.bytes[self.nodes_start + i * self.layout.stride + 1] as usize
    }

    /// Read operand `k` of node `i`, from the inline slots or, if the node
    /// spilled, from the pool. This is the load whose cost the width axis moves.
    #[inline]
    pub fn operand(&self, i: usize, k: usize, arity: usize) -> u32 {
        let base = self.nodes_start + i * self.layout.stride + self.layout.header_bytes;
        if arity <= self.layout.inline_operands {
            let o = base + k * 4;
            u32::from_le_bytes(self.bytes[o..o + 4].try_into().unwrap())
        } else {
            let pool_off = u32::from_le_bytes(self.bytes[base..base + 4].try_into().unwrap()) as usize;
            let o = self.pool_start + (pool_off + k) * 4;
            u32::from_le_bytes(self.bytes[o..o + 4].try_into().unwrap())
        }
    }
}

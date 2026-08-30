//! Writing the value image: the byte form a produced value crosses back in.
//!
//! Mirrors the layout `vehje-runtime-abi`'s `value_image` module pins, so the
//! Rust reader and this writer agree by contract rather than by luck:
//!
//! - header: `magic`, `version`, `node_count`, `pool_count`, `region_count`,
//!   `blob_len`, `root` (seven little-endian words).
//! - nodes: `node_count` records of six words (`tag`, `region`,
//!   `children.start`, `children.len`, `blob.offset`, `blob.len`).
//! - pool: `pool_count` words, the flat child-index backing.
//! - regions: `region_count` records of two words (`start`, `len`).
//! - blob: `blob_len` bytes.
//!
//! Nodes emit children before parents, so a parent records child indices it
//! already knows and a reader proves acyclicity with one monotone comparison.
//!
//! The runtime evaluates scalars today, so this writes the single-node image a
//! scalar needs. Sequences and records gain their writer when those values
//! exist; writing a general N-node emitter before there is an N-node value
//! would be guessing at a shape instead of serving one.

const std = @import("std");

/// Wire magic: the ASCII bytes `VEV0`, little-endian. Distinct from the
/// residual's magic so a residual image is never mistaken for a value image.
pub const MAGIC: u32 = 0x3056_4556;
/// Value-image format version.
pub const VERSION: u32 = 1;

const WORD: usize = 4;
const HEADER_WORDS: usize = 7;
const NODE_WORDS: usize = 6;

/// The value-node kinds, matching `vehje-runtime-abi`'s `ValueTag` codes.
pub const Tag = enum(u32) {
    unit = 0,
    boolean = 1,
    int = 2,
    str = 3,
    seq = 4,
    record = 5,
    outcome = 6,
};

pub const WriteError = error{
    /// The lent buffer cannot hold the whole image. Nothing is written, so a
    /// refusal never leaves a partial image for a reader to misinterpret.
    BufferTooSmall,
};

fn putU32(out: []u8, at: usize, w: u32) void {
    out[at] = @truncate(w & 0xff);
    out[at + 1] = @truncate((w >> 8) & 0xff);
    out[at + 2] = @truncate((w >> 16) & 0xff);
    out[at + 3] = @truncate((w >> 24) & 0xff);
}

/// The byte length of a single-node image carrying `payload_len` blob bytes.
pub fn scalarLen(payload_len: usize) usize {
    return HEADER_WORDS * WORD + NODE_WORDS * WORD + payload_len;
}

/// Write a one-node image holding a scalar, returning the bytes written.
///
/// The node is the root, belongs to region zero, has no children, and spans the
/// whole blob. A `unit` passes an empty payload, a boolean one byte, an integer
/// eight little-endian bytes.
pub fn writeScalar(out: []u8, tag: Tag, payload: []const u8) WriteError!usize {
    const total = scalarLen(payload.len);
    if (out.len < total) return WriteError.BufferTooSmall;

    putU32(out, 0 * WORD, MAGIC);
    putU32(out, 1 * WORD, VERSION);
    putU32(out, 2 * WORD, 1); // node_count
    putU32(out, 3 * WORD, 0); // pool_count
    putU32(out, 4 * WORD, 0); // region_count
    putU32(out, 5 * WORD, @intCast(payload.len)); // blob_len
    putU32(out, 6 * WORD, 0); // root

    const node = HEADER_WORDS * WORD;
    putU32(out, node + 0 * WORD, @intFromEnum(tag));
    putU32(out, node + 1 * WORD, 0); // region
    putU32(out, node + 2 * WORD, 0); // children.start
    putU32(out, node + 3 * WORD, 0); // children.len
    putU32(out, node + 4 * WORD, 0); // blob.offset
    putU32(out, node + 5 * WORD, @intCast(payload.len)); // blob.len

    const blob = node + NODE_WORDS * WORD;
    @memcpy(out[blob .. blob + payload.len], payload);
    return total;
}

/// The byte length of an N-node image with the given pool and blob sizes.
pub fn treeLen(node_count: usize, pool_count: usize, blob_len: usize) usize {
    return HEADER_WORDS * WORD + node_count * NODE_WORDS * WORD + pool_count * WORD + blob_len;
}

/// The header fields an N-node image declares.
pub const Header = struct {
    node_count: u32,
    pool_count: u32,
    region_count: u32,
    blob_len: u32,
    root: u32,
};

/// Write the seven-word header.
pub fn putHeader(out: []u8, h: Header) void {
    putU32(out, 0 * WORD, MAGIC);
    putU32(out, 1 * WORD, VERSION);
    putU32(out, 2 * WORD, h.node_count);
    putU32(out, 3 * WORD, h.pool_count);
    putU32(out, 4 * WORD, h.region_count);
    putU32(out, 5 * WORD, h.blob_len);
    putU32(out, 6 * WORD, h.root);
}

/// Write node record `i`.
pub fn putNode(out: []u8, i: usize, tag: u32, cstart: u32, clen: u32, boff: u32, blen: u32) void {
    const at = HEADER_WORDS * WORD + i * NODE_WORDS * WORD;
    putU32(out, at + 0 * WORD, tag);
    putU32(out, at + 1 * WORD, 0); // region
    putU32(out, at + 2 * WORD, cstart);
    putU32(out, at + 3 * WORD, clen);
    putU32(out, at + 4 * WORD, boff);
    putU32(out, at + 5 * WORD, blen);
}

/// Write the flat child-index pool, which follows the node records.
pub fn putPool(out: []u8, node_count: usize, pool: []const u32) void {
    const at = HEADER_WORDS * WORD + node_count * NODE_WORDS * WORD;
    for (pool, 0..) |w, i| putU32(out, at + i * WORD, w);
}

/// Write the byte blob, which follows the pool (the region table is empty).
pub fn putBlob(out: []u8, node_count: usize, pool_count: usize, blob: []const u8) void {
    const at = HEADER_WORDS * WORD + node_count * NODE_WORDS * WORD + pool_count * WORD;
    if (blob.len != 0) @memcpy(out[at .. at + blob.len], blob);
}

/// Encode an integer payload as eight little-endian bytes.
pub fn intPayload(v: i64) [8]u8 {
    const bits: u64 = @bitCast(v);
    var b: [8]u8 = undefined;
    var i: usize = 0;
    while (i < 8) : (i += 1) b[i] = @truncate((bits >> @intCast(i * 8)) & 0xff);
    return b;
}

pub const ReadError = error{
    /// The bytes are shorter than a header, or shorter than the sections the
    /// header declares. Named separately from a bad magic so a truncated image
    /// is distinguishable from a foreign one.
    Truncated,
    /// The magic is not `VEV0`, so these bytes are not a value image.
    BadMagic,
    /// The version is one this reader does not decode.
    BadVersion,
    /// The root, a child index, or a span reaches outside the image.
    OutOfRange,
    /// A child index is not strictly below its parent's. Because nodes emit
    /// children before parents, that one comparison is the whole acyclicity
    /// proof, so violating it is how a cyclic or reordered image is caught.
    NotAcyclic,
};

/// A reader over an UNTRUSTED value image.
///
/// Every index and span is bounds-checked against the image, so a malformed
/// input is a named refusal and never a read past the end. This is the one
/// decoder for the boundary: the same reader validates a compound arriving from
/// a family handler and a value crossing back to the host, rather than two
/// decoders written to the same specification and trusted to agree.
pub const Reader = struct {
    bytes: []const u8,
    node_count: u32,
    pool_count: u32,
    region_count: u32,
    blob_len: u32,
    root: u32,
    nodes_at: usize,
    pool_at: usize,
    blob_at: usize,

    pub fn parse(bytes: []const u8) ReadError!Reader {
        if (bytes.len < HEADER_WORDS * WORD) return ReadError.Truncated;
        if (readU32(bytes, 0) != MAGIC) return ReadError.BadMagic;
        if (readU32(bytes, 1 * WORD) != VERSION) return ReadError.BadVersion;

        const node_count = readU32(bytes, 2 * WORD);
        const pool_count = readU32(bytes, 3 * WORD);
        const region_count = readU32(bytes, 4 * WORD);
        const blob_len = readU32(bytes, 5 * WORD);
        const root = readU32(bytes, 6 * WORD);

        const nodes_at = HEADER_WORDS * WORD;
        const pool_at = nodes_at + @as(usize, node_count) * NODE_WORDS * WORD;
        const regions_at = pool_at + @as(usize, pool_count) * WORD;
        const blob_at = regions_at + @as(usize, region_count) * 2 * WORD;
        if (bytes.len < blob_at + blob_len) return ReadError.Truncated;
        if (node_count == 0 or root >= node_count) return ReadError.OutOfRange;

        return .{
            .bytes = bytes,
            .node_count = node_count,
            .pool_count = pool_count,
            .region_count = region_count,
            .blob_len = blob_len,
            .root = root,
            .nodes_at = nodes_at,
            .pool_at = pool_at,
            .blob_at = blob_at,
        };
    }

    fn word(self: Reader, n: u32, w: usize) u32 {
        return readU32(self.bytes, self.nodes_at + @as(usize, n) * NODE_WORDS * WORD + w * WORD);
    }

    pub fn tagOf(self: Reader, n: u32) ReadError!Tag {
        if (n >= self.node_count) return ReadError.OutOfRange;
        const t = self.word(n, 0);
        if (t > @intFromEnum(Tag.outcome)) return ReadError.OutOfRange;
        return @enumFromInt(t);
    }

    pub fn childStart(self: Reader, n: u32) u32 {
        return self.word(n, 2);
    }
    pub fn childLen(self: Reader, n: u32) u32 {
        return self.word(n, 3);
    }
    pub fn blobOff(self: Reader, n: u32) u32 {
        return self.word(n, 4);
    }
    pub fn blobLen(self: Reader, n: u32) u32 {
        return self.word(n, 5);
    }

    /// The `k`th child of node `n`.
    pub fn child(self: Reader, n: u32, k: u32) ReadError!u32 {
        if (k >= self.childLen(n)) return ReadError.OutOfRange;
        const at = self.childStart(n) + k;
        if (at >= self.pool_count) return ReadError.OutOfRange;
        return readU32(self.bytes, self.pool_at + @as(usize, at) * WORD);
    }

    /// Node `n`'s blob span.
    pub fn blob(self: Reader, n: u32) ReadError![]const u8 {
        const off = self.blobOff(n);
        const len = self.blobLen(n);
        if (@as(usize, off) + len > self.blob_len) return ReadError.OutOfRange;
        return self.bytes[self.blob_at + off .. self.blob_at + off + len];
    }

    /// The full structural check over an untrusted image: every span in range,
    /// and every child index strictly below its parent's.
    pub fn validate(self: Reader) ReadError!void {
        var n: u32 = 0;
        while (n < self.node_count) : (n += 1) {
            _ = try self.tagOf(n);
            _ = try self.blob(n);
            const len = self.childLen(n);
            if (@as(usize, self.childStart(n)) + len > self.pool_count) return ReadError.OutOfRange;
            var k: u32 = 0;
            while (k < len) : (k += 1) {
                const c = try self.child(n, k);
                if (c >= self.node_count) return ReadError.OutOfRange;
                if (c >= n) return ReadError.NotAcyclic;
            }
        }
    }
};

test "a scalar image carries its header, record, and payload" {
    var buf: [64]u8 = undefined;
    const payload = intPayload(7);
    const n = try writeScalar(buf[0..], .int, payload[0..]);
    try std.testing.expectEqual(scalarLen(8), n);

    // header
    try std.testing.expectEqual(MAGIC, readU32(buf[0..], 0));
    try std.testing.expectEqual(@as(u32, 1), readU32(buf[0..], 2 * WORD)); // node_count
    try std.testing.expectEqual(@as(u32, 8), readU32(buf[0..], 5 * WORD)); // blob_len
    // node record
    const node = HEADER_WORDS * WORD;
    try std.testing.expectEqual(@intFromEnum(Tag.int), readU32(buf[0..], node));
    try std.testing.expectEqual(@as(u32, 8), readU32(buf[0..], node + 5 * WORD));
    // payload
    try std.testing.expectEqual(@as(u8, 7), buf[node + NODE_WORDS * WORD]);
}

test "a buffer that cannot hold the image is refused" {
    var buf: [8]u8 = undefined;
    try std.testing.expectError(WriteError.BufferTooSmall, writeScalar(buf[0..], .unit, &.{}));
}

fn readU32(bytes: []const u8, at: usize) u32 {
    return @as(u32, bytes[at]) |
        (@as(u32, bytes[at + 1]) << 8) |
        (@as(u32, bytes[at + 2]) << 16) |
        (@as(u32, bytes[at + 3]) << 24);
}

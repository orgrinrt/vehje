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

/// Encode an integer payload as eight little-endian bytes.
pub fn intPayload(v: i64) [8]u8 {
    const bits: u64 = @bitCast(v);
    var b: [8]u8 = undefined;
    var i: usize = 0;
    while (i < 8) : (i += 1) b[i] = @truncate((bits >> @intCast(i * 8)) & 0xff);
    return b;
}

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

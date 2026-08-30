//! The runtime value arena: where a compound value lives during a run.
//!
//! Unlike a string literal, which is a slice into the residual image the host
//! already lent, a produced record or sequence exists nowhere until the program
//! builds it. So it needs backing, and the backing is lent in the same shape as
//! the environment arena: the host provides the space, the runtime bump
//! allocates within it, nothing is freed within a run, and the runtime owns none
//! of it once the call returns. Exhausting it is a named refusal.
//!
//! The node record carries its scalar payload INLINE rather than through a pool
//! indirection. That is measured, not assumed: a pool-indirected walk costs
//! 10.7 ns per node against 4.4 ns inline, and the difference is attributable to
//! the indirection itself rather than to transport.
//!
//! Values are stored plainly. There is no intern table and no deduplication
//! here, and that is a design position rather than an omission: both were
//! candidates under the optimisation mandate and both were refused by
//! measurement for this exact domain. See `mock/benches/string-interning/` and
//! `mock/benches/dedup-vs-reuse/`.

const image = @import("value_image.zig");

pub const Tag = image.Tag;

pub const ArenaError = error{
    /// The lent node or pool space is exhausted. The runtime allocates nothing,
    /// so this is a refusal rather than a growth point.
    ValueArenaFull,
};

/// A span into the arena's flat child-index pool.
pub const Span = struct {
    start: u32 = 0,
    len: u32 = 0,
};

/// One value-node record.
///
/// `payload` carries a boolean or an integer inline. `children` spans the pool
/// for a sequence, a record, or an outcome. `blob` spans the byte blob for a
/// string.
///
/// A RECORD NAMES ITS FIELDS THROUGH ITS CHILDREN, alternating a `str` node
/// holding the field name with the field's value, so `children.len` is even and
/// field `i` is the pair at `2i` and `2i + 1`. The value-node record has no key
/// slot and giving it one would widen every node to serve one kind, so the
/// naming rides in the pool where the variable-arity data already lives. It also
/// keeps `Project` on the strategy the measurements settled, comparing
/// field-name ids at the access site.
pub const ValueNode = struct {
    tag: Tag,
    payload: i64 = 0,
    children: Span = .{},
    blob: Span = .{},
};

/// The caller-lent value arena. Bump-allocated, never popped.
pub const ValueArena = struct {
    slots: []ValueNode,
    pool: []u32,
    blob: []u8,
    len: usize = 0,
    pool_len: usize = 0,
    blob_len: usize = 0,

    /// Append a node, returning its index.
    pub fn alloc(self: *ValueArena, n: ValueNode) ArenaError!u32 {
        if (self.len >= self.slots.len) return ArenaError.ValueArenaFull;
        self.slots[self.len] = n;
        self.len += 1;
        return @intCast(self.len - 1);
    }

    /// Append `kids` to the pool, returning the span that names them.
    pub fn allocChildren(self: *ValueArena, kids: []const u32) ArenaError!Span {
        if (self.pool_len + kids.len > self.pool.len) return ArenaError.ValueArenaFull;
        const at = self.pool_len;
        for (kids, 0..) |k, i| self.pool[at + i] = k;
        self.pool_len += kids.len;
        return .{ .start = @intCast(at), .len = @intCast(kids.len) };
    }

    /// Copy `bytes` into the blob, returning the span that names them.
    pub fn allocBlob(self: *ValueArena, bytes: []const u8) ArenaError!Span {
        if (self.blob_len + bytes.len > self.blob.len) return ArenaError.ValueArenaFull;
        const at = self.blob_len;
        @memcpy(self.blob[at .. at + bytes.len], bytes);
        self.blob_len += bytes.len;
        return .{ .start = @intCast(at), .len = @intCast(bytes.len) };
    }

    /// Append a scalar, keeping the payload inline for the walk AND in the blob
    /// for the wire.
    ///
    /// The image's node record has no inline payload slot, so a scalar's value
    /// travels in the blob; the arena also keeps it inline because that is what
    /// the walk cost measured. Maintaining both is the constructor's job, so no
    /// caller can build a node whose two representations disagree and no
    /// serialiser has to reconstruct one from the other.
    pub fn allocInt(self: *ValueArena, v: i64) ArenaError!u32 {
        const b = image.intPayload(v);
        const span = try self.allocBlob(b[0..]);
        return self.alloc(.{ .tag = .int, .payload = v, .blob = span });
    }

    pub fn allocBool(self: *ValueArena, v: bool) ArenaError!u32 {
        const b = [_]u8{@intFromBool(v)};
        const span = try self.allocBlob(b[0..]);
        return self.alloc(.{ .tag = .boolean, .payload = @intFromBool(v), .blob = span });
    }

    pub fn allocUnit(self: *ValueArena) ArenaError!u32 {
        return self.alloc(.{ .tag = .unit });
    }

    pub fn allocStr(self: *ValueArena, s: []const u8) ArenaError!u32 {
        const span = try self.allocBlob(s);
        return self.alloc(.{ .tag = .str, .blob = span });
    }

    /// Append a record from alternating key and value node indices.
    ///
    /// `kids` must be even in length: a `str` node naming the field, then the
    /// field's value. Every child must already exist, which the arena's
    /// bump discipline enforces for free, and which is what makes the emitted
    /// image children-before-parents without a sort.
    pub fn allocRecord(self: *ValueArena, kids: []const u32) ArenaError!u32 {
        const span = try self.allocChildren(kids);
        return self.alloc(.{ .tag = .record, .children = span });
    }

    pub fn allocSeq(self: *ValueArena, kids: []const u32) ArenaError!u32 {
        const span = try self.allocChildren(kids);
        return self.alloc(.{ .tag = .seq, .children = span });
    }

    pub fn node(self: *const ValueArena, i: u32) ValueNode {
        return self.slots[i];
    }

    pub fn child(self: *const ValueArena, s: Span, k: u32) u32 {
        return self.pool[s.start + k];
    }

    pub fn blobOf(self: *const ValueArena, s: Span) []const u8 {
        return self.blob[s.start .. s.start + s.len];
    }
};

/// Serialise the subtree rooted at `root` into a value image.
///
/// The arena already holds its nodes children-before-parents, because a node
/// can only name children that were allocated before it. So the emission order
/// is the arena's own order restricted to the reachable set, a parent records
/// child indices it already knows, nothing is back-patched, and the reader's one
/// monotone comparison proves acyclicity.
///
/// The whole arena's reachable prefix is emitted rather than only the strict
/// subtree, which keeps indices stable and the walk linear; `root` names the
/// entry point in the header. Refuses a too-small buffer without a partial
/// write, as `writeScalar` does.
pub fn writeTree(out: []u8, arena: *const ValueArena, root: u32) image.WriteError!usize {
    const n = arena.len;
    const total = image.treeLen(n, arena.pool_len, arena.blob_len);
    if (out.len < total) return image.WriteError.BufferTooSmall;

    image.putHeader(out, .{
        .node_count = @intCast(n),
        .pool_count = @intCast(arena.pool_len),
        .region_count = 0,
        .blob_len = @intCast(arena.blob_len),
        .root = root,
    });

    var i: usize = 0;
    while (i < n) : (i += 1) {
        const v = arena.slots[i];
        // the blob span is already correct for every node, because the scalar
        // constructors write the payload into the blob as they set it inline.
        image.putNode(out, i, @intFromEnum(v.tag), v.children.start, v.children.len, v.blob.start, v.blob.len);
    }
    image.putPool(out, n, arena.pool[0..arena.pool_len]);
    image.putBlob(out, n, arena.pool_len, arena.blob[0..arena.blob_len]);
    return total;
}

const std = @import("std");

test "a record round-trips through the image and back" {
    var slots: [16]ValueNode = undefined;
    var pool: [32]u32 = undefined;
    var blob: [128]u8 = undefined;
    var arena = ValueArena{ .slots = slots[0..], .pool = pool[0..], .blob = blob[0..] };

    // { name: "ok", count: 7 }, children alternating key then value
    const k0 = try arena.allocStr("name");
    const v0 = try arena.allocStr("ok");
    const k1 = try arena.allocStr("count");
    const v1 = try arena.allocInt(7);
    const rec = try arena.allocRecord(&.{ k0, v0, k1, v1 });

    var buf: [512]u8 = undefined;
    const n = try writeTree(buf[0..], &arena, rec);

    const r = try image.Reader.parse(buf[0..n]);
    try r.validate();
    try std.testing.expectEqual(rec, r.root);
    try std.testing.expectEqual(image.Tag.record, try r.tagOf(r.root));
    try std.testing.expectEqual(@as(u32, 4), r.childLen(r.root));

    const gk1 = try r.child(r.root, 2);
    const gv1 = try r.child(r.root, 3);
    try std.testing.expectEqualStrings("count", try r.blob(gk1));
    try std.testing.expectEqual(image.Tag.int, try r.tagOf(gv1));
    // the integer survives as eight little-endian blob bytes
    try std.testing.expectEqual(@as(u8, 7), (try r.blob(gv1))[0]);
}

test "an arena smaller than the value refuses by name" {
    var slots: [2]ValueNode = undefined;
    var pool: [4]u32 = undefined;
    var blob: [32]u8 = undefined;
    var arena = ValueArena{ .slots = slots[0..], .pool = pool[0..], .blob = blob[0..] };
    _ = try arena.allocInt(1);
    _ = try arena.allocInt(2);
    try std.testing.expectError(ArenaError.ValueArenaFull, arena.allocInt(3));
}

test "a buffer too small for the tree is refused without a partial write" {
    var slots: [4]ValueNode = undefined;
    var pool: [4]u32 = undefined;
    var blob: [32]u8 = undefined;
    var arena = ValueArena{ .slots = slots[0..], .pool = pool[0..], .blob = blob[0..] };
    const a = try arena.allocInt(1);
    var buf: [8]u8 = undefined;
    try std.testing.expectError(image.WriteError.BufferTooSmall, writeTree(buf[0..], &arena, a));
}

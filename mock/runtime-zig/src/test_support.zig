//! Shared scaffolding for the runtime's test suites.
//!
//! The `Build` helper lets a test state a PROGRAM rather than a byte image, and
//! `arenaRef` lends the value arena. Both suites import this, so neither has to
//! carry the other's setup and neither file outgrows the size the rule allows.
const std = @import("std");
const rt = @import("runtime.zig");
const value_image = @import("value_image.zig");
const varena = @import("value_arena.zig");

// The value arena every test lends the runtime. One backing, reset per call, so
// a test never sees another test's nodes and none of them has to size its own.
pub var ta_slots: [512]varena.ValueNode = undefined;
pub var ta_pool: [1024]u32 = undefined;
pub var ta_blob: [8192]u8 = undefined;
var ta = varena.ValueArena{ .slots = &.{}, .pool = &.{}, .blob = &.{} };

pub fn arenaRef() *varena.ValueArena {
    ta = .{ .slots = ta_slots[0..], .pool = ta_pool[0..], .blob = ta_blob[0..] };
    return &ta;
}

pub const WORD = rt.WORD;
pub const HEADER_WORDS = rt.HEADER_WORDS;
pub const NODE_WORDS = rt.NODE_WORDS;
pub const MAGIC = rt.MAGIC;
pub const TAG_LIT = rt.TAG_LIT;
pub const TAG_VAR = rt.TAG_VAR;
pub const TAG_LET = rt.TAG_LET;
pub const TAG_LAMBDA = rt.TAG_LAMBDA;
pub const TAG_APPLY = rt.TAG_APPLY;
pub const TAG_IF = rt.TAG_IF;
pub const LIT_UNIT = rt.LIT_UNIT;
pub const LIT_BOOL = rt.LIT_BOOL;
pub const LIT_INT = rt.LIT_INT;
pub const Binding = rt.Binding;
pub const Value = rt.Value;
pub const EvalError = rt.EvalError;
pub const evalImage = rt.evalImage;
pub const TAG_RAW = rt.TAG_RAW;
pub const VehjeOperand = rt.VehjeOperand;
pub const Session = rt.Session;
pub const VehjeHost = rt.VehjeHost;

fn putU32(buf: []u8, at: usize, w: u32) void {
    buf[at] = @truncate(w & 0xff);
    buf[at + 1] = @truncate((w >> 8) & 0xff);
    buf[at + 2] = @truncate((w >> 16) & 0xff);
    buf[at + 3] = @truncate((w >> 24) & 0xff);
}

/// A wire-image builder, so a test states the program rather than the bytes.
pub const Build = struct {
    buf: [2048]u8 = [_]u8{0} ** 2048,
    n: u32 = 0,
    pool: [32]u32 = undefined,
    pool_n: u32 = 0,
    blob: [64]u8 = undefined,
    blob_n: u32 = 0,

    /// Append a node record: `tag` plus its payload words in encoder order.
    pub fn node(self: *Build, tag: u32, payload: []const u32) u32 {
        const at = HEADER_WORDS * WORD + @as(usize, self.n) * NODE_WORDS * WORD;
        putU32(self.buf[0..], at, tag);
        for (payload, 0..) |w, i| putU32(self.buf[0..], at + (i + 1) * WORD, w);
        self.n += 1;
        return self.n - 1;
    }

    /// A string literal: the bytes go in the blob, the record names their span.
    pub fn str(self: *Build, text: []const u8) u32 {
        const off = self.blob_n;
        for (text) |c| {
            self.blob[self.blob_n] = c;
            self.blob_n += 1;
        }
        return self.node(TAG_LIT, &.{ rt.LIT_STR, off, @intCast(text.len) });
    }

    /// `base.key`: the base is the first child word, the key a blob span in the
    /// two after it, which is the shape a string literal already uses.
    pub fn project(self: *Build, base: u32, key: []const u8) u32 {
        const off = self.blob_n;
        for (key) |c| {
            self.blob[self.blob_n] = c;
            self.blob_n += 1;
        }
        return self.node(rt.TAG_PROJECT, &.{ base, off, @intCast(key.len) });
    }

    pub fn unit(self: *Build) u32 {
        return self.node(TAG_LIT, &.{LIT_UNIT});
    }
    pub fn int(self: *Build, v: i64) u32 {
        const bits: u64 = @bitCast(v);
        return self.node(TAG_LIT, &.{ LIT_INT, @truncate(bits), @truncate(bits >> 32) });
    }
    pub fn boolean(self: *Build, b: bool) u32 {
        return self.node(TAG_LIT, &.{ LIT_BOOL, @intFromBool(b) });
    }
    pub fn varRef(self: *Build, sym: u32) u32 {
        return self.node(TAG_VAR, &.{sym});
    }
    pub fn lambda(self: *Build, param: u32, body: u32) u32 {
        return self.node(TAG_LAMBDA, &.{ param, body });
    }
    pub fn let(self: *Build, sym: u32, value: u32, body: u32) u32 {
        return self.node(TAG_LET, &.{ 0, sym, value, body });
    }
    pub fn letRec(self: *Build, sym: u32, value: u32, body: u32) u32 {
        return self.node(TAG_LET, &.{ 1, sym, value, body });
    }
    pub fn if_(self: *Build, c: u32, t: u32, e: u32) u32 {
        return self.node(TAG_IF, &.{ c, t, e });
    }

    /// Apply `callee` to `args`, pooling the argument list.
    pub fn apply(self: *Build, callee: u32, args: []const u32) u32 {
        const start = self.pool_n;
        for (args) |a| {
            self.pool[self.pool_n] = a;
            self.pool_n += 1;
        }
        return self.node(TAG_APPLY, &.{ callee, start, @intCast(args.len) });
    }

    /// A family operation over `args`, pooling the operand list.
    pub fn raw(self: *Build, family: u32, args: []const u32) u32 {
        const start = self.pool_n;
        for (args) |a| {
            self.pool[self.pool_n] = a;
            self.pool_n += 1;
        }
        return self.node(TAG_RAW, &.{ family, start, @intCast(args.len) });
    }

    /// Write the header and pool, returning the finished image.
    pub fn finish(self: *Build, root: u32) []const u8 {
        putU32(self.buf[0..], 0 * WORD, MAGIC);
        putU32(self.buf[0..], 1 * WORD, 1); // version
        putU32(self.buf[0..], 2 * WORD, 0); // tier = Arena
        putU32(self.buf[0..], 3 * WORD, self.n);
        putU32(self.buf[0..], 4 * WORD, self.pool_n);
        putU32(self.buf[0..], 5 * WORD, self.blob_n); // blob_len
        putU32(self.buf[0..], 6 * WORD, root);
        const pool_base = HEADER_WORDS * WORD + @as(usize, self.n) * NODE_WORDS * WORD;
        var i: u32 = 0;
        while (i < self.pool_n) : (i += 1) {
            putU32(self.buf[0..], pool_base + @as(usize, i) * WORD, self.pool[i]);
        }
        const blob_base = pool_base + @as(usize, self.pool_n) * WORD;
        var j: u32 = 0;
        while (j < self.blob_n) : (j += 1) self.buf[blob_base + j] = self.blob[j];
        return self.buf[0 .. blob_base + self.blob_n];
    }
};

pub fn run(image: []const u8) EvalError!Value {
    var slots: [64]Binding = undefined;
    return evalImage(image, slots[0..], null, null, arenaRef());
}

/// A host sink that hands out one fixed region and records what was committed,
/// which is the degenerate single-reserve case the transfer contract allows.
pub var sink_buf: [128]u8 = undefined;
pub var sink_len: usize = 0;
pub var sink_refuses: bool = false;

pub fn testReserve(userdata: ?*anyopaque, hint: usize) callconv(.c) ?[*]u8 {
    _ = userdata;
    if (sink_refuses or hint > sink_buf.len) return null;
    return sink_buf[0..].ptr;
}

pub fn testCommit(userdata: ?*anyopaque, written: usize) callconv(.c) void {
    _ = userdata;
    sink_len = written;
}

pub fn testSink() rt.VehjeSink {
    sink_len = 0;
    return .{ .reserve = testReserve, .commit = testCommit, .userdata = null };
}

pub fn imageWord(bytes: []const u8, at: usize) u32 {
    return @as(u32, bytes[at]) |
        (@as(u32, bytes[at + 1]) << 8) |
        (@as(u32, bytes[at + 2]) << 16) |
        (@as(u32, bytes[at + 3]) << 24);
}

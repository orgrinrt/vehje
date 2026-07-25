//! Tests for the tier-0 tree-walk interpreter and the value crossing.

const std = @import("std");
const rt = @import("runtime.zig");
const value_image = @import("value_image.zig");

const WORD = rt.WORD;
const HEADER_WORDS = rt.HEADER_WORDS;
const NODE_WORDS = rt.NODE_WORDS;
const MAGIC = rt.MAGIC;
const TAG_LIT = rt.TAG_LIT;
const TAG_VAR = rt.TAG_VAR;
const TAG_LET = rt.TAG_LET;
const TAG_LAMBDA = rt.TAG_LAMBDA;
const TAG_APPLY = rt.TAG_APPLY;
const TAG_IF = rt.TAG_IF;
const LIT_UNIT = rt.LIT_UNIT;
const LIT_BOOL = rt.LIT_BOOL;
const LIT_INT = rt.LIT_INT;
const Binding = rt.Binding;
const Value = rt.Value;
const EvalError = rt.EvalError;
const evalImage = rt.evalImage;

fn putU32(buf: []u8, at: usize, w: u32) void {
    buf[at] = @truncate(w & 0xff);
    buf[at + 1] = @truncate((w >> 8) & 0xff);
    buf[at + 2] = @truncate((w >> 16) & 0xff);
    buf[at + 3] = @truncate((w >> 24) & 0xff);
}

/// A wire-image builder, so a test states the program rather than the bytes.
const Build = struct {
    buf: [2048]u8 = [_]u8{0} ** 2048,
    n: u32 = 0,
    pool: [32]u32 = undefined,
    pool_n: u32 = 0,

    /// Append a node record: `tag` plus its payload words in encoder order.
    fn node(self: *Build, tag: u32, payload: []const u32) u32 {
        const at = HEADER_WORDS * WORD + @as(usize, self.n) * NODE_WORDS * WORD;
        putU32(self.buf[0..], at, tag);
        for (payload, 0..) |w, i| putU32(self.buf[0..], at + (i + 1) * WORD, w);
        self.n += 1;
        return self.n - 1;
    }

    fn unit(self: *Build) u32 {
        return self.node(TAG_LIT, &.{LIT_UNIT});
    }
    fn int(self: *Build, v: i64) u32 {
        const bits: u64 = @bitCast(v);
        return self.node(TAG_LIT, &.{ LIT_INT, @truncate(bits), @truncate(bits >> 32) });
    }
    fn boolean(self: *Build, b: bool) u32 {
        return self.node(TAG_LIT, &.{ LIT_BOOL, @intFromBool(b) });
    }
    fn varRef(self: *Build, sym: u32) u32 {
        return self.node(TAG_VAR, &.{sym});
    }
    fn lambda(self: *Build, param: u32, body: u32) u32 {
        return self.node(TAG_LAMBDA, &.{ param, body });
    }
    fn let(self: *Build, sym: u32, value: u32, body: u32) u32 {
        return self.node(TAG_LET, &.{ 0, sym, value, body });
    }
    fn letRec(self: *Build, sym: u32, value: u32, body: u32) u32 {
        return self.node(TAG_LET, &.{ 1, sym, value, body });
    }
    fn if_(self: *Build, c: u32, t: u32, e: u32) u32 {
        return self.node(TAG_IF, &.{ c, t, e });
    }

    /// Apply `callee` to `args`, pooling the argument list.
    fn apply(self: *Build, callee: u32, args: []const u32) u32 {
        const start = self.pool_n;
        for (args) |a| {
            self.pool[self.pool_n] = a;
            self.pool_n += 1;
        }
        return self.node(TAG_APPLY, &.{ callee, start, @intCast(args.len) });
    }

    /// Write the header and pool, returning the finished image.
    fn finish(self: *Build, root: u32) []const u8 {
        putU32(self.buf[0..], 0 * WORD, MAGIC);
        putU32(self.buf[0..], 1 * WORD, 1); // version
        putU32(self.buf[0..], 2 * WORD, 0); // tier = Arena
        putU32(self.buf[0..], 3 * WORD, self.n);
        putU32(self.buf[0..], 4 * WORD, self.pool_n);
        putU32(self.buf[0..], 5 * WORD, 0); // blob_len
        putU32(self.buf[0..], 6 * WORD, root);
        const pool_base = HEADER_WORDS * WORD + @as(usize, self.n) * NODE_WORDS * WORD;
        var i: u32 = 0;
        while (i < self.pool_n) : (i += 1) {
            putU32(self.buf[0..], pool_base + @as(usize, i) * WORD, self.pool[i]);
        }
        return self.buf[0 .. pool_base + @as(usize, self.pool_n) * WORD];
    }
};

fn run(image: []const u8) EvalError!Value {
    var slots: [64]Binding = undefined;
    return evalImage(image, slots[0..]);
}

test "let and if evaluate to a value" {
    // let x = 1 in if true then x else 2  =>  1
    var b = Build{};
    const x: u32 = 0xABCD_1234;
    const body = b.if_(b.boolean(true), b.varRef(x), b.int(2));
    const root = b.let(x, b.int(1), body);
    try std.testing.expectEqual(Value{ .int = 1 }, try run(b.finish(root)));
}

test "the false branch is taken and a free Var is unbound" {
    var b = Build{};
    const root = b.if_(b.boolean(false), b.int(1), b.int(2));
    try std.testing.expectEqual(Value{ .int = 2 }, try run(b.finish(root)));

    var b2 = Build{};
    const free = b2.varRef(42);
    try std.testing.expectError(EvalError.Unbound, run(b2.finish(free)));
}

test "a lambda applies to its argument" {
    // (\x -> x) 7  =>  7
    var b = Build{};
    const x: u32 = 11;
    const id = b.lambda(x, b.varRef(x));
    const root = b.apply(id, &.{b.int(7)});
    try std.testing.expectEqual(Value{ .int = 7 }, try run(b.finish(root)));
}

test "a closure reads a binding from where it was written, not where it is called" {
    // let a = 5 in (let f = (\x -> a) in (let a = 99 in f 0))  =>  5
    //
    // The inner `a` shadows the outer one at the call site. `f` captured the
    // chain that names the outer `a`, so it still reads 5: capture is by the
    // environment the Lambda was evaluated in, not by the caller's scope.
    var b = Build{};
    const a: u32 = 1;
    const f: u32 = 2;
    const x: u32 = 3;
    const call = b.apply(b.varRef(f), &.{b.int(0)});
    const inner = b.let(a, b.int(99), call);
    const bind_f = b.let(f, b.lambda(x, b.varRef(a)), inner);
    const root = b.let(a, b.int(5), bind_f);
    try std.testing.expectEqual(Value{ .int = 5 }, try run(b.finish(root)));
}

test "a multi-argument apply curries through nested lambdas" {
    // (\x -> \y -> y) 1 2  =>  2
    var b = Build{};
    const x: u32 = 4;
    const y: u32 = 5;
    const inner = b.lambda(y, b.varRef(y));
    const outer = b.lambda(x, inner);
    const root = b.apply(outer, &.{ b.int(1), b.int(2) });
    try std.testing.expectEqual(Value{ .int = 2 }, try run(b.finish(root)));
}

test "a recursive binding is in scope for its own value" {
    // let rec f = (\x -> if x then 1 else f true) in f false  =>  1
    //
    // The body recurses once: `f false` takes the else branch and calls
    // `f true`, which takes the then branch. This only terminates if the
    // closure's captured chain names `f`, which is what `rec` provides.
    var b = Build{};
    const f: u32 = 6;
    const x: u32 = 7;
    const recur = b.apply(b.varRef(f), &.{b.boolean(true)});
    const body = b.if_(b.varRef(x), b.int(1), recur);
    const root = b.letRec(f, b.lambda(x, body), b.apply(b.varRef(f), &.{b.boolean(false)}));
    try std.testing.expectEqual(Value{ .int = 1 }, try run(b.finish(root)));
}

test "applying a non-closure is an error, not a wrong answer" {
    var b = Build{};
    const root = b.apply(b.int(1), &.{b.int(2)});
    try std.testing.expectError(EvalError.NotCallable, run(b.finish(root)));
}

test "a node index past the arena is refused" {
    var b = Build{};
    _ = b.int(1);
    const image = b.finish(9); // root names a node that does not exist
    try std.testing.expectError(EvalError.Corrupt, run(image));
}

test "exhausting the lent environment arena is reported, not overrun" {
    // Each `let` consumes one slot; two slots cannot hold three bindings.
    var b = Build{};
    const inner = b.let(3, b.int(3), b.varRef(3));
    const mid = b.let(2, b.int(2), inner);
    const root = b.let(1, b.int(1), mid);
    const image = b.finish(root);
    var slots: [2]Binding = undefined;
    try std.testing.expectError(EvalError.EnvFull, evalImage(image, slots[0..]));
}

// ── the value crossing ────────────────────────────────────────────────────

/// A host sink that hands out one fixed region and records what was committed,
/// which is the degenerate single-reserve case the transfer contract allows.
var sink_buf: [128]u8 = undefined;
var sink_len: usize = 0;
var sink_refuses: bool = false;

fn testReserve(userdata: ?*anyopaque, hint: usize) callconv(.c) ?[*]u8 {
    _ = userdata;
    if (sink_refuses or hint > sink_buf.len) return null;
    return sink_buf[0..].ptr;
}

fn testCommit(userdata: ?*anyopaque, written: usize) callconv(.c) void {
    _ = userdata;
    sink_len = written;
}

fn testSink() rt.VehjeSink {
    sink_len = 0;
    return .{ .reserve = testReserve, .commit = testCommit, .userdata = null };
}

fn imageWord(bytes: []const u8, at: usize) u32 {
    return @as(u32, bytes[at]) |
        (@as(u32, bytes[at + 1]) << 8) |
        (@as(u32, bytes[at + 2]) << 16) |
        (@as(u32, bytes[at + 3]) << 24);
}

test "a produced value crosses back through the sink as a value image" {
    // let x = 42 in x  =>  42, committed as a one-node value image.
    var b = Build{};
    const x: u32 = 21;
    const root = b.let(x, b.int(42), b.varRef(x));
    const program = b.finish(root);

    sink_refuses = false;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rc);

    const out = sink_buf[0..sink_len];
    try std.testing.expectEqual(value_image.scalarLen(8), sink_len);
    try std.testing.expectEqual(value_image.MAGIC, imageWord(out, 0));
    try std.testing.expectEqual(@as(u32, 1), imageWord(out, 2 * 4)); // one node
    try std.testing.expectEqual(@as(u32, 8), imageWord(out, 5 * 4)); // eight blob bytes

    const node = 7 * 4;
    try std.testing.expectEqual(@intFromEnum(value_image.Tag.int), imageWord(out, node));
    const payload = out[node + 6 * 4 ..];
    try std.testing.expectEqual(@as(u8, 42), payload[0]);
}

test "a closure is refused rather than partially written" {
    // A lambda is a value the host cannot hold: it names an environment inside
    // this run. The call fails and nothing is committed.
    var b = Build{};
    const root = b.lambda(1, b.int(1));
    const program = b.finish(root);

    sink_refuses = false;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink);
    try std.testing.expectEqual(rt.VEHJE_RESULT_ERR, rc);
    try std.testing.expectEqual(@as(usize, 0), sink_len);
}

test "a sink that refuses its reservation is backpressure, not a crash" {
    var b = Build{};
    const root = b.int(1);
    const program = b.finish(root);

    sink_refuses = true;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink);
    try std.testing.expectEqual(rt.VEHJE_RESULT_ERR, rc);
    try std.testing.expectEqual(@as(usize, 0), sink_len);
    sink_refuses = false;
}

test "no sink means the host wanted only the outcome" {
    var b = Build{};
    const root = b.int(1);
    const program = b.finish(root);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rt.vehje_runtime_execute(null, program.ptr, program.len, null));
}

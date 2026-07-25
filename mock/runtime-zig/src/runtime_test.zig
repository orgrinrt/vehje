//! Tests for the tier-0 tree-walk interpreter and the value crossing.

const std = @import("std");
const rt = @import("runtime.zig");
const value_image = @import("value_image.zig");
const varena = @import("value_arena.zig");

// The value arena every test lends the runtime. One backing, reset per call, so
// a test never sees another test's nodes and none of them has to size its own.
var ta_slots: [512]varena.ValueNode = undefined;
var ta_pool: [1024]u32 = undefined;
var ta_blob: [8192]u8 = undefined;
var ta = varena.ValueArena{ .slots = &.{}, .pool = &.{}, .blob = &.{} };

fn arenaRef() *varena.ValueArena {
    ta = .{ .slots = ta_slots[0..], .pool = ta_pool[0..], .blob = ta_blob[0..] };
    return &ta;
}

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
const TAG_RAW = rt.TAG_RAW;
const VehjeOperand = rt.VehjeOperand;
const Session = rt.Session;
const VehjeHost = rt.VehjeHost;

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
    blob: [64]u8 = undefined,
    blob_n: u32 = 0,

    /// Append a node record: `tag` plus its payload words in encoder order.
    fn node(self: *Build, tag: u32, payload: []const u32) u32 {
        const at = HEADER_WORDS * WORD + @as(usize, self.n) * NODE_WORDS * WORD;
        putU32(self.buf[0..], at, tag);
        for (payload, 0..) |w, i| putU32(self.buf[0..], at + (i + 1) * WORD, w);
        self.n += 1;
        return self.n - 1;
    }

    /// A string literal: the bytes go in the blob, the record names their span.
    fn str(self: *Build, text: []const u8) u32 {
        const off = self.blob_n;
        for (text) |c| {
            self.blob[self.blob_n] = c;
            self.blob_n += 1;
        }
        return self.node(TAG_LIT, &.{ rt.LIT_STR, off, @intCast(text.len) });
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

    /// A family operation over `args`, pooling the operand list.
    fn raw(self: *Build, family: u32, args: []const u32) u32 {
        const start = self.pool_n;
        for (args) |a| {
            self.pool[self.pool_n] = a;
            self.pool_n += 1;
        }
        return self.node(TAG_RAW, &.{ family, start, @intCast(args.len) });
    }

    /// Write the header and pool, returning the finished image.
    fn finish(self: *Build, root: u32) []const u8 {
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

fn run(image: []const u8) EvalError!Value {
    var slots: [64]Binding = undefined;
    return evalImage(image, slots[0..], null, null, arenaRef());
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
    try std.testing.expectError(EvalError.EnvFull, evalImage(image, slots[0..], null, null, arenaRef()));
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
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
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
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_ERR, rc);
    try std.testing.expectEqual(@as(usize, 0), sink_len);
}

test "a sink that refuses its reservation is backpressure, not a crash" {
    var b = Build{};
    const root = b.int(1);
    const program = b.finish(root);

    sink_refuses = true;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_ERR, rc);
    try std.testing.expectEqual(@as(usize, 0), sink_len);
    sink_refuses = false;
}

test "no sink means the host wanted only the outcome" {
    var b = Build{};
    const root = b.int(1);
    const program = b.finish(root);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rt.vehje_runtime_execute(null, program.ptr, program.len, null, null));
}

// ── family operations ─────────────────────────────────────────────────────

/// A stand-in host implementing one family: integer addition over its operands.
/// The runtime knows none of this, which is the point.
const ADD_FAMILY: u32 = 1;

/// The order operands were seen in, so left-to-right evaluation is observable.
var seen: [8]i64 = undefined;
var seen_n: usize = 0;
var host_refuses: bool = false;

fn addHost(
    userdata: ?*anyopaque,
    family: u32,
    args: [*]const VehjeOperand,
    argc: usize,
    out: *VehjeOperand,
) callconv(.c) i32 {
    _ = userdata;
    if (host_refuses or family != ADD_FAMILY) return -1;
    var sum: i64 = 0;
    var i: usize = 0;
    while (i < argc) : (i += 1) {
        if (seen_n < seen.len) {
            seen[seen_n] = args[i].payload;
            seen_n += 1;
        }
        sum += args[i].payload;
    }
    out.* = .{ .tag = rt.SCALAR_INT, .payload = sum };
    return 0;
}

fn hostOf() VehjeHost {
    seen_n = 0;
    return .{ .call = addHost, .userdata = null };
}

fn runWithHost(image: []const u8, host: ?*const VehjeHost) EvalError!Value {
    var slots: [64]Binding = undefined;
    return evalImage(image, slots[0..], host, null, arenaRef());
}

test "a family operation reaches its handler and computes" {
    // The whole point: the Core has no addition, and a program adds anyway.
    var b = Build{};
    const root = b.raw(ADD_FAMILY, &.{ b.int(2), b.int(3) });
    host_refuses = false;
    const h = hostOf();
    try std.testing.expectEqual(Value{ .int = 5 }, try runWithHost(b.finish(root), &h));
}

test "operands reach the handler left to right" {
    // A host call may have effects, so the order operands are produced in is
    // part of what the program means, not an implementation detail.
    var b = Build{};
    const root = b.raw(ADD_FAMILY, &.{ b.int(10), b.int(20), b.int(30) });
    host_refuses = false;
    const h = hostOf();
    _ = try runWithHost(b.finish(root), &h);
    try std.testing.expectEqual(@as(usize, 3), seen_n);
    try std.testing.expectEqual(@as(i64, 10), seen[0]);
    try std.testing.expectEqual(@as(i64, 20), seen[1]);
    try std.testing.expectEqual(@as(i64, 30), seen[2]);
}

test "a handler's result is an ordinary value, so operations nest" {
    // add(add(1, 2), 3) => 6
    var b = Build{};
    const inner = b.raw(ADD_FAMILY, &.{ b.int(1), b.int(2) });
    const root = b.raw(ADD_FAMILY, &.{ inner, b.int(3) });
    host_refuses = false;
    const h = hostOf();
    try std.testing.expectEqual(Value{ .int = 6 }, try runWithHost(b.finish(root), &h));
}

test "a family operation composes with the Core forms" {
    // let x = add(2, 3) in if true then x else 0  =>  5
    var b = Build{};
    const x: u32 = 99;
    const sum = b.raw(ADD_FAMILY, &.{ b.int(2), b.int(3) });
    const body = b.if_(b.boolean(true), b.varRef(x), b.int(0));
    const root = b.let(x, sum, body);
    host_refuses = false;
    const h = hostOf();
    try std.testing.expectEqual(Value{ .int = 5 }, try runWithHost(b.finish(root), &h));
}

test "a family operation with no host is refused, not crashed" {
    var b = Build{};
    const root = b.raw(ADD_FAMILY, &.{b.int(1)});
    try std.testing.expectError(EvalError.NoHost, runWithHost(b.finish(root), null));
}

test "a host that refuses an operation is distinguishable from one that answers" {
    var b = Build{};
    const root = b.raw(ADD_FAMILY, &.{b.int(1)});
    host_refuses = true;
    const h = hostOf();
    try std.testing.expectError(EvalError.HostFailed, runWithHost(b.finish(root), &h));
    host_refuses = false;
}

test "a closure operand is refused, having no scalar form" {
    var b = Build{};
    const lam = b.lambda(1, b.int(1));
    const root = b.raw(ADD_FAMILY, &.{lam});
    host_refuses = false;
    const h = hostOf();
    try std.testing.expectError(EvalError.Unsupported, runWithHost(b.finish(root), &h));
}

// ── strings ───────────────────────────────────────────────────────────────

/// A host that returns the string it was handed, so a returned string's
/// survival past the call is observable.
fn echoHost(
    userdata: ?*anyopaque,
    family: u32,
    args: [*]const VehjeOperand,
    argc: usize,
    out: *VehjeOperand,
) callconv(.c) i32 {
    _ = userdata;
    _ = family;
    if (argc != 1 or args[0].tag != rt.SCALAR_STR) return -1;
    out.* = args[0];
    return 0;
}

test "a string literal evaluates to its bytes" {
    var b = Build{};
    const root = b.str("hello");
    const v = try run(b.finish(root));
    try std.testing.expectEqualStrings("hello", v.str);
}

test "a literal reaching past the blob is refused" {
    var b = Build{};
    _ = b.str("hi");
    // A record naming a span wider than the blob holds.
    const bad = b.node(TAG_LIT, &.{ rt.LIT_STR, 0, 99 });
    try std.testing.expectError(EvalError.Corrupt, run(b.finish(bad)));
}

test "a string reaches a handler with its bytes intact" {
    var b = Build{};
    const root = b.raw(1, &.{b.str("world")});
    var scratch: [64]u8 = undefined;
    var session = Session{ .scratch = scratch[0..] };
    const h = VehjeHost{ .call = echoHost, .userdata = null };
    var slots: [16]Binding = undefined;
    const v = try evalImage(b.finish(root), slots[0..], &h, &session, arenaRef());
    try std.testing.expectEqualStrings("world", v.str);
}

test "a returned string survives the call, having been copied into scratch" {
    // The handler hands back bytes it borrowed; the value must not depend on
    // the handler's frame still existing.
    var b = Build{};
    const root = b.raw(1, &.{b.str("kept")});
    var scratch: [64]u8 = undefined;
    var session = Session{ .scratch = scratch[0..] };
    const h = VehjeHost{ .call = echoHost, .userdata = null };
    var slots: [16]Binding = undefined;
    const v = try evalImage(b.finish(root), slots[0..], &h, &session, arenaRef());
    // The value points into the scratch, not at the handler's operand.
    try std.testing.expect(v.str.ptr == scratch[0..].ptr);
    try std.testing.expectEqualStrings("kept", v.str);
}

test "returning a string with no scratch, or too little, is named" {
    var b = Build{};
    const root = b.raw(1, &.{b.str("toolong")});
    const image = b.finish(root);
    const h = VehjeHost{ .call = echoHost, .userdata = null };
    var slots: [16]Binding = undefined;

    try std.testing.expectError(EvalError.NoScratch, evalImage(image, slots[0..], &h, null, arenaRef()));

    var tiny: [3]u8 = undefined;
    var session = Session{ .scratch = tiny[0..] };
    try std.testing.expectError(EvalError.ScratchFull, evalImage(image, slots[0..], &h, &session, arenaRef()));
}

test "a string value marshals into the value image" {
    var b = Build{};
    const root = b.str("out");
    const program = b.finish(root);

    sink_refuses = false;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rc);

    const out = sink_buf[0..sink_len];
    try std.testing.expectEqual(value_image.scalarLen(3), sink_len);
    const node = 7 * 4;
    try std.testing.expectEqual(@intFromEnum(value_image.Tag.str), imageWord(out, node));
    try std.testing.expectEqualStrings("out", out[node + 6 * 4 ..]);
}

// A host that builds a record: it ignores its operands and returns
// `{ name: "ok", count: 7 }` as a value image written into a static buffer.
var record_buf: [512]u8 = undefined;

fn recordHost(
    userdata: ?*anyopaque,
    family: u32,
    args: [*]const rt.VehjeOperand,
    argc: usize,
    out: *rt.VehjeOperand,
) callconv(.c) i32 {
    _ = userdata;
    _ = family;
    _ = args;
    _ = argc;
    var slots: [8]varena.ValueNode = undefined;
    var pool: [8]u32 = undefined;
    var blob: [64]u8 = undefined;
    var a = varena.ValueArena{ .slots = slots[0..], .pool = pool[0..], .blob = blob[0..] };
    const k0 = a.allocStr("name") catch return -1;
    const v0 = a.allocStr("ok") catch return -1;
    const k1 = a.allocStr("count") catch return -1;
    const v1 = a.allocInt(7) catch return -1;
    const rec = a.allocRecord(&.{ k0, v0, k1, v1 }) catch return -1;
    const n = varena.writeTree(record_buf[0..], &a, rec) catch return -1;
    out.* = .{ .tag = rt.COMPOUND_RECORD, .payload = @intCast(n), .bytes = record_buf[0..].ptr };
    return 0;
}

/// A host that returns bytes which are not a value image at all.
fn badImageHost(
    userdata: ?*anyopaque,
    family: u32,
    args: [*]const rt.VehjeOperand,
    argc: usize,
    out: *rt.VehjeOperand,
) callconv(.c) i32 {
    _ = userdata;
    _ = family;
    _ = args;
    _ = argc;
    const junk = "not an image at all, not even close";
    out.* = .{ .tag = rt.COMPOUND_RECORD, .payload = junk.len, .bytes = junk.ptr };
    return 0;
}

test "a handler can return a record, and it becomes the program's value" {
    var b = Build{};
    const root = b.raw(1, &.{b.int(0)});
    var scratch: [512]u8 = undefined;
    var session = Session{ .scratch = scratch[0..] };
    const h = VehjeHost{ .call = recordHost, .userdata = null };
    var slots: [16]Binding = undefined;
    const arena = arenaRef();
    const v = try evalImage(b.finish(root), slots[0..], &h, &session, arena);

    // the value is a compound living in the arena, with its fields alternating
    // key then value in the child pool
    const node = arena.node(v.compound);
    try std.testing.expectEqual(value_image.Tag.record, node.tag);
    try std.testing.expectEqual(@as(u32, 4), node.children.len);
    const k1 = arena.node(arena.child(node.children, 2));
    const v1 = arena.node(arena.child(node.children, 3));
    try std.testing.expectEqualStrings("count", arena.blobOf(k1.blob));
    try std.testing.expectEqual(@as(i64, 7), v1.payload);
}

test "a returned image the reader rejects is BadImage, not a wrong value" {
    var b = Build{};
    const root = b.raw(1, &.{b.int(0)});
    var scratch: [512]u8 = undefined;
    var session = Session{ .scratch = scratch[0..] };
    const h = VehjeHost{ .call = badImageHost, .userdata = null };
    var slots: [16]Binding = undefined;
    try std.testing.expectError(
        EvalError.BadImage,
        evalImage(b.finish(root), slots[0..], &h, &session, arenaRef()),
    );
}

test "a record crosses back to the host as a value image" {
    var b = Build{};
    const root = b.raw(1, &.{b.int(0)});
    var scratch: [512]u8 = undefined;
    var session = Session{ .scratch = scratch[0..] };
    const h = VehjeHost{ .call = recordHost, .userdata = null };
    var slots: [16]Binding = undefined;
    const arena = arenaRef();
    const v = try evalImage(b.finish(root), slots[0..], &h, &session, arena);

    var out: [1024]u8 = undefined;
    const n = try varena.writeTree(out[0..], arena, v.compound);
    const r = try value_image.Reader.parse(out[0..n]);
    try r.validate();
    try std.testing.expectEqual(value_image.Tag.record, try r.tagOf(r.root));
    try std.testing.expectEqualStrings("name", try r.blob(try r.child(r.root, 0)));
}

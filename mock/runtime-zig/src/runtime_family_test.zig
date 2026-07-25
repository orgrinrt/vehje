//! Family operations: how a value reaches a handler and comes back.
//!
//! The core-form half lives in `runtime_test.zig`.

const std = @import("std");
const rt = @import("runtime.zig");
const value_image = @import("value_image.zig");
const varena = @import("value_arena.zig");
const support = @import("test_support.zig");
const imageWord = support.imageWord;
const arenaRef = support.arenaRef;
const WORD = support.WORD;
const HEADER_WORDS = support.HEADER_WORDS;
const NODE_WORDS = support.NODE_WORDS;
const MAGIC = support.MAGIC;
const TAG_LIT = support.TAG_LIT;
const TAG_VAR = support.TAG_VAR;
const TAG_LET = support.TAG_LET;
const TAG_LAMBDA = support.TAG_LAMBDA;
const TAG_APPLY = support.TAG_APPLY;
const TAG_IF = support.TAG_IF;
const LIT_UNIT = support.LIT_UNIT;
const LIT_BOOL = support.LIT_BOOL;
const LIT_INT = support.LIT_INT;
const Binding = support.Binding;
const Value = support.Value;
const EvalError = support.EvalError;
const evalImage = support.evalImage;
const TAG_RAW = support.TAG_RAW;
const VehjeOperand = support.VehjeOperand;
const Session = support.Session;
const VehjeHost = support.VehjeHost;
const Build = support.Build;
const run = support.run;
const testReserve = support.testReserve;
const testCommit = support.testCommit;
const testSink = support.testSink;

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

    support.sink_refuses = false;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rc);

    const out = support.sink_buf[0..support.sink_len];
    try std.testing.expectEqual(value_image.scalarLen(3), support.sink_len);
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


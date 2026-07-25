//! Evaluate the Core image, computing the arithmetic family in the runtime.
//!
//! The point of this file is the `TAG_RAW` arm. The shipped runtime dispatches
//! every family operation to a host callback and refuses with `NoHost` when none
//! is supplied (`mock/runtime-zig/src/runtime.zig:410-415`), which the standing
//! call corrects: the host configures the runtime and lends it allocations, and
//! does not supply arithmetic. Here the operation's meaning arrives as a program
//! over a closed primitive vocabulary and is specialised at comptime, so nothing
//! is called out to.
//!
//! This is a sketch evaluator over the subset the parser emits, not a second
//! implementation of the shipped one. It exists to prove the shape the source
//! changelist will then implement in `runtime.zig` itself.

const std = @import("std");
const cl = @import("clause.zig");
const chk = @import("check.zig");

// The closed primitive vocabulary, as in
// `mock/research/sketches/202607260800_four-way-projection/`.
const PUSH_ARG: u8 = 0x01;
const ADD: u8 = 0x10;
const LT: u8 = 0x11;
const MUL: u8 = 0x12;
const SUB: u8 = 0x13;

/// The operation table: what the language definition emits as data, indexed by
/// opcode. The framework declares none of this; the consumer language does.
const OPS = [_][]const u8{
    &[_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, ADD }, // OP_ADD
    &[_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, SUB }, // OP_SUB
    &[_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, MUL }, // OP_MUL
    &[_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, LT }, // OP_LT
};

/// One operation's program, specialised. `prog` is comptime, so the walk and the
/// dispatch vanish and only the arithmetic survives.
fn runOp(comptime prog: []const u8, args: []const i64) i64 {
    var stack: [8]i64 = undefined;
    comptime var sp: usize = 0;
    comptime var pc: usize = 0;
    inline while (pc < prog.len) {
        const op = comptime prog[pc];
        switch (op) {
            PUSH_ARG => {
                const n = comptime prog[pc + 1];
                stack[sp] = args[n];
                comptime sp += 1;
                comptime pc += 2;
            },
            ADD => {
                stack[sp - 2] = stack[sp - 2] + stack[sp - 1];
                comptime sp -= 1;
                comptime pc += 1;
            },
            SUB => {
                stack[sp - 2] = stack[sp - 2] - stack[sp - 1];
                comptime sp -= 1;
                comptime pc += 1;
            },
            MUL => {
                stack[sp - 2] = stack[sp - 2] * stack[sp - 1];
                comptime sp -= 1;
                comptime pc += 1;
            },
            LT => {
                stack[sp - 2] = if (stack[sp - 2] < stack[sp - 1]) 1 else 0;
                comptime sp -= 1;
                comptime pc += 1;
            },
            else => @compileError("unknown primitive"),
        }
    }
    return stack[0];
}

/// The specialised dispatch: one arm per operation, each fully specialised, and
/// an unknown opcode is a refusal rather than a fallthrough.
fn applyOp(op: u32, args: []const i64) Error!i64 {
    inline for (OPS, 0..) |prog, i| {
        if (op == @as(u32, i)) return runOp(prog, args);
    }
    return Error.UnknownOperation;
}

pub const Error = error{
    Corrupt,
    Unbound,
    Unsupported,
    UnknownOperation,
    EnvFull,
    TooManyOperands,
    NotCallable,
    NotAnInt,
    NotARecord,
    NoSuchField,
    RecArenaFull,
    BadRecordKey,
    NotASequence,
    OutOfRange,
    ValArenaFull,
};

/// A closure names the environment it was written in, which is why the
/// environment is a linked chain rather than a stack that pops: a function may
/// outlive the scope that produced it and must still read what it captured.
const Closure = struct { param: u32, body: u32, env: u32 };

/// One field of a record: its name and its value. The record itself is a span
/// of these, which keeps a record value two words wide.
const RecEntry = struct { key: []const u8, val: Value };

const Value = union(enum) {
    int: i64,
    record: struct { start: u32, len: u32 },
    seq: struct { start: u32, len: u32 },
    /// A zero-copy slice of the image's blob. The wire already carries the
    /// bytes, so producing a string value costs nothing.
    str: []const u8,
    closure: Closure,

    fn asInt(self: Value) Error!i64 {
        return switch (self) {
            .int => |v| v,
            else => Error.NotAnInt,
        };
    }
};

const ENV_NIL: u32 = 0xFFFF_FFFF;

const Binding = struct { sym: u32, val: Value, parent: u32 };

/// A caller-lent bump arena of linked bindings. Nothing pops; a binding names
/// its parent, so capturing an environment is recording one index.
pub const Env = struct {
    slots: []Binding,
    n: u32 = 0,
    /// Record fields live in their own caller-lent arena. Nothing is freed
    /// within a run, matching the binding arena above.
    recs: []RecEntry,
    nrec: u32 = 0,
    /// Sequence elements, in their own caller-lent arena for the same reason
    /// record fields have one.
    vals: []Value,
    nval: u32 = 0,

    fn push(self: *Env, sym: u32, val: Value, parent: u32) Error!u32 {
        if (self.n >= self.slots.len) return Error.EnvFull;
        self.slots[self.n] = .{ .sym = sym, .val = val, .parent = parent };
        self.n += 1;
        return self.n - 1;
    }

    fn lookup(self: *const Env, sym: u32, cur: u32) Error!Value {
        var i = cur;
        while (i != ENV_NIL) {
            if (self.slots[i].sym == sym) return self.slots[i].val;
            i = self.slots[i].parent;
        }
        return Error.Unbound;
    }
};

const Image = struct {
    bytes: []const u8,
    node_count: u32,
    pool_base: usize,
    pool_count: u32,
    blob_base: usize,
    blob_len: u32,
    root: u32,

    fn parse(bytes: []const u8) Error!Image {
        if (bytes.len < cl.HEADER_WORDS * cl.WORD) return Error.Corrupt;
        if (rd(bytes, 0) != cl.MAGIC) return Error.Corrupt;
        const node_count = rd(bytes, 3 * cl.WORD);
        const pool_count = rd(bytes, 4 * cl.WORD);
        const blob_len = rd(bytes, 5 * cl.WORD);
        const root = rd(bytes, 6 * cl.WORD);
        const pool_base = cl.HEADER_WORDS * cl.WORD + @as(usize, node_count) * cl.NODE_WORDS * cl.WORD;
        const blob_base = pool_base + @as(usize, pool_count) * cl.WORD;
        if (blob_base + @as(usize, blob_len) > bytes.len) return Error.Corrupt;
        return .{
            .bytes = bytes,
            .node_count = node_count,
            .pool_base = pool_base,
            .pool_count = pool_count,
            .blob_base = blob_base,
            .blob_len = blob_len,
            .root = root,
        };
    }

    /// Bounds-checked because the image is untrusted even when this process
    /// produced it. A decoder that trusts its input is the wrong shape to grow
    /// into one that reads an image off disk.
    fn blob(self: *const Image, off: u32, len: u32) Error![]const u8 {
        if (@as(usize, off) + @as(usize, len) > self.blob_len) return Error.Corrupt;
        return self.bytes[self.blob_base + off .. self.blob_base + off + len];
    }

    fn word(self: *const Image, idx: u32, slot: usize) Error!u32 {
        if (idx >= self.node_count) return Error.Corrupt;
        return rd(self.bytes, cl.HEADER_WORDS * cl.WORD + (@as(usize, idx) * cl.NODE_WORDS + slot) * cl.WORD);
    }

    fn pooled(self: *const Image, at: u32) Error!u32 {
        if (at >= self.pool_count) return Error.Corrupt;
        return rd(self.bytes, self.pool_base + @as(usize, at) * cl.WORD);
    }
};

fn rd(bytes: []const u8, at: usize) u32 {
    return std.mem.readInt(u32, bytes[at..][0..4], .little);
}

fn evalNode(img: *const Image, idx: u32, env: *Env, cur: u32) Error!Value {
    switch (try img.word(idx, 0)) {
        cl.TAG_LIT => switch (try img.word(idx, 1)) {
            cl.LIT_INT => {
                const lo = try img.word(idx, 2);
                const hi = try img.word(idx, 3);
                return Value{ .int = @bitCast((@as(u64, hi) << 32) | @as(u64, lo)) };
            },
            cl.LIT_STR => return Value{ .str = try img.blob(try img.word(idx, 2), try img.word(idx, 3)) },
            else => return Error.Unsupported,
        },
        cl.TAG_VAR => return env.lookup(try img.word(idx, 1), cur),
        cl.TAG_LET => {
            const rec = (try img.word(idx, 1)) != 0;
            const name = try img.word(idx, 2);
            const value_ref = try img.word(idx, 3);
            const body_ref = try img.word(idx, 4);
            if (rec) {
                // In scope for its own value, so a closure the value produces
                // captures a chain that already names it.
                const slot = try env.push(name, Value{ .int = 0 }, cur);
                env.slots[slot].val = try evalNode(img, value_ref, env, slot);
                return evalNode(img, body_ref, env, slot);
            }
            const v = try evalNode(img, value_ref, env, cur);
            return evalNode(img, body_ref, env, try env.push(name, v, cur));
        },
        cl.TAG_LAMBDA => return Value{ .closure = .{
            .param = try img.word(idx, 1),
            .body = try img.word(idx, 2),
            .env = cur,
        } },
        cl.TAG_APPLY => {
            var f = try evalNode(img, try img.word(idx, 1), env, cur);
            const start = try img.word(idx, 2);
            const len = try img.word(idx, 3);
            var k: u32 = 0;
            while (k < len) : (k += 1) {
                const arg = try evalNode(img, try img.pooled(start + k), env, cur);
                const c = switch (f) {
                    .closure => |c| c,
                    else => return Error.NotCallable,
                };
                // The body runs under the environment the lambda captured, not
                // the caller's, which is what makes capture lexical.
                f = try evalNode(img, c.body, env, try env.push(c.param, arg, c.env));
            }
            return f;
        },
        cl.TAG_PROJECT => {
            const base = try evalNode(img, try img.word(idx, 1), env, cur);
            const key = try img.blob(try img.word(idx, 2), try img.word(idx, 3));
            const r = switch (base) {
                .record => |r| r,
                else => return Error.NotARecord,
            };
            var k: u32 = 0;
            while (k < r.len) : (k += 1) {
                const e = env.recs[r.start + k];
                if (std.mem.eql(u8, e.key, key)) return e.val;
            }
            return Error.NoSuchField;
        },
        cl.TAG_IF => {
            const c = try (try evalNode(img, try img.word(idx, 1), env, cur)).asInt();
            return evalNode(img, try img.word(idx, if (c != 0) 2 else 3), env, cur);
        },
        cl.TAG_RAW => {
            if ((try img.word(idx, 1)) != cl.ARITH) return Error.Unsupported;
            const start = try img.word(idx, 2);
            const len = try img.word(idx, 3);
            // Operand 0 carries the opcode; the rest are the arguments, evaluated
            // left to right because operand order is observable.
            const op: u32 = @intCast(try (try evalNode(img, try img.pooled(start), env, cur)).asInt());
            // The vocabulary has two shapes in it, and this is where that shows:
            // a constructor takes a variable number of operands and yields a
            // compound, so it cannot go through the fixed-arity scalar path.
            if (op == cl.OP_MAKE_REC) {
                const first = env.nrec;
                var k: u32 = 1;
                while (k + 1 < len) : (k += 2) {
                    const key = switch (try evalNode(img, try img.pooled(start + k), env, cur)) {
                        .str => |t| t,
                        else => return Error.BadRecordKey,
                    };
                    const val = try evalNode(img, try img.pooled(start + k + 1), env, cur);
                    if (env.nrec >= env.recs.len) return Error.RecArenaFull;
                    env.recs[env.nrec] = .{ .key = key, .val = val };
                    env.nrec += 1;
                }
                return Value{ .record = .{ .start = first, .len = env.nrec - first } };
            }
            if (op == cl.OP_MAKE_SEQ) {
                const first = env.nval;
                var k: u32 = 1;
                while (k < len) : (k += 1) {
                    const v = try evalNode(img, try img.pooled(start + k), env, cur);
                    if (env.nval >= env.vals.len) return Error.ValArenaFull;
                    env.vals[env.nval] = v;
                    env.nval += 1;
                }
                return Value{ .seq = .{ .start = first, .len = env.nval - first } };
            }
            if (op == cl.OP_LEN) {
                const s0 = try evalNode(img, try img.pooled(start + 1), env, cur);
                return switch (s0) {
                    .seq => |q| Value{ .int = @intCast(q.len) },
                    else => Error.NotASequence,
                };
            }
            if (op == cl.OP_AT) {
                const s0 = try evalNode(img, try img.pooled(start + 1), env, cur);
                const i = try (try evalNode(img, try img.pooled(start + 2), env, cur)).asInt();
                const q = switch (s0) {
                    .seq => |q| q,
                    else => return Error.NotASequence,
                };
                if (i < 0 or @as(u32, @intCast(i)) >= q.len) return Error.OutOfRange;
                return env.vals[q.start + @as(u32, @intCast(i))];
            }
            if (op == cl.OP_PUSH) {
                const s0 = try evalNode(img, try img.pooled(start + 1), env, cur);
                const v = try evalNode(img, try img.pooled(start + 2), env, cur);
                const q = switch (s0) {
                    .seq => |q| q,
                    else => return Error.NotASequence,
                };
                // A fresh span, because the source sequence is immutable and may
                // still be referenced. Copying is the honest cost of that.
                const first = env.nval;
                if (@as(usize, env.nval) + q.len + 1 > env.vals.len) return Error.ValArenaFull;
                var k: u32 = 0;
                while (k < q.len) : (k += 1) {
                    env.vals[env.nval] = env.vals[q.start + k];
                    env.nval += 1;
                }
                env.vals[env.nval] = v;
                env.nval += 1;
                return Value{ .seq = .{ .start = first, .len = q.len + 1 } };
            }
            if (len < 1 or len > 4) return Error.TooManyOperands;
            var args: [3]i64 = undefined;
            var k: u32 = 1;
            while (k < len) : (k += 1) {
                args[k - 1] = try (try evalNode(img, try img.pooled(start + k), env, cur)).asInt();
            }
            return Value{ .int = try applyOp(op, args[0 .. len - 1]) };
        },
        else => return Error.Unsupported,
    }
}

/// Source text to a value, with no Rust and no host anywhere in the path.
pub fn run(src: []const u8) !i64 {
    var node_buf: [8192 * cl.NODE_WORDS]u32 = undefined;
    var pool_buf: [4096]u32 = undefined;
    var name_buf: [512][]const u8 = undefined;
    var blob_buf: [8192]u8 = undefined;
    var image: [524288]u8 = undefined;
    var slots: [8192]Binding = undefined;
    var recs: [2048]RecEntry = undefined;
    var vals: [8192]Value = undefined;

    var b = cl.Builder{ .nodes = &node_buf, .pool = &pool_buf, .blob = &blob_buf };
    var names = cl.Names{ .buf = &name_buf };
    var p = try cl.Parser.init(src, &b, &names);
    const root = try p.program();
    const len = try cl.writeImage(&b, root, &image);

    // Prove, then evaluate. The canon's centre of gravity is prove-then-erase,
    // and an evaluator that runs whatever it is handed does the erasing without
    // the proving. This is the junction the shipped pipeline still lacks:
    // `serialize` documents itself as taking a checked program while its
    // signature takes a bare arena, so nothing enforces the order there.
    var types: [32768]chk.Ty = undefined;
    var subst: [8192]u32 = undefined;
    var tenv: [4096]chk.TyBinding = undefined;
    var tfields: [4096]chk.Field = undefined;
    var ctx = chk.Ctx{ .types = &types, .subst = &subst, .env = &tenv, .fields = &tfields };
    _ = try chk.check(image[0..len], &ctx);

    const img = try Image.parse(image[0..len]);
    var env = Env{ .slots = &slots, .recs = &recs, .vals = &vals };
    return (try evalNode(&img, img.root, &env, ENV_NIL)).asInt();
}

/// The standard library, written in Clause. Embedded rather than read at run
/// time so the tests exercise the same bytes that would ship.
pub const STD = @embedFile("std.clause");

/// Run a program with the standard library in scope, which is simply the
/// library's text followed by the program's: a `fn` declaration's body is
/// everything after it, so prepending the library nests the program inside it.
pub fn runWithStd(src: []const u8) !i64 {
    var joined: [16384]u8 = undefined;
    if (STD.len + 1 + src.len > joined.len) return Error.Corrupt;
    @memcpy(joined[0..STD.len], STD);
    joined[STD.len] = '\n';
    @memcpy(joined[STD.len + 1 ..][0..src.len], src);
    return run(joined[0 .. STD.len + 1 + src.len]);
}

/// Source text to a string value. Separate from `run` because a string value
/// borrows the image, which lives in this frame, so the bytes are copied into
/// the caller's buffer rather than returned as a dangling slice.
pub fn runStr(src: []const u8, out: []u8) ![]const u8 {
    var node_buf: [8192 * cl.NODE_WORDS]u32 = undefined;
    var pool_buf: [4096]u32 = undefined;
    var name_buf: [512][]const u8 = undefined;
    var blob_buf: [8192]u8 = undefined;
    var image: [524288]u8 = undefined;
    var slots: [8192]Binding = undefined;
    var recs: [2048]RecEntry = undefined;
    var vals: [8192]Value = undefined;

    var b = cl.Builder{ .nodes = &node_buf, .pool = &pool_buf, .blob = &blob_buf };
    var names = cl.Names{ .buf = &name_buf };
    var p = try cl.Parser.init(src, &b, &names);
    const root = try p.program();
    const len = try cl.writeImage(&b, root, &image);

    var types: [32768]chk.Ty = undefined;
    var subst: [8192]u32 = undefined;
    var tenv: [4096]chk.TyBinding = undefined;
    var tfields: [4096]chk.Field = undefined;
    var ctx = chk.Ctx{ .types = &types, .subst = &subst, .env = &tenv, .fields = &tfields };
    _ = try chk.check(image[0..len], &ctx);

    const img = try Image.parse(image[0..len]);
    var env = Env{ .slots = &slots, .recs = &recs, .vals = &vals };
    return switch (try evalNode(&img, img.root, &env, ENV_NIL)) {
        .str => |t| blk: {
            @memcpy(out[0..t.len], t);
            break :blk out[0..t.len];
        },
        else => Error.NotAnInt,
    };
}

test "a clause program computes, with no host and no rust" {
    try std.testing.expectEqual(@as(i64, 7), try run("1 + 2 * 3"));
    try std.testing.expectEqual(@as(i64, 9), try run("(1 + 2) * 3"));
    try std.testing.expectEqual(@as(i64, -1), try run("2 - 3"));
}

test "bindings are in scope for what follows them" {
    try std.testing.expectEqual(@as(i64, 20), try run("let x = 2; let y = 3; x * y + 14"));
    try std.testing.expectEqual(@as(i64, 5), try run("let x = 2; let y = x + 3; y"));
}

test "a later binding shadows an earlier one" {
    try std.testing.expectEqual(@as(i64, 9), try run("let x = 2; let x = 9; x"));
}

test "the conditional picks a branch and the comparison is a family operation" {
    try std.testing.expectEqual(@as(i64, 20), try run("let x = 2; let y = 3; if x < y { x * 10 } else { y * 10 }"));
    try std.testing.expectEqual(@as(i64, 30), try run("let x = 5; let y = 3; if x < y { x * 10 } else { y * 10 }"));
}

test "a branch body is itself a program, so it may bind" {
    try std.testing.expectEqual(@as(i64, 12), try run("if 1 < 2 { let k = 4; k * 3 } else { 0 }"));
}

test "an unbound name is refused rather than defaulted" {
    try std.testing.expectError(Error.Unbound, run("x + 1"));
}

test "a function is declared and called" {
    try std.testing.expectEqual(@as(i64, 7), try run("fn double(n) { n * 2 } double(3) + 1"));
    try std.testing.expectEqual(@as(i64, 11), try run("fn add(a, b) { a + b } add(4, 7)"));
}

test "a function calls another function" {
    try std.testing.expectEqual(@as(i64, 20), try run(
        \\fn double(n) { n * 2 }
        \\fn quad(n) { double(double(n)) }
        \\quad(5)
    ));
}

test "recursion terminates because a fn binds recursively" {
    try std.testing.expectEqual(@as(i64, 120), try run(
        \\fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } }
        \\fact(5)
    ));
}

test "a closure captures where it was written, not where it is called" {
    try std.testing.expectEqual(@as(i64, 7), try run(
        \\fn adder(a) { fn inner(b) { a + b } inner }
        \\adder(3)(4)
    ));
}

test "partial application falls out of currying rather than being a feature" {
    try std.testing.expectEqual(@as(i64, 11), try run(
        \\fn add(a, b) { a + b }
        \\add(4)(7)
    ));
}

test "an ill-typed program is refused before it can evaluate" {
    // The evaluator would happily run this: a bool is 0 or 1 in an i64, so the
    // conditional would pick a branch. The gate is what stops it.
    try std.testing.expectError(chk.Error.Mismatch, run("if 1 { 1 } else { 2 }"));
    try std.testing.expectError(chk.Error.Mismatch, run("1 + (2 < 3)"));
}

test "a string is a value, carried in the image blob" {
    var out: [64]u8 = undefined;
    try std.testing.expectEqualStrings("hello", try runStr("\"hello\"", &out));
    try std.testing.expectEqualStrings("bound", try runStr("let s = \"bound\"; s", &out));
    try std.testing.expectEqualStrings("yes", try runStr("if 1 < 2 { \"yes\" } else { \"no\" }", &out));
}

test "a string flows through a function like any other value" {
    var out: [64]u8 = undefined;
    try std.testing.expectEqualStrings("x", try runStr("fn id(v) { v } id(\"x\")", &out));
}

test "a string is refused where a number belongs, before evaluation" {
    var out: [64]u8 = undefined;
    try std.testing.expectError(chk.Error.Mismatch, run("\"a\" + 1"));
    try std.testing.expectError(chk.Error.Mismatch, runStr("if \"a\" { \"x\" } else { \"y\" }", &out));
    try std.testing.expectError(chk.Error.Mismatch, runStr("if 1 < 2 { \"a\" } else { 1 }", &out));
}

test "a record is built and a field is read back" {
    try std.testing.expectEqual(@as(i64, 7), try run("let r = { a: 7, b: 9 }; r.a"));
    try std.testing.expectEqual(@as(i64, 9), try run("let r = { a: 7, b: 9 }; r.b"));
    try std.testing.expectEqual(@as(i64, 16), try run("let r = { a: 7, b: 9 }; r.a + r.b"));
}

test "a record field can be any value, including another record" {
    var out: [64]u8 = undefined;
    try std.testing.expectEqualStrings("ok", try runStr("let r = { name: \"ok\", n: 1 }; r.name", &out));
    try std.testing.expectEqual(@as(i64, 3), try run("let r = { inner: { deep: 3 } }; r.inner.deep"));
}

test "a record flows through a function and is projected after" {
    try std.testing.expectEqual(@as(i64, 5), try run("fn id(v) { v } id({ k: 5 }).k"));
}

test "a missing field is refused before evaluation" {
    try std.testing.expectError(chk.Error.NoSuchField, run("let r = { a: 1 }; r.b"));
}

test "projecting a non-record is refused before evaluation" {
    try std.testing.expectError(chk.Error.Mismatch, run("let n = 1; n.a"));
}

test "a record literal is refused in an if-condition, where a block belongs" {
    try std.testing.expectError(cl.Error.UnexpectedToken, run("if { a: 1 } { 1 } else { 2 }"));
}

test "a sequence is built and eliminated" {
    try std.testing.expectEqual(@as(i64, 3), try run("len([1, 2, 3])"));
    try std.testing.expectEqual(@as(i64, 2), try run("at([1, 2, 3], 1)"));
    try std.testing.expectEqual(@as(i64, 0), try run("len([])"));
}

test "real clause code walks a sequence by recursion" {
    // The first thing in this language that looks like a standard-library
    // function rather than a demonstration.
    try std.testing.expectEqual(@as(i64, 6), try run(
        \\fn sum_from(s, i) { if i < len(s) { at(s, i) + sum_from(s, i + 1) } else { 0 } }
        \\fn sum(s) { sum_from(s, 0) }
        \\sum([1, 2, 3])
    ));
    try std.testing.expectEqual(@as(i64, 24), try run(
        \\fn prod_from(s, i) { if i < len(s) { at(s, i) * prod_from(s, i + 1) } else { 1 } }
        \\fn product(s) { prod_from(s, 0) }
        \\product([1, 2, 3, 4])
    ));
}

test "a sequence may hold records, and its elements project" {
    try std.testing.expectEqual(@as(i64, 9), try run("at([{ v: 9 }, { v: 8 }], 0).v"));
}

test "a heterogeneous sequence is refused, because a sequence is homogeneous" {
    try std.testing.expectError(chk.Error.Mismatch, run("len([1, \"two\"])"));
}

test "indexing a non-sequence is refused before evaluation" {
    try std.testing.expectError(chk.Error.Mismatch, run("at(5, 0)"));
    try std.testing.expectError(chk.Error.Mismatch, run("len({ a: 1 })"));
}

test "an out-of-range index is a runtime refusal, since length is not in the type" {
    try std.testing.expectError(Error.OutOfRange, run("at([1, 2], 5)"));
}

test "push extends a sequence without mutating it" {
    try std.testing.expectEqual(@as(i64, 3), try run("len(push([1, 2], 3))"));
    try std.testing.expectEqual(@as(i64, 3), try run("at(push([1, 2], 3), 2)"));
    // The source is unchanged, because values are immutable.
    try std.testing.expectEqual(@as(i64, 2), try run("let s = [1, 2]; let t = push(s, 3); len(s)"));
}

test "push is homogeneous with its sequence" {
    try std.testing.expectError(chk.Error.Mismatch, run("len(push([1, 2], \"three\"))"));
}

test "the standard library is written in clause and it runs" {
    try std.testing.expectEqual(@as(i64, 6), try runWithStd("sum([1, 2, 3])"));
    try std.testing.expectEqual(@as(i64, 24), try runWithStd("product([1, 2, 3, 4])"));
    try std.testing.expectEqual(@as(i64, 10), try runWithStd("sum(range(5))"));
}

test "map and filter are higher-order and generic" {
    try std.testing.expectEqual(@as(i64, 12), try runWithStd(
        \\fn double(n) { n * 2 }
        \\sum(map(double, [1, 2, 3]))
    ));
    try std.testing.expectEqual(@as(i64, 2), try runWithStd(
        \\fn big(n) { 2 < n }
        \\len(filter(big, [1, 2, 3, 4]))
    ));
    try std.testing.expectEqual(@as(i64, 2), try runWithStd(
        \\fn positive(n) { 0 < n }
        \\count(positive, [1, 0, 3])
    ));
}

test "map changes the element type, so misusing the result is refused" {
    // map(positive, ints) is a sequence of booleans, so feeding it back to a
    // predicate over integers is a type error. The checker finding this is the
    // point: it means map is generic in two variables, not one.
    try std.testing.expectError(chk.Error.Mismatch, runWithStd(
        \\fn positive(n) { 0 < n }
        \\count(positive, map(positive, [1, 2, 3]))
    ));
}

test "fold is the shape the others are special cases of" {
    try std.testing.expectEqual(@as(i64, 15), try runWithStd("fold(add2, 0, [1, 2, 3, 4, 5])"));
    try std.testing.expectEqual(@as(i64, 5), try runWithStd("fold(max2, 0, [3, 5, 1])"));
}

test "reverse preserves length and flips order" {
    try std.testing.expectEqual(@as(i64, 3), try runWithStd("len(reverse([1, 2, 3]))"));
    try std.testing.expectEqual(@as(i64, 1), try runWithStd("at(reverse([1, 2, 3]), 2)"));
    try std.testing.expectEqual(@as(i64, 3), try runWithStd("at(reverse([1, 2, 3]), 0)"));
}

test "all and any quantify over a sequence" {
    try std.testing.expectEqual(@as(i64, 1), try runWithStd(
        \\fn positive(n) { 0 < n }
        \\if all(positive, [1, 2, 3]) { 1 } else { 0 }
    ));
    try std.testing.expectEqual(@as(i64, 0), try runWithStd(
        \\fn positive(n) { 0 < n }
        \\if all(positive, [1, 0, 3]) { 1 } else { 0 }
    ));
    try std.testing.expectEqual(@as(i64, 1), try runWithStd(
        \\fn big(n) { 2 < n }
        \\if any(big, [1, 2, 3]) { 1 } else { 0 }
    ));
}

test "the library composes with itself" {
    // range(5) is [0,1,2,3,4]; 1 < n keeps 2,3,4; doubled is 4,6,8; sum 18.
    try std.testing.expectEqual(@as(i64, 18), try runWithStd(
        \\fn double(n) { n * 2 }
        \\fn odd_ish(n) { 1 < n }
        \\sum(map(double, filter(odd_ish, range(5))))
    ));
}

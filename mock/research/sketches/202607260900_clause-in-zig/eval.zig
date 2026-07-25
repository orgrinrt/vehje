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
};

/// A closure names the environment it was written in, which is why the
/// environment is a linked chain rather than a stack that pops: a function may
/// outlive the scope that produced it and must still read what it captured.
const Closure = struct { param: u32, body: u32, env: u32 };

const Value = union(enum) {
    int: i64,
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
    root: u32,

    fn parse(bytes: []const u8) Error!Image {
        if (bytes.len < cl.HEADER_WORDS * cl.WORD) return Error.Corrupt;
        if (rd(bytes, 0) != cl.MAGIC) return Error.Corrupt;
        const node_count = rd(bytes, 3 * cl.WORD);
        const pool_count = rd(bytes, 4 * cl.WORD);
        const root = rd(bytes, 6 * cl.WORD);
        const pool_base = cl.HEADER_WORDS * cl.WORD + @as(usize, node_count) * cl.NODE_WORDS * cl.WORD;
        if (pool_base + @as(usize, pool_count) * cl.WORD > bytes.len) return Error.Corrupt;
        return .{ .bytes = bytes, .node_count = node_count, .pool_base = pool_base, .pool_count = pool_count, .root = root };
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
        cl.TAG_LIT => {
            if ((try img.word(idx, 1)) != cl.LIT_INT) return Error.Unsupported;
            const lo = try img.word(idx, 2);
            const hi = try img.word(idx, 3);
            return Value{ .int = @bitCast((@as(u64, hi) << 32) | @as(u64, lo)) };
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
        cl.TAG_IF => {
            const c = try (try evalNode(img, try img.word(idx, 1), env, cur)).asInt();
            return evalNode(img, try img.word(idx, if (c != 0) 2 else 3), env, cur);
        },
        cl.TAG_RAW => {
            if ((try img.word(idx, 1)) != cl.ARITH) return Error.Unsupported;
            const start = try img.word(idx, 2);
            const len = try img.word(idx, 3);
            if (len < 1 or len > 4) return Error.TooManyOperands;
            // Operand 0 carries the opcode; the rest are the arguments, evaluated
            // left to right because operand order is observable.
            const op: u32 = @intCast(try (try evalNode(img, try img.pooled(start), env, cur)).asInt());
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
    var node_buf: [512 * cl.NODE_WORDS]u32 = undefined;
    var pool_buf: [256]u32 = undefined;
    var name_buf: [64][]const u8 = undefined;
    var image: [16384]u8 = undefined;
    var slots: [512]Binding = undefined;

    var b = cl.Builder{ .nodes = &node_buf, .pool = &pool_buf };
    var names = cl.Names{ .buf = &name_buf };
    var p = try cl.Parser.init(src, &b, &names);
    const root = try p.program();
    const len = try cl.writeImage(&b, root, &image);

    // Prove, then evaluate. The canon's centre of gravity is prove-then-erase,
    // and an evaluator that runs whatever it is handed does the erasing without
    // the proving. This is the junction the shipped pipeline still lacks:
    // `serialize` documents itself as taking a checked program while its
    // signature takes a bare arena, so nothing enforces the order there.
    var types: [1024]chk.Ty = undefined;
    var subst: [256]u32 = undefined;
    var tenv: [256]chk.TyBinding = undefined;
    var ctx = chk.Ctx{ .types = &types, .subst = &subst, .env = &tenv };
    _ = try chk.check(image[0..len], &ctx);

    const img = try Image.parse(image[0..len]);
    var env = Env{ .slots = &slots };
    return (try evalNode(&img, img.root, &env, ENV_NIL)).asInt();
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

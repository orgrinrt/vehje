//! Zig runtime: the Tier::Arena tree-walk interpreter.
//!
//! Decodes the flat serialized IR arena that `vehje-runtime-abi` emits (a
//! seven-word header, then fixed seven-word node records, then the flat child
//! pool, then the string blob, little-endian) and evaluates it directly, the
//! reference semantics of the `Arena` tier. The `Bytecode` tier (a
//! CFG-of-blocks linear form) is a later optimisation.
//!
//! Binders are matched by their `Sym` bits (the wire's stable binder identity,
//! one word per binder), not by resolved text, so a `Var` binds to its `Let`
//! by integer equality.
//!
//! Environments are a caller-lent bump arena of linked bindings, never a
//! popping stack: a binding names its parent, so capturing an environment is
//! recording one index. A closure that outlives the scope it was written in
//! still reads the bindings it captured, which a popping stack cannot promise.
//! Nothing is freed within a run; exhausting the lent arena is `EnvFull`.
//!
//! The C ABI entry (`vehje_runtime_execute`) returns a plain `i32` result code.
//! The Rust-side `VehjeResult` is `#[repr(i32)]`; a plain `i32` on both sides
//! keeps the scalar-in-register calling convention portable. Returning the
//! produced value across the ABI (into a host value arena) is the next gate.

const std = @import("std");

pub const VEHJE_RESULT_OK: i32 = 0;
pub const VEHJE_RESULT_ERR: i32 = -1;
pub const VEHJE_RESULT_NULL_HANDLE: i32 = -2;
pub const VEHJE_RESULT_INVALID_INPUT: i32 = -3;

// Wire format constants, mirroring `vehje-runtime-abi/src/wire/serialize.rs`.
const WORD: usize = 4;
const HEADER_WORDS: usize = 7;
const NODE_WORDS: usize = 7;
const MAGIC: u32 = 0x3048_4556; // "VEH0" little-endian

// Node tag codes.
const TAG_LIT: u32 = 0;
const TAG_VAR: u32 = 1;
const TAG_LET: u32 = 2;
const TAG_LAMBDA: u32 = 3;
const TAG_APPLY: u32 = 4;
const TAG_PROJECT: u32 = 5;
const TAG_IF: u32 = 6;
const TAG_MATCH: u32 = 7;
const TAG_ITER: u32 = 8;
const TAG_INTERP: u32 = 9;
const TAG_RAW: u32 = 10;
const TAG_HANDLE: u32 = 11;

// Literal sub-tag codes.
const LIT_UNIT: u32 = 0;
const LIT_BOOL: u32 = 1;
const LIT_INT: u32 = 2;
const LIT_STR: u32 = 3;

/// The empty environment: the parent of a binding introduced at top level.
pub const ENV_NIL: u32 = 0xFFFF_FFFF;

/// A function value: the parameter's `Sym` bits, the body node, and the
/// environment captured where the `Lambda` was evaluated.
pub const Closure = struct { param: u32, body: u32, env: u32 };

/// A runtime value. Strings and family values land with their forms.
///
/// A closure is deliberately runtime-internal: it names an environment inside
/// this run, so it has no representation in the host value arena and cannot
/// cross the ABI.
pub const Value = union(enum) {
    unit,
    boolean: bool,
    int: i64,
    closure: Closure,
};

pub const EvalError = error{
    Unbound, // a Var whose binder is not in scope
    Unsupported, // a form or literal kind this tier does not evaluate
    NotCallable, // an Apply whose callee did not evaluate to a closure
    EnvFull, // the caller-lent environment arena is exhausted (no alloc)
    Corrupt, // a node index, pool index, or record past the image
};

/// One environment binding: a binder's `Sym` bits, its value, and the binding
/// it extends. A chain of these is an environment; capturing one is recording
/// its index.
const Binding = struct { sym: u32, val: Value, parent: u32 };

/// The caller-lent environment arena. Bump-allocated, never popped, so a
/// captured environment stays readable for the whole run.
const Env = struct {
    slots: []Binding,
    len: usize = 0,

    /// Extend `parent` with `sym = val`, returning the new chain head.
    fn push(self: *Env, sym: u32, val: Value, parent: u32) EvalError!u32 {
        if (self.len >= self.slots.len) return EvalError.EnvFull;
        self.slots[self.len] = .{ .sym = sym, .val = val, .parent = parent };
        self.len += 1;
        return @intCast(self.len - 1);
    }

    /// The value `sym` is bound to in the chain rooted at `head`.
    fn lookup(self: *const Env, head: u32, sym: u32) EvalError!Value {
        var cur = head;
        while (cur != ENV_NIL) {
            if (cur >= self.len) return EvalError.Corrupt;
            const b = self.slots[cur];
            if (b.sym == sym) return b.val;
            cur = b.parent;
        }
        return EvalError.Unbound;
    }
};

/// A decoded wire image: the bytes plus the section bases the header fixes.
const Image = struct {
    bytes: []const u8,
    node_count: u32,
    pool_base: usize,
    pool_count: u32,
    root: u32,

    fn parse(bytes: []const u8) EvalError!Image {
        if (bytes.len < HEADER_WORDS * WORD) return EvalError.Corrupt;
        if ((try readU32(bytes, 0)) != MAGIC) return EvalError.Corrupt;
        const node_count = try readU32(bytes, 3 * WORD);
        const pool_count = try readU32(bytes, 4 * WORD);
        const root = try readU32(bytes, 6 * WORD);
        // Sections are laid out header, nodes, pool, blob.
        const pool_base = HEADER_WORDS * WORD + @as(usize, node_count) * NODE_WORDS * WORD;
        return .{
            .bytes = bytes,
            .node_count = node_count,
            .pool_base = pool_base,
            .pool_count = pool_count,
            .root = root,
        };
    }

    /// The byte offset of node `idx`'s record.
    fn nodeAt(self: *const Image, idx: u32) EvalError!usize {
        if (idx >= self.node_count) return EvalError.Corrupt;
        return HEADER_WORDS * WORD + @as(usize, idx) * NODE_WORDS * WORD;
    }

    /// Payload word `slot` of node `idx` (slot 0 is the tag).
    fn word(self: *const Image, idx: u32, slot: usize) EvalError!u32 {
        return readU32(self.bytes, (try self.nodeAt(idx)) + slot * WORD);
    }

    /// The node index at pool position `at`.
    fn pooled(self: *const Image, at: u32) EvalError!u32 {
        if (at >= self.pool_count) return EvalError.Corrupt;
        return readU32(self.bytes, self.pool_base + @as(usize, at) * WORD);
    }
};

/// Read a little-endian u32 at byte offset `at`.
fn readU32(bytes: []const u8, at: usize) EvalError!u32 {
    if (at + WORD > bytes.len) return EvalError.Corrupt;
    return @as(u32, bytes[at]) |
        (@as(u32, bytes[at + 1]) << 8) |
        (@as(u32, bytes[at + 2]) << 16) |
        (@as(u32, bytes[at + 3]) << 24);
}

/// Evaluate node `idx` under the environment chain `cur`. Structural recursion
/// over the finite arena.
fn eval(img: *const Image, idx: u32, env: *Env, cur: u32) EvalError!Value {
    switch (try img.word(idx, 0)) {
        TAG_LIT => return switch (try img.word(idx, 1)) {
            LIT_UNIT => Value.unit,
            LIT_BOOL => Value{ .boolean = (try img.word(idx, 2)) != 0 },
            LIT_INT => blk: {
                const lo = try img.word(idx, 2);
                const hi = try img.word(idx, 3);
                break :blk Value{ .int = @bitCast((@as(u64, hi) << 32) | @as(u64, lo)) };
            },
            else => EvalError.Unsupported, // Str value: awaits the value arena
        },
        TAG_VAR => return env.lookup(cur, try img.word(idx, 1)),
        TAG_LET => {
            const rec = (try img.word(idx, 1)) != 0;
            const name = try img.word(idx, 2);
            const value_ref = try img.word(idx, 3);
            const body_ref = try img.word(idx, 4);
            if (rec) {
                // The binding is in scope for its own value, so a closure the
                // value produces captures a chain that already names it. The
                // slot holds unit until the value is known, then takes it.
                const slot = try env.push(name, Value.unit, cur);
                env.slots[slot].val = try eval(img, value_ref, env, slot);
                return eval(img, body_ref, env, slot);
            }
            const v = try eval(img, value_ref, env, cur);
            return eval(img, body_ref, env, try env.push(name, v, cur));
        },
        TAG_LAMBDA => return Value{ .closure = .{
            .param = try img.word(idx, 1),
            .body = try img.word(idx, 2),
            .env = cur,
        } },
        TAG_APPLY => {
            const callee_ref = try img.word(idx, 1);
            const args_start = try img.word(idx, 2);
            const args_len = try img.word(idx, 3);
            var f = try eval(img, callee_ref, env, cur);
            // Multi-parameter lambdas desugar to nested `Lambda`, so a
            // multi-argument `Apply` is applied one argument at a time.
            var k: u32 = 0;
            while (k < args_len) : (k += 1) {
                const arg = try eval(img, try img.pooled(args_start + k), env, cur);
                const c = switch (f) {
                    .closure => |c| c,
                    else => return EvalError.NotCallable,
                };
                f = try eval(img, c.body, env, try env.push(c.param, arg, c.env));
            }
            return f;
        },
        TAG_IF => {
            const c = try eval(img, try img.word(idx, 1), env, cur);
            const take = switch (c) {
                .boolean => |b| b,
                else => return EvalError.Unsupported,
            };
            return eval(img, try img.word(idx, if (take) 2 else 3), env, cur);
        },
        // Project, Match, Iter, Interp, Raw, Handle land with their forms.
        // Arithmetic is not a Core form: it arrives as a family operation
        // through `Raw`, dispatched to a host-provided handler (the runtime
        // half of the handler discipline), which is the next gate.
        else => return EvalError.Unsupported,
    }
}

/// Evaluate a serialized residual image from its header's root, with a
/// caller-lent environment arena.
pub fn evalImage(bytes: []const u8, slots: []Binding) EvalError!Value {
    const img = try Image.parse(bytes);
    var env = Env{ .slots = slots };
    return eval(&img, img.root, &env, ENV_NIL);
}

export fn vehje_runtime_new() ?*anyopaque {
    return null;
}

export fn vehje_runtime_free(rt: ?*anyopaque) void {
    _ = rt;
}

export fn vehje_runtime_execute(
    rt: ?*anyopaque,
    input: [*]const u8,
    len: usize,
) i32 {
    _ = rt;
    var slots: [1024]Binding = undefined;
    // FIXME: the produced value is discarded; returning it across the C ABI
    // into a host value arena is the next gate. For now a successful evaluation
    // returns OK, an evaluation error returns ERR.
    _ = evalImage(input[0..len], slots[0..]) catch return VEHJE_RESULT_ERR;
    return VEHJE_RESULT_OK;
}

// ── tests ─────────────────────────────────────────────────────────────────

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

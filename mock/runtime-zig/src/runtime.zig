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
const value_image = @import("value_image.zig");

pub const VEHJE_RESULT_OK: i32 = 0;
pub const VEHJE_RESULT_ERR: i32 = -1;
pub const VEHJE_RESULT_NULL_HANDLE: i32 = -2;
pub const VEHJE_RESULT_INVALID_INPUT: i32 = -3;

// Wire format constants, mirroring `vehje-runtime-abi/src/wire/serialize.rs`.
pub const WORD: usize = 4;
pub const HEADER_WORDS: usize = 7;
pub const NODE_WORDS: usize = 7;
pub const MAGIC: u32 = 0x3048_4556; // "VEH0" little-endian

// Node tag codes.
pub const TAG_LIT: u32 = 0;
pub const TAG_VAR: u32 = 1;
pub const TAG_LET: u32 = 2;
pub const TAG_LAMBDA: u32 = 3;
pub const TAG_APPLY: u32 = 4;
pub const TAG_PROJECT: u32 = 5;
pub const TAG_IF: u32 = 6;
pub const TAG_MATCH: u32 = 7;
pub const TAG_ITER: u32 = 8;
pub const TAG_INTERP: u32 = 9;
pub const TAG_RAW: u32 = 10;
pub const TAG_HANDLE: u32 = 11;

// Literal sub-tag codes.
pub const LIT_UNIT: u32 = 0;
pub const LIT_BOOL: u32 = 1;
pub const LIT_INT: u32 = 2;
pub const LIT_STR: u32 = 3;

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
pub const Binding = struct { sym: u32, val: Value, parent: u32 };

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

/// The host's reserve-then-commit transfer sink, mirroring
/// `vehje-runtime-abi`'s `#[repr(C)] VehjeSink`.
///
/// `reserve` returns backing for the next chunk or null (the backpressure
/// signal); `commit` advances the sink by the bytes actually written. The
/// runtime never retains the sink past the call.
pub const VehjeSink = extern struct {
    reserve: *const fn (userdata: ?*anyopaque, hint: usize) callconv(.c) ?[*]u8,
    commit: *const fn (userdata: ?*anyopaque, written: usize) callconv(.c) void,
    userdata: ?*anyopaque,
};

/// Serialise a produced value into `out`, returning the bytes written.
///
/// A closure is refused: it names an environment inside this run, so it has no
/// value-arena representation and cannot be a value the host still holds after
/// the call returns.
fn marshal(v: Value, out: []u8) !usize {
    return switch (v) {
        .unit => value_image.writeScalar(out, .unit, &.{}),
        .boolean => |b| value_image.writeScalar(out, .boolean, &.{@intFromBool(b)}),
        .int => |n| blk: {
            const payload = value_image.intPayload(n);
            break :blk value_image.writeScalar(out, .int, payload[0..]);
        },
        .closure => error.NotRepresentable,
    };
}

pub export fn vehje_runtime_execute(
    rt: ?*anyopaque,
    input: [*]const u8,
    len: usize,
    sink: ?*const VehjeSink,
) i32 {
    _ = rt;
    var slots: [1024]Binding = undefined;
    const v = evalImage(input[0..len], slots[0..]) catch return VEHJE_RESULT_ERR;

    // With no sink the host wanted only the outcome, so evaluating was the
    // whole job. With one, the value crosses back as a value image.
    const s = sink orelse return VEHJE_RESULT_OK;

    var buf: [64]u8 = undefined;
    const n = marshal(v, buf[0..]) catch return VEHJE_RESULT_ERR;
    const dst = s.reserve(s.userdata, n) orelse return VEHJE_RESULT_ERR;
    @memcpy(dst[0..n], buf[0..n]);
    s.commit(s.userdata, n);
    return VEHJE_RESULT_OK;
}

test {
    // Keep the split-out suite discoverable from this root.
    _ = @import("runtime_test.zig");
}

//! Zig runtime: the Tier::Arena tree-walk interpreter.
//!
//! Decodes the flat serialized IR arena that `vehje-runtime-abi` emits (a
//! seven-word header, then fixed seven-word node records, little-endian) and
//! evaluates it directly, the reference semantics of the `Arena` tier. The
//! `Bytecode` tier (a CFG-of-blocks linear form) is a later optimisation.
//!
//! Binders are matched by their `Sym` bits (the wire's stable binder identity,
//! one word per binder), not by resolved text, so a `Var` binds to its `Let`
//! by integer equality.
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

/// A runtime value. The first-landing subset: unit, bool, int. Strings,
/// closures (from `Lambda`), and family values land with their forms.
pub const Value = union(enum) {
    unit,
    boolean: bool,
    int: i64,
};

pub const EvalError = error{
    Unbound, // a Var whose binder is not in scope
    Unsupported, // a form or literal kind the first landing does not evaluate
    EnvFull, // the fixed environment stack overflowed (no alloc)
    Corrupt, // a node index or record past the arena
};

/// One environment binding: a binder's Sym bits mapped to its value.
const Binding = struct { sym: u32, val: Value };

/// Read a little-endian u32 at byte offset `at`.
fn readU32(bytes: []const u8, at: usize) EvalError!u32 {
    if (at + WORD > bytes.len) return EvalError.Corrupt;
    return @as(u32, bytes[at]) |
        (@as(u32, bytes[at + 1]) << 8) |
        (@as(u32, bytes[at + 2]) << 16) |
        (@as(u32, bytes[at + 3]) << 24);
}

/// The byte offset of node `idx`'s record.
fn nodeBase(idx: u32) usize {
    return HEADER_WORDS * WORD + @as(usize, idx) * NODE_WORDS * WORD;
}

/// Evaluate the node at `idx` under `env` (a caller-lent binding stack with
/// `env_len` live entries). Structural recursion over the finite arena.
fn eval(bytes: []const u8, idx: u32, env: []Binding, env_len: *usize) EvalError!Value {
    const base = nodeBase(idx);
    const tag = try readU32(bytes, base);
    switch (tag) {
        TAG_LIT => {
            const kind = try readU32(bytes, base + WORD);
            return switch (kind) {
                LIT_UNIT => Value.unit,
                LIT_BOOL => Value{ .boolean = (try readU32(bytes, base + 2 * WORD)) != 0 },
                LIT_INT => blk: {
                    const lo = try readU32(bytes, base + 2 * WORD);
                    const hi = try readU32(bytes, base + 3 * WORD);
                    const bits: u64 = (@as(u64, hi) << 32) | @as(u64, lo);
                    break :blk Value{ .int = @bitCast(bits) };
                },
                else => EvalError.Unsupported, // Str value: awaits the value arena
            };
        },
        TAG_VAR => {
            const sym = try readU32(bytes, base + WORD);
            var i = env_len.*;
            while (i > 0) {
                i -= 1;
                if (env[i].sym == sym) return env[i].val;
            }
            return EvalError.Unbound;
        },
        TAG_LET => {
            const name = try readU32(bytes, base + 2 * WORD);
            const value_ref = try readU32(bytes, base + 3 * WORD);
            const body_ref = try readU32(bytes, base + 4 * WORD);
            const v = try eval(bytes, value_ref, env, env_len);
            if (env_len.* >= env.len) return EvalError.EnvFull;
            env[env_len.*] = .{ .sym = name, .val = v };
            env_len.* += 1;
            const r = try eval(bytes, body_ref, env, env_len);
            env_len.* -= 1; // pop the binding leaving this scope
            return r;
        },
        TAG_IF => {
            const cond_ref = try readU32(bytes, base + WORD);
            const then_ref = try readU32(bytes, base + 2 * WORD);
            const else_ref = try readU32(bytes, base + 3 * WORD);
            const c = try eval(bytes, cond_ref, env, env_len);
            const take = switch (c) {
                .boolean => |b| b,
                else => return EvalError.Unsupported,
            };
            return eval(bytes, if (take) then_ref else else_ref, env, env_len);
        },
        // Lambda/Apply (closures), Project, Match, Iter, Interp, Raw, Handle
        // land with their forms; the first landing evaluates the pure
        // computation subset.
        else => return EvalError.Unsupported,
    }
}

/// Evaluate a serialized residual image from its header's root, with a
/// caller-lent environment stack.
pub fn evalImage(bytes: []const u8, env: []Binding) EvalError!Value {
    if (bytes.len < HEADER_WORDS * WORD) return EvalError.Corrupt;
    if ((try readU32(bytes, 0)) != MAGIC) return EvalError.Corrupt;
    const root = try readU32(bytes, 6 * WORD);
    var env_len: usize = 0;
    return eval(bytes, root, env, &env_len);
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
    var env: [256]Binding = undefined;
    const bytes = input[0..len];
    // FIXME: the produced value is discarded; returning it across the C ABI
    // into a host value arena is the next gate. For now a successful evaluation
    // returns OK, an evaluation error returns ERR.
    _ = evalImage(bytes, env[0..]) catch return VEHJE_RESULT_ERR;
    return VEHJE_RESULT_OK;
}

// ── tests ─────────────────────────────────────────────────────────────────

fn putU32(buf: []u8, at: usize, w: u32) void {
    buf[at] = @truncate(w & 0xff);
    buf[at + 1] = @truncate((w >> 8) & 0xff);
    buf[at + 2] = @truncate((w >> 16) & 0xff);
    buf[at + 3] = @truncate((w >> 24) & 0xff);
}

test "tree-walk eval of let and if" {
    // let x = 1 in if true then x else 2   =>   1
    // nodes: 0 = Lit Int 1, 1 = Var x, 2 = Lit Bool true, 3 = Lit Int 2,
    //        4 = If(cond=2, then=1, else=3), 5 = Let(x, value=0, body=4).
    const node_count: u32 = 6;
    const x_bits: u32 = 0xABCD_1234;
    var buf: [HEADER_WORDS * WORD + 6 * NODE_WORDS * WORD]u8 = undefined;
    @memset(buf[0..], 0);

    // header
    putU32(buf[0..], 0 * WORD, MAGIC);
    putU32(buf[0..], 1 * WORD, 1); // version
    putU32(buf[0..], 2 * WORD, 0); // tier = Arena
    putU32(buf[0..], 3 * WORD, node_count);
    putU32(buf[0..], 4 * WORD, 0); // pool_count
    putU32(buf[0..], 5 * WORD, 0); // blob_len
    putU32(buf[0..], 6 * WORD, 5); // root = node 5

    // node 0: Lit Int 1
    var b = nodeBase(0);
    putU32(buf[0..], b, TAG_LIT);
    putU32(buf[0..], b + WORD, LIT_INT);
    putU32(buf[0..], b + 2 * WORD, 1); // lo
    putU32(buf[0..], b + 3 * WORD, 0); // hi

    // node 1: Var x
    b = nodeBase(1);
    putU32(buf[0..], b, TAG_VAR);
    putU32(buf[0..], b + WORD, x_bits);

    // node 2: Lit Bool true
    b = nodeBase(2);
    putU32(buf[0..], b, TAG_LIT);
    putU32(buf[0..], b + WORD, LIT_BOOL);
    putU32(buf[0..], b + 2 * WORD, 1);

    // node 3: Lit Int 2
    b = nodeBase(3);
    putU32(buf[0..], b, TAG_LIT);
    putU32(buf[0..], b + WORD, LIT_INT);
    putU32(buf[0..], b + 2 * WORD, 2);
    putU32(buf[0..], b + 3 * WORD, 0);

    // node 4: If(cond=2, then=1, else=3)
    b = nodeBase(4);
    putU32(buf[0..], b, TAG_IF);
    putU32(buf[0..], b + WORD, 2);
    putU32(buf[0..], b + 2 * WORD, 1);
    putU32(buf[0..], b + 3 * WORD, 3);

    // node 5: Let(name=x, value=0, body=4)
    b = nodeBase(5);
    putU32(buf[0..], b, TAG_LET);
    putU32(buf[0..], b + WORD, 0); // rec = false
    putU32(buf[0..], b + 2 * WORD, x_bits); // name
    putU32(buf[0..], b + 3 * WORD, 0); // value = node 0
    putU32(buf[0..], b + 4 * WORD, 4); // body = node 4

    var env: [16]Binding = undefined;
    const result = try evalImage(buf[0..], env[0..]);
    try std.testing.expectEqual(Value{ .int = 1 }, result);
}

test "false branch and unbound" {
    // if false then 1 else 2  =>  2
    const x_bits: u32 = 7;
    var buf: [HEADER_WORDS * WORD + 4 * NODE_WORDS * WORD]u8 = undefined;
    @memset(buf[0..], 0);
    putU32(buf[0..], 0 * WORD, MAGIC);
    putU32(buf[0..], 3 * WORD, 4); // node_count
    putU32(buf[0..], 6 * WORD, 3); // root = If node

    var b = nodeBase(0); // Lit Bool false
    putU32(buf[0..], b, TAG_LIT);
    putU32(buf[0..], b + WORD, LIT_BOOL);
    putU32(buf[0..], b + 2 * WORD, 0);
    b = nodeBase(1); // Lit Int 1
    putU32(buf[0..], b, TAG_LIT);
    putU32(buf[0..], b + WORD, LIT_INT);
    putU32(buf[0..], b + 2 * WORD, 1);
    b = nodeBase(2); // Lit Int 2
    putU32(buf[0..], b, TAG_LIT);
    putU32(buf[0..], b + WORD, LIT_INT);
    putU32(buf[0..], b + 2 * WORD, 2);
    b = nodeBase(3); // If(cond=0, then=1, else=2)
    putU32(buf[0..], b, TAG_IF);
    putU32(buf[0..], b + WORD, 0);
    putU32(buf[0..], b + 2 * WORD, 1);
    putU32(buf[0..], b + 3 * WORD, 2);
    _ = x_bits;

    var env: [16]Binding = undefined;
    const result = try evalImage(buf[0..], env[0..]);
    try std.testing.expectEqual(Value{ .int = 2 }, result);

    // a bare Var with an empty env is unbound.
    var buf2: [HEADER_WORDS * WORD + NODE_WORDS * WORD]u8 = undefined;
    @memset(buf2[0..], 0);
    putU32(buf2[0..], 0 * WORD, MAGIC);
    putU32(buf2[0..], 3 * WORD, 1);
    putU32(buf2[0..], 6 * WORD, 0);
    const vb = nodeBase(0);
    putU32(buf2[0..], vb, TAG_VAR);
    putU32(buf2[0..], vb + WORD, 42);
    var env2: [16]Binding = undefined;
    try std.testing.expectError(EvalError.Unbound, evalImage(buf2[0..], env2[0..]));
}

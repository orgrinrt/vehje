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
    unit,
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
    /// Per node, the impl binder a trait-method reference resolved to, or NONE.
    /// Written by the checker, read here. This is the whole of dispatch.
    resolved: []const u32 = &.{},

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

/// Try one pattern against one value. Returns the environment the arm's body
/// runs under, or null if the pattern did not match. Bindings made before a
/// later sub-pattern fails are left on the arena, which is fine: nothing pops,
/// and the failed arm's scope is simply never used.
fn bindPattern(img: *const Image, pat: u32, v: Value, env: *Env, cur: u32) Error!?u32 {
    switch (try img.word(pat, 0)) {
        cl.PAT_WILD => return cur,
        cl.PAT_BIND => return try env.push(try img.word(pat, 1), v, cur),
        cl.PAT_LIT_INT => {
            const lo = try img.word(pat, 1);
            const hi = try img.word(pat, 2);
            const want: i64 = @bitCast((@as(u64, hi) << 32) | @as(u64, lo));
            return switch (v) {
                .int => |n| if (n == want) cur else null,
                else => Error.Unsupported,
            };
        },
        cl.PAT_LIT_STR => {
            const want = try img.blob(try img.word(pat, 1), try img.word(pat, 2));
            return switch (v) {
                .str => |t| if (std.mem.eql(u8, t, want)) cur else null,
                else => Error.Unsupported,
            };
        },
        cl.PAT_OR => {
            if (try bindPattern(img, try img.word(pat, 1), v, env, cur)) |sc| return sc;
            return bindPattern(img, try img.word(pat, 2), v, env, cur);
        },
        cl.PAT_RANGE => {
            const lo: i64 = @bitCast((@as(u64, try img.word(pat, 2)) << 32) | @as(u64, try img.word(pat, 1)));
            const hi: i64 = @bitCast((@as(u64, try img.word(pat, 4)) << 32) | @as(u64, try img.word(pat, 3)));
            const inclusive = (try img.word(pat, 5)) != 0;
            return switch (v) {
                .int => |n| if (n >= lo and (if (inclusive) n <= hi else n < hi)) cur else null,
                else => Error.Unsupported,
            };
        },
        cl.PAT_REC => {
            const start = try img.word(pat, 1);
            const n = try img.word(pat, 2);
            const rec = switch (v) {
                .record => |r| r,
                else => return Error.NotARecord,
            };
            var scope = cur;
            var k: u32 = 0;
            while (k < n) : (k += 2) {
                const key_node = try img.pooled(start + k);
                const key = try img.blob(try img.word(key_node, 2), try img.word(key_node, 3));
                var found: ?Value = null;
                var j: u32 = 0;
                while (j < rec.len) : (j += 1) {
                    const e = env.recs[rec.start + j];
                    if (std.mem.eql(u8, e.key, key)) {
                        found = e.val;
                        break;
                    }
                }
                const fv = found orelse return Error.NoSuchField;
                const sub = try bindPattern(img, try img.pooled(start + k + 1), fv, env, scope);
                scope = sub orelse return null;
            }
            return scope;
        },
        else => return Error.Unsupported,
    }
}

fn evalNode(img: *const Image, idx: u32, env: *Env, cur: u32) Error!Value {
    switch (try img.word(idx, 0)) {
        cl.TAG_LIT => switch (try img.word(idx, 1)) {
            cl.LIT_INT => {
                const lo = try img.word(idx, 2);
                const hi = try img.word(idx, 3);
                return Value{ .int = @bitCast((@as(u64, hi) << 32) | @as(u64, lo)) };
            },
            cl.LIT_UNIT => return Value.unit,
            cl.LIT_STR => return Value{ .str = try img.blob(try img.word(idx, 2), try img.word(idx, 3)) },
            else => return Error.Unsupported,
        },
        cl.TAG_VAR => {
            // Dispatch: if the checker resolved this reference to an impl, the
            // impl's binder is what to look up. The value's shape is never
            // inspected, so types still erase.
            const sym = if (env.resolved.len > idx and env.resolved[idx] != chk.NONE)
                env.resolved[idx]
            else
                try img.word(idx, 1);
            return env.lookup(sym, cur);
        },
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
        cl.TAG_MATCH => {
            const scrut = try evalNode(img, try img.word(idx, 1), env, cur);
            const start = try img.word(idx, 2);
            const arms = try img.word(idx, 3);
            var a: u32 = 0;
            while (a < arms) : (a += 1) {
                const pat = try img.pooled(start + a * 3);
                const guard = try img.pooled(start + a * 3 + 1);
                const body = try img.pooled(start + a * 3 + 2);
                // Bindings a pattern introduces are pushed onto the environment
                // chain, so the arm's body reads them like any other binding.
                const scope = try bindPattern(img, pat, scrut, env, cur);
                if (scope) |sc| {
                    // The guard runs under the arm's bindings, so it may test
                    // what the pattern just bound.
                    if (guard != cl.NO_GUARD) {
                        if ((try evalNode(img, guard, env, sc)).asInt() catch 0 == 0) continue;
                    }
                    return evalNode(img, body, env, sc);
                }
            }
            // The parser requires an irrefutable last arm, so this is a decode
            // fault rather than a program the checker let through.
            return Error.Corrupt;
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

    var traits: [32]cl.TraitDecl = undefined;
    var impls: [64]cl.ImplDecl = undefined;
    var resolved: [8192]u32 = undefined;

    var b = cl.Builder{ .nodes = &node_buf, .pool = &pool_buf, .blob = &blob_buf };
    var names = cl.Names{ .buf = &name_buf };
    var p = try cl.Parser.init(src, &b, &names, &traits, &impls);
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
    var pending: [1024]chk.Constraint = undefined;
    @memset(resolved[0..b.n], chk.NONE);
    var ctx = chk.Ctx{
        .types = &types,
        .subst = &subst,
        .env = &tenv,
        .fields = &tfields,
        .traits = traits[0..p.ntraits],
        .impls = impls[0..p.nimpls],
        .resolved = resolved[0..b.n],
        .pending = &pending,
    };
    _ = try chk.check(image[0..len], &ctx);

    const img = try Image.parse(image[0..len]);
    var env = Env{ .slots = &slots, .recs = &recs, .vals = &vals, .resolved = resolved[0..b.n] };
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

    var traits: [32]cl.TraitDecl = undefined;
    var impls: [64]cl.ImplDecl = undefined;
    var resolved: [8192]u32 = undefined;

    var b = cl.Builder{ .nodes = &node_buf, .pool = &pool_buf, .blob = &blob_buf };
    var names = cl.Names{ .buf = &name_buf };
    var p = try cl.Parser.init(src, &b, &names, &traits, &impls);
    const root = try p.program();
    const len = try cl.writeImage(&b, root, &image);

    var types: [32768]chk.Ty = undefined;
    var subst: [8192]u32 = undefined;
    var tenv: [4096]chk.TyBinding = undefined;
    var tfields: [4096]chk.Field = undefined;
    var pending: [1024]chk.Constraint = undefined;
    @memset(resolved[0..b.n], chk.NONE);
    var ctx = chk.Ctx{
        .types = &types,
        .subst = &subst,
        .env = &tenv,
        .fields = &tfields,
        .traits = traits[0..p.ntraits],
        .impls = impls[0..p.nimpls],
        .resolved = resolved[0..b.n],
        .pending = &pending,
    };
    _ = try chk.check(image[0..len], &ctx);

    const img = try Image.parse(image[0..len]);
    var env = Env{ .slots = &slots, .recs = &recs, .vals = &vals, .resolved = resolved[0..b.n] };
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

test "a trait dispatches to its impl" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\trait Doubler { fn dbl(Self) -> Self }
        \\impl Doubler for Int { fn dbl(x) { x * 2 } }
        \\dbl(5)
    ));
}

test "one trait, several impls, chosen by the type at the call site" {
    const src =
        \\trait Sizer { fn size(Self) -> Int }
        \\impl Sizer for Int { fn size(x) { x + 100 } }
        \\impl Sizer for Str { fn size(s) { 7 } }
    ;
    var buf: [4096]u8 = undefined;
    @memcpy(buf[0..src.len], src);
    const tail1 = "\nsize(5)";
    @memcpy(buf[src.len..][0..tail1.len], tail1);
    try std.testing.expectEqual(@as(i64, 105), try run(buf[0 .. src.len + tail1.len]));
    const tail2 = "\nsize(\"anything\")";
    @memcpy(buf[src.len..][0..tail2.len], tail2);
    try std.testing.expectEqual(@as(i64, 7), try run(buf[0 .. src.len + tail2.len]));
}

test "a function with a trait bound resolves through to the impl" {
    try std.testing.expectEqual(@as(i64, 20), try run(
        \\trait Doubler { fn dbl(Self) -> Self }
        \\impl Doubler for Int { fn dbl(x) { x * 2 } }
        \\fn twice(v) { dbl(dbl(v)) }
        \\twice(5)
    ));
}

test "a trait method with no impl for the type is refused" {
    try std.testing.expectError(chk.Error.NoImpl, run(
        \\trait Doubler { fn dbl(Self) -> Self }
        \\impl Doubler for Int { fn dbl(x) { x * 2 } }
        \\fn use_it(v) { dbl(v) }
        \\use_it("a string")
    ));
}

test "an impl whose body does not match the trait signature is refused" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\trait Doubler { fn dbl(Self) -> Self }
        \\impl Doubler for Int { fn dbl(x) { 0 < x } }
        \\dbl(5)
    ));
}

test "coherence: a second impl for the same type is refused" {
    try std.testing.expectError(cl.Error.DuplicateImpl, run(
        \\trait Doubler { fn dbl(Self) -> Self }
        \\impl Doubler for Int { fn dbl(x) { x * 2 } }
        \\impl Doubler for Int { fn dbl(x) { x * 3 } }
        \\dbl(5)
    ));
}

test "an impl of an undeclared trait is refused" {
    try std.testing.expectError(cl.Error.UnknownTrait, run(
        \\impl Nope for Int { fn dbl(x) { x } }
        \\1
    ));
}

test "an associated type is chosen by the impl, and the method's result follows" {
    // Int's Out is Str and Str's Out is Int, so conv's result type depends on
    // which impl applies rather than on the call's syntax.
    const decls =
        \\trait Conv { type Out; fn conv(Self) -> Out }
        \\impl Conv for Int { type Out = Str; fn conv(x) { "from int" } }
        \\impl Conv for Str { type Out = Int; fn conv(s) { 42 } }
    ;
    var buf: [4096]u8 = undefined;
    var out: [64]u8 = undefined;
    @memcpy(buf[0..decls.len], decls);

    const t1 = "\nconv(\"a string\")";
    @memcpy(buf[decls.len..][0..t1.len], t1);
    try std.testing.expectEqual(@as(i64, 42), try run(buf[0 .. decls.len + t1.len]));

    const t2 = "\nconv(7)";
    @memcpy(buf[decls.len..][0..t2.len], t2);
    try std.testing.expectEqualStrings("from int", try runStr(buf[0 .. decls.len + t2.len], &out));
}

test "using an associated result at the wrong type is refused" {
    // conv(7) is a Str by Int's impl, so adding one to it is a type error that
    // only the associated type's resolution can catch.
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\trait Conv { type Out; fn conv(Self) -> Out }
        \\impl Conv for Int { type Out = Str; fn conv(x) { "from int" } }
        \\conv(7) + 1
    ));
}

test "an impl whose body disagrees with its own associated type is refused" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\trait Conv { type Out; fn conv(Self) -> Out }
        \\impl Conv for Int { type Out = Str; fn conv(x) { 99 } }
        \\1
    ));
}

test "the associated type flows into an ordinary function" {
    try std.testing.expectEqual(@as(i64, 43), try run(
        \\trait Conv { type Out; fn conv(Self) -> Out }
        \\impl Conv for Str { type Out = Int; fn conv(s) { 42 } }
        \\fn bump(v) { conv(v) + 1 }
        \\bump("x")
    ));
}

test "match on integer literals, with a binding as the catch-all" {
    try std.testing.expectEqual(@as(i64, 100), try run("match 0 { 0 => 100, n => n }"));
    try std.testing.expectEqual(@as(i64, 7), try run("match 7 { 0 => 100, n => n }"));
    try std.testing.expectEqual(@as(i64, 42), try run("match 5 { 0 => 1, 5 => 42, _ => 0 }"));
}

test "match binds what it destructures" {
    try std.testing.expectEqual(@as(i64, 9), try run("match 4 { 0 => 0, n => n + 5 }"));
    try std.testing.expectEqual(@as(i64, 12), try run(
        \\fn classify(n) { match n { 0 => 0, 1 => 1, k => k * 4 } }
        \\classify(3)
    ));
}

test "match on strings" {
    var out: [64]u8 = undefined;
    try std.testing.expectEqualStrings("yes", try runStr("match \"a\" { \"a\" => \"yes\", _ => \"no\" }", &out));
    try std.testing.expectEqualStrings("no", try runStr("match \"b\" { \"a\" => \"yes\", _ => \"no\" }", &out));
}

test "match destructures a record and binds its fields" {
    // A bare record literal cannot be a scrutinee, because `match r {` cannot
    // tell the record from the arm block. Rust has the same restriction and the
    // same two ways out: bind it first, or parenthesise it.
    try std.testing.expectEqual(@as(i64, 8), try run(
        \\let r = { a: 3, b: 5 };
        \\match r { { a: x, b: y } => x + y }
    ));
    try std.testing.expectEqual(@as(i64, 8), try run("match ({ a: 3, b: 5 }) { { a: x, b: y } => x + y }"));
    try std.testing.expectEqual(@as(i64, 1), try run(
        \\let r = { a: 0, b: 5 };
        \\match r { { a: 0, b: _ } => 1, _ => 2 }
    ));
    try std.testing.expectEqual(@as(i64, 2), try run(
        \\let r = { a: 9, b: 5 };
        \\match r { { a: 0, b: _ } => 1, _ => 2 }
    ));
}

test "match arms must agree on a result type" {
    try std.testing.expectError(chk.Error.Mismatch, run("match 1 { 0 => 1, _ => \"other\" }"));
}

test "a pattern of the wrong shape is refused" {
    try std.testing.expectError(chk.Error.Mismatch, run("match 1 { \"a\" => 1, _ => 2 }"));
    // A refutable record pattern, so the arm after it is reachable and the
    // only fault left is the type error this test is about.
    try std.testing.expectError(chk.Error.Mismatch, run("match 1 { { a: 0 } => 1, _ => 2 }"));
}

test "a match without an irrefutable last arm is refused" {
    try std.testing.expectError(cl.Error.NonExhaustive, run("match 1 { 0 => 1, 2 => 3 }"));
}

test "match composes with the standard library" {
    try std.testing.expectEqual(@as(i64, 3), try runWithStd(
        \\fn size_class(s) { match len(s) { 0 => 0, 1 => 1, _ => 3 } }
        \\size_class([1, 2, 3, 4])
    ));
}

test "a while loop accumulates through rebinding" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\let mut acc = 0;
        \\let mut i = 0;
        \\while i < 5 {
        \\  acc += i;
        \\  i += 1;
        \\}
        \\acc
    ));
}

test "a while loop walks a sequence" {
    try std.testing.expectEqual(@as(i64, 10), try runWithStd(
        \\let s = [1, 2, 3, 4];
        \\let mut total = 0;
        \\let mut i = 0;
        \\while i < len(s) {
        \\  total += at(s, i);
        \\  i += 1;
        \\}
        \\total
    ));
}

test "a loop that never runs leaves its state alone" {
    try std.testing.expectEqual(@as(i64, 99), try run(
        \\let mut x = 99;
        \\let mut i = 5;
        \\while i < 5 {
        \\  x += 1;
        \\  i += 1;
        \\}
        \\x
    ));
}

test "a loop body may bind its own locals" {
    // i runs 0..3, so step is 2,3,4,5 and the total is 14.
    try std.testing.expectEqual(@as(i64, 14), try run(
        \\let mut acc = 0;
        \\let mut i = 0;
        \\while i < 4 {
        \\  let step = i + 2;
        \\  acc += step;
        \\  i += 1;
        \\}
        \\acc
    ));
}

test "minus-assign counts down" {
    try std.testing.expectEqual(@as(i64, 6), try run(
        \\let mut n = 3;
        \\let mut acc = 0;
        \\while 0 < n {
        \\  acc += n;
        \\  n -= 1;
        \\}
        \\acc
    ));
}

test "assigning a non-mutable binding is refused" {
    try std.testing.expectError(cl.Error.NotMutable, run(
        \\let x = 1;
        \\let mut i = 0;
        \\while i < 2 {
        \\  x = 5;
        \\  i += 1;
        \\}
        \\x
    ));
}

test "the loop's state keeps its type" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\let mut acc = 0;
        \\let mut i = 0;
        \\while i < 2 {
        \\  acc = "text";
        \\  i += 1;
        \\}
        \\acc
    ));
}

test "a module groups items and a path reaches them" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\mod M { pub fn double(n) { n * 2 } }
        \\M::double(5)
    ));
    try std.testing.expectEqual(@as(i64, 11), try run(
        \\mod M {
        \\  pub fn double(n) { n * 2 }
        \\  pub fn bump(n) { n + 1 }
        \\}
        \\M::bump(M::double(5))
    ));
}

test "items inside a module see each other" {
    try std.testing.expectEqual(@as(i64, 21), try run(
        \\mod M {
        \\  fn double(n) { n * 2 }
        \\  pub fn quad_plus(n) { double(double(n)) + 1 }
        \\}
        \\M::quad_plus(5)
    ));
}

test "a module's function may recurse" {
    try std.testing.expectEqual(@as(i64, 120), try run(
        \\mod M { pub fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } } }
        \\M::fact(5)
    ));
}

test "use brings one item into scope" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\mod M { pub fn double(n) { n * 2 } }
        \\use M::double;
        \\double(5)
    ));
}

test "a module is a value, so its items can be taken out and used" {
    try std.testing.expectEqual(@as(i64, 8), try run(
        \\mod M { pub fn double(n) { n * 2 } }
        \\let d = M.double;
        \\d(4)
    ));
}

test "projecting off a parameter is refused, for want of row polymorphism" {
    // Passing a module to a function and projecting inside it would need the
    // parameter's type to be "some record with a double field", which is a row
    // type. Refusing is honest; inferring a concrete record here would be wrong.
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\mod M { pub fn double(n) { n * 2 } }
        \\fn apply_double(m, v) { m.double(v) }
        \\apply_double(M, 4)
    ));
}

test "reaching a name a module does not have is refused" {
    try std.testing.expectError(chk.Error.NoSuchField, run(
        \\mod M { pub fn double(n) { n * 2 } }
        \\M::missing(5)
    ));
}

test "a module's items keep their types across the path" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\mod M { pub fn double(n) { n * 2 } }
        \\M::double("not a number")
    ));
}

test "a trait may declare several methods" {
    try std.testing.expectEqual(@as(i64, 27), try run(
        \\trait Num { fn dbl(Self) -> Self  fn trip(Self) -> Self }
        \\impl Num for Int { fn dbl(x) { x * 2 } fn trip(x) { x * 3 } }
        \\dbl(6) + trip(5)
    ));
}

test "an impl may write its methods in any order" {
    try std.testing.expectEqual(@as(i64, 27), try run(
        \\trait Num { fn dbl(Self) -> Self  fn trip(Self) -> Self }
        \\impl Num for Int { fn trip(x) { x * 3 } fn dbl(x) { x * 2 } }
        \\dbl(6) + trip(5)
    ));
}

test "methods of a multi-method trait dispatch independently by type" {
    const decls =
        \\trait Show { fn tag(Self) -> Int  fn width(Self) -> Int }
        \\impl Show for Int { fn tag(x) { 1 } fn width(x) { x } }
        \\impl Show for Str { fn tag(s) { 2 } fn width(s) { 99 } }
    ;
    var buf: [4096]u8 = undefined;
    @memcpy(buf[0..decls.len], decls);
    const t1 = "\ntag(5) + width(7)";
    @memcpy(buf[decls.len..][0..t1.len], t1);
    try std.testing.expectEqual(@as(i64, 8), try run(buf[0 .. decls.len + t1.len]));
    const t2 = "\ntag(\"s\") + width(\"s\")";
    @memcpy(buf[decls.len..][0..t2.len], t2);
    try std.testing.expectEqual(@as(i64, 101), try run(buf[0 .. decls.len + t2.len]));
}

test "an impl missing one of the trait's methods is refused" {
    try std.testing.expectError(cl.Error.MissingMethod, run(
        \\trait Num { fn dbl(Self) -> Self  fn trip(Self) -> Self }
        \\impl Num for Int { fn dbl(x) { x * 2 } }
        \\dbl(6)
    ));
}

test "an impl of a method the trait never declared is refused" {
    try std.testing.expectError(cl.Error.UnknownTrait, run(
        \\trait Num { fn dbl(Self) -> Self }
        \\impl Num for Int { fn nope(x) { x } }
        \\1
    ));
}

test "each method's signature is checked separately" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\trait Num { fn dbl(Self) -> Self  fn name(Self) -> Str }
        \\impl Num for Int { fn dbl(x) { x * 2 } fn name(x) { x } }
        \\dbl(1)
    ));
}

test "or-patterns match any alternative" {
    try std.testing.expectEqual(@as(i64, 1), try run("match 2 { 1 | 2 | 3 => 1, _ => 0 }"));
    try std.testing.expectEqual(@as(i64, 0), try run("match 9 { 1 | 2 | 3 => 1, _ => 0 }"));
    var out: [64]u8 = undefined;
    try std.testing.expectEqualStrings("vowel", try runStr(
        \\match "e" { "a" | "e" | "i" => "vowel", _ => "other" }
    , &out));
}

test "range patterns, exclusive and inclusive" {
    try std.testing.expectEqual(@as(i64, 1), try run("match 3 { 0..5 => 1, _ => 0 }"));
    try std.testing.expectEqual(@as(i64, 0), try run("match 5 { 0..5 => 1, _ => 0 }"));
    try std.testing.expectEqual(@as(i64, 1), try run("match 5 { 0..=5 => 1, _ => 0 }"));
    try std.testing.expectEqual(@as(i64, 1), try run("match -3 { -5..0 => 1, _ => 0 }"));
}

test "guards test what the pattern bound" {
    try std.testing.expectEqual(@as(i64, 1), try run("match 8 { n if 5 < n => 1, _ => 0 }"));
    try std.testing.expectEqual(@as(i64, 0), try run("match 2 { n if 5 < n => 1, _ => 0 }"));
    try std.testing.expectEqual(@as(i64, 20), try run(
        \\let r = { a: 3, b: 5 };
        \\match r { { a: x, b: y } if x < y => x * y + 5, _ => 0 }
    ));
}

test "a guard makes an arm refutable, so it cannot be the last one alone" {
    try std.testing.expectError(cl.Error.NonExhaustive, run("match 1 { n if 0 < n => 1 }"));
}

test "a guard must be a condition" {
    try std.testing.expectError(chk.Error.Mismatch, run("match 1 { n if n => 1, _ => 0 }"));
}

test "alternatives may not bind, because the two sides must agree" {
    try std.testing.expectError(cl.Error.BindingInAlternative, run("match 1 { 1 | n => n, _ => 0 }"));
}

test "alternatives and ranges keep the scrutinee's type" {
    try std.testing.expectError(chk.Error.Mismatch, run("match \"s\" { 1 | 2 => 1, _ => 0 }"));
    try std.testing.expectError(chk.Error.Mismatch, run("match \"s\" { 0..5 => 1, _ => 0 }"));
}

test "guards and ranges compose in a classifier" {
    try std.testing.expectEqual(@as(i64, 3), try runWithStd(
        \\fn classify(n) {
        \\  match n {
        \\    0 => 0,
        \\    1..=9 => 1,
        \\    k if k < 100 => 2,
        \\    _ => 3
        \\  }
        \\}
        \\classify(500)
    ));
    try std.testing.expectEqual(@as(i64, 1), try runWithStd(
        \\fn classify(n) {
        \\  match n { 0 => 0, 1..=9 => 1, k if k < 100 => 2, _ => 3 }
        \\}
        \\classify(5)
    ));
}

test "a for loop walks a sequence" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\let mut total = 0;
        \\for x in [1, 2, 3, 4] { total += x; }
        \\total
    ));
}

test "a for loop over an empty sequence runs no iterations" {
    try std.testing.expectEqual(@as(i64, 7), try run(
        \\let mut total = 7;
        \\for x in [] { total += x; }
        \\total
    ));
}

test "a for loop body may bind and may use the element" {
    try std.testing.expectEqual(@as(i64, 20), try run(
        \\let mut total = 0;
        \\for x in [1, 2, 3, 4] { let doubled = x * 2; total += doubled; }
        \\total
    ));
}

test "for composes with the standard library" {
    // range(3) is [0,1,2]; add2(10) makes it [10,11,12]; the total is 33.
    try std.testing.expectEqual(@as(i64, 33), try runWithStd(
        \\let mut total = 0;
        \\for x in map(add2(10), range(3)) { total += x; }
        \\total
    ));
}

test "the loop index cannot be captured by a source name" {
    // The desugaring's index and sequence come from a synthetic range, so a
    // program using the names i or s is unaffected by them.
    try std.testing.expectEqual(@as(i64, 106), try run(
        \\let i = 100;
        \\let s = 0;
        \\let mut total = 0;
        \\for x in [1, 2, 3] { total += x; }
        \\total + i
    ));
}

test "for keeps the element's type" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\let mut total = 0;
        \\for x in ["a", "b"] { total += x; }
        \\total
    ));
}

test "iterating a non-sequence is refused" {
    try std.testing.expectError(chk.Error.Mismatch, run(
        \\let mut total = 0;
        \\for x in 5 { total += x; }
        \\total
    ));
}

test "a macro is evaluated at the compile stage and leaves a constant" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\macro double(n) -> Int { n * 2 }
        \\double!(5)
    ));
    try std.testing.expectEqual(@as(i64, 26), try run(
        \\macro double(n) -> Int { n * 2 }
        \\double!(3) + double!(10)
    ));
}

test "a macro may branch and take several parameters" {
    try std.testing.expectEqual(@as(i64, 7), try run(
        \\macro pick(a, b) -> Int { if a < b { b } else { a } }
        \\pick!(7, 3)
    ));
    try std.testing.expectEqual(@as(i64, 9), try run(
        \\macro pick(a, b) -> Int { if a < b { b } else { a } }
        \\pick!(2, 9)
    ));
}

test "a macro may take another macro's expansion" {
    try std.testing.expectEqual(@as(i64, 20), try run(
        \\macro double(n) -> Int { n * 2 }
        \\macro quad(n) -> Int { double!(0) + n * 4 }
        \\quad!(5)
    ));
}

test "the expansion is a constant, so it composes with runtime code" {
    try std.testing.expectEqual(@as(i64, 30), try run(
        \\macro ten() -> Int { 10 }
        \\fn triple(n) { n * 3 }
        \\triple(ten!())
    ));
}

test "a macro given a runtime value is refused, because there is no later stage" {
    try std.testing.expectError(cl.Error.NotConstant, run(
        \\macro double(n) -> Int { n * 2 }
        \\fn f(x) { double!(x) }
        \\f(3)
    ));
}

test "an undeclared macro is refused" {
    try std.testing.expectError(cl.Error.UnknownMacro, run("nope!(1)"));
}

test "a macro called with the wrong number of arguments is refused" {
    try std.testing.expectError(cl.Error.WrongMacroArity, run(
        \\macro double(n) -> Int { n * 2 }
        \\double!(1, 2)
    ));
}

test "a macro body that is not constant-evaluable is refused at the call" {
    try std.testing.expectError(cl.Error.NotConstant, run(
        \\macro bad(n) -> Int { unknown_name + n }
        \\bad!(1)
    ));
}

test "a supertrait's impl is required, in either declaration order" {
    try std.testing.expectEqual(@as(i64, 12), try run(
        \\trait Base { fn base(Self) -> Int }
        \\trait Derived: Base { fn derived(Self) -> Int }
        \\impl Base for Int { fn base(x) { x + 1 } }
        \\impl Derived for Int { fn derived(x) { x + 2 } }
        \\base(4) + derived(5)
    ));
    // The supertrait's impl written second is still accepted, because the
    // requirement is checked over the whole table rather than in order.
    try std.testing.expectEqual(@as(i64, 12), try run(
        \\trait Base { fn base(Self) -> Int }
        \\trait Derived: Base { fn derived(Self) -> Int }
        \\impl Derived for Int { fn derived(x) { x + 2 } }
        \\impl Base for Int { fn base(x) { x + 1 } }
        \\base(4) + derived(5)
    ));
}

test "implementing a trait without its supertrait is refused" {
    try std.testing.expectError(chk.Error.MissingSuperImpl, run(
        \\trait Base { fn base(Self) -> Int }
        \\trait Derived: Base { fn derived(Self) -> Int }
        \\impl Derived for Int { fn derived(x) { x + 2 } }
        \\derived(5)
    ));
}

test "the whole supertrait chain is required" {
    try std.testing.expectError(chk.Error.MissingSuperImpl, run(
        \\trait A { fn a(Self) -> Int }
        \\trait B: A { fn b(Self) -> Int }
        \\trait C: B { fn c(Self) -> Int }
        \\impl B for Int { fn b(x) { x } }
        \\impl C for Int { fn c(x) { x } }
        \\c(1)
    ));
    try std.testing.expectEqual(@as(i64, 6), try run(
        \\trait A { fn a(Self) -> Int }
        \\trait B: A { fn b(Self) -> Int }
        \\trait C: B { fn c(Self) -> Int }
        \\impl A for Int { fn a(x) { x } }
        \\impl B for Int { fn b(x) { x * 2 } }
        \\impl C for Int { fn c(x) { x * 3 } }
        \\a(1) + b(1) + c(1)
    ));
}

test "naming an undeclared supertrait is refused" {
    try std.testing.expectError(cl.Error.UnknownTrait, run(
        \\trait Derived: Nope { fn derived(Self) -> Int }
        \\1
    ));
}

test "a private item is reachable inside its module and not outside" {
    // quad_plus calls double, which is private. The call works; reaching double
    // from outside does not, and it fails as a missing field because a private
    // item simply is not one.
    try std.testing.expectEqual(@as(i64, 21), try run(
        \\mod M {
        \\  fn double(n) { n * 2 }
        \\  pub fn quad_plus(n) { double(double(n)) + 1 }
        \\}
        \\M::quad_plus(5)
    ));
    try std.testing.expectError(chk.Error.NoSuchField, run(
        \\mod M {
        \\  fn double(n) { n * 2 }
        \\  pub fn quad_plus(n) { double(double(n)) + 1 }
        \\}
        \\M::double(5)
    ));
}

test "modules nest, and a private inner module stays private" {
    try std.testing.expectEqual(@as(i64, 12), try run(
        \\mod Outer {
        \\  pub mod Inner { pub fn six() { 6 } }
        \\  pub fn twelve() { Inner::six() * 2 }
        \\}
        \\Outer::twelve()
    ));
    try std.testing.expectEqual(@as(i64, 6), try run(
        \\mod Outer { pub mod Inner { pub fn six() { 6 } } }
        \\Outer::Inner::six()
    ));
    try std.testing.expectError(chk.Error.NoSuchField, run(
        \\mod Outer { mod Inner { pub fn six() { 6 } } pub fn twelve() { Inner::six() * 2 } }
        \\Outer::Inner::six()
    ));
}

test "attributes are parsed and erased" {
    try std.testing.expectEqual(@as(i64, 10), try run(
        \\#[inline]
        \\fn double(n) { n * 2 }
        \\double(5)
    ));
    try std.testing.expectEqual(@as(i64, 6), try run(
        \\#[doc]
        \\#[cfg(anything, nested[deeper])]
        \\mod M { #[export] pub fn six() { 6 } }
        \\M::six()
    ));
}

test "unit is a value and a nullary function takes one" {
    try std.testing.expectEqual(@as(i64, 6), try run("fn six() { 6 } six()"));
    try std.testing.expectEqual(@as(i64, 12), try run("fn six() { 6 } six() + six()"));
}

test "unit does not unify with anything else" {
    try std.testing.expectError(chk.Error.Mismatch, run("fn six() { 6 } six() + ()"));
    try std.testing.expectError(chk.Error.Mismatch, run("if 1 < 2 { () } else { 1 }"));
}

test "an arm after an irrefutable one is refused as unreachable" {
    try std.testing.expectError(cl.Error.UnreachableArm, run("match 1 { _ => 1, 2 => 3 }"));
    try std.testing.expectError(cl.Error.UnreachableArm, run("match 1 { n => n, _ => 0 }"));
    try std.testing.expectError(cl.Error.UnreachableArm, run(
        \\let r = { a: 1 };
        \\match r { { a: x } => x, _ => 0 }
    ));
}

test "a guarded arm does not make what follows unreachable" {
    // The guard may fail, so a later arm is genuinely reachable.
    try std.testing.expectEqual(@as(i64, 0), try run("match 1 { n if 5 < n => 1, _ => 0 }"));
}

//! Type inference over the Core image, in the runtime.
//!
//! The canon's centre of gravity is prove-then-erase: the type system is the
//! verification layer and the runtime is a dumb evaluator of proven-safe
//! programs. That only means anything if something actually proves. This is that
//! pass for the subset the parser emits, and it runs before evaluation, so an
//! ill-typed program is refused rather than evaluated into a wrong answer.
//!
//! Hindley-Milner with generalisation at binding sites, which is the seed the
//! bounds and associated types grow from: a generalised binder is already a
//! generic, and a bound is a constraint carried alongside the quantifier rather
//! than a different mechanism.
//!
//! No allocation. Types live in a caller-lent arena and are named by index;
//! substitution is an array indexed by variable id.

const std = @import("std");
const cl = @import("clause.zig");

pub const Error = error{
    Corrupt,
    Unbound,
    Unsupported,
    Mismatch,
    NoSuchField,
    Occurs,
    OutOfTypes,
    TooManyVars,
    EnvFull,
};

pub const NONE: u32 = 0xFFFF_FFFF;

/// A type. `func` names its parameter and result by arena index, so the
/// representation is flat and nothing points at anything it outlives.
pub const Ty = union(enum) {
    int,
    boolean,
    str,
    tvar: u32,
    func: struct { p: u32, r: u32 },
    /// A record type is a span of the field table. Fields are kept in the order
    /// the literal wrote them, and two record types match only if their fields
    /// match pairwise; reordering is not yet a subtyping question this answers.
    record: struct { first: u32, count: u32 },
    /// A sequence is homogeneous: one element type, named by arena index. A
    /// heterogeneous list would need a sum type, which this subset has not got.
    seq: u32,
};

/// One field of a record type: its name, and the type at that name.
pub const Field = struct { key: []const u8, ty: u32 };

/// A binding's type, plus how many leading variables are quantified. A scheme
/// with `quantified == 0` is a plain monotype; anything higher is a generic.
const Scheme = struct { ty: u32, quantified: u32, first: u32 };

pub const TyBinding = struct { sym: u32, scheme: Scheme, parent: u32 };

pub const Ctx = struct {
    types: []Ty,
    n: u32 = 0,
    /// `subst[v]` is the arena index a variable resolves to, or `NONE`.
    subst: []u32,
    nvars: u32 = 0,
    env: []TyBinding,
    nenv: u32 = 0,
    fields: []Field,
    nf: u32 = 0,

    fn alloc(self: *Ctx, t: Ty) Error!u32 {
        if (self.n >= self.types.len) return Error.OutOfTypes;
        self.types[self.n] = t;
        self.n += 1;
        return self.n - 1;
    }

    fn fresh(self: *Ctx) Error!u32 {
        if (self.nvars >= self.subst.len) return Error.TooManyVars;
        const v = self.nvars;
        self.nvars += 1;
        self.subst[v] = NONE;
        return self.alloc(.{ .tvar = v });
    }

    /// Follow substitution links until a variable is unbound or the type is
    /// concrete. Not path-compressing; the chains here are short and the extra
    /// mutation would buy nothing at this size.
    fn resolve(self: *const Ctx, t: u32) u32 {
        var cur = t;
        while (true) {
            switch (self.types[cur]) {
                .tvar => |v| {
                    if (self.subst[v] == NONE) return cur;
                    cur = self.subst[v];
                },
                else => return cur,
            }
        }
    }

    fn occurs(self: *const Ctx, v: u32, t: u32) bool {
        const r = self.resolve(t);
        return switch (self.types[r]) {
            .tvar => |w| w == v,
            .func => |f| self.occurs(v, f.p) or self.occurs(v, f.r),
            .seq => |e| self.occurs(v, e),
            .record => |rec| blk: {
                var i: u32 = 0;
                while (i < rec.count) : (i += 1) {
                    if (self.occurs(v, self.fields[rec.first + i].ty)) break :blk true;
                }
                break :blk false;
            },
            else => false,
        };
    }

    pub fn unify(self: *Ctx, a: u32, b: u32) Error!void {
        const ra = self.resolve(a);
        const rb = self.resolve(b);
        if (ra == rb) return;
        switch (self.types[ra]) {
            .tvar => |v| {
                // The occurs check is what keeps an infinite type from being
                // accepted; without it `fn f(x) { x(x) }` unifies happily and
                // the checker proves nothing.
                if (self.occurs(v, rb)) return Error.Occurs;
                self.subst[v] = rb;
                return;
            },
            else => {},
        }
        switch (self.types[rb]) {
            .tvar => |v| {
                if (self.occurs(v, ra)) return Error.Occurs;
                self.subst[v] = ra;
                return;
            },
            else => {},
        }
        return switch (self.types[ra]) {
            .int => if (self.types[rb] == .int) {} else Error.Mismatch,
            .boolean => if (self.types[rb] == .boolean) {} else Error.Mismatch,
        .str => if (self.types[rb] == .str) {} else Error.Mismatch,
            .func => |fa| switch (self.types[rb]) {
                .func => |fb| {
                    try self.unify(fa.p, fb.p);
                    try self.unify(fa.r, fb.r);
                },
                else => Error.Mismatch,
            },
            .seq => |ea| switch (self.types[rb]) {
                .seq => |eb| try self.unify(ea, eb),
                else => Error.Mismatch,
            },
            .record => |ra2| switch (self.types[rb]) {
                .record => |rb2| {
                    if (ra2.count != rb2.count) return Error.Mismatch;
                    var i: u32 = 0;
                    while (i < ra2.count) : (i += 1) {
                        const fa2 = self.fields[ra2.first + i];
                        const fb2 = self.fields[rb2.first + i];
                        if (!std.mem.eql(u8, fa2.key, fb2.key)) return Error.Mismatch;
                        try self.unify(fa2.ty, fb2.ty);
                    }
                },
                else => Error.Mismatch,
            },
            .tvar => unreachable,
        };
    }

    fn push(self: *Ctx, sym: u32, scheme: Scheme, parent: u32) Error!u32 {
        if (self.nenv >= self.env.len) return Error.EnvFull;
        self.env[self.nenv] = .{ .sym = sym, .scheme = scheme, .parent = parent };
        self.nenv += 1;
        return self.nenv - 1;
    }

    fn lookup(self: *const Ctx, sym: u32, cur: u32) Error!Scheme {
        var i = cur;
        while (i != NONE) {
            if (self.env[i].sym == sym) return self.env[i].scheme;
            i = self.env[i].parent;
        }
        return Error.Unbound;
    }

    /// Replace a scheme's quantified variables with fresh ones, so two uses of a
    /// generic binding do not constrain each other. This is where a generic
    /// becomes usable at more than one type.
    fn instantiate(self: *Ctx, s: Scheme) Error!u32 {
        if (s.quantified == 0) return s.ty;
        var map_buf: [32]u32 = undefined;
        if (s.quantified > map_buf.len) return Error.TooManyVars;
        var i: u32 = 0;
        while (i < s.quantified) : (i += 1) map_buf[i] = try self.fresh();
        return self.copy(s.ty, s.first, s.quantified, map_buf[0..s.quantified]);
    }

    fn copy(self: *Ctx, t: u32, first: u32, count: u32, map: []const u32) Error!u32 {
        const r = self.resolve(t);
        return switch (self.types[r]) {
            .tvar => |v| if (v >= first and v < first + count) map[v - first] else r,
            .int, .boolean, .str => r,
            .seq => |e| try self.alloc(.{ .seq = try self.copy(e, first, count, map) }),
            .func => |f| blk: {
                const p = try self.copy(f.p, first, count, map);
                const rr = try self.copy(f.r, first, count, map);
                break :blk try self.alloc(.{ .func = .{ .p = p, .r = rr } });
            },
            .record => |rec| blk: {
                // Copy every field type BEFORE reserving the span. Recursing
                // into a nested record appends to this same array, so writing
                // the parent's entries as we go interleaves the child's into
                // the parent's span and the field lookup then misses.
                var tmp: [16]u32 = undefined;
                if (rec.count > tmp.len) return Error.OutOfTypes;
                var i: u32 = 0;
                while (i < rec.count) : (i += 1) {
                    tmp[i] = try self.copy(self.fields[rec.first + i].ty, first, count, map);
                }
                if (@as(usize, self.nf) + rec.count > self.fields.len) return Error.OutOfTypes;
                const at = self.nf;
                i = 0;
                while (i < rec.count) : (i += 1) {
                    self.fields[self.nf] = .{ .key = self.fields[rec.first + i].key, .ty = tmp[i] };
                    self.nf += 1;
                }
                break :blk try self.alloc(.{ .record = .{ .first = at, .count = rec.count } });
            },
        };
    }
};

/// The arithmetic family's operation types, which in the real stage arrive as
/// part of the same signature data that carries the operation bodies. The
/// checker reads them; it does not know what arithmetic means.
fn opType(ctx: *Ctx, op: u32) Error!struct { arg: u32, res: u32 } {
    const int = try ctx.alloc(.int);
    return switch (op) {
        cl.OP_ADD, cl.OP_SUB, cl.OP_MUL => .{ .arg = int, .res = int },
        cl.OP_LT => .{ .arg = int, .res = try ctx.alloc(.boolean) },
        else => Error.Unsupported,
    };
}

const Image = struct {
    bytes: []const u8,
    node_count: u32,
    pool_base: usize,
    pool_count: u32,
    root: u32,

    fn parse(bytes: []const u8) Error!Image {
        if (bytes.len < cl.HEADER_WORDS * cl.WORD) return Error.Corrupt;
        const node_count = rd(bytes, 3 * cl.WORD);
        const pool_count = rd(bytes, 4 * cl.WORD);
        const root = rd(bytes, 6 * cl.WORD);
        const pool_base = cl.HEADER_WORDS * cl.WORD + @as(usize, node_count) * cl.NODE_WORDS * cl.WORD;
        return .{ .bytes = bytes, .node_count = node_count, .pool_base = pool_base, .pool_count = pool_count, .root = root };
    }

    fn word(self: *const Image, idx: u32, slot: usize) Error!u32 {
        if (idx >= self.node_count) return Error.Corrupt;
        return rd(self.bytes, cl.HEADER_WORDS * cl.WORD + (@as(usize, idx) * cl.NODE_WORDS + slot) * cl.WORD);
    }

    fn blob(self: *const Image, off: u32, len: u32) Error![]const u8 {
        const base = self.pool_base + @as(usize, self.pool_count) * cl.WORD;
        if (base + @as(usize, off) + @as(usize, len) > self.bytes.len) return Error.Corrupt;
        return self.bytes[base + off .. base + off + len];
    }

    fn pooled(self: *const Image, at: u32) Error!u32 {
        if (at >= self.pool_count) return Error.Corrupt;
        return rd(self.bytes, self.pool_base + @as(usize, at) * cl.WORD);
    }
};

fn rd(bytes: []const u8, at: usize) u32 {
    return std.mem.readInt(u32, bytes[at..][0..4], .little);
}

fn infer(img: *const Image, idx: u32, ctx: *Ctx, cur: u32) Error!u32 {
    switch (try img.word(idx, 0)) {
        cl.TAG_LIT => return switch (try img.word(idx, 1)) {
            cl.LIT_INT => ctx.alloc(.int),
            cl.LIT_STR => ctx.alloc(.str),
            else => Error.Unsupported,
        },
        cl.TAG_VAR => return ctx.instantiate(try ctx.lookup(try img.word(idx, 1), cur)),
        cl.TAG_LET => {
            const rec = (try img.word(idx, 1)) != 0;
            const name = try img.word(idx, 2);
            const mark = ctx.nvars;
            var vt: u32 = undefined;
            var scope = cur;
            if (rec) {
                // The binder is in scope for its own value, monomorphically:
                // generalising before the value is inferred would let a
                // recursive call take a type the definition has not earned.
                const hole = try ctx.fresh();
                scope = try ctx.push(name, .{ .ty = hole, .quantified = 0, .first = 0 }, cur);
                vt = try infer(img, try img.word(idx, 3), ctx, scope);
                try ctx.unify(hole, vt);
            } else {
                vt = try infer(img, try img.word(idx, 3), ctx, cur);
            }
            // Generalise the variables this binding introduced. A binding whose
            // type still mentions only its own fresh variables is generic in
            // them, which is what makes one definition usable at many types.
            const gen = ctx.nvars - mark;
            const body_scope = try ctx.push(name, .{ .ty = vt, .quantified = gen, .first = mark }, cur);
            return infer(img, try img.word(idx, 4), ctx, body_scope);
        },
        cl.TAG_LAMBDA => {
            const pt = try ctx.fresh();
            const scope = try ctx.push(try img.word(idx, 1), .{ .ty = pt, .quantified = 0, .first = 0 }, cur);
            const rt = try infer(img, try img.word(idx, 2), ctx, scope);
            return ctx.alloc(.{ .func = .{ .p = pt, .r = rt } });
        },
        cl.TAG_APPLY => {
            var ft = try infer(img, try img.word(idx, 1), ctx, cur);
            const start = try img.word(idx, 2);
            const len = try img.word(idx, 3);
            var k: u32 = 0;
            while (k < len) : (k += 1) {
                const at = try infer(img, try img.pooled(start + k), ctx, cur);
                const rt = try ctx.fresh();
                const want = try ctx.alloc(.{ .func = .{ .p = at, .r = rt } });
                try ctx.unify(ft, want);
                ft = rt;
            }
            return ft;
        },
        cl.TAG_PROJECT => {
            const bt = ctx.resolve(try infer(img, try img.word(idx, 1), ctx, cur));
            const key = try img.blob(try img.word(idx, 2), try img.word(idx, 3));
            const rec = switch (ctx.types[bt]) {
                .record => |r| r,
                // A projection off an unresolved variable would need row
                // polymorphism to type; refusing is honest until that exists.
                else => return Error.Mismatch,
            };
            var i: u32 = 0;
            while (i < rec.count) : (i += 1) {
                const f = ctx.fields[rec.first + i];
                if (std.mem.eql(u8, f.key, key)) return f.ty;
            }
            return Error.NoSuchField;
        },
        cl.TAG_IF => {
            const ct = try infer(img, try img.word(idx, 1), ctx, cur);
            const b = try ctx.alloc(.boolean);
            try ctx.unify(ct, b);
            const tt = try infer(img, try img.word(idx, 2), ctx, cur);
            const et = try infer(img, try img.word(idx, 3), ctx, cur);
            try ctx.unify(tt, et);
            return tt;
        },
        cl.TAG_RAW => {
            const start = try img.word(idx, 2);
            const len = try img.word(idx, 3);
            const op_node = try img.pooled(start);
            // The opcode is a literal the lowering placed; its value is read
            // from the image rather than inferred, because it names which
            // operation this is rather than being an operand of it.
            const lo = try img.word(op_node, 2);
            if (lo == cl.OP_MAKE_REC) {
                // Same ordering constraint as `copy`: infer every field first,
                // because inferring a nested record literal appends to the very
                // array this record's span is about to name.
                var keys: [16][]const u8 = undefined;
                var tys: [16]u32 = undefined;
                var nfields: u32 = 0;
                var k: u32 = 1;
                while (k + 1 < len) : (k += 2) {
                    if (nfields == keys.len) return Error.OutOfTypes;
                    const kn = try img.pooled(start + k);
                    keys[nfields] = try img.blob(try img.word(kn, 2), try img.word(kn, 3));
                    tys[nfields] = try infer(img, try img.pooled(start + k + 1), ctx, cur);
                    nfields += 1;
                }
                if (@as(usize, ctx.nf) + nfields > ctx.fields.len) return Error.OutOfTypes;
                const at = ctx.nf;
                var j: u32 = 0;
                while (j < nfields) : (j += 1) {
                    ctx.fields[ctx.nf] = .{ .key = keys[j], .ty = tys[j] };
                    ctx.nf += 1;
                }
                return ctx.alloc(.{ .record = .{ .first = at, .count = nfields } });
            }
            if (lo == cl.OP_MAKE_SEQ) {
                // Every element unifies with one element type, which is what
                // makes the sequence homogeneous rather than merely uniform.
                const et = try ctx.fresh();
                var k: u32 = 1;
                while (k < len) : (k += 1) {
                    const it = try infer(img, try img.pooled(start + k), ctx, cur);
                    try ctx.unify(it, et);
                }
                return ctx.alloc(.{ .seq = et });
            }
            if (lo == cl.OP_LEN) {
                const st = try infer(img, try img.pooled(start + 1), ctx, cur);
                const et = try ctx.fresh();
                const want = try ctx.alloc(.{ .seq = et });
                try ctx.unify(st, want);
                return ctx.alloc(.int);
            }
            if (lo == cl.OP_AT) {
                const st = try infer(img, try img.pooled(start + 1), ctx, cur);
                const it = try infer(img, try img.pooled(start + 2), ctx, cur);
                const int = try ctx.alloc(.int);
                try ctx.unify(it, int);
                const et = try ctx.fresh();
                const want = try ctx.alloc(.{ .seq = et });
                try ctx.unify(st, want);
                return et;
            }
            if (lo == cl.OP_PUSH) {
                const st = try infer(img, try img.pooled(start + 1), ctx, cur);
                const vt = try infer(img, try img.pooled(start + 2), ctx, cur);
                const want = try ctx.alloc(.{ .seq = vt });
                try ctx.unify(st, want);
                return want;
            }
            const sig = try opType(ctx, lo);
            var k: u32 = 1;
            while (k < len) : (k += 1) {
                const at = try infer(img, try img.pooled(start + k), ctx, cur);
                try ctx.unify(at, sig.arg);
            }
            return sig.res;
        },
        else => return Error.Unsupported,
    }
}

/// Infer the program's type, refusing an ill-typed one.
pub fn check(image: []const u8, ctx: *Ctx) Error!u32 {
    const img = try Image.parse(image);
    const t = try infer(&img, img.root, ctx, NONE);
    return ctx.resolve(t);
}

// ---------------------------------------------------------------- tests

const Shape = enum { int, boolean, str, func, record, seq };

fn typeOf(src: []const u8) !Shape {
    var node_buf: [8192 * cl.NODE_WORDS]u32 = undefined;
    var pool_buf: [4096]u32 = undefined;
    var name_buf: [512][]const u8 = undefined;
    var blob_buf: [8192]u8 = undefined;
    var image: [524288]u8 = undefined;
    var types: [32768]Ty = undefined;
    var subst: [8192]u32 = undefined;
    var env: [4096]TyBinding = undefined;
    var fields: [4096]Field = undefined;

    var b = cl.Builder{ .nodes = &node_buf, .pool = &pool_buf, .blob = &blob_buf };
    var names = cl.Names{ .buf = &name_buf };
    var p = try cl.Parser.init(src, &b, &names);
    const root = try p.program();
    const len = try cl.writeImage(&b, root, &image);

    var ctx = Ctx{ .types = &types, .subst = &subst, .env = &env, .fields = &fields };
    const t = try check(image[0..len], &ctx);
    return switch (ctx.types[t]) {
        .int => .int,
        .boolean => .boolean,
        .str => .str,
        .func => .func,
        .record => .record,
        .seq => .seq,
        .tvar => .func,
    };
}

test "a string literal has a string type" {
    try std.testing.expectEqual(Shape.str, try typeOf("\"hi\""));
    try std.testing.expectEqual(Shape.str, try typeOf("let s = \"hi\"; s"));
    try std.testing.expectEqual(Shape.str, try typeOf("if 1 < 2 { \"a\" } else { \"b\" }"));
}

test "a string is not an int" {
    try std.testing.expectError(Error.Mismatch, typeOf("\"a\" + 1"));
    try std.testing.expectError(Error.Mismatch, typeOf("if \"a\" { 1 } else { 2 }"));
    try std.testing.expectError(Error.Mismatch, typeOf("if 1 < 2 { \"a\" } else { 1 }"));
}

test "arithmetic is int and comparison is bool, from the operation signatures" {
    try std.testing.expectEqual(Shape.int, try typeOf("1 + 2 * 3"));
    try std.testing.expectEqual(Shape.boolean, try typeOf("1 < 2"));
}

test "the conditional demands a bool, which the untyped runtime could not" {
    try std.testing.expectEqual(Shape.int, try typeOf("if 1 < 2 { 1 } else { 2 }"));
    // Before this pass existed, `if 1 {...}` evaluated happily because a bool
    // was 0 or 1 in an i64. It is now refused.
    try std.testing.expectError(Error.Mismatch, typeOf("if 1 { 1 } else { 2 }"));
}

test "both branches must agree" {
    try std.testing.expectError(Error.Mismatch, typeOf("if 1 < 2 { 1 } else { 2 < 3 }"));
}

test "an operand of the wrong type is refused" {
    try std.testing.expectError(Error.Mismatch, typeOf("1 + (2 < 3)"));
}

test "a function is inferred and applied at its type" {
    try std.testing.expectEqual(Shape.int, try typeOf("fn double(n) { n * 2 } double(3)"));
    try std.testing.expectError(Error.Mismatch, typeOf("fn double(n) { n * 2 } double(1 < 2)"));
}

test "a generic binding is usable at more than one type" {
    // `id` is generalised, so its two uses do not constrain each other. This is
    // the seed the bounds and associated types grow from: without it, the first
    // use would pin the second.
    try std.testing.expectEqual(Shape.int, try typeOf(
        \\fn id(x) { x }
        \\if id(1 < 2) { id(1) } else { 0 }
    ));
}

test "self application is refused by the occurs check" {
    try std.testing.expectError(Error.Occurs, typeOf("fn f(x) { x(x) } 0"));
}

test "recursion type-checks and stays monomorphic in its own body" {
    try std.testing.expectEqual(Shape.int, try typeOf(
        \\fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } }
        \\fact(5)
    ));
}

test "an unbound name is refused before evaluation" {
    try std.testing.expectError(Error.Unbound, typeOf("x + 1"));
}

test "a record literal has a record type and its fields keep their own types" {
    try std.testing.expectEqual(Shape.record, try typeOf("{ a: 1, b: \"x\" }"));
    try std.testing.expectEqual(Shape.int, try typeOf("let r = { a: 1, b: \"x\" }; r.a"));
    try std.testing.expectEqual(Shape.str, try typeOf("let r = { a: 1, b: \"x\" }; r.b"));
}

test "records with different shapes do not unify" {
    try std.testing.expectError(Error.Mismatch, typeOf("if 1 < 2 { { a: 1 } } else { { a: \"x\" } }"));
    try std.testing.expectError(Error.Mismatch, typeOf("if 1 < 2 { { a: 1 } } else { { b: 1 } }"));
    try std.testing.expectError(Error.Mismatch, typeOf("if 1 < 2 { { a: 1 } } else { { a: 1, b: 2 } }"));
}

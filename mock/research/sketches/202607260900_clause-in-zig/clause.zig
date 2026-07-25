//! Clause source to Core IR, in the runtime, with no Rust in the loop.
//!
//! The canon puts every per-script analysis in the runtime artifact ("Rust never
//! runs per-script analyses, the runtime does, for all scripts", inverse
//! catalogue:38) and states that the Rust side never sees an end-user script
//! (positive:45). So the lexer, the parser, and the lowering to Core belong
//! here, in Zig, and this file is where that starts.
//!
//! No allocation. Every buffer is caller-lent, matching the runtime's own
//! discipline; a program that outgrows what it was lent is refused rather than
//! grown.

const std = @import("std");

pub const WORD: usize = 4;
pub const HEADER_WORDS: usize = 8;
pub const NODE_WORDS: usize = 7;
pub const MAGIC: u32 = 0x3048_4556;

pub const TAG_LIT: u32 = 0;
pub const TAG_VAR: u32 = 1;
pub const TAG_LET: u32 = 2;
pub const TAG_LAMBDA: u32 = 3;
pub const TAG_APPLY: u32 = 4;
pub const TAG_PROJECT: u32 = 5;
pub const TAG_IF: u32 = 6;
pub const TAG_MATCH: u32 = 7;
pub const TAG_RAW: u32 = 10;

/// Pattern nodes. They live in the same arena as expressions but are never
/// evaluated as expressions; the match arm is the only thing that reads them.
/// Keeping them here rather than in a separate region is a sketch choice, and
/// the Core round that settles the framework's pattern representation is where
/// that gets decided properly.
pub const PAT_WILD: u32 = 20;
pub const PAT_BIND: u32 = 21;
pub const PAT_LIT_INT: u32 = 22;
pub const PAT_LIT_STR: u32 = 23;
pub const PAT_REC: u32 = 24;
/// Alternatives. They may not bind, so both sides agree on the empty set of
/// bindings by construction rather than by a check that they match.
pub const PAT_OR: u32 = 25;
/// An integer range. `inclusive` is the third slot.
pub const PAT_RANGE: u32 = 26;

/// No guard on this arm.
pub const NO_GUARD: u32 = 0xFFFF_FFFF;

pub const LIT_UNIT: u32 = 0;
pub const LIT_INT: u32 = 2;
pub const LIT_STR: u32 = 3;

/// The arithmetic family's id. A consumer language declares its own families;
/// the framework declares none.
pub const ARITH: u32 = 1;

/// Operation codes within the arithmetic family. These name entries in the
/// operation table, whose bodies are programs over the closed primitive
/// vocabulary; they are not themselves the semantics.
pub const OP_ADD: u32 = 0;
pub const OP_SUB: u32 = 1;
pub const OP_MUL: u32 = 2;
pub const OP_LT: u32 = 3;
/// Record construction. Unlike the scalar operations above, its arity is not
/// fixed: it takes alternating key and value operands and yields a compound.
/// That shape is the coverage finding recorded in the round topic, and this is
/// the candidate that answers it by growing the vocabulary rather than by adding
/// a second kind of table entry.
pub const OP_MAKE_REC: u32 = 4;
/// Sequence construction, the second variable-arity constructor.
pub const OP_MAKE_SEQ: u32 = 5;
/// Sequence elimination. These are the beginnings of a prelude: in the finished
/// shape the language definition supplies them as ordinary operations rather
/// than the parser recognising two names, and that shortcut is noted in the
/// sketch's README.
pub const OP_LEN: u32 = 6;
pub const OP_AT: u32 = 7;
/// Extend a sequence, yielding a new one. Values are immutable, so this is a
/// construction rather than a mutation, which is what keeps the lease proof
/// static.
pub const OP_PUSH: u32 = 8;

/// A type as written in a trait method's signature. `Self` is the implementing
/// type; the rest are the concrete types this subset has.
pub const TyName = enum { self_ty, assoc_ty, int, boolean, str };

/// A trait: one method, its parameter types, and its result type. One method per
/// trait keeps the constraint machinery honest without a method table, and the
/// generalisation to several is mechanical.
/// One method's declared shape.
pub const MethodDecl = struct {
    name: []const u8,
    /// The method name's interned sym, so the checker can bind it without
    /// re-interning and the parser stays the only place names are interned.
    sym: u32,
    params: [4]TyName,
    nparams: u8,
    ret: TyName,
};

pub const MAX_METHODS: usize = 4;

pub const TraitDecl = struct {
    name: []const u8,
    methods: [MAX_METHODS]MethodDecl,
    nmethods: u8,
    /// The associated type's name, empty when the trait declares none. An
    /// associated type is a second thing an impl chooses, alongside the method
    /// bodies, and any signature may mention it wherever it mentions Self.
    assoc_name: []const u8 = "",
    /// The supertrait's index, or NO_SUPER. An impl of this trait requires an
    /// impl of its supertrait for the same type.
    super_idx: u32 = NO_SUPER,
};

/// This trait has no supertrait.
pub const NO_SUPER: u32 = 0xFFFF_FFFF;

/// An implementation: which trait, for which type, and the binder its method
/// body was bound under. Coherence is one impl per trait-and-type pair.
pub const ImplDecl = struct {
    trait_idx: u32,
    for_ty: TyName,
    /// One binder per method, in the trait's declaration order.
    method_syms: [MAX_METHODS]u32,
    assoc: TyName = .int,
};

/// Impl-method binders start here. Source names are interner indices counting
/// from zero, so the two ranges cannot meet.
pub const IMPL_SYM_BASE: u32 = 0x4000_0000;

/// Loop-function binders. Disjoint from both source names and impl methods.
pub const LOOP_SYM_BASE: u32 = 0x5000_0000;

/// Binders a desugaring invents (a `for` loop's index and its sequence), from a
/// range no source name can reach, so a program that happens to use the name
/// `i` cannot capture or be captured by one.
pub const SYN_SYM_BASE: u32 = 0x6000_0000;

/// A macro: a function evaluated at a compile stage rather than at run time.
/// The census's reading, made operational: the operation is discharged by
/// whichever stage provides its handler, and for a macro that stage is this one.
pub const MacroDecl = struct {
    sym: u32,
    params: [4]u32,
    nparams: u8,
    body: u32,
    ret: TyName,
};

pub const Error = error{
    UnexpectedByte,
    UnexpectedToken,
    OutOfNodes,
    OutOfPool,
    OutOfBlob,
    Unterminated,
    TooManyNames,
    TooManyParams,
    Unsupported,
    TooManyTraits,
    UnknownTrait,
    UnknownType,
    DuplicateImpl,
    NonExhaustive,
    NotMutable,
    MissingMethod,
    BindingInAlternative,
    NotConstant,
    UnknownMacro,
    WrongMacroArity,
};

// ---------------------------------------------------------------- lexer

const Kind = enum { int, str_lit, ident, dot, colon, lbracket, rbracket, arrow, kw_trait, kw_impl, kw_for, kw_type, kw_match, kw_while, kw_mut, kw_mod, kw_use, kw_in, kw_macro, kw_pub, bang, pound, colon_colon, kw_if_guard, dotdot, dotdot_eq, pipe, fat_arrow, underscore, plus_assign, minus_assign, kw_let, kw_fn, kw_if, kw_else, plus, minus, star, lt, assign, semi, comma, lparen, rparen, lbrace, rbrace, eof };

const Token = struct { kind: Kind, start: usize, end: usize, value: i64 };

const Lexer = struct {
    src: []const u8,
    i: usize = 0,

    fn next(self: *Lexer) Error!Token {
        // Whitespace and line comments are trivia. A doc-comment form belongs
        // here later, because the doc pass consumes it and a lexer that folds it
        // into trivia makes that pass impossible.
        while (self.i < self.src.len) {
            const ch = self.src[self.i];
            if (ch == ' ' or ch == '\n' or ch == '\t' or ch == '\r') {
                self.i += 1;
                continue;
            }
            if (ch == '/' and self.i + 1 < self.src.len and self.src[self.i + 1] == '/') {
                while (self.i < self.src.len and self.src[self.i] != '\n') self.i += 1;
                continue;
            }
            break;
        }
        const start = self.i;
        if (self.i >= self.src.len) return .{ .kind = .eof, .start = start, .end = start, .value = 0 };
        const c = self.src[self.i];
        if (c >= '0' and c <= '9') {
            var v: i64 = 0;
            while (self.i < self.src.len and self.src[self.i] >= '0' and self.src[self.i] <= '9') {
                v = v * 10 + @as(i64, self.src[self.i] - '0');
                self.i += 1;
            }
            return .{ .kind = .int, .start = start, .end = self.i, .value = v };
        }
        if (c == '_' or (c | 0x20) >= 'a' and (c | 0x20) <= 'z') {
            while (self.i < self.src.len) {
                const d = self.src[self.i];
                if (d == '_' or (d >= '0' and d <= '9') or ((d | 0x20) >= 'a' and (d | 0x20) <= 'z')) {
                    self.i += 1;
                } else break;
            }
            const text = self.src[start..self.i];
            const kind: Kind = if (std.mem.eql(u8, text, "let"))
                .kw_let
            else if (std.mem.eql(u8, text, "fn"))
                .kw_fn
            else if (std.mem.eql(u8, text, "trait"))
                .kw_trait
            else if (std.mem.eql(u8, text, "impl"))
                .kw_impl
            else if (std.mem.eql(u8, text, "for"))
                .kw_for
            else if (std.mem.eql(u8, text, "type"))
                .kw_type
            else if (std.mem.eql(u8, text, "match"))
                .kw_match
            else if (std.mem.eql(u8, text, "while"))
                .kw_while
            else if (std.mem.eql(u8, text, "mut"))
                .kw_mut
            else if (std.mem.eql(u8, text, "mod"))
                .kw_mod
            else if (std.mem.eql(u8, text, "use"))
                .kw_use
            else if (std.mem.eql(u8, text, "in"))
                .kw_in
            else if (std.mem.eql(u8, text, "macro"))
                .kw_macro
            else if (std.mem.eql(u8, text, "pub"))
                .kw_pub
            else if (std.mem.eql(u8, text, "_"))
                .underscore
            else if (std.mem.eql(u8, text, "if"))
                .kw_if
            else if (std.mem.eql(u8, text, "else"))
                .kw_else
            else
                .ident;
            return .{ .kind = kind, .start = start, .end = self.i, .value = 0 };
        }
        if (c == '"') {
            self.i += 1;
            while (self.i < self.src.len and self.src[self.i] != '"') self.i += 1;
            if (self.i >= self.src.len) return Error.Unterminated;
            self.i += 1;
            return .{ .kind = .str_lit, .start = start, .end = self.i, .value = 0 };
        }
        if (c == '.' and self.i + 1 < self.src.len and self.src[self.i + 1] == '.') {
            if (self.i + 2 < self.src.len and self.src[self.i + 2] == '=') {
                self.i += 3;
                return .{ .kind = .dotdot_eq, .start = start, .end = self.i, .value = 0 };
            }
            self.i += 2;
            return .{ .kind = .dotdot, .start = start, .end = self.i, .value = 0 };
        }
        if (c == ':' and self.i + 1 < self.src.len and self.src[self.i + 1] == ':') {
            self.i += 2;
            return .{ .kind = .colon_colon, .start = start, .end = self.i, .value = 0 };
        }
        if (c == '+' and self.i + 1 < self.src.len and self.src[self.i + 1] == '=') {
            self.i += 2;
            return .{ .kind = .plus_assign, .start = start, .end = self.i, .value = 0 };
        }
        if (c == '-' and self.i + 1 < self.src.len and self.src[self.i + 1] == '=') {
            self.i += 2;
            return .{ .kind = .minus_assign, .start = start, .end = self.i, .value = 0 };
        }
        if (c == '=' and self.i + 1 < self.src.len and self.src[self.i + 1] == '>') {
            self.i += 2;
            return .{ .kind = .fat_arrow, .start = start, .end = self.i, .value = 0 };
        }
        if (c == '-' and self.i + 1 < self.src.len and self.src[self.i + 1] == '>') {
            self.i += 2;
            return .{ .kind = .arrow, .start = start, .end = self.i, .value = 0 };
        }
        self.i += 1;
        const kind: Kind = switch (c) {
            '+' => .plus,
            '-' => .minus,
            '*' => .star,
            '<' => .lt,
            '=' => .assign,
            ';' => .semi,
            ',' => .comma,
            '.' => .dot,
            '|' => .pipe,
            '!' => .bang,
            '#' => .pound,
            '[' => .lbracket,
            ']' => .rbracket,
            ':' => .colon,
            '(' => .lparen,
            ')' => .rparen,
            '{' => .lbrace,
            '}' => .rbrace,
            else => return Error.UnexpectedByte,
        };
        return .{ .kind = kind, .start = start, .end = self.i, .value = 0 };
    }
};

// ---------------------------------------------------------------- IR builder

/// Writes Core nodes straight into the caller's wire buffer, in the layout the
/// runtime already decodes, so no separate AST exists to keep in step with it.
pub const Builder = struct {
    nodes: []u32,
    pool: []u32,
    /// String bytes live in a blob section, named by offset and length, which is
    /// the layout the runtime already decodes.
    blob: []u8,
    n: u32 = 0,
    p: u32 = 0,
    bl: u32 = 0,

    fn slot(self: *Builder, idx: u32, k: usize) *u32 {
        return &self.nodes[@as(usize, idx) * NODE_WORDS + k];
    }

    fn alloc(self: *Builder, tag: u32) Error!u32 {
        if ((@as(usize, self.n) + 1) * NODE_WORDS > self.nodes.len) return Error.OutOfNodes;
        const idx = self.n;
        self.n += 1;
        var k: usize = 0;
        while (k < NODE_WORDS) : (k += 1) self.slot(idx, k).* = 0;
        self.slot(idx, 0).* = tag;
        return idx;
    }

    /// The unit value. A function of no arguments takes one of these, so that
    /// "no arguments" is still an application and the Core needs no nullary
    /// form.
    pub fn unit(self: *Builder) Error!u32 {
        const idx = try self.alloc(TAG_LIT);
        self.slot(idx, 1).* = LIT_UNIT;
        return idx;
    }

    pub fn lit(self: *Builder, v: i64) Error!u32 {
        const idx = try self.alloc(TAG_LIT);
        const bits: u64 = @bitCast(v);
        self.slot(idx, 1).* = LIT_INT;
        self.slot(idx, 2).* = @truncate(bits);
        self.slot(idx, 3).* = @truncate(bits >> 32);
        return idx;
    }

    /// A string literal is a span of the blob, so producing one copies the
    /// bytes once and every later reference is two words.
    pub fn str(self: *Builder, text: []const u8) Error!u32 {
        if (@as(usize, self.bl) + text.len > self.blob.len) return Error.OutOfBlob;
        const off = self.bl;
        @memcpy(self.blob[off .. off + text.len], text);
        self.bl += @intCast(text.len);
        const idx = try self.alloc(TAG_LIT);
        self.slot(idx, 1).* = LIT_STR;
        self.slot(idx, 2).* = off;
        self.slot(idx, 3).* = @intCast(text.len);
        return idx;
    }

    pub fn variable(self: *Builder, sym: u32) Error!u32 {
        const idx = try self.alloc(TAG_VAR);
        self.slot(idx, 1).* = sym;
        return idx;
    }

    pub fn let(self: *Builder, name: u32, value: u32, body: u32) Error!u32 {
        const idx = try self.alloc(TAG_LET);
        self.slot(idx, 1).* = 0; // not recursive
        self.slot(idx, 2).* = name;
        self.slot(idx, 3).* = value;
        self.slot(idx, 4).* = body;
        return idx;
    }

    pub fn cond(self: *Builder, c: u32, t: u32, e: u32) Error!u32 {
        const idx = try self.alloc(TAG_IF);
        self.slot(idx, 1).* = c;
        self.slot(idx, 2).* = t;
        self.slot(idx, 3).* = e;
        return idx;
    }

    /// A binding in scope for its own value, which is what makes a recursive
    /// function terminate rather than fail to find itself.
    pub fn letRec(self: *Builder, name: u32, value: u32, body: u32) Error!u32 {
        const idx = try self.let(name, value, body);
        self.slot(idx, 1).* = 1;
        return idx;
    }

    pub fn lambda(self: *Builder, param: u32, body: u32) Error!u32 {
        const idx = try self.alloc(TAG_LAMBDA);
        self.slot(idx, 1).* = param;
        self.slot(idx, 2).* = body;
        return idx;
    }

    /// Application takes its arguments from the pool. Core application is one
    /// argument at a time, so a multi-argument call is a chain of these.
    pub fn apply(self: *Builder, callee: u32, args: []const u32) Error!u32 {
        if (@as(usize, self.p) + args.len > self.pool.len) return Error.OutOfPool;
        const start = self.p;
        for (args) |a| {
            self.pool[self.p] = a;
            self.p += 1;
        }
        const idx = try self.alloc(TAG_APPLY);
        self.slot(idx, 1).* = callee;
        self.slot(idx, 2).* = start;
        self.slot(idx, 3).* = @intCast(args.len);
        return idx;
    }

    /// A record: a family operation over alternating key and value operands.
    /// Construction is a family concern because content-as-values makes it the
    /// signature's introduction projection; the Core is the eliminator algebra.
    pub fn record(self: *Builder, kv: []const u32) Error!u32 {
        const code = try self.lit(@intCast(OP_MAKE_REC));
        if (@as(usize, self.p) + 1 + kv.len > self.pool.len) return Error.OutOfPool;
        const start = self.p;
        self.pool[self.p] = code;
        self.p += 1;
        for (kv) |x| {
            self.pool[self.p] = x;
            self.p += 1;
        }
        const idx = try self.alloc(TAG_RAW);
        self.slot(idx, 1).* = ARITH;
        self.slot(idx, 2).* = start;
        self.slot(idx, 3).* = @intCast(1 + kv.len);
        return idx;
    }

    pub fn patWild(self: *Builder) Error!u32 {
        return self.alloc(PAT_WILD);
    }

    pub fn patBind(self: *Builder, sym: u32) Error!u32 {
        const idx = try self.alloc(PAT_BIND);
        self.slot(idx, 1).* = sym;
        return idx;
    }

    pub fn patInt(self: *Builder, v: i64) Error!u32 {
        const idx = try self.alloc(PAT_LIT_INT);
        const bits: u64 = @bitCast(v);
        self.slot(idx, 1).* = @truncate(bits);
        self.slot(idx, 2).* = @truncate(bits >> 32);
        return idx;
    }

    pub fn patStr(self: *Builder, text: []const u8) Error!u32 {
        if (@as(usize, self.bl) + text.len > self.blob.len) return Error.OutOfBlob;
        const off = self.bl;
        @memcpy(self.blob[off .. off + text.len], text);
        self.bl += @intCast(text.len);
        const idx = try self.alloc(PAT_LIT_STR);
        self.slot(idx, 1).* = off;
        self.slot(idx, 2).* = @intCast(text.len);
        return idx;
    }

    /// A record pattern: alternating key-string and sub-pattern nodes in the
    /// pool, the same shape the record literal uses for its operands.
    pub fn patOr(self: *Builder, a: u32, b: u32) Error!u32 {
        const idx = try self.alloc(PAT_OR);
        self.slot(idx, 1).* = a;
        self.slot(idx, 2).* = b;
        return idx;
    }

    pub fn patRange(self: *Builder, lo: i64, hi: i64, inclusive: bool) Error!u32 {
        const idx = try self.alloc(PAT_RANGE);
        const l: u64 = @bitCast(lo);
        const h: u64 = @bitCast(hi);
        self.slot(idx, 1).* = @truncate(l);
        self.slot(idx, 2).* = @truncate(l >> 32);
        self.slot(idx, 3).* = @truncate(h);
        self.slot(idx, 4).* = @truncate(h >> 32);
        self.slot(idx, 5).* = if (inclusive) 1 else 0;
        return idx;
    }

    pub fn patRec(self: *Builder, kv: []const u32) Error!u32 {
        if (@as(usize, self.p) + kv.len > self.pool.len) return Error.OutOfPool;
        const start = self.p;
        for (kv) |x| {
            self.pool[self.p] = x;
            self.p += 1;
        }
        const idx = try self.alloc(PAT_REC);
        self.slot(idx, 1).* = start;
        self.slot(idx, 2).* = @intCast(kv.len);
        return idx;
    }

    /// Arms are triples in the pool: a pattern, its guard (or NO_GUARD), then
    /// its body.
    pub fn match_(self: *Builder, scrutinee: u32, arms: []const u32) Error!u32 {
        if (@as(usize, self.p) + arms.len > self.pool.len) return Error.OutOfPool;
        const start = self.p;
        for (arms) |x| {
            self.pool[self.p] = x;
            self.p += 1;
        }
        const idx = try self.alloc(TAG_MATCH);
        self.slot(idx, 1).* = scrutinee;
        self.slot(idx, 2).* = start;
        self.slot(idx, 3).* = @intCast(arms.len / 3);
        return idx;
    }

    /// A family operation over a variable number of operands, which is the shape
    /// both constructors share.
    pub fn variadic(self: *Builder, op: u32, args: []const u32) Error!u32 {
        const code = try self.lit(@intCast(op));
        if (@as(usize, self.p) + 1 + args.len > self.pool.len) return Error.OutOfPool;
        const start = self.p;
        self.pool[self.p] = code;
        self.p += 1;
        for (args) |x| {
            self.pool[self.p] = x;
            self.p += 1;
        }
        const idx = try self.alloc(TAG_RAW);
        self.slot(idx, 1).* = ARITH;
        self.slot(idx, 2).* = start;
        self.slot(idx, 3).* = @intCast(1 + args.len);
        return idx;
    }

    /// Reading a field back out is elimination, so it is a Core form and not a
    /// family operation. The key is a blob span, the same shape a string uses.
    pub fn project(self: *Builder, base: u32, key: []const u8) Error!u32 {
        if (@as(usize, self.bl) + key.len > self.blob.len) return Error.OutOfBlob;
        const off = self.bl;
        @memcpy(self.blob[off .. off + key.len], key);
        self.bl += @intCast(key.len);
        const idx = try self.alloc(TAG_PROJECT);
        self.slot(idx, 1).* = base;
        self.slot(idx, 2).* = off;
        self.slot(idx, 3).* = @intCast(key.len);
        return idx;
    }

    /// A family operation: the opcode rides as the first operand, so the family
    /// surface is one id rather than one per operator.
    pub fn arith(self: *Builder, op: u32, lhs: u32, rhs: u32) Error!u32 {
        const code = try self.lit(@intCast(op));
        if (@as(usize, self.p) + 3 > self.pool.len) return Error.OutOfPool;
        const start = self.p;
        self.pool[self.p] = code;
        self.pool[self.p + 1] = lhs;
        self.pool[self.p + 2] = rhs;
        self.p += 3;
        const idx = try self.alloc(TAG_RAW);
        self.slot(idx, 1).* = ARITH;
        self.slot(idx, 2).* = start;
        self.slot(idx, 3).* = 3;
        return idx;
    }
};

// ---------------------------------------------------------------- parser

/// Names are interned to a small table; a `Sym` is its index. The runtime binds
/// and looks up by these bits, which is what lets a binder carry identity
/// without carrying text.
pub const Names = struct {
    buf: [][]const u8,
    n: u32 = 0,

    fn intern(self: *Names, text: []const u8) Error!u32 {
        var i: u32 = 0;
        while (i < self.n) : (i += 1) {
            if (std.mem.eql(u8, self.buf[i], text)) return i;
        }
        if (self.n >= self.buf.len) return Error.TooManyNames;
        self.buf[self.n] = text;
        self.n += 1;
        return self.n - 1;
    }
};

pub const Parser = struct {
    lx: Lexer,
    tok: Token,
    b: *Builder,
    names: *Names,
    macros: [16]MacroDecl = undefined,
    nmacros: u32 = 0,
    traits: []TraitDecl,
    ntraits: u32 = 0,
    impls: []ImplDecl,
    nimpls: u32 = 0,
    /// `{` is ambiguous: it opens a block after an `if` condition and a record
    /// literal in ordinary expression position. Cleared while parsing a
    /// condition, which is the same resolution Rust uses.
    allow_record: bool = true,
    /// The mutable locals in scope, innermost last. A `while` becomes a
    /// recursive function over exactly these, so the list is the loop's state.
    muts: [16]u32 = undefined,
    nmuts: u32 = 0,
    /// Loop functions get binders from their own range, disjoint from source
    /// names, for the same reason impl methods do.
    nloops: u32 = 0,

    pub fn init(src: []const u8, b: *Builder, names: *Names, traits: []TraitDecl, impls: []ImplDecl) Error!Parser {
        var lx = Lexer{ .src = src };
        const first = try lx.next();
        return .{ .lx = lx, .tok = first, .b = b, .names = names, .traits = traits, .impls = impls };
    }

    fn tyName(self: *Parser) Error!TyName {
        return self.tyNameIn("");
    }

    /// `assoc` names the associated type in scope, if any, so a signature can
    /// mention it by the name the trait gave it.
    fn tyNameIn(self: *Parser, assoc: []const u8) Error!TyName {
        if (self.tok.kind != .ident) return Error.UnexpectedToken;
        const t = self.lx.src[self.tok.start..self.tok.end];
        try self.bump();
        if (assoc.len > 0 and std.mem.eql(u8, t, assoc)) return .assoc_ty;
        if (std.mem.eql(u8, t, "Self")) return .self_ty;
        if (std.mem.eql(u8, t, "Int")) return .int;
        if (std.mem.eql(u8, t, "Bool")) return .boolean;
        if (std.mem.eql(u8, t, "Str")) return .str;
        return Error.UnknownType;
    }

    fn traitIndex(self: *const Parser, name: []const u8) Error!u32 {
        var i: u32 = 0;
        while (i < self.ntraits) : (i += 1) {
            if (std.mem.eql(u8, self.traits[i].name, name)) return i;
        }
        return Error.UnknownTrait;
    }

    /// Every impl's method gets its own binder, so the impls of one trait for
    /// different types are different bindings and dispatch is a choice between
    /// binders rather than a runtime inspection of a value.
    ///
    /// Minted from a numeric range disjoint from source names rather than by
    /// interning a mangled string: a mangled name would have to live somewhere,
    /// and the obvious somewhere is a stack buffer the interner would outlive.
    /// Hygiene here is structural, not a naming convention.
    fn implSym(_: *Parser, impl_index: u32) u32 {
        return IMPL_SYM_BASE + impl_index * @as(u32, MAX_METHODS);
    }

    fn bump(self: *Parser) Error!void {
        self.tok = try self.lx.next();
    }

    fn expect(self: *Parser, k: Kind) Error!void {
        if (self.tok.kind != k) return Error.UnexpectedToken;
        try self.bump();
    }

    /// A program is a sequence of `let` bindings followed by a tail expression,
    /// which is exactly the shape `Let` nests into: each binding's body is
    /// everything after it.
    /// Attributes are parsed and dropped. They are metadata for a later stage
    /// (a target, a doc pass) and carry no meaning to the checker or the
    /// evaluator, so erasing them here is the honest treatment rather than a
    /// shortcut: nothing downstream has asked for them yet.
    fn skipAttributes(self: *Parser) Error!void {
        while (self.tok.kind == .pound) {
            try self.bump();
            try self.expect(.lbracket);
            var depth: u32 = 1;
            while (depth > 0) {
                if (self.tok.kind == .eof) return Error.UnexpectedToken;
                if (self.tok.kind == .lbracket) depth += 1;
                if (self.tok.kind == .rbracket) depth -= 1;
                try self.bump();
            }
        }
    }

    pub fn program(self: *Parser) Error!u32 {
        try self.skipAttributes();
        // `pub` at the top level is accepted and means nothing: there is no
        // enclosing module to be private from.
        if (self.tok.kind == .kw_pub) try self.bump();
        if (self.tok.kind == .kw_trait) {
            // A trait declares a method's shape. It binds nothing and erases
            // entirely; only the checker ever sees it.
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const tname = self.lx.src[self.tok.start..self.tok.end];
            try self.bump();
            // `trait B: A { .. }` declares A as B's supertrait. Resolved here
            // because A must already be declared to be named.
            var super_idx: u32 = NO_SUPER;
            if (self.tok.kind == .colon) {
                try self.bump();
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                super_idx = try self.traitIndex(self.lx.src[self.tok.start..self.tok.end]);
                try self.bump();
            }
            try self.expect(.lbrace);
            var assoc_name: []const u8 = "";
            if (self.tok.kind == .kw_type) {
                try self.bump();
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                assoc_name = self.lx.src[self.tok.start..self.tok.end];
                try self.bump();
                try self.expect(.semi);
            }
            var ms: [MAX_METHODS]MethodDecl = undefined;
            var nm: u8 = 0;
            while (self.tok.kind == .kw_fn) {
                try self.bump();
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                if (nm == MAX_METHODS) return Error.TooManyTraits;
                const mname = self.lx.src[self.tok.start..self.tok.end];
                try self.bump();
                try self.expect(.lparen);
                var ps: [4]TyName = undefined;
                var np: u8 = 0;
                while (self.tok.kind != .rparen) {
                    if (np == ps.len) return Error.TooManyParams;
                    ps[np] = try self.tyNameIn(assoc_name);
                    np += 1;
                    if (self.tok.kind == .comma) try self.bump();
                }
                try self.expect(.rparen);
                try self.expect(.arrow);
                const ret = try self.tyNameIn(assoc_name);
                ms[nm] = .{
                    .name = mname,
                    .sym = try self.names.intern(mname),
                    .params = ps,
                    .nparams = np,
                    .ret = ret,
                };
                nm += 1;
            }
            try self.expect(.rbrace);
            if (nm == 0) return Error.UnexpectedToken;
            if (self.ntraits == self.traits.len) return Error.TooManyTraits;
            self.traits[self.ntraits] = .{
                .name = tname,
                .methods = ms,
                .nmethods = nm,
                .assoc_name = assoc_name,
                .super_idx = super_idx,
            };
            self.ntraits += 1;
            return self.program();
        }
        if (self.tok.kind == .kw_impl) {
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const tname = self.lx.src[self.tok.start..self.tok.end];
            try self.bump();
            const ti = try self.traitIndex(tname);
            try self.expect(.kw_for);
            const for_ty = try self.tyName();
            // Coherence: one impl per trait and type. Checked here because a
            // second impl would make dispatch ambiguous with nothing to break
            // the tie.
            var j: u32 = 0;
            while (j < self.nimpls) : (j += 1) {
                if (self.impls[j].trait_idx == ti and self.impls[j].for_ty == for_ty) return Error.DuplicateImpl;
            }
            try self.expect(.lbrace);
            var assoc: TyName = .int;
            if (self.tok.kind == .kw_type) {
                try self.bump();
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                try self.bump();
                try self.expect(.assign);
                assoc = try self.tyName();
                try self.expect(.semi);
            }
            if (self.nimpls == self.impls.len) return Error.TooManyTraits;
            const impl_index = self.nimpls;
            var syms: [MAX_METHODS]u32 = undefined;
            var bodies: [MAX_METHODS]u32 = undefined;
            var nb: u8 = 0;
            while (self.tok.kind == .kw_fn) {
                const d = try self.fnDecl();
                if (nb == self.traits[ti].nmethods) return Error.TooManyTraits;
                // Methods are matched to the trait by name, not by order, so an
                // impl may write them in any order and a missing one is caught.
                var mi: u8 = 0;
                var found_m = false;
                while (mi < self.traits[ti].nmethods) : (mi += 1) {
                    if (self.traits[ti].methods[mi].sym == d.name) {
                        found_m = true;
                        break;
                    }
                }
                if (!found_m) return Error.UnknownTrait;
                syms[mi] = self.implSym(impl_index) + mi;
                bodies[mi] = d.value;
                nb += 1;
            }
            try self.expect(.rbrace);
            if (nb != self.traits[ti].nmethods) return Error.MissingMethod;
            self.impls[impl_index] = .{ .trait_idx = ti, .for_ty = for_ty, .method_syms = syms, .assoc = assoc };
            self.nimpls += 1;
            var out = try self.program();
            var mk: u8 = self.traits[ti].nmethods;
            while (mk > 0) {
                mk -= 1;
                out = try self.b.letRec(syms[mk], bodies[mk], out);
            }
            return out;
        }
        if (self.tok.kind == .kw_fn) {
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.lparen);
            var params: [8]u32 = undefined;
            var np: usize = 0;
            while (self.tok.kind != .rparen) {
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                if (np == params.len) return Error.TooManyParams;
                params[np] = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
                np += 1;
                try self.bump();
                if (self.tok.kind == .comma) try self.bump();
            }
            try self.expect(.rparen);
            try self.expect(.lbrace);
            const body = try self.program();
            try self.expect(.rbrace);
            // Curried: Core application takes one argument at a time, so a
            // multi-parameter function is nested lambdas, innermost last.
            var f = body;
            var k = np;
            while (k > 0) {
                k -= 1;
                f = try self.b.lambda(params[k], f);
            }
            const rest = try self.program();
            // Recursive, so the function is in scope for its own body and calls
            // to itself resolve rather than escaping to an outer binding.
            return self.b.letRec(name, f, rest);
        }
        if (self.tok.kind == .kw_mod) {
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const mname = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.lbrace);
            var item_syms: [32]u32 = undefined;
            var item_srcs: [32][]const u8 = undefined;
            var nitems: usize = 0;
            const value = try self.moduleBody(&item_syms, &item_srcs, &nitems);
            try self.expect(.rbrace);
            return self.b.let(mname, value, try self.program());
        }
        if (self.tok.kind == .kw_use) {
            // `use M::f;` binds f to M's f. A path is a projection, so this is
            // an ordinary binding and not a second namespace mechanism.
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const mname = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.colon_colon);
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const item = self.lx.src[self.tok.start..self.tok.end];
            const isym = try self.names.intern(item);
            try self.bump();
            try self.expect(.semi);
            const base = try self.b.variable(mname);
            const proj = try self.b.project(base, item);
            return self.b.let(isym, proj, try self.program());
        }
        if (self.tok.kind == .kw_macro) {
            // A macro declares its result type, which is what makes it a typed
            // function at a compile stage rather than a token rewriter.
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const mname = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.lparen);
            var ps: [4]u32 = undefined;
            var np: u8 = 0;
            while (self.tok.kind != .rparen) {
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                if (np == ps.len) return Error.TooManyParams;
                ps[np] = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
                np += 1;
                try self.bump();
                if (self.tok.kind == .comma) try self.bump();
            }
            try self.expect(.rparen);
            try self.expect(.arrow);
            const ret = try self.tyName();
            try self.expect(.lbrace);
            const body = try self.expression();
            try self.expect(.rbrace);
            if (self.nmacros == self.macros.len) return Error.TooManyTraits;
            self.macros[self.nmacros] = .{ .sym = mname, .params = ps, .nparams = np, .body = body, .ret = ret };
            self.nmacros += 1;
            // A macro binds nothing: it erases entirely, and only its call sites
            // remain, as the constants it produced.
            return self.program();
        }
        if (self.tok.kind == .kw_for) {
            // `for x in E { body }` is the while desugaring with an index the
            // parser supplies: bind the sequence once, count up to its length,
            // bind the element each turn.
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const elem = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.kw_in);
            const saved_f = self.allow_record;
            self.allow_record = false;
            const seq = try self.expression();
            self.allow_record = saved_f;
            try self.expect(.lbrace);

            const s_sym = SYN_SYM_BASE + self.nloops * 2;
            const i_sym = SYN_SYM_BASE + self.nloops * 2 + 1;
            const loop_sym = LOOP_SYM_BASE + self.nloops;
            self.nloops += 1;

            // The index joins the mutable locals, so the loop function carries
            // it alongside whatever the enclosing scope was already threading.
            if (self.nmuts == self.muts.len) return Error.TooManyParams;
            self.muts[self.nmuts] = i_sym;
            self.nmuts += 1;
            const n = self.nmuts;

            const body = try self.forBody(loop_sym, n, i_sym, s_sym, elem);
            try self.expect(.rbrace);
            self.nmuts -= 1;
            const rest = try self.program();

            const len_call = try self.b.variadic(OP_LEN, &[_]u32{try self.b.variable(s_sym)});
            const cond = try self.b.arith(OP_LT, try self.b.variable(i_sym), len_call);
            const branch = try self.b.cond(cond, body, rest);

            var f = branch;
            var k = n;
            while (k > 0) {
                k -= 1;
                f = try self.b.lambda(self.muts[k], f);
            }
            // Start at zero: the index is the last parameter, so the call passes
            // the enclosing mutables as they stand and zero for the counter.
            var args: [16]u32 = undefined;
            var j: u32 = 0;
            while (j + 1 < n) : (j += 1) args[j] = try self.b.variable(self.muts[j]);
            args[n - 1] = try self.b.lit(0);
            var call = try self.b.variable(loop_sym);
            j = 0;
            while (j < n) : (j += 1) call = try self.b.apply(call, args[j .. j + 1]);
            const looped = try self.b.letRec(loop_sym, f, call);
            return self.b.let(s_sym, seq, looped);
        }
        if (self.tok.kind == .kw_while) {
            // `while c { body } rest` becomes a recursive function of the
            // mutable locals:
            //
            //   let rec L = \x1..\xn. if c { body; L(x1..xn) } else { rest }
            //   in L(x1..xn)
            //
            // Assignments inside the body are `let` rebindings that shadow the
            // parameters, so the tail call reads the updated values without any
            // renaming: shadowing is the state update.
            try self.bump();
            const saved_r = self.allow_record;
            self.allow_record = false;
            const cond = try self.expression();
            self.allow_record = saved_r;
            try self.expect(.lbrace);

            const loop_sym = LOOP_SYM_BASE + self.nloops;
            self.nloops += 1;
            const n = self.nmuts;

            const body = try self.loopBody(loop_sym, n);
            try self.expect(.rbrace);
            const rest = try self.program();
            const branch = try self.b.cond(cond, body, rest);

            var f = branch;
            var k = n;
            while (k > 0) {
                k -= 1;
                f = try self.b.lambda(self.muts[k], f);
            }
            const call = try self.loopCall(loop_sym, n);
            return self.b.letRec(loop_sym, f, call);
        }
        if (self.tok.kind == .kw_let) {
            try self.bump();
            var is_mut = false;
            if (self.tok.kind == .kw_mut) {
                try self.bump();
                is_mut = true;
            }
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.assign);
            const value = try self.expression();
            try self.expect(.semi);
            if (is_mut) {
                if (self.nmuts == self.muts.len) return Error.TooManyParams;
                self.muts[self.nmuts] = name;
                self.nmuts += 1;
            }
            const body = try self.program();
            return self.b.let(name, value, body);
        }
        return self.expression();
    }

    /// Precedence climbing. Comparison binds loosest, then additive, then
    /// multiplicative, which is the ordering the surface grammar fixes.
    pub fn expression(self: *Parser) Error!u32 {
        return self.comparison();
    }

    fn comparison(self: *Parser) Error!u32 {
        var lhs = try self.additive();
        while (self.tok.kind == .lt) {
            try self.bump();
            const rhs = try self.additive();
            lhs = try self.b.arith(OP_LT, lhs, rhs);
        }
        return lhs;
    }

    fn additive(self: *Parser) Error!u32 {
        var lhs = try self.multiplicative();
        while (self.tok.kind == .plus or self.tok.kind == .minus) {
            const op: u32 = if (self.tok.kind == .plus) OP_ADD else OP_SUB;
            try self.bump();
            const rhs = try self.multiplicative();
            lhs = try self.b.arith(op, lhs, rhs);
        }
        return lhs;
    }

    fn multiplicative(self: *Parser) Error!u32 {
        var lhs = try self.postfix();
        while (self.tok.kind == .star) {
            try self.bump();
            const rhs = try self.postfix();
            lhs = try self.b.arith(OP_MUL, lhs, rhs);
        }
        return lhs;
    }

    /// Call syntax, applied left to right so `f(a)(b)` and `f(a, b)` produce the
    /// same Core, which is what makes partial application fall out rather than
    /// being a separate feature.
    /// Parse `fn name(params) { body }` and return the name and the curried
    /// lambda. Shared by top-level declarations and module bodies so the two
    /// cannot drift apart.
    fn fnDecl(self: *Parser) Error!struct { name: u32, value: u32 } {
        try self.expect(.kw_fn);
        if (self.tok.kind != .ident) return Error.UnexpectedToken;
        const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
        try self.bump();
        try self.expect(.lparen);
        var params: [8]u32 = undefined;
        var np: usize = 0;
        while (self.tok.kind != .rparen) {
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            if (np == params.len) return Error.TooManyParams;
            params[np] = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            np += 1;
            try self.bump();
            if (self.tok.kind == .comma) try self.bump();
        }
        try self.expect(.rparen);
        try self.expect(.lbrace);
        const body = try self.program();
        try self.expect(.rbrace);
        var f = body;
        if (np == 0) {
            // A nullary function still binds something, from the synthetic
            // range so it cannot shadow a source name.
            f = try self.b.lambda(SYN_SYM_BASE + 0xFFFF, f);
        }
        var k = np;
        while (k > 0) {
            k -= 1;
            f = try self.b.lambda(params[k], f);
        }
        return .{ .name = name, .value = f };
    }

    /// A module body: items, then a record of them. A module IS a record, so a
    /// path is a projection and nothing new is needed to represent one.
    fn moduleBody(self: *Parser, names_out: []u32, srcs: [][]const u8, n: *usize) Error!u32 {
        try self.skipAttributes();
        var is_pub = false;
        if (self.tok.kind == .kw_pub) {
            try self.bump();
            is_pub = true;
        }
        if (self.tok.kind == .kw_mod) {
            // A nested module is an item like any other: it binds a record, and
            // it is exported only if it is public.
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const sub_src = self.lx.src[self.tok.start..self.tok.end];
            const sub_name = try self.names.intern(sub_src);
            try self.bump();
            try self.expect(.lbrace);
            var sub_syms: [32]u32 = undefined;
            var sub_srcs: [32][]const u8 = undefined;
            var sub_n: usize = 0;
            const value = try self.moduleBody(&sub_syms, &sub_srcs, &sub_n);
            try self.expect(.rbrace);
            if (is_pub) {
                if (n.* == names_out.len) return Error.TooManyParams;
                names_out[n.*] = sub_name;
                srcs[n.*] = sub_src;
                n.* += 1;
            }
            return self.b.let(sub_name, value, try self.moduleBody(names_out, srcs, n));
        }
        if (self.tok.kind == .rbrace) {
            var kv: [32]u32 = undefined;
            var i: usize = 0;
            while (i < n.*) : (i += 1) {
                if (i * 2 + 2 > kv.len) return Error.TooManyParams;
                kv[i * 2] = try self.b.str(srcs[i]);
                kv[i * 2 + 1] = try self.b.variable(names_out[i]);
            }
            return self.b.record(kv[0 .. n.* * 2]);
        }
        if (self.tok.kind == .kw_fn) {
            const start_tok = self.tok;
            const d = try self.fnDecl();
            // A private item is still bound inside the module, so its siblings
            // can call it; it simply does not become a field. Reaching it from
            // outside then fails as a missing field, which is the right answer
            // arrived at without a second mechanism.
            if (is_pub) {
                if (n.* == names_out.len) return Error.TooManyParams;
                names_out[n.*] = d.name;
                var lx2 = Lexer{ .src = self.lx.src, .i = start_tok.end };
                const nt = try lx2.next();
                srcs[n.*] = self.lx.src[nt.start..nt.end];
                n.* += 1;
            }
            return self.b.letRec(d.name, d.value, try self.moduleBody(names_out, srcs, n));
        }
        return Error.UnexpectedToken;
    }

    /// Evaluate a node at compile time. Only what a macro body may contain: a
    /// literal, a parameter, arithmetic, and a conditional. Anything else is
    /// refused rather than deferred, because a macro that cannot be evaluated
    /// here has no other stage to fall back to.
    fn constEval(self: *const Parser, node: u32, env: []const u32, vals: []const i64) Error!i64 {
        const base = @as(usize, node) * NODE_WORDS;
        const tag = self.b.nodes[base];
        switch (tag) {
            TAG_LIT => {
                if (self.b.nodes[base + 1] != LIT_INT) return Error.NotConstant;
                const lo = self.b.nodes[base + 2];
                const hi = self.b.nodes[base + 3];
                return @bitCast((@as(u64, hi) << 32) | @as(u64, lo));
            },
            TAG_VAR => {
                const sym = self.b.nodes[base + 1];
                var i: usize = 0;
                while (i < env.len) : (i += 1) {
                    if (env[i] == sym) return vals[i];
                }
                return Error.NotConstant;
            },
            TAG_IF => {
                const c = try self.constEval(self.b.nodes[base + 1], env, vals);
                const branch = if (c != 0) self.b.nodes[base + 2] else self.b.nodes[base + 3];
                return self.constEval(branch, env, vals);
            },
            TAG_RAW => {
                const start = self.b.nodes[base + 2];
                const len = self.b.nodes[base + 3];
                if (len != 3) return Error.NotConstant;
                const op_node = self.b.pool[start];
                const op = try self.constEval(op_node, env, vals);
                const a = try self.constEval(self.b.pool[start + 1], env, vals);
                const bb = try self.constEval(self.b.pool[start + 2], env, vals);
                return switch (@as(u32, @intCast(op))) {
                    OP_ADD => a + bb,
                    OP_SUB => a - bb,
                    OP_MUL => a * bb,
                    OP_LT => if (a < bb) 1 else 0,
                    else => Error.NotConstant,
                };
            },
            else => return Error.NotConstant,
        }
    }

    fn macroIndex(self: *const Parser, sym: u32) ?u32 {
        var i: u32 = 0;
        while (i < self.nmacros) : (i += 1) {
            if (self.macros[i].sym == sym) return i;
        }
        return null;
    }

    fn isMut(self: *const Parser, sym: u32) bool {
        var i: u32 = 0;
        while (i < self.nmuts) : (i += 1) {
            if (self.muts[i] == sym) return true;
        }
        return false;
    }

    /// The recursive call that closes a loop body, passing the mutable locals as
    /// they currently stand. Shadowing means "as they currently stand" is
    /// whatever the body's assignments rebound them to.
    fn loopCall(self: *Parser, loop_sym: u32, n: u32) Error!u32 {
        var args: [16]u32 = undefined;
        var j: u32 = 0;
        while (j < n) : (j += 1) args[j] = try self.b.variable(self.muts[j]);
        var call = try self.b.variable(loop_sym);
        j = 0;
        while (j < n) : (j += 1) call = try self.b.apply(call, args[j .. j + 1]);
        return call;
    }

    /// A `for` body: bind the element, then the statements, then advance the
    /// index and recurse. The increment is emitted here rather than parsed, so
    /// a body that never mentions the index still advances.
    fn forBody(self: *Parser, loop_sym: u32, n: u32, i_sym: u32, s_sym: u32, elem: u32) Error!u32 {
        const at_call = try self.b.variadic(OP_AT, &[_]u32{
            try self.b.variable(s_sym),
            try self.b.variable(i_sym),
        });
        const inner = try self.forStatements(loop_sym, n, i_sym);
        return self.b.let(elem, at_call, inner);
    }

    fn forStatements(self: *Parser, loop_sym: u32, n: u32, i_sym: u32) Error!u32 {
        if (self.tok.kind == .rbrace) {
            const bumped = try self.b.arith(OP_ADD, try self.b.variable(i_sym), try self.b.lit(1));
            return self.b.let(i_sym, bumped, try self.loopCall(loop_sym, n));
        }
        if (self.tok.kind == .kw_let) {
            try self.bump();
            if (self.tok.kind == .kw_mut) try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.assign);
            const value = try self.expression();
            try self.expect(.semi);
            return self.b.let(name, value, try self.forStatements(loop_sym, n, i_sym));
        }
        if (self.tok.kind == .ident) {
            const save_lx = self.lx;
            const save_tok = self.tok;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            const k = self.tok.kind;
            if (k == .assign or k == .plus_assign or k == .minus_assign) {
                if (!self.isMut(name)) return Error.NotMutable;
                try self.bump();
                const rhs = try self.expression();
                try self.expect(.semi);
                const value = switch (k) {
                    .plus_assign => try self.b.arith(OP_ADD, try self.b.variable(name), rhs),
                    .minus_assign => try self.b.arith(OP_SUB, try self.b.variable(name), rhs),
                    else => rhs,
                };
                return self.b.let(name, value, try self.forStatements(loop_sym, n, i_sym));
            }
            self.lx = save_lx;
            self.tok = save_tok;
        }
        const e = try self.expression();
        try self.expect(.semi);
        return self.b.let(try self.names.intern("_"), e, try self.forStatements(loop_sym, n, i_sym));
    }

    /// A loop body: statements, then the recursive call. An assignment is a
    /// `let` that shadows, which is the census's resolution of `mut`: a local
    /// reassignment is rebinding, and only a genuine place would need an effect.
    fn loopBody(self: *Parser, loop_sym: u32, n: u32) Error!u32 {
        if (self.tok.kind == .rbrace) return self.loopCall(loop_sym, n);
        if (self.tok.kind == .kw_let) {
            try self.bump();
            if (self.tok.kind == .kw_mut) try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.assign);
            const value = try self.expression();
            try self.expect(.semi);
            return self.b.let(name, value, try self.loopBody(loop_sym, n));
        }
        if (self.tok.kind == .ident) {
            // Two-token lookahead by save and restore, because an identifier at
            // statement position may open an assignment or an expression.
            const save_lx = self.lx;
            const save_tok = self.tok;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            const k = self.tok.kind;
            if (k == .assign or k == .plus_assign or k == .minus_assign) {
                if (!self.isMut(name)) return Error.NotMutable;
                try self.bump();
                const rhs = try self.expression();
                try self.expect(.semi);
                const value = switch (k) {
                    .plus_assign => try self.b.arith(OP_ADD, try self.b.variable(name), rhs),
                    .minus_assign => try self.b.arith(OP_SUB, try self.b.variable(name), rhs),
                    else => rhs,
                };
                return self.b.let(name, value, try self.loopBody(loop_sym, n));
            }
            self.lx = save_lx;
            self.tok = save_tok;
        }
        // A bare expression statement still binds, so its effects keep their
        // place in the order even though its value is discarded.
        const e = try self.expression();
        try self.expect(.semi);
        return self.b.let(try self.names.intern("_"), e, try self.loopBody(loop_sym, n));
    }

    /// One pattern. Deliberately small: a literal, a binding, a wildcard, or a
    /// record of sub-patterns. Alternatives, ranges, and rest are the obvious
    /// next ones and are not here.
    /// A pattern, possibly a chain of alternatives. Alternatives may not bind,
    /// so both sides agree on the empty set of bindings by construction rather
    /// than by a check that their binding sets match.
    fn pattern(self: *Parser) Error!u32 {
        var p = try self.patternPrimary();
        while (self.tok.kind == .pipe) {
            try self.bump();
            const q = try self.patternPrimary();
            if (self.binds(p) or self.binds(q)) return Error.BindingInAlternative;
            p = try self.b.patOr(p, q);
        }
        return p;
    }

    /// Whether a pattern introduces any binding.
    fn binds(self: *const Parser, pat: u32) bool {
        const tag = self.b.nodes[@as(usize, pat) * NODE_WORDS];
        if (tag == PAT_BIND) return true;
        if (tag == PAT_OR) {
            return self.binds(self.b.nodes[@as(usize, pat) * NODE_WORDS + 1]) or
                self.binds(self.b.nodes[@as(usize, pat) * NODE_WORDS + 2]);
        }
        if (tag == PAT_REC) {
            const start = self.b.nodes[@as(usize, pat) * NODE_WORDS + 1];
            const n = self.b.nodes[@as(usize, pat) * NODE_WORDS + 2];
            var k: u32 = 1;
            while (k < n) : (k += 2) {
                if (self.binds(self.b.pool[start + k])) return true;
            }
        }
        return false;
    }

    fn patternPrimary(self: *Parser) Error!u32 {
        switch (self.tok.kind) {
            .underscore => {
                try self.bump();
                return self.b.patWild();
            },
            .int, .minus => {
                var v: i64 = 0;
                if (self.tok.kind == .minus) {
                    try self.bump();
                    if (self.tok.kind != .int) return Error.UnexpectedToken;
                    v = -self.tok.value;
                } else {
                    v = self.tok.value;
                }
                try self.bump();
                if (self.tok.kind == .dotdot or self.tok.kind == .dotdot_eq) {
                    const inclusive = self.tok.kind == .dotdot_eq;
                    try self.bump();
                    var hi: i64 = 0;
                    if (self.tok.kind == .minus) {
                        try self.bump();
                        if (self.tok.kind != .int) return Error.UnexpectedToken;
                        hi = -self.tok.value;
                    } else {
                        if (self.tok.kind != .int) return Error.UnexpectedToken;
                        hi = self.tok.value;
                    }
                    try self.bump();
                    return self.b.patRange(v, hi, inclusive);
                }
                return self.b.patInt(v);
            },
            .str_lit => {
                const text = self.lx.src[self.tok.start + 1 .. self.tok.end - 1];
                try self.bump();
                return self.b.patStr(text);
            },
            .ident => {
                const sym = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
                try self.bump();
                return self.b.patBind(sym);
            },
            .lbrace => {
                try self.bump();
                var kv: [16]u32 = undefined;
                var nk: usize = 0;
                while (self.tok.kind != .rbrace) {
                    if (self.tok.kind != .ident) return Error.UnexpectedToken;
                    if (nk + 2 > kv.len) return Error.TooManyParams;
                    const key = self.lx.src[self.tok.start..self.tok.end];
                    try self.bump();
                    try self.expect(.colon);
                    kv[nk] = try self.b.str(key);
                    kv[nk + 1] = try self.pattern();
                    nk += 2;
                    if (self.tok.kind == .comma) try self.bump();
                }
                try self.expect(.rbrace);
                return self.b.patRec(kv[0..nk]);
            },
            else => return Error.UnexpectedToken,
        }
    }

    /// Whether a pattern matches everything, which is how exhaustiveness is
    /// approximated here: the last arm must be irrefutable. That is sound and
    /// checkable without a usefulness algorithm, and it refuses some programs a
    /// real exhaustiveness check would accept.
    fn irrefutable(self: *const Parser, pat: u32) bool {
        const tag = self.b.nodes[@as(usize, pat) * NODE_WORDS];
        if (tag == PAT_WILD or tag == PAT_BIND) return true;
        // A record pattern is irrefutable when every sub-pattern is, because a
        // record has exactly the fields it has: there is no other shape it
        // could have taken for the match to fall through.
        if (tag == PAT_REC) {
            const start = self.b.nodes[@as(usize, pat) * NODE_WORDS + 1];
            const n = self.b.nodes[@as(usize, pat) * NODE_WORDS + 2];
            var k: u32 = 1;
            while (k < n) : (k += 2) {
                if (!self.irrefutable(self.b.pool[start + k])) return false;
            }
            return true;
        }
        return false;
    }

    fn postfix(self: *Parser) Error!u32 {
        // A prelude name in call position lowers to its family operation rather
        // than to an application of a binding, because no binding exists for it.
        var builtin: ?u32 = null;
        if (self.tok.kind == .ident) {
            const t = self.lx.src[self.tok.start..self.tok.end];
            if (std.mem.eql(u8, t, "len")) builtin = OP_LEN;
            if (std.mem.eql(u8, t, "at")) builtin = OP_AT;
            if (std.mem.eql(u8, t, "push")) builtin = OP_PUSH;
            if (builtin != null) {
                try self.bump();
                if (self.tok.kind != .lparen) return Error.UnexpectedToken;
            }
        }
        if (builtin == null and self.tok.kind == .ident) {
            const save_lx = self.lx;
            const save_tok = self.tok;
            const sym = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            if (self.tok.kind == .bang) {
                const mi = self.macroIndex(sym) orelse return Error.UnknownMacro;
                try self.bump();
                try self.expect(.lparen);
                var argv: [4]i64 = undefined;
                var na: u8 = 0;
                while (self.tok.kind != .rparen) {
                    if (na == argv.len) return Error.TooManyParams;
                    // Arguments are evaluated at this stage too, so a macro may
                    // take the result of another macro but not a runtime value.
                    const anode = try self.expression();
                    argv[na] = try self.constEval(anode, &.{}, &.{});
                    na += 1;
                    if (self.tok.kind == .comma) try self.bump();
                }
                try self.expect(.rparen);
                const m = self.macros[mi];
                if (na != m.nparams) return Error.WrongMacroArity;
                const v = try self.constEval(m.body, m.params[0..m.nparams], argv[0..na]);
                // The expansion is a constant. Nothing of the macro survives
                // into the residual, which is the whole point of discharging it
                // at this stage.
                return self.b.lit(v);
            }
            self.lx = save_lx;
            self.tok = save_tok;
        }
        var e: u32 = if (builtin != null) 0 else try self.primary();
        while (self.tok.kind == .lparen or self.tok.kind == .dot or self.tok.kind == .colon_colon) {
            if (self.tok.kind == .lparen and builtin != null) {
                try self.bump();
                var args: [4]u32 = undefined;
                var na: usize = 0;
                while (self.tok.kind != .rparen) {
                    if (na == args.len) return Error.TooManyParams;
                    args[na] = try self.expression();
                    na += 1;
                    if (self.tok.kind == .comma) try self.bump();
                }
                try self.expect(.rparen);
                e = try self.b.variadic(builtin.?, args[0..na]);
                builtin = null;
                continue;
            }
            if (self.tok.kind == .colon_colon) {
                try self.bump();
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                const item = self.lx.src[self.tok.start..self.tok.end];
                try self.bump();
                e = try self.b.project(e, item);
                continue;
            }
            if (self.tok.kind == .dot) {
                try self.bump();
                if (self.tok.kind != .ident) return Error.UnexpectedToken;
                const field = self.lx.src[self.tok.start..self.tok.end];
                try self.bump();
                e = try self.b.project(e, field);
                continue;
            }
            try self.bump();
            var args: [8]u32 = undefined;
            var na: usize = 0;
            while (self.tok.kind != .rparen) {
                if (na == args.len) return Error.TooManyParams;
                args[na] = try self.expression();
                na += 1;
                if (self.tok.kind == .comma) try self.bump();
            }
            try self.expect(.rparen);
            if (na == 0) {
                // A call with no arguments passes unit, matching the unit
                // parameter a nullary declaration takes. "No arguments" stays an
                // application, so the Core needs no nullary form.
                args[0] = try self.b.unit();
                na = 1;
            }
            var i: usize = 0;
            while (i < na) : (i += 1) {
                e = try self.b.apply(e, args[i .. i + 1]);
            }
        }
        return e;
    }

    fn primary(self: *Parser) Error!u32 {
        switch (self.tok.kind) {
            .int => {
                const v = self.tok.value;
                try self.bump();
                return self.b.lit(v);
            },
            .minus => {
                // Unary negation, lowered to a subtraction from zero rather
                // than a separate operation: the arithmetic family already has
                // subtraction, and a negate opcode would be a second spelling
                // of it.
                try self.bump();
                const zero = try self.b.lit(0);
                const e = try self.postfix();
                return self.b.arith(OP_SUB, zero, e);
            },
            .str_lit => {
                // Trim the quotes; escapes are not in this subset.
                const text = self.lx.src[self.tok.start + 1 .. self.tok.end - 1];
                try self.bump();
                return self.b.str(text);
            },
            .ident => {
                const sym = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
                try self.bump();
                return self.b.variable(sym);
            },
            .lparen => {
                try self.bump();
                if (self.tok.kind == .rparen) {
                    try self.bump();
                    return self.b.unit();
                }
                const saved = self.allow_record;
                self.allow_record = true;
                const e = try self.expression();
                self.allow_record = saved;
                try self.expect(.rparen);
                return e;
            },
            .lbracket => {
                try self.bump();
                var items: [32]u32 = undefined;
                var ni: usize = 0;
                const saved = self.allow_record;
                self.allow_record = true;
                while (self.tok.kind != .rbracket) {
                    if (ni == items.len) return Error.TooManyParams;
                    items[ni] = try self.expression();
                    ni += 1;
                    if (self.tok.kind == .comma) try self.bump();
                }
                self.allow_record = saved;
                try self.expect(.rbracket);
                return self.b.variadic(OP_MAKE_SEQ, items[0..ni]);
            },
            .lbrace => {
                if (!self.allow_record) return Error.UnexpectedToken;
                try self.bump();
                var kv: [16]u32 = undefined;
                var nk: usize = 0;
                while (self.tok.kind != .rbrace) {
                    if (self.tok.kind != .ident) return Error.UnexpectedToken;
                    if (nk + 2 > kv.len) return Error.TooManyParams;
                    const key = self.lx.src[self.tok.start..self.tok.end];
                    try self.bump();
                    try self.expect(.colon);
                    kv[nk] = try self.b.str(key);
                    kv[nk + 1] = try self.expression();
                    nk += 2;
                    if (self.tok.kind == .comma) try self.bump();
                }
                try self.expect(.rbrace);
                return self.b.record(kv[0..nk]);
            },
            .kw_match => {
                try self.bump();
                const saved_m = self.allow_record;
                self.allow_record = false;
                const scrutinee = try self.expression();
                self.allow_record = saved_m;
                try self.expect(.lbrace);
                var arms: [48]u32 = undefined;
                var na: usize = 0;
                var last_irrefutable = false;
                while (self.tok.kind != .rbrace) {
                    if (na + 3 > arms.len) return Error.TooManyParams;
                    const pat = try self.pattern();
                    var guard: u32 = NO_GUARD;
                    if (self.tok.kind == .kw_if) {
                        // A guard makes the arm refutable however irrefutable
                        // its pattern is, because the condition may be false.
                        try self.bump();
                        const saved_g = self.allow_record;
                        self.allow_record = false;
                        guard = try self.expression();
                        self.allow_record = saved_g;
                    }
                    last_irrefutable = guard == NO_GUARD and self.irrefutable(pat);
                    try self.expect(.fat_arrow);
                    arms[na] = pat;
                    arms[na + 1] = guard;
                    arms[na + 2] = try self.expression();
                    na += 3;
                    if (self.tok.kind == .comma) try self.bump();
                }
                try self.expect(.rbrace);
                if (na == 0 or !last_irrefutable) return Error.NonExhaustive;
                return self.b.match_(scrutinee, arms[0..na]);
            },
            .kw_if => {
                try self.bump();
                const saved = self.allow_record;
                self.allow_record = false;
                const c = try self.expression();
                self.allow_record = saved;
                try self.expect(.lbrace);
                const t = try self.program();
                try self.expect(.rbrace);
                try self.expect(.kw_else);
                try self.expect(.lbrace);
                const e = try self.program();
                try self.expect(.rbrace);
                return self.b.cond(c, t, e);
            },
            else => return Error.UnexpectedToken,
        }
    }
};

/// Assemble the wire image the runtime decodes: header, nodes, pool, and the
/// empty blob and clause sections this subset does not use yet.
pub fn writeImage(b: *const Builder, root: u32, out: []u8) Error!usize {
    const need = HEADER_WORDS * WORD + @as(usize, b.n) * NODE_WORDS * WORD + @as(usize, b.p) * WORD + @as(usize, b.bl);
    if (out.len < need) return Error.OutOfNodes;
    var w: usize = 0;
    const put = struct {
        fn f(buf: []u8, at: usize, v: u32) void {
            std.mem.writeInt(u32, buf[at..][0..4], v, .little);
        }
    }.f;
    put(out, 0, MAGIC);
    put(out, 1 * WORD, 2); // version
    put(out, 2 * WORD, 0); // tier: arena
    put(out, 3 * WORD, b.n);
    put(out, 4 * WORD, b.p);
    put(out, 5 * WORD, b.bl);
    put(out, 6 * WORD, root);
    put(out, 7 * WORD, 0); // clause_count
    w = HEADER_WORDS * WORD;
    var i: usize = 0;
    while (i < @as(usize, b.n) * NODE_WORDS) : (i += 1) {
        put(out, w, b.nodes[i]);
        w += WORD;
    }
    i = 0;
    while (i < b.p) : (i += 1) {
        put(out, w, b.pool[i]);
        w += WORD;
    }
    if (b.bl > 0) {
        @memcpy(out[w .. w + b.bl], b.blob[0..b.bl]);
        w += b.bl;
    }
    return w;
}

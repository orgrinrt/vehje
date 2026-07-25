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
pub const TAG_RAW: u32 = 10;

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
pub const TyName = enum { self_ty, int, boolean, str };

/// A trait: one method, its parameter types, and its result type. One method per
/// trait keeps the constraint machinery honest without a method table, and the
/// generalisation to several is mechanical.
pub const TraitDecl = struct {
    name: []const u8,
    method: []const u8,
    /// The method name's interned sym, so the checker can bind it without
    /// re-interning and the parser stays the only place names are interned.
    method_sym: u32,
    params: [4]TyName,
    nparams: u8,
    ret: TyName,
};

/// An implementation: which trait, for which type, and the binder its method
/// body was bound under. Coherence is one impl per trait-and-type pair.
pub const ImplDecl = struct { trait_idx: u32, for_ty: TyName, method_sym: u32 };

/// Impl-method binders start here. Source names are interner indices counting
/// from zero, so the two ranges cannot meet.
pub const IMPL_SYM_BASE: u32 = 0x4000_0000;

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
};

// ---------------------------------------------------------------- lexer

const Kind = enum { int, str_lit, ident, dot, colon, lbracket, rbracket, arrow, kw_trait, kw_impl, kw_for, kw_let, kw_fn, kw_if, kw_else, plus, minus, star, lt, assign, semi, comma, lparen, rparen, lbrace, rbrace, eof };

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
    traits: []TraitDecl,
    ntraits: u32 = 0,
    impls: []ImplDecl,
    nimpls: u32 = 0,
    /// `{` is ambiguous: it opens a block after an `if` condition and a record
    /// literal in ordinary expression position. Cleared while parsing a
    /// condition, which is the same resolution Rust uses.
    allow_record: bool = true,

    pub fn init(src: []const u8, b: *Builder, names: *Names, traits: []TraitDecl, impls: []ImplDecl) Error!Parser {
        var lx = Lexer{ .src = src };
        const first = try lx.next();
        return .{ .lx = lx, .tok = first, .b = b, .names = names, .traits = traits, .impls = impls };
    }

    fn tyName(self: *Parser) Error!TyName {
        if (self.tok.kind != .ident) return Error.UnexpectedToken;
        const t = self.lx.src[self.tok.start..self.tok.end];
        try self.bump();
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
        return IMPL_SYM_BASE + impl_index;
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
    pub fn program(self: *Parser) Error!u32 {
        if (self.tok.kind == .kw_trait) {
            // A trait declares a method's shape. It binds nothing and erases
            // entirely; only the checker ever sees it.
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const tname = self.lx.src[self.tok.start..self.tok.end];
            try self.bump();
            try self.expect(.lbrace);
            try self.expect(.kw_fn);
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const mname = self.lx.src[self.tok.start..self.tok.end];
            try self.bump();
            try self.expect(.lparen);
            var ps: [4]TyName = undefined;
            var np: u8 = 0;
            while (self.tok.kind != .rparen) {
                if (np == ps.len) return Error.TooManyParams;
                ps[np] = try self.tyName();
                np += 1;
                if (self.tok.kind == .comma) try self.bump();
            }
            try self.expect(.rparen);
            try self.expect(.arrow);
            const ret = try self.tyName();
            try self.expect(.rbrace);
            if (self.ntraits == self.traits.len) return Error.TooManyTraits;
            self.traits[self.ntraits] = .{
                .name = tname,
                .method = mname,
                .method_sym = try self.names.intern(mname),
                .params = ps,
                .nparams = np,
                .ret = ret,
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
            try self.expect(.kw_fn);
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            try self.bump();
            try self.expect(.lparen);
            var params: [4]u32 = undefined;
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
            try self.expect(.rbrace);
            var f = body;
            var k = np;
            while (k > 0) {
                k -= 1;
                f = try self.b.lambda(params[k], f);
            }
            if (self.nimpls == self.impls.len) return Error.TooManyTraits;
            const sym = self.implSym(self.nimpls);
            self.impls[self.nimpls] = .{ .trait_idx = ti, .for_ty = for_ty, .method_sym = sym };
            self.nimpls += 1;
            const rest = try self.program();
            return self.b.letRec(sym, f, rest);
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
        if (self.tok.kind == .kw_let) {
            try self.bump();
            if (self.tok.kind != .ident) return Error.UnexpectedToken;
            const name = try self.names.intern(self.lx.src[self.tok.start..self.tok.end]);
            try self.bump();
            try self.expect(.assign);
            const value = try self.expression();
            try self.expect(.semi);
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
        var e: u32 = if (builtin != null) 0 else try self.primary();
        while (self.tok.kind == .lparen or self.tok.kind == .dot) {
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
            if (na == 0) return Error.Unsupported;
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

// Carrier-integrated Zig interpreter: consumes the identical REC24 wire bytes
// the Rust carrier emits, runs the full 17-opcode semantics, and folds the same
// checksum, so it cross-validates byte-exact against the Rust interpreters. Two
// dispatch shapes: a switch (LLVM jump table) and a guaranteed-tail-call
// (`@call(.always_tail)`) token-threaded loop. The tail shape is the one Rust
// needed a nightly feature (`become`) to express; Zig has it stable, and the
// shipped vehje runtime is Zig, so this is the load-bearing cross-language cell.
//
// REC24 layout (from carrier ir.rs): 16-byte wire header
// [version, node_count, const_count, pool_count] each u32 LE; then const_count
// u64 LE consts; then node_count 24-byte records. A record is
// [op:u8, arity:u8, flags:u16, shape_id:u32] (8-byte header) then three inline
// u32 operand slots (arity <= 3 always inline at REC24, so no pool read). CONST's
// operand 0 is a const-pool index; every other op's operands are earlier node
// indices. Children-before-parents, so a single forward pass fills results.
const std = @import("std");

const HEADER: usize = 16;
const STRIDE: usize = 24;
const REC_HEADER: usize = 8;

inline fn rdU32(b: [*]const u8, off: usize) u32 {
    return std.mem.readInt(u32, b[off..][0..4], .little);
}
inline fn rdU64(b: [*]const u8, off: usize) u64 {
    return std.mem.readInt(u64, b[off..][0..8], .little);
}

inline fn nodesStart(b: [*]const u8) usize {
    const const_count = rdU32(b, 8);
    return HEADER + @as(usize, const_count) * 8;
}
inline fn opAt(b: [*]const u8, ns: usize, i: usize) u8 {
    return b[ns + i * STRIDE];
}
inline fn operand(b: [*]const u8, ns: usize, i: usize, k: usize) usize {
    // REC24: three inline operands, so operand k is always in-record.
    return rdU32(b, ns + i * STRIDE + REC_HEADER + k * 4);
}
inline fn constAt(b: [*]const u8, idx: usize) u64 {
    return rdU64(b, HEADER + idx * 8);
}
inline fn sh(v: u64) u6 {
    return @intCast(v & 63); // matches Rust wrapping_shl/shr masking for u64
}

// Opcodes (must match carrier ir::op).
const CONST: u8 = 0;
const ADD: u8 = 1;
const SUB: u8 = 2;
const MUL: u8 = 3;
const AND: u8 = 4;
const OR: u8 = 5;
const XOR: u8 = 6;
const SHL: u8 = 7;
const SHR: u8 = 8;
const MIN: u8 = 9;
const MAX: u8 = 10;
const EQ: u8 = 11;
const LT: u8 = 12;
const SELECT: u8 = 13;
const NEG: u8 = 14;
const NOT: u8 = 15;
const INPUT: u8 = 16;

fn checksum(results: [*]const u64, n: usize) u64 {
    var h: u64 = 0;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        h = std.math.rotl(u64, h, 7) ^ results[i];
    }
    return h;
}

// Switch dispatch: one loop, a match per node. LLVM lowers the 17-arm switch to
// a jump table.
pub fn interpSwitch(bytes: [*]const u8, results: [*]u64, seed: u64) u64 {
    const n: usize = rdU32(bytes, 4);
    const ns = nodesStart(bytes);
    var i: usize = 0;
    while (i < n) : (i += 1) {
        const op = opAt(bytes, ns, i);
        const v: u64 = switch (op) {
            CONST => constAt(bytes, operand(bytes, ns, i, 0)),
            INPUT => seed,
            ADD => results[operand(bytes, ns, i, 0)] +% results[operand(bytes, ns, i, 1)],
            SUB => results[operand(bytes, ns, i, 0)] -% results[operand(bytes, ns, i, 1)],
            MUL => results[operand(bytes, ns, i, 0)] *% results[operand(bytes, ns, i, 1)],
            AND => results[operand(bytes, ns, i, 0)] & results[operand(bytes, ns, i, 1)],
            OR => results[operand(bytes, ns, i, 0)] | results[operand(bytes, ns, i, 1)],
            XOR => results[operand(bytes, ns, i, 0)] ^ results[operand(bytes, ns, i, 1)],
            SHL => results[operand(bytes, ns, i, 0)] << sh(results[operand(bytes, ns, i, 1)]),
            SHR => results[operand(bytes, ns, i, 0)] >> sh(results[operand(bytes, ns, i, 1)]),
            MIN => @min(results[operand(bytes, ns, i, 0)], results[operand(bytes, ns, i, 1)]),
            MAX => @max(results[operand(bytes, ns, i, 0)], results[operand(bytes, ns, i, 1)]),
            EQ => @intFromBool(results[operand(bytes, ns, i, 0)] == results[operand(bytes, ns, i, 1)]),
            LT => @intFromBool(results[operand(bytes, ns, i, 0)] < results[operand(bytes, ns, i, 1)]),
            SELECT => if (results[operand(bytes, ns, i, 0)] != 0)
                results[operand(bytes, ns, i, 1)]
            else
                results[operand(bytes, ns, i, 2)],
            NEG => 0 -% results[operand(bytes, ns, i, 0)],
            NOT => ~results[operand(bytes, ns, i, 0)],
            else => 0,
        };
        results[i] = v;
    }
    return checksum(results, n);
}

// Tail-threaded (token-threaded) dispatch: one handler per opcode, each does its
// node's work then `@call(.always_tail)` back into `step`, which reads the next
// node's opcode and tail-calls its handler. Control never returns to a central
// loop; the guaranteed tail call is the dispatch mechanism.
const St = struct {
    bytes: [*]const u8,
    ns: usize,
    results: [*]u64,
    n: usize,
    i: usize,
    seed: u64,
};

const Handler = *const fn (*St) void;

inline fn op0(st: *St) usize {
    return operand(st.bytes, st.ns, st.i, 0);
}
inline fn op1(st: *St) usize {
    return operand(st.bytes, st.ns, st.i, 1);
}
inline fn op2(st: *St) usize {
    return operand(st.bytes, st.ns, st.i, 2);
}
inline fn advance(st: *St, v: u64) void {
    st.results[st.i] = v;
    st.i += 1;
    return @call(.always_tail, step, .{st});
}

fn hConst(st: *St) void {
    return advance(st, constAt(st.bytes, op0(st)));
}
fn hInput(st: *St) void {
    return advance(st, st.seed);
}
fn hAdd(st: *St) void {
    return advance(st, st.results[op0(st)] +% st.results[op1(st)]);
}
fn hSub(st: *St) void {
    return advance(st, st.results[op0(st)] -% st.results[op1(st)]);
}
fn hMul(st: *St) void {
    return advance(st, st.results[op0(st)] *% st.results[op1(st)]);
}
fn hAnd(st: *St) void {
    return advance(st, st.results[op0(st)] & st.results[op1(st)]);
}
fn hOr(st: *St) void {
    return advance(st, st.results[op0(st)] | st.results[op1(st)]);
}
fn hXor(st: *St) void {
    return advance(st, st.results[op0(st)] ^ st.results[op1(st)]);
}
fn hShl(st: *St) void {
    return advance(st, st.results[op0(st)] << sh(st.results[op1(st)]));
}
fn hShr(st: *St) void {
    return advance(st, st.results[op0(st)] >> sh(st.results[op1(st)]));
}
fn hMin(st: *St) void {
    return advance(st, @min(st.results[op0(st)], st.results[op1(st)]));
}
fn hMax(st: *St) void {
    return advance(st, @max(st.results[op0(st)], st.results[op1(st)]));
}
fn hEq(st: *St) void {
    return advance(st, @intFromBool(st.results[op0(st)] == st.results[op1(st)]));
}
fn hLt(st: *St) void {
    return advance(st, @intFromBool(st.results[op0(st)] < st.results[op1(st)]));
}
fn hSelect(st: *St) void {
    const v = if (st.results[op0(st)] != 0) st.results[op1(st)] else st.results[op2(st)];
    return advance(st, v);
}
fn hNeg(st: *St) void {
    return advance(st, 0 -% st.results[op0(st)]);
}
fn hNot(st: *St) void {
    return advance(st, ~st.results[op0(st)]);
}

const TABLE = [17]Handler{
    hConst, hAdd, hSub, hMul, hAnd, hOr, hXor, hShl, hShr,
    hMin,   hMax, hEq,  hLt,  hSelect, hNeg, hNot, hInput,
};

fn step(st: *St) void {
    if (st.i >= st.n) return;
    const op = opAt(st.bytes, st.ns, st.i);
    return @call(.always_tail, TABLE[op], .{st});
}

pub fn interpTail(bytes: [*]const u8, results: [*]u64, seed: u64) u64 {
    const n: usize = rdU32(bytes, 4);
    var st = St{
        .bytes = bytes,
        .ns = nodesStart(bytes),
        .results = results,
        .n = n,
        .i = 0,
        .seed = seed,
    };
    step(&st);
    return checksum(results, n);
}

// C ABI exports (for the eventual harness cdylib variant and the crossval CLI).
export fn carrier_zig_switch(bytes: [*]const u8, results: [*]u64, seed: u64) callconv(.c) u64 {
    return interpSwitch(bytes, results, seed);
}
export fn carrier_zig_tail(bytes: [*]const u8, results: [*]u64, seed: u64) callconv(.c) u64 {
    return interpTail(bytes, results, seed);
}

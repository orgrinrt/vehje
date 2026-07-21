// Shared Zig interpreter for the dispatch bench: switch vs tail-threaded
// (@call(.always_tail)), the Deegen / LuaJIT-remake question that a Rust-only
// carrier cannot ask because Rust has no guaranteed tail call. Both modes
// interpret the identical Zig-generated opcode stream and fold the identical
// rolling hash, so the two variants cross-validate byte-exact through the
// harness; any measured gap is the dispatch shape and nothing else.
//
// The variant is a first-class harness cdylib: it exports the same plain C ABI
// (`bench_entry` / `bench_name` / `bench_abi_hash`) the Rust `#[bench_variant]`
// macro emits, times with the same hardware counter (CNTVCT_EL0), and returns
// the same repr-C FfiBenchCall.
const std = @import("std");

pub const FfiBenchCall = extern struct { run_ticks: u64 };

// FNV-1a over the FfiBenchCall layout, byte-for-byte identical to
// mockspace_bench_core::abi_hash so the harness accepts the dylib on load.
pub fn abiHash() u64 {
    var h: u64 = 0xcbf29ce484222325;
    h ^= 8; // size_of(FfiBenchCall)
    h = h *% 0x100000001b3;
    h ^= 1; // field count
    h = h *% 0x100000001b3;
    h ^= 8; // run_ticks size
    h = h *% 0x100000001b3;
    return h;
}

inline fn readCounter() u64 {
    return asm volatile ("mrs %[ret], CNTVCT_EL0"
        : [ret] "=r" (-> u64),
    );
}

const VOCAB: u8 = 8;
const MAXN: usize = 16384;

// program state, generated once per subprocess from a fixed seed so the
// optimizer cannot fold it and the two modes see identical work.
var PROG: [MAXN]u8 = undefined;
var KS: [MAXN]u64 = undefined;
var inited_n: usize = 0;

fn genProgram(n: usize, seed: u64) void {
    var s: u64 = seed | 1;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        PROG[i] = @intCast(s % VOCAB);
        KS[i] = s | 1;
    }
    inited_n = n;
}

inline fn sh(k: u64) u6 {
    return @intCast(k & 63);
}

inline fn applyOp(op: u8, acc: u64, k: u64) u64 {
    return switch (op) {
        0 => acc +% k,
        1 => acc -% k,
        2 => acc *% (k | 1),
        3 => acc ^ k,
        4 => acc | k,
        5 => acc & k,
        6 => acc << sh(k),
        else => std.math.rotl(u64, acc, sh(k)),
    };
}

// switch dispatch: a single jump-table match per node.
fn interpSwitch(n: usize, input: u64) u64 {
    var acc: u64 = input;
    var pc: usize = 0;
    while (pc < n) : (pc += 1) {
        acc = applyOp(PROG[pc], acc, KS[pc]);
    }
    return acc;
}

// tail-threaded dispatch: each opcode handler does its work then tail-calls the
// next handler via a function-pointer table, so control never returns to a
// central loop. This is the shape Rust cannot express and the reason a Zig
// variant exists.
const St = struct { pc: usize, n: usize, acc: u64 };
const Handler = *const fn (*St) void;

fn hAdd(st: *St) void {
    st.acc = st.acc +% KS[st.pc];
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hSub(st: *St) void {
    st.acc = st.acc -% KS[st.pc];
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hMul(st: *St) void {
    st.acc = st.acc *% (KS[st.pc] | 1);
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hXor(st: *St) void {
    st.acc = st.acc ^ KS[st.pc];
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hOr(st: *St) void {
    st.acc = st.acc | KS[st.pc];
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hAnd(st: *St) void {
    st.acc = st.acc & KS[st.pc];
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hShl(st: *St) void {
    st.acc = st.acc << sh(KS[st.pc]);
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}
fn hRot(st: *St) void {
    st.acc = std.math.rotl(u64, st.acc, sh(KS[st.pc]));
    st.pc += 1;
    return @call(.always_tail, next, .{st});
}

const TABLE = [8]Handler{ hAdd, hSub, hMul, hXor, hOr, hAnd, hShl, hRot };

fn next(st: *St) void {
    if (st.pc >= st.n) return;
    return @call(.always_tail, TABLE[PROG[st.pc]], .{st});
}

fn interpTail(n: usize, input: u64) u64 {
    var st = St{ .pc = 0, .n = n, .acc = input };
    next(&st);
    return st.acc;
}

const ITERS: usize = 16;

// The shared entry body, parameterized by dispatch mode at comptime so both
// variants compile from one source. `input`/`output` are the harness FFI
// buffers; the input byte is folded per iteration so nothing hoists.
pub fn runBench(comptime tail: bool, input: [*]const u8, output: [*]u8, n: usize) FfiBenchCall {
    if (inited_n != n) genProgram(n, 0x5eed_d15c_0000_0001 ^ n);
    const start = readCounter();
    var hash: u64 = 0;
    var k: usize = 0;
    while (k < ITERS) : (k += 1) {
        const seed: u64 = @as(u64, input[k % n]) ^ @as(u64, k);
        const r = if (tail) interpTail(n, seed) else interpSwitch(n, seed);
        hash = std.math.rotl(u64, hash, 7) ^ r;
    }
    const end = readCounter();
    // write the 8-byte little-endian checksum for cross-validation.
    var i: usize = 0;
    while (i < 8) : (i += 1) {
        output[i] = @truncate(hash >> @intCast(i * 8));
    }
    return FfiBenchCall{ .run_ticks = end -% start };
}

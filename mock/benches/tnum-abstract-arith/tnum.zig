const std = @import("std");
// The tnum numeric residual: the eBPF-verifier-style TRISTATE-NUMBER abstract domain the design uses to track
// numeric values through the program at COMPILE time (prove bounds, fold constants, eliminate runtime checks).
// A tnum = { value: u64 (known bits), mask: u64 (which bits are UNKNOWN) }. Question: is tnum arithmetic cheap
// enough to run on EVERY numeric op during the check pass? Bench the tnum ops (add/and/or/mul/shl) vs concrete
// arithmetic, sizing the compile-side abstract-interpretation overhead per numeric node.
const Tnum = struct { value: u64, mask: u64 };
inline fn known(v: u64) Tnum { return .{ .value = v, .mask = 0 }; }
// tnum_add (the eBPF algorithm: carry propagation through unknown bits)
inline fn tAdd(a: Tnum, b: Tnum) Tnum {
    const sm = a.mask +% b.mask;
    const sv = a.value +% b.value;
    const sigma = sm +% sv;
    const chi = sigma ^ sv;
    const mu = chi | a.mask | b.mask;
    return .{ .value = sv & ~mu, .mask = mu };
}
inline fn tAnd(a: Tnum, b: Tnum) Tnum {
    const alpha = a.value | a.mask; const beta = b.value | b.mask; const v = a.value & b.value;
    return .{ .value = v, .mask = alpha & beta & ~v };
}
inline fn tOr(a: Tnum, b: Tnum) Tnum {
    const v = a.value | b.value; const mu = a.mask | b.mask;
    return .{ .value = v, .mask = mu & ~v };
}
inline fn tShl(a: Tnum, s: u6) Tnum { return .{ .value = a.value << s, .mask = a.mask << s }; }
// tnum_mul (eBPF long-multiplication over tnums: iterate bits)
inline fn tMul(a: Tnum, b: Tnum) Tnum {
    var acc = Tnum{ .value = 0, .mask = 0 };
    var aa = a; var bb = b; var i: usize = 0;
    while (i < 64) : (i += 1) {
        if ((aa.value & 1) != 0) acc = tAdd(acc, bb)
        else if ((aa.mask & 1) != 0) acc = tAdd(acc, .{ .value = 0, .mask = bb.value | bb.mask });
        aa = .{ .value = aa.value >> 1, .mask = aa.mask >> 1 };
        bb = tShl(bb, 1);
    }
    return acc;
}
inline fn tMulFast(a: Tnum, b: Tnum) Tnum {
    // fast path: if both operands fully known, the product is known (concrete multiply) => no bit loop.
    // this is the multiply-by-constant / constant-folded case, the overwhelming majority in real programs.
    if (a.mask == 0 and b.mask == 0) return .{ .value = a.value *% b.value, .mask = 0 };
    return tMul(a, b); // fall back to the bit loop only for genuine unknown*unknown
}
const N: usize = 20_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }
pub fn main() void {
    const al = std.heap.page_allocator;
    // inputs: mix of fully-known and partially-unknown tnums (models real analysis where some values are known)
    const inp = al.alloc(Tnum, 1024) catch unreachable;
    var s: u64 = 5; for (inp) |*t| { s = s*%6364136223846793005+%1; const unknown = if ((s>>40)%2==0) @as(u64,0) else (s >> 20); t.* = .{ .value = (s ^ 0x1234) & ~unknown, .mask = unknown }; }
    std.debug.print("tnum abstract arithmetic throughput (per numeric-op abstract eval):\n", .{});
    // concrete baseline: plain u64 add
    { var best: u64 = std.math.maxInt(u64); var sink: u64 = 0;
      for (0..5) |_| { const t0=nowNs(); var i: usize = 0; while (i<N):(i+=1){ sink +%= inp[i&1023].value +% inp[(i>>3)&1023].value; } const dt=nowNs()-t0; if(dt<best)best=dt; }
      std.mem.doNotOptimizeAway(sink); std.debug.print("  concrete u64 add (baseline): {d:.2} ns/op\n", .{@as(f64,@floatFromInt(best))/@as(f64,N)}); }
    inline for (.{ "tnum_add", "tnum_and", "tnum_or", "tnum_shl", "tnum_mul (64-bit-loop)" }, 0..) |name, idx| {
        var best: u64 = std.math.maxInt(u64); var sink: u64 = 0;
        for (0..5) |_| { const t0=nowNs(); var i: usize = 0; while (i<N):(i+=1){ const a = inp[i&1023]; const b = inp[(i>>3)&1023];
            const r = switch (idx) { 0 => tAdd(a,b), 1 => tAnd(a,b), 2 => tOr(a,b), 3 => tShl(a, @intCast(i & 63)), else => tMul(a,b) };
            sink +%= r.value ^ r.mask; } const dt=nowNs()-t0; if(dt<best)best=dt; }
        std.mem.doNotOptimizeAway(sink);
        std.debug.print("  {s}: {d:.2} ns/op\n", .{ name, @as(f64,@floatFromInt(best))/@as(f64,N) });
    }
    // realistic mul mix: build inputs where 80% of operand pairs are both-known (multiply-by-constant case)
    const known_a = al.alloc(Tnum, 1024) catch unreachable;
    var ss: u64 = 77; for (known_a, 0..) |*t, i| { ss = ss*%6364136223846793005+%1; if (i % 5 == 0) t.* = inp[i&1023] else t.* = known(ss >> 40); }
    { var best: u64 = std.math.maxInt(u64); var sink: u64 = 0;
      for (0..5) |_| { const t0=nowNs(); var i: usize = 0; while (i<N):(i+=1){ const r = tMulFast(known_a[i&1023], known_a[(i>>3)&1023]); sink +%= r.value ^ r.mask; } const dt=nowNs()-t0; if(dt<best)best=dt; }
      std.mem.doNotOptimizeAway(sink);
      std.debug.print("  tnum_mul FAST-PATH (80%% both-known, the multiply-by-constant case): {d:.2} ns/op\n", .{@as(f64,@floatFromInt(best))/@as(f64,N)}); }
}

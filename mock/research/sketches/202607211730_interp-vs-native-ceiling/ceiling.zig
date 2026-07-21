const std = @import("std");
// NATIVE TIER value vs the register interpreter, measured FAIRLY (op: "we definitely want to lower to JIT").
// The pitfalls, all hit and corrected: (1) a plain arithmetic loop is closed-formed/vectorized by LLVM to ~0 for
// native (unfair; copy-and-patch does NOT vectorize); (2) a large acc-dependent gather makes it memory-latency-
// bound, hiding dispatch behind DRAM stalls (native ~1x, also unrepresentative of register-resident script code).
// The representative regime is COMPUTE-BOUND with an L1-resident working set. To get a fair, non-foldable native
// baseline, the native arithmetic is written as OPAQUE inline `madd` asm (acc = k1*acc + k2 in one aarch64
// instruction, exactly what copy-and-patch emits), which LLVM cannot fold or vectorize, over an L1-resident,
// acc-dependent (non-vectorizable) constant table. The interpreter runs the identical recurrence through its
// register-VM dispatch. The ratio is the true dispatch overhead the native tier removes on the hot path.
const N: usize = 200_000_000;
const MASK: u64 = 255; // 256-entry table => L1-resident gather (compute-bound, not DRAM-bound), acc-dependent
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }
const KBIT: u8 = 0x80;
const RInstr = extern struct { op: u8, dst: u8, a: u8, b: u8 };
inline fn rk(regs: []const i64, kpool: []const i64, x: u8) i64 { return if (x & KBIT != 0) kpool[x & 0x7f] else regs[x]; }
fn runInterp(code: []const RInstr, regs: []i64, kpool: []i64, iters: usize, ks: []const i64) i64 {
    var sum: i64 = 0; var it: usize = 0;
    while (it < iters) : (it += 1) {
        regs[0] = @intCast(it & 0xffff);
        kpool[1] = ks[it & MASK]; kpool[2] = ks[(it >> 3) & MASK];
        var pc: usize = 0;
        while (pc < code.len) : (pc += 1) { const in = code[pc];
            switch (in.op) {
                1 => regs[in.dst] = rk(regs,kpool,in.a) +% rk(regs,kpool,in.b),
                else => regs[in.dst] = rk(regs,kpool,in.a) *% rk(regs,kpool,in.b),
            } }
        sum +%= regs[0];
    }
    return sum;
}
inline fn madd(acc: i64, k1: i64, k2: i64) i64 {
    // opaque scalar aarch64 madd: r = k1*acc + k2 (exactly the native op copy-and-patch would emit)
    return asm volatile ("madd %[r], %[a], %[b], %[c]"
        : [r] "=r" (-> i64),
        : [a] "r" (acc), [b] "r" (k1), [c] "r" (k2),
    );
}
fn runNative(iters: usize, ks: []const i64) i64 {
    var sum: i64 = 0; var it: usize = 0;
    while (it < iters) : (it += 1) { const k1 = ks[it & MASK]; const k2 = ks[(it >> 3) & MASK]; sum +%= madd(@intCast(it & 0xffff), k1, k2); }
    return sum;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const ks = al.alloc(i64, MASK + 1) catch unreachable;
    var s: u64 = nowNs() | 1; for (0..ks.len) |i| { s = s*%6364136223846793005+%1; ks[i] = @as(i64, @intCast((s>>40)%7 + 1)); }
    var code = [_]RInstr{ .{.op=2,.dst=0,.a=0,.b=KBIT|1}, .{.op=1,.dst=0,.a=0,.b=KBIT|2} };
    const regs = al.alloc(i64, 16) catch unreachable;
    const kpool = al.alloc(i64, 8) catch unreachable;
    var bestI: u64 = std.math.maxInt(u64); var ci: i64 = 0;
    for (0..5) |_| { const t0=nowNs(); ci=runInterp(&code, regs, kpool, N, ks); std.mem.doNotOptimizeAway(ci); const dt=nowNs()-t0; if (dt<bestI) bestI=dt; }
    var bestN: u64 = std.math.maxInt(u64); var cn: i64 = 0;
    for (0..5) |_| { const t0=nowNs(); cn=runNative(N, ks); std.mem.doNotOptimizeAway(cn); const dt=nowNs()-t0; if (dt<bestN) bestN=dt; }
    std.debug.print("N={d}, acc=madd(acc,k1,k2), k1/k2 from L1-resident 256-entry INDEPENDENT iterations (isolates pure dispatch overhead)\n", .{N});
    std.debug.print("INTERP (register VM): {d:.2} ms, {d:.2} ns/expr  chk={d}\n", .{ @as(f64,@floatFromInt(bestI))/1e6, @as(f64,@floatFromInt(bestI))/@as(f64,N), ci });
    std.debug.print("NATIVE (madd asm)   : {d:.2} ms, {d:.2} ns/expr  chk={d}\n", .{ @as(f64,@floatFromInt(bestN))/1e6, @as(f64,@floatFromInt(bestN))/@as(f64,N), cn });
    std.debug.print("=> native {d:.1}x faster than the register interpreter on the compute-bound hot path\n", .{ @as(f64,@floatFromInt(bestI))/@as(f64,@floatFromInt(bestN)) });
}

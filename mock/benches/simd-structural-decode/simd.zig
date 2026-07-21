const std = @import("std");
// BN4: SIMD structural decode throughput (simdjson-style). Validate the arena's
// structural kind bytes (all < NKINDS) at SIMD width vs a scalar loop. The
// alignment-and-stride-predictable format (SK20 finding) is what admits this.
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec)); }
const NKINDS: u8 = 4;
fn scalarValid(k: []const u8) bool { for (k) |x| { if (x >= NKINDS) return false; } return true; }
fn simdValid(k: []const u8) bool {
    const W = 32; const Vu = @Vector(W, u8);
    const lim: Vu = @splat(NKINDS);
    var i: usize = 0;
    while (i + W <= k.len) : (i += W) {
        const v: Vu = k[i..][0..W].*;
        if (@reduce(.Or, v >= lim)) return false;
    }
    while (i < k.len) : (i += 1) { if (k[i] >= NKINDS) return false; }
    return true;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const N: usize = 64 * 1024 * 1024; // 64M structural bytes
    const k = al.alignedAlloc(u8, .@"64", N) catch unreachable;
    var prng = std.Random.DefaultPrng.init(1); prng.random().bytes(k);
    for (k) |*x| x.* &= 3; // all valid (< 4)
    std.debug.print("variant,bytes,ns,gb_per_s\n", .{});
    inline for (.{ .{ "scalar", scalarValid }, .{ "simd", simdValid } }) |v| {
        _ = v[1](k); _ = v[1](k); // warmup
        var best: u64 = std.math.maxInt(u64);
        for (0..7) |_| { const a = nowns(); const ok = v[1](k); std.mem.doNotOptimizeAway(ok); best = @min(best, nowns() - a); }
        const gbps = @as(f64, @floatFromInt(N)) / @as(f64, @floatFromInt(best));
        std.debug.print("{s},{d},{d},{d:.2}\n", .{ v[0], N, best, gbps });
    }
}

const std = @import("std");
// Expansion: optimal SIMD vector width for the structural decode (BN4 used 32B).
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
fn validW(comptime W: usize, k: []const u8) bool {
    const Vu = @Vector(W, u8); const lim: Vu = @splat(4);
    var i: usize = 0; while (i + W <= k.len) : (i += W) { const v: Vu = k[i..][0..W].*; if (@reduce(.Or, v >= lim)) return false; }
    while (i < k.len) : (i += 1) if (k[i] >= 4) return false;
    return true;
}
pub fn main() void {
    const al = std.heap.page_allocator; const N: usize = 128*1024*1024;
    const k = al.alignedAlloc(u8, .@"64", N) catch unreachable; var p = std.Random.DefaultPrng.init(1); p.random().bytes(k); for (k)|*x| x.* &= 3;
    std.debug.print("simd_width_bytes,gb_per_s\n",.{});
    inline for (.{16,32,64}) |W| {
        _ = validW(W, k);
        var best: u64 = std.math.maxInt(u64); for(0..7)|_|{ const a=nowns(); const ok=validW(W,k); std.mem.doNotOptimizeAway(ok); best=@min(best,nowns()-a);}
        std.debug.print("{d},{d:.1}\n",.{W, @as(f64,@floatFromInt(N))/@as(f64,@floatFromInt(best))});
    }
}

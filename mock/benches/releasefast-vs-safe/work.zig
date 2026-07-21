const std = @import("std");
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec)); }
// bounds-check + integer-overflow-check heavy workload (where ReleaseSafe adds checks)
fn work(a: []u32) u64 {
    var acc: u64 = 0; var idx: usize = 0;
    for (0..a.len * 4) |i| { idx = (idx + a[idx % a.len] +% i) % a.len; acc +%= a[idx]; a[idx] +%= 1; }
    return acc;
}
pub fn main() void {
    const al = std.heap.page_allocator; const a = al.alloc(u32, 1 << 20) catch unreachable;
    var prng = std.Random.DefaultPrng.init(2); for (a) |*x| x.* = prng.random().int(u32);
    _ = work(a);
    var best: u64 = std.math.maxInt(u64);
    for (0..7) |_| { const t = nowns(); const r = work(a); std.mem.doNotOptimizeAway(r); best = @min(best, nowns() - t); }
    std.debug.print("{d}\n", .{best});
}

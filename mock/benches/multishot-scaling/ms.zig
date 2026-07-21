const std = @import("std");
// Expansion: bounded multi-shot enumeration throughput (SK2 perf). How fast does the
// no-alloc bounded multi-shot handler enumerate the choice space (resumptions/sec)?
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
fn kont(choices: []const u32) i64 { var a: i64 = 0; for (choices) |c| a = a *% 31 +% c; return a; } // the continuation body
fn collectAll(doms: []const u32, budget: []i64) usize {
    var choices: [8]u32 = @splat(0); var count: usize = 0;
    const total = blk: { var p: usize = 1; for (doms) |d| p *= d; break :blk p; };
    while (count < total) : (count += 1) {
        budget[count] = kont(choices[0..doms.len]);
        var j = doms.len; while (j > 0) { j -= 1; choices[j] += 1; if (choices[j] < doms[j]) break; choices[j] = 0; }
    }
    return count;
}
pub fn main() void {
    const al = std.heap.page_allocator; const budget = al.alloc(i64, 20_000_000) catch unreachable;
    std.debug.print("choice_space,resumptions,ns,ns_per_resumption,M_resume_per_s\n",.{});
    const configs = [_][]const u32{ &.{10,10}, &.{10,10,10}, &.{20,20,20}, &.{16,16,16,16}, &.{8,8,8,8,8,8} };
    for (configs) |doms| {
        _ = collectAll(doms, budget); // warmup
        const t = nowns(); const n = collectAll(doms, budget); const ns = nowns() - t;
        std.mem.doNotOptimizeAway(budget[n-1]);
        std.debug.print("{d},{d},{d},{d:.2},{d:.0}\n",.{ blk:{var p:usize=1;for(doms)|d|p*=d;break :blk p;}, n, ns, @as(f64,@floatFromInt(ns))/@as(f64,@floatFromInt(n)), @as(f64,@floatFromInt(n))/(@as(f64,@floatFromInt(ns))/1000.0) });
    }
}

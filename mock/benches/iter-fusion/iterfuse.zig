const std = @import("std");
// The Iter core form (iteration/comprehension), which templating/config/script consumers hammer: loop over a
// collection, filter, transform, emit. The design fork is ITERATOR FUSION. A materialized pipeline allocates an
// intermediate collection per stage (map/filter each build a new collection); the no-alloc constraint pressures
// hard toward FUSION (compose stages into one loop, no intermediates). Two fusion shapes exist (pull vs push).
// Bench a realistic pipeline `source |> filter(even) |> map(*3+1) |> sum` three ways:
//   A) MATERIALIZED: filter builds a temp array, map builds another, then sum. (needs alloc; the anti-pattern)
//   B) PULL-FUSED (iterator/next): consumer pulls; each next() walks source until a passing element, maps it.
//   C) PUSH-FUSED (source-driven): source loop pushes each element through filter->map->sink inline.
// Measures ns/element + validates fusion removes the intermediates. Also a 3-stage (filter,map,map) variant to
// show fusion scaling with pipeline depth.
const N: usize = 20_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

inline fn keep(x: i64) bool { return (x & 1) == 0; }
inline fn xf(x: i64) i64 {
    // opaque scalar transform (models an interpreted map stage; blocks auto-vectorization on both sides)
    return asm volatile ("madd %[r], %[a], %[b], %[c]" : [r] "=r" (-> i64) : [a] "r" (x), [b] "r" (@as(i64,3)), [c] "r" (@as(i64,1)));
}

fn materialized(src: []const i64, tmp1: []i64, tmp2: []i64) i64 {
    var n1: usize = 0; for (src) |x| { if (keep(x)) { tmp1[n1] = x; n1 += 1; } }      // filter -> tmp1
    for (0..n1) |i| { tmp2[i] = xf(tmp1[i]); }                                          // map -> tmp2
    var s: i64 = 0; for (0..n1) |i| { s +%= tmp2[i]; }                                  // sum
    return s;
}
fn pullFused(src: []const i64) i64 {
    var s: i64 = 0; var i: usize = 0;
    while (i < src.len) : (i += 1) { const x = src[i]; if (!keep(x)) continue; s +%= xf(x); } // pull: skip+map inline
    return s;
}
fn pushFused(src: []const i64) i64 {
    var s: i64 = 0; for (src) |x| { if (keep(x)) { s +%= xf(x); } }                     // push: source drives inline
    return s;
}
// 3-stage: filter -> map -> map -> sum
inline fn xf2(x: i64) i64 {
    return asm volatile ("eor %[r], %[a], %[m]" : [r] "=r" (-> i64) : [a] "r" (x), [m] "r" (@as(i64,0x5a5a)));
}
fn materialized3(src: []const i64, t1: []i64, t2: []i64, t3: []i64) i64 {
    var n: usize = 0; for (src) |x| { if (keep(x)) { t1[n]=x; n+=1; } }
    for (0..n) |i| t2[i] = xf(t1[i]);
    for (0..n) |i| t3[i] = xf2(t2[i]);
    var s: i64 = 0; for (0..n) |i| s +%= t3[i]; return s;
}
fn pushFused3(src: []const i64) i64 { var s: i64 = 0; for (src) |x| { if (keep(x)) s +%= xf2(xf(x)); } return s; }

pub fn main() void {
    const al = std.heap.page_allocator;
    const src = al.alloc(i64, N) catch unreachable;
    var s: u64 = 3; for (src) |*x| { s = s*%6364136223846793005+%1; x.* = @intCast(s >> 40); }
    const t1 = al.alloc(i64, N) catch unreachable; const t2 = al.alloc(i64, N) catch unreachable; const t3 = al.alloc(i64, N) catch unreachable;
    std.debug.print("pipeline: source |> filter(even) |> map(*3+1) |> sum  ({d} elements)\n", .{N});
    inline for (.{ "A materialized (2 temps)", "B pull-fused", "C push-fused" }, 0..) |name, idx| {
        var best: u64 = std.math.maxInt(u64); var chk: i64 = 0;
        for (0..5) |_| { const t0 = nowNs(); chk = switch (idx) { 0 => materialized(src, t1, t2), 1 => pullFused(src), else => pushFused(src) }; const dt = nowNs()-t0; if (dt < best) best = dt; }
        std.mem.doNotOptimizeAway(chk);
        std.debug.print("  {s}: {d:.2} ns/elem  ({d:.0} M-elem/s)  chk={d}\n", .{ name, @as(f64,@floatFromInt(best))/@as(f64,N), @as(f64,N)*1e3/@as(f64,@floatFromInt(best)), chk });
    }
    std.debug.print("3-stage: filter |> map |> map |> sum (fusion scaling with depth)\n", .{});
    inline for (.{ "A materialized (3 temps)", "C push-fused" }, 0..) |name, idx| {
        var best: u64 = std.math.maxInt(u64); var chk: i64 = 0;
        for (0..5) |_| { const t0 = nowNs(); chk = if (idx==0) materialized3(src, t1, t2, t3) else pushFused3(src); const dt = nowNs()-t0; if (dt < best) best = dt; }
        std.mem.doNotOptimizeAway(chk);
        std.debug.print("  {s}: {d:.2} ns/elem  ({d:.0} M-elem/s)\n", .{ name, @as(f64,@floatFromInt(best))/@as(f64,N), @as(f64,N)*1e3/@as(f64,@floatFromInt(best)) });
    }
}

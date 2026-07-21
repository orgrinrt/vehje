const std = @import("std");
// Match core-form LOWERING fork (op folded Match into the core set; it lowers on its own, not via Ifs). Real
// match arms have ARBITRARY keys and bodies (a linear scan cannot be strength-reduced away, unlike a scan whose
// body is a closed form of the index). Strategies over arm count K:
//   A) LINEAR IF-CHAIN: compare tag against keys[0], keys[1], ... in order (few arms, or a hot early arm).
//   B) JUMP TABLE: for DENSE small-range keys, index arm directly by tag (O(1)); here a direct-indexed table.
//   C) BINARY DECISION TREE: sorted keys, log2(K) comparisons (many SPARSE arms; table would be huge/holey).
// Distributions: UNIFORM (each arm equally likely) and HOT-FIRST (90% hit arm 0). Output = the lowering rule
// (crossover K where table/tree beats chain; hot-arm exception).
const N: usize = 50_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

// arm keys are ARBITRARY (shuffled), bodies are an opaque per-arm value array (not computable from key or index)
var g_keys: [64]u64 = undefined;   // keys[a] = the arbitrary key of arm a (sorted for the tree variant)
var g_bodies: [64]i64 = undefined; // bodies[a] = arm a's result
var g_dense: [4096]i64 = undefined; // jump table: dense-key -> body (for the dense-key case)

fn ifChain(tag: u64, k: usize) i64 {
    var a: usize = 0; while (a < k) : (a += 1) { if (g_keys[a] == tag) return g_bodies[a]; }
    return -1;
}
fn jumpTable(tag: u64) i64 { return g_dense[tag & 4095]; }
fn decisionTree(tag: u64, k: usize) i64 {
    var l: usize = 0; var h: usize = k;
    while (l < h) { const mid = (l + h) / 2; const key = g_keys[mid]; if (key == tag) return g_bodies[mid]; if (tag < key) h = mid else l = mid + 1; }
    return -1;
}
fn genTags(al: std.mem.Allocator, k: usize, hot_first: bool) []u64 {
    const tags = al.alloc(u64, 4096) catch unreachable; var s: u64 = 12345;
    for (tags) |*t| { s = s*%6364136223846793005+%1; const arm: usize = if (hot_first and (s>>40)%10 != 0) 0 else (s >> 33) % k; t.* = g_keys[arm]; }
    return tags;
}
fn setupArms(k: usize) void {
    // sorted arbitrary keys (spread out so they are sparse, not 0..k), bodies opaque
    var s: u64 = 999;
    for (0..k) |a| { s = s*%2862933555777941757+%3037000493; g_keys[a] = (s >> 20) % 1000000; g_bodies[a] = @intCast((s >> 5) & 0xffff); }
    std.mem.sort(u64, g_keys[0..k], {}, comptime std.sort.asc(u64));
    // dense table: map the low bits of each key into the 4096 table (models the dense-key jump-table case)
    @memset(&g_dense, -1); for (0..k) |a| g_dense[g_keys[a] & 4095] = g_bodies[a];
}
pub fn main() void {
    const al = std.heap.page_allocator;
    std.debug.print("Match lowering: ns/match by arm count K (UNIFORM arm hits; arbitrary sparse keys)\n  K | if-chain | jump-table | decision-tree\n", .{});
    for ([_]usize{ 2, 4, 8, 16, 32, 64 }) |k| {
        setupArms(k); const tags = genTags(al, k, false);
        var b: [3]u64 = .{ std.math.maxInt(u64) } ** 3;
        inline for (0..3) |strat| {
            var best: u64 = std.math.maxInt(u64); var sink: i64 = 0;
            for (0..5) |_| { const t0 = nowNs(); var i: usize = 0; while (i < N) : (i += 1) { const tag = tags[i & 4095];
                sink +%= switch (strat) { 0 => ifChain(tag, k), 1 => jumpTable(tag), else => decisionTree(tag, k) }; }
                const dt = nowNs()-t0; if (dt < best) best = dt; }
            std.mem.doNotOptimizeAway(sink); b[strat] = best;
        }
        std.debug.print("  {d:>2} |  {d:>5.2}  |   {d:>5.2}   |    {d:>5.2}\n", .{ k,
            @as(f64,@floatFromInt(b[0]))/@as(f64,N), @as(f64,@floatFromInt(b[1]))/@as(f64,N), @as(f64,@floatFromInt(b[2]))/@as(f64,N) });
    }
    std.debug.print("\nHOT-FIRST (90%% hit arm 0):\n  K | if-chain | jump-table | decision-tree\n", .{});
    for ([_]usize{ 8, 32, 64 }) |k| {
        setupArms(k);
        // for hot-first, the hot arm must be first in scan order => make arm 0's key the hot one (pre-sort we lose which is arm0). Use a separate hot key = g_keys[0].
        const tags = genTags(al, k, true);
        var b: [3]u64 = .{ std.math.maxInt(u64) } ** 3;
        inline for (0..3) |strat| {
            var best: u64 = std.math.maxInt(u64); var sink: i64 = 0;
            for (0..5) |_| { const t0 = nowNs(); var i: usize = 0; while (i < N) : (i += 1) { const tag = tags[i & 4095];
                sink +%= switch (strat) { 0 => ifChain(tag, k), 1 => jumpTable(tag), else => decisionTree(tag, k) }; }
                const dt = nowNs()-t0; if (dt < best) best = dt; }
            std.mem.doNotOptimizeAway(sink); b[strat] = best;
        }
        std.debug.print("  {d:>2} |  {d:>5.2}  |   {d:>5.2}   |    {d:>5.2}\n", .{ k,
            @as(f64,@floatFromInt(b[0]))/@as(f64,N), @as(f64,@floatFromInt(b[1]))/@as(f64,N), @as(f64,@floatFromInt(b[2]))/@as(f64,N) });
    }
    std.debug.print("(hot-first: 90%% of tags hit g_keys[0], the smallest sorted key = first in scan order)\n", .{});
}

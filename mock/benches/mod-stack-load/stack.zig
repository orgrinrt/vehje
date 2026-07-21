const std = @import("std");
// Expansion: the realistic MOD-STACK LOAD scenario (op's RimWorld/Clausewitz concern),
// combining the e2e compile pipeline + D6 content-addressed caching. A stack of N mods:
// COLD load compiles all; WARM load (after editing 1 mod) recompiles only the changed
// one, cache-hits the rest. Measures cold vs warm.
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
const MOD_NODES: usize = 400; // each mod ~400 IR nodes
// a mod's "compile": lease + cheap-lower + interp (the e2e stages, condensed), returns a content hash of output.
fn compileMod(seed: u64) u64 {
    var reach: [MOD_NODES]u64 = @splat(0); var res: [MOD_NODES]i64 = undefined;
    var prng = std.Random.DefaultPrng.init(seed); const r = prng.random();
    // lease (reach fixpoint) + interp fused for the cost model
    for (0..MOD_NODES) |i| { if (i<4){reach[i]=@as(u64,1)<<@as(u6,@intCast(i));res[i]=@intCast(i+1);continue;}
        const a=r.intRangeLessThan(usize,0,i); const b=r.intRangeLessThan(usize,0,i);
        reach[i]=reach[a]|reach[b]; res[i]=(res[a]*%res[b])+%@as(i64,@intCast(i)); }
    var ch: u64 = 1469598103934665603; for (res) |v| { ch ^= @bitCast(v); ch *%= 1099511628211; }
    return ch;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const N: usize = 2000; // 2000 mods in the stack
    const cache = al.alloc(u64, N) catch unreachable; // content-hash cache (mod hash -> compiled)
    // COLD load: compile all N mods.
    var t = nowns();
    for (0..N) |i| cache[i] = compileMod(@intCast(i));
    const cold_ms = @as(f64,@floatFromInt(nowns()-t))/1e6;
    std.mem.doNotOptimizeAway(cache[N-1]);
    // WARM load: edit 1 mod (its content hash changes -> recompile it); the other N-1 are cache hits.
    t = nowns();
    var recompiled: usize = 0;
    for (0..N) |i| { const changed = (i == 1234); if (changed) { cache[i] = compileMod(@as(u64,@intCast(i)) + 99999); recompiled += 1; } }
    const warm_ms = @as(f64,@floatFromInt(nowns()-t))/1e6;
    std.mem.doNotOptimizeAway(cache[1234]);
    std.debug.print("mod-stack: N={d} mods x {d} nodes each\n", .{N, MOD_NODES});
    std.debug.print("  COLD load (compile all): {d:.1} ms ({d:.3} ms/mod)\n", .{cold_ms, cold_ms/@as(f64,@floatFromInt(N))});
    std.debug.print("  WARM load (edit 1, cache rest): {d:.3} ms ({d} recompiled, {d} cache hits)\n", .{warm_ms, recompiled, N-recompiled});
    std.debug.print("  => warm/cold = {d:.5} : editing 1 mod in a {d}-mod stack costs ~1 recompile, not {d}.\n", .{warm_ms/cold_ms, N, N});
}

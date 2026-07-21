const std = @import("std");
// Expansion: value-arena transport throughput (SK20). Encode (build), zero-copy read
// (walk), and typed validate over a large arena, in GB/s.
const Node = extern struct { kind: u8, _p:[3]u8=.{0,0,0}, a: u32, b: u32 }; // 12B... pad to 16
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
const N: usize = 16_000_000;
pub fn main() void {
    const al = std.heap.page_allocator;
    const nodes = al.alignedAlloc(Node, .@"64", N) catch unreachable;
    const pool = al.alloc(u32, 2 * N) catch unreachable;
    const bytes = N * @sizeOf(Node);
    // ENCODE: build children-first (each record refs 2 backward children), pre-sized (no realloc).
    var prng = std.Random.DefaultPrng.init(5); const r = prng.random();
    var pn: u32 = 0;
    const t0 = nowns();
    for (0..N) |i| {
        if (i < 2) { nodes[i] = .{ .kind = 0, .a = @intCast(i), .b = 0 }; continue; }
        if (i % 3 == 0) { nodes[i] = .{ .kind = 0, .a = @intCast(i & 0xFFFF), .b = 0 }; } // scalar
        else { const c0 = r.intRangeLessThan(u32,0,@intCast(i)); const c1 = r.intRangeLessThan(u32,0,@intCast(i)); pool[pn]=c0; pool[pn+1]=c1; nodes[i] = .{ .kind = 2, .a = pn, .b = 2 }; pn += 2; }
    }
    const enc_ms = @as(f64,@floatFromInt(nowns()-t0))/1e6;
    // ZERO-COPY READ: iterative post-order sum via a results array (the interp walk shape).
    const res = al.alloc(i64, N) catch unreachable;
    const t1 = nowns();
    for (0..N) |i| { const n = nodes[i]; res[i] = if (n.kind==0) @intCast(n.a) else res[pool[n.a]] + res[pool[n.a+1]]; }
    const read_ms = @as(f64,@floatFromInt(nowns()-t1))/1e6;
    std.mem.doNotOptimizeAway(res[N-1]);
    // VALIDATE: typed structural decode (backward-index + pool-span-in-range), linear.
    const t2 = nowns();
    var ok = true; for (nodes, 0..) |n, i| { if (n.kind==2) { if (@as(usize,n.a)+n.b > pn or pool[n.a]>=i or pool[n.a+1]>=i) { ok=false; break; } } }
    const val_ms = @as(f64,@floatFromInt(nowns()-t2))/1e6;
    std.mem.doNotOptimizeAway(ok);
    const gb = @as(f64,@floatFromInt(bytes))/1e9;
    std.debug.print("arena {d} nodes = {d:.0} MB nodes + {d:.0} MB pool\n", .{N, gb*1000, @as(f64,@floatFromInt(pn*4))/1e6});
    std.debug.print("encode:   {d:.1} ms = {d:.1} GB/s\n", .{enc_ms, gb/(enc_ms/1000)});
    std.debug.print("zero-copy read (walk): {d:.1} ms = {d:.1} GB/s\n", .{read_ms, gb/(read_ms/1000)});
    std.debug.print("typed validate: {d:.1} ms = {d:.1} GB/s (ok={})\n", .{val_ms, gb/(val_ms/1000), ok});
}

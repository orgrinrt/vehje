const std = @import("std");
// BN3 (D6): differential/incremental recompute. A compile-stage query (reach over a
// balanced tree of N shards) is computed fully, then ONE shard (a leaf binder) is
// edited; the incremental recompute propagates ONLY up the changed leaf's ancestor
// path (differential dataflow's benefit), vs a full recompute of all N. Measures
// node-visits (proportional to work). Confirms "edit one -> recompute ~changes, not N".
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec)); }
pub fn main() void {
    const al = std.heap.page_allocator;
    std.debug.print("N,full_visits,incr_visits,ratio,full_ns,incr_ns\n", .{});
    for ([_]usize{ 1023, 65535, 1_048_575 }) |N| { // complete binary trees (2^k - 1)
        const reach = al.alloc(u64, N) catch unreachable;
        // node i children = 2i+1, 2i+2 (heap layout). leaves seed a binder bit.
        // FULL: bottom-up, reach[i] = ownbit | reach[2i+1] | reach[2i+2]. count visits.
        var full_visits: usize = 0;
        const t0 = nowns();
        var i: usize = N; while (i > 0) { i -= 1; var r: u64 = (@as(u64, 1) << @intCast(i % 60)); const l = 2 * i + 1; const rr = 2 * i + 2; if (l < N) r |= reach[l]; if (rr < N) r |= reach[rr]; reach[i] = r; full_visits += 1; }
        const full_ns = nowns() - t0;
        // EDIT one leaf (last node) + INCREMENTAL recompute: only walk up its ancestor path.
        var incr_visits: usize = 0;
        const t1 = nowns();
        var node: usize = N - 1; // a leaf
        // change its own bit, then re-OR up to root, stopping if a node's value is unchanged
        reach[node] |= (@as(u64, 1) << 63); // the edit
        while (true) { incr_visits += 1; if (node == 0) break; const parent = (node - 1) / 2;
            const l = 2 * parent + 1; const rr = 2 * parent + 2; var r: u64 = (@as(u64, 1) << @intCast(parent % 60)); if (l < N) r |= reach[l]; if (rr < N) r |= reach[rr];
            if (r == reach[parent]) break; // no change propagates further (differential early-out)
            reach[parent] = r; node = parent; }
        const incr_ns = nowns() - t1;
        std.debug.print("{d},{d},{d},{d:.5},{d},{d}\n", .{ N, full_visits, incr_visits, @as(f64, @floatFromInt(incr_visits)) / @as(f64, @floatFromInt(full_visits)), full_ns, incr_ns });
    }
}

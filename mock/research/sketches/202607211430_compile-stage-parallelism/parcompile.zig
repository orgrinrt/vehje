const std = @import("std");
// Expansion: does the COMPILE STAGE parallelize across independent mods? The e2e capstone showed the
// compile stage (lease-infer + cheap-lower) dominates cost (3.24ms of 3.4ms), and mod-stack-load ran the
// stack SERIALLY. op's load-time concern (Clausewitz/RimWorld cold load of a big mod stack) hinges on
// whether N independent mods compile across cores. Measure core-scaling speedup 1->2->4->8 threads.
//
// Each "mod compile" = a representative compile-stage workload on an independent per-mod IR arena:
// a reachability-style fixpoint fold + a const-fold/CSE hash-cons pass (the two ops the e2e measured as
// the compile-stage cost). Mods are INDEPENDENT (the D6 independent-mod-caching regime), so no shared
// mutable state, no locks: the embarrassingly-parallel case, which is exactly what a mod stack is.

const NODES_PER_MOD: usize = 8192; // a mid-size mod's IR
const Node = extern struct { op: u8, _p:[3]u8=.{0,0,0}, a: u32, b: u32 };

fn buildMod(seed: u64, buf: []Node) void {
    var s = seed;
    for (buf, 0..) |*n, i| {
        s = s *% 6364136223846793005 +% 1442695040888963407;
        const r = s >> 33;
        n.* = .{ .op = @intCast(r % 3), .a = if (i>0) @intCast((r % i)) else 0, .b = if (i>0) @intCast(((r>>8) % i)) else 0 };
    }
}
// the per-mod compile-stage work: reach-fold (backward OR) + a const-fold/CSE counting pass. Pure per-mod.
fn compileMod(nodes: []const Node, reach: []u64, folded: *u64) void {
    for (nodes, 0..) |n, i| {
        var m: u64 = @as(u64, 1) << @intCast(i % 63);
        if (n.op == 2) { m |= reach[n.a] | reach[n.b]; } // a "call" merges child reach
        reach[i] = m;
    }
    var f: u64 = 0;
    var seen: [4096]u64 = [_]u64{0} ** 4096; // tiny CSE table (hash-cons stand-in)
    for (nodes) |n| { const h = (@as(u64,n.op) *% 0x9E3779B1 ^ (@as(u64,n.a)<<1) ^ n.b) & 4095; if (seen[h] == 0) { seen[h] = 1; f += 1; } }
    folded.* = f;
}

const Job = struct { nodes: []const Node, reach: []u64, folded: *u64 };
fn worker(jobs: []Job, next: *std.atomic.Value(usize)) void {
    while (true) {
        const i = next.fetchAdd(1, .monotonic);
        if (i >= jobs.len) break;
        compileMod(jobs[i].nodes, jobs[i].reach, jobs[i].folded);
    }
}

fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

pub fn main() void {
    const al = std.heap.page_allocator;
    const N_MODS: usize = 2000; // op's RimWorld-scale stack
    const allNodes = al.alloc(Node, N_MODS * NODES_PER_MOD) catch unreachable;
    const allReach = al.alloc(u64, N_MODS * NODES_PER_MOD) catch unreachable;
    const folded = al.alloc(u64, N_MODS) catch unreachable;
    for (0..N_MODS) |m| buildMod(@as(u64, m) *% 2654435761 +% 7919, allNodes[m*NODES_PER_MOD..(m+1)*NODES_PER_MOD]);
    var jobs = al.alloc(Job, N_MODS) catch unreachable;
    for (0..N_MODS) |m| jobs[m] = .{ .nodes = allNodes[m*NODES_PER_MOD..(m+1)*NODES_PER_MOD], .reach = allReach[m*NODES_PER_MOD..(m+1)*NODES_PER_MOD], .folded = &folded[m] };

    const thread_counts = [_]usize{ 1, 2, 4, 8 };
    var base_ns: u64 = 0;
    std.debug.print("2000 mods x 8192 nodes, compile-stage (reach-fold + CSE) per mod:\n", .{});
    for (thread_counts) |nt| {
        var best: u64 = std.math.maxInt(u64);
        for (0..5) |_| {
            var next = std.atomic.Value(usize).init(0);
            const t0 = nowNs();
            if (nt == 1) { worker(jobs, &next); }
            else {
                const threads = al.alloc(std.Thread, nt) catch unreachable;
                for (0..nt) |k| threads[k] = std.Thread.spawn(.{}, worker, .{ jobs, &next }) catch unreachable;
                for (0..nt) |k| threads[k].join();
                al.free(threads);
            }
            const dt = nowNs() - t0;
            if (dt < best) best = dt;
        }
        if (nt == 1) base_ns = best;
        const spd = @as(f64, @floatFromInt(base_ns)) / @as(f64, @floatFromInt(best));
        std.debug.print("  {d} thread(s): {d:.2} ms   speedup {d:.2}x   ({d:.0} mods/s)\n", .{ nt, @as(f64,@floatFromInt(best))/1e6, spd, @as(f64,@floatFromInt(N_MODS))*1e9/@as(f64,@floatFromInt(best)) });
    }
    var sink: u64 = 0; for (folded) |f| sink +%= f; std.debug.print("(sink {d})\n", .{sink});
}

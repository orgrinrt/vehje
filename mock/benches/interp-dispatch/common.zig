const std = @import("std");
pub const Node = extern struct { op: u8, _p: [3]u8 = .{ 0, 0, 0 }, a: u32, b: u32 };
pub const N: usize = 4_000_000;

pub fn genProgram(alloc: std.mem.Allocator, local: bool) []Node {
    var prng = std.Random.DefaultPrng.init(0xC0FFEE_1234);
    const rnd = prng.random();
    const nodes = alloc.alloc(Node, N) catch unreachable;
    const win: u32 = 16; // backward-local window (L1-hot) when `local`
    for (0..N) |i| {
        if (i < 2) { nodes[i] = .{ .op = 0, .a = @intCast(i + 1), .b = 0 }; continue; }
        const which = rnd.intRangeAtMost(u8, 0, 2);
        if (which == 0) {
            nodes[i] = .{ .op = 0, .a = @intCast(i % 7 + 1), .b = 0 };
        } else {
            var a: u32 = undefined; var b: u32 = undefined;
            if (local) {
                const lo: u32 = if (i > win) @intCast(i - win) else 0;
                a = rnd.intRangeLessThan(u32, lo, @intCast(i));
                b = rnd.intRangeLessThan(u32, lo, @intCast(i));
            } else {
                a = rnd.intRangeLessThan(u32, 0, @intCast(i));
                b = rnd.intRangeLessThan(u32, 0, @intCast(i));
            }
            nodes[i] = .{ .op = if (which == 1) 1 else 2, .a = a, .b = b };
        }
    }
    nodes[N - 1] = .{ .op = 3, .a = 0, .b = 0 };
    return nodes;
}
fn nowns() u64 {
    var ts: std.c.timespec = undefined;
    _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts);
    return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec));
}
pub fn report(name: []const u8, wl: []const u8, nodes: []const Node, res: []i64, run: *const fn ([]const Node, []i64) void) void {
    run(nodes, res); run(nodes, res); // warmup
    const passes = 9;
    var times: [passes]u64 = undefined;
    for (0..passes) |p| { const a = nowns(); run(nodes, res); times[p] = nowns() - a; }
    std.mem.sort(u64, &times, {}, std.sort.asc(u64));
    const med = times[passes / 2];
    const nspo = @as(f64, @floatFromInt(med)) / @as(f64, @floatFromInt(nodes.len));
    std.debug.print("{s},{s},{d:.4},{d}\n", .{ name, wl, nspo, res[nodes.len - 2] });
}
pub fn runBoth(name: []const u8, run: *const fn ([]const Node, []i64) void) void {
    const alloc = std.heap.page_allocator;
    const res = alloc.alloc(i64, N) catch unreachable;
    const local = genProgram(alloc, true);
    report(name, "local", local, res, run);
    const random = genProgram(alloc, false);
    report(name, "random", random, res, run);
}

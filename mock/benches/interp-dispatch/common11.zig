const std = @import("std");
pub const Node = extern struct { op: u8, _p: [3]u8 = .{ 0, 0, 0 }, a: u32, b: u32 };
pub const N: usize = 4_000_000;
pub const NOPS: u8 = 11; // 0=lit .. 9=various, 10=halt
pub fn genProgram(alloc: std.mem.Allocator) []Node {
    var prng = std.Random.DefaultPrng.init(0xBEEF_5678); const rnd = prng.random();
    const nodes = alloc.alloc(Node, N) catch unreachable; const win: u32 = 16;
    for (0..N) |i| {
        if (i < 2) { nodes[i] = .{ .op = 0, .a = @intCast(i + 1), .b = 0 }; continue; }
        const op = rnd.intRangeAtMost(u8, 0, 9); // spread across 10 real ops
        if (op == 0) { nodes[i] = .{ .op = 0, .a = @intCast(i % 7 + 1), .b = 0 }; }
        else { const lo: u32 = if (i > win) @intCast(i - win) else 0;
            nodes[i] = .{ .op = op, .a = rnd.intRangeLessThan(u32, lo, @intCast(i)), .b = rnd.intRangeLessThan(u32, lo, @intCast(i)) }; }
    }
    nodes[N - 1] = .{ .op = 10, .a = 0, .b = 0 }; return nodes;
}
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec)); }
pub fn report(name: []const u8, nodes: []const Node, res: []i64, run: *const fn ([]const Node, []i64) void) void {
    run(nodes, res); run(nodes, res); const passes = 9; var times: [passes]u64 = undefined;
    for (0..passes) |p| { const a = nowns(); run(nodes, res); times[p] = nowns() - a; }
    std.mem.sort(u64, &times, {}, std.sort.asc(u64));
    std.debug.print("{s},11ops,{d:.4},{d}\n", .{ name, @as(f64, @floatFromInt(times[passes/2])) / @as(f64, @floatFromInt(nodes.len)), res[nodes.len-2] });
}

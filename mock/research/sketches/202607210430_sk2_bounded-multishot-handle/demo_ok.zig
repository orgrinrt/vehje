const std = @import("std"); const h = @import("handle.zig");
// the "rest of the program" after 3 amb points (domains 2,3,2), defunctionalised
fn program(c: []const u32) i64 { return @as(i64, c[0]) * 100 + @as(i64, c[1]) * 10 + @as(i64, c[2]); }
pub fn main() void {
    const doms = &[_]u32{ 2, 3, 2 }; // 12 combinations
    var budget: [16]i64 = undefined; // host lends 16 slots (>= 12: fits)
    const n = h.collectAll(doms, 16, &budget, program);
    std.debug.print("bounded multi-shot: {d} resumptions, NO heap, deterministic:\n ", .{n});
    for (0..n) |i| std.debug.print("{d} ", .{budget[i]});
    std.debug.print("\n", .{});
    // determinism: a second run yields identical results
    var b2: [16]i64 = undefined; _ = h.collectAll(doms, 16, &b2, program);
    std.debug.print("deterministic (run2 == run1): {}\n", .{std.mem.eql(i64, budget[0..n], b2[0..n])});
}

const std = @import("std");
fn nowns() u64 {
    const ts = std.posix.clock_gettime(.MONOTONIC) catch unreachable;
    return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec));
}
pub fn main() void {
    const a = nowns(); var s: u64 = 0; for (0..1000000) |i| s +%= i; const b = nowns();
    std.debug.print("elapsed_ns={d} s={d}\n", .{ b - a, s });
}

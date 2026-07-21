const std = @import("std");
pub const N: usize = 4_000_000;
// logical program: op 0=lit,1=add(ar2),2=mul(ar2),3=call(ar3). operands = backward-local result indices.
pub const Logical = struct { op: u8, arity: u8, ops: [3]u32 };
pub fn gen(alloc: std.mem.Allocator, call_frac_pct: u32) []Logical {
    var prng = std.Random.DefaultPrng.init(0xD00D_0001); const rnd = prng.random();
    const a = alloc.alloc(Logical, N) catch unreachable; const win: u32 = 16;
    for (0..N) |i| {
        if (i < 3) { a[i] = .{ .op = 0, .arity = 0, .ops = .{ @intCast(i + 1), 0, 0 } }; continue; }
        const lo: u32 = if (i > win) @intCast(i - win) else 0;
        const roll = rnd.intRangeAtMost(u32, 0, 99);
        if (roll < call_frac_pct) {
            a[i] = .{ .op = 3, .arity = 3, .ops = .{ rnd.intRangeLessThan(u32, lo, @intCast(i)), rnd.intRangeLessThan(u32, lo, @intCast(i)), rnd.intRangeLessThan(u32, lo, @intCast(i)) } };
        } else {
            const opk: u8 = if (rnd.boolean()) 1 else 2;
            a[i] = .{ .op = opk, .arity = 2, .ops = .{ rnd.intRangeLessThan(u32, lo, @intCast(i)), rnd.intRangeLessThan(u32, lo, @intCast(i)), 0 } };
        }
    }
    return a;
}
pub fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec)); }
pub fn timeWalk(name: []const u8, cf: u32, nbytes: usize, run: *const fn () i64) void {
    _ = run(); _ = run();
    var t: [9]u64 = undefined; for (0..9) |p| { const a = nowns(); const chk = run(); std.mem.doNotOptimizeAway(chk); t[p] = nowns() - a; }
    std.mem.sort(u64, &t, {}, std.sort.asc(u64));
    std.debug.print("{s},{d},{d},{d:.4}\n", .{ name, nbytes, cf, @as(f64, @floatFromInt(t[4])) / @as(f64, @floatFromInt(N)) });
}

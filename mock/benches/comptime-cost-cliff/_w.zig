const std = @import("std");
const Rec = struct { k: u32, v: u32 };
pub fn main() void {
    @setEvalBranchQuota(1000000000);
    const sum = comptime blk: {
        var recs: [2000]Rec = undefined;
        for (0..2000) |i| recs[i] = .{ .k = @intCast((i *% 2654435761) % 2000), .v = @intCast(i) };
        var acc: u64 = 0;
        for (0..recs.len) |i| { acc +%= recs[i].v;
            for (0..i) |j| { if (recs[j].k == recs[i].k) acc +%= 1; } }
        break :blk acc;
    };
    std.debug.print("content acc={d}\n", .{sum});
}

const std = @import("std");
const Rec = struct { k: u32, v: u32 };
pub fn main() void {
    @setEvalBranchQuota(1000000000);
    const sum = comptime blk: {
        var recs: [__N__]Rec = undefined;
        for (0..__N__) |i| recs[i] = .{ .k = @intCast((i *% 2654435761) % __N__), .v = @intCast(i) };
        var acc: u64 = 0; var last: u32 = 0;
        for (0..recs.len) |i| { acc +%= recs[i].v; if (recs[i].k < last) acc +%= 1; last = recs[i].k; }
        break :blk acc;
    };
    std.debug.print("cnt1 acc={d}\n", .{sum});
}

const std = @import("std");
pub fn main() void {
    @setEvalBranchQuota(1000000000);
    const T = comptime blk: {
        var names: [__N__][:0]const u8 = undefined;
        var vals: [__N__]u32 = undefined;
        for (0..__N__) |i| { names[i] = std.fmt.comptimePrint("f{d}", .{i}); vals[i] = @intCast(i); }
        break :blk @Enum(u32, .exhaustive, &names, &vals);
    };
    std.debug.print("table fields={d}\n", .{@typeInfo(T).@"enum".fields.len});
}

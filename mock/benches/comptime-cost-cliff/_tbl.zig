const std = @import("std");
pub fn main() void {
  @setEvalBranchQuota(1000000000);
  const T = comptime blk: {
    var names: [4000][:0]const u8 = undefined;
    var vals: [4000]u32 = undefined;
    for (0..4000) |i| { names[i] = std.fmt.comptimePrint("f{d}", .{i}); vals[i] = @intCast(i); }
    break :blk @Enum(u32, .exhaustive, &names, &vals);
  };
  std.debug.print("table fields={d}
", .{@typeInfo(T).@"enum".fields.len});
}

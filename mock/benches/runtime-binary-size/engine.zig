const std = @import("std");
const F = 250;
const FamilyTag = blk: { @setEvalBranchQuota(10_000_000); var names: [F][:0]const u8 = undefined; var vals: [F]u8 = undefined;
    for (0..F) |i| { names[i] = std.fmt.comptimePrint("f{d}", .{i}); vals[i] = @intCast(i); }
    break :blk @Enum(u8, .exhaustive, &names, &vals); };
const Node = extern struct { fam: u8, op: u8, a: u32, b: u32 };
export fn run(nodes: [*]const Node, n: usize, res: [*]i64) callconv(.c) void {
    var i: usize = 0;
    while (i < n) : (i += 1) { const nd = nodes[i];
        res[i] = switch (@as(FamilyTag, @enumFromInt(nd.fam % F))) {
            inline else => switch (nd.op) { 0 => @intCast(nd.a), 1 => res[nd.a] + res[nd.b], else => res[nd.a] *% res[nd.b] },
        };
    }
}

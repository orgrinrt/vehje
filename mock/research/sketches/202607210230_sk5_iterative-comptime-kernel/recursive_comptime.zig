const std = @import("std");
// Comptime recursive fold over a left-spine IR tree of height D (worst case:
// tree height == the compiler's own comptime call-stack depth).
const Node = struct { val: usize, child: ?usize };
fn foldRec(comptime nodes: []const Node, comptime i: usize) usize {
    const n = nodes[i];
    if (n.child) |c| return n.val + foldRec(nodes, c);
    return n.val;
}
fn spine(comptime D: usize) [D]Node {
    var a: [D]Node = undefined;
    for (0..D) |k| a[k] = .{ .val = k, .child = if (k + 1 < D) k + 1 else null };
    return a;
}
pub fn main() void {
    @setEvalBranchQuota(1_000_000_000);
    const D: usize = __D__;
    const nodes = comptime spine(D);
    const s = comptime foldRec(&nodes, 0);
    std.debug.print("recursive fold D={d} = {d} (expect {d})\n", .{ D, s, D * (D - 1) / 2 });
}

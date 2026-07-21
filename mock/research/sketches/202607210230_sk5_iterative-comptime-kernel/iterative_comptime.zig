const std = @import("std");
const Node = struct { val: usize, l: ?usize, r: ?usize };
fn spine(comptime D: usize) [D]Node {
    var a: [D]Node = undefined;
    for (0..D) |k| a[k] = .{ .val = k, .l = if (k + 1 < D) k + 1 else null, .r = null };
    return a;
}
// Defunctionalised work-stack fold: explicit comptime stack, fixed capacity CAP
// (the depth-cap bound). No native recursion, so the compiler's call stack is flat.
fn foldIter(comptime nodes: []const Node, comptime root: usize, comptime CAP: usize) usize {
    comptime {
        var stack: [CAP]usize = undefined;
        var sp: usize = 0;
        stack[sp] = root; sp += 1;
        var acc: usize = 0;
        while (sp > 0) {
            sp -= 1;
            const n = nodes[stack[sp]];
            acc += n.val;
            if (n.r) |c| { stack[sp] = c; sp += 1; }
            if (n.l) |c| { stack[sp] = c; sp += 1; }
        }
        return acc;
    }
}
pub fn main() void {
    @setEvalBranchQuota(1_000_000_000);
    const D: usize = 100000; // far past the recursive segfault point
    const nodes = comptime spine(D);
    const s = comptime foldIter(&nodes, 0, 8); // CAP=8 suffices for a spine (max live frontier 1)
    std.debug.print("iterative fold D={d} = {d} (expect {d})\n", .{ D, s, D * (D - 1) / 2 });
}

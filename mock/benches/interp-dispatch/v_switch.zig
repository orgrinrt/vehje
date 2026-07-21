const std = @import("std"); const c = @import("common.zig");
fn run(nodes: []const c.Node, res: []i64) void {
    var i: usize = 0;
    while (i < nodes.len) : (i += 1) { const n = nodes[i]; switch (n.op) {
        0 => res[i] = @intCast(n.a), 1 => res[i] = res[n.a] + res[n.b], 2 => res[i] = res[n.a] *% res[n.b], else => return, } }
}
pub fn main() void { c.runBoth("switch", &run); }

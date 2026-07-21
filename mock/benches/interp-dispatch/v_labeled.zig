const std = @import("std"); const c = @import("common.zig");
fn run(nodes: []const c.Node, res: []i64) void {
    var i: usize = 0;
    sw: switch (nodes[i].op) {
        0 => { res[i] = @intCast(nodes[i].a); i += 1; if (i < nodes.len) continue :sw nodes[i].op; },
        1 => { res[i] = res[nodes[i].a] + res[nodes[i].b]; i += 1; if (i < nodes.len) continue :sw nodes[i].op; },
        2 => { res[i] = res[nodes[i].a] *% res[nodes[i].b]; i += 1; if (i < nodes.len) continue :sw nodes[i].op; },
        else => {},
    }
}
pub fn main() void { c.runBoth("labeled", &run); }

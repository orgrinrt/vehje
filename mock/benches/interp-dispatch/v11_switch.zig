const std = @import("std"); const c = @import("common11.zig");
fn run(nodes: []const c.Node, res: []i64) void { var i: usize = 0;
    while (i < nodes.len) : (i += 1) { const n = nodes[i]; const x = res[n.a]; const y = res[n.b]; switch (n.op) {
        0 => res[i] = @intCast(n.a), 1 => res[i] = x + y, 2 => res[i] = x *% y, 3 => res[i] = x - y,
        4 => res[i] = x & y, 5 => res[i] = x | y, 6 => res[i] = x ^ y, 7 => res[i] = x >> @intCast(@as(u6, @truncate(@as(u64, @bitCast(y))))),
        8 => res[i] = @min(x, y), 9 => res[i] = @max(x, y), else => return, } }
}
pub fn main() void { const a = std.heap.page_allocator; c.report("switch", c.genProgram(a), a.alloc(i64, c.N) catch unreachable, &run); }

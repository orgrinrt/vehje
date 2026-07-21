const std = @import("std"); const c = @import("common_rw.zig");
const Node = extern struct { op: u8, arity: u8, _p: u16 = 0, a: u32, b: u32, cc: u32, _p2: u32 = 0 }; // 24 bytes
var nodes: []Node = undefined; var res: []i64 = undefined;
fn run() i64 {
    var i: usize = 0; while (i < nodes.len) : (i += 1) { const n = nodes[i]; switch (n.op) {
        0 => res[i] = @intCast(n.a), 1 => res[i] = res[n.a] + res[n.b], 2 => res[i] = res[n.a] *% res[n.b],
        else => res[i] = res[n.a] + res[n.b] + res[n.cc] } }
    return res[nodes.len - 2];
}
pub fn main() void {
    const al = std.heap.page_allocator;
    for ([_]u32{5,20,40}) |cf| {
        const lg = c.gen(al, cf);
        nodes = al.alloc(Node, c.N) catch unreachable; res = al.alloc(i64, c.N) catch unreachable;
        for (lg, 0..) |l, i| nodes[i] = .{ .op = l.op, .arity = l.arity, .a = l.ops[0], .b = l.ops[1], .cc = l.ops[2] };
        c.timeWalk("rec24", cf, 24, &run);
    }
}

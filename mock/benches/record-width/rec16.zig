const std = @import("std"); const c = @import("common_rw.zig");
const Node = extern struct { op: u8, arity: u8, _p: u16 = 0, a: u32, b: u32 }; // 16 bytes
var nodes: []Node = undefined; var pool: []u32 = undefined; var res: []i64 = undefined;
fn run() i64 {
    var i: usize = 0; while (i < nodes.len) : (i += 1) { const n = nodes[i]; switch (n.op) {
        0 => res[i] = @intCast(n.a), 1 => res[i] = res[n.a] + res[n.b], 2 => res[i] = res[n.a] *% res[n.b],
        else => { var s: i64 = 0; var k: usize = 0; while (k < n.arity) : (k += 1) s += res[pool[n.b + k]]; res[i] = s; } } }
    return res[nodes.len - 2];
}
pub fn main() void {
    const al = std.heap.page_allocator;
    for ([_]u32{5,20,40}) |cf| {
        const lg = c.gen(al, cf);
        nodes = al.alloc(Node, c.N) catch unreachable; res = al.alloc(i64, c.N) catch unreachable;
        var plist = std.ArrayListUnmanaged(u32).empty;
        for (lg, 0..) |l, i| { if (l.arity <= 2) nodes[i] = .{ .op = l.op, .arity = l.arity, .a = l.ops[0], .b = l.ops[1] }
            else { const off: u32 = @intCast(plist.items.len); var k: usize = 0; while (k < l.arity) : (k += 1) plist.append(al, l.ops[k]) catch unreachable; nodes[i] = .{ .op = l.op, .arity = l.arity, .a = 0, .b = off }; } }
        pool = plist.items;
        c.timeWalk("rec16", cf, 16, &run);
    }
}

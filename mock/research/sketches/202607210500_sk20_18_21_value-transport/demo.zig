const std = @import("std"); const A = @import("arena.zig");
pub fn main() void {
    const al = std.heap.page_allocator;
    var ar = A.Arena{ .al = al };
    // value: record{ scalar 10, record{ scalar 3, scalar 4, blob "hi" }, scalar 100 }  (children-first)
    const s10 = ar.emitScalar(10);
    const s3 = ar.emitScalar(3); const s4 = ar.emitScalar(4); const bl = ar.emitBlob("hi");
    const inner = ar.emitRecord(&.{ s3, s4, bl });
    const s100 = ar.emitScalar(100);
    const root = ar.emitRecord(&.{ s10, inner, s100 });
    const nodes = ar.nodes.items; const pool = ar.pool.items;
    std.debug.print("nodes={d} pool={d} blob={d} sum={d} (expect 117)\n", .{ nodes.len, pool.len, ar.blob.items.len, A.sumScalars(nodes, pool, root) });
    std.debug.print("validate trusted arena: {} (expect true)\n", .{A.validate(nodes, pool, ar.blob.items.len, 16)});
    // SK18: corrupt a child index to a FORWARD ref (cycle) -> decode must reject
    var bad = al.dupe(A.Node, nodes) catch unreachable; var badpool = al.dupe(u32, pool) catch unreachable;
    badpool[badpool.len - 1] = @intCast(nodes.len); // forward/out-of-range child
    std.debug.print("validate corrupted (forward child) arena: {} (expect false)\n", .{A.validate(bad, badpool, ar.blob.items.len, 16)});
    // blob overrun
    bad[bl].b = 9999;
    std.debug.print("validate corrupted (blob overrun) arena: {} (expect false)\n", .{A.validate(bad, pool, ar.blob.items.len, 16)});
}

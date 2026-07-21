const std = @import("std");
// SK20/SK18/SK21: the value-arena transport. Fixed-width records + flat child-index
// pool + byte blob, relative (backward) indices, children-first emission (no
// back-patching), zero-copy read, and the typed structural decode (untrusted path).
pub const Kind = enum(u8) { scalar = 0, blob = 1, record = 2 };
pub const Node = extern struct { kind: u8, _p: [3]u8 = .{ 0, 0, 0 }, a: u32, b: u32 }; // 16B

pub const Arena = struct {
    nodes: std.ArrayListUnmanaged(Node) = .empty,
    pool: std.ArrayListUnmanaged(u32) = .empty, // child node-indices (backward)
    blob: std.ArrayListUnmanaged(u8) = .empty,
    al: std.mem.Allocator,
    pub fn emitScalar(self: *Arena, v: i64) u32 {
        const u: u64 = @bitCast(v);
        self.nodes.append(self.al, .{ .kind = 0, .a = @truncate(u), .b = @truncate(u >> 32) }) catch unreachable;
        return @intCast(self.nodes.items.len - 1);
    }
    pub fn emitBlob(self: *Arena, bytes: []const u8) u32 {
        const off: u32 = @intCast(self.blob.items.len);
        self.blob.appendSlice(self.al, bytes) catch unreachable;
        self.nodes.append(self.al, .{ .kind = 1, .a = off, .b = @intCast(bytes.len) }) catch unreachable;
        return @intCast(self.nodes.items.len - 1);
    }
    // children already emitted (children-first) -> their indices are known, no back-patch.
    pub fn emitRecord(self: *Arena, children: []const u32) u32 {
        const off: u32 = @intCast(self.pool.items.len);
        self.pool.appendSlice(self.al, children) catch unreachable;
        self.nodes.append(self.al, .{ .kind = 2, .a = off, .b = @intCast(children.len) }) catch unreachable;
        return @intCast(self.nodes.items.len - 1);
    }
};

// Zero-copy reader: sum all scalar leaves reachable from `root` (walks the arena, no deserialise).
pub fn sumScalars(nodes: []const Node, pool: []const u32, root: u32) i64 {
    const n = nodes[root];
    return switch (n.kind) {
        0 => @bitCast((@as(u64, n.a) | (@as(u64, n.b) << 32))),
        1 => 0,
        else => blk: { var s: i64 = 0; for (pool[n.a .. n.a + n.b]) |ci| s += sumScalars(nodes, pool, ci); break :blk s; },
    };
}

// SK18: typed structural decode of an UNTRUSTED arena. Linear, complete: every record's
// child indices are strictly backward (< own index) => acyclic by construction; every
// pool span and blob ref in range; depth <= cap. Returns false on any violation.
pub fn validate(nodes: []const Node, pool: []const u32, blob_len: usize, depth_cap: u32) bool {
    for (nodes, 0..) |n, i| {
        switch (n.kind) {
            0 => {},
            1 => { if (@as(usize, n.a) + n.b > blob_len) return false; },
            2 => {
                if (@as(usize, n.a) + n.b > pool.len) return false;
                for (pool[n.a .. n.a + n.b]) |ci| { if (ci >= i) return false; } // strictly backward
            },
            else => return false,
        }
    }
    // depth bound via a linear DP over the backward DAG (no recursion)
    if (nodes.len == 0) return true;
    const al = std.heap.page_allocator; const d = al.alloc(u32, nodes.len) catch unreachable; defer al.free(d);
    for (nodes, 0..) |n, i| {
        if (n.kind != 2) { d[i] = 1; continue; }
        var m: u32 = 0; for (pool[n.a .. n.a + n.b]) |ci| m = @max(m, d[ci]);
        d[i] = m + 1; if (d[i] > depth_cap) return false;
    }
    return true;
}

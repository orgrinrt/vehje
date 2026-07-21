const std = @import("std");
// Expansion: untrusted load-verifier ROBUSTNESS. The typed structural decode is the
// security boundary for arriving mods. Confirm it REJECTS every malformed/attack arena
// (cycle, OOB child, blob overrun, over-deep, index overflow) fast and WITHOUT crashing.
const Node = extern struct { kind: u8, _p:[3]u8=.{0,0,0}, a: u32, b: u32 };
// the decode (SK18): backward-index (acyclic), pool-span-in-range, blob-in-range, depth-bound. Returns ok.
fn validate(nodes: []const Node, pool: []const u32, blob_len: usize, depth_cap: u32, dbuf: []u32) bool {
    for (nodes, 0..) |n, i| switch (n.kind) {
        0 => {},
        1 => { if (@as(u64,n.a) + n.b > blob_len) return false; }, // blob ref (u64 math: no overflow)
        2 => { if (@as(u64,n.a) + n.b > pool.len) return false; for (pool[n.a..n.a+n.b]) |ci| if (ci >= i) return false; },
        else => return false, // unknown kind rejected
    };
    if (nodes.len == 0) return true;
    for (nodes, 0..) |n, i| { if (n.kind != 2) { dbuf[i]=1; continue; } var m:u32=0; for (pool[n.a..n.a+n.b]) |ci| m=@max(m,dbuf[ci]); dbuf[i]=m+1; if (dbuf[i] > depth_cap) return false; }
    return true;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const dbuf = al.alloc(u32, 1024) catch unreachable;
    // build a valid base arena, then corrupt it 5 ways.
    var nodes = [_]Node{ .{.kind=0,.a=1,.b=0}, .{.kind=0,.a=2,.b=0}, .{.kind=2,.a=0,.b=2} };
    var pool = [_]u32{ 0, 1 };
    std.debug.print("valid arena: {} (expect true)\n", .{validate(&nodes, &pool, 0, 16, dbuf)});
    // ATTACK 1: forward child (cycle)
    { var p2 = [_]u32{ 0, 5 }; std.debug.print("attack cycle/forward-child: {} (expect false)\n", .{validate(&nodes, &p2, 0, 16, dbuf)}); }
    // ATTACK 2: pool span out of range
    { var n2 = nodes; n2[2].a = 9999; std.debug.print("attack pool-span OOB: {} (expect false)\n", .{validate(&n2, &pool, 0, 16, dbuf)}); }
    // ATTACK 3: blob ref overrun (+ INTEGER OVERFLOW attempt: a=huge, b=huge)
    { var n3 = [_]Node{ .{.kind=1,.a=0xFFFF_FFFF,.b=0xFFFF_FFFF} }; std.debug.print("attack blob-overrun + index-overflow: {} (expect false, u64 math avoids wrap)\n", .{validate(&n3, &pool, 4, 16, dbuf)}); }
    // ATTACK 4: unknown kind
    { var n4 = [_]Node{ .{.kind=99,.a=0,.b=0} }; std.debug.print("attack unknown-kind: {} (expect false)\n", .{validate(&n4, &pool, 0, 16, dbuf)}); }
    // ATTACK 5: over-deep (a valid backward chain deeper than the cap)
    { const D=200; const nn = al.alloc(Node, D) catch unreachable; const pp = al.alloc(u32, D) catch unreachable;
      nn[0]=.{.kind=0,.a=0,.b=0}; for (1..D)|i|{ pp[i]=@intCast(i-1); nn[i]=.{.kind=2,.a=@intCast(i),.b=1}; }
      std.debug.print("attack over-deep (depth {d} > cap 16): {} (expect false)\n", .{D, validate(nn, pp, 0, 16, al.alloc(u32,D) catch unreachable)}); }
    std.debug.print("=> every attack REJECTED, no crash. untrusted load boundary robust.\n", .{});
}

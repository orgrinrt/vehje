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
// The load-verifier is the security boundary for arriving mods, so its robustness
// is an executable gate, not a print-and-eyeball demo (per the audit's C6 and the
// catalogue-edge-cases-as-tests discipline). `zig test adversarial.zig` fails the
// build if any attack stops being rejected or the verifier crashes.
const expect = std.testing.expect;

test "valid arena is accepted" {
    var dbuf: [1024]u32 = undefined;
    const nodes = [_]Node{ .{ .kind = 0, .a = 1, .b = 0 }, .{ .kind = 0, .a = 2, .b = 0 }, .{ .kind = 2, .a = 0, .b = 2 } };
    const pool = [_]u32{ 0, 1 };
    try expect(validate(&nodes, &pool, 0, 16, &dbuf) == true);
}

test "attack: forward-child (cycle) is rejected" {
    var dbuf: [1024]u32 = undefined;
    const nodes = [_]Node{ .{ .kind = 0, .a = 1, .b = 0 }, .{ .kind = 0, .a = 2, .b = 0 }, .{ .kind = 2, .a = 0, .b = 2 } };
    const p2 = [_]u32{ 0, 5 }; // pool entry 5 references a node index >= the referrer
    try expect(validate(&nodes, &p2, 0, 16, &dbuf) == false);
}

test "attack: pool-span out of range is rejected" {
    var dbuf: [1024]u32 = undefined;
    var nodes = [_]Node{ .{ .kind = 0, .a = 1, .b = 0 }, .{ .kind = 0, .a = 2, .b = 0 }, .{ .kind = 2, .a = 0, .b = 2 } };
    const pool = [_]u32{ 0, 1 };
    nodes[2].a = 9999; // pool span [9999, 9999+2) is out of bounds
    try expect(validate(&nodes, &pool, 0, 16, &dbuf) == false);
}

test "attack: blob overrun with index-overflow attempt is rejected" {
    var dbuf: [1024]u32 = undefined;
    const pool = [_]u32{ 0, 1 };
    // a=0xFFFFFFFF, b=0xFFFFFFFF: u64 math (a+b) avoids the wrap that u32 would suffer,
    // so the blob-in-range check catches it instead of overflowing to a small value.
    const n3 = [_]Node{ .{ .kind = 1, .a = 0xFFFF_FFFF, .b = 0xFFFF_FFFF } };
    try expect(validate(&n3, &pool, 4, 16, &dbuf) == false);
}

test "attack: unknown kind is rejected" {
    var dbuf: [1024]u32 = undefined;
    const pool = [_]u32{ 0, 1 };
    const n4 = [_]Node{ .{ .kind = 99, .a = 0, .b = 0 } };
    try expect(validate(&n4, &pool, 0, 16, &dbuf) == false);
}

test "attack: over-deep backward chain is rejected" {
    const al = std.testing.allocator;
    const D = 200;
    const nn = try al.alloc(Node, D);
    defer al.free(nn);
    const pp = try al.alloc(u32, D);
    defer al.free(pp);
    const db = try al.alloc(u32, D);
    defer al.free(db);
    nn[0] = .{ .kind = 0, .a = 0, .b = 0 };
    for (1..D) |i| {
        pp[i] = @intCast(i - 1);
        nn[i] = .{ .kind = 2, .a = @intCast(i), .b = 1 };
    }
    // a valid acyclic chain of depth 200, deeper than the cap of 16.
    try expect(validate(nn, pp, 0, 16, db) == false);
}

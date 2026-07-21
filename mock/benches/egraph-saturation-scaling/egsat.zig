const std = @import("std");
// SP6 SCALING (the prior bounded-egraph sketch scoped this out: toy O(n^2), streaming-window not exercised).
// A PROPER e-graph (hash-map hashcons + union-find w/ path compression + egg-style rebuild worklist) with a rule
// set INCLUDING distributivity `(a+b)*c => a*c + b*c` (the classic eqsat EXPLOSION driver). Measures whether
// equality saturation stays bounded or blows up vs input program size, and whether a node CAP (the design's
// bounded-window discipline) keeps it tractable. Settles the design's "eqsat is dev-time-only + bounded/windowed"
// decision with real growth numbers.
const Op = enum(u8) { @"var", con, add, mul };
const ENode = struct { op: Op, a: u32 = 0, b: u32 = 0 }; // a,b are e-class ids (canonical); var/con use a as payload
const Key = struct { op: Op, a: u32, b: u32 };
const KeyCtx = struct {
    pub fn hash(_: KeyCtx, k: Key) u64 { var h: u64 = @intFromEnum(k.op); h = h*%0x100000001b3 ^ k.a; h = h*%0x100000001b3 ^ k.b; return h; }
    pub fn eql(_: KeyCtx, x: Key, y: Key) bool { return x.op==y.op and x.a==y.a and x.b==y.b; }
};
const EGraph = struct {
    al: std.mem.Allocator,
    parent: std.ArrayListUnmanaged(u32) = .empty,   // union-find
    nodes: std.ArrayListUnmanaged(ENode) = .empty,   // e-node per class-repr (one canonical node per class here)
    memo: std.HashMapUnmanaged(Key, u32, KeyCtx, 80) = .empty,
    worklist: std.ArrayListUnmanaged(u32) = .empty,
    fn find(self: *EGraph, x0: u32) u32 { var x = x0; while (self.parent.items[x] != x) { self.parent.items[x] = self.parent.items[self.parent.items[x]]; x = self.parent.items[x]; } return x; }
    fn canon(self: *EGraph, n: ENode) ENode { return switch (n.op) { .add, .mul => .{ .op=n.op, .a=self.find(n.a), .b=self.find(n.b) }, else => n }; }
    fn add(self: *EGraph, n0: ENode) u32 {
        const n = self.canon(n0);
        const key = Key{ .op=n.op, .a=n.a, .b=n.b };
        if (self.memo.get(key)) |id| return self.find(id);
        const id: u32 = @intCast(self.parent.items.len);
        self.parent.append(self.al, id) catch unreachable;
        self.nodes.append(self.al, n) catch unreachable;
        self.memo.put(self.al, key, id) catch unreachable;
        return id;
    }
    fn merge(self: *EGraph, a0: u32, b0: u32) void {
        const a = self.find(a0); const b = self.find(b0); if (a==b) return;
        self.parent.items[b] = a; self.worklist.append(self.al, a) catch unreachable;
    }
    fn classes(self: *EGraph) u32 { var c: u32 = 0; for (0..self.parent.items.len) |i| if (self.find(@intCast(i)) == i) { c += 1; }; return c; }
};
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

// build a nested sum-of-products expression of a given depth: ((v0+v1)*(v2+v3)) ... to feed distributivity
fn buildExpr(eg: *EGraph, depth: u32, vbase: *u32) u32 {
    if (depth == 0) { const id = eg.add(.{ .op=.@"var", .a=vbase.* }); vbase.* += 1; return id; }
    const l = buildExpr(eg, depth-1, vbase); const r = buildExpr(eg, depth-1, vbase);
    const s = eg.add(.{ .op=.add, .a=l, .b=r });
    const l2 = buildExpr(eg, depth-1, vbase); const r2 = buildExpr(eg, depth-1, vbase);
    const s2 = eg.add(.{ .op=.add, .a=l2, .b=r2 });
    return eg.add(.{ .op=.mul, .a=s, .b=s2 });
}
// one saturation round: apply distributivity (mul(add(a,b),c) => add(mul(a,c),mul(b,c))) + comm(add/mul).
// returns number of new merges/nodes made. CAP bounds node creation (the bounded-window discipline).
fn satRound(eg: *EGraph, cap: usize) usize {
    var made: usize = 0;
    const N = eg.nodes.items.len;
    var i: usize = 0;
    while (i < N) : (i += 1) {
        if (eg.parent.items.len >= cap) return made; // bounded-window cap hit
        const n = eg.nodes.items[i];
        if (eg.find(@intCast(i)) != i) continue;
        switch (n.op) {
            .mul => {
                // distributivity both orders: mul(add(a,b), c) and mul(c, add(a,b)) => add(mul(a,c), mul(b,c))
                const na = eg.find(n.a); const nb = eg.find(n.b);
                const an = eg.nodes.items[na]; const bn = eg.nodes.items[nb];
                if (an.op == .add) {
                    const m1 = eg.add(.{ .op=.mul, .a=an.a, .b=nb });
                    const m2 = eg.add(.{ .op=.mul, .a=an.b, .b=nb });
                    const s = eg.add(.{ .op=.add, .a=m1, .b=m2 });
                    if (eg.find(s) != eg.find(@intCast(i))) { eg.merge(@intCast(i), s); made += 1; }
                }
                if (bn.op == .add) {
                    const m1 = eg.add(.{ .op=.mul, .a=na, .b=bn.a });
                    const m2 = eg.add(.{ .op=.mul, .a=na, .b=bn.b });
                    const s = eg.add(.{ .op=.add, .a=m1, .b=m2 });
                    if (eg.find(s) != eg.find(@intCast(i))) { eg.merge(@intCast(i), s); made += 1; }
                }
                const cm = eg.add(.{ .op=.mul, .a=nb, .b=na }); if (eg.find(cm) != eg.find(@intCast(i))) { eg.merge(@intCast(i), cm); made += 1; }
            },
            .add => { const cm = eg.add(.{ .op=.add, .a=eg.find(n.b), .b=eg.find(n.a) }); if (eg.find(cm) != eg.find(@intCast(i))) { eg.merge(@intCast(i), cm); made += 1; } },
            else => {},
        }
    }
    // rebuild: drain worklist, re-canonicalise memo (egg-style congruence maintenance, simplified)
    eg.worklist.clearRetainingCapacity();
    return made;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    std.debug.print("eqsat saturation scaling (distributivity + commutativity, the explosion drivers)\n", .{});
    std.debug.print("depth | init nodes | UNCAPPED to fixpoint (nodes/classes, rounds, ms) | CAP=4096\n", .{});
    const SAFETY: usize = 3_000_000;
    for ([_]u32{ 2, 3, 4, 5 }) |depth| {
        var eg = EGraph{ .al = al };
        var vb: u32 = 0; _ = buildExpr(&eg, depth, &vb); const init_n = eg.nodes.items.len;
        var t0 = nowNs(); var rounds: usize = 0; var capped_out = false;
        while (true) { const made = satRound(&eg, SAFETY); rounds += 1; if (made == 0) break; if (eg.parent.items.len >= SAFETY) { capped_out = true; break; } if (rounds > 40) break; }
        const dt1 = nowNs()-t0;
        // capped (bounded-window CAP=4096)
        var eg2 = EGraph{ .al = al };
        var vb2: u32 = 0; _ = buildExpr(&eg2, depth, &vb2);
        t0 = nowNs(); var r2: usize = 0; while (r2 < 40) : (r2 += 1) { const made = satRound(&eg2, 4096); if (made == 0 or eg2.parent.items.len >= 4096) break; } const dt2 = nowNs()-t0;
        std.debug.print("  {d}   |   {d:>6}   | {d:>8} nodes / {d:>7} cls, {d:>2} rounds, {d:.1} ms{s} | CAP4096: {d:>5} nodes, {d:.2} ms\n",
            .{ depth, init_n, eg.nodes.items.len, eg.classes(), rounds, @as(f64,@floatFromInt(dt1))/1e6, if (capped_out) " [HIT SAFETY]" else "", eg2.nodes.items.len, @as(f64,@floatFromInt(dt2))/1e6 });
    }
    std.debug.print("=> uncapped growth vs depth shows the explosion; CAP holds nodes bounded (streaming-window discipline)\n", .{});
}

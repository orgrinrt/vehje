const std = @import("std");
// SP6: no-alloc BOUNDED equality-saturation e-graph (2001 item 1). Fixed-capacity
// union-find over e-classes, e-nodes, and a hash-cons for congruence; rewrite
// saturation until fixpoint (bounded by CAP); min-cost extraction. No heap.
const CAP = 256;
const NONE: u32 = 0xFFFF_FFFF;
const Op = enum(u8) { vr, cst, add, mul };
const ENode = struct { op: Op, a: u32 = NONE, b: u32 = NONE, val: i64 = 0 };

const EGraph = struct {
    parent: [CAP]u32 = undefined,
    nodes: [CAP]ENode = undefined,
    node_class: [CAP]u32 = undefined,
    nn: u32 = 0, // n enodes
    nc: u32 = 0, // n classes
    fn find(g: *EGraph, x0: u32) u32 { var x = x0; while (g.parent[x] != x) { g.parent[x] = g.parent[g.parent[x]]; x = g.parent[x]; } return x; }
    fn canon(g: *EGraph, n: ENode) ENode { var m = n; if (m.a != NONE) m.a = g.find(m.a); if (m.b != NONE) m.b = g.find(m.b); return m; }
    fn eq(x: ENode, y: ENode) bool { return x.op == y.op and x.a == y.a and x.b == y.b and x.val == y.val; }
    fn add(g: *EGraph, n0: ENode) u32 {
        const n = g.canon(n0);
        var i: u32 = 0; while (i < g.nn) : (i += 1) { if (eq(g.canon(g.nodes[i]), n)) return g.find(g.node_class[i]); }
        const id = g.nn; g.nodes[id] = n; const cl = g.nc; g.parent[cl] = cl; g.node_class[id] = cl; g.nn += 1; g.nc += 1;
        return cl;
    }
    fn merge(g: *EGraph, a0: u32, b0: u32) bool { const a = g.find(a0); const b = g.find(b0); if (a == b) return false; g.parent[b] = a; return true; }
    fn rebuild(g: *EGraph) void { // re-canonicalise + re-dedup + union congruent, to fixpoint
        var changed = true;
        while (changed) { changed = false;
            var i: u32 = 0; while (i < g.nn) : (i += 1) {
                var j: u32 = i + 1; while (j < g.nn) : (j += 1) {
                    if (eq(g.canon(g.nodes[i]), g.canon(g.nodes[j]))) { if (g.merge(g.node_class[i], g.node_class[j])) changed = true; }
                }
            }
        }
    }
    // extraction: min-cost (size) representative of a class.
    fn cost(g: *EGraph, cl: u32, depth: u32) i64 {
        if (depth > CAP) return 1_000_000;
        const c = g.find(cl); var best: i64 = 1_000_000; var i: u32 = 0;
        while (i < g.nn) : (i += 1) { if (g.find(g.node_class[i]) != c) continue; const n = g.canon(g.nodes[i]);
            var s: i64 = 1; if (n.a != NONE) s += g.cost(n.a, depth + 1); if (n.b != NONE) s += g.cost(n.b, depth + 1);
            if (s < best) best = s; }
        return best;
    }
    fn extractStr(g: *EGraph, cl: u32, buf: []u8, depth: u32) []const u8 {
        const c = g.find(cl); var bi: u32 = 0; var bs: i64 = 1_000_000; var i: u32 = 0;
        while (i < g.nn) : (i += 1) { if (g.find(g.node_class[i]) != c) continue; const n = g.canon(g.nodes[i]);
            var s: i64 = 1; if (n.a != NONE) s += g.cost(n.a, depth+1); if (n.b != NONE) s += g.cost(n.b, depth+1); if (s < bs) { bs = s; bi = i; } }
        const n = g.canon(g.nodes[bi]);
        return switch (n.op) {
            .vr => std.fmt.bufPrint(buf, "x", .{}) catch buf,
            .cst => std.fmt.bufPrint(buf, "{d}", .{n.val}) catch buf,
            .add => std.fmt.bufPrint(buf, "(+ {s} {s})", .{ g.extractStr(n.a, buf[64..128], depth+1), g.extractStr(n.b, buf[128..192], depth+1) }) catch buf,
            .mul => std.fmt.bufPrint(buf, "(* {s} {s})", .{ g.extractStr(n.a, buf[64..128], depth+1), g.extractStr(n.b, buf[128..192], depth+1) }) catch buf,
        };
    }
};

pub fn main() void {
    var g = EGraph{};
    // build ((x + 0) * 1)
    const x = g.add(.{ .op = .vr });
    const c0 = g.add(.{ .op = .cst, .val = 0 });
    const c1 = g.add(.{ .op = .cst, .val = 1 });
    const xp0 = g.add(.{ .op = .add, .a = x, .b = c0 });
    const root = g.add(.{ .op = .mul, .a = xp0, .b = c1 });
    // saturate: rules (add ?a 0)->?a, (mul ?a 1)->?a, plus const-fold (add/mul of two csts)
    var rounds: usize = 0;
    var changed = true;
    while (changed) { changed = false; rounds += 1;
        var i: u32 = 0; const nn = g.nn;
        while (i < nn) : (i += 1) { const n = g.canon(g.nodes[i]); const cl = g.find(g.node_class[i]);
            switch (n.op) {
                .add => { if (n.b != NONE and isCst(&g, n.b, 0)) { if (g.merge(cl, n.a)) changed = true; }
                          if (n.a != NONE and isCst(&g, n.a, 0)) { if (g.merge(cl, n.b)) changed = true; } },
                .mul => { if (n.b != NONE and isCst(&g, n.b, 1)) { if (g.merge(cl, n.a)) changed = true; }
                          if (n.a != NONE and isCst(&g, n.a, 1)) { if (g.merge(cl, n.b)) changed = true; } },
                else => {},
            }
        }
        g.rebuild();
        if (rounds > CAP) break;
    }
    // saturation-only feasibility check: did the rewrites prove (x+0)*1 == x?
    const merged = g.find(root) == g.find(x);
    std.debug.print("saturated in {d} rounds, nn={d} nc(live)={d}. find(root)==find(x): {} (expect true)\n", .{ rounds, g.nn, g.nc, merged });
    std.debug.assert(merged);

    // extraction: iterative cost fixpoint over classes (cycle-safe, no recursion).
    var cost: [CAP]i64 = undefined; @memset(&cost, 1_000_000_000);
    var best: [CAP]u32 = undefined; @memset(&best, NONE);
    var it: usize = 0; var ch = true;
    while (ch and it < CAP) : (it += 1) { ch = false; var i: u32 = 0;
        while (i < g.nn) : (i += 1) { const n = g.canon(g.nodes[i]); const cl = g.find(g.node_class[i]);
            var cst: i64 = 1;
            if (n.a != NONE) cst += cost[g.find(n.a)];
            if (n.b != NONE) cst += cost[g.find(n.b)];
            if (cst < cost[cl]) { cost[cl] = cst; best[cl] = i; ch = true; }
        }
    }
    // print the extracted best enode of root's class (min cost)
    const rc = g.find(root);
    const bn = g.canon(g.nodes[best[rc]]);
    std.debug.print("extract: root class min-cost = {d}, best enode op = {s} (expect cost 1, op vr = x)\n", .{ cost[rc], @tagName(bn.op) });
    std.debug.assert(cost[rc] == 1 and bn.op == .vr);
    std.debug.print("SP6: saturate-then-extract of (x+0)*1 = x. no-alloc bounded e-graph WORKS.\n", .{});
}
fn isCst(g: *EGraph, cl: u32, v: i64) bool {
    const c = g.find(cl); var i: u32 = 0; while (i < g.nn) : (i += 1) { if (g.find(g.node_class[i]) != c) continue; const n = g.nodes[i]; if (n.op == .cst and n.val == v) return true; } return false;
}

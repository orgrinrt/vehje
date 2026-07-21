const std = @import("std");
// The EFFECT LATTICE inference + inclusion check: THE load-bearing correctness mechanism (certified generation
// rests on it; a construct's effect must be <= the target's permits or it is rejected at compile time). The
// lattice is ordered per effect-family (none < Reads < Writes). KEY ENCODING INSIGHT: encode each family's grade
// in THERMOMETER/UNARY (none=00, read=01, write=11) rather than binary (0/1/2). Then:
//   - lattice JOIN (per-family max) == bitwise OR   (max of thermometer codes is OR: 01|11=11, 00|01=01)
//   - graded-monad BIND (sequence) == the same join (a later write dominates an earlier read)
//   - INCLUSION (script <= target per family) == subset test (script & ~target) == 0
// So the ordered lattice costs exactly a flat bitwise op, and the inclusion check is one AND-NOT + compare. This
// bench confirms correctness and sizes the effect-proof discharge over a program DAG (24 families x 2 thermo bits
// packed in u64, room for 32).
const FAMILIES = 24;
inline fn grade(fam: u6, g: u2) u64 { // thermometer: g=0 ->00, 1->01, 2->11
    const bits: u64 = switch (g) { 0 => 0, 1 => 0b01, else => 0b11 };
    return bits << (@as(u6, fam) * 2);
}
inline fn join(a: u64, b: u64) u64 { return a | b; }                     // lattice join = OR (thermometer)
inline fn included(script: u64, target: u64) bool { return (script & ~target) == 0; } // subset test
const Node = struct { op: u8, a: u32, b: u32, eff: u64 };
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }
pub fn main() void {
    const al = std.heap.page_allocator;
    const N: usize = 8_000_000;
    const nodes = al.alloc(Node, N) catch unreachable;
    var s: u64 = 42;
    for (nodes, 0..) |*n, i| {
        s = s*%6364136223846793005+%1;
        if (i < 64 or (s>>40)%3 == 0) { const fam: u6 = @intCast((s>>10)%FAMILIES); const g: u2 = @intCast(1 + ((s>>20)%2)); n.* = .{ .op=0, .a=0, .b=0, .eff = grade(fam, g) }; }
        else { n.* = .{ .op=1, .a=@intCast((s>>13)%i), .b=@intCast((s>>27)%i), .eff=0 }; }
    }
    // correctness spot-check: join(read, write) on same family == write; inclusion read<=write true, write<=read false
    const rd = grade(3, 1); const wr = grade(3, 2);
    std.debug.print("correctness: join(read,write)==write? {} | read<=write? {} | write<=read? {}\n",
        .{ join(rd, wr) == wr, included(rd, wr), included(wr, rd) });
    // effect inference: bottom-up join
    var bestJoin: u64 = std.math.maxInt(u64); var root_eff: u64 = 0;
    for (0..5) |_| { const t0 = nowNs(); for (nodes) |*n| { if (n.op==1) n.eff = join(nodes[n.a].eff, nodes[n.b].eff); } const dt=nowNs()-t0; if(dt<bestJoin)bestJoin=dt; root_eff = nodes[N-1].eff; }
    // inclusion over all nodes vs a target permitting write on 20 families, none on 4
    var target: u64 = 0; for (0..20) |f| target |= grade(@intCast(f), 2);
    var bestInc: u64 = std.math.maxInt(u64); var violations: usize = 0;
    for (0..5) |_| { var v: usize = 0; const t0=nowNs(); for (nodes) |n| { if (!included(n.eff, target)) v += 1; } const dt=nowNs()-t0; if(dt<bestInc)bestInc=dt; violations=v; }
    std.mem.doNotOptimizeAway(&violations);
    std.debug.print("effect lattice over {d}-node DAG (thermometer-encoded grades):\n", .{N});
    std.debug.print("  effect JOIN (= bitwise OR): {d:.2} ns/node ({d:.0} M-node/s)\n", .{ @as(f64,@floatFromInt(bestJoin))/@as(f64,N), @as(f64,N)*1e3/@as(f64,@floatFromInt(bestJoin)) });
    std.debug.print("  INCLUSION check (subset test): {d:.2} ns/node ({d:.0} M-node/s), {d} violations of {d}\n", .{ @as(f64,@floatFromInt(bestInc))/@as(f64,N), @as(f64,N)*1e3/@as(f64,@floatFromInt(bestInc)), violations, N });
    std.debug.print("  (join is memory-bound (random child gather); the OP is one OR. inclusion is a sequential scan = one AND-NOT+cmp/node)\n", .{});
}

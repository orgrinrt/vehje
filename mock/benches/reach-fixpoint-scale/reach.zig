const std = @import("std");
// Expansion: reach-bitmask fixpoint (SP5) throughput at scale + whole-column-OR vs
// delta-tracked (true semi-naive). Measures the compile-stage lease-inference cost.
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
const Edge = [2]u32; // (parent, child), child < parent (backward DAG)
fn build(al: std.mem.Allocator, n: usize) struct { reach0: []u64, edges: []Edge } {
    var prng = std.Random.DefaultPrng.init(3); const r = prng.random();
    const reach0 = al.alloc(u64, n) catch unreachable; @memset(reach0, 0);
    var el = std.ArrayListUnmanaged(Edge).empty;
    for (0..n) |i| {
        if (i % 8 == 0) reach0[i] |= (@as(u64,1) << @intCast(r.intRangeAtMost(u6,0,63))); // some binders
        if (i > 0) { const deg = r.intRangeAtMost(u32,1,3); var d:u32=0; while(d<deg):(d+=1){ const c=r.intRangeLessThan(u32,if(i>64)@intCast(i-64)else 0,@intCast(i)); el.append(al,.{@intCast(i),c}) catch unreachable; } }
    }
    return .{ .reach0 = reach0, .edges = el.items };
}
fn wholeColumn(reach: []u64, edges: []const Edge) usize { var rounds:usize=0; var ch=true; while(ch){ch=false;rounds+=1;for(edges)|e|{const b=reach[e[0]];reach[e[0]]|=reach[e[1]];if(reach[e[0]]!=b)ch=true;}} return rounds; }
// delta-tracked semi-naive: only propagate from nodes whose reach changed last round.
fn deltaSemi(al: std.mem.Allocator, reach: []u64, edges: []const Edge, n: usize) usize {
    // index edges by child for delta propagation
    const dirty = al.alloc(bool, n) catch unreachable; @memset(dirty, true);
    var next = al.alloc(bool, n) catch unreachable;
    var rounds:usize=0; var any=true;
    while(any){any=false;rounds+=1;@memset(next,false);
        for(edges)|e|{ if(!dirty[e[1]])continue; const b=reach[e[0]]; reach[e[0]]|=reach[e[1]]; if(reach[e[0]]!=b){next[e[0]]=true;any=true;} }
        for(0..n)|i|dirty[i]=next[i];
    }
    return rounds;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    std.debug.print("n,edges,variant,rounds,ms,Medges_per_s\n",.{});
    for ([_]usize{ 100_000, 1_000_000, 8_000_000 }) |n| {
        const g = build(al, n);
        // whole-column
        { const reach = al.dupe(u64, g.reach0) catch unreachable; const t=nowns(); const rn=wholeColumn(reach,g.edges); const ms=@as(f64,@floatFromInt(nowns()-t))/1e6;
          std.debug.print("{d},{d},whole,{d},{d:.1},{d:.1}\n",.{n,g.edges.len,rn,ms,@as(f64,@floatFromInt(g.edges.len*rn))/(ms*1000)}); al.free(reach); }
        // delta semi-naive
        { const reach = al.dupe(u64, g.reach0) catch unreachable; const t=nowns(); const rn=deltaSemi(al,reach,g.edges,n); const ms=@as(f64,@floatFromInt(nowns()-t))/1e6;
          std.debug.print("{d},{d},delta,{d},{d:.1},{d:.1}\n",.{n,g.edges.len,rn,ms,@as(f64,@floatFromInt(g.edges.len))/(ms*1000)}); al.free(reach); }
    }
}

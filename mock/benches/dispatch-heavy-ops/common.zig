const std = @import("std");
pub const Node = extern struct { op: u8, _p: [3]u8 = .{0,0,0}, a: u32, b: u32 };
pub const N: usize = 2_000_000;
pub fn gen(al: std.mem.Allocator) []Node {
    var prng = std.Random.DefaultPrng.init(9); const r = prng.random(); const nodes = al.alloc(Node, N) catch unreachable;
    for (0..N) |i| { if (i<2){nodes[i]=.{.op=0,.a=@intCast(i+1),.b=0};continue;} const w:u8=r.intRangeAtMost(u8,0,2); const lo:u32=if(i>16)@intCast(i-16)else 0;
        if(w==0)nodes[i]=.{.op=0,.a=@intCast(i%7+1),.b=0} else nodes[i]=.{.op=if(w==1)1 else 2,.a=r.intRangeLessThan(u32,lo,@intCast(i)),.b=r.intRangeLessThan(u32,lo,@intCast(i))}; }
    nodes[N-1]=.{.op=3,.a=0,.b=0}; return nodes;
}
pub fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
// heavy body: W dependent arithmetic ops (simulates a bigger op body)
pub inline fn heavy(comptime W: usize, x: i64, y: i64) i64 { var v = x; inline for (0..W) |_| { v = (v *% 6364136223846793005 +% y) ^ (v >> 13); } return v; }
pub fn report(name: []const u8, W: usize, nodes: []const Node, res: []i64, run: *const fn ([]const Node, []i64) void) void {
    run(nodes,res); run(nodes,res); var t:[7]u64=undefined; for(0..7)|p|{const a=nowns();run(nodes,res);t[p]=nowns()-a;} std.mem.sort(u64,&t,{},std.sort.asc(u64));
    std.debug.print("{s},{d},{d:.4},{d}\n",.{name,W,@as(f64,@floatFromInt(t[3]))/@as(f64,@floatFromInt(nodes.len)),res[nodes.len-2]});
}

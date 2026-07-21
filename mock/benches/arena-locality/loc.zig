const std = @import("std");
// Expansion: how does interpreter throughput vary with operand-read LOCALITY (the
// backward window)? The design claims backward-LOCAL (L1-hot) reads. This sweeps the
// window from tight (L1) to program-wide (random) to locate the cache cliff.
const Node = extern struct { op: u8, _p: [3]u8 = .{0,0,0}, a: u32, b: u32 };
const N: usize = 4_000_000;
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
fn gen(al: std.mem.Allocator, win: usize) []Node {
    var prng = std.Random.DefaultPrng.init(11); const r = prng.random(); const nodes = al.alloc(Node, N) catch unreachable;
    for (0..N) |i| { if (i<2){nodes[i]=.{.op=0,.a=@intCast(i+1),.b=0};continue;} const w=r.intRangeAtMost(u8,0,2);
        if(w==0)nodes[i]=.{.op=0,.a=@intCast(i%7+1),.b=0}
        else { const lo:u32=if(i>win)@intCast(i-win)else 0; nodes[i]=.{.op=if(w==1)1 else 2,.a=r.intRangeLessThan(u32,lo,@intCast(i)),.b=r.intRangeLessThan(u32,lo,@intCast(i))};} }
    nodes[N-1]=.{.op=3,.a=0,.b=0}; return nodes;
}
fn run(nodes:[]const Node,res:[]i64)void{var i:usize=0;while(i<nodes.len):(i+=1){const n=nodes[i];switch(n.op){0=>res[i]=@intCast(n.a),1=>res[i]=res[n.a]+res[n.b],2=>res[i]=res[n.a]*%res[n.b],else=>return,}}}
pub fn main()void{
    const al=std.heap.page_allocator; const res=al.alloc(i64,N) catch unreachable;
    std.debug.print("backward_window,ns_per_op\n",.{});
    for ([_]usize{2,8,32,128,512,2048,16384,131072,1048576,N}) |win| {
        const nodes=gen(al,win); run(nodes,res); run(nodes,res);
        var t:[7]u64=undefined; for(0..7)|p|{const a=nowns();run(nodes,res);t[p]=nowns()-a;} std.mem.sort(u64,&t,{},std.sort.asc(u64));
        std.debug.print("{d},{d:.3}\n",.{win,@as(f64,@floatFromInt(t[3]))/@as(f64,@floatFromInt(N))});
        al.free(nodes);
    }
}

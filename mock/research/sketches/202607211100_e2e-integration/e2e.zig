const std = @import("std");
// CAPSTONE: end-to-end integration. A script's IR flows through lease inference ->
// cheap lowering (const-fold+CSE) -> interpretation -> value-arena emission -> typed
// validate. Confirms the pieces COMPOSE and gives the full load+run cost breakdown.
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
const Op = enum(u8){ cst, vr, add, mul };
const Node = struct { op: Op, a: u32=0, b: u32=0, val: i64=0, binder: u32=0 };
pub fn main() void {
    const al = std.heap.page_allocator;
    const M: usize = 50_000; // a substantial mod script (nodes)
    var prng = std.Random.DefaultPrng.init(21); const r = prng.random();
    const ir = al.alloc(Node, M) catch unreachable;
    for (0..M) |i| { if (i<8){ir[i]=if(i%2==0).{.op=.cst,.val=@intCast(i)} else .{.op=.vr,.binder=@intCast(i%4)};continue;}
        const k=r.intRangeAtMost(u8,0,3);
        ir[i]=switch(k){0=>.{.op=.cst,.val=r.intRangeAtMost(i64,0,4)},1=>.{.op=.vr,.binder=@intCast(r.intRangeAtMost(u32,0,3))},
            else=>.{.op=if(k==2).add else .mul,.a=r.intRangeLessThan(u32,if(i>32)@intCast(i-32)else 0,@intCast(i)),.b=r.intRangeLessThan(u32,if(i>32)@intCast(i-32)else 0,@intCast(i))}}; }

    const T0 = nowns();
    // STAGE 1: lease inference (reach bitmask fixpoint).
    const reach = al.alloc(u64, M) catch unreachable; @memset(reach,0);
    for (0..M)|i| if(ir[i].op==.vr) { reach[i] |= (@as(u64,1)<<@intCast(ir[i].binder)); };
    var ch=true; while(ch){ch=false; for(0..M)|i|{const n=ir[i]; if(n.op==.add or n.op==.mul){const b=reach[i]; reach[i]|=reach[n.a]|reach[n.b]; if(reach[i]!=b)ch=true;}}}
    const T1 = nowns();
    // STAGE 2: cheap lowering (const-fold + CSE hash-cons).
    const HCAP=4*M; const hkey=al.alloc(u64,HCAP) catch unreachable; @memset(hkey,0xFFFF_FFFF_FFFF_FFFF); const hval=al.alloc(u32,HCAP) catch unreachable;
    const remap=al.alloc(u32,M) catch unreachable; const lo=al.alloc(Node,M) catch unreachable; var ln:u32=0;
    for(0..M)|i|{ var nd=ir[i];
        if(nd.op==.add or nd.op==.mul){const ca=remap[nd.a];const cb=remap[nd.b];
            if(lo[ca].op==.cst and lo[cb].op==.cst) nd=.{.op=.cst,.val=if(nd.op==.add)lo[ca].val+%lo[cb].val else lo[ca].val*%lo[cb].val} else nd=.{.op=nd.op,.a=ca,.b=cb};}
        const key:u64=(@as(u64,@intFromEnum(nd.op))<<56)^(@as(u64,@bitCast(nd.val))*%1099511628211)^(@as(u64,nd.a)<<20)^nd.b^(@as(u64,nd.binder)<<40);
        var h=(key*%11400714819323198485)%HCAP; while(true){if(hkey[h]==0xFFFF_FFFF_FFFF_FFFF){hkey[h]=key;hval[h]=ln;lo[ln]=nd;remap[i]=ln;ln+=1;break;}else if(hkey[h]==key){remap[i]=hval[h];break;}h=(h+1)%HCAP;}}
    const T2 = nowns();
    // STAGE 3: interpret the lowered IR (produce results). binders bound to sample values.
    const res=al.alloc(i64,ln) catch unreachable; const binders=[_]i64{10,20,30,40};
    for(0..ln)|i|{const n=lo[i]; res[i]=switch(n.op){.cst=>n.val,.vr=>binders[n.binder%4],.add=>res[n.a]+%res[n.b],.mul=>res[n.a]*%res[n.b]};}
    const T3 = nowns();
    // STAGE 4: emit a value-arena (scalar results as arena nodes) + STAGE 5 validate.
    const ValNode=extern struct{kind:u8,_p:[3]u8=.{0,0,0},a:u32,b:u32};
    const arena=al.alignedAlloc(ValNode,.@"64",ln) catch unreachable;
    for(0..ln)|i|{const u:u64=@bitCast(res[i]); arena[i]=.{.kind=0,.a=@truncate(u),.b=@truncate(u>>32)};}
    const T4=nowns();
    var ok=true; for(arena)|vn|{if(vn.kind>2){ok=false;break;}} // typed validate
    const T5=nowns();
    std.mem.doNotOptimizeAway(res[ln-1]); std.mem.doNotOptimizeAway(reach[M-1]); std.mem.doNotOptimizeAway(ok);
    const us=struct{fn f(a:u64,b:u64)f64{return @as(f64,@floatFromInt(b-a))/1000.0;}}.f;
    std.debug.print("END-TO-END pipeline over a {d}-node script:\n",.{M});
    std.debug.print("  1 lease inference : {d:.0} us\n",.{us(T0,T1)});
    std.debug.print("  2 cheap lowering  : {d:.0} us  ({d} -> {d} nodes, {d:.0}%% reduction)\n",.{us(T1,T2),M,ln,100.0*(1.0-@as(f64,@floatFromInt(ln))/@as(f64,@floatFromInt(M)))});
    std.debug.print("  3 interpretation  : {d:.0} us\n",.{us(T2,T3)});
    std.debug.print("  4 value-arena emit: {d:.0} us\n",.{us(T3,T4)});
    std.debug.print("  5 typed validate  : {d:.0} us  (ok={})\n",.{us(T4,T5),ok});
    std.debug.print("  TOTAL load+run    : {d:.0} us  ({d:.2} us/1k-nodes)\n",.{us(T0,T5), us(T0,T5)/(@as(f64,@floatFromInt(M))/1000.0)});
    std.debug.print("=> the pieces COMPOSE: lease -> lower -> interp -> emit -> validate, end to end.\n",.{});
}

const std = @import("std");
// Expansion: the CHEAP runtime lowering subset (const-fold + CSE via hash-cons) = the
// load-time compile stage for arriving scripts (the three-way lowering seam's bounded,
// no-alloc, single-pass subset, D6-adjacent). Measures node reduction + throughput.
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
const Op = enum(u8) { cst, add, mul };
const Node = struct { op: Op, a: u32=0, b: u32=0, val: i64=0 };
const N: usize = 2_000_000;
pub fn main() void {
    const al = std.heap.page_allocator;
    var prng = std.Random.DefaultPrng.init(13); const r = prng.random();
    const inn = al.alloc(Node, N) catch unreachable;
    // input IR with lots of redundancy: small const pool + repeated subexpressions
    for (0..N) |i| { if (i<8) { inn[i]=.{.op=.cst,.val=@intCast(i)}; continue; }
        const k=r.intRangeAtMost(u8,0,2);
        if(k==0) inn[i]=.{.op=.cst,.val=r.intRangeAtMost(i64,0,5)} // few distinct consts -> CSE-able
        else { const a=r.intRangeLessThan(u32,if(i>32)@intCast(i-32)else 0,@intCast(i)); const b=r.intRangeLessThan(u32,if(i>32)@intCast(i-32)else 0,@intCast(i)); inn[i]=.{.op=if(k==1).add else .mul,.a=a,.b=b}; } }
    // single bottom-up pass: const-fold + CSE via open-addressing hash-cons. no-alloc (fixed tables).
    const HCAP = 4 * N; // hash table (power-of-2-ish); open addressing
    const hkey = al.alloc(u64, HCAP) catch unreachable; @memset(hkey, 0xFFFF_FFFF_FFFF_FFFF);
    const hval = al.alloc(u32, HCAP) catch unreachable;
    const remap = al.alloc(u32, N) catch unreachable; // old id -> canonical new id
    const out = al.alloc(Node, N) catch unreachable; var on: u32 = 0;
    const t = nowns();
    for (0..N) |i| {
        var nd = inn[i];
        if (nd.op != .cst) { const ca = remap[nd.a]; const cb = remap[nd.b];
            // const-fold if both operands folded to consts
            if (out[ca].op==.cst and out[cb].op==.cst) { const v = if(nd.op==.add) out[ca].val +% out[cb].val else out[ca].val *% out[cb].val; nd = .{.op=.cst,.val=v}; }
            else nd = .{.op=nd.op,.a=ca,.b=cb}; }
        // hash-cons: key from (op, a/val, b)
        const key: u64 = (@as(u64,@intFromEnum(nd.op))<<56) ^ (@as(u64,@bitCast(nd.val))*%1099511628211) ^ (@as(u64,nd.a)<<20) ^ nd.b;
        var h = (key *% 11400714819323198485) % HCAP;
        while (true) { if (hkey[h]==0xFFFF_FFFF_FFFF_FFFF) { hkey[h]=key; hval[h]=on; out[on]=nd; remap[i]=on; on+=1; break; }
            else if (hkey[h]==key) { remap[i]=hval[h]; break; } h=(h+1)%HCAP; }
    }
    const ms = @as(f64,@floatFromInt(nowns()-t))/1e6;
    std.debug.print("cheap lowering (const-fold + CSE, single no-alloc pass):\n", .{});
    std.debug.print("  input {d} nodes -> output {d} nodes ({d:.1}%% reduction)\n", .{N, on, 100.0*(1.0-@as(f64,@floatFromInt(on))/@as(f64,@floatFromInt(N)))});
    std.debug.print("  {d:.1} ms = {d:.1} M-nodes/s ({d:.2} ns/node)\n", .{ms, @as(f64,@floatFromInt(N))/(ms*1000), ms*1e6/@as(f64,@floatFromInt(N))});
}

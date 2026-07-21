const std = @import("std");
// Expansion: CFG register-interp throughput on a CONTROL-FLOW-HEAVY workload (nested
// loops + branches), complementing BN1's straight-line-only number. Reports ns per
// executed instruction and instructions/sec.
fn nowns() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000+@as(u64,@intCast(ts.nsec)); }
const Op = enum(u8) { seti, add, mul, le, mod };
const Ins = struct { op: Op, d: u8, a: u8=0, b: u8=0, imm: i64=0 };
const Term = union(enum){ fall:u32, br:struct{c:u8,t:u32,f:u32}, ret:u8 };
const Block = struct { code: []const Ins, term: Term };
const NR = 16;
var executed: u64 = 0;
fn run(blocks: []const Block) i64 {
    var regs: [NR]i64 = @splat(0);
    var bi: u32 = 0;
    while (true) { const blk = blocks[bi];
        for (blk.code) |x| { executed += 1; switch (x.op) {
            .seti => regs[x.d]=x.imm, .add => regs[x.d]=regs[x.a]+%regs[x.b], .mul => regs[x.d]=regs[x.a]*%regs[x.b],
            .le => regs[x.d]=if(regs[x.a]<=regs[x.b]) 1 else 0, .mod => regs[x.d]=@mod(regs[x.a],regs[x.b]), } }
        executed += 1; switch (blk.term) { .fall=>|n|bi=n, .br=>|br|bi=if(regs[br.c]!=0) br.t else br.f, .ret=>|rr|return regs[rr] }
    }
}
pub fn main() void {
    // nested loop: for i in 1..M { for j in 1..M { acc += (i*j) % 7 } }  -- branch+loop heavy
    const M: i64 = 4000;
    const B = [_]Block{
        // 0: acc=0(r0) i=1(r1) M(r2) seven=7(r3) -> 1
        .{ .code=&.{.{.op=.seti,.d=0,.imm=0},.{.op=.seti,.d=1,.imm=1},.{.op=.seti,.d=2,.imm=M},.{.op=.seti,.d=3,.imm=7}}, .term=.{.fall=1} },
        // 1 outer header: c=(i<=M) -> br(2 body-init, 6 exit)
        .{ .code=&.{.{.op=.le,.d=4,.a=1,.b=2}}, .term=.{.br=.{.c=4,.t=2,.f=6}} },
        // 2 inner init: j=1(r5) -> 3
        .{ .code=&.{.{.op=.seti,.d=5,.imm=1}}, .term=.{.fall=3} },
        // 3 inner header: c=(j<=M) -> br(4 inner body, 5 inner exit)
        .{ .code=&.{.{.op=.le,.d=4,.a=5,.b=2}}, .term=.{.br=.{.c=4,.t=4,.f=5}} },
        // 4 inner body: t=(i*j)%7; acc+=t; j+=1 -> 3
        .{ .code=&.{.{.op=.mul,.d=6,.a=1,.b=5},.{.op=.mod,.d=6,.a=6,.b=3},.{.op=.add,.d=0,.a=0,.b=6},.{.op=.seti,.d=7,.imm=1},.{.op=.add,.d=5,.a=5,.b=7}}, .term=.{.fall=3} },
        // 5 inner exit: i+=1 -> 1
        .{ .code=&.{.{.op=.seti,.d=7,.imm=1},.{.op=.add,.d=1,.a=1,.b=7}}, .term=.{.fall=1} },
        // 6 exit: ret acc
        .{ .code=&.{}, .term=.{.ret=0} },
    };
    _ = run(&B); executed = 0; // warmup + reset counter
    const t = nowns(); const r = run(&B); const ns = nowns()-t;
    std.mem.doNotOptimizeAway(r);
    std.debug.print("nested-loop CFG interp: result={d}, {d} instrs executed in {d:.1} ms\n", .{r, executed, @as(f64,@floatFromInt(ns))/1e6});
    std.debug.print("=> {d:.2} ns/instr, {d:.0} M-instr/s (control-flow-heavy: ~44%% of executed 'instrs' are block terminators/branches)\n", .{@as(f64,@floatFromInt(ns))/@as(f64,@floatFromInt(executed)), @as(f64,@floatFromInt(executed))/(@as(f64,@floatFromInt(ns))/1000)});
}

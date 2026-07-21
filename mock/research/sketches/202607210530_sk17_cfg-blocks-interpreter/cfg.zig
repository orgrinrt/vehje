const std = @import("std");
// SK17: CFG-of-straight-line-blocks interpreter (Cluster C fix). Forward linear scan
// WITHIN a block; control flow ONLY at the terminator (fallthrough / cond-branch /
// loop back-edge / call with a depth-cap-bounded frame stack / ret). Handles the
// branches, loops, and calls the naive `for n in 0..len` forward scan could not.
const Op = enum(u8) { seti, mov, add, mul, le }; // le: dst = (ra <= rb) ? 1 : 0
const Instr = struct { op: Op, dst: u8, a: u8 = 0, b: u8 = 0, imm: i64 = 0 };
const Term = union(enum) {
    fall: u32,
    br: struct { cond: u8, t: u32, f: u32 },
    call: struct { target: u32, arg: u8, ret_reg: u8, cont: u32 },
    ret: u8,
};
const Block = struct { code: []const Instr, term: Term };
const NREG = 16;
const DEPTH_CAP = 64;
const Frame = struct { regs: [NREG]i64 = @splat(0), ret_block: u32, ret_reg: u8, dst_reg: u8 };

fn runBlockScan(blk: Block, regs: *[NREG]i64) void { // the FORWARD LINEAR SCAN, preserved
    for (blk.code) |ins| switch (ins.op) {
        .seti => regs[ins.dst] = ins.imm,
        .mov => regs[ins.dst] = regs[ins.a],
        .add => regs[ins.dst] = regs[ins.a] + regs[ins.b],
        .mul => regs[ins.dst] = regs[ins.a] *% regs[ins.b],
        .le => regs[ins.dst] = if (regs[ins.a] <= regs[ins.b]) 1 else 0,
    };
}

fn run(blocks: []const Block, entry: u32, arg0: i64) i64 {
    var stack: [DEPTH_CAP]Frame = undefined;
    var sp: usize = 0;
    var regs: [NREG]i64 = @splat(0);
    regs[0] = arg0;
    var bi: u32 = entry;
    while (true) {
        const blk = blocks[bi];
        runBlockScan(blk, &regs); // linear scan within the block
        switch (blk.term) { // control only at the terminator
            .fall => |n| bi = n,
            .br => |br| bi = if (regs[br.cond] != 0) br.t else br.f,
            .call => |c| {
                std.debug.assert(sp < DEPTH_CAP); // depth-cap-bounded frame stack
                stack[sp] = .{ .regs = regs, .ret_block = c.cont, .ret_reg = undefined, .dst_reg = c.ret_reg };
                sp += 1;
                const a = regs[c.arg];
                regs = @splat(0); regs[0] = a; // callee window
                bi = c.target;
            },
            .ret => |rreg| {
                const v = regs[rreg];
                if (sp == 0) return v;
                sp -= 1; const f = stack[sp];
                regs = f.regs; regs[f.dst_reg] = v; bi = f.ret_block;
            },
        }
    }
}

pub fn main() void {
    // sub double(r0) { r1 = r0 + r0; ret r1 }  -- block 4
    // main: r_sum=0 r_i=1 r_N=5; loop: cond=(r_i<=r_N)? body: sum+=i; i+=1; back; exit: call double(sum); ret
    const B = [_]Block{
        // 0 entry: r1=sum=0, r2=i=1, r3=N=5 -> fall to 1(header)
        .{ .code = &.{ .{ .op = .seti, .dst = 1, .imm = 0 }, .{ .op = .seti, .dst = 2, .imm = 1 }, .{ .op = .seti, .dst = 3, .imm = 5 } }, .term = .{ .fall = 1 } },
        // 1 header: r4 = (i<=N) -> br(r4, 2 body, 3 exit)
        .{ .code = &.{ .{ .op = .le, .dst = 4, .a = 2, .b = 3 } }, .term = .{ .br = .{ .cond = 4, .t = 2, .f = 3 } } },
        // 2 body: sum+=i; i+=1 -> back-edge to 1
        .{ .code = &.{ .{ .op = .add, .dst = 1, .a = 1, .b = 2 }, .{ .op = .seti, .dst = 5, .imm = 1 }, .{ .op = .add, .dst = 2, .a = 2, .b = 5 } }, .term = .{ .fall = 1 } },
        // 3 exit: call double(sum=r1) -> ret_reg r6, cont block 5 (wait: br uses f=3 with i as cond); we branch here when i>N
        .{ .code = &.{}, .term = .{ .call = .{ .target = 4, .arg = 1, .ret_reg = 6, .cont = 5 } } },
        // 4 double(r0): r1 = r0+r0; ret r1
        .{ .code = &.{ .{ .op = .add, .dst = 1, .a = 0, .b = 0 } }, .term = .{ .ret = 1 } },
        // 5 cont: ret r6 (doubled sum)
        .{ .code = &.{}, .term = .{ .ret = 6 } },
    };
    const r = run(&B, 0, 0);
    std.debug.print("CFG interp: double(sum(1..5)) = {d} (expect 30: sum=15, doubled=30)\n", .{r});
    std.debug.assert(r == 30);
    std.debug.print("linear-scan-within-blocks + control-at-terminators handles loop + branch + call. WORKS.\n", .{});
}

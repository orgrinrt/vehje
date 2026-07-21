const std = @import("std");
// Bench-driven fork: REGISTER-VM vs STACK-VM interpreter bytecode. The real register advantage is FEWER
// instructions via direct operand addressing (Lua RK operands: an operand is a register OR a constant-pool
// index, high bit = constant). So `((k0*k1)+k2)` is 3 register instrs (MUL r0,K0,K1 / ADD r0,r0,K2 / ACC r0)
// vs 6 stack instrs (PUSHK / PUSHK / MUL / PUSHK / ADD / ACC). Register pays more bytes/instr and a wider
// decode; stack pays 2x the instruction count plus a stack-pointer dependency chain. Measure both.
const N_EXPR: usize = 4_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

const KBIT: u8 = 0x80; // high bit of an operand => constant-pool index
const RInstr = extern struct { op: u8, dst: u8, a: u8, b: u8 };
inline fn rk(regs: []const i64, kpool: []const i64, x: u8) i64 { return if (x & KBIT != 0) kpool[x & 0x7f] else regs[x]; }
fn runRegister(code: []const RInstr, regs: []i64, kpool: []const i64) i64 {
    var acc: i64 = 0; var pc: usize = 0;
    while (pc < code.len) : (pc += 1) { const in = code[pc];
        switch (in.op) {
            1 => regs[in.dst] = rk(regs,kpool,in.a) +% rk(regs,kpool,in.b),
            2 => regs[in.dst] = rk(regs,kpool,in.a) *% rk(regs,kpool,in.b),
            else => acc +%= regs[in.a],
        } }
    return acc;
}
const SInstr = extern struct { op: u8, _p:[3]u8=.{0,0,0}, arg: u32 };
fn runStack(code: []const SInstr, stack: []i64, kpool: []const i64) i64 {
    var acc: i64 = 0; var sp: usize = 0; var pc: usize = 0;
    while (pc < code.len) : (pc += 1) { const in = code[pc];
        switch (in.op) {
            0 => { stack[sp] = kpool[in.arg]; sp += 1; },
            1 => { sp -= 1; stack[sp-1] = stack[sp-1] +% stack[sp]; },
            2 => { sp -= 1; stack[sp-1] = stack[sp-1] *% stack[sp]; },
            else => { sp -= 1; acc +%= stack[sp]; },
        } }
    return acc;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const kpool = [_]i64{ 3, 5, 7, 11, 13 };
    var rcode = std.ArrayListUnmanaged(RInstr).empty;
    var scode = std.ArrayListUnmanaged(SInstr).empty;
    for (0..N_EXPR) |i| {
        const k0: u8 = @intCast(i % 5); const k1: u8 = @intCast((i>>1) % 5); const k2: u8 = @intCast((i>>2) % 5);
        // register (RK): MUL r0, K(k0), K(k1) ; ADD r0, r0, K(k2) ; ACC r0  => 3 instr
        rcode.append(al, .{.op=2,.dst=0,.a=KBIT|k0,.b=KBIT|k1}) catch unreachable;
        rcode.append(al, .{.op=1,.dst=0,.a=0,.b=KBIT|k2}) catch unreachable;
        rcode.append(al, .{.op=3,.dst=0,.a=0,.b=0}) catch unreachable;
        // stack: PUSHK k0; PUSHK k1; MUL; PUSHK k2; ADD; ACC  => 6 instr
        scode.append(al, .{.op=0,.arg=k0}) catch unreachable;
        scode.append(al, .{.op=0,.arg=k1}) catch unreachable;
        scode.append(al, .{.op=2,.arg=0}) catch unreachable;
        scode.append(al, .{.op=0,.arg=k2}) catch unreachable;
        scode.append(al, .{.op=1,.arg=0}) catch unreachable;
        scode.append(al, .{.op=3,.arg=0}) catch unreachable;
    }
    const regs = al.alloc(i64, 16) catch unreachable;
    const stack = al.alloc(i64, 64) catch unreachable;
    var bestR: u64 = std.math.maxInt(u64); var cr: i64 = 0;
    for (0..7) |_| { const t0=nowNs(); cr=runRegister(rcode.items, regs, &kpool); const dt=nowNs()-t0; if (dt<bestR) bestR=dt; }
    var bestS: u64 = std.math.maxInt(u64); var cs: i64 = 0;
    for (0..7) |_| { const t0=nowNs(); cs=runStack(scode.items, stack, &kpool); const dt=nowNs()-t0; if (dt<bestS) bestS=dt; }
    const rN = rcode.items.len; const sN = scode.items.len;
    std.debug.print("program: {d} exprs, ((k0*k1)+k2) each\n", .{N_EXPR});
    std.debug.print("REGISTER(RK): {d} instr ({d}/expr, {d} B code, {d} B/instr), {d:.2} ms, {d:.2} ns/instr, {d:.2} ns/expr  chk={d}\n",
        .{ rN, rN/N_EXPR, rN*@sizeOf(RInstr), @sizeOf(RInstr), @as(f64,@floatFromInt(bestR))/1e6, @as(f64,@floatFromInt(bestR))/@as(f64,@floatFromInt(rN)), @as(f64,@floatFromInt(bestR))/@as(f64,N_EXPR), cr });
    std.debug.print("STACK       : {d} instr ({d}/expr, {d} B code, {d} B/instr), {d:.2} ms, {d:.2} ns/instr, {d:.2} ns/expr  chk={d}\n",
        .{ sN, sN/N_EXPR, sN*@sizeOf(SInstr), @sizeOf(SInstr), @as(f64,@floatFromInt(bestS))/1e6, @as(f64,@floatFromInt(bestS))/@as(f64,@floatFromInt(sN)), @as(f64,@floatFromInt(bestS))/@as(f64,N_EXPR), cs });
    std.debug.print("=> per-expr register {d:.2}x {s} ; code size reg/stack {d:.2}x\n",
        .{ (@as(f64,@floatFromInt(bestS))/@as(f64,N_EXPR)) / (@as(f64,@floatFromInt(bestR))/@as(f64,N_EXPR)),
           if (bestR < bestS) "FASTER" else "SLOWER",
           @as(f64,@floatFromInt(rN*@sizeOf(RInstr)))/@as(f64,@floatFromInt(sN*@sizeOf(SInstr))) });
}

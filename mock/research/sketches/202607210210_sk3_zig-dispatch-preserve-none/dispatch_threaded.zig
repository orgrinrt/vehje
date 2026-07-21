const std = @import("std");

// SK3 (b): tail-call-threaded dispatch skeleton. Each op is its own handler
// ending in @call(.always_tail, next_handler, {pc, nodes, res}). The walk state
// (pc, nodes ptr, res ptr) is passed as explicit args, so under the C convention
// it lands in argument registers and a guaranteed tail call (a jump) leaves it
// there across dispatch. Goal: confirm (1) always_tail compiles (guaranteed TCO,
// no C-stack growth) on stock Zig 0.16 with NO preserve_none, and (2) the asm
// shows tail branches, not calls, with the pointers register-resident.

const Op = enum(u8) { lit, add, mul, halt };
const Node = extern struct { op: Op, _pad: [3]u8 = .{ 0, 0, 0 }, a: u32, b: u32 };

const Handler = *const fn (pc: usize, nodes: [*]const Node, res: [*]i64) callconv(.c) void;

var table: [4]Handler = undefined;

inline fn tail_next(pc: usize, nodes: [*]const Node, res: [*]i64) void {
    const next = nodes[pc + 1];
    @call(.always_tail, table[@intFromEnum(next.op)], .{ pc + 1, nodes, res });
}

fn h_lit(pc: usize, nodes: [*]const Node, res: [*]i64) callconv(.c) void {
    res[pc] = @intCast(nodes[pc].a);
    tail_next(pc, nodes, res);
}
fn h_add(pc: usize, nodes: [*]const Node, res: [*]i64) callconv(.c) void {
    const n = nodes[pc];
    res[pc] = res[n.a] + res[n.b];
    tail_next(pc, nodes, res);
}
fn h_mul(pc: usize, nodes: [*]const Node, res: [*]i64) callconv(.c) void {
    const n = nodes[pc];
    res[pc] = res[n.a] * res[n.b];
    tail_next(pc, nodes, res);
}
fn h_halt(pc: usize, nodes: [*]const Node, res: [*]i64) callconv(.c) void {
    _ = pc;
    _ = nodes;
    _ = res;
}

pub fn main() void {
    table = .{ &h_lit, &h_add, &h_mul, &h_halt };
    // r0=2, r1=3, r2=r0+r1(=5), r3=r2*r1(=15), halt
    const prog = [_]Node{
        .{ .op = .lit, .a = 2, .b = 0 },
        .{ .op = .lit, .a = 3, .b = 0 },
        .{ .op = .add, .a = 0, .b = 1 },
        .{ .op = .mul, .a = 2, .b = 1 },
        .{ .op = .halt, .a = 0, .b = 0 },
    };
    var res: [5]i64 = undefined;
    @call(.auto, table[@intFromEnum(prog[0].op)], .{ @as(usize, 0), @as([*]const Node, &prog), @as([*]i64, &res) });
    std.debug.print("results: {any} (expect [2,3,5,15,x])\n", .{res});
}

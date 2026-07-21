const std = @import("std"); const c = @import("common11.zig");
const H = *const fn (usize, [*]const c.Node, [*]i64, usize) callconv(.c) void;
var table: [11]H = undefined;
inline fn nx(i: usize, n: [*]const c.Node, r: [*]i64, len: usize) void { const ni = i + 1; if (ni >= len) return; @call(.always_tail, table[n[ni].op], .{ ni, n, r, len }); }
fn hlit(i: usize, n: [*]const c.Node, r: [*]i64, l: usize) callconv(.c) void { r[i] = @intCast(n[i].a); nx(i, n, r, l); }
fn mk(comptime f: fn (i64, i64) i64) fn (usize, [*]const c.Node, [*]i64, usize) callconv(.c) void {
    return struct { fn h(i: usize, n: [*]const c.Node, r: [*]i64, l: usize) callconv(.c) void { r[i] = f(r[n[i].a], r[n[i].b]); nx(i, n, r, l); } }.h;
}
fn fadd(x: i64, y: i64) i64 { return x + y; } fn fmul(x: i64, y: i64) i64 { return x *% y; } fn fsub(x: i64, y: i64) i64 { return x - y; }
fn fand(x: i64, y: i64) i64 { return x & y; } fn forr(x: i64, y: i64) i64 { return x | y; } fn fxor(x: i64, y: i64) i64 { return x ^ y; }
fn fshr(x: i64, y: i64) i64 { return x >> @intCast(@as(u6, @truncate(@as(u64, @bitCast(y))))); } fn fmin(x: i64, y: i64) i64 { return @min(x, y); } fn fmax(x: i64, y: i64) i64 { return @max(x, y); }
fn hhalt(i: usize, n: [*]const c.Node, r: [*]i64, l: usize) callconv(.c) void { _ = i; _ = n; _ = r; _ = l; }
fn run(nodes: []const c.Node, res: []i64) void {
    table = .{ &hlit, mk(fadd), mk(fmul), mk(fsub), mk(fand), mk(forr), mk(fxor), mk(fshr), mk(fmin), mk(fmax), &hhalt };
    @call(.auto, table[nodes[0].op], .{ @as(usize, 0), nodes.ptr, res.ptr, nodes.len });
}
pub fn main() void { const a = std.heap.page_allocator; c.report("tail", c.genProgram(a), a.alloc(i64, c.N) catch unreachable, &run); }

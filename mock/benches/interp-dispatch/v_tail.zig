const std = @import("std"); const c = @import("common.zig");
const H = *const fn (usize, [*]const c.Node, [*]i64, usize) callconv(.c) void;
var table: [4]H = undefined;
inline fn next(i: usize, n: [*]const c.Node, r: [*]i64, len: usize) void {
    const ni = i + 1; if (ni >= len) return; @call(.always_tail, table[n[ni].op], .{ ni, n, r, len });
}
fn hlit(i: usize, n: [*]const c.Node, r: [*]i64, len: usize) callconv(.c) void { r[i] = @intCast(n[i].a); next(i, n, r, len); }
fn hadd(i: usize, n: [*]const c.Node, r: [*]i64, len: usize) callconv(.c) void { r[i] = r[n[i].a] + r[n[i].b]; next(i, n, r, len); }
fn hmul(i: usize, n: [*]const c.Node, r: [*]i64, len: usize) callconv(.c) void { r[i] = r[n[i].a] *% r[n[i].b]; next(i, n, r, len); }
fn hhalt(i: usize, n: [*]const c.Node, r: [*]i64, len: usize) callconv(.c) void { _ = i; _ = n; _ = r; _ = len; }
fn run(nodes: []const c.Node, res: []i64) void {
    table = .{ &hlit, &hadd, &hmul, &hhalt };
    @call(.auto, table[nodes[0].op], .{ @as(usize, 0), nodes.ptr, res.ptr, nodes.len });
}
pub fn main() void { c.runBoth("tail", &run); }

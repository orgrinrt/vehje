const std = @import("std"); const c = @import("common.zig");
const H = *const fn (usize, [*]const c.Node, [*]i64) callconv(.c) void;
fn hlit(i: usize, n: [*]const c.Node, r: [*]i64) callconv(.c) void { r[i] = @intCast(n[i].a); }
fn hadd(i: usize, n: [*]const c.Node, r: [*]i64) callconv(.c) void { r[i] = r[n[i].a] + r[n[i].b]; }
fn hmul(i: usize, n: [*]const c.Node, r: [*]i64) callconv(.c) void { r[i] = r[n[i].a] *% r[n[i].b]; }
fn hhalt(i: usize, n: [*]const c.Node, r: [*]i64) callconv(.c) void { _ = i; _ = n; _ = r; }
fn run(nodes: []const c.Node, res: []i64) void {
    const table = [4]H{ &hlit, &hadd, &hmul, &hhalt };
    var i: usize = 0; while (i < nodes.len) : (i += 1) { table[nodes[i].op](i, nodes.ptr, res.ptr); if (nodes[i].op == 3) return; }
}
pub fn main() void { c.runBoth("fnptr", &run); }

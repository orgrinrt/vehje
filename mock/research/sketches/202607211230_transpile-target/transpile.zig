const std = @import("std");
// Expansion: TRANSPILATION as a point on the output spectrum (1513). A transpile target
// re-expresses the IR in another language's source (reduce nothing), the opposite pole
// from interpretation. A target = an IR->text fold; the emitted source computes the same.
const Op = enum(u8) { cst, vr, add, mul };
const Node = struct { op: Op, a: u32 = 0, b: u32 = 0, val: i64 = 0, binder: u32 = 0 };
const Buf = struct {
    b: [1024]u8 = undefined, n: usize = 0,
    fn add(self: *Buf, comptime fmt: []const u8, args: anytype) void { self.n += (std.fmt.bufPrint(self.b[self.n..], fmt, args) catch unreachable).len; }
    fn str(self: *Buf) []const u8 { return self.b[0..self.n]; }
};
fn emit(nodes: []const Node, i: u32, out: *Buf) void {
    const n = nodes[i];
    switch (n.op) {
        .cst => out.add("{d}", .{n.val}),
        .vr => out.add("v{d}", .{n.binder}),
        .add => { out.add("(", .{}); emit(nodes, n.a, out); out.add(" + ", .{}); emit(nodes, n.b, out); out.add(")", .{}); },
        .mul => { out.add("(", .{}); emit(nodes, n.a, out); out.add(" * ", .{}); emit(nodes, n.b, out); out.add(")", .{}); },
    }
}
pub fn main() void {
    // IR: let v0 = (3+4) in (v0 * 2)  -> expect 14
    const nodes = [_]Node{ .{.op=.cst,.val=3}, .{.op=.cst,.val=4}, .{.op=.add,.a=0,.b=1}, .{.op=.vr,.binder=0}, .{.op=.cst,.val=2}, .{.op=.mul,.a=3,.b=4} };
    var c = Buf{}; c.add("long prog(void){{ long v0 = ", .{}); emit(&nodes, 2, &c); c.add("; return ", .{}); emit(&nodes, 5, &c); c.add("; }}\n", .{});
    var lua = Buf{}; lua.add("local v0 = ", .{}); emit(&nodes, 2, &lua); lua.add("\nreturn v0 * 2\n", .{});
    std.debug.print("TRANSPILE IR -> C:\n{s}", .{c.str()});
    std.debug.print("TRANSPILE IR -> Lua:\n{s}", .{lua.str()});
}

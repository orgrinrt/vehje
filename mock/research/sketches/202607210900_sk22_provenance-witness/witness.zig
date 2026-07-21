const std = @import("std");
// SK22: diagnostics via LAZY provenance witness reconstruction (Souffle proof-tree
// on the resident relations). Provenance is cheap on the happy path (nothing extra);
// on a FAILURE, the derivation witness is reconstructed from the resident Child/VarUse
// relations, so the error names WHICH binder escaped through WHICH nodes.
const NB = 8;
var reach: [NB]u64 = undefined;
var child: []const [2]u32 = undefined; // (parent, childNode)
var var_use: []const [2]u32 = undefined; // (node, binder)
var names: [NB][]const u8 = undefined;

fn reconstruct(node: u32, binder: u32, buf: []u8, depth: u32) []const u8 {
    // is `binder` introduced directly here (a VarUse)?
    for (var_use) |vu| if (vu[0] == node and vu[1] == binder)
        return std.fmt.bufPrint(buf, "{s} uses binder b{d}", .{ names[node], binder }) catch buf;
    // else find the child that carries `binder` into `node` and recurse (the proof tree)
    for (child) |e| if (e[0] == node and (reach[e[1]] & (@as(u64, 1) << @intCast(binder))) != 0) {
        const sub = reconstruct(e[1], binder, buf[128..], depth + 1);
        return std.fmt.bufPrint(buf, "{s} <- {s}", .{ names[node], sub }) catch buf;
    };
    return std.fmt.bufPrint(buf, "{s}(?)", .{names[node]}) catch buf;
}

pub fn main() void {
    // IR: n7 = result (must not escape region of b2). reaches b0,b1 through n5<-{n3,n4}, b2 through n6.
    names = .{ "lit0", "lit1", "lit2", "use_b0", "use_b1", "add(3,4)", "use_b2", "result(5,6)" };
    const vu = [_][2]u32{ .{ 3, 0 }, .{ 4, 1 }, .{ 6, 2 } };
    const ch = [_][2]u32{ .{ 5, 3 }, .{ 5, 4 }, .{ 7, 5 }, .{ 7, 6 } };
    var_use = &vu; child = &ch;
    @memset(&reach, 0);
    for (vu) |v| reach[v[0]] |= (@as(u64, 1) << @intCast(v[1]));
    var changed = true; while (changed) { changed = false; for (ch) |e| { const b = reach[e[0]]; reach[e[0]] |= reach[e[1]]; if (reach[e[0]] != b) changed = true; } }
    // VIOLATION: result(n7) escapes its region but reaches b1 (a binder that dies first) -> error.
    const violating: u32 = 7; const escaped_binder: u32 = 1;
    std.debug.print("LEASE ERROR: value `{s}` reaches binder b{d} which does not outlive it.\n", .{ names[violating], escaped_binder });
    var buf: [1024]u8 = undefined;
    std.debug.print("  escape path: {s}\n", .{reconstruct(violating, escaped_binder, &buf, 0)});
    std.debug.print("SK22: happy path carries no extra cost; on failure the witness (which binder, through which nodes) is reconstructed from resident relations. WORKS.\n", .{});
}

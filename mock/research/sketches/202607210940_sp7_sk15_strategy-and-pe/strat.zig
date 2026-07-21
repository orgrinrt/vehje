const std = @import("std");
// SP7: proof-directed / graded strategy selection. A region carries a GRADE naming
// which evaluator is legal+optimal; selection is a constant-time grade dispatch, no
// runtime search. Native/copy-and-patch is ONE strategy, chosen where the grade proves
// it wins (hot + stencil-permitted). SK15: partial evaluation as the extraction
// preference (a static predicate folds to one arm; a dynamic one stays a branch).
const Strategy = enum { linear_scan, block_transfer, stencil_native, folded };
const RegionGrade = struct { straight_line: bool, hot: bool, stencil_ok: bool, static_pred: bool };
fn select(g: RegionGrade) Strategy { // constant-time, from the proof grades
    if (g.static_pred) return .folded; // SK15: predicate known -> PE folds it away
    if (g.hot and g.stencil_ok) return .stencil_native; // native only where it provably wins
    if (g.straight_line) return .linear_scan; // the L1-hot forward scan (BN1: switch)
    return .block_transfer; // branchy -> CFG block transfer (SK17)
}
pub fn main() void {
    const cases = [_]struct { name: []const u8, g: RegionGrade, want: Strategy }{
        .{ .name = "straight-line arithmetic", .g = .{ .straight_line = true, .hot = false, .stencil_ok = false, .static_pred = false }, .want = .linear_scan },
        .{ .name = "hot loop, stencil-permitted", .g = .{ .straight_line = true, .hot = true, .stencil_ok = true, .static_pred = false }, .want = .stencil_native },
        .{ .name = "hot loop, iOS (no stencil)", .g = .{ .straight_line = true, .hot = true, .stencil_ok = false, .static_pred = false }, .want = .linear_scan },
        .{ .name = "branchy region", .g = .{ .straight_line = false, .hot = false, .stencil_ok = false, .static_pred = false }, .want = .block_transfer },
        .{ .name = "static-predicate branch", .g = .{ .straight_line = false, .hot = false, .stencil_ok = false, .static_pred = true }, .want = .folded },
    };
    for (cases) |c| {
        const s = select(c.g);
        std.debug.print("region '{s}' -> {s} (want {s}) {s}\n", .{ c.name, @tagName(s), @tagName(c.want), if (s == c.want) "OK" else "MISMATCH" });
        std.debug.assert(s == c.want);
    }
    std.debug.print("SP7: grade selects the evaluator per region (const-time, native=one strategy where it wins). SK15: static predicate folds (PE). WORKS.\n", .{});
}

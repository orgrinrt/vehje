const std = @import("std");

// SK2 (CR1): no-alloc BOUNDED MULTI-SHOT algebraic-effect Handle over a host-lent
// budget, with a fail-closed comptime budget-fit check.
// Multi-shot effect = nondeterministic `amb`/`choose`; a collect-all handler resumes
// the continuation once per choice combination. Realised WITHOUT heap by:
//  (1) defunctionalising the continuation into a pure fn of its captured choices,
//  (2) enumerating the BOUNDED choice space in fixed host-lent buffers,
//  (3) a comptime check that the pattern footprint fits the lent budget (fail-closed),
//  (4) deterministic fixed mixed-radix enumeration order.
fn combos(comptime doms: []const u32) usize { var p: usize = 1; inline for (doms) |d| p *= d; return p; }

pub fn collectAll(
    comptime doms: []const u32,
    comptime CAP: usize,
    budget: *[CAP]i64,
    comptime k: fn ([]const u32) i64,
) usize {
    const total = comptime combos(doms);
    comptime if (total > CAP) @compileError(std.fmt.comptimePrint(
        "bounded multi-shot pattern needs {d} continuation slots, host lent only {d}", .{ total, CAP }));
    var choices: [doms.len]u32 = @splat(0);
    var count: usize = 0;
    while (count < total) : (count += 1) {
        budget[count] = k(&choices); // <- resume the continuation with this choice combo
        var j: usize = doms.len; // mixed-radix increment (deterministic order)
        while (j > 0) { j -= 1; choices[j] += 1; if (choices[j] < doms[j]) break; choices[j] = 0; }
    }
    return count;
}

const std = @import("std"); const h = @import("handle.zig");
fn prog(c: []const u32) i64 { _ = c; return 0; }
pub fn main() void {
    const doms = &[_]u32{ 10, 10, 10 }; // 1000 combos
    var budget: [16]i64 = undefined; // host lent only 16 -> MUST @compileError
    _ = h.collectAll(doms, 16, &budget, prog);
}

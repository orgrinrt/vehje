const engine = @import("engine.zig");
pub fn main() void {
    const t = engine.FamilyTag.Witcher; // not a generated variant -> must fail
    _ = t;
}

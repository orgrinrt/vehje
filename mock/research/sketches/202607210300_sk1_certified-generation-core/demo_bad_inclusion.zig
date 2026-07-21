const engine = @import("engine.zig");
comptime { engine.checkIncludedComptime(1 << 2); } // Clausewitz, unsupported -> must fail
pub fn main() void {}

//! Emit the Zig side's corpus results in the same shape the Rust side prints,
//! so the two projections are diffed byte-for-byte rather than compared by eye.
//!
//! Uses the comptime-specialised arm, which is the one whose agreement with Rust
//! is in question: the interpreted arm is already covered by `proj.zig`'s tests.

const std = @import("std");
const proj = @import("proj.zig");

pub fn main() void {
    for (proj.CORPUS) |c| {
        const args = [_]i64{ c[0], c[1] };
        std.debug.print("{d} {d} {d} {d} {d} {d}\n", .{
            c[0],
            c[1],
            proj.evalComptime(&proj.OP_ADD, &args),
            proj.evalComptime(&proj.OP_LT, &args),
            proj.evalComptime(&proj.OP_MUL, &args),
            proj.evalComptime(&proj.OP_DIFFSQ, &args),
        });
    }
}

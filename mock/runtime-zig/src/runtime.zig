// clause runtime — nuked.
//
// Nuked-equivalent stub. Rewrite from design docs in Phase 6 impl
// round.

const std = @import("std");

export fn clause_runtime_init(arena_bytes: u32) callconv(.C) ?*anyopaque {
    _ = arena_bytes;
    return null;
}

export fn clause_runtime_shutdown(handle: ?*anyopaque) callconv(.C) void {
    _ = handle;
}

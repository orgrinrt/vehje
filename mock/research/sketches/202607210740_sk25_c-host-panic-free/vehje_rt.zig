// SK25: a Zig runtime function exported over the C ABI. Errors are RETURN CODES,
// never panics (a panic would abort the embedding C host). This is the embeddability
// contract: a C host embeds the runtime and no panic crosses the boundary.
const std = @import("std");
export fn vehje_decode(ptr: [*]const u8, len: usize, out: *u32) callconv(.c) c_int {
    if (len < 4) return 1; // truncated -> error code, not a panic
    const b = ptr[0..len];
    const v = @as(u32, b[0]) | (@as(u32, b[1]) << 8) | (@as(u32, b[2]) << 16) | (@as(u32, b[3]) << 24);
    if (v > 0x0010_0000) return 2; // out of range -> error code
    out.* = v;
    return 0;
}

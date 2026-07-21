const std = @import("std");
// SK6: byte-shift decode (not pointer overlay) for unaligned @embedFile-style bytes.
// SK7: panic-free untrusted path (error unions, since a panic aborts the host over C ABI).
// SK23: differential-testing gate (the same kernel at comptime AND runtime -> identical).

// SK6: read a u32 LE from an arbitrary (possibly unaligned) byte offset by shifting.
fn readU32LE(b: []const u8, off: usize) u32 {
    return @as(u32, b[off]) | (@as(u32, b[off + 1]) << 8) | (@as(u32, b[off + 2]) << 16) | (@as(u32, b[off + 3]) << 24);
}
// SK7: an untrusted decode that returns an ERROR (never panics) on bad input.
const DecodeError = error{ TruncatedInput, OutOfRange };
fn decodeChecked(b: []const u8, off: usize, max: u32) DecodeError!u32 {
    if (off + 4 > b.len) return DecodeError.TruncatedInput; // no panic, an error union
    const v = readU32LE(b, off);
    if (v > max) return DecodeError.OutOfRange;
    return v;
}
// SK23: one kernel usable at comptime and runtime (fold a byte buffer).
fn kernel(b: []const u8) u32 {
    var acc: u32 = 2166136261;
    for (b) |x| { acc ^= x; acc = acc *% 16777619; }
    return acc;
}

pub fn main() void {
    // SK6: decode from a DELIBERATELY unaligned offset (1) -> byte-shift works.
    const buf = [_]u8{ 0xFF, 0x78, 0x56, 0x34, 0x12, 0x00 }; // u32 at off=1 is 0x12345678
    std.debug.print("SK6 byte-shift decode @unaligned off=1: 0x{X} (expect 0x12345678)\n", .{readU32LE(&buf, 1)});
    std.debug.assert(readU32LE(&buf, 1) == 0x12345678);

    // SK7: bad input yields an error, not a panic (a panic would abort a C host).
    const ok = decodeChecked(&buf, 1, 0xFFFFFFFF) catch unreachable;
    const trunc = decodeChecked(&buf, 4, 0xFFFFFFFF); // off+4 > len -> error
    const oor = decodeChecked(&buf, 1, 0x1000); // value > max -> error
    std.debug.print("SK7 panic-free: ok={X}, truncated={any}, out-of-range={any} (both errors, no panic)\n", .{ ok, trunc, oor });
    std.debug.assert(trunc == DecodeError.TruncatedInput and oor == DecodeError.OutOfRange);

    // SK23: same kernel at COMPTIME and RUNTIME -> identical (the differential gate).
    const data = "the quick brown fox jumps over the lazy dog";
    const ct = comptime kernel(data); // comptime locus
    const rt = kernel(data); // runtime locus
    std.debug.print("SK23 differential gate: comptime={X} runtime={X} equal={}\n", .{ ct, rt, ct == rt });
    std.debug.assert(ct == rt);
    std.debug.print("SK6/SK7/SK23 all WORK.\n", .{});
}

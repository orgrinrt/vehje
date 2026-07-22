// Cross-validation CLI for the Zig carrier interpreter. Reads stdin as
// [seed:u64 LE][program wire bytes], runs both the switch and tail dispatch over
// the identical bytes, verifies the two agree (the within-Zig cross-validation),
// and writes the 8-byte LE checksum to stdout. The Rust side generates a carrier
// program, pipes it here, and compares this checksum to its own interpreter, the
// cross-language byte-exact validation. Non-zero exit on any mismatch.
const std = @import("std");
const interp = @import("interp.zig");

// libc read/write directly: Zig 0.16 routes std.fs I/O through a new Io
// interface, but a byte-in/byte-out filter is clearest with the raw syscalls.
extern "c" fn read(fd: c_int, buf: [*]u8, n: usize) isize;
extern "c" fn write(fd: c_int, buf: [*]const u8, n: usize) isize;

// BSS, so a large results array does not blow the stack. 65536 nodes is well
// above the bench program sizes.
var RESULTS: [1 << 16]u64 = undefined;
var INBUF: [1 << 20]u8 = undefined;

pub fn main() !void {
    var total: usize = 0;
    while (total < INBUF.len) {
        const nr = read(0, INBUF[total..].ptr, INBUF.len - total);
        if (nr <= 0) break;
        total += @intCast(nr);
    }
    if (total < 8 + 16) return error.ShortInput;
    const seed = std.mem.readInt(u64, INBUF[0..8], .little);
    const prog = INBUF[8..total];
    const node_count: usize = std.mem.readInt(u32, prog[4..8], .little);
    if (node_count > RESULTS.len) return error.TooManyNodes;

    const cs_switch = interp.interpSwitch(prog.ptr, &RESULTS, seed);
    const cs_tail = interp.interpTail(prog.ptr, &RESULTS, seed);
    if (cs_switch != cs_tail) {
        std.debug.print("MISMATCH switch={x} tail={x}\n", .{ cs_switch, cs_tail });
        std.process.exit(1);
    }
    var out: [8]u8 = undefined;
    std.mem.writeInt(u64, out[0..8], cs_switch, .little);
    _ = write(1, &out, 8);
}

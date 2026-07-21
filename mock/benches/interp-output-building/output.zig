const std = @import("std");
// The TEXT-GENERATION hot path for op's emphasized biggest consumers (doc DSLs, config, typst/scribble-nature
// templating). The Interp core form lowers to interleaved [append literal span][append formatted value] into the
// reserve/commit output sink. This is what those consumers DO all day (produce output text), so its throughput is
// load-bearing. Three lowering strategies for `"Hi {name}, {count} items worth {total}.\n"`:
//   A) FORMAT-TO-TEMP-THEN-COPY: format each value into a scratch buffer, then memcpy into output (naive).
//   B) FORMAT-IN-PLACE: format each value directly into the reserved output cursor (no scratch, no double copy).
//   C) STATIC-LITERAL-SPANS + in-place values: literal runs are precompiled &'static byte spans (memcpy'd
//      wholesale), only the values are formatted in place. The lowering the design implies (literals are constants
//      in the residual; only the holes are computed).
// Measures output MB/s. Also reports ns per template-instantiation.
const N: usize = 5_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

// minimal integer-to-decimal into a buffer at cursor, returns bytes written (avoids std.fmt overhead differences)
inline fn writeInt(buf: []u8, v: i64) usize {
    if (v == 0) { buf[0] = '0'; return 1; }
    var tmp: [20]u8 = undefined; var n: usize = 0; var x: u64 = @intCast(if (v < 0) -v else v);
    while (x > 0) : (x /= 10) { tmp[n] = @intCast('0' + x % 10); n += 1; }
    var w: usize = 0; if (v < 0) { buf[0] = '-'; w = 1; }
    var k: usize = 0; while (k < n) : (k += 1) { buf[w+k] = tmp[n-1-k]; } return w + n;
}
const NAMES = [_][]const u8{ "alice", "bob", "charlie", "dave", "eve", "frank", "grace", "heidi" };
const lit0 = "Hi "; const lit1 = ", "; const lit2 = " items worth "; const lit3 = ".\n";

fn runFormatToTemp(out: []u8, i: usize) usize {
    var scratch: [64]u8 = undefined; var c: usize = 0;
    const name = NAMES[i & 7]; const count: i64 = @intCast(i & 4095); const total: i64 = @intCast((i * 7) & 65535);
    // literal
    @memcpy(out[c..c+lit0.len], lit0); c += lit0.len;
    // value via temp then copy
    @memcpy(out[c..c+name.len], name); c += name.len;
    @memcpy(out[c..c+lit1.len], lit1); c += lit1.len;
    var n = writeInt(&scratch, count); @memcpy(out[c..c+n], scratch[0..n]); c += n;
    @memcpy(out[c..c+lit2.len], lit2); c += lit2.len;
    n = writeInt(&scratch, total); @memcpy(out[c..c+n], scratch[0..n]); c += n;
    @memcpy(out[c..c+lit3.len], lit3); c += lit3.len;
    return c;
}
fn runFormatInPlace(out: []u8, i: usize) usize {
    var c: usize = 0;
    const name = NAMES[i & 7]; const count: i64 = @intCast(i & 4095); const total: i64 = @intCast((i * 7) & 65535);
    @memcpy(out[c..c+lit0.len], lit0); c += lit0.len;
    @memcpy(out[c..c+name.len], name); c += name.len;
    @memcpy(out[c..c+lit1.len], lit1); c += lit1.len;
    c += writeInt(out[c..], count);              // in place, no scratch
    @memcpy(out[c..c+lit2.len], lit2); c += lit2.len;
    c += writeInt(out[c..], total);
    @memcpy(out[c..c+lit3.len], lit3); c += lit3.len;
    return c;
}
// C is the same as B here at runtime (literals are already static spans memcpy'd), but models the design where
// the lowering is a precompiled span-list: we express it as a fixed instruction list [LIT,VAL,LIT,VAL,LIT,VAL,LIT]
const Op = enum { lit, ival, sval };
const Step = struct { op: Op, span: []const u8 = "", which: u8 = 0 };
fn runSpanList(out: []u8, steps: []const Step, i: usize) usize {
    var c: usize = 0;
    const name = NAMES[i & 7]; const count: i64 = @intCast(i & 4095); const total: i64 = @intCast((i * 7) & 65535);
    for (steps) |st| switch (st.op) {
        .lit => { @memcpy(out[c..c+st.span.len], st.span); c += st.span.len; },
        .sval => { @memcpy(out[c..c+name.len], name); c += name.len; },
        .ival => { c += writeInt(out[c..], if (st.which == 0) count else total); },
    };
    return c;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const out = al.alloc(u8, 1 << 16) catch unreachable;
    const steps = [_]Step{ .{.op=.lit,.span=lit0}, .{.op=.sval}, .{.op=.lit,.span=lit1}, .{.op=.ival,.which=0}, .{.op=.lit,.span=lit2}, .{.op=.ival,.which=1}, .{.op=.lit,.span=lit3} };
    var total_bytes: u64 = 0;
    inline for (.{ "A format-to-temp+copy", "B format-in-place", "C span-list (design lowering)" }, 0..) |name, idx| {
        var best: u64 = std.math.maxInt(u64); var sink: u64 = 0;
        for (0..5) |_| {
            const t0 = nowNs(); var bytes: u64 = 0;
            var i: usize = 0; while (i < N) : (i += 1) {
                var cursor: usize = 0;
                const wrote = switch (idx) { 0 => runFormatToTemp(out[cursor..], i), 1 => runFormatInPlace(out[cursor..], i), else => runSpanList(out[cursor..], &steps, i) };
                cursor += wrote; bytes += wrote; sink +%= out[wrote-1];
                _ = &cursor;
            }
            const dt = nowNs()-t0; if (dt < best) best = dt; total_bytes = bytes;
        }
        std.mem.doNotOptimizeAway(sink);
        const mbps = @as(f64,@floatFromInt(total_bytes)) / (@as(f64,@floatFromInt(best))/1e9) / (1<<20);
        std.debug.print("{s}: {d:.2} ns/template, {d:.0} MB/s output  ({d} bytes/template)\n",
            .{ name, @as(f64,@floatFromInt(best))/@as(f64,N), mbps, total_bytes/N });
    }
}

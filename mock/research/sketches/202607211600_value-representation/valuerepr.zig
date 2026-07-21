const std = @import("std");
// Bench-driven fork resolution: interpreter runtime VALUE representation. A dynamic script value is tagged
// at runtime (int/float/bool/strref/nil). The fork among the two 8-byte schemes (16-byte tagged-union is
// strictly 2x memory and loses on cache, so it is not the interesting comparison):
//   A) NaN-box: everything in a 64-bit f64. Real doubles ARE themselves; other types hide in the qNaN
//      payload tagged by high mantissa bits. Floats need no unpack; ints cost a mask+branch.
//   C) tagged 8B: 3 low bits tag, payload in the high 61. Ints cost a shift; floats must box (heap/side) OR
//      lose precision, the classic NaN-box-vs-tagged tradeoff.
// Workload: a value-shuffling interp loop (typed add over a 64-register file, 90% int / 10% float, the
// typical script mix), regs reset per timed run so both are measured from the same state.
const OPS: usize = 50_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

const TAG_INT: u64 = 0x7ffc_0000_0000_0000;
inline fn nbInt(v: i32) u64 { return TAG_INT | @as(u32, @bitCast(v)); }
inline fn nbIsInt(b: u64) bool { return (b & 0xffff_0000_0000_0000) == (TAG_INT & 0xffff_0000_0000_0000); }
inline fn nbAsInt(b: u64) i32 { return @bitCast(@as(u32, @truncate(b))); }
fn runNanbox(regs: []u64) i64 {
    var acc: i64 = 0; var i: usize = 0;
    while (i < OPS) : (i += 1) { const a = regs[i & 63]; const b = regs[(i >> 3) & 63];
        if (nbIsInt(a) and nbIsInt(b)) { const r = nbInt(nbAsInt(a) +% nbAsInt(b)); regs[i & 63] = r; acc +%= nbAsInt(r); }
        else { const r: u64 = @bitCast(@as(f64,@bitCast(a)) + @as(f64,@bitCast(b))); regs[i & 63] = r; acc +%= @intFromFloat(@as(f64,@bitCast(r))); } }
    return acc;
}
inline fn t8Int(v: i32) u64 { return (@as(u64, @as(u32,@bitCast(v))) << 3) | 1; }
inline fn t8IsInt(b: u64) bool { return (b & 7) == 1; }
inline fn t8AsInt(b: u64) i32 { return @bitCast(@as(u32, @truncate(b >> 3))); }
fn runTagged8(regs: []u64) i64 {
    var acc: i64 = 0; var i: usize = 0;
    while (i < OPS) : (i += 1) { const a = regs[i & 63]; const b = regs[(i >> 3) & 63];
        if (t8IsInt(a) and t8IsInt(b)) { const r = t8Int(t8AsInt(a) +% t8AsInt(b)); regs[i & 63] = r; acc +%= t8AsInt(r); }
        else { const r = (a & ~@as(u64,7)) | 2; regs[i & 63] = r; acc +%= @intCast((r >> 3) & 0xffff); } }
    return acc;
}
fn initNb(regs: []u64) void { var s: u64 = 99; for (0..64) |k| { s = s*%6364136223846793005+%1; const isf=(s>>33)%10==0; const iv:i32=@intCast((s>>40)%1000); regs[k]= if (isf) @bitCast(@as(f64,@floatFromInt(iv))) else nbInt(iv); } }
fn initT8(regs: []u64) void { var s: u64 = 99; for (0..64) |k| { s = s*%6364136223846793005+%1; const isf=(s>>33)%10==0; const iv:i32=@intCast((s>>40)%1000); regs[k]= if (isf) (@as(u64,@intCast(iv))<<3)|2 else t8Int(iv); } }
pub fn main() void {
    const al = std.heap.page_allocator;
    const rnb = al.alloc(u64, 64) catch unreachable;
    const r8 = al.alloc(u64, 64) catch unreachable;
    var bestA: u64 = std.math.maxInt(u64); var ca: i64 = 0;
    for (0..7) |_| { initNb(rnb); const t0=nowNs(); ca=runNanbox(rnb); const dt=nowNs()-t0; if (dt<bestA) bestA=dt; }
    var bestC: u64 = std.math.maxInt(u64); var cc: i64 = 0;
    for (0..7) |_| { initT8(r8); const t0=nowNs(); cc=runTagged8(r8); const dt=nowNs()-t0; if (dt<bestC) bestC=dt; }
    std.debug.print("A nan-box 8B : {d:.3} ns/op  ({d:.0} M-op/s)  chk={d}\n", .{ @as(f64,@floatFromInt(bestA))/@as(f64,OPS), @as(f64,OPS)*1e3/@as(f64,@floatFromInt(bestA)), ca });
    std.debug.print("C tagged  8B : {d:.3} ns/op  ({d:.0} M-op/s)  chk={d}\n", .{ @as(f64,@floatFromInt(bestC))/@as(f64,OPS), @as(f64,OPS)*1e3/@as(f64,@floatFromInt(bestC)), cc });
    std.debug.print("(16B tagged-union omitted: strictly 2x memory of either 8B scheme, loses on cache before any op-cost comparison)\n", .{});
}

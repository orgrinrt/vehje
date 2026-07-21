const std = @import("std");
// SK19: tristate-number (tnum) bounded abstract interpretation, the NUMERIC load
// residual (2001 item 4, eBPF-shaped). tnum = {value, mask}: mask bit set => that bit
// is UNKNOWN. Tracks known/unknown bits through arithmetic without concrete eval.
const Tnum = struct {
    value: u64, // known bits
    mask: u64, // 1 = unknown
    fn known(v: u64) Tnum { return .{ .value = v, .mask = 0 }; }
    fn unknownInRange(bits: u6) Tnum { return .{ .value = 0, .mask = (@as(u64, 1) << bits) - 1 }; } // low `bits` unknown
    fn andT(a: Tnum, b: Tnum) Tnum {
        const av = a.value & ~a.mask; const bv = b.value & ~b.mask; // definite bits
        return .{ .value = av & bv, .mask = (a.mask | av) & (b.mask | bv) & (a.mask | b.mask) };
    }
    // provable upper bound: max possible value = value | mask
    fn max(t: Tnum) u64 { return t.value | t.mask; }
    fn provablyLt(t: Tnum, n: u64) bool { return t.max() < n; }
};

// SK13: generational-reference check at the avoidance boundary (Vale). A reference
// carries a generation; the region carries a generation; deref checks they match
// (else the region was freed/reused). One u32 compare, the dynamic residual for the
// references reachability inference could NOT place statically.
const Region = struct { gen: u32 = 1, data: i64 = 0, alive: bool = true };
const GenRef = struct { region: *Region, gen: u32 };
fn makeRef(r: *Region) GenRef { return .{ .region = r, .gen = r.gen }; }
fn deref(ref: GenRef) ?i64 { return if (ref.region.gen == ref.gen and ref.region.alive) ref.region.data else null; }
fn freeRegion(r: *Region) void { r.alive = false; r.gen +%= 1; } // bump gen on free/reuse

pub fn main() void {
    // SK19: a value masked to low 8 bits is provably < 256, not < 100.
    const v = Tnum.unknownInRange(8); // 0..255 unknown low byte
    std.debug.print("SK19 tnum: max(unknown 8-bit)={d}, provably<256:{} provably<100:{}\n", .{ v.max(), v.provablyLt(256), v.provablyLt(100) });
    std.debug.assert(v.provablyLt(256) and !v.provablyLt(100));
    // AND with a mask of 0x0F makes it provably < 16
    const masked = Tnum.andT(v, Tnum.known(0x0F));
    std.debug.print("SK19 tnum: (unknown8 & 0x0F) max={d} provably<16:{}\n", .{ masked.max(), masked.provablyLt(16) });
    std.debug.assert(masked.provablyLt(16));

    // SK13: valid deref, then free+reuse -> stale ref caught.
    var r = Region{ .gen = 1, .data = 42 };
    const ref = makeRef(&r);
    std.debug.print("SK13 genref: deref valid = {any} (expect 42)\n", .{deref(ref)});
    std.debug.assert(deref(ref).? == 42);
    freeRegion(&r); r.gen +%= 0; r.data = 999; r.alive = true; // region slot reused (gen bumped to 2)
    std.debug.print("SK13 genref: deref after free/reuse = {any} (expect null: stale generation caught)\n", .{deref(ref)});
    std.debug.assert(deref(ref) == null);
    std.debug.print("SK19 tnum numeric residual + SK13 generational-ref avoidance residual WORK.\n", .{});
}

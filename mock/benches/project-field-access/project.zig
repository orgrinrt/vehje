const std = @import("std");
// The Project core form (field access / projection): record.field, the record-heavy templating/config hot path.
// vehje compiles ahead of time (Rust compile side), so field access splits by whether the record SHAPE is known
// at compile time. Strategies:
//   A) DIRECT OFFSET (compile-resolved): the field name resolved to a fixed slot offset at compile time; runtime
//      is a plain load. The static-shape case (the majority for locally-built records).
//   B) LINEAR NAME SCAN: dynamic record; find the field by comparing the accessed name against the record's field
//      names in order (small records).
//   C) HASH LOOKUP: dynamic record; hash the field name to a slot (large dynamic records).
//   D) INLINE CACHE (monomorphic PIC): cache the last (shape_id, name)->offset; guard on shape_id; hit => direct
//      load, miss => fall back to lookup + refill. The classic dynamic-language field-access accelerator.
// Measures ns/access. Distributions: MONOMORPHIC (every access same shape, the common loop-over-same-records case)
// and POLYMORPHIC (shape varies, defeats a monomorphic inline cache).
const N: usize = 50_000_000;
const FIELDS: usize = 8;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

const Rec = struct { shape_id: u32, names: [FIELDS]u32, vals: [FIELDS]i64 }; // names = interned field-name ids

fn directOffset(rec: *const Rec, off: usize) i64 { return rec.vals[off]; }
fn linearScan(rec: *const Rec, name: u32) i64 { for (0..FIELDS) |i| { if (rec.names[i] == name) return rec.vals[i]; } return -1; }
fn hashLookup(rec: *const Rec, name: u32) i64 {
    var h: usize = (name *% 2654435761) & (FIELDS-1); var probes: usize = 0;
    while (probes < FIELDS) : (probes += 1) { if (rec.names[h] == name) return rec.vals[h]; h = (h + 1) & (FIELDS-1); }
    return -1;
}
const IC = struct { shape_id: u32 = 0xffffffff, off: usize = 0 };
fn inlineCache(rec: *const Rec, name: u32, ic: *IC) i64 {
    if (rec.shape_id == ic.shape_id) return rec.vals[ic.off];       // guard hit: direct load
    for (0..FIELDS) |i| { if (rec.names[i] == name) { ic.shape_id = rec.shape_id; ic.off = i; return rec.vals[i]; } } // miss: refill
    return -1;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    // one shape: field names are ids 100..107, value = id*11. access field #5 (name 105).
    var protoNames: [FIELDS]u32 = undefined; for (0..FIELDS) |i| protoNames[i] = @intCast(100 + i);
    // MONOMORPHIC: 1024 records all same shape (shape_id 1)
    const monom = al.alloc(Rec, 1024) catch unreachable;
    for (monom, 0..) |*r, ri| { r.shape_id = 1; r.names = protoNames; for (0..FIELDS) |i| r.vals[i] = @intCast((ri*FIELDS+i)*11); }
    // POLYMORPHIC: 1024 records, 4 rotating shapes with fields in different orders (defeats monomorphic IC)
    const poly = al.alloc(Rec, 1024) catch unreachable;
    for (poly, 0..) |*r, ri| { r.shape_id = @intCast(1 + (ri & 3)); for (0..FIELDS) |i| { const perm = (i + (ri & 3)) & (FIELDS-1); r.names[i] = protoNames[perm]; r.vals[i] = @intCast((ri*FIELDS+perm)*11); } }
    const NAME: u32 = 105; const OFF: usize = 5; // direct-offset only valid for the monomorphic fixed shape
    for ([_][]const u8{ "MONOMORPHIC (same shape)", "POLYMORPHIC (4 shapes)" }, 0..) |label, mi| {
        const recs = if (mi == 0) monom else poly;
        std.debug.print("{s}:\n", .{label});
        inline for (.{ "direct-offset", "linear-scan", "hash-lookup", "inline-cache" }, 0..) |sname, si| {
            var best: u64 = std.math.maxInt(u64); var sink: i64 = 0; var ic = IC{};
            for (0..5) |_| { ic = IC{}; const t0 = nowNs(); var i: usize = 0;
                while (i < N) : (i += 1) { const rec = &recs[i & 1023];
                    sink +%= switch (si) { 0 => directOffset(rec, OFF), 1 => linearScan(rec, NAME), 2 => hashLookup(rec, NAME), else => inlineCache(rec, NAME, &ic) };
                }
                const dt = nowNs()-t0; if (dt < best) best = dt; }
            std.mem.doNotOptimizeAway(sink);
            // direct-offset is only correct for monomorphic; note it
            const note = if (si == 0 and mi == 1) " (INVALID for poly: shape varies, offset not fixed)" else "";
            std.debug.print("  {s}: {d:.2} ns/access{s}\n", .{ sname, @as(f64,@floatFromInt(best))/@as(f64,N), note });
        }
    }
}

const std = @import("std");
// SK16 settled the PEAK-MEMORY side of exact-meet reuse (emit-time referrer count-down, peak = live frontier,
// no runtime refcounts). It did NOT size the THROUGHPUT payoff of in-place-when-unique mutation vs always-copy on
// the record-UPDATE hot path (templating/scripting/config consumers mutate records constantly: set field, extend).
// This benches that. The reuse analysis' uniqueness verdict is KNOWN AT EMIT TIME (SK16), modeled here as a
// per-record `unique` bit the emitter set; the runtime just reads it (no refcount traffic).
//   A) ALWAYS-COPY (pure functional update): every field-set allocates a fresh record + copies all fields.
//   B) IN-PLACE-WHEN-UNIQUE (the exact-meet model): unique record => mutate the field in place (no copy);
//      shared record => copy once, then the copy is unique for subsequent updates.
//   C) ALWAYS-MUTABLE (the ceiling; unsafe, ignores sharing): always in place.
// Workload: FIELDS-wide records, UPDATES field-sets each, over N records. A tunable SHARED_FRAC of updates hit a
// shared record (forcing a copy in B). Measures ns/update + total copies.
const FIELDS: usize = 16;
const Rec = [FIELDS]i64;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

const N: usize = 2_000_000;
const UPDATES: usize = 8; // field-sets per record

fn runAlwaysCopy(arena: []Rec, top: *usize, seedbase: []const Rec, shared: []const bool) i64 {
    var acc: i64 = 0; top.* = 0;
    for (0..N) |i| {
        if (top.* + UPDATES + 2 >= arena.len) top.* = 0;
        var cur: usize = top.*; arena[cur] = seedbase[i & 1023]; top.* += 1;
        for (0..UPDATES) |u| {
            const nn = top.*; arena[nn] = arena[cur]; arena[nn][u & (FIELDS-1)] = @intCast(u); top.* += 1; cur = nn; // copy every update
        }
        acc +%= arena[cur][0]; _ = shared;
    }
    return acc;
}
fn runInPlaceUnique(arena: []Rec, top: *usize, seedbase: []const Rec, shared: []const bool, copies: *usize) i64 {
    var acc: i64 = 0; top.* = 0; copies.* = 0;
    for (0..N) |i| {
        if (top.* + UPDATES + 2 >= arena.len) top.* = 0;
        var cur: usize = top.*; arena[cur] = seedbase[i & 1023]; top.* += 1;
        var is_unique = true;
        for (0..UPDATES) |u| {
            const forced_shared = shared[(i*UPDATES+u) & 1023];
            if (is_unique and !forced_shared) { arena[cur][u & (FIELDS-1)] = @intCast(u); } // in place, no copy
            else { const nn = top.*; arena[nn] = arena[cur]; arena[nn][u & (FIELDS-1)] = @intCast(u); top.* += 1; cur = nn; copies.* += 1; is_unique = true; }
        }
        acc +%= arena[cur][0];
    }
    return acc;
}
fn runAlwaysMutable(arena: []Rec, top: *usize, seedbase: []const Rec) i64 {
    var acc: i64 = 0; top.* = 0;
    for (0..N) |i| {
        if (top.* + 2 >= arena.len) top.* = 0;
        const cur: usize = top.*; arena[cur] = seedbase[i & 1023]; top.* += 1;
        for (0..UPDATES) |u| { arena[cur][u & (FIELDS-1)] = @intCast(u); } // always in place
        acc +%= arena[cur][0];
    }
    return acc;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const arena = al.alloc(Rec, 1 << 20) catch unreachable;
    const seed = al.alloc(Rec, 1024) catch unreachable;
    var s: u64 = 1; for (seed) |*r| for (r) |*f| { s = s*%6364136223846793005+%1; f.* = @intCast(s >> 40); };
    // three sharing regimes: 0%, 20%, 60% of updates forced-shared
    for ([_]u32{ 0, 20, 60 }) |frac| {
        const shared = al.alloc(bool, 1024) catch unreachable;
        var ss: u64 = 7; for (shared) |*b| { ss = ss*%6364136223846793005+%1; b.* = (ss >> 40) % 100 < frac; }
        var top: usize = 0;
        var bC: u64 = std.math.maxInt(u64); var cc: i64 = 0;
        for (0..5) |_| { const t0=nowNs(); cc=runAlwaysCopy(arena, &top, seed, shared); const dt=nowNs()-t0; if (dt<bC) bC=dt; }
        var bR: u64 = std.math.maxInt(u64); var cr: i64 = 0; var cops: usize = 0;
        for (0..5) |_| { const t0=nowNs(); cr=runInPlaceUnique(arena, &top, seed, shared, &cops); const dt=nowNs()-t0; if (dt<bR) bR=dt; }
        var bM: u64 = std.math.maxInt(u64); var cm: i64 = 0;
        for (0..5) |_| { const t0=nowNs(); cm=runAlwaysMutable(arena, &top, seed); const dt=nowNs()-t0; if (dt<bM) bM=dt; }
        std.mem.doNotOptimizeAway(cc); std.mem.doNotOptimizeAway(cr); std.mem.doNotOptimizeAway(cm);
        const upd = N * UPDATES;
        std.debug.print("shared={d}%: COPY {d:.2} ns/upd | REUSE {d:.2} ns/upd ({d} copies, {d:.0}% of updates) | MUTABLE(ceiling) {d:.2} ns/upd  => reuse {d:.2}x faster than copy\n",
            .{ frac, @as(f64,@floatFromInt(bC))/@as(f64,upd), @as(f64,@floatFromInt(bR))/@as(f64,upd), cops, @as(f64,@floatFromInt(cops))*100.0/@as(f64,upd), @as(f64,@floatFromInt(bM))/@as(f64,upd), @as(f64,@floatFromInt(bC))/@as(f64,@floatFromInt(bR)) });
    }
}

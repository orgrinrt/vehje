const std = @import("std");
// Does value DEDUPLICATION (hash-consing produced values) compose with the exact-meet REUSE analysis, or do the
// two undercut each other? They are not independent: hash-consing makes structurally equal values share one
// representation, which RAISES the sharing rate by construction, and `record-update-reuse` measured that reuse
// pays 9.7x at 0% sharing and only 1.5x at 60%. So dedup buys cheap equality and memory by spending exactly the
// property reuse runs on. Benching them separately and switching both on would recommend two things that fight.
//
// Four strategies over the same workload (build a record, update fields on it, compare it against others):
//   A) PLAIN            : copy-on-write update, structural (field-by-field) equality. The naive runtime.
//   B) REUSE ONLY       : in-place-when-unique update (the measured win), structural equality.
//   C) DEDUP ALL        : hash-cons EVERY constructed value; equality is an id compare; update must always copy
//                         (an interned value is shared by construction, so in-place would corrupt every sharer).
//   D) DEDUP SHARED + REUSE : intern a value only when the emitter says it may be shared; a value proven unique
//                         stays OUT of the table and mutates in place. Equality is an id compare when both sides
//                         are interned, structural otherwise. This is the composition that should dissolve the
//                         conflict: nothing else can reach a unique value, so it does not need a dedup entry.
//
// Axes: SHARED_FRAC (how often the emitter's verdict is "may be shared") x COMPARES (equality ops per record).
// Metrics: ns per record cycle, copies made, values interned (memory proxy), structural compares performed.
const FIELDS: usize = 16;
const Rec = [FIELDS]i64;

fn nowNs() u64 {
    var ts: std.c.timespec = undefined;
    _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts);
    return @as(u64, @intCast(ts.sec)) * 1_000_000_000 + @as(u64, @intCast(ts.nsec));
}

const N: usize = 500_000;
const UPDATES: usize = 8;
const ARENA: usize = 1 << 19;
const TSIZE: usize = 1 << 20;
const EMPTY: u32 = 0xFFFF_FFFF;

fn hashRec(r: *const Rec) u64 {
    var h: u64 = 0xcbf29ce484222325;
    for (r.*) |f| {
        h ^= @as(u64, @bitCast(f));
        h *%= 0x100000001b3;
    }
    return h;
}

fn recEql(a: *const Rec, b: *const Rec) bool {
    for (a.*, b.*) |x, y| {
        if (x != y) return false;
    }
    return true;
}

const Stats = struct {
    copies: usize = 0,
    interned: usize = 0,
    structural: usize = 0,
    acc: i64 = 0,
};

/// Insert `idx` into the open-addressed table, or return the index already
/// holding a structurally equal record.
fn internOrGet(slots: []u32, arena: []Rec, idx: u32) ?u32 {
    const h = hashRec(&arena[idx]);
    var i: usize = @intCast(h & (TSIZE - 1));
    while (true) {
        const s = slots[i];
        if (s == EMPTY) {
            slots[i] = idx;
            return null;
        }
        if (recEql(&arena[s], &arena[idx])) return s;
        i = (i + 1) & (TSIZE - 1);
    }
}

fn runPlain(arena: []Rec, seed: []const Rec, shared: []const bool, compares: usize, st: *Stats) void {
    var top: usize = 0;
    var prev: usize = 0;
    for (0..N) |i| {
        if (top + UPDATES + 2 >= ARENA) top = 0;
        var cur: usize = top;
        arena[cur] = seed[i & 1023];
        top += 1;
        for (0..UPDATES) |u| {
            const nn = top;
            arena[nn] = arena[cur];
            arena[nn][u & (FIELDS - 1)] = @intCast(u);
            top += 1;
            cur = nn;
            st.copies += 1;
        }
        for (0..compares) |_| {
            st.structural += 1;
            if (recEql(&arena[cur], &arena[prev])) st.acc +%= 1;
        }
        prev = cur;
        _ = shared;
    }
}

fn runReuseOnly(arena: []Rec, seed: []const Rec, shared: []const bool, compares: usize, st: *Stats) void {
    var top: usize = 0;
    var prev: usize = 0;
    for (0..N) |i| {
        if (top + UPDATES + 2 >= ARENA) top = 0;
        var cur: usize = top;
        arena[cur] = seed[i & 1023];
        top += 1;
        for (0..UPDATES) |u| {
            if (!shared[(i * UPDATES + u) & 1023]) {
                arena[cur][u & (FIELDS - 1)] = @intCast(u);
            } else {
                const nn = top;
                arena[nn] = arena[cur];
                arena[nn][u & (FIELDS - 1)] = @intCast(u);
                top += 1;
                cur = nn;
                st.copies += 1;
            }
        }
        for (0..compares) |_| {
            st.structural += 1;
            if (recEql(&arena[cur], &arena[prev])) st.acc +%= 1;
        }
        prev = cur;
    }
}

fn runDedupAll(arena: []Rec, slots: []u32, seed: []const Rec, shared: []const bool, compares: usize, st: *Stats) void {
    var top: usize = 0;
    var prev: u32 = 0;
    for (slots) |*s| s.* = EMPTY;
    for (0..N) |i| {
        if (top + UPDATES + 2 >= ARENA) {
            top = 0;
            for (slots) |*s| s.* = EMPTY;
        }
        var cur: u32 = @intCast(top);
        arena[cur] = seed[i & 1023];
        top += 1;
        if (internOrGet(slots, arena, cur)) |hit| {
            cur = hit;
            top -= 1;
        } else st.interned += 1;
        for (0..UPDATES) |u| {
            const nn: u32 = @intCast(top);
            arena[nn] = arena[cur];
            arena[nn][u & (FIELDS - 1)] = @intCast(u);
            top += 1;
            st.copies += 1;
            cur = nn;
            if (internOrGet(slots, arena, cur)) |hit| {
                cur = hit;
                top -= 1;
            } else st.interned += 1;
        }
        for (0..compares) |_| {
            if (cur == prev) st.acc +%= 1;
        }
        prev = cur;
        _ = shared;
    }
}

fn runDedupSharedPlusReuse(arena: []Rec, slots: []u32, seed: []const Rec, shared: []const bool, compares: usize, st: *Stats) void {
    var top: usize = 0;
    var prev: u32 = 0;
    var prev_interned = false;
    for (slots) |*s| s.* = EMPTY;
    for (0..N) |i| {
        if (top + UPDATES + 2 >= ARENA) {
            top = 0;
            for (slots) |*s| s.* = EMPTY;
        }
        var cur: u32 = @intCast(top);
        arena[cur] = seed[i & 1023];
        top += 1;
        var cur_interned = false;
        for (0..UPDATES) |u| {
            if (!shared[(i * UPDATES + u) & 1023]) {
                // proven unique: nothing else can reach it, so it needs no dedup
                // entry and the update is a plain in-place write.
                arena[cur][u & (FIELDS - 1)] = @intCast(u);
                cur_interned = false;
            } else {
                const nn: u32 = @intCast(top);
                arena[nn] = arena[cur];
                arena[nn][u & (FIELDS - 1)] = @intCast(u);
                top += 1;
                st.copies += 1;
                cur = nn;
                if (internOrGet(slots, arena, cur)) |hit| {
                    cur = hit;
                    top -= 1;
                } else st.interned += 1;
                cur_interned = true;
            }
        }
        for (0..compares) |_| {
            if (cur_interned and prev_interned) {
                if (cur == prev) st.acc +%= 1;
            } else {
                st.structural += 1;
                if (recEql(&arena[cur], &arena[prev])) st.acc +%= 1;
            }
        }
        prev = cur;
        prev_interned = cur_interned;
    }
}

pub fn main() void {
    const al = std.heap.page_allocator;
    const arena = al.alloc(Rec, ARENA) catch unreachable;
    const slots = al.alloc(u32, TSIZE) catch unreachable;
    const seed = al.alloc(Rec, 1024) catch unreachable;
    const shared = al.alloc(bool, 1024) catch unreachable;

    // A duplicate-rich seed population: only 64 distinct base records across the
    // 1024 slots, so structurally equal values genuinely recur and dedup has
    // something to find. A population with no duplicates would measure only the
    // hashing overhead and none of the payoff.
    var s: u64 = 1;
    for (0..64) |k| for (&seed[k]) |*f| {
        s = s *% 6364136223846793005 +% 1;
        f.* = @intCast(s >> 40);
    };
    for (64..1024) |k| seed[k] = seed[k & 63];

    std.debug.print("strategy,shared_pct,compares,ns_per_record,copies,interned,structural\n", .{});

    for ([_]u32{ 0, 20, 60 }) |frac| {
        var ss: u64 = 7;
        for (shared) |*b| {
            ss = ss *% 6364136223846793005 +% 1;
            b.* = (ss >> 40) % 100 < frac;
        }
        for ([_]usize{ 2, 8 }) |compares| {
            inline for (.{ "plain", "reuse", "dedup_all", "dedup_shared_reuse" }, 0..) |name, which| {
                var best: u64 = std.math.maxInt(u64);
                var last: Stats = .{};
                for (0..5) |_| {
                    var st: Stats = .{};
                    const t0 = nowNs();
                    switch (which) {
                        0 => runPlain(arena, seed, shared, compares, &st),
                        1 => runReuseOnly(arena, seed, shared, compares, &st),
                        2 => runDedupAll(arena, slots, seed, shared, compares, &st),
                        else => runDedupSharedPlusReuse(arena, slots, seed, shared, compares, &st),
                    }
                    const dt = nowNs() - t0;
                    if (dt < best) best = dt;
                    last = st;
                }
                std.debug.print("{s},{d},{d},{d:.2},{d},{d},{d}\n", .{
                    name, frac, compares,
                    @as(f64, @floatFromInt(best)) / @as(f64, @floatFromInt(N)),
                    last.copies, last.interned, last.structural,
                });
            }
        }
    }
}

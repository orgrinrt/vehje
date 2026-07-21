const std = @import("std");
// Expansion (closes the thread the parallelism finding opened): per-mod-local interners are what make the
// compile stage parallel (no shared interner mutation at the parallel boundary). But ONE composed runtime
// wants ONE string table. So parallel compile is followed by a MERGE: dedupe N per-mod tables into a global
// table and rewrite each mod's local string-refs to global ids. Question: is the merge cheap relative to the
// parallel compile win (18ms for 2000 mods), or does it serialise away the speedup?
//
// Model: N mods, each interned M strings locally (heavy overlap: mods share common vocabulary like keywords,
// field names). Merge = hash every (mod, local_id) string into a global open-addressing table, build the
// per-mod local->global remap, done single-threaded (the merge is the serial tail of a parallel compile).

const N_MODS: usize = 2000;
const STRINGS_PER_MOD: usize = 256;   // a mod's distinct identifiers/literals
const SHARED_VOCAB: usize = 200;       // ~78% of each mod's strings drawn from a shared vocabulary
const VOCAB_POOL: usize = 4096;        // the shared vocabulary size
const UNIQUE_POOL: usize = 100_000;    // per-mod-unique strings drawn from a big pool

fn fnv(s: []const u8) u64 { var h: u64 = 0xcbf29ce484222325; for (s) |c| { h ^= c; h *%= 0x100000001b3; } return h; }
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

pub fn main() void {
    const al = std.heap.page_allocator;
    // build a string corpus. each string is just an id we render to bytes; overlap modelled by drawing ids.
    // per-mod local tables: mod m's strings are STRINGS_PER_MOD ids; SHARED_VOCAB of them from [0,VOCAB_POOL),
    // the rest from [VOCAB_POOL, VOCAB_POOL+UNIQUE_POOL). Store as the raw ids (the "string content" key).
    const local = al.alloc(u32, N_MODS * STRINGS_PER_MOD) catch unreachable;
    var s: u64 = 0x1234;
    for (0..N_MODS) |m| for (0..STRINGS_PER_MOD) |k| {
        s = s *% 6364136223846793005 +% 1442695040888963407; const r = s >> 33;
        const id: u32 = if (k < SHARED_VOCAB) @intCast(r % VOCAB_POOL) else @intCast(VOCAB_POOL + (r % UNIQUE_POOL));
        local[m*STRINGS_PER_MOD + k] = id;
    };
    // total local strings = N_MODS*STRINGS_PER_MOD = 512K. render each id to a short byte string for hashing.
    // MERGE: global open-addressing table keyed by the rendered string, producing a dense global id + remap.
    const CAP: usize = 1 << 20; // 1M slots for up to ~204K distinct globals, <25% load
    const gkey = al.alloc(u32, CAP) catch unreachable; @memset(gkey, 0xFFFF_FFFF);
    const gid = al.alloc(u32, CAP) catch unreachable;
    const remap = al.alloc(u32, N_MODS * STRINGS_PER_MOD) catch unreachable;
    var buf: [16]u8 = undefined;

    var best: u64 = std.math.maxInt(u64);
    var distinct: u32 = 0;
    for (0..5) |_| {
        @memset(gkey, 0xFFFF_FFFF); distinct = 0;
        const t0 = nowNs();
        for (local, 0..) |id, i| {
            const key = std.fmt.bufPrint(&buf, "s{d}", .{id}) catch unreachable;
            var h = fnv(key) & (CAP - 1);
            while (true) {
                if (gkey[h] == 0xFFFF_FFFF) { gkey[h] = id; gid[h] = distinct; remap[i] = distinct; distinct += 1; break; }
                if (gkey[h] == id) { remap[i] = gid[h]; break; }
                h = (h + 1) & (CAP - 1);
            }
        }
        const dt = nowNs() - t0;
        if (dt < best) best = dt;
    }
    const total_local = N_MODS * STRINGS_PER_MOD;
    std.debug.print("merge {d} local strings ({d} mods x {d}) -> {d} distinct globals\n", .{ total_local, N_MODS, STRINGS_PER_MOD, distinct });
    std.debug.print("merge time: {d:.3} ms   ({d:.1} M-strings/s)   dedup ratio {d:.1}x\n", .{
        @as(f64,@floatFromInt(best))/1e6,
        @as(f64,@floatFromInt(total_local))*1e3/@as(f64,@floatFromInt(best)),
        @as(f64,@floatFromInt(total_local))/@as(f64,@floatFromInt(distinct)) });
    std.debug.print("vs parallel-compile 8t ~18ms for the same 2000 mods => merge is {d:.0}%% of the parallel compile\n", .{ @as(f64,@floatFromInt(best))/1e6/18.0*100.0 });
    var sink: u64 = 0; for (remap) |r| sink +%= r; std.debug.print("(sink {d})\n", .{sink});
}

const std = @import("std");
// Expansion (the realistic mod case, punted 3x): op's consumers (Clausewitz/RimWorld) have mods that
// OVERRIDE/EXTEND each other = a cross-mod dependency DAG, not independent mods. Two questions the
// independent-mod benches did NOT answer:
//   Q1 COLD parallel: with a dep DAG, parallel compile is bounded by the CRITICAL PATH (a level-sync
//      schedule: level L can't start until level L-1's mods are compiled). What's the achievable speedup?
//   Q2 WARM incremental: when ONE mod changes, its transitive DEPENDENTS must recompile. What's the
//      blast radius (how many mods) for a realistic DAG shape?
//
// Model a realistic mod DAG: N mods, each depends on 0..K earlier mods (a "load-order" DAG: a mod can
// only depend on mods before it in load order, which is exactly how Paradox/RimWorld load orders work).
// Layered by longest-path-to-root => the level-sync parallel schedule; transitive-dependents => blast radius.

const N: usize = 2000;
const MAX_DEPS: usize = 3;      // each mod depends on up to 3 earlier mods (realistic: base + a couple patches)
const COMPILE_MS_PER_MOD: f64 = 0.05; // ~0.05ms/mod compile (from e2e: mid mod ~3ms for 50k nodes => small mods faster)

fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

pub fn main() void {
    const al = std.heap.page_allocator;
    var deps = al.alloc([MAX_DEPS]i32, N) catch unreachable;
    var ndeps = al.alloc(u8, N) catch unreachable;
    // dependents adjacency (for warm blast radius)
    var depnts = std.ArrayListUnmanaged(u32).empty;
    var dep_head = al.alloc(u32, N + 1) catch unreachable; // built after counting
    var s: u64 = 0xBEEF;
    var counts = al.alloc(u32, N) catch unreachable; @memset(counts, 0);
    for (0..N) |m| {
        var k: usize = 0;
        if (m > 0) {
            s = s *% 6364136223846793005 +% 1442695040888963407;
            const want = (s >> 33) % (MAX_DEPS + 1); // 0..MAX_DEPS
            while (k < want) : (k += 1) {
                s = s *% 6364136223846793005 +% 1442695040888963407;
                const d: u32 = @intCast((s >> 33) % m); // an earlier mod
                deps[m][k] = @intCast(d);
                counts[d] += 1;
            }
        }
        ndeps[m] = @intCast(k);
    }
    // build CSR dependents
    dep_head[0] = 0; for (0..N) |m| dep_head[m+1] = dep_head[m] + counts[m];
    depnts.resize(al, dep_head[N]) catch unreachable;
    var cursor = al.alloc(u32, N) catch unreachable; for (0..N) |m| cursor[m] = dep_head[m];
    for (0..N) |m| for (0..ndeps[m]) |k| { const d: usize = @intCast(deps[m][k]); depnts.items[cursor[d]] = @intCast(m); cursor[d] += 1; };

    // Q1: level = 1 + max(level of deps). critical path = max level. level-sync parallel time (infinite cores)
    // = sum over levels of (max mod cost in level) ~ COMPILE_MS_PER_MOD * num_levels (uniform cost).
    var level = al.alloc(u32, N) catch unreachable;
    var maxlvl: u32 = 0; var width = al.alloc(u32, N) catch unreachable; @memset(width, 0);
    for (0..N) |m| { var lv: u32 = 0; for (0..ndeps[m]) |k| { const d: usize = @intCast(deps[m][k]); lv = @max(lv, level[d] + 1); } level[m] = lv; maxlvl = @max(maxlvl, lv); width[lv] += 1; }
    const serial_ms = @as(f64, N) * COMPILE_MS_PER_MOD;
    const critical_ms = @as(f64, @floatFromInt(maxlvl + 1)) * COMPILE_MS_PER_MOD; // infinite-core level-sync
    // finite-core (8) level-sync: each level takes ceil(width/8) * cost
    var finite8_ms: f64 = 0; for (0..maxlvl+1) |lv| { const w = width[lv]; finite8_ms += @as(f64,@floatFromInt((w + 7)/8)) * COMPILE_MS_PER_MOD; }
    std.debug.print("DAG: {d} mods, up to {d} deps each, {d} levels (critical-path depth)\n", .{ N, MAX_DEPS, maxlvl+1 });
    std.debug.print("Q1 COLD: serial {d:.1}ms | inf-core level-sync {d:.2}ms ({d:.0}x) | 8-core level-sync {d:.2}ms ({d:.1}x)\n", .{
        serial_ms, critical_ms, serial_ms/critical_ms, finite8_ms, serial_ms/finite8_ms });

    // Q2: WARM blast radius. change 1 random mod => transitive dependents recompile. average over 200 samples.
    const visited = al.alloc(bool, N) catch unreachable;
    var stack = al.alloc(u32, N) catch unreachable;
    var total: u64 = 0; var maxbr: u64 = 0; const SAMPLES: usize = 200;
    for (0..SAMPLES) |it| {
        @memset(visited, false);
        s = s *% 6364136223846793005 +% 1442695040888963407; const start: u32 = @intCast((s>>33) % N);
        var sp: usize = 0; stack[sp] = start; sp += 1; visited[start] = true; var br: u64 = 0;
        while (sp > 0) { sp -= 1; const x = stack[sp]; br += 1; for (dep_head[x]..dep_head[x+1]) |e| { const y = depnts.items[e]; if (!visited[y]) { visited[y] = true; stack[sp] = y; sp += 1; } } }
        total += br; maxbr = @max(maxbr, br); _ = it;
    }
    std.debug.print("Q2 WARM: edit 1 mod => avg blast radius {d:.1} mods (max {d}) of {d} => {d:.1}% recompile\n", .{
        @as(f64,@floatFromInt(total))/@as(f64,SAMPLES), maxbr, N, @as(f64,@floatFromInt(total))/@as(f64,SAMPLES)/@as(f64,N)*100.0 });
}

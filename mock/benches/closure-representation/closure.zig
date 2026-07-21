const std = @import("std");
// Lambda + Apply as CLOSURES: closure creation + call + captured-variable access, common in scripting (callbacks,
// higher-order fns in the Iter pipelines). The representation fork:
//   A) FLAT CLOSURE: copy the captured values into the closure object at creation. O(K) creation, O(1) access.
//   B) LINKED ENV: the closure holds a pointer to its parent frame; captured-variable access walks the env chain.
//      O(1) creation, O(depth) access.
// Two usage patterns decide it: CREATE-ONCE-CALL-MANY (a closure made once, invoked in a hot loop: access cost
// dominates) and CREATE-MANY-CALL-ONCE (a closure made per iteration, invoked once: creation cost dominates, the
// Iter-pipeline-callback case). Sweep capture count K and env depth D.
const CALLS: usize = 20_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

const MAXK = 8;
// FLAT: closure carries a copy of K captured values
const Flat = struct { caps: [MAXK]i64, k: usize };
inline fn flatMake(src: []const i64, k: usize) Flat { var f: Flat = .{ .caps = undefined, .k = k }; for (0..k) |i| f.caps[i] = src[i]; return f; }
inline fn flatCall(f: *const Flat, idx: usize) i64 { return f.caps[idx]; } // O(1) access

// LINKED: env frame with a parent pointer; access walks up `depth` frames then indexes
const Env = struct { slots: [MAXK]i64, parent: ?*const Env };
inline fn linkedCall(leaf: *const Env, depth: usize, idx: usize) i64 {
    var e: *const Env = leaf; var d = depth; while (d > 0) : (d -= 1) { e = e.parent.?; } return e.slots[idx];
}
pub fn main() void {
    const al = std.heap.page_allocator;
    var src: [MAXK]i64 = undefined; for (0..MAXK) |i| src[i] = @intCast(i*7+1);
    // build a linked env chain of depth 8 for the access-depth sweep
    const chain = al.alloc(Env, 8) catch unreachable;
    for (0..8) |i| { chain[i] = .{ .slots = src, .parent = if (i == 0) null else &chain[i-1] }; }
    const leaf = &chain[7];

    std.debug.print("CREATE-ONCE, CALL-MANY (access cost dominates):\n", .{});
    // flat: make once (K=8), call CALLS times accessing a capture
    {
        var best: u64 = std.math.maxInt(u64); var sink: i64 = 0;
        for (0..5) |_| { const f = flatMake(&src, 8); const t0 = nowNs(); var i: usize = 0; while (i < CALLS) : (i += 1) sink +%= flatCall(&f, i & 7); const dt = nowNs()-t0; if (dt < best) best = dt; }
        std.mem.doNotOptimizeAway(sink);
        std.debug.print("  FLAT   (K=8): {d:.2} ns/call\n", .{ @as(f64,@floatFromInt(best))/@as(f64,CALLS) });
    }
    // linked: access at depths 1, 4, 8
    for ([_]usize{ 1, 4, 7 }) |depth| {
        var best: u64 = std.math.maxInt(u64); var sink: i64 = 0;
        for (0..5) |_| { const t0 = nowNs(); var i: usize = 0; while (i < CALLS) : (i += 1) sink +%= linkedCall(leaf, depth, i & 7); const dt = nowNs()-t0; if (dt < best) best = dt; }
        std.mem.doNotOptimizeAway(sink);
        std.debug.print("  LINKED (depth {d}): {d:.2} ns/call\n", .{ depth, @as(f64,@floatFromInt(best))/@as(f64,CALLS) });
    }
    std.debug.print("\nCREATE-MANY, CALL-ONCE (creation cost dominates; the Iter-callback case):\n", .{});
    // per iteration: create a closure then call it once. sweep capture count K.
    for ([_]usize{ 1, 4, 8 }) |k| {
        // FLAT: make (copy K) + 1 call
        var bF: u64 = std.math.maxInt(u64); var sF: i64 = 0;
        for (0..5) |_| { const t0 = nowNs(); var i: usize = 0; while (i < CALLS) : (i += 1) { const f = flatMake(&src, k); sF +%= flatCall(&f, i & (k-1)); } const dt = nowNs()-t0; if (dt < bF) bF = dt; }
        std.mem.doNotOptimizeAway(sF);
        // LINKED: make (1 ptr + slot init) + 1 call at depth 1
        var bL: u64 = std.math.maxInt(u64); var sL: i64 = 0;
        for (0..5) |_| { const t0 = nowNs(); var i: usize = 0; while (i < CALLS) : (i += 1) { var e = Env{ .slots = src, .parent = leaf }; std.mem.doNotOptimizeAway(&e); sL +%= linkedCall(&e, 0, i & (k-1)); } const dt = nowNs()-t0; if (dt < bL) bL = dt; }
        std.mem.doNotOptimizeAway(sL);
        std.debug.print("  K={d}: FLAT (copy {d} + call) {d:.2} ns | LINKED (ptr + call) {d:.2} ns\n", .{ k, k, @as(f64,@floatFromInt(bF))/@as(f64,CALLS), @as(f64,@floatFromInt(bL))/@as(f64,CALLS) });
    }
}

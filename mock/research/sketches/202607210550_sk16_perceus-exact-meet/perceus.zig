const std = @import("std");
// SK16: Perceus-derived exact-meet + in-place reuse for produced values (2055).
// At EMIT time (no runtime refcounts, the honest-keeper's "derived not verbatim"),
// each node's referrer count is known; when its LAST referrer is emitted, the node's
// slot is reclaimed and reused by the next allocation. Result: peak live slots =
// the live frontier, not the total node count. Collector-free, exact, bounded.
pub fn main() void {
    const al = std.heap.page_allocator;
    // a chain-with-fan value DAG: node i (i>=2) consumes i-1 and i-2 (the classic
    // fibonacci-shaped sharing). Referrer count of node k = how many later nodes use it.
    const N: usize = 1_000_000;
    var refcount = al.alloc(u32, N) catch unreachable; @memset(refcount, 0);
    for (2..N) |i| { refcount[i - 1] += 1; refcount[i - 2] += 1; }
    // emit in order; a free-list of reclaimed slots; a node takes a reused slot if any.
    var slot_of = al.alloc(u32, N) catch unreachable;
    var free = std.ArrayListUnmanaged(u32).empty;
    var next_slot: u32 = 0; var live: u32 = 0; var peak: u32 = 0;
    var remaining = al.dupe(u32, refcount) catch unreachable;
    for (0..N) |i| {
        // allocate a slot for node i (reuse if free)
        const s = if (free.pop()) |r| r else blk: { const t = next_slot; next_slot += 1; break :blk t; };
        slot_of[i] = s; live += 1; peak = @max(peak, live);
        // this emit is the last referrer of its operands (i-1, i-2) if their remaining hits 0 -> reclaim
        if (i >= 2) { // node i (i>=2) references i-1 and i-2; leaves 0,1 have no operands
            remaining[i - 1] -= 1; if (remaining[i - 1] == 0) { free.append(al, slot_of[i - 1]) catch unreachable; live -= 1; }
            remaining[i - 2] -= 1; if (remaining[i - 2] == 0) { free.append(al, slot_of[i - 2]) catch unreachable; live -= 1; }
        }
    }
    std.debug.print("N={d} nodes: slots actually used (peak live) = {d}, high-water next_slot = {d}\n", .{ N, peak, next_slot });
    std.debug.print("exact-meet reuse: peak/N = {d:.4} (frontier-bounded, NOT total). collector-free, no runtime refcounts.\n", .{@as(f64, @floatFromInt(peak)) / @as(f64, @floatFromInt(N))});
}

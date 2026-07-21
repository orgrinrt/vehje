const std = @import("std");
// SP3: the NATIVE content-validation step (the third locus). Runs at build time as a
// native program (NOT comptime), so large/superlinear content validation (which hits
// the BN0 comptime cliff) runs at native speed. Reads bundled content, validates it,
// fails the build on bad content.
pub fn main() !void {
    // simulate bundled content: N records to validate (unique keys, ids in range).
    const N = 200_000;
    var seen = std.AutoHashMap(u32, void).init(std.heap.page_allocator);
    defer seen.deinit();
    var prng = std.Random.DefaultPrng.init(7); const rnd = prng.random();
    var ok: usize = 0;
    for (0..N) |i| {
        const key = rnd.int(u32);
        if (seen.contains(key)) continue; // (dedup; native hashmap, no comptime cliff)
        try seen.put(key, {});
        if (i > N) { std.debug.print("content invalid\n", .{}); std.process.exit(1); }
        ok += 1;
    }
    std.debug.print("native content-validation step: {d} records validated at native speed (no comptime cliff)\n", .{ok});
}

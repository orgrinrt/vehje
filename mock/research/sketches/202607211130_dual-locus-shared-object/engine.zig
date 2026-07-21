// The ONE hand-authored engine, compiled once to a C-ABI object. Both loci (Rust
// dev-side, C/Zig runtime-side) link THIS SAME object -> no divergence by construction
// (the counter-audit's dual-locus resolution / return-to-canon single-engine).
export fn vehje_engine_reachhash(binders: [*]const u32, n_binders: usize, seed: u64) callconv(.c) u64 {
    // a deterministic "engine" computation: fold the binder set into a reach hash.
    var acc: u64 = seed ^ 1469598103934665603;
    var i: usize = 0;
    while (i < n_binders) : (i += 1) { acc ^= (@as(u64, 1) << @intCast(binders[i] & 63)); acc = acc *% 1099511628211; }
    return acc;
}

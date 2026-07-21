// zig_tail variant root: tail-threaded (@call(.always_tail)) interpreter cdylib.
const d = @import("dispatch.zig");
export fn bench_entry(input: [*]const u8, output: [*]u8, n: usize) d.FfiBenchCall {
    return d.runBench(true, input, output, n);
}
export fn bench_name() [*:0]const u8 {
    return "zig_tail";
}
export fn bench_abi_hash() u64 {
    return d.abiHash();
}

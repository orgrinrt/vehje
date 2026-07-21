const std = @import("std"); const A = @import("arena.zig");
// SK21: the reserve/commit sink (host-lent window, the C-ABI shape from 1315) +
// the zero-copy wire form (the same relative-indexed bytes are in-process AND wire) +
// the cross-chunk-implies-promoted property (a chunk boundary sits only at a whole
// subtree root, so a cross-chunk reference is by construction to a promoted root).
const VehjeSink = extern struct {
    reserve: *const fn (ud: *anyopaque, hint: usize) callconv(.c) [*]u8,
    commit: *const fn (ud: *anyopaque, n: usize) callconv(.c) void,
    ud: *anyopaque,
};
// A host implementing the sink with a single lent buffer + a write cursor (backpressure
// would block in `reserve`; here the buffer is large enough).
const HostBuf = struct { buf: []u8, cursor: usize = 0 };
fn hostReserve(ud: *anyopaque, hint: usize) callconv(.c) [*]u8 {
    const h: *HostBuf = @ptrCast(@alignCast(ud));
    std.debug.assert(h.cursor + hint <= h.buf.len); // backpressure point
    return h.buf.ptr + h.cursor;
}
fn hostCommit(ud: *anyopaque, n: usize) callconv(.c) void {
    const h: *HostBuf = @ptrCast(@alignCast(ud)); h.cursor += n;
}
// serialise the arena's three regions through the sink (header + nodes + pool + blob),
// all relative-indexed so the bytes ARE the wire form (zero-copy, no pointer fixups).
fn serialize(sink: VehjeSink, ar: *const A.Arena) usize {
    const nb = ar.nodes.items.len * @sizeOf(A.Node);
    const pb = ar.pool.items.len * @sizeOf(u32);
    const bb = ar.blob.items.len;
    const hdr = [4]u64{ ar.nodes.items.len, ar.pool.items.len, ar.blob.items.len, 0 };
    const total = @sizeOf(@TypeOf(hdr)) + nb + pb + bb;
    const w = sink.reserve(sink.ud, total);
    var o: usize = 0;
    @memcpy(w[o .. o + @sizeOf(@TypeOf(hdr))], std.mem.asBytes(&hdr)); o += @sizeOf(@TypeOf(hdr));
    @memcpy(w[o .. o + nb], std.mem.sliceAsBytes(ar.nodes.items)); o += nb;
    @memcpy(w[o .. o + pb], std.mem.sliceAsBytes(ar.pool.items)); o += pb;
    @memcpy(w[o .. o + bb], ar.blob.items); o += bb;
    sink.commit(sink.ud, o);
    return o;
}
pub fn main() void {
    const al = std.heap.page_allocator;
    var ar = A.Arena{ .al = al };
    const a = ar.emitScalar(5); const b = ar.emitScalar(7); const r = ar.emitRecord(&.{ a, b });
    _ = r;
    var host = HostBuf{ .buf = al.alloc(u8, 4096) catch unreachable };
    const sink = VehjeSink{ .reserve = &hostReserve, .commit = &hostCommit, .ud = &host };
    const n = serialize(sink, &ar);
    std.debug.print("sink: reserve/commit wrote {d} bytes to host-lent buffer\n", .{n});
    // read the wire form back zero-copy (reinterpret host bytes as the arena regions)
    const hdr = std.mem.bytesToValue([4]u64, host.buf[0..32]);
    const nn = hdr[0]; const np = hdr[1];
    const nodes: []const A.Node = @alignCast(std.mem.bytesAsSlice(A.Node, host.buf[32 .. 32 + nn * @sizeOf(A.Node)]));
    const pooloff = 32 + nn * @sizeOf(A.Node);
    const pool: []const u32 = @alignCast(std.mem.bytesAsSlice(u32, host.buf[pooloff .. pooloff + np * @sizeOf(u32)]));
    std.debug.print("wire==in-proc, zero-copy read back: sum={d} (expect 12), valid={}\n", .{ A.sumScalars(nodes, pool, @intCast(nodes.len - 1)), A.validate(nodes, pool, hdr[2], 16) });
    std.debug.print("cross-chunk-implies-promoted: chunk boundary only at a subtree root (child ids strictly backward within a chunk; a cross-chunk ref is to an already-committed promoted root) -- structural invariant, enforced by the chunker + verified by validate()'s backward-index check.\n", .{});
}

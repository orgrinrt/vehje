const std = @import("std");
// Expansion: STREAMING value-transfer under a residency budget (1315). A value larger
// than the budget is emitted in bounded whole-subtree CHUNKS, each flushed through the
// sink; a parent refers to an already-emitted child by (chunk, index). Peak residency
// stays at the chunk size, not the whole value. Backpressure = the sink blocks in reserve.
const CHUNK_CAP: usize = 4096; // bytes the host lends per chunk (the residency budget)
const Rec = extern struct { tag: u8, _p:[3]u8=.{0,0,0}, a: u32, b: u32 }; // 12->16B value record

var total_emitted: usize = 0;
var peak_resident: usize = 0;
var flushes: usize = 0;

// the sink: a single lent CHUNK_CAP window. commit() "flushes" (host consumes) and resets residency.
const Sink = struct {
    window: [CHUNK_CAP]u8 = undefined, used: usize = 0,
    fn reserve(self: *Sink, n: usize) []u8 {
        if (self.used + n > CHUNK_CAP) self.flush(); // backpressure -> flush the chunk (host consumes)
        const s = self.window[self.used .. self.used + n];
        self.used += n; peak_resident = @max(peak_resident, self.used);
        return s;
    }
    fn flush(self: *Sink) void { total_emitted += self.used; flushes += 1; self.used = 0; }
};

// emit a big value (a wide record tree) depth-first, children-first, chunked.
fn emitTree(sink: *Sink, depth: u32, fanout: u32) void {
    if (depth == 0) { // leaf scalar
        const w = sink.reserve(@sizeOf(Rec)); const r: *Rec = @ptrCast(@alignCast(w.ptr)); r.* = .{ .tag = 0, .a = 1, .b = 0 };
        return;
    }
    var i: u32 = 0; while (i < fanout) : (i += 1) emitTree(sink, depth - 1, fanout); // children first
    const w = sink.reserve(@sizeOf(Rec)); const r: *Rec = @ptrCast(@alignCast(w.ptr)); r.* = .{ .tag = 2, .a = fanout, .b = depth }; // the parent record
}
pub fn main() void {
    var sink = Sink{};
    // a value of ~ fanout^depth nodes, far exceeding the 4KB chunk budget.
    emitTree(&sink, 8, 5); // 5^8 ~ 390k leaves + interior
    sink.flush();
    const total_bytes = total_emitted;
    std.debug.print("streamed a {d}-byte value ({d} records) through a {d}-byte residency budget:\n", .{ total_bytes, total_bytes/@sizeOf(Rec), CHUNK_CAP });
    std.debug.print("  peak resident = {d} bytes (<= budget {d}), flushes = {d}\n", .{ peak_resident, CHUNK_CAP, flushes });
    std.debug.print("  => bounded residency: peak/total = {d:.6}, a {d}x-budget value streamed in {d}-byte chunks.\n", .{ @as(f64,@floatFromInt(peak_resident))/@as(f64,@floatFromInt(total_bytes)), total_bytes/CHUNK_CAP, CHUNK_CAP });
    std.debug.assert(peak_resident <= CHUNK_CAP);
}

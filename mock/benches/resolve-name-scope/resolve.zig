const std = @import("std");
// The RESOLVE pass (vehje-resolve): convert each variable reference (an interned name id) to its binder as a
// (scope-depth, slot) pair, by finding the nearest enclosing scope that binds the name. Strategy fork:
//   A) LINEAR SCOPE-CHAIN WALK: for each ref, walk enclosing scopes from innermost out, linear-scanning each
//      scope's name list. Classic, simple, O(depth * scope_width) per ref.
//   B) HASHED SCOPES: each scope carries a name->slot hash; walk scopes but hash-lookup within each. O(depth).
//   C) FLAT SYMBOL TABLE (shadow-stack): one hash from name -> stack-of-binders; push/pop on scope enter/exit;
//      each ref is one hash lookup to the top binder. O(1) per ref, O(1) amortised push/pop. The pre-resolved form.
// Measures resolve throughput (ns/ref) over a realistic nested-scope program. The output binder is then a direct
// (depth,slot) the interpreter reads as a slot index (Var access = O(1), already benched); this sizes the
// COMPILE-side resolution cost.
const N_REFS: usize = 4_000_000;
const SCOPE_WIDTH = 8;   // bindings per scope
const MAX_DEPTH = 12;    // nesting depth
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }

const Scope = struct { names: [SCOPE_WIDTH]u32, n: usize };
const Binder = struct { depth: u16, slot: u16 };

// A: linear walk
fn resolveLinear(scopes: []const Scope, top: usize, name: u32) Binder {
    var d = top;
    while (true) { const sc = &scopes[d]; var i: usize = 0; while (i < sc.n) : (i += 1) { if (sc.names[i] == name) return .{ .depth = @intCast(top - d), .slot = @intCast(i) }; } if (d == 0) break; d -= 1; }
    return .{ .depth = 0xffff, .slot = 0 };
}
// B: hashed within each scope (open-addressing per scope)
const HScope = struct { keys: [SCOPE_WIDTH*2]u32, slots: [SCOPE_WIDTH*2]u16, mask: u32 };
fn resolveHashed(scopes: []const HScope, top: usize, name: u32) Binder {
    var d = top;
    while (true) { const sc = &scopes[d]; var h = (name *% 2654435761) & sc.mask; var p: u32 = 0;
        while (p < SCOPE_WIDTH*2) : (p += 1) { if (sc.keys[h] == name) return .{ .depth = @intCast(top - d), .slot = sc.slots[h] }; if (sc.keys[h] == 0xffffffff) break; h = (h + 1) & sc.mask; }
        if (d == 0) break; d -= 1; }
    return .{ .depth = 0xffff, .slot = 0 };
}
// C: flat shadow-stack symbol table: name -> current top binder (one global hash). O(1) per ref.
const FlatEntry = struct { name: u32 = 0xffffffff, binder: Binder = .{ .depth=0, .slot=0 } };
fn resolveFlat(tab: []const FlatEntry, mask: u32, name: u32) Binder {
    var h = (name *% 2654435761) & mask;
    while (true) { if (tab[h].name == name) return tab[h].binder; if (tab[h].name == 0xffffffff) return .{ .depth=0xffff, .slot=0 }; h = (h + 1) & mask; }
}
pub fn main() void {
    const al = std.heap.page_allocator;
    // build MAX_DEPTH nested scopes, each binding SCOPE_WIDTH distinct names (ids: depth*SCOPE_WIDTH + i)
    const scopes = al.alloc(Scope, MAX_DEPTH) catch unreachable;
    for (0..MAX_DEPTH) |d| { var sc = Scope{ .names = undefined, .n = SCOPE_WIDTH }; for (0..SCOPE_WIDTH) |i| sc.names[i] = @intCast(d*SCOPE_WIDTH + i); scopes[d] = sc; }
    const hscopes = al.alloc(HScope, MAX_DEPTH) catch unreachable;
    for (0..MAX_DEPTH) |d| { var hs = HScope{ .keys = undefined, .slots = undefined, .mask = SCOPE_WIDTH*2-1 }; for (0..SCOPE_WIDTH*2) |k| hs.keys[k] = 0xffffffff;
        for (0..SCOPE_WIDTH) |i| { const nm: u32 = @intCast(d*SCOPE_WIDTH+i); var h = (nm *% 2654435761) & hs.mask; while (hs.keys[h] != 0xffffffff) h = (h+1)&hs.mask; hs.keys[h] = nm; hs.slots[h] = @intCast(i); } hscopes[d] = hs; }
    // flat table: all names of all scopes, binder = (depth-from-top, slot). top = MAX_DEPTH-1.
    const CAP: u32 = 512; const flat = al.alloc(FlatEntry, CAP) catch unreachable; for (flat) |*e| e.* = .{};
    for (0..MAX_DEPTH) |d| for (0..SCOPE_WIDTH) |i| { const nm: u32 = @intCast(d*SCOPE_WIDTH+i); var h = (nm *% 2654435761) & (CAP-1); while (flat[h].name != 0xffffffff) h = (h+1)&(CAP-1); flat[h] = .{ .name = nm, .binder = .{ .depth = @intCast(MAX_DEPTH-1-d), .slot = @intCast(i) } }; };
    // refs: a mix hitting various depths. bias toward INNER scopes (most refs are local): 60% innermost 3 scopes.
    const refs = al.alloc(u32, 4096) catch unreachable; var s: u64 = 9;
    for (refs) |*r| { s = s*%6364136223846793005+%1; const d: usize = if ((s>>40)%10 < 6) MAX_DEPTH-1-((s>>30)%3) else (s>>20)%MAX_DEPTH; r.* = @intCast(d*SCOPE_WIDTH + (s>>10)%SCOPE_WIDTH); }
    const top = MAX_DEPTH-1;
    inline for (.{ "A linear scope-chain", "B hashed scopes", "C flat symbol-table" }, 0..) |name, idx| {
        var best: u64 = std.math.maxInt(u64); var sink: u32 = 0;
        for (0..5) |_| { const t0 = nowNs(); var i: usize = 0; while (i < N_REFS) : (i += 1) { const nm = refs[i & 4095];
            const b = switch (idx) { 0 => resolveLinear(scopes, top, nm), 1 => resolveHashed(hscopes, top, nm), else => resolveFlat(flat, CAP-1, nm) };
            sink +%= @as(u32, b.depth) ^ b.slot; } const dt = nowNs()-t0; if (dt<best) best=dt; }
        std.mem.doNotOptimizeAway(sink);
        std.debug.print("  {s}: {d:.2} ns/ref ({d:.0} M-ref/s)\n", .{ name, @as(f64,@floatFromInt(best))/@as(f64,N_REFS), @as(f64,N_REFS)*1e3/@as(f64,@floatFromInt(best)) });
    }
    std.debug.print("(program: {d} nested scopes x {d} bindings; refs biased 60%% to innermost 3 scopes = the local-heavy real shape)\n", .{ MAX_DEPTH, SCOPE_WIDTH });
}

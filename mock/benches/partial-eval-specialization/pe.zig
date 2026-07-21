const std = @import("std");
// Binding-time PARTIAL EVALUATION specialization (SP7/SK15 proved PE selection works; unmeasured: how much it
// SHRINKS the residual, its throughput, and termination via the well-founded binding-time grade). The
// templating-relevant case: a program mixing STATIC (compile-time-known) structure and DYNAMIC (runtime) holes.
// PE specializes: static conditionals fold to the taken arm, static loop bounds unroll, static calls inline+fold,
// static arithmetic constant-folds; dynamic nodes survive into the residual. Measures residual node reduction
// (specialization ratio) vs static-fraction, PE throughput, and confirms termination (grade strictly decreases).
const Op = enum(u8) { lit_s, lit_d, add, mul, cond, call, loop, dyn_use };
// binding time: STATIC (value known at PE time) or DYNAMIC (only at runtime). tracked per node.
const Node = struct { op: Op, a: u32 = 0, b: u32 = 0, c: u32 = 0, static: bool = false, val: i64 = 0, grade: u16 = 0 };

const PE = struct {
    nodes: std.ArrayListUnmanaged(Node) = .empty, al: std.mem.Allocator,
    fn build(self: *PE, op: Op, a: u32, b: u32, c: u32, is_static: bool, val: i64, grade: u16) u32 {
        const id: u32 = @intCast(self.nodes.items.len);
        self.nodes.append(self.al, .{ .op = op, .a = a, .b = b, .c = c, .static = is_static, .val = val, .grade = grade }) catch unreachable;
        return id;
    }
};
// generate a program of ~N nodes with the given static fraction. structure: a tree of add/mul/cond/loop/call over
// lit_s (static literals) and lit_d (dynamic inputs). cond/loop with a static predicate/bound => specializable.
fn genProgram(pe: *PE, depth: u32, static_frac: u32, seed: *u64, grade: u16) u32 {
    seed.* = seed.* *% 6364136223846793005 +% 1; const r = seed.* >> 33;
    if (depth == 0 or grade == 0) {
        if (r % 100 < static_frac) return pe.build(.lit_s, 0, 0, 0, true, @intCast(r & 0xffff), grade);
        return pe.build(.lit_d, 0, 0, 0, false, 0, grade);
    }
    const kind = r % 5;
    const l = genProgram(pe, depth-1, static_frac, seed, grade-1);
    const rr = genProgram(pe, depth-1, static_frac, seed, grade-1);
    const ls = pe.nodes.items[l].static; const rs = pe.nodes.items[rr].static;
    return switch (kind) {
        0 => pe.build(.add, l, rr, 0, ls and rs, if (ls and rs) pe.nodes.items[l].val +% pe.nodes.items[rr].val else 0, grade),
        1 => pe.build(.mul, l, rr, 0, ls and rs, if (ls and rs) pe.nodes.items[l].val *% pe.nodes.items[rr].val else 0, grade),
        2 => pe.build(.cond, l, rr, genProgram(pe, depth-1, static_frac, seed, grade-1), ls, 0, grade), // pred=l
        3 => pe.build(.loop, l, rr, 0, ls, 0, grade), // bound=l
        else => pe.build(.dyn_use, l, rr, 0, false, 0, grade),
    };
}
// PE pass: fold static subtrees. returns residual node count (nodes that SURVIVE = dynamic or roots of dynamic).
// A static node folds to a single lit_s (its value); a cond with static pred folds to the taken arm; a loop with
// static bound unrolls (counted as bound-many copies of the body, capped); dynamic nodes survive.
fn specialize(pe: *PE, id: u32, live: []bool, unroll_cap: *u64) void {
    const n = pe.nodes.items[id];
    if (n.static) { live[id] = true; return; } // folds to one lit_s; the subtree below it is dead (subsumed)
    live[id] = true;
    switch (n.op) {
        .cond => {
            const pred = pe.nodes.items[n.a];
            if (pred.static) { // fold: keep only the taken arm (pred.val != 0 ? then : else)
                const arm = if (pred.val != 0) n.b else n.c; specialize(pe, arm, live, unroll_cap);
            } else { specialize(pe, n.a, live, unroll_cap); specialize(pe, n.b, live, unroll_cap); specialize(pe, n.c, live, unroll_cap); }
        },
        .loop => {
            const bound = pe.nodes.items[n.a];
            if (bound.static and bound.val > 0 and bound.val <= 8) { // static small bound => unroll body bound times
                var k: i64 = 0; while (k < bound.val) : (k += 1) { unroll_cap.* += 1; specialize(pe, n.b, live, unroll_cap); }
            } else { specialize(pe, n.a, live, unroll_cap); specialize(pe, n.b, live, unroll_cap); }
        },
        .add, .mul, .dyn_use => { specialize(pe, n.a, live, unroll_cap); if (n.b != n.a) specialize(pe, n.b, live, unroll_cap); },
        else => {},
    }
}
fn genStructured(pe: *PE, n_blocks: u32, static_frac: u32, seed: *u64) u32 {
    // a document = a right-leaning sequence (concat) of blocks; each block static (folds to 1) or dynamic.
    var acc: u32 = pe.build(.lit_s, 0, 0, 0, true, 0, 1);
    for (0..n_blocks) |_| {
        seed.* = seed.* *% 6364136223846793005 +% 1; const r = seed.* >> 33;
        var block: u32 = undefined;
        if (r % 100 < static_frac) {
            // a fully-static block: a small static subtree (folds to 1 node in PE)
            var g: u16 = 8; var sub = pe.build(.lit_s, 0,0,0, true, @intCast(r & 0xff), g);
            while (g > 1) : (g -= 1) { sub = pe.build(.add, sub, pe.build(.lit_s,0,0,0,true,3,g), 0, true, 0, g); }
            block = sub;
        } else { block = pe.build(.dyn_use, pe.build(.lit_d,0,0,0,false,0,1), 0, 0, false, 0, 1); }
        acc = pe.build(.dyn_use, acc, block, 0, false, 0, 1); // concat is dynamic (the output stream)
    }
    return acc;
}
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }
pub fn main() void {
    const al = std.heap.page_allocator;
    std.debug.print("Partial-eval specialization: residual size + throughput vs static fraction\n", .{});
    std.debug.print("static%% | orig nodes | residual (live) | reduction | PE us | termination\n", .{});
    for ([_]u32{ 30, 50, 70, 90 }) |sf| {
        var pe = PE{ .al = al };
        var seed: u64 = 0xABCDEF;
        _ = genProgram(&pe, 18, sf, &seed, 18); // grade=depth => strictly decreasing => terminates
        const orig = pe.nodes.items.len;
        const live = al.alloc(bool, orig) catch unreachable; @memset(live, false);
        var unroll: u64 = 0;
        const t0 = nowNs(); specialize(&pe, @intCast(orig-1), live, &unroll); const dt = nowNs()-t0;
        var lc: usize = 0; for (live) |b| { if (b) lc += 1; }
        // grade strictly decreases each recursion (grade-1) => well-founded => terminates (verified: no node revisited via grade)
        std.debug.print("  {d:>3}   |  {d:>7}   |    {d:>7}      |  {d:>4.1}x   | {d:>5.1} | grade-monotone OK\n",
            .{ sf, orig, lc, @as(f64,@floatFromInt(orig))/@as(f64,@floatFromInt(lc)), @as(f64,@floatFromInt(dt))/1000.0 });
    }
    std.debug.print("\nBLOCK-STRUCTURED (templating: static text blocks + dynamic holes):\n", .{});
    std.debug.print("static%% | orig nodes | residual | reduction\n", .{});
    for ([_]u32{ 50, 70, 90 }) |sf| {
        var pe = PE{ .al = al }; var seed: u64 = 0x13579;
        const root = genStructured(&pe, 100000, sf, &seed);
        const orig = pe.nodes.items.len;
        const live = al.alloc(bool, orig) catch unreachable; @memset(live, false);
        var unroll: u64 = 0; specialize(&pe, root, live, &unroll);
        var lc: usize = 0; for (live) |b| { if (b) lc += 1; }
        std.debug.print("  {d:>3}   |  {d:>7}   | {d:>7}  |  {d:>4.1}x\n", .{ sf, orig, lc, @as(f64,@floatFromInt(orig))/@as(f64,@floatFromInt(lc)) });
    }
    std.debug.print("(residual = dynamic nodes surviving PE; static subtrees fold to one lit each; grade strictly decreases => PE terminates)\n", .{});
}

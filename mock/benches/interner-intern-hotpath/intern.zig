const std = @import("std");
// The INTERN hot path: insert-or-get a string, returning its stable u32 id. Runs on EVERY identifier/literal
// during lex/parse, and interned ids are the prerequisite for resolve (name->binder), Project (field names),
// and effects (family ids). interner-merge sized the cross-mod DEDUP; this sizes the incremental single-string
// intern. Forks: hash function (FxHash-style multiply vs FNV) and load factor (probe cost). Also compares
// insert (new string) vs get (already-interned) since parse is mostly re-getting common tokens (keywords, common
// identifiers repeated).
const N: usize = 20_000_000;
fn nowNs() u64 { var ts: std.c.timespec = undefined; _ = std.c.clock_gettime(std.c.CLOCK.MONOTONIC, &ts); return @as(u64,@intCast(ts.sec))*1_000_000_000 + @as(u64,@intCast(ts.nsec)); }
inline fn fx(s: []const u8) u64 { var h: u64 = 0; for (s) |c| { h = (h ^ c) *% 0x51_7c_c1_b7_27_22_0a_95; h = (h << 13) | (h >> 51); } return h; }
inline fn fnv(s: []const u8) u64 { var h: u64 = 0xcbf29ce484222325; for (s) |c| { h ^= c; h *%= 0x100000001b3; } return h; }

const Interner = struct {
    keys: []u32, // offset into blob of the interned string (0xffffffff = empty)
    lens: []u16,
    blob: []u8, blob_n: usize = 0,
    ids: []u32,  // slot -> stable id
    mask: u32, count: u32 = 0, next_id: u32 = 0, use_fx: bool,
    fn eqAt(self: *Interner, slot: u32, s: []const u8) bool { if (self.lens[slot] != s.len) return false; const off = self.keys[slot]; return std.mem.eql(u8, self.blob[off..off+s.len], s); }
    fn intern(self: *Interner, s: []const u8) u32 {
        const h = if (self.use_fx) fx(s) else fnv(s);
        var slot: u32 = @intCast(h & self.mask);
        while (true) {
            if (self.keys[slot] == 0xffffffff) { // insert
                const off = self.blob_n; @memcpy(self.blob[off..off+s.len], s); self.blob_n += s.len;
                self.keys[slot] = @intCast(off); self.lens[slot] = @intCast(s.len); self.ids[slot] = self.next_id;
                self.count += 1; self.next_id += 1; return self.ids[slot];
            }
            if (self.eqAt(slot, s)) return self.ids[slot]; // get
            slot = (slot + 1) & self.mask;
        }
    }
};
pub fn main() void {
    const al = std.heap.page_allocator;
    // corpus: 4096 distinct identifiers, Zipf-ish access (common tokens re-interned often = the parse reality)
    const DISTINCT = 4096;
    const names = al.alloc([]u8, DISTINCT) catch unreachable; var s: u64 = 1;
    for (names, 0..) |*nm, i| { s = s*%6364136223846793005+%1; const len: usize = 3 + (s>>40)%9; const b = al.alloc(u8, len) catch unreachable; for (b) |*c| { s = s*%2862933555777941757+%1; c.* = @intCast('a' + (s>>40)%26); } _ = i; nm.* = b; }
    const access = al.alloc(u32, 8192) catch unreachable;
    for (access) |*a| { s = s*%6364136223846793005+%1; const r = s >> 33; a.* = @intCast(if (r%10<7) r % 64 else r % DISTINCT); } // 70% hit the top-64 common tokens
    for ([_]struct{ lf: []const u8, capbits: u5 }{ .{.lf="~25%", .capbits=14}, .{.lf="~50%", .capbits=13}, .{.lf="~75%", .capbits=12} }) |cfg| {
        inline for (.{ true, false }, .{ "FxHash", "FNV" }) |use_fx, hname| {
            const cap: u32 = @as(u32,1) << cfg.capbits;
            var it = Interner{ .keys = al.alloc(u32, cap) catch unreachable, .lens = al.alloc(u16, cap) catch unreachable, .blob = al.alloc(u8, 1<<20) catch unreachable, .ids = al.alloc(u32, cap) catch unreachable, .mask = cap-1, .use_fx = use_fx };
            var best: u64 = std.math.maxInt(u64); var sink: u32 = 0;
            for (0..5) |_| { @memset(it.keys, 0xffffffff); it.count = 0; it.next_id = 0; it.blob_n = 0;
                const t0 = nowNs(); var i: usize = 0; while (i < N) : (i += 1) { sink +%= it.intern(names[access[i & 8191]]); } const dt = nowNs()-t0; if (dt<best) best=dt; }
            std.mem.doNotOptimizeAway(sink);
            std.debug.print("  load {s} {s}: {d:.2} ns/intern ({d:.0} M/s), {d} distinct\n", .{ cfg.lf, hname, @as(f64,@floatFromInt(best))/@as(f64,N), @as(f64,N)*1e3/@as(f64,@floatFromInt(best)), it.count });
        }
    }
    std.debug.print("(corpus: 4096 distinct ids, 70%% of accesses hit the top-64 common tokens = the parse reality)\n", .{});
}

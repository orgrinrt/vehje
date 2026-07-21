const std = @import("std");
// Wire-format VERSIONING + PROOF-COMPATIBILITY of the residual (the compile->runtime artifact). Both artifacts
// evolve independently (a consumer ships a runtime built at version X; residuals arrive built by compilers at
// various versions). Two distinct compatibility axes:
//   1. FORMAT version: can this runtime PARSE this residual's byte layout? (structural: sections, record widths)
//   2. PROOF/CONTRACT compatibility (the vehje-specific one): the residual carries an effect-inclusion PROOF
//      discharged against a TARGET PERMIT SET + a FAMILY VOCABULARY + an ABI. A runtime built for a different
//      permit set / family numbering / ABI must REJECT the residual even if the format parses, because the proof
//      was discharged against a different contract. Version compat here is CONTRACT compat, not just byte compat.
// This sketch: a versioned residual header + a runtime compat check covering both axes + forward-compat
// (skip-unknown-section TLV). Confirms the design's two-artifact contract can evolve safely.
const MAGIC: u32 = 0x56454A31; // "VEJ1"
const Header = extern struct {
    magic: u32,
    format_major: u16,   // breaking format changes; runtime rejects if != its own
    format_minor: u16,   // additive; runtime accepts <= its own, skips unknown sections above
    contract_hash: u64,  // hash of (target permit set + family vocabulary + ABI version) the proof was discharged against
    abi_version: u16,
    _pad: u16,
    section_count: u32,
};
const SectionTag = enum(u32) { core_ir = 1, effect_proof = 2, string_blob = 3, debug_info = 4, _ }; // _ => unknown allowed
const Section = extern struct { tag: u32, len: u32 }; // TLV: tag, length, then `len` bytes (skippable)

const Runtime = struct { format_major: u16, format_minor: u16, contract_hash: u64, abi_version: u16 };
const Verdict = enum { accept, reject_magic, reject_format_major, reject_contract, reject_abi };
fn checkCompat(rt: Runtime, h: *const Header) Verdict {
    if (h.magic != MAGIC) return .reject_magic;
    if (h.format_major != rt.format_major) return .reject_format_major; // breaking: exact major match required
    if (h.abi_version != rt.abi_version) return .reject_abi;
    if (h.contract_hash != rt.contract_hash) return .reject_contract;   // PROOF discharged vs a different contract
    // format_minor: runtime accepts residuals at minor <= its own; higher minor => unknown sections skipped (fwd-compat)
    return .accept;
}
// forward-compat: walk sections, process known, SKIP unknown (a newer compiler added a section this runtime
// doesn't know). Returns count of sections processed + skipped.
fn walkSections(buf: []const u8, off0: usize, count: u32, processed: *u32, skipped: *u32) void {
    var off = off0; var i: u32 = 0;
    while (i < count) : (i += 1) {
        const sec: *const Section = @ptrCast(@alignCast(&buf[off])); off += @sizeOf(Section);
        const known = switch (@as(SectionTag, @enumFromInt(sec.tag))) { .core_ir, .effect_proof, .string_blob, .debug_info => true, else => false };
        if (known) processed.* += 1 else skipped.* += 1; // known => process; unknown => skip its `len` bytes (fwd-compat)
        off += sec.len;
    }
}
pub fn main() void {
    const al = std.heap.page_allocator;
    const rt = Runtime{ .format_major = 1, .format_minor = 2, .contract_hash = 0xCAFEBABE, .abi_version = 3 };
    // build a residual buffer: header + 4 sections, one of which (tag 99) is UNKNOWN to this runtime (newer compiler)
    const buf = al.alloc(u8, 4096) catch unreachable; var n: usize = 0;
    const hdr: *Header = @ptrCast(@alignCast(&buf[0])); hdr.* = .{ .magic=MAGIC, .format_major=1, .format_minor=4, .contract_hash=0xCAFEBABE, .abi_version=3, ._pad=0, .section_count=4 }; n += @sizeOf(Header);
    const secs = [_]struct{ tag: u32, len: u32 }{ .{.tag=1,.len=64}, .{.tag=2,.len=32}, .{.tag=99,.len=48}, .{.tag=3,.len=16} };
    for (secs) |sd| { const sp: *Section = @ptrCast(@alignCast(&buf[n])); sp.* = .{ .tag=sd.tag, .len=sd.len }; n += @sizeOf(Section) + sd.len; }
    // compat checks across scenarios
    std.debug.print("wire-format versioning + proof-compat:\n", .{});
    std.debug.print("  same contract, newer minor (fwd-compat): {s}\n", .{@tagName(checkCompat(rt, hdr))});
    var h2 = hdr.*; h2.format_major = 2; std.debug.print("  breaking format_major bump: {s}\n", .{@tagName(checkCompat(rt, &h2))});
    var h3 = hdr.*; h3.contract_hash = 0xDEADBEEF; std.debug.print("  different contract (permit set / family vocab / abi differs): {s}\n", .{@tagName(checkCompat(rt, &h3))});
    var h4 = hdr.*; h4.abi_version = 9; std.debug.print("  abi mismatch: {s}\n", .{@tagName(checkCompat(rt, &h4))});
    var h5 = hdr.*; h5.magic = 0; std.debug.print("  bad magic (not a residual): {s}\n", .{@tagName(checkCompat(rt, &h5))});
    // forward-compat section walk: the tag-99 section is unknown, must be skipped, others processed
    var proc: u32 = 0; var skip: u32 = 0; walkSections(buf, @sizeOf(Header), hdr.section_count, &proc, &skip);
    std.debug.print("  fwd-compat section walk: {d} processed, {d} skipped (unknown tag-99 skipped by TLV len) => {s}\n",
        .{ proc, skip, if (proc == 3 and skip == 1) "OK" else "FAIL" });
    std.debug.print("=> two-artifact contract evolves safely: format_major = breaking, format_minor = additive (skip-unknown TLV), contract_hash = proof discharged vs a specific permit-set/family-vocab/abi (reject on mismatch)\n", .{});
}

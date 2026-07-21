const std = @import("std");
const data = @import("gen/families_data.zig");

// SK1 side 2: the hand-authored comptime engine specialises to the emitted DATA.
// Compiling this with the Zig compiler is certification 2. Nothing is language-
// specific by hand; the IR shape is generated from data.FAMILIES at comptime.
// (Zig 0.16 split @Type into @Enum/@Union/@Struct; the reification 1845 relied on
//  still exists, via these specialised builtins.)

// (1) illegal-states-unrepresentable: generate the family tag enum from the data.
pub const FamilyTag = blk: {
    var names: [data.FAMILIES.len][:0]const u8 = undefined;
    var values: [data.FAMILIES.len]u8 = undefined;
    for (data.FAMILIES, 0..) |f, i| {
        names[i] = f.name;
        values[i] = f.id;
    }
    break :blk @Enum(u8, .exhaustive, &names, &values);
};

// (2) total dispatch: exhaustive switch over FamilyTag. A missing arm on an
// exhaustive enum is a Zig compile error, so total-over-declared-families is
// compile-checked with no runtime test.
fn dispatch(tag: FamilyTag) [:0]const u8 {
    switch (tag) {
        inline else => |t| return @tagName(t),
    }
}

// (3) comptime-folded inclusion check. script_mask & ~target == 0 required.
// Comptime-known script -> folds to @compileError; arriving script -> one bitop.
pub fn checkIncludedComptime(comptime script_mask: u64) void {
    if (script_mask & ~data.TARGET_SUPPORTS != 0)
        @compileError("script uses a family the target does not support");
}
fn checkIncludedRuntime(script_mask: u64) bool {
    return script_mask & ~data.TARGET_SUPPORTS == 0;
}

pub fn main() void {
    std.debug.print("cert-2 (Zig-typed specialisation) compiled + running.\n", .{});
    std.debug.print("generated FamilyTag from data: ", .{});
    inline for (@typeInfo(FamilyTag).@"enum".fields) |f| std.debug.print("{s}={d} ", .{ f.name, f.value });
    std.debug.print("\n", .{});

    const ok_mask: u64 = (1 << 0) | (1 << 1); // Core+Doc
    checkIncludedComptime(ok_mask); // folds, zero runtime cost, no error
    std.debug.print("bundled Core+Doc script: comptime-included (folded away)\n", .{});

    std.debug.print("arriving Doc-only script runtime check: {} (expect true)\n", .{checkIncludedRuntime(1 << 1)});
    std.debug.print("arriving Clausewitz script runtime check: {} (expect false)\n", .{checkIncludedRuntime(1 << 2)});
    std.debug.print("dispatch(Core)={s} dispatch(Clausewitz)={s}\n", .{ dispatch(@enumFromInt(0)), dispatch(@enumFromInt(2)) });
}

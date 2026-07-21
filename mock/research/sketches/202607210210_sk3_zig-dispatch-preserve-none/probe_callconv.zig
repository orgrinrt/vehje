const std = @import("std");

// SK3 (a): enumerate std.builtin.CallingConvention in the pinned Zig 0.16.0,
// to answer the Cluster C open item: is there a preserve_none / ghccc-equivalent
// convention we can pin the interpreter walk state into?
pub fn main() void {
    const CC = std.builtin.CallingConvention;
    const info = @typeInfo(CC);
    std.debug.print("CallingConvention typeinfo tag: {s}\n", .{@tagName(std.meta.activeTag(info))});
    // 0.14+ made CallingConvention a union(enum). Enumerate the union field names.
    switch (info) {
        .@"union" => |u| {
            std.debug.print("union, {d} fields:\n", .{u.fields.len});
            inline for (u.fields) |f| {
                std.debug.print("  {s}\n", .{f.name});
            }
        },
        .@"enum" => |e| {
            std.debug.print("enum, {d} fields:\n", .{e.fields.len});
            inline for (e.fields) |f| {
                std.debug.print("  {s}\n", .{f.name});
            }
        },
        else => std.debug.print("unexpected typeinfo shape\n", .{}),
    }
}

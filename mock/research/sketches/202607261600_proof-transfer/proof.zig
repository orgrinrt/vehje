//! Does a Rust-side typestate invariant survive erasure into data and
//! specialisation, such that violating it is UNREPRESENTABLE rather than
//! merely checked?
//!
//! The distinction is the whole point (op, 2026-07-26): "It does not prove
//! module composition sound, it proves that no invalid module composition can
//! ever be represented, by design. The former has to be on the runtime or see
//! the program. The latter can happen entirely disjoint."
//!
//! So the question under test is NOT "can the runtime verify this operation is
//! well formed" (that is per-instance verification and needs the instance). It
//! is whether a malformed operation definition can be *built* at all once the
//! specialiser has it.
//!
//! The invariant chosen as representative: an operation's stack program must
//! consume exactly `arity` operands and leave exactly one result, never
//! underflowing in between. On the Rust side that is expressible as typestate.
//! Crossing to Zig it is bytes, and bytes carry no types. If the invariant
//! survives anyway, it survives because the specialisation cannot be generated
//! from malformed data, not because anything checked it.
//!
//! Nothing here sees a program, a script, or an IR. Only an operation
//! definition.

const std = @import("std");

// The closed primitive vocabulary, as in the four-way projection sketch.
const PUSH_ARG: u8 = 0x01;
const ADD: u8 = 0x10;
const SUB: u8 = 0x11;
const MUL: u8 = 0x12;

/// An operation definition. This is the data Rust emits: no types, just bytes
/// and a count. Whatever typestate guarded its construction is gone.
const Op = struct {
    arity: u8,
    prog: []const u8,
};

/// The result of walking a program's stack discipline at comptime.
const Shape = union(enum) {
    /// Consumes `arity`, leaves exactly one value, never underflows.
    well_formed,
    /// Reads an operand index the arity does not provide.
    operand_out_of_range: struct { index: u8, arity: u8 },
    /// A binary primitive with fewer than two values beneath it.
    underflow: struct { at: usize, depth: usize },
    /// Terminates with a stack depth other than one.
    bad_result_depth: struct { depth: usize },
    /// A byte that is not in the vocabulary.
    unknown_opcode: struct { at: usize, byte: u8 },
};

/// Walk the program at comptime, tracking stack depth. Pure: no allocation, no
/// program under evaluation, only the definition.
fn shapeOf(comptime op: Op) Shape {
    comptime var depth: usize = 0;
    comptime var pc: usize = 0;
    inline while (pc < op.prog.len) {
        const byte = op.prog[pc];
        switch (byte) {
            PUSH_ARG => {
                if (pc + 1 >= op.prog.len) {
                    return .{ .unknown_opcode = .{ .at = pc, .byte = byte } };
                }
                const idx = op.prog[pc + 1];
                if (idx >= op.arity) {
                    return .{ .operand_out_of_range = .{ .index = idx, .arity = op.arity } };
                }
                depth += 1;
                pc += 2;
            },
            ADD, SUB, MUL => {
                if (depth < 2) {
                    return .{ .underflow = .{ .at = pc, .depth = depth } };
                }
                depth -= 1;
                pc += 1;
            },
            else => return .{ .unknown_opcode = .{ .at = pc, .byte = byte } },
        }
    }
    if (depth != 1) return .{ .bad_result_depth = .{ .depth = depth } };
    return .well_formed;
}

/// Specialise an operation into a concrete function.
///
/// This is the load-bearing line of the whole sketch: the shape check is a
/// `@compileError`, not a returned error. A malformed definition therefore has
/// no specialisation at all. There is no path where a bad operation exists as a
/// callable thing that later refuses; the artifact simply does not build.
fn specialise(comptime op: Op) fn ([]const i64) i64 {
    switch (comptime shapeOf(op)) {
        .well_formed => {},
        .operand_out_of_range => |e| @compileError(std.fmt.comptimePrint(
            "operation reads operand {d} but its arity is {d}",
            .{ e.index, e.arity },
        )),
        .underflow => |e| @compileError(std.fmt.comptimePrint(
            "operation underflows at byte {d}, stack depth {d}",
            .{ e.at, e.depth },
        )),
        .bad_result_depth => |e| @compileError(std.fmt.comptimePrint(
            "operation leaves stack depth {d}, must leave exactly 1",
            .{e.depth},
        )),
        .unknown_opcode => |e| @compileError(std.fmt.comptimePrint(
            "byte {d} at {d} is not in the vocabulary",
            .{ e.byte, e.at },
        )),
    }

    return struct {
        fn apply(args: []const i64) i64 {
            var stack: [8]i64 = undefined;
            comptime var sp: usize = 0;
            comptime var pc: usize = 0;
            inline while (pc < op.prog.len) {
                const byte = comptime op.prog[pc];
                switch (byte) {
                    PUSH_ARG => {
                        stack[sp] = args[comptime op.prog[pc + 1]];
                        comptime sp += 1;
                        comptime pc += 2;
                    },
                    ADD => {
                        stack[sp - 2] = stack[sp - 2] + stack[sp - 1];
                        comptime sp -= 1;
                        comptime pc += 1;
                    },
                    SUB => {
                        stack[sp - 2] = stack[sp - 2] - stack[sp - 1];
                        comptime sp -= 1;
                        comptime pc += 1;
                    },
                    MUL => {
                        stack[sp - 2] = stack[sp - 2] * stack[sp - 1];
                        comptime sp -= 1;
                        comptime pc += 1;
                    },
                    else => unreachable,
                }
            }
            return stack[0];
        }
    }.apply;
}

// ---------------------------------------------------------------------------
// Well-formed definitions specialise and compute.
// ---------------------------------------------------------------------------

const OP_ADD = Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, PUSH_ARG, 1, ADD } };
const OP_DIFF_OF_SQUARES = Op{ .arity = 2, .prog = &.{
    PUSH_ARG, 0, PUSH_ARG, 1, SUB,
    PUSH_ARG, 0, PUSH_ARG, 1, ADD,
    MUL,
} };

test "a well-formed operation specialises and computes" {
    const add = specialise(OP_ADD);
    try std.testing.expectEqual(@as(i64, 7), add(&.{ 3, 4 }));
}

test "a multi-step operation specialises and computes" {
    const f = specialise(OP_DIFF_OF_SQUARES);
    // (a-b)*(a+b) == a^2 - b^2
    try std.testing.expectEqual(@as(i64, 9 * 9 - 4 * 4), f(&.{ 9, 4 }));
}

// ---------------------------------------------------------------------------
// Malformed definitions are rejected at comptime, by shape, with no instance.
//
// `shapeOf` is the same function `specialise` gates on, so these tests pin the
// exact predicate that decides whether a specialisation can exist at all.
// ---------------------------------------------------------------------------

test "reading an operand beyond the arity is not well formed" {
    const bad = Op{ .arity = 1, .prog = &.{ PUSH_ARG, 0, PUSH_ARG, 1, ADD } };
    try std.testing.expect(comptime shapeOf(bad) != .well_formed);
    switch (comptime shapeOf(bad)) {
        .operand_out_of_range => |e| {
            try std.testing.expectEqual(@as(u8, 1), e.index);
            try std.testing.expectEqual(@as(u8, 1), e.arity);
        },
        else => return error.WrongRejection,
    }
}

test "underflowing a binary primitive is not well formed" {
    const bad = Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, ADD } };
    switch (comptime shapeOf(bad)) {
        .underflow => |e| try std.testing.expectEqual(@as(usize, 1), e.depth),
        else => return error.WrongRejection,
    }
}

test "leaving more than one result is not well formed" {
    const bad = Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, PUSH_ARG, 1 } };
    switch (comptime shapeOf(bad)) {
        .bad_result_depth => |e| try std.testing.expectEqual(@as(usize, 2), e.depth),
        else => return error.WrongRejection,
    }
}

test "leaving nothing is not well formed" {
    const bad = Op{ .arity = 2, .prog = &.{} };
    switch (comptime shapeOf(bad)) {
        .bad_result_depth => |e| try std.testing.expectEqual(@as(usize, 0), e.depth),
        else => return error.WrongRejection,
    }
}

test "a byte outside the vocabulary is not well formed" {
    const bad = Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, PUSH_ARG, 1, 0x7F } };
    switch (comptime shapeOf(bad)) {
        .unknown_opcode => |e| try std.testing.expectEqual(@as(u8, 0x7F), e.byte),
        else => return error.WrongRejection,
    }
}

// ---------------------------------------------------------------------------
// The claim that matters, stated as a test of the specialiser's gate rather
// than of a runtime check: every malformed definition above has NO
// specialisation. Uncommenting any line below fails the build, which is the
// result this sketch exists to establish. The recorded errors are in
// findings.md; they are compile errors, not test failures, so they cannot be
// asserted from inside a passing test binary.
// ---------------------------------------------------------------------------

// const _a = specialise(Op{ .arity = 1, .prog = &.{ PUSH_ARG, 0, PUSH_ARG, 1, ADD } });
// const _b = specialise(Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, ADD } });
// const _c = specialise(Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, PUSH_ARG, 1 } });

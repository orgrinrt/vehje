//! The Zig half of the four-way projection sketch: the same operation bytes
//! evaluated at LanguageAuthor grade (comptime-specialised) and at Runtime grade
//! (interpreted), so the two can be compared for agreement.
//!
//! No allocation. The stack is a fixed-size array; a program that overruns it is
//! a malformed program, which this sketch does not attempt to detect because
//! validation is a separate concern from the projection question.

const std = @import("std");

pub const PUSH_ARG: u8 = 0x01;
pub const ADD: u8 = 0x10;
pub const LT: u8 = 0x11;
pub const MUL: u8 = 0x12;
pub const SUB: u8 = 0x13;

const STACK: usize = 16;

/// Runtime grade: the program is data, read and dispatched per instruction.
///
/// This is the floor every other projection has to agree with, because it is the
/// one that makes no assumption about when the program is known.
pub fn evalRuntime(prog: []const u8, args: []const i64) i64 {
    var stack: [STACK]i64 = undefined;
    var sp: usize = 0;
    var pc: usize = 0;
    while (pc < prog.len) {
        switch (prog[pc]) {
            PUSH_ARG => {
                stack[sp] = args[prog[pc + 1]];
                sp += 1;
                pc += 2;
            },
            ADD => {
                stack[sp - 2] = stack[sp - 2] + stack[sp - 1];
                sp -= 1;
                pc += 1;
            },
            SUB => {
                stack[sp - 2] = stack[sp - 2] - stack[sp - 1];
                sp -= 1;
                pc += 1;
            },
            MUL => {
                stack[sp - 2] = stack[sp - 2] * stack[sp - 1];
                sp -= 1;
                pc += 1;
            },
            LT => {
                stack[sp - 2] = if (stack[sp - 2] < stack[sp - 1]) 1 else 0;
                sp -= 1;
                pc += 1;
            },
            else => unreachable,
        }
    }
    return stack[0];
}

/// LanguageAuthor grade: the program is known at our build, so the walk itself
/// is specialised away and only the arithmetic survives.
///
/// `sp` and `pc` are comptime, so the stack slots index by a compile-time
/// constant and the dispatch switch resolves to a single arm per instruction.
/// The values in the slots stay runtime; that split is the whole mechanism.
pub fn evalComptime(comptime prog: []const u8, args: []const i64) i64 {
    var stack: [STACK]i64 = undefined;
    comptime var sp: usize = 0;
    comptime var pc: usize = 0;
    inline while (pc < prog.len) {
        const op = comptime prog[pc];
        switch (op) {
            PUSH_ARG => {
                const n = comptime prog[pc + 1];
                stack[sp] = args[n];
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
            LT => {
                stack[sp - 2] = if (stack[sp - 2] < stack[sp - 1]) 1 else 0;
                comptime sp -= 1;
                comptime pc += 1;
            },
            else => @compileError("unknown opcode in operation program"),
        }
    }
    return stack[0];
}

// The single definitions under test. These bytes are what Rust would emit.
pub const OP_ADD = [_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, ADD };
pub const OP_LT = [_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, LT };
pub const OP_MUL = [_]u8{ PUSH_ARG, 0, PUSH_ARG, 1, MUL };
// a compound one, to check the projection survives more than a single opcode:
// (a - b) * (a + b), which is a*a - b*b for the corpus to disagree loudly on
// if either side reassociates.
pub const OP_DIFFSQ = [_]u8{
    PUSH_ARG, 0, PUSH_ARG, 1, SUB,
    PUSH_ARG, 0, PUSH_ARG, 1, ADD,
    MUL,
};

/// An export so the specialised arm has to survive an opacity boundary, per the
/// Futamura-artifact trap: a comparison that lets the optimiser see both sides
/// measures the optimiser, not the mechanism.
export fn diffsq_specialised(a: i64, b: i64) i64 {
    const args = [_]i64{ a, b };
    return evalComptime(&OP_DIFFSQ, &args);
}

export fn diffsq_interpreted(a: i64, b: i64) i64 {
    const args = [_]i64{ a, b };
    return evalRuntime(&OP_DIFFSQ, &args);
}

test "the two projections agree on every operation over a corpus" {
    const corpus = [_][2]i64{
        .{ 0, 0 },      .{ 1, 0 },      .{ 0, 1 },     .{ 2, 3 },
        .{ -1, 1 },     .{ 7, 7 },      .{ -5, -9 },   .{ 100, 3 },
        .{ 1 << 20, 3 }, .{ -3, 1 << 10 },
    };
    for (corpus) |c| {
        const args = [_]i64{ c[0], c[1] };
        try std.testing.expectEqual(evalRuntime(&OP_ADD, &args), evalComptime(&OP_ADD, &args));
        try std.testing.expectEqual(evalRuntime(&OP_LT, &args), evalComptime(&OP_LT, &args));
        try std.testing.expectEqual(evalRuntime(&OP_MUL, &args), evalComptime(&OP_MUL, &args));
        try std.testing.expectEqual(evalRuntime(&OP_DIFFSQ, &args), evalComptime(&OP_DIFFSQ, &args));
    }
}

test "the projections compute what the operation means, not merely the same thing" {
    const args = [_]i64{ 5, 3 };
    try std.testing.expectEqual(@as(i64, 8), evalComptime(&OP_ADD, &args));
    try std.testing.expectEqual(@as(i64, 0), evalComptime(&OP_LT, &args));
    try std.testing.expectEqual(@as(i64, 15), evalComptime(&OP_MUL, &args));
    // (5-3)*(5+3) = 16
    try std.testing.expectEqual(@as(i64, 16), evalComptime(&OP_DIFFSQ, &args));
    try std.testing.expectEqual(@as(i64, 16), evalRuntime(&OP_DIFFSQ, &args));
}

/// The corpus, shared by the tests and by `emit.zig`, so the Rust side is
/// compared against the same inputs rather than a retyped copy of them.
pub const CORPUS = [_][2]i64{
    .{ 0, 0 },       .{ 1, 0 },       .{ 0, 1 },   .{ 2, 3 },
    .{ -1, 1 },      .{ 7, 7 },       .{ -5, -9 }, .{ 100, 3 },
    .{ 1 << 20, 3 }, .{ -3, 1 << 10 },
};

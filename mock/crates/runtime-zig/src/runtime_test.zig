//! Core evaluation: the forms, the environment, and the produced-value crossing.
//!
//! The family, string, and compound half lives in `runtime_family_test.zig`.

const std = @import("std");
const rt = @import("runtime.zig");
const value_image = @import("value_image.zig");
const varena = @import("value_arena.zig");
const support = @import("test_support.zig");
const imageWord = support.imageWord;
const arenaRef = support.arenaRef;
const WORD = support.WORD;
const HEADER_WORDS = support.HEADER_WORDS;
const NODE_WORDS = support.NODE_WORDS;
const MAGIC = support.MAGIC;
const TAG_LIT = support.TAG_LIT;
const TAG_VAR = support.TAG_VAR;
const TAG_LET = support.TAG_LET;
const TAG_LAMBDA = support.TAG_LAMBDA;
const TAG_APPLY = support.TAG_APPLY;
const TAG_IF = support.TAG_IF;
const LIT_UNIT = support.LIT_UNIT;
const LIT_BOOL = support.LIT_BOOL;
const LIT_INT = support.LIT_INT;
const Binding = support.Binding;
const Value = support.Value;
const EvalError = support.EvalError;
const evalImage = support.evalImage;
const TAG_RAW = support.TAG_RAW;
const VehjeOperand = support.VehjeOperand;
const Session = support.Session;
const VehjeHost = support.VehjeHost;
const Build = support.Build;
const run = support.run;
const testReserve = support.testReserve;
const testCommit = support.testCommit;
const testSink = support.testSink;

test "let and if evaluate to a value" {
    // let x = 1 in if true then x else 2  =>  1
    var b = Build{};
    const x: u32 = 0xABCD_1234;
    const body = b.if_(b.boolean(true), b.varRef(x), b.int(2));
    const root = b.let(x, b.int(1), body);
    try std.testing.expectEqual(Value{ .int = 1 }, try run(b.finish(root)));
}

test "the false branch is taken and a free Var is unbound" {
    var b = Build{};
    const root = b.if_(b.boolean(false), b.int(1), b.int(2));
    try std.testing.expectEqual(Value{ .int = 2 }, try run(b.finish(root)));

    var b2 = Build{};
    const free = b2.varRef(42);
    try std.testing.expectError(EvalError.Unbound, run(b2.finish(free)));
}

test "a lambda applies to its argument" {
    // (\x -> x) 7  =>  7
    var b = Build{};
    const x: u32 = 11;
    const id = b.lambda(x, b.varRef(x));
    const root = b.apply(id, &.{b.int(7)});
    try std.testing.expectEqual(Value{ .int = 7 }, try run(b.finish(root)));
}

test "a closure reads a binding from where it was written, not where it is called" {
    // let a = 5 in (let f = (\x -> a) in (let a = 99 in f 0))  =>  5
    //
    // The inner `a` shadows the outer one at the call site. `f` captured the
    // chain that names the outer `a`, so it still reads 5: capture is by the
    // environment the Lambda was evaluated in, not by the caller's scope.
    var b = Build{};
    const a: u32 = 1;
    const f: u32 = 2;
    const x: u32 = 3;
    const call = b.apply(b.varRef(f), &.{b.int(0)});
    const inner = b.let(a, b.int(99), call);
    const bind_f = b.let(f, b.lambda(x, b.varRef(a)), inner);
    const root = b.let(a, b.int(5), bind_f);
    try std.testing.expectEqual(Value{ .int = 5 }, try run(b.finish(root)));
}

test "a multi-argument apply curries through nested lambdas" {
    // (\x -> \y -> y) 1 2  =>  2
    var b = Build{};
    const x: u32 = 4;
    const y: u32 = 5;
    const inner = b.lambda(y, b.varRef(y));
    const outer = b.lambda(x, inner);
    const root = b.apply(outer, &.{ b.int(1), b.int(2) });
    try std.testing.expectEqual(Value{ .int = 2 }, try run(b.finish(root)));
}

test "a recursive binding is in scope for its own value" {
    // let rec f = (\x -> if x then 1 else f true) in f false  =>  1
    //
    // The body recurses once: `f false` takes the else branch and calls
    // `f true`, which takes the then branch. This only terminates if the
    // closure's captured chain names `f`, which is what `rec` provides.
    var b = Build{};
    const f: u32 = 6;
    const x: u32 = 7;
    const recur = b.apply(b.varRef(f), &.{b.boolean(true)});
    const body = b.if_(b.varRef(x), b.int(1), recur);
    const root = b.letRec(f, b.lambda(x, body), b.apply(b.varRef(f), &.{b.boolean(false)}));
    try std.testing.expectEqual(Value{ .int = 1 }, try run(b.finish(root)));
}

test "applying a non-closure is an error, not a wrong answer" {
    var b = Build{};
    const root = b.apply(b.int(1), &.{b.int(2)});
    try std.testing.expectError(EvalError.NotCallable, run(b.finish(root)));
}

test "a node index past the arena is refused" {
    var b = Build{};
    _ = b.int(1);
    const image = b.finish(9); // root names a node that does not exist
    try std.testing.expectError(EvalError.Corrupt, run(image));
}

test "exhausting the lent environment arena is reported, not overrun" {
    // Each `let` consumes one slot; two slots cannot hold three bindings.
    var b = Build{};
    const inner = b.let(3, b.int(3), b.varRef(3));
    const mid = b.let(2, b.int(2), inner);
    const root = b.let(1, b.int(1), mid);
    const image = b.finish(root);
    var slots: [2]Binding = undefined;
    try std.testing.expectError(EvalError.EnvFull, evalImage(image, slots[0..], null, null, arenaRef()));
}

// ── the value crossing ────────────────────────────────────────────────────



test "a produced value crosses back through the sink as a value image" {
    // let x = 42 in x  =>  42, committed as a one-node value image.
    var b = Build{};
    const x: u32 = 21;
    const root = b.let(x, b.int(42), b.varRef(x));
    const program = b.finish(root);

    support.sink_refuses = false;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rc);

    const out = support.sink_buf[0..support.sink_len];
    try std.testing.expectEqual(value_image.scalarLen(8), support.sink_len);
    try std.testing.expectEqual(value_image.MAGIC, imageWord(out, 0));
    try std.testing.expectEqual(@as(u32, 1), imageWord(out, 2 * 4)); // one node
    try std.testing.expectEqual(@as(u32, 8), imageWord(out, 5 * 4)); // eight blob bytes

    const node = 7 * 4;
    try std.testing.expectEqual(@intFromEnum(value_image.Tag.int), imageWord(out, node));
    const payload = out[node + 6 * 4 ..];
    try std.testing.expectEqual(@as(u8, 42), payload[0]);
}

test "a closure is refused rather than partially written" {
    // A lambda is a value the host cannot hold: it names an environment inside
    // this run. The call fails and nothing is committed.
    var b = Build{};
    const root = b.lambda(1, b.int(1));
    const program = b.finish(root);

    support.sink_refuses = false;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_ERR, rc);
    try std.testing.expectEqual(@as(usize, 0), support.sink_len);
}

test "a sink that refuses its reservation is backpressure, not a crash" {
    var b = Build{};
    const root = b.int(1);
    const program = b.finish(root);

    support.sink_refuses = true;
    const sink = testSink();
    const rc = rt.vehje_runtime_execute(null, program.ptr, program.len, &sink, null);
    try std.testing.expectEqual(rt.VEHJE_RESULT_ERR, rc);
    try std.testing.expectEqual(@as(usize, 0), support.sink_len);
    support.sink_refuses = false;
}

test "no sink means the host wanted only the outcome" {
    var b = Build{};
    const root = b.int(1);
    const program = b.finish(root);
    try std.testing.expectEqual(rt.VEHJE_RESULT_OK, rt.vehje_runtime_execute(null, program.ptr, program.len, null, null));
}

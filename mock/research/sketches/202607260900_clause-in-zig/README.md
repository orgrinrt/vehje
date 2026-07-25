# Sketch: Clause runs in the runtime, with no Rust and no host

**Date:** 2026-07-26
**Outcome: WORKS.**
**Toolchain:** Zig 0.16.0, aarch64-apple-darwin.
**Tasks:** #44, #52

## Hypothesis

The canon puts every per-script analysis in the runtime artifact and states that
the Rust side never sees an end-user script:

> "a dev-time Rust language compiler (emits validated data and structural
> proofs, **never sees an end-user script**, never ships)"
> (`mock/research/canon/the-soul-of-vehje-positive-catalogue.md:45`)

> "**Rust never runs per-script analyses, the runtime does, for all scripts**"
> (`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md:38`, killing
> the dual-locus framing)

So a Clause program's whole path, from source text to a value, belongs inside the
Zig runtime. And per the lead designer's standing call 2, the host configures the
runtime and lends it allocations, and does not supply arithmetic. This sketch
tests both together: can source reach a value with neither Rust nor a host
anywhere in the path.

## Result

Yes. `zig test eval.zig`, eleven tests, all passing:

```
1 + 2 * 3                                                   ==> 7
let x = 2; let y = 3; if x < y { x * 10 } else { y * 10 }   ==> 20
if 1 < 2 { let k = 4; k * 3 } else { 0 }                    ==> 12
let x = 2; let x = 9; x                                     ==> 9
x + 1                                                       ==> refused, Unbound

fn double(n) { n * 2 } double(3) + 1                        ==> 7
fn add(a, b) { a + b } add(4, 7)                            ==> 11
fn double(n) { n * 2 }
fn quad(n) { double(double(n)) } quad(5)                    ==> 20
fn fact(n) { if n < 2 { 1 } else { n * fact(n - 1) } }
fact(5)                                                     ==> 120
fn adder(a) { fn inner(b) { a + b } inner } adder(3)(4)     ==> 7
fn add(a, b) { a + b } add(4)(7)                            ==> 11
```

Three of those are worth naming. `fact` terminates because `fn` lowers to a
binding in scope for its own value, so the call to itself resolves rather than
escaping. `adder(3)(4)` works because a closure names the environment it was
written in, which is why the environment is a linked chain rather than a stack
that pops. And `add(4)(7)` equalling `add(4, 7)` is not a feature: Core
application takes one argument at a time, so a multi-parameter function is nested
lambdas and partial application falls out of the lowering.

The path is: source, lexer, precedence-climbing parser, Core IR written straight
into a lent wire buffer in the layout the runtime already decodes, then
evaluation. No intermediate AST, so there is no second tree to keep in step with
the IR. Every buffer is caller-lent and nothing allocates.

Arithmetic is a family operation, since the Core has no `+`, and its meaning
arrives as a program over the closed primitive vocabulary proved admissible in
`mock/research/sketches/202607260800_four-way-projection/`. `applyOp` dispatches
`inline for` over the operation table, so each arm is specialised at comptime and
the operation's walk disappears. `1 + 2 * 3` is computed by the runtime, not
handed to anyone.

## What this establishes, stated narrowly

The locus is demonstrated, not argued: a language front end runs where the canon
puts it, and the arithmetic that was previously a host callback in a test file is
now computed inside the runtime by data the language definition supplies.

## What it does not establish, stated plainly

**This is a slice of the grammar, and the bar is the whole grammar.** It has
integers, names, `let`, `fn` with recursion and closures and currying,
`if`/`else`, and four operators. It does not have generics, traits, associated
types, patterns, `match`, macros, strings, records, sequences, loops, modules,
`use`, attributes, or any of the rest of the surface the normative grammar
(`mock/research/original-docs/CLAUSE_EBNF.md`) requires. Nothing is done until
the full intended language is expressible.

It also has no resolve pass and no type checker, so it is not yet statically
analysable in the sense the goal requires. Names bind by a backward scan at
evaluation time, which is the wrong shape for a language whose identity is that
the type system is the verification layer. The parser emits IR directly, which is
right for a subset and will need revisiting when a checker sits between them.

The evaluator here is a subset evaluator in the sketch, not a change to
`mock/runtime-zig/src/runtime.zig`, which still routes every family operation to
a host. That change is the source changelist of round
`202607260500_topic.the-languageauthor-specialisation-stage.md`, and this sketch
exists to prove the shape before that changelist implements it.

Booleans are represented as `0` and `1` in an `i64` rather than as the Core's
`Bool`, because the comparison operation returns what its program computes and
there is no checker yet to demand better. That is a subset artifact and it is not
the intended representation.

## Reproducing

```
zig test eval.zig
```

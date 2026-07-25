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

Yes. `zig test eval.zig`, 40 tests, all passing. The core of it:

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

## Data: strings, records, sequences

```
let r = { a: 7, b: 9 }; r.a + r.b                ==> 16
let r = { inner: { deep: 3 } }; r.inner.deep     ==> 3
at([{ v: 9 }, { v: 8 }], 0).v                    ==> 9

let r = { a: 1 }; r.b                            refused, NoSuchField
let n = 1; n.a                                   refused, Mismatch
if 1 < 2 { { a: 1 } } else { { a: "x" } }        refused, Mismatch
len([1, "two"])                                  refused, Mismatch
if { a: 1 } { 1 } else { 2 }                     refused, UnexpectedToken
at([1, 2], 5)                                    refused at run time, OutOfRange
```

Construction is a family operation and projection is a Core form, which is the
split the census fixes: content-as-values makes construction the signature's
introduction projection, and the Core is the eliminator algebra.

This is where the round's coverage question got answered by building rather than
arguing. The scalar vocabulary that carries arithmetic cannot express a
constructor, because a constructor takes a variable number of operands and yields
a compound. The `TAG_RAW` arm branches on exactly that, and the branch is the
finding made visible. It is one arm of the eventual bench, not a settled choice.

The last refusal is the struct-literal ambiguity, resolved as Rust resolves it:
`{` opens a block after an `if` condition and a record literal elsewhere.

`len` and `at` are recognised by the parser as prelude names in call position.
That is a sketch shortcut: in the finished shape the language definition supplies
them as ordinary operations, and the parser recognises nothing.

## Real Clause code

The first program here that reads as a standard-library function rather than a
demonstration:

```
fn sum_from(s, i) { if i < len(s) { at(s, i) + sum_from(s, i + 1) } else { 0 } }
fn sum(s) { sum_from(s, 0) }
sum([1, 2, 3])                                   ==> 6
```

## What it does not establish, stated plainly

**This is a slice of the grammar, and the bar is the whole grammar.** It has
integers, strings, names, `let`, `fn` with recursion and closures and currying,
`if`/`else`, four operators, records with field access, and sequences with a
two-operation prelude. It does not have bounds, traits, associated types,
coherence, patterns, `match`, macros, loops, modules, `use`, attributes, a
separate resolve pass, or monomorphisation, nor the rest of the surface the
normative grammar (`mock/research/original-docs/CLAUSE_EBNF.md`) requires.
Nothing is done until the full intended language is expressible.

Traits specifically are gated behind monomorphisation rather than syntax: types
erase before the Core, so trait dispatch needs dictionaries or specialisation,
and that is a pass between check and evaluation that does not exist.

## Inference, in the runtime, before evaluation

`check.zig` is Hindley-Milner over the same image, with generalisation at binding
sites. It refuses what the untyped evaluator accepted:

```
if 1 { 1 } else { 2 }                      refused, Mismatch
if 1 < 2 { 1 } else { 2 < 3 }              refused, Mismatch (branches disagree)
1 + (2 < 3)                                refused, Mismatch
fn double(n) { n * 2 } double(1 < 2)       refused, Mismatch
fn f(x) { x(x) } 0                         refused, Occurs
x + 1                                      refused, Unbound

fn id(x) { x }
if id(1 < 2) { id(1) } else { 0 }          ==> int
```

The last one is the load-bearing test. `id` is generalised, so its two uses do
not constrain each other and one definition serves both `Bool` and `Int`. That is
what a generic *is* in inference terms, and it is the seed the bounds and
associated types grow from: a bound is a constraint carried alongside the
quantifier, not a different mechanism.

Two smaller points worth keeping. A recursive binding stays monomorphic inside
its own body, because generalising before the value is inferred would let a
recursive call take a type the definition has not earned. And the arithmetic
family's operation *types* come from the same signature data that carries the
operation bodies, so the checker reads what the operations mean rather than
knowing what arithmetic is.

The first refusal above is the one that matters most: booleans were `0` and `1`
in an `i64` because nothing demanded better, and now something does.

The evaluator here is a subset evaluator in the sketch, not a change to
`mock/runtime-zig/src/runtime.zig`, which still routes every family operation to
a host. That change is the source changelist of round
`202607260500_topic.the-languageauthor-specialisation-stage.md`, and this sketch
exists to prove the shape before that changelist implements it.

At runtime a boolean is still `0` or `1` in an `i64`, because the comparison
operation returns what its program computes. The difference the checker makes is
that this is no longer observable: a program that treats one as the other is
refused before it runs. The representation remains a subset artifact and is not
the intended one.

## Reproducing

```
zig test eval.zig     # 40 tests, the whole pipeline plus the checker's
zig test check.zig    # the inference tests alone
```

## Still owed on the checker itself

Inference runs, but nothing yet forces it to: `eval.zig`'s `run` evaluates
without calling `check`, which is the same prove-then-erase gap the shipped
pipeline has (`serialize` documents itself as taking a checked program while its
signature takes a bare arena). Wiring the gate so an unchecked program cannot be
evaluated is owed, and is the smaller half of the junction question.

There is also no separate resolve pass. Names are looked up during inference and
again during evaluation, by the same backward scan over a linked scope. That
works for a subset with no modules, paths, or imports, and stops working the
moment `use` and multi-segment paths arrive.

Bounds, associated types, and coherence are not started. The generalisation
machinery above is their foundation, not a partial version of them.

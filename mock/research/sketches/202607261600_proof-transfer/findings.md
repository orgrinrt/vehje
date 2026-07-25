# Findings: does the typestate survive erasure into data?

**Outcome: WORKS, with one condition that is now an obligation on the generator.**

## The hypothesis

op, 2026-07-26 (standing call 12): the typestate enforced on the Rust side
survives comptime specialisation implicitly, because any shape drifting from the
enforced-valid ones is unrepresentable and errors. "Enforce once; any invalid
shape would be inrepresentable and cause errors." Held as conviction, not proof.

It carries the whole project. Standing call 18 states the consequence of its
failure without hedging: if the static proof cannot be had, the design's novelty
is gone and the honest move is to delete it and use something existing.

The question was framed per op's later correction (call 20): not "can the runtime
verify this operation is well formed", which is per-instance verification needing
the instance, but whether a malformed operation can be **built** at all.

## What was tested

An operation definition crossing as bytes plus a count, with all Rust-side
typestate erased. The representative invariant: the stack program consumes
exactly `arity` operands and leaves exactly one result, never underflowing.

`specialise` gates on `shapeOf` with `@compileError`, not with a returned error.
That is the load-bearing line: a malformed definition has no specialisation at
all, rather than existing as a callable thing that later refuses.

Nothing in the sketch sees a program, a script, or an IR. Only a definition.

## Result

Seven tests pass (`zig test proof.zig`, Zig 0.16.0). Two confirm well-formed
operations specialise and compute; five pin the rejection predicate.

All four malformed shapes fail to **compile** when their specialisation is
reached:

```
error: operation reads operand 1 but its arity is 1
error: operation underflows at byte 2, stack depth 1
error: operation leaves stack depth 2, must leave exactly 1
error: byte 127 at 4 is not in the vocabulary
```

Compile errors, not test failures, not runtime refusals. The invariant survived
erasure into untyped bytes, and it survived because the specialisation cannot be
generated from malformed data, not because anything inspected an instance.

## The condition, which is a real constraint

**Zig analyses lazily.** A malformed specialisation bound to an unreferenced
top-level declaration is never analysed, and the build succeeds:

```zig
// compiles clean; _x is never referenced, so specialise() is never run
const _x = specialise(Op{ .arity = 2, .prog = &.{ PUSH_ARG, 0, ADD } });
```

Forcing the reference inside a test produces the error. So the guarantee is
precisely: **no malformed operation can be used**, not **no malformed operation
can be present in the package**.

For the shipped design that is sufficient, because the generated runtime
references every operation in the language definition it was specialised against.
But that referencing is now a load-bearing obligation on the generator rather
than something the language gives for free. If a future generator emits an
operation nothing dispatches to, that operation is unchecked. Worth a gate of its
own: the specialiser must reach every operation in the package.

## A methodological note worth keeping

The first run of the negative cases compiled cleanly and printed no errors. Taken
at face value that reads as a refutation of the project's founding assumption. It
was laziness, not refutation. One more command separated "the hypothesis is dead"
from "the hypothesis holds under a stated condition".

## What this unblocks and what it does not

**Unblocks.** S1 was the gate on every other gate. Calls 12 and 18 rest on this
and both now have evidence rather than conviction. G2, G3, and G6 can proceed.

**Does not establish.** That the invariant chosen here generalises to every
invariant the Rust side enforces. This is one representative invariant with
arithmetic shape. Richer ones (effect inclusion, lease non-escape, the graded
axes) have not been tested and are not implied. G3's reclamation obligation in
particular is a different and harder shape: it is a property of a program's
bindings rather than of an operation's stack discipline, and nothing here says it
yields to the same treatment.

**Does not establish.** That the vocabulary covers constructors. That is G6, and
the clause-in-zig sketch already shows it does not today.

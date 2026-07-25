# Monomorphisation: the shapes, and why the limitation stands until one lands

**Date:** 2026-07-26
**Status:** research, from attempting the build rather than from surveying it
**Bears on:** the Clause sketch's standing trait limitation, and the framework's
own erasure story

## The limitation, and exactly what causes it

In `mock/research/sketches/202607260900_clause-in-zig/`, a function whose body
raises a trait obligation stays monomorphic in the type that obligation is
about. `fn twice(v) { dbl(dbl(v)) }` works, but at one type per program.

The cause is precise rather than incidental. Dispatch is a per-node table:
inference resolves each trait-method reference to an impl and writes the choice
into `resolved[node]`, and the evaluator reads it. That keeps types erased, which
is the property the canon's centre of gravity requires. But `twice`'s body is one
set of nodes however many times `twice` is used, so two uses at different types
would need two different values in one slot.

Generalising anyway does not help: the body is inferred once, its obligation's
`Self` variable is generalised into `twice`'s scheme, and discharge then finds it
unresolved. That is `AmbiguousConstraint`, not a wrong answer, so the current
restriction refuses rather than mis-dispatches. The refusal is sound; it is the
generality that is missing.

## Three shapes, and what each costs here

**Clone per use site.** At each reference to a bounded binding, emit a fresh copy
of its value subtree, so each use has its own nodes and its own resolution slots.
Simple, and monomorphisation is duplication anyway.

It does not terminate on recursion. A recursive bounded function references
itself, so cloning at each reference clones forever. Making it terminate means a
worklist keyed by (binding, type) with a memo, which is the real algorithm rather
than the shortcut, and at that point the simplicity that motivated it is gone.

**Worklist monomorphisation.** Infer, collect each use's concrete instantiation,
clone one specialisation per distinct (binding, type) pair, rewrite the uses to
name their specialisation, re-infer. Recursion terminates because the memo
returns the in-progress specialisation.

This is the shape that fits the canon: types erase, nothing is passed at runtime,
and the residual contains only specialised code. It needs a node-cloning pass
over the arena with child remapping, an instantiation record from the checker,
and a second inference pass. It is the honest answer and it is not small.

**Dictionary passing.** Give a bounded function an extra parameter carrying the
impl's method, resolved at the call site.

It avoids duplication and it terminates trivially. But it puts a value at runtime
that exists only because a type existed, which is the thing prove-then-erase says
should not survive. The canon does not forbid it in as many words, and a
dictionary is not a type, but it is the mechanism erasure is meant to make
unnecessary. Choosing it would be a design call about the identity rather than an
implementation convenience, so it is not one to make in passing.

## What this settles and what it does not

Settled: the limitation is not laziness, and it is not a bug to patch. It is
where the missing pass shows. Any lifting of it requires a real specialisation
step between check and evaluation.

Not settled: which shape. The worklist is the obvious fit and the dictionary
shape is the obvious cheat, and the choice touches whether anything type-shaped
may exist at runtime, which is an identity question rather than a speed one.

The framework side has the same question waiting, unresolved in the same way:
`vehje-lower`'s passes are no-ops today, and whatever specialises a bounded
generic there will face this fork with the same three candidates.

## Why this is written down rather than built

The fork is large, the shapes differ in what they imply about erasure, and
picking wrong means work that gets thrown away. That is the case the workspace's
own charting discipline names for designing before coding. The alternative,
starting the worklist pass at the end of a long session and leaving it half-done,
would leave the sketch worse than the honest refusal it has now.

# Sketch: can the reuse uniqueness bit be derived from the lease masks?

**Date:** 2026-07-25
**Hypothesis:** the exact-meet reuse analysis needs a per-site referrer count, and the check pass already
computes a per-node `Lease(ReachMask)` over binder slots. Reach answers "what does this node reach" and the
reuse verdict needs "what reaches this binder", which look like inverses, so the referrer count should fall out
of one pass over a grade table that already exists. Recorded as intended in
`202607251610_baked-in-optimisation-mandate.md`, explicitly not demonstrated, pending this check.

**Outcome: FAILS as stated.** The inverse reading does not hold, for a reason visible in the join rule. A
corrected two-part derivation does hold, and it is cheaper than the original guess in one half and a genuinely
different analysis in the other.

## Why it fails

`infer` computes `reach` as the join of the children's reach (`vehje-typecheck/src/lib.rs:236-244`), with two
contributions and two removals: a `Var` inserts its resolved binder's slot (`:248-253`), and `Let` and `Lambda`
each remove their own binder's slot after joining, because the variable they introduce does not escape its own
binder (`:255-267`).

So `reach(n)` is the set of **free** binder slots reachable from `n`, and it is transitively closed upward
within each binder's scope. Every ancestor of a `Var(b)` that sits inside `b`'s scope carries `b` in its mask,
not just the use site. Counting nodes whose mask contains `b` therefore counts the ancestor chain, which is a
subtree-closure size and has no relationship to how many places actually hold the value. The count is wrong,
and it is wrong upward without bound, so it cannot even be used as a conservative over-approximation with a
useful threshold.

The deeper mistake in the hypothesis is a category error. Reach is about **escape**: which binders a value can
still be reached through. Uniqueness at an update site is about **last use**: whether anyone still holds the
value after this point. A value used twice, where the second use is the update itself, is unique at that
update, and no count of uses distinguishes that from a value used twice with the update first. Last use is a
dataflow property over an ordering; reach is a set closed over the tree. They are not inverses and one does not
yield the other.

## What does hold, in two parts

**Reach is a sound escape filter, which is one necessary half.** If a value's reach mask is not contained in
the binders dying at the current scope, the value escapes, and an escaping value can never be reused in place
regardless of its use count. So the existing masks do real work here: they cheaply rule out the reuse
candidates that are unsafe for a reason unrelated to counting. That is a necessary condition, not a sufficient
one, and it is already computed.

**Last use is the other half and it is a new pass, but a cheap one.** The reason it is cheap is ANF. The
lowering already normalises to A-normal form (`vehje-lower/src/anf.rs`), so every intermediate is bound and the
body is a linear chain of `Let`s. Within such a chain, the last use of a binder is simply its last occurrence
in the linear order, which one backward scan finds. Branching is where it stops being trivial: a binder used in
one arm of an `If` and not the other is live past the join on one path only, so the safe verdict at the update
site is the meet over the paths, which is the ordinary liveness meet.

## The corrected shape

The reuse bit at an update site is the conjunction of two facts: the value does not escape, which the existing
reach mask answers, and this is the last use along every path, which a backward liveness scan over the ANF
chain answers. Neither alone is enough, and the second is not derivable from the first.

## What this changes

The memo's claim needs correcting from "plausibly derivable from machinery already shipped" to "half derivable,
half a new backward pass whose cost ANF has already paid down". That is a weaker claim and a more useful one,
because it names the actual work.

The safety direction still holds and is worth restating, since it is what lets this be built before the slot
collision is fixed. `slot_of` folds the binder index modulo the mask width, so two binders far enough apart
share a slot; under the escape filter a collision merges two binders, which can only make a value look like it
escapes when it does not. That refuses a reuse that would have been safe, costing speed and never soundness.

## What it unblocks, and what it does not

It unblocks the reuse analysis being specified honestly: an escape filter over existing masks, plus a backward
last-use scan over the ANF chain, plus the emitted bit the runtime's update path already reads (the runtime
side shipped in round `202607251700` and defaults to copy until the bit exists).

It does not unblock building it yet, because the liveness meet at branch joins wants its own design decision
about how conservative to be at `Match` and `Handle`, where the paths are not as simple as an `If`'s two arms.

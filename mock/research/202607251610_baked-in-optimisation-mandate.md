# The baked-in optimisation mandate: what is already measured, and what is owed

**Date:** 2026-07-25
**Status:** research. A standing-directive reading plus a bench inventory, not a design round.
**Source:** op, 2026-07-25. The runtime is not to be naive. The optimisations known to help the vast majority of cases are baked in as defaults, opt-out for a consumer that wants to hand-tune, with host configuration as the control surface and sensible ergonomic defaults either way. And, in the same breath: if a named mechanism turns out less optimal than an alternative, the alternative wins, and when in doubt the harness decides.

That second clause is the important half, and it is the workspace's own discipline applied by op to op's own proposal. The intent is canon: bake the wins in, do not ship a naive runtime, make it opt-out rather than opt-in. The mechanisms named alongside it (interning strings, interning equality-heavy compounds, active deduplication, reclaiming as values leave scope) are candidates, and which of them actually earns its place is a measurement, not a decree.

Reading the directive that way turns this memo into an inventory question rather than a design question, and the inventory has a surprise in it: three of the four named mechanisms have already been measured under the harness, and two of the three came back decisively positive. What is owed is mostly wiring, not discovery.

## What the harness already settled

Four existing cells bear on the directive directly. Each is a committed bench with a findings artifact, so these are measurements, not recollections.

| Mechanism | Cell | Result | State |
|---|---|---|---|
| Interning identifiers on the compile side | `interner-intern-hotpath` | ~19 ns per token, string-hash-bound; load factor and hash choice are second-order | measured, not a bottleneck |
| In-place update when a record is provably unique | `record-update-reuse` | 9.7x at zero sharing, 3.3x at 20%, 1.5x at 60%, never loses | measured, decisive |
| Field access strategy | `project-field-access` | 0.54 ns compile-resolved offset, 0.61 ns monomorphic inline cache, 1.27 ns hash, 1.57 ns linear scan | measured, strategy settled |
| Value arena walk | `value-arena-throughput` | pool-indirected walk 10.7 ns/node against 4.4 ns/node inline operands | measured, favours inline |

The reuse result is the load-bearing one and deserves restating precisely, because it is the mechanism behind op's "aliasing as things drop out of scope" and it is the strongest number in the whole bench corpus. Functional record update, lowered to in-place mutation wherever the emitter can prove the record has one referrer, runs at the always-mutable ceiling when records are unique and degenerates to plain copy-on-write when they are not. It has no losing regime. The uniqueness verdict is computed at emit time, so the runtime carries no reference counts and pays no analysis cost: it reads an emitted bit and either writes in place or copies. The companion result bounds peak memory to the live frontier by the same mechanism.

That is exactly the property that makes an immutable-value scripting language viable at scale, and it is the reason the no-collector axiom does not cost what it looks like it costs. The consumer writes pure functional updates; the emitter turns them into in-place writes wherever it can prove safety.

## The derivation that makes the uniqueness bit cheap

The reuse analysis needs, per value-producing site, how many places reach that value. The check pass already computes something adjacent: a per-node `Lease`, which is a `ReachMask` over in-scope binder slots recording which binders a value can reach, joined up from the children.

Those two are inverses of each other. Reach answers "what does this node reach"; the reuse verdict needs "what reaches this binder". Given the per-node masks the check pass already writes into the grade table, the referrer count of a binder is the number of nodes whose mask contains that binder's slot, which is one pass over a table that already exists. So the uniqueness bit is plausibly derivable from machinery already shipped, rather than needing a second analysis. This is **intended**, not demonstrated: it wants a sketch that confirms the direction and the pass shape before anything is built on it.

One property of the derivation is worth naming now because it decides whether it is safe to rely on. Slot assignment currently folds the binder index modulo the mask width, so two binders far enough apart share a slot; that collision is already catalogued as a red test. Under the derivation, a collision merges two binders into one slot, which inflates the referrer count, which makes a unique value look shared, which makes the emitter copy where it could have reused. That is a loss of speed and never a loss of soundness. The conservative direction is the safe one, which means the derivation can be built before the collision is fixed, and the fix is a later speed improvement rather than a correctness prerequisite.

## What is genuinely unmeasured

Three things the directive touches have no cell, and they are the honest bench agenda.

**Runtime string values, as distinct from compile-side identifiers.** The measured interner cost covers identifiers and literals during lexing, where the population is small, heavily repeated, and known before anything runs. A running program's strings are a different population: built by concatenation and interpolation, frequently unique, frequently short-lived. Interning those pays a hash of the bytes on every construction to buy an identity comparison later, which is a good trade when strings are compared often and constructed rarely, and a bad one when the reverse holds. Since a templating consumer builds far more strings than it compares, the sign of this trade is not obvious and should not be assumed from the compile-side number. Today a string literal is already a zero-copy slice into the residual, which costs nothing to produce and cannot be improved on for the literal case; the question is only about constructed strings.

**Deduplication of compound values, meaning hash-consing the value arena.** This is the compound analogue of interning and the same trade appears one level up: a hash of the contents on every construction against an identity comparison later, plus the structural sharing that comes free with it. There is a nearby negative result worth respecting: on the compile side, interned operands were measured unable to win on a value DAG. That is a different arena with a different access pattern, so it does not settle the runtime question, but it is evidence that the intuition "interning always helps" is not reliable and the measurement is required.

**The interaction between deduplication and reuse, which may be a genuine conflict.** Hash-consing makes structurally equal values share one representation, which by construction raises the sharing rate. Reuse pays 9.7x when sharing is zero and 1.5x when sharing is 60%. So deduplication buys memory and cheap equality by spending exactly the property that makes reuse fast. These two mechanisms are not independent and cannot be benched separately and then both switched on; the cell has to measure them together, across the sharing axis, or it will recommend two things that undercut each other.

That interaction is the most interesting unmeasured thing here and it is the one a naive implementation would get wrong, which is a decent argument that op's instinct to bake optimisations in deliberately is right and that the deliberation has to include how they compose.

## What the control surface looks like

Two precedents in the design fix the shape, so this needs no invention. Host configuration is how the framework exposes a knob whose right value is workload-dependent: the allocation-space count is host-provided and adapts to whatever the host offers, and the recursion-depth backstop is host-configurable with a stated default. Both follow the same rule, that a knob is exposed once a measurement has confirmed there is something to tune, and never before.

Applying that here: each optimisation is on by default, the default is whatever its cell measured as best across the representative workloads, and the host may turn it off. Off has to mean something honest, which is the plain semantics without the optimisation, never a different answer. An optimisation whose absence changes the result is not an optimisation.

The one place this needs care is that a bit like the reuse verdict is emitted into the residual at compile time, so the switch has to be read at compile time too, not at runtime. A residual emitted with reuse bits and executed by a runtime told to ignore them is coherent, since ignoring the bit means copying, which is always safe. The reverse, a residual emitted without the bits and a runtime asked to reuse, is not, since the runtime has no verdict to read. So the safe direction of the switch is asymmetric, which is worth stating in the design round rather than discovering later.

## The owed work, in order

The wiring comes first because it is where measured value is sitting unclaimed. The reuse analysis is measured decisive and not built; the derivation above suggests it is cheaper to build than it looked. The field-access strategy is measured and settled and wants to be what `Project` actually lowers to when evaluation lands. The inline-operand record shape is measured and should be what the value arena adopts rather than a pool indirection.

The new cells come second, and there are three: constructed runtime strings across a construct-to-compare ratio sweep; compound deduplication against the same sweep; and the two together across the sharing axis, which is the one that decides whether they can both be defaults.

The sequencing consequence for the language work is direct. The compound operand and return escape is the widest unblock in the framework, and it is the same decision as how a produced value is represented, interned, deduplicated, and reclaimed. Building the value arena first and revisiting its representation after these cells run would be building it twice. So the cells run first, and the value arena lands once, shaped by what they say.

## Resolution of the composition question (2026-07-25, same day)

The third cell named above was built and run, so this section supersedes the "unmeasured" status of the
interaction. It is `mock/benches/dedup-vs-reuse/` with its generator, harness results across five sizes and two
sharing regimes, and a findings artifact; the numbers are in `results/dedup_reuse_unique/` and
`results/dedup_reuse_shared/`.

The interaction is not a tradeoff to tune. It is an exclusion. Once a value is handed to a deduplication table
the table holds a reference to it, so no local uniqueness verdict can ever again license an in-place write on
that value. Interning does not merely raise the sharing rate that reuse is sensitive to; it permanently
forfeits reuse for every value it touches while still charging the hash on construction. Measured: reuse alone
is fastest at every size in both regimes (2.7x to 2.9x over copy-on-write at 20% sharing, 1.4x to 1.6x at 60%),
deduplicating records is 9x to 19x slower than reuse and 6x to 7x slower than doing nothing, and the two
combined land at 4.0x to 5.9x, which is worse than plain. Combining them is worse than either alone and worse
than neither.

So of the mechanisms the directive named, reuse is confirmed as a default by two independent cells, and
deduplicating compound values is refused for records at this shape. The refusal has a stated boundary: narrow
records, comparison-heavy workloads, and the memory axis are each unmeasured and are where deduplication could
still earn a place. Strings remain a separate population with their own cell owed, and must not inherit this
verdict, since a constructed string is cheaper to hash and compared more often than a sixteen-field record.

This is the directive working as intended. The instinct to bake interning in was reasonable, the measurement
refused it for the largest case, and the refusal came with the structural reason rather than only a number.

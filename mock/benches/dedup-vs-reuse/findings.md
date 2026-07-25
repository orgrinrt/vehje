# Dedup vs reuse: does hash-consing produced values compose with the reuse analysis?

**Date:** 2026-07-25
**Type:** harness cell. Four Rust cdylib variants per regime, per-variant subprocess isolation, cross-validated
byte-identical output, 6 passes x 100 runs, 5 sizes. Generator `gen_dedup_vs_reuse.py`. Artifacts:
`results/dedup_reuse_unique/`, `results/dedup_reuse_shared/`, `.bench_history/dedup_reuse_*.tsv`.
**Settles:** whether automatic deduplication of compound values can be a baked-in default alongside the
exact-meet reuse analysis, which is the composition question the optimisation mandate turns on.

## Why this cell

The standing directive is that the runtime is not naive: optimisations known to help the majority of cases are
baked in as defaults, opt-out rather than opt-in. Two of the named candidates are automatic interning of
equality-heavy compound values and active deduplication, and one is reclaiming as values leave scope, which is
the reuse analysis `record-update-reuse` already measured at 9.7x with no losing regime.

Those two are not independent, which is the whole reason for this cell. Hash-consing makes structurally equal
values share one representation, so it raises the sharing rate by construction, and reuse pays 9.7x at zero
sharing but only 1.5x at 60%. Deduplication therefore buys cheap equality and memory by spending exactly the
property reuse runs on. Measuring them separately and switching both on would have shipped two mechanisms that
fight each other.

## Strategies

All four compute the same answer, which the harness enforces: it cross-validated 4 variants across 100 seeds
per size and confirmed byte-identical output before timing anything. That check is the operational form of the
rule that an optimisation which changes the result is not an optimisation.

- **plain**: copy-on-write update, structural equality. The naive runtime.
- **reuse**: in-place when the emitter's verdict says unique, structural equality.
- **dedup**: hash-cons every constructed value, equality is an index compare, updates always copy.
- **both**: the emitter picks one per site. A may-be-shared value is interned and never mutates in place; a
  proven-unique value stays out of the table and mutates freely.

Workload: 16-field records, 8 field updates each, 2 equality comparisons per record, over a duplicate-rich
population (64 distinct base records) so deduplication has something to find.

## Results (ns, lower is better; multiplier is against the fastest)

20% may-be-shared, the templating and config norm:

| n | reuse | plain | both | dedup |
|---|---|---|---|---|
| 1024 | **12.64 us** (1.00x) | 34.29 us (2.71x) | 49.98 us (3.96x) | 237.21 us (18.77x) |
| 4096 | **51.88 us** (1.00x) | 140.19 us (2.70x) | 211.78 us (4.08x) | 973.38 us (18.76x) |
| 16384 | **213.17 us** (1.00x) | 622.28 us (2.92x) | 850.41 us (3.99x) | 3.92 ms (18.41x) |

60% may-be-shared, reuse's adversarial regime:

| n | reuse | plain | both | dedup |
|---|---|---|---|---|
| 1024 | **24.78 us** (1.00x) | 34.63 us (1.40x) | 142.74 us (5.76x) | 234.98 us (9.48x) |
| 4096 | **101.02 us** (1.00x) | 139.13 us (1.38x) | 594.36 us (5.88x) | 977.74 us (9.68x) |
| 16384 | **438.57 us** (1.00x) | 685.55 us (1.56x) | 2.48 ms (5.65x) | 3.97 ms (9.04x) |

## The finding: reuse is a default, record deduplication is refused, and the two do not compose

Three results, all consistent across every size and both regimes.

**Reuse alone wins everywhere.** It is 2.7x to 2.9x faster than copy-on-write in the mostly-unique regime and
still 1.4x to 1.6x faster in the heavily-shared one, with no size at which it loses. That independently
corroborates `record-update-reuse`, which measured 3.3x at 20% sharing and 1.5x at 60% through a different
implementation and a different timing path; two cells agreeing this closely on a mechanism is the strongest
evidence in the corpus for baking it in.

**Deduplicating records is refused, decisively.** It runs 9x to 19x slower than reuse and 6x to 7x slower than
doing nothing at all. Hashing sixteen fields on every construction dominates the entire workload, and the index
comparison it buys back is not remotely worth it at two comparisons per eight updates. This is the mandate's
"intern equality-heavy compounds" candidate, and for records at this shape the measurement refuses it.

**The composition is worse than doing nothing.** This is the load-bearing result and the one that could not
have been predicted from the two mechanisms measured separately. Combining them does not split the difference
between 1.00x and 18x; it lands at 4.0x in the unique regime and 5.9x in the shared one, both worse than plain.
The reason is structural: once a value is handed to the table, the table holds a reference to it, so no local
uniqueness verdict can license an in-place write on it ever again. Interning therefore does not merely raise
the measured sharing rate, it permanently forfeits reuse for every value it touches, while still charging the
hash. The two mechanisms are not additive and not even independent; they are mutually exclusive per value.

That constraint surfaced as a bug before it surfaced as a design rule. A first standalone draft of this cell
mutated in place a record still held by the table, which corrupts the table (the probe finds a record whose
contents no longer hash to that slot, so the chain terminates on neither a match nor an empty slot) and
produced timings in the tens of microseconds per record. Stating it properly: **the dedup table is itself a
referrer.** The draft is kept at `dedup.zig` as the audit trail, with its numbers marked as not results.

## Design impact

Build the reuse analysis and make it the default; it is now corroborated by two independent cells and has no
losing regime in either. Do not deduplicate records by default. Do not attempt to run both.

If deduplication is wanted for its memory or its equality benefit rather than its throughput, it is an opt-in
per-site decision that costs the reuse win on the values it covers, and the design should present it that way
rather than as a free improvement.

## Boundary

Three things this cell does not settle, each of which could move deduplication's sign.

Record width is 16 fields, so the hash walks 128 bytes per construction. A narrow record of two to four fields
hashes proportionally cheaper while the index comparison stays the same price, so the crossover as width
shrinks is unmeasured and is the most likely place deduplication earns a place.

The comparison-to-construction ratio is 2 comparisons per 8 updates, which favours reuse by construction, since
comparisons are what deduplication sells. A comparison-heavy workload is deduplication's best case and is not
measured here.

Memory is not measured at all; this cell times only. Deduplication's memory argument stands untested, and in a
regime where peak memory rather than throughput is the binding constraint the conclusion could differ. Note
that reuse already bounds peak memory to the live frontier, so the two are competing on that axis too.

Strings are a separate population with a separate answer and are not covered here. A constructed runtime string
is cheaper to hash than a 16-field record and is compared more often, so the string question needs its own cell
and must not inherit this one's verdict.

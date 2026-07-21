# Closure representation: create-once-call-many (flat vs linked)

Sibling bench: `closure_create_many` (the inverting usage pattern).

## What this measures

The `Lambda` + `Apply` closure representation fork under create-once-call-many: a
closure is built once (captures fixed) and invoked many times, so captured-variable
ACCESS cost dominates and creation amortises to nothing. Two variants race, differing
only in representation:

- `closure_call_many_flat`: captures copied into flat storage once; each call reads a
  slot by input-derived index (O(1) access).
- `closure_call_many_linked`: the closure points into a 4-frame environment chain;
  each call walks the chain to the root frame and reads the slot (O(depth) access).

Both hold the same capture values, so the accumulator is representation-independent
and the harness cross-validates flat and linked byte-for-byte. The FFI input drives
the read index and folds into the accumulator, so the access cannot hoist. State (env
+ chain) is built once via `OnceLock` and outside the timed loop; only the repeated
access is timed. Work scales as `N * REP` (REP = 64), a volume axis (the working set
is a handful of frames at every size, not a cache-regime axis).

## Measured results

Ratio to baseline (closure_call_many_flat), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | closure_call_many_flat (base) | closure_call_many_linked (ratio) |
|---|---|---|
| 64 | 3186 ns | 2.98x |
| 256 | 10666 ns | 3.01x |
| 1024 | 44125 ns | 2.90x |
| 4096 | 174056 ns | 2.93x |
| 16384 | 671133 ns | 3.03x |

## Cost-model sanity line

At n=16384, the baseline (closure_call_many_flat) median is 671133 ns for flat env: N calls per invocation, one indexed load per captured var. Treating n as the work-item count, that is 40.96 ns/item, about 131.1 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass (byte-identical accumulator across variants).

## Verdict

A flat captured environment calls 2.9x to 3.0x faster than a linked (parent-pointer chased) environment, stable across all sizes. For call-heavy closures, flatten the environment at creation. The inverse cost is measured in closure_create_many.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).

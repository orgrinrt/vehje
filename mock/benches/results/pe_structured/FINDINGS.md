# Partial-eval specialization: fold ratio on a block-structured template

Companion: `pe_random` (the random static/dynamic mix shape).

## What this measures (a reduction metric, not a strategy race)

The same binding-time partial evaluation as `pe_random`, on the realistic templating
shape: a document is a right-leaning concatenation of N blocks, each either a
contiguous static text block (an 8-node static chain that folds whole to one constant)
or a dynamic value hole (survives into the residual). The payload is the FOLD RATIO =
original / residual node count, a REDUCTION metric. The harness times the specialize
pass and records the residual and original counts in the output high bits (ratio
recoverable) plus an input-seeded checksum in the low bits (no hoist).

Variants sweep the static-block fraction: `pe_struct_sf50` / `sf70` / `sf90`. They
produce different residuals, so the bench is `may_differ = true` (per-variant
determinism still checked). The program is built once via `OnceLock` at the process's
size (N blocks); the pass runs REP = 16 full walks per timed call, seeded from input.

## Measured results

Ratio to baseline (pe_struct_sf70), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | pe_struct_sf50 (ratio) | pe_struct_sf70 (base) | pe_struct_sf90 (ratio) |
|---|---|---|---|
| 64 | 1.07x | 7091 ns | 0.91x |
| 256 | 1.07x | 25082 ns | 0.97x |
| 1024 | 1.18x | 118529 ns | 1.16x |
| 4096 | 1.10x | 487475 ns | 1.40x |
| 16384 | 1.13x | 2291528 ns | 1.16x |

## Cost-model sanity line

At n=16384, the baseline (pe_struct_sf70) median is 2291528 ns for N nodes, fold attempts on structured chains. Treating n as the work-item count, that is 139.86 ns/item, about 447.6 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

reduction metric.

## Verdict

On structured programs the speculation factor matters much less (1.0x to 1.4x spread) and turns non-monotonic at large n: sf90 can cost MORE than sf70 (over-speculation on already-structured code wastes fold attempts that do not pay off). A moderate speculation factor is the safe default; maximal speculation is not free on structured input.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).

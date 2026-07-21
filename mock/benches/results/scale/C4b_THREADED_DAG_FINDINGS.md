# C4b: threaded level-sync DAG compile, real parallel speedup

**Strength: measurement** (CNTVCT_EL0, 5 runs median; cross-validated: the XOR-folded compile result is
identical across all thread counts).

## The audit defect this fixes

The old parallel-compile claim rested on a frictionless model: `ceil(width / 8)` levels of perfectly
parallel work with no coordination cost, extrapolated to a 5.64x-7.7x speedup. This measures the real
mechanism: a synthetic module DAG (64 levels x 512 modules), a persistent pool of T OS threads reused across
levels, per-level barriers (the level-sync), and a real `compile_module` (8 interpreter passes) per module,
modules pulled within a level by an atomic cursor (work-stealing). The compile work and the fold are the
carrier's, shared with the C4a incremental bench.

## Result (32768 modules, 64 levels x 512, Apple M1, 4 performance + 4 efficiency cores)

| threads | time | speedup | efficiency |
|---|---|---|---|
| 1 | 17.5 ms | 1.00x | 100% |
| 2 | 9.8 ms  | 1.79x | 89% |
| 4 | 7.6 ms  | 2.30x | 57% |
| 8 | 8.2 ms  | 2.14x | 27% |

Real threaded compile peaks at ~2.3x (4 threads) and DEGRADES at 8 threads. This is far below the
frictionless model's 5.6x-7.7x.

## The finding

The frictionless `ceil(width/8)` model overstated parallel compile speedup by roughly 2.5x. Three real costs
the model ignored cap it:

1. **Heterogeneous cores.** The M1 has 4 performance and 4 efficiency cores. Two threads land on two P-cores
   (89% efficiency); four saturate the P-cores (57%); eight spill onto the much slower E-cores, so the 5th-8th
   threads add little and coordination cost makes 8 threads slower than 4. A homogeneous 8-core machine would
   scale further, but the number that matters for a claim about THIS machine is ~2.3x, not 5.6x.
2. **The level-sync barrier.** 64 per-level barriers serialize the pool 64 times; a thin or skewed level
   leaves threads idle at the barrier. The frictionless model assumed no barrier cost.
3. **Shared-memory contention.** All threads stream module bytes and interpreter state through the same L2 /
   memory system; at 4+ threads the compile is partly bandwidth-bound, not compute-bound.

The honest headline: content-addressed caching (C4a, warm reload) is the larger and more reliable win for
incremental builds than thread-parallel cold compile, which on this machine tops out near 2.3x. The two
compose (cache first, parallelize the misses), but the parallel multiple is ~2.3x here, not the model's 5.6x.

## Cost-model / boundary

Serial baseline: 17.5 ms for 32768 8-pass compiles = 534 ns/module, consistent with the C4a per-module
compile cost. The result is machine-specific (M1 big.LITTLE); the method (persistent pool, per-level barrier,
real compile, atomic work-stealing, XOR cross-validation) transfers, and re-running on the target deployment
hardware is the right move before quoting a parallel multiple. A persistent pool was used deliberately: an
earlier per-level thread::scope respawn measured thread-spawn churn (~60% of the time at 4 threads) rather
than parallelism, and was replaced.

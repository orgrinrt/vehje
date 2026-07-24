# abi_marshal (madd)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_madd_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_madd_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_madd_marshal_null dominates: 26436% faster than the next best (abi_marshal_madd_aos)

abi_marshal_madd_marshal_null (10.10 us) leads abi_marshal_madd_aos (2.68 ms) by 26436%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_madd_marshal_null beats baseline by 100% (significant)

abi_marshal_madd_marshal_null is -2.67 ms (100%) faster than baseline abi_marshal_madd_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_madd_soa_transposed is an outlier: 266.4x slower than the field

abi_marshal_madd_soa_transposed (2.69 ms) is 266.4x the fastest (10.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_madd_marshal_null} vs {abi_marshal_madd_aos, abi_marshal_madd_soa_native, abi_marshal_madd_soa_transposed} (26436% apart)

The field splits into a fast tier {abi_marshal_madd_marshal_null} and a slow tier {abi_marshal_madd_aos, abi_marshal_madd_soa_native, abi_marshal_madd_soa_transposed} with a 26436% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 266.4x the fastest

Fastest abi_marshal_madd_marshal_null (10.10 us) to slowest abi_marshal_madd_soa_transposed (2.69 ms): 266.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_madd_marshal_null** at 10100.9 ns median (-99.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 266.41x (fastest 10100.9 ns, slowest 2690930.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2680432ns | 2682888ns | 2664053ns | 2680097ns | 2689123ns | base |
| abi_marshal_madd_marshal_null | 12329ns | 12346ns | 11948ns | 12299ns | 12565ns | -99.54% |
| abi_marshal_madd_soa_native | 2703454ns | 2687019ns | 2679545ns | 2685903ns | 2741734ns | +0.86% |
| abi_marshal_madd_soa_transposed | 2692801ns | 2693704ns | 2677991ns | 2692056ns | 2701325ns | +0.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2677770ns | 2661240ns | 2686404ns | base | 0.000 |
| abi_marshal_madd_marshal_null | 10092ns | 9805ns | 10273ns | -99.62% | 0.000 |
| abi_marshal_madd_soa_native | 2700862ns | 2676842ns | 2739120ns | +0.86% | 0.000 |
| abi_marshal_madd_soa_transposed | 2690109ns | 2675395ns | 2698645ns | +0.46% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_madd_aos | 43324.2 | 2680149.9 | 2677770.0 | n/a |
| abi_marshal_madd_marshal_null | 27302.4 | 10220.3 | 10091.5 | n/a |
| abi_marshal_madd_soa_native | 43361.7 | 2702704.3 | 2700861.9 | n/a |
| abi_marshal_madd_soa_transposed | 44201.4 | 2691712.8 | 2690108.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_madd_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_madd_aos | 0.000 | 0.4% |
| abi_marshal_madd_marshal_null | 0.000 | 97.1% |
| abi_marshal_madd_soa_native | 0.000 | 0.4% |
| abi_marshal_madd_soa_transposed | 0.000 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_madd_aos | 2680432ns | 2680432ns | base |
| abi_marshal_madd_marshal_null | 12329ns | 12329ns | -99.54% |
| abi_marshal_madd_soa_native | 2703454ns | 2703454ns | +0.86% |
| abi_marshal_madd_soa_transposed | 2692801ns | 2692801ns | +0.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2680341ns | base | --- | [2666565, 2686404] | --- | --- | --- | --- |
| abi_marshal_madd_marshal_null | 10101ns | -2670166.2ns (-99.6%) | [-2676205, -2656664]ns | [9901, 10273] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_madd_soa_native | 2684478ns | no significant difference | [-980, +57096]ns | [2678988, 2739120] | no | 0.2188 | 0.2188 | 0 |
| abi_marshal_madd_soa_transposed | 2690930ns | +12240.6ns (+0.5%) | [+7065, +17710]ns | [2680751, 2698645] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_madd_aos | abi_marshal_madd_marshal_null | abi_marshal_madd_soa_native | abi_marshal_madd_soa_transposed |
|---|---|---|---|---|
| 1 | 2686479ns | -99.6% | +2.3% | +0.5% |
| 2 | 2683113ns | -99.6% | -0.1% | +0.2% |
| 3 | 2661240ns | -99.6% | +0.8% | +0.5% |
| 4 | 2677570ns | -99.6% | +1.9% | +0.3% |
| 5 | 2686329ns | -99.6% | +0.0% | +0.5% |
| 6 | 2671890ns | -99.6% | +0.2% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_madd_aos | -0.186 | ok |
| abi_marshal_madd_marshal_null | -0.155 | ok |
| abi_marshal_madd_soa_native | -0.254 | moderate- |
| abi_marshal_madd_soa_transposed | 0.152 | ok |

**Consistency summary:**

- **abi_marshal_madd_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_madd_soa_native**: won 0/6, lost 4/6
- **abi_marshal_madd_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_madd_aos | 8081843.8ns | 2677770.0ns | 301.8% | HIGH |
| abi_marshal_madd_marshal_null | 138880.3ns | 10091.5ns | 1376.2% | HIGH |
| abi_marshal_madd_soa_native | 8148411.9ns | 2700861.9ns | 301.7% | HIGH |
| abi_marshal_madd_soa_transposed | 8121377.5ns | 2690108.5ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_madd_aos (n=6, range 2661239.6-2686404.0 ns)
  2661239.6 |########################################
  2662497.8 |
  2663756.0 |
  2665014.3 |
  2666272.5 |
  2667530.7 |
  2668788.9 |
  2670047.1 |
  2671305.3 |########################################
  2672563.6 |
  2673821.8 |
  2675080.0 |
  2676338.2 |########################################
  2677596.4 |
  2678854.6 |
  2680112.9 |
  2681371.1 |
  2682629.3 |########################################
  2683887.5 |
  2685145.7 |########################################
  (0 below, 1 above range)

abi_marshal_madd_marshal_null (n=6, range 9804.6-10273.1 ns)
   9804.6 |####################
   9828.0 |
   9851.5 |
   9874.9 |
   9898.3 |
   9921.7 |
   9945.1 |
   9968.6 |
   9992.0 |########################################
  10015.4 |
  10038.8 |
  10062.3 |
  10085.7 |
  10109.1 |
  10132.5 |
  10156.0 |
  10179.4 |####################
  10202.8 |####################
  10226.2 |
  10249.7 |
  (0 below, 1 above range)

abi_marshal_madd_soa_native (n=6, range 2676842.1-2739120.4 ns)
  2676842.1 |####################
  2679956.0 |########################################
  2683069.9 |
  2686183.8 |####################
  2689297.8 |
  2692411.7 |
  2695525.6 |
  2698639.5 |
  2701753.4 |
  2704867.3 |
  2707981.2 |
  2711095.2 |
  2714209.1 |
  2717323.0 |
  2720436.9 |
  2723550.8 |
  2726664.7 |####################
  2729778.7 |
  2732892.6 |
  2736006.5 |
  (0 below, 1 above range)

abi_marshal_madd_soa_transposed (n=6, range 2675394.6-2698644.6 ns)
  2675394.6 |########################################
  2676557.1 |
  2677719.6 |
  2678882.1 |
  2680044.6 |
  2681207.1 |
  2682369.6 |
  2683532.1 |
  2684694.6 |
  2685857.1 |########################################
  2687019.6 |
  2688182.1 |########################################
  2689344.6 |
  2690507.1 |
  2691669.6 |
  2692832.1 |########################################
  2693994.6 |
  2695157.1 |
  2696319.6 |
  2697482.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_madd_aos**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_madd_marshal_null**: bridge=1374.4% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_native**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_transposed**: bridge=302.0% of algo (FFI overhead may distort results)

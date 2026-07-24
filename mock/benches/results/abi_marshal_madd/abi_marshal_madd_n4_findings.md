# abi_marshal (madd)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_madd_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_madd_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_madd_marshal_null dominates: 12853% faster than the next best (abi_marshal_madd_soa_native)

abi_marshal_madd_marshal_null (20.72 us) leads abi_marshal_madd_soa_native (2.68 ms) by 12853%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_madd_marshal_null beats baseline by 99% (significant)

abi_marshal_madd_marshal_null is -2.67 ms (99%) faster than baseline abi_marshal_madd_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_madd_soa_transposed is an outlier: 130.6x slower than the field

abi_marshal_madd_soa_transposed (2.71 ms) is 130.6x the fastest (20.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_madd_marshal_null} vs {abi_marshal_madd_soa_native, abi_marshal_madd_aos, abi_marshal_madd_soa_transposed} (12853% apart)

The field splits into a fast tier {abi_marshal_madd_marshal_null} and a slow tier {abi_marshal_madd_soa_native, abi_marshal_madd_aos, abi_marshal_madd_soa_transposed} with a 12853% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 130.6x the fastest

Fastest abi_marshal_madd_marshal_null (20.72 us) to slowest abi_marshal_madd_soa_transposed (2.71 ms): 130.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_madd_marshal_null** at 20722.2 ns median (-99.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 130.62x (fastest 20722.2 ns, slowest 2706649.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2695170ns | 2697664ns | 2687766ns | 2695026ns | 2699087ns | base |
| abi_marshal_madd_marshal_null | 23079ns | 23026ns | 22581ns | 22912ns | 23578ns | -99.14% |
| abi_marshal_madd_soa_native | 2685843ns | 2686684ns | 2678183ns | 2685721ns | 2689855ns | -0.35% |
| abi_marshal_madd_soa_transposed | 2707368ns | 2709291ns | 2697028ns | 2707897ns | 2711744ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2692570ns | 2685176ns | 2696437ns | base | 0.000 |
| abi_marshal_madd_marshal_null | 20782ns | 20333ns | 21230ns | -99.23% | 0.000 |
| abi_marshal_madd_soa_native | 2683279ns | 2675624ns | 2687263ns | -0.35% | 0.000 |
| abi_marshal_madd_soa_transposed | 2704782ns | 2694525ns | 2709133ns | +0.45% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_madd_aos | 43489.2 | 2693791.0 | 2692569.9 | n/a |
| abi_marshal_madd_marshal_null | 28186.8 | 20738.6 | 20781.5 | n/a |
| abi_marshal_madd_soa_native | 41501.4 | 2684203.6 | 2683279.2 | n/a |
| abi_marshal_madd_soa_transposed | 42360.1 | 2703307.9 | 2704782.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_madd_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_madd_aos | 0.000 | 0.8% |
| abi_marshal_madd_marshal_null | 0.000 | 98.1% |
| abi_marshal_madd_soa_native | 0.000 | 0.8% |
| abi_marshal_madd_soa_transposed | 0.000 | 0.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_madd_aos | 2695170ns | 2695170ns | base |
| abi_marshal_madd_marshal_null | 23079ns | 23079ns | -99.14% |
| abi_marshal_madd_soa_native | 2685843ns | 2685843ns | -0.35% |
| abi_marshal_madd_soa_transposed | 2707368ns | 2707368ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2695105ns | base | --- | [2686168, 2696437] | --- | --- | --- | --- |
| abi_marshal_madd_marshal_null | 20722ns | -2674393.8ns (-99.2%) | [-2675563, -2665409]ns | [20392, 21230] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_madd_soa_native | 2684168ns | -8721.1ns (-0.3%) | [-17879, -1272]ns | [2678406, 2687263] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_marshal_madd_soa_transposed | 2706649ns | +12819.4ns (+0.5%) | [+2279, +21539]ns | [2698564, 2709133] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_madd_aos | abi_marshal_madd_marshal_null | abi_marshal_madd_soa_native | abi_marshal_madd_soa_transposed |
|---|---|---|---|---|
| 1 | 2685176ns | -99.2% | +0.0% | +0.8% |
| 2 | 2695673ns | -99.2% | -0.2% | +0.5% |
| 3 | 2697200ns | -99.2% | -0.6% | +0.2% |
| 4 | 2687160ns | -99.2% | -0.1% | +0.8% |
| 5 | 2694840ns | -99.2% | -0.4% | +0.4% |
| 6 | 2695370ns | -99.2% | -0.7% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_madd_aos | -0.309 | moderate- |
| abi_marshal_madd_marshal_null | 0.458 | moderate+ |
| abi_marshal_madd_soa_native | -0.063 | ok |
| abi_marshal_madd_soa_transposed | -0.102 | ok |

**Consistency summary:**

- **abi_marshal_madd_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_madd_soa_native**: won 4/6, lost 0/6
- **abi_marshal_madd_soa_transposed**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_madd_aos | 8123317.1ns | 2692569.9ns | 301.7% | HIGH |
| abi_marshal_madd_marshal_null | 171028.3ns | 20781.5ns | 823.0% | HIGH |
| abi_marshal_madd_soa_native | 8095222.5ns | 2683279.2ns | 301.7% | HIGH |
| abi_marshal_madd_soa_transposed | 8154857.3ns | 2704782.3ns | 301.5% | HIGH |

## Distribution (algo ns)

```
abi_marshal_madd_aos (n=6, range 2685176.2-2696436.8 ns)
  2685176.2 |####################
  2685739.2 |
  2686302.3 |
  2686865.3 |####################
  2687428.3 |
  2687991.4 |
  2688554.4 |
  2689117.4 |
  2689680.5 |
  2690243.5 |
  2690806.5 |
  2691369.6 |
  2691932.6 |
  2692495.6 |
  2693058.7 |
  2693621.7 |
  2694184.7 |
  2694747.8 |####################
  2695310.8 |########################################
  2695873.8 |
  (0 below, 1 above range)

abi_marshal_madd_marshal_null (n=6, range 20332.9-21230.4 ns)
  20332.9 |########################################
  20377.8 |
  20422.7 |########################################
  20467.5 |
  20512.4 |
  20557.3 |
  20602.2 |
  20647.0 |########################################
  20691.9 |
  20736.8 |
  20781.7 |########################################
  20826.5 |
  20871.4 |
  20916.3 |
  20961.2 |
  21006.0 |
  21050.9 |
  21095.8 |
  21140.7 |
  21185.5 |########################################
  (0 below, 1 above range)

abi_marshal_madd_soa_native (n=6, range 2675623.8-2687263.3 ns)
  2675623.8 |########################################
  2676205.8 |
  2676787.8 |
  2677369.7 |
  2677951.7 |
  2678533.7 |
  2679115.7 |
  2679697.6 |
  2680279.6 |
  2680861.6 |########################################
  2681443.6 |
  2682025.6 |
  2682607.5 |
  2683189.5 |
  2683771.5 |########################################
  2684353.5 |########################################
  2684935.4 |########################################
  2685517.4 |
  2686099.4 |
  2686681.4 |
  (0 below, 1 above range)

abi_marshal_madd_soa_transposed (n=6, range 2694525.0-2709133.3 ns)
  2694525.0 |########################################
  2695255.4 |
  2695985.8 |
  2696716.2 |
  2697446.7 |
  2698177.1 |
  2698907.5 |
  2699637.9 |
  2700368.3 |
  2701098.7 |
  2701829.1 |
  2702559.6 |########################################
  2703290.0 |
  2704020.4 |
  2704750.8 |
  2705481.2 |########################################
  2706211.6 |
  2706942.1 |########################################
  2707672.5 |########################################
  2708402.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_madd_aos**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_marshal_madd_marshal_null**: bridge=824.0% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_native**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_transposed**: bridge=301.4% of algo (FFI overhead may distort results)

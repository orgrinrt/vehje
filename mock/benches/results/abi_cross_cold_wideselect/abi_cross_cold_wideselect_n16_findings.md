# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_scalar is an outlier: 825.0x slower than the field

abi_cross_cold_wideselect_warm_scalar (2.10 ms) is 825.0x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (2.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} (77484% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} with a 77484% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 825.0x the fastest

Fastest abi_cross_cold_wideselect_warm_null (2.54 us) to slowest abi_cross_cold_wideselect_warm_scalar (2.10 ms): 825.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 2544.4 ns median
- 3 variants significantly slower than baseline
- Spread: 824.96x (fastest 2544.4 ns, slowest 2098987.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 4979ns | 4965ns | 4910ns | 4950ns | 5056ns | +2.80% |
| abi_cross_cold_wideselect_cold_scalar | 2101970ns | 2098721ns | 2092094ns | 2097005ns | 2114357ns | +43300.45% |
| abi_cross_cold_wideselect_warm_null | 4843ns | 4805ns | 4716ns | 4782ns | 4999ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2104019ns | 2102470ns | 2091795ns | 2099345ns | 2117141ns | +43342.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 2713ns | 2695ns | 2743ns | +5.99% | 0.006 |
| abi_cross_cold_wideselect_cold_scalar | 2098494ns | 2088906ns | 2110441ns | +81878.85% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 2560ns | 2498ns | 2637ns | base | 0.006 |
| abi_cross_cold_wideselect_warm_scalar | 2100478ns | 2088428ns | 2113437ns | +81956.35% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 34092.4 | 2835.3 | 2713.3 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 75088.6 | 2094801.0 | 2098494.5 | n/a |
| abi_cross_cold_wideselect_warm_null | 29051.3 | 2694.2 | 2559.8 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 70239.3 | 2097446.5 | 2100478.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.006 | 92.5% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_wideselect_warm_null | 0.006 | 98.2% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 4979ns | 4979ns | +2.80% |
| abi_cross_cold_wideselect_cold_scalar | 2101970ns | 2101970ns | +43300.45% |
| abi_cross_cold_wideselect_warm_null | 4843ns | 4843ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2104019ns | 2104019ns | +43342.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 2544ns | base | --- | [2498, 2637] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 2701ns | +155.2ns (+6.1%) | [+94, +212]ns | [2696, 2743] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2095429ns | +2092886.6ns (+82256.2%) | [+2087067, +2107850]ns | [2089614, 2110441] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2098988ns | +2096443.5ns (+82396.0%) | [+2086466, +2110846]ns | [2089010, 2113437] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 2499ns | +7.9% | +83909.9% | +83523.9% |
| 2 | 2585ns | +4.4% | +80801.2% | +81114.7% |
| 3 | 2498ns | +7.9% | +84178.4% | +84311.2% |
| 4 | 2589ns | +4.4% | +80590.1% | +80571.6% |
| 5 | 2503ns | +9.0% | +83402.6% | +83719.4% |
| 6 | 2685ns | +2.7% | +78708.5% | +78808.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.381 | moderate+ |
| abi_cross_cold_wideselect_cold_scalar | -0.324 | moderate- |
| abi_cross_cold_wideselect_warm_null | -0.490 | moderate- |
| abi_cross_cold_wideselect_warm_scalar | -0.165 | ok |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 125063.1ns | 2713.3ns | 4609.3% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6366660.8ns | 2098494.5ns | 303.4% | HIGH |
| abi_cross_cold_wideselect_warm_null | 118853.8ns | 2559.8ns | 4643.1% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6371317.2ns | 2100478.3ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 2695.4-2743.3 ns)
   2695.4 |########################################
   2697.8 |####################
   2700.2 |####################
   2702.6 |
   2705.0 |
   2707.4 |
   2709.8 |
   2712.2 |
   2714.6 |
   2717.0 |
   2719.4 |
   2721.7 |
   2724.1 |
   2726.5 |####################
   2728.9 |
   2731.3 |
   2733.7 |
   2736.1 |
   2738.5 |
   2740.9 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2088906.2-2110441.2 ns)
  2088906.2 |########################################
  2089983.0 |########################################
  2091059.7 |########################################
  2092136.5 |
  2093213.2 |
  2094290.0 |
  2095366.7 |
  2096443.5 |
  2097520.2 |
  2098597.0 |########################################
  2099673.7 |
  2100750.5 |
  2101827.2 |
  2102904.0 |
  2103980.7 |
  2105057.5 |########################################
  2106134.2 |
  2107211.0 |
  2108287.7 |
  2109364.5 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 2497.9-2636.7 ns)
   2497.9 |########################################
   2504.8 |
   2511.8 |
   2518.7 |
   2525.7 |
   2532.6 |
   2539.5 |
   2546.5 |
   2553.4 |
   2560.4 |
   2567.3 |
   2574.2 |
   2581.2 |#############
   2588.1 |#############
   2595.1 |
   2602.0 |
   2608.9 |
   2615.9 |
   2622.8 |
   2629.8 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2088427.5-2113436.9 ns)
  2088427.5 |########################################
  2089678.0 |
  2090928.4 |
  2092178.9 |
  2093429.4 |
  2094679.9 |
  2095930.3 |
  2097180.8 |####################
  2098431.3 |
  2099681.7 |####################
  2100932.2 |
  2102182.7 |
  2103433.1 |
  2104683.6 |
  2105934.1 |
  2107184.6 |
  2108435.0 |####################
  2109685.5 |
  2110936.0 |
  2112186.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=4624.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=4677.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=303.1% of algo (FFI overhead may distort results)

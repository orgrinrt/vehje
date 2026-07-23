# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_cold_scalar is an outlier: 860.9x slower than the field

abi_cross_cold_real_cold_scalar (2.19 ms) is 860.9x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (2.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (80784% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 80784% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 860.9x the fastest

Fastest abi_cross_cold_real_warm_null (2.54 us) to slowest abi_cross_cold_real_cold_scalar (2.19 ms): 860.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 2538.9 ns median
- 3 variants significantly slower than baseline
- Spread: 860.88x (fastest 2538.9 ns, slowest 2185729.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 5020ns | 4959ns | 4904ns | 4942ns | 5194ns | +3.86% |
| abi_cross_cold_real_cold_scalar | 2189279ns | 2189211ns | 2179682ns | 2187279ns | 2197078ns | +45196.22% |
| abi_cross_cold_real_warm_null | 4833ns | 4794ns | 4706ns | 4777ns | 4981ns | base |
| abi_cross_cold_real_warm_scalar | 2183774ns | 2179968ns | 2173810ns | 2177983ns | 2197444ns | +45082.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2731ns | 2675ns | 2823ns | +7.13% | 0.006 |
| abi_cross_cold_real_cold_scalar | 2185859ns | 2176334ns | 2193668ns | +85641.24% | 0.000 |
| abi_cross_cold_real_warm_null | 2549ns | 2480ns | 2610ns | base | 0.006 |
| abi_cross_cold_real_warm_scalar | 2180474ns | 2170425ns | 2194147ns | +85430.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33886.9 | 2844.7 | 2731.2 | n/a |
| abi_cross_cold_real_cold_scalar | 70227.7 | 2187138.1 | 2185858.5 | n/a |
| abi_cross_cold_real_warm_null | 28884.5 | 2721.8 | 2549.4 | n/a |
| abi_cross_cold_real_warm_scalar | 65911.1 | 2180510.4 | 2180474.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.006 | 92.1% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.006 | 97.7% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 5020ns | 5020ns | +3.86% |
| abi_cross_cold_real_cold_scalar | 2189279ns | 2189279ns | +45196.22% |
| abi_cross_cold_real_warm_null | 4833ns | 4833ns | base |
| abi_cross_cold_real_warm_scalar | 2183774ns | 2183774ns | +45082.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2539ns | base | --- | [2499, 2610] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2691ns | +149.6ns (+5.9%) | [+76, +320]ns | [2679, 2823] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2185730ns | +2183185.8ns (+85987.7%) | [+2175654, +2191087]ns | [2178178, 2193668] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2176618ns | +2174114.8ns (+85630.5%) | [+2168093, +2191568]ns | [2170658, 2194147] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2611ns | +2.4% | +84013.4% | +83019.8% |
| 2 | 2519ns | +6.5% | +86289.9% | +86073.8% |
| 3 | 2608ns | +3.4% | +83563.0% | +84461.5% |
| 4 | 2550ns | +5.3% | +85806.8% | +85481.7% |
| 5 | 2528ns | +15.7% | +86152.1% | +86061.1% |
| 6 | 2480ns | +9.8% | +88191.6% | +87636.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.100 | ok |
| abi_cross_cold_real_cold_scalar | -0.466 | moderate- |
| abi_cross_cold_real_warm_null | -0.153 | ok |
| abi_cross_cold_real_warm_scalar | -0.095 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 125398.5ns | 2731.2ns | 4591.4% | HIGH |
| abi_cross_cold_real_cold_scalar | 6629730.3ns | 2185858.5ns | 303.3% | HIGH |
| abi_cross_cold_real_warm_null | 119261.1ns | 2549.4ns | 4678.1% | HIGH |
| abi_cross_cold_real_warm_scalar | 6607827.4ns | 2180474.4ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2675.0-2823.3 ns)
   2675.0 |####################
   2682.4 |########################################
   2689.8 |####################
   2697.3 |
   2704.7 |
   2712.1 |
   2719.5 |####################
   2726.9 |
   2734.3 |
   2741.8 |
   2749.2 |
   2756.6 |
   2764.0 |
   2771.4 |
   2778.8 |
   2786.3 |
   2793.7 |
   2801.1 |
   2808.5 |
   2815.9 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2176334.2-2193668.1 ns)
  2176334.2 |########################################
  2177200.9 |
  2178067.6 |
  2178934.3 |
  2179801.0 |########################################
  2180667.7 |
  2181534.4 |########################################
  2182401.1 |
  2183267.8 |
  2184134.5 |
  2185001.1 |
  2185867.8 |
  2186734.5 |
  2187601.2 |
  2188467.9 |########################################
  2189334.6 |
  2190201.3 |########################################
  2191068.0 |
  2191934.7 |
  2192801.4 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2479.6-2609.8 ns)
   2479.6 |########################################
   2486.1 |
   2492.6 |
   2499.1 |
   2505.6 |
   2512.1 |
   2518.6 |########################################
   2525.2 |########################################
   2531.7 |
   2538.2 |
   2544.7 |########################################
   2551.2 |
   2557.7 |
   2564.2 |
   2570.7 |
   2577.2 |
   2583.7 |
   2590.2 |
   2596.7 |
   2603.2 |########################################
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2170425.4-2194146.9 ns)
  2170425.4 |########################################
  2171611.5 |
  2172797.5 |
  2173983.6 |
  2175169.7 |####################
  2176355.8 |
  2177541.9 |####################
  2178727.9 |
  2179914.0 |
  2181100.1 |
  2182286.1 |####################
  2183472.2 |
  2184658.3 |
  2185844.4 |
  2187030.4 |
  2188216.5 |
  2189402.6 |
  2190588.7 |
  2191774.8 |
  2192960.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=4653.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=4709.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.2% of algo (FFI overhead may distort results)

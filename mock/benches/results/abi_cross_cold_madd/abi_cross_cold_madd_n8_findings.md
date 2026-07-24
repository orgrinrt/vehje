# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_null dominates: 13% faster than the next best (abi_cross_cold_madd_cold_null)

abi_cross_cold_madd_warm_null (3.08 us) leads abi_cross_cold_madd_cold_null (3.49 us) by 13%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_madd_warm_scalar is an outlier: 902.3x slower than the field

abi_cross_cold_madd_warm_scalar (2.78 ms) is 902.3x the fastest (3.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (3.08 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} (79072% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} with a 79072% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 902.3x the fastest

Fastest abi_cross_cold_madd_warm_null (3.08 us) to slowest abi_cross_cold_madd_warm_scalar (2.78 ms): 902.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 3083.8 ns median
- 3 variants significantly slower than baseline
- Spread: 902.34x (fastest 3083.8 ns, slowest 2782594.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5944ns | 5788ns | 5560ns | 5757ns | 6418ns | +9.94% |
| abi_cross_cold_madd_cold_scalar | 2767710ns | 2765930ns | 2752861ns | 2764373ns | 2780139ns | +51085.95% |
| abi_cross_cold_madd_warm_null | 5407ns | 5425ns | 5289ns | 5381ns | 5505ns | base |
| abi_cross_cold_madd_warm_scalar | 2798273ns | 2786326ns | 2763372ns | 2783699ns | 2837584ns | +51651.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 3633ns | 3378ns | 3991ns | +18.22% | 0.002 |
| abi_cross_cold_madd_cold_scalar | 2763968ns | 2749409ns | 2776396ns | +89848.53% | 0.000 |
| abi_cross_cold_madd_warm_null | 3073ns | 2990ns | 3124ns | base | 0.003 |
| abi_cross_cold_madd_warm_scalar | 2794273ns | 2759690ns | 2833205ns | +90834.74% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 35639.1 | 3908.8 | 3632.6 | n/a |
| abi_cross_cold_madd_cold_scalar | 81796.2 | 2768300.2 | 2763968.5 | n/a |
| abi_cross_cold_madd_warm_null | 29761.5 | 3210.6 | 3072.8 | n/a |
| abi_cross_cold_madd_warm_scalar | 83542.4 | 2797162.4 | 2794272.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.002 | 85.7% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.003 | 97.0% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5944ns | 5944ns | +9.94% |
| abi_cross_cold_madd_cold_scalar | 2767710ns | 2767710ns | +51085.95% |
| abi_cross_cold_madd_warm_null | 5407ns | 5407ns | base |
| abi_cross_cold_madd_warm_scalar | 2798273ns | 2798273ns | +51651.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 3084ns | base | --- | [3011, 3124] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 3489ns | +411.2ns (+13.3%) | [+326, +942]ns | [3418, 3991] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_cold_scalar | 2762102ns | +2759001.0ns (+89469.0%) | [+2750321, +2773365]ns | [2753407, 2776396] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2782594ns | +2779563.0ns (+90135.8%) | [+2763957, +2830080]ns | [2767020, 2833205] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 3072ns | +12.9% | +90494.2% | +90284.8% |
| 2 | 3108ns | +41.4% | +88856.4% | +91539.6% |
| 3 | 3141ns | +11.7% | +87693.1% | +89644.9% |
| 4 | 3031ns | +14.1% | +90615.6% | +90954.8% |
| 5 | 2990ns | +20.0% | +92505.9% | +93135.5% |
| 6 | 3095ns | +9.1% | +89072.3% | +89539.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.247 | moderate- |
| abi_cross_cold_madd_cold_scalar | 0.001 | ok |
| abi_cross_cold_madd_warm_null | 0.076 | ok |
| abi_cross_cold_madd_warm_scalar | -0.023 | ok |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 129403.7ns | 3632.6ns | 3562.3% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8389810.4ns | 2763968.5ns | 303.5% | HIGH |
| abi_cross_cold_madd_warm_null | 122476.9ns | 3072.8ns | 3985.8% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8466386.1ns | 2794272.9ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 3377.9-3991.2 ns)
   3377.9 |####################
   3408.6 |
   3439.2 |########################################
   3469.9 |
   3500.6 |####################
   3531.2 |
   3561.9 |####################
   3592.6 |
   3623.2 |
   3653.9 |
   3684.6 |
   3715.2 |
   3745.9 |
   3776.6 |
   3807.2 |
   3837.9 |
   3868.6 |
   3899.2 |
   3929.9 |
   3960.6 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2749408.7-2776396.2 ns)
  2749408.7 |########################################
  2750758.1 |
  2752107.5 |
  2753456.8 |
  2754806.2 |
  2756155.6 |########################################
  2757505.0 |
  2758854.3 |########################################
  2760203.7 |
  2761553.1 |
  2762902.5 |
  2764251.9 |########################################
  2765601.2 |
  2766950.6 |
  2768300.0 |########################################
  2769649.4 |
  2770998.7 |
  2772348.1 |
  2773697.5 |
  2775046.9 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 2990.4-3124.2 ns)
   2990.4 |########################################
   2997.1 |
   3003.8 |
   3010.5 |
   3017.2 |
   3023.8 |
   3030.5 |########################################
   3037.2 |
   3043.9 |
   3050.6 |
   3057.3 |
   3064.0 |
   3070.7 |########################################
   3077.3 |
   3084.0 |
   3090.7 |########################################
   3097.4 |
   3104.1 |########################################
   3110.8 |
   3117.5 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2759690.0-2833204.6 ns)
  2759690.0 |########################################
  2763365.7 |
  2767041.5 |
  2770717.2 |########################################
  2774392.9 |########################################
  2778068.6 |
  2781744.4 |
  2785420.1 |########################################
  2789095.8 |
  2792771.6 |
  2796447.3 |
  2800123.0 |
  2803798.8 |
  2807474.5 |
  2811150.2 |
  2814826.0 |
  2818501.7 |########################################
  2822177.4 |
  2825853.1 |
  2829528.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=3616.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=3981.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=303.6% of algo (FFI overhead may distort results)

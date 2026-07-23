# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 15% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (2.98 us) leads abi_cross_cold_real_cold_null (3.42 us) by 15%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_cold_scalar is an outlier: 734.0x slower than the field

abi_cross_cold_real_cold_scalar (2.19 ms) is 734.0x the fastest (2.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (2.98 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (63638% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 63638% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 734.0x the fastest

Fastest abi_cross_cold_real_warm_null (2.98 us) to slowest abi_cross_cold_real_cold_scalar (2.19 ms): 734.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 2984.8 ns median
- 3 variants significantly slower than baseline
- Spread: 734.05x (fastest 2984.8 ns, slowest 2190978.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 5668ns | 5659ns | 5595ns | 5655ns | 5725ns | +7.74% |
| abi_cross_cold_real_cold_scalar | 2201242ns | 2194460ns | 2172412ns | 2189134ns | 2233818ns | +41741.54% |
| abi_cross_cold_real_warm_null | 5261ns | 5215ns | 5152ns | 5202ns | 5404ns | base |
| abi_cross_cold_real_warm_scalar | 2191756ns | 2183438ns | 2177969ns | 2182134ns | 2213083ns | +41561.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 3414ns | 3368ns | 3441ns | +13.55% | 0.002 |
| abi_cross_cold_real_cold_scalar | 2197599ns | 2168986ns | 2229717ns | +72989.25% | 0.000 |
| abi_cross_cold_real_warm_null | 3007ns | 2934ns | 3087ns | base | 0.003 |
| abi_cross_cold_real_warm_scalar | 2188289ns | 2174795ns | 2209364ns | +72679.61% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 32612.6 | 4034.5 | 3414.1 | n/a |
| abi_cross_cold_real_cold_scalar | 73763.9 | 2189085.9 | 2197598.7 | n/a |
| abi_cross_cold_real_warm_null | 28090.8 | 3120.9 | 3006.7 | n/a |
| abi_cross_cold_real_warm_scalar | 68274.7 | 2188305.4 | 2188288.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.002 | 85.8% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.003 | 98.3% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 5668ns | 5668ns | +7.74% |
| abi_cross_cold_real_cold_scalar | 2201242ns | 2201242ns | +41741.54% |
| abi_cross_cold_real_warm_null | 5261ns | 5261ns | base |
| abi_cross_cold_real_warm_scalar | 2191756ns | 2191756ns | +41561.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2985ns | base | --- | [2949, 3087] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 3420ns | +412.5ns (+13.8%) | [+348, +462]ns | [3381, 3441] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2190979ns | +2187980.0ns (+73304.1%) | [+2169122, +2226674]ns | [2172100, 2229717] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2179960ns | +2176939.5ns (+72934.2%) | [+2172579, +2206327]ns | [2175542, 2209364] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2978ns | +13.1% | +74239.2% | +73100.1% |
| 2 | 2964ns | +15.6% | +73085.1% | +75363.6% |
| 3 | 3064ns | +12.8% | +71208.3% | +71066.1% |
| 4 | 2934ns | +15.7% | +74795.7% | +74082.4% |
| 5 | 2992ns | +14.5% | +72598.6% | +72584.6% |
| 6 | 3110ns | +9.8% | +72127.4% | +70076.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.234 | moderate- |
| abi_cross_cold_real_cold_scalar | -0.283 | moderate- |
| abi_cross_cold_real_warm_null | -0.263 | moderate- |
| abi_cross_cold_real_warm_scalar | -0.164 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 119201.1ns | 3414.1ns | 3491.5% | HIGH |
| abi_cross_cold_real_cold_scalar | 6645127.9ns | 2197598.7ns | 302.4% | HIGH |
| abi_cross_cold_real_warm_null | 120923.8ns | 3006.7ns | 4021.8% | HIGH |
| abi_cross_cold_real_warm_scalar | 6646947.8ns | 2188288.7ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 3367.5-3440.6 ns)
   3367.5 |########################################
   3371.2 |
   3374.8 |
   3378.5 |
   3382.1 |
   3385.8 |
   3389.4 |
   3393.1 |########################################
   3396.7 |
   3400.4 |
   3404.1 |
   3407.7 |
   3411.4 |########################################
   3415.0 |
   3418.7 |
   3422.3 |########################################
   3426.0 |########################################
   3429.6 |
   3433.3 |
   3436.9 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2168985.8-2229717.0 ns)
  2168985.8 |########################################
  2172022.4 |
  2175058.9 |########################################
  2178095.5 |
  2181132.0 |
  2184168.6 |########################################
  2187205.2 |
  2190241.7 |
  2193278.3 |
  2196314.9 |########################################
  2199351.4 |
  2202388.0 |
  2205424.5 |
  2208461.1 |
  2211497.7 |########################################
  2214534.2 |
  2217570.8 |
  2220607.4 |
  2223643.9 |
  2226680.5 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2933.7-3086.7 ns)
   2933.7 |########################################
   2941.3 |
   2949.0 |
   2956.6 |########################################
   2964.3 |
   2971.9 |########################################
   2979.6 |
   2987.2 |########################################
   2994.9 |
   3002.5 |
   3010.2 |
   3017.8 |
   3025.5 |
   3033.1 |
   3040.8 |
   3048.4 |
   3056.1 |
   3063.8 |########################################
   3071.4 |
   3079.0 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2174794.6-2209363.5 ns)
  2174794.6 |########################################
  2176523.0 |
  2178251.5 |####################
  2179979.9 |####################
  2181708.4 |####################
  2183436.8 |
  2185165.3 |
  2186893.7 |
  2188622.2 |
  2190350.6 |
  2192079.1 |
  2193807.5 |
  2195536.0 |
  2197264.4 |
  2198992.9 |
  2200721.3 |
  2202449.8 |
  2204178.2 |
  2205906.7 |
  2207635.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=3549.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=4054.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.7% of algo (FFI overhead may distort results)

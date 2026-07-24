# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_cold_scalar is an outlier: 691.4x slower than the field

abi_cross_cold_real_cold_scalar (2.20 ms) is 691.4x the fastest (3.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null) are a dead heat (<1%)

abi_cross_cold_real_warm_null (3.18 us) and abi_cross_cold_real_cold_null (3.20 us) differ by 0.78%, inside the noise, even though the wider field spreads 69039.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (3.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (68314% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 68314% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 691.4x the fastest

Fastest abi_cross_cold_real_warm_null (3.18 us) to slowest abi_cross_cold_real_cold_scalar (2.20 ms): 691.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_real_cold_null's edge over baseline is significant but tiny (29 ns, 0.91%)

abi_cross_cold_real_cold_null differs from baseline abi_cross_cold_real_warm_null by 29 ns (0.91%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 3176.1 ns median
- 2 variants significantly slower than baseline
- Spread: 691.39x (fastest 3176.1 ns, slowest 2195898.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 6254ns | 5509ns | 5352ns | 5483ns | 7860ns | +13.33% |
| abi_cross_cold_real_cold_scalar | 2244493ns | 2199749ns | 2183309ns | 2195734ns | 2348222ns | +40573.38% |
| abi_cross_cold_real_warm_null | 5518ns | 5513ns | 5385ns | 5475ns | 5650ns | base |
| abi_cross_cold_real_warm_scalar | 2211769ns | 2193480ns | 2175213ns | 2190190ns | 2262417ns | +39980.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 3616ns | 3129ns | 4514ns | +13.05% | 0.071 |
| abi_cross_cold_real_cold_scalar | 2240267ns | 2179975ns | 2342836ns | +69937.52% | 0.000 |
| abi_cross_cold_real_warm_null | 3199ns | 3139ns | 3272ns | base | 0.080 |
| abi_cross_cold_real_warm_scalar | 2208081ns | 2171719ns | 2258519ns | +68931.30% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 43649.2 | 3524.8 | 3616.1 | n/a |
| abi_cross_cold_real_cold_scalar | 85291.0 | 2234545.3 | 2240266.6 | n/a |
| abi_cross_cold_real_warm_null | 29877.8 | 3246.8 | 3198.7 | n/a |
| abi_cross_cold_real_warm_scalar | 75423.6 | 2213110.6 | 2208081.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_cross_cold_real_cold_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.080 | 97.8% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.081 | 98.5% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 6254ns | 6254ns | +13.33% |
| abi_cross_cold_real_cold_scalar | 2244493ns | 2244493ns | +40573.38% |
| abi_cross_cold_real_warm_null | 5518ns | 5518ns | base |
| abi_cross_cold_real_warm_scalar | 2211769ns | 2211769ns | +39980.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 3176ns | base | --- | [3148, 3272] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 3201ns | no significant difference | [-67, +1291]ns | [3134, 4514] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_cold_real_cold_scalar | 2195898ns | +2192739.8ns (+69039.8%) | [+2178901, +2339564]ns | [2182066, 2342836] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2189809ns | +2186650.2ns (+68848.1%) | [+2172751, +2255247]ns | [2175916, 2258519] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 3160ns | +6.5% | +69726.0% | +68963.4% |
| 2 | 3139ns | -0.0% | +69354.7% | +69359.1% |
| 3 | 3192ns | -0.2% | +68332.4% | +67942.7% |
| 4 | 3157ns | +1.8% | +69109.6% | +69487.2% |
| 5 | 3286ns | +72.3% | +74614.9% | +67911.0% |
| 6 | 3258ns | -4.0% | +68361.8% | +69946.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.261 | moderate- |
| abi_cross_cold_real_cold_scalar | -0.095 | ok |
| abi_cross_cold_real_warm_null | 0.254 | moderate+ |
| abi_cross_cold_real_warm_scalar | 0.419 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 2/6, lost 3/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 142364.0ns | 3616.1ns | 3936.9% | HIGH |
| abi_cross_cold_real_cold_scalar | 6824098.4ns | 2240266.6ns | 304.6% | HIGH |
| abi_cross_cold_real_warm_null | 122617.7ns | 3198.7ns | 3833.4% | HIGH |
| abi_cross_cold_real_warm_scalar | 6720050.2ns | 2208081.2ns | 304.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 3129.2-4513.9 ns)
   3129.2 |########################################
   3198.4 |#############
   3267.7 |
   3336.9 |#############
   3406.1 |
   3475.4 |
   3544.6 |
   3613.9 |
   3683.1 |
   3752.3 |
   3821.6 |
   3890.8 |
   3960.0 |
   4029.3 |
   4098.5 |
   4167.8 |
   4237.0 |
   4306.2 |
   4375.5 |
   4444.7 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2179974.6-2342835.6 ns)
  2179974.6 |########################################
  2188117.6 |
  2196260.7 |
  2204403.8 |#############
  2212546.8 |
  2220689.9 |
  2228832.9 |#############
  2236976.0 |
  2245119.0 |
  2253262.1 |
  2261405.1 |
  2269548.1 |
  2277691.2 |
  2285834.2 |
  2293977.3 |
  2302120.4 |
  2310263.4 |
  2318406.5 |
  2326549.5 |
  2334692.6 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 3138.7-3272.1 ns)
   3138.7 |########################################
   3145.4 |
   3152.0 |########################################
   3158.7 |########################################
   3165.4 |
   3172.0 |
   3178.7 |
   3185.4 |########################################
   3192.0 |
   3198.7 |
   3205.4 |
   3212.0 |
   3218.7 |
   3225.4 |
   3232.0 |
   3238.7 |
   3245.4 |
   3252.0 |########################################
   3258.7 |
   3265.4 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2171718.8-2258518.8 ns)
  2171718.8 |########################################
  2176058.8 |########################################
  2180398.8 |########################################
  2184738.8 |
  2189078.8 |
  2193418.8 |########################################
  2197758.8 |
  2202098.8 |
  2206438.8 |
  2210778.8 |
  2215118.8 |
  2219458.8 |
  2223798.8 |
  2228138.8 |
  2232478.8 |########################################
  2236818.8 |
  2241158.8 |
  2245498.8 |
  2249838.8 |
  2254178.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: CV=25.4% (high variance, measurements may be unstable)
- **abi_cross_cold_real_cold_null**: bridge=4070.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=3863.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=305.7% of algo (FFI overhead may distort results)

# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_cold_scalar is an outlier: 694.4x slower than the field

abi_cross_cold_real_cold_scalar (2.15 ms) is 694.4x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (3.10 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (68166% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 68166% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 694.4x the fastest

Fastest abi_cross_cold_real_warm_null (3.10 us) to slowest abi_cross_cold_real_cold_scalar (2.15 ms): 694.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_real_cold_null's edge over baseline is significant but tiny (25 ns, 0.80%)

abi_cross_cold_real_cold_null differs from baseline abi_cross_cold_real_warm_null by 25 ns (0.80%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 3098.6 ns median
- 2 variants significantly slower than baseline
- Spread: 694.41x (fastest 3098.6 ns, slowest 2151670.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 5472ns | 5484ns | 5305ns | 5447ns | 5594ns | +2.13% |
| abi_cross_cold_real_cold_scalar | 2155448ns | 2154646ns | 2149283ns | 2154000ns | 2160703ns | +40126.59% |
| abi_cross_cold_real_warm_null | 5358ns | 5303ns | 5258ns | 5300ns | 5496ns | base |
| abi_cross_cold_real_warm_scalar | 2152875ns | 2153914ns | 2143020ns | 2151254ns | 2160233ns | +40078.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 3180ns | 3096ns | 3275ns | +1.72% | 0.081 |
| abi_cross_cold_real_cold_scalar | 2152484ns | 2146402ns | 2157598ns | +68761.13% | 0.000 |
| abi_cross_cold_real_warm_null | 3126ns | 3085ns | 3194ns | base | 0.082 |
| abi_cross_cold_real_warm_scalar | 2149955ns | 2140413ns | 2157165ns | +68680.22% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 31623.8 | 3188.0 | 3179.5 | n/a |
| abi_cross_cold_real_cold_scalar | 57217.9 | 2151036.0 | 2152484.0 | n/a |
| abi_cross_cold_real_warm_null | 27104.0 | 3145.1 | 3125.8 | n/a |
| abi_cross_cold_real_warm_scalar | 51222.0 | 2151724.2 | 2149955.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.081 | 97.9% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.083 | 99.6% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 5472ns | 5472ns | +2.13% |
| abi_cross_cold_real_cold_scalar | 2155448ns | 2155448ns | +40126.59% |
| abi_cross_cold_real_warm_null | 5358ns | 5358ns | base |
| abi_cross_cold_real_warm_scalar | 2152875ns | 2152875ns | +40078.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 3099ns | base | --- | [3085, 3194] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 3151ns | no significant difference | [-43, +179]ns | [3112, 3275] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_real_cold_scalar | 2151670ns | +2148499.5ns (+69338.9%) | [+2145097, +2154478]ns | [2148184, 2157598] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2150913ns | +2147742.3ns (+69314.4%) | [+2138669, +2154077]ns | [2141788, 2157165] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 3090ns | +0.2% | +69478.2% | +69815.2% |
| 2 | 3107ns | +7.7% | +69145.5% | +69092.4% |
| 3 | 3234ns | -2.3% | +66433.1% | +66437.3% |
| 4 | 3085ns | +1.4% | +69466.4% | +69361.4% |
| 5 | 3085ns | +3.8% | +69916.8% | +69711.0% |
| 6 | 3153ns | -0.4% | +68246.5% | +67787.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.458 | moderate- |
| abi_cross_cold_real_cold_scalar | -0.193 | ok |
| abi_cross_cold_real_warm_null | -0.299 | moderate- |
| abi_cross_cold_real_warm_scalar | -0.300 | moderate- |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 2/6, lost 4/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 125311.9ns | 3179.5ns | 3941.2% | HIGH |
| abi_cross_cold_real_cold_scalar | 6514783.2ns | 2152484.0ns | 302.7% | HIGH |
| abi_cross_cold_real_warm_null | 120299.2ns | 3125.8ns | 3848.5% | HIGH |
| abi_cross_cold_real_warm_scalar | 6508493.2ns | 2149955.1ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 3095.8-3275.2 ns)
   3095.8 |########################################
   3104.8 |
   3113.7 |
   3122.7 |########################################
   3131.7 |########################################
   3140.7 |
   3149.6 |
   3158.6 |########################################
   3167.6 |
   3176.6 |
   3185.5 |
   3194.5 |
   3203.5 |########################################
   3212.4 |
   3221.4 |
   3230.4 |
   3239.4 |
   3248.3 |
   3257.3 |
   3266.3 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2146401.7-2157597.5 ns)
  2146401.7 |####################
  2146961.5 |
  2147521.3 |
  2148081.1 |
  2148640.9 |
  2149200.7 |
  2149760.4 |####################
  2150320.2 |
  2150880.0 |
  2151439.8 |########################################
  2151999.6 |
  2152559.4 |
  2153119.2 |
  2153679.0 |
  2154238.8 |
  2154798.5 |####################
  2155358.3 |
  2155918.1 |
  2156477.9 |
  2157037.7 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 3085.4-3193.6 ns)
   3085.4 |########################################
   3090.8 |
   3096.2 |
   3101.6 |
   3107.0 |#############
   3112.4 |
   3117.8 |
   3123.3 |
   3128.7 |
   3134.1 |
   3139.5 |
   3144.9 |
   3150.3 |#############
   3155.7 |
   3161.1 |
   3166.5 |
   3171.9 |
   3177.3 |
   3182.7 |
   3188.1 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2140413.3-2157164.5 ns)
  2140413.3 |########################################
  2141250.9 |
  2142088.4 |
  2142926.0 |########################################
  2143763.5 |
  2144601.1 |
  2145438.7 |
  2146276.2 |
  2147113.8 |
  2147951.4 |
  2148788.9 |
  2149626.5 |########################################
  2150464.0 |
  2151301.6 |########################################
  2152139.2 |
  2152976.7 |
  2153814.3 |########################################
  2154651.9 |
  2155489.4 |
  2156327.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=3989.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=3860.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=302.6% of algo (FFI overhead may distort results)

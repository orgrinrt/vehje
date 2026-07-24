# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 303% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (3.60 us) leads abi_cross_cold_real_cold_null (14.50 us) by 303%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_cold_scalar is an outlier: 615.1x slower than the field

abi_cross_cold_real_cold_scalar (2.21 ms) is 615.1x the fastest (3.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (3.60 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (15025% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 15025% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 615.1x the fastest

Fastest abi_cross_cold_real_warm_null (3.60 us) to slowest abi_cross_cold_real_cold_scalar (2.21 ms): 615.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 3599.0 ns median
- 3 variants significantly slower than baseline
- Spread: 615.09x (fastest 3599.0 ns, slowest 2213726.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 16713ns | 16744ns | 15161ns | 16340ns | 18048ns | +182.10% |
| abi_cross_cold_real_cold_scalar | 2247761ns | 2217376ns | 2197899ns | 2213954ns | 2323402ns | +37840.95% |
| abi_cross_cold_real_warm_null | 5924ns | 5837ns | 5726ns | 5822ns | 6178ns | base |
| abi_cross_cold_real_warm_scalar | 2195874ns | 2196274ns | 2190279ns | 2194913ns | 2200112ns | +36965.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 14437ns | 12831ns | 15755ns | +296.85% | 0.000 |
| abi_cross_cold_real_cold_scalar | 2243978ns | 2194769ns | 2318955ns | +61581.64% | 0.000 |
| abi_cross_cold_real_warm_null | 3638ns | 3514ns | 3773ns | base | 0.001 |
| abi_cross_cold_real_warm_scalar | 2192419ns | 2186825ns | 2196452ns | +60164.39% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33972.1 | 19574.9 | 14437.4 | n/a |
| abi_cross_cold_real_cold_scalar | 78469.9 | 2238928.3 | 2243978.1 | n/a |
| abi_cross_cold_real_warm_null | 28270.8 | 3752.4 | 3638.0 | n/a |
| abi_cross_cold_real_warm_scalar | 71190.8 | 2196834.1 | 2192418.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.000 | 24.2% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_real_warm_null | 0.001 | 97.6% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 16713ns | 16713ns | +182.10% |
| abi_cross_cold_real_cold_scalar | 2247761ns | 2247761ns | +37840.95% |
| abi_cross_cold_real_warm_null | 5924ns | 5924ns | base |
| abi_cross_cold_real_warm_scalar | 2195874ns | 2195874ns | +36965.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 3599ns | base | --- | [3542, 3773] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 14498ns | +10833.7ns (+301.0%) | [+9460, +12105]ns | [13058, 15755] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2213726ns | +2210184.1ns (+61411.1%) | [+2195539, +2315297]ns | [2199254, 2318955] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2192850ns | +2189185.6ns (+60827.6%) | [+2184362, +2192794]ns | [2187954, 2196452] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 3584ns | +258.0% | +62568.6% | +61187.2% |
| 2 | 3514ns | +335.2% | +62806.8% | +62393.8% |
| 3 | 3570ns | +332.0% | +62001.6% | +61162.5% |
| 4 | 3815ns | +259.2% | +57665.1% | +57293.0% |
| 5 | 3731ns | +331.2% | +64007.9% | +58768.3% |
| 6 | 3614ns | +267.6% | +60626.3% | +60468.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.492 | moderate- |
| abi_cross_cold_real_cold_scalar | -0.407 | moderate- |
| abi_cross_cold_real_warm_null | 0.273 | moderate+ |
| abi_cross_cold_real_warm_scalar | -0.157 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 150386.4ns | 14437.4ns | 1041.6% | HIGH |
| abi_cross_cold_real_cold_scalar | 6803485.7ns | 2243978.1ns | 303.2% | HIGH |
| abi_cross_cold_real_warm_null | 122325.9ns | 3638.0ns | 3362.4% | HIGH |
| abi_cross_cold_real_warm_scalar | 6660147.9ns | 2192418.6ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 12831.2-15755.4 ns)
  12831.2 |########################################
  12977.4 |
  13123.6 |
  13269.8 |########################################
  13416.0 |
  13562.2 |########################################
  13708.5 |
  13854.7 |
  14000.9 |
  14147.1 |
  14293.3 |
  14439.5 |
  14585.7 |
  14731.9 |
  14878.1 |
  15024.4 |
  15170.6 |########################################
  15316.8 |########################################
  15463.0 |
  15609.2 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2194769.2-2318954.8 ns)
  2194769.2 |########################################
  2200978.5 |########################################
  2207187.8 |########################################
  2213397.0 |########################################
  2219606.3 |
  2225815.6 |
  2232024.9 |
  2238234.1 |
  2244443.4 |########################################
  2250652.7 |
  2256862.0 |
  2263071.3 |
  2269280.5 |
  2275489.8 |
  2281699.1 |
  2287908.4 |
  2294117.6 |
  2300326.9 |
  2306536.2 |
  2312745.5 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 3514.2-3773.1 ns)
   3514.2 |########################################
   3527.1 |
   3540.1 |
   3553.0 |
   3566.0 |########################################
   3578.9 |########################################
   3591.9 |
   3604.8 |########################################
   3617.8 |
   3630.7 |
   3643.6 |
   3656.6 |
   3669.5 |
   3682.5 |
   3695.4 |
   3708.4 |
   3721.3 |########################################
   3734.3 |
   3747.2 |
   3760.2 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2186825.4-2196451.5 ns)
  2186825.4 |####################
  2187306.7 |
  2187788.0 |
  2188269.3 |
  2188750.6 |####################
  2189231.9 |####################
  2189713.2 |
  2190194.5 |
  2190675.8 |
  2191157.1 |
  2191638.5 |
  2192119.8 |
  2192601.1 |
  2193082.4 |
  2193563.7 |
  2194045.0 |
  2194526.3 |
  2195007.6 |
  2195488.9 |
  2195970.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=1032.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=3401.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.7% of algo (FFI overhead may distort results)

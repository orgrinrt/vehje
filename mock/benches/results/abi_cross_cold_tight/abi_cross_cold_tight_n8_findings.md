# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_warm_null dominates: 14% faster than the next best (abi_cross_cold_tight_cold_null)

abi_cross_cold_tight_warm_null (3.02 us) leads abi_cross_cold_tight_cold_null (3.43 us) by 14%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_tight_warm_scalar is an outlier: 679.5x slower than the field

abi_cross_cold_tight_warm_scalar (2.05 ms) is 679.5x the fastest (3.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_tight_warm_null)

The baseline abi_cross_cold_tight_warm_null is the fastest (3.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} vs {abi_cross_cold_tight_cold_scalar, abi_cross_cold_tight_warm_scalar} (59486% apart)

The field splits into a fast tier {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} and a slow tier {abi_cross_cold_tight_cold_scalar, abi_cross_cold_tight_warm_scalar} with a 59486% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 679.5x the fastest

Fastest abi_cross_cold_tight_warm_null (3.02 us) to slowest abi_cross_cold_tight_warm_scalar (2.05 ms): 679.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_tight_warm_null) is the fastest** at 3021.1 ns median
- 3 variants significantly slower than baseline
- Spread: 679.48x (fastest 3021.1 ns, slowest 2052730.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5673ns | 5674ns | 5567ns | 5640ns | 5775ns | +7.30% |
| abi_cross_cold_tight_cold_scalar | 2050801ns | 2049130ns | 2041972ns | 2048307ns | 2058955ns | +38690.96% |
| abi_cross_cold_tight_warm_null | 5287ns | 5298ns | 5147ns | 5272ns | 5378ns | base |
| abi_cross_cold_tight_warm_scalar | 2054020ns | 2056478ns | 2041592ns | 2053078ns | 2061648ns | +38751.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 3435ns | 3370ns | 3501ns | +13.77% | 0.002 |
| abi_cross_cold_tight_cold_scalar | 2047394ns | 2038700ns | 2055359ns | +67706.86% | 0.000 |
| abi_cross_cold_tight_warm_null | 3019ns | 2936ns | 3074ns | base | 0.003 |
| abi_cross_cold_tight_warm_scalar | 2050320ns | 2037909ns | 2057891ns | +67803.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 31339.2 | 3560.8 | 3435.2 | n/a |
| abi_cross_cold_tight_cold_scalar | 72673.2 | 2048578.2 | 2047394.1 | n/a |
| abi_cross_cold_tight_warm_null | 27769.8 | 3107.3 | 3019.5 | n/a |
| abi_cross_cold_tight_warm_scalar | 73499.2 | 2049410.6 | 2050320.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.002 | 85.5% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_tight_warm_null | 0.003 | 97.2% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5673ns | 5673ns | +7.30% |
| abi_cross_cold_tight_cold_scalar | 2050801ns | 2050801ns | +38690.96% |
| abi_cross_cold_tight_warm_null | 5287ns | 5287ns | base |
| abi_cross_cold_tight_warm_scalar | 2054020ns | 2054020ns | +38751.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 3021ns | base | --- | [2963, 3074] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 3433ns | +432.3ns (+14.3%) | [+329, +486]ns | [3371, 3501] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_cold_scalar | 2045792ns | +2042761.5ns (+67617.6%) | [+2038010, +2052353]ns | [2041031, 2055359] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2052731ns | +2049715.5ns (+67847.8%) | [+2037301, +2054887]ns | [2040340, 2057891] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 3002ns | +14.2% | +67818.2% | +67953.8% |
| 2 | 2936ns | +14.9% | +69727.2% | +69966.1% |
| 3 | 3072ns | +9.7% | +66449.0% | +66915.0% |
| 4 | 3040ns | +14.6% | +67107.0% | +67332.0% |
| 5 | 2990ns | +17.7% | +68374.9% | +68637.8% |
| 6 | 3077ns | +11.7% | +66869.5% | +66136.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.314 | moderate+ |
| abi_cross_cold_tight_cold_scalar | -0.075 | ok |
| abi_cross_cold_tight_warm_null | -0.280 | moderate- |
| abi_cross_cold_tight_warm_scalar | -0.167 | ok |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 123612.4ns | 3435.2ns | 3598.4% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6226543.1ns | 2047394.1ns | 304.1% | HIGH |
| abi_cross_cold_tight_warm_null | 120238.9ns | 3019.5ns | 3982.1% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6221106.5ns | 2050320.3ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 3369.6-3500.8 ns)
   3369.6 |########################################
   3376.2 |
   3382.7 |
   3389.3 |
   3395.8 |
   3402.4 |
   3409.0 |
   3415.5 |
   3422.1 |
   3428.6 |####################
   3435.2 |####################
   3441.8 |
   3448.3 |
   3454.9 |
   3461.4 |
   3468.0 |
   3474.6 |
   3481.1 |####################
   3487.7 |
   3494.2 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2038700.0-2055359.1 ns)
  2038700.0 |########################################
  2039533.0 |
  2040365.9 |
  2041198.9 |
  2042031.8 |
  2042864.8 |########################################
  2043697.7 |########################################
  2044530.7 |
  2045363.7 |
  2046196.6 |
  2047029.6 |########################################
  2047862.5 |
  2048695.5 |
  2049528.4 |########################################
  2050361.4 |
  2051194.4 |
  2052027.3 |
  2052860.3 |
  2053693.2 |
  2054526.2 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 2936.2-3074.2 ns)
   2936.2 |########################################
   2943.1 |
   2950.0 |
   2956.9 |
   2963.8 |
   2970.7 |
   2977.6 |
   2984.5 |########################################
   2991.4 |
   2998.3 |########################################
   3005.2 |
   3012.1 |
   3019.0 |
   3025.9 |
   3032.8 |
   3039.7 |########################################
   3046.6 |
   3053.5 |
   3060.4 |
   3067.3 |########################################
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2037908.7-2057890.6 ns)
  2037908.7 |########################################
  2038907.8 |
  2039906.9 |
  2040906.0 |
  2041905.1 |########################################
  2042904.2 |
  2043903.3 |
  2044902.4 |
  2045901.5 |
  2046900.6 |
  2047899.6 |
  2048898.7 |
  2049897.8 |########################################
  2050896.9 |
  2051896.0 |
  2052895.1 |
  2053894.2 |
  2054893.3 |########################################
  2055892.4 |
  2056891.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=3611.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=3986.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=303.1% of algo (FFI overhead may distort results)

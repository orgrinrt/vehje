# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_warm_null dominates: 127% faster than the next best (abi_cross_cold_tight_cold_null)

abi_cross_cold_tight_warm_null (3.44 us) leads abi_cross_cold_tight_cold_null (7.81 us) by 127%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_tight_cold_scalar is an outlier: 597.4x slower than the field

abi_cross_cold_tight_cold_scalar (2.06 ms) is 597.4x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_tight_warm_null)

The baseline abi_cross_cold_tight_warm_null is the fastest (3.44 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (26162% apart)

The field splits into a fast tier {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 26162% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 597.4x the fastest

Fastest abi_cross_cold_tight_warm_null (3.44 us) to slowest abi_cross_cold_tight_cold_scalar (2.06 ms): 597.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_tight_warm_null) is the fastest** at 3440.6 ns median
- 3 variants significantly slower than baseline
- Spread: 597.40x (fastest 3440.6 ns, slowest 2055427.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 10158ns | 10109ns | 9942ns | 10061ns | 10412ns | +76.29% |
| abi_cross_cold_tight_cold_scalar | 2059735ns | 2058942ns | 2053008ns | 2057244ns | 2066835ns | +35644.61% |
| abi_cross_cold_tight_warm_null | 5762ns | 5773ns | 5612ns | 5721ns | 5899ns | base |
| abi_cross_cold_tight_warm_scalar | 2054157ns | 2055446ns | 2041877ns | 2053768ns | 2060883ns | +35547.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 7850ns | 7674ns | 8060ns | +127.84% | 0.000 |
| abi_cross_cold_tight_cold_scalar | 2056270ns | 2049802ns | 2063365ns | +59582.76% | 0.000 |
| abi_cross_cold_tight_warm_null | 3445ns | 3363ns | 3516ns | base | 0.001 |
| abi_cross_cold_tight_warm_scalar | 2050689ns | 2038418ns | 2057265ns | +59420.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 33505.5 | 10793.2 | 7849.8 | n/a |
| abi_cross_cold_tight_cold_scalar | 72640.8 | 2060991.7 | 2056270.0 | n/a |
| abi_cross_cold_tight_warm_null | 28526.2 | 3594.2 | 3445.3 | n/a |
| abi_cross_cold_tight_warm_scalar | 69043.3 | 2054075.2 | 2050689.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.000 | 43.0% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_tight_warm_null | 0.001 | 97.7% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 10158ns | 10158ns | +76.29% |
| abi_cross_cold_tight_cold_scalar | 2059735ns | 2059735ns | +35644.61% |
| abi_cross_cold_tight_warm_null | 5762ns | 5762ns | base |
| abi_cross_cold_tight_warm_scalar | 2054157ns | 2054157ns | +35547.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 3441ns | base | --- | [3379, 3516] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 7814ns | +4403.1ns (+128.0%) | [+4232, +4578]ns | [7675, 8060] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_cold_scalar | 2055427ns | +2052048.4ns (+59642.2%) | [+2046517, +2059909]ns | [2050017, 2063365] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2052018ns | +2048549.8ns (+59540.5%) | [+2039327, +2053855]ns | [2042784, 2057265] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 3423ns | +135.4% | +60233.1% | +59452.4% |
| 2 | 3542ns | +127.6% | +57781.8% | +57903.8% |
| 3 | 3458ns | +124.1% | +59171.9% | +59330.8% |
| 4 | 3363ns | +134.3% | +61083.1% | +61133.9% |
| 5 | 3491ns | +119.8% | +58957.8% | +58544.2% |
| 6 | 3395ns | +126.1% | +60380.9% | +60267.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.283 | moderate+ |
| abi_cross_cold_tight_cold_scalar | -0.162 | ok |
| abi_cross_cold_tight_warm_null | -0.374 | moderate- |
| abi_cross_cold_tight_warm_scalar | -0.059 | ok |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 116026.9ns | 7849.8ns | 1478.1% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6259591.3ns | 2056270.0ns | 304.4% | HIGH |
| abi_cross_cold_tight_warm_null | 121298.3ns | 3445.3ns | 3520.7% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6225817.6ns | 2050689.3ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 7673.7-8060.4 ns)
   7673.7 |########################################
   7693.0 |
   7712.4 |
   7731.7 |####################
   7751.0 |
   7770.4 |
   7789.7 |
   7809.0 |
   7828.4 |
   7847.7 |
   7867.0 |####################
   7886.4 |
   7905.7 |
   7925.1 |
   7944.4 |
   7963.7 |
   7983.1 |
   8002.4 |
   8021.7 |
   8041.1 |####################
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2049801.7-2063365.4 ns)
  2049801.7 |########################################
  2050479.9 |
  2051158.1 |
  2051836.3 |
  2052514.4 |
  2053192.6 |####################
  2053870.8 |
  2054549.0 |
  2055227.2 |
  2055905.4 |
  2056583.6 |
  2057261.8 |####################
  2057939.9 |
  2058618.1 |
  2059296.3 |
  2059974.5 |
  2060652.7 |
  2061330.9 |####################
  2062009.1 |
  2062687.3 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 3362.9-3516.4 ns)
   3362.9 |########################################
   3370.6 |
   3378.3 |
   3385.9 |
   3393.6 |########################################
   3401.3 |
   3409.0 |
   3416.6 |########################################
   3424.3 |
   3432.0 |
   3439.7 |
   3447.4 |
   3455.0 |########################################
   3462.7 |
   3470.4 |
   3478.1 |
   3485.7 |########################################
   3493.4 |
   3501.1 |
   3508.8 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2038418.3-2057265.4 ns)
  2038418.3 |####################
  2039360.7 |
  2040303.0 |
  2041245.4 |
  2042187.7 |
  2043130.1 |
  2044072.4 |
  2045014.8 |
  2045957.1 |
  2046899.5 |####################
  2047841.9 |
  2048784.2 |####################
  2049726.6 |
  2050668.9 |
  2051611.3 |
  2052553.6 |
  2053496.0 |
  2054438.3 |########################################
  2055380.7 |
  2056323.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=1499.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=3521.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=303.4% of algo (FFI overhead may distort results)

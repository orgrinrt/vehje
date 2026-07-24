# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_null dominates: 125% faster than the next best (abi_cross_cold_wideselect_cold_null)

abi_cross_cold_wideselect_warm_null (3.44 us) leads abi_cross_cold_wideselect_cold_null (7.72 us) by 125%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_wideselect_cold_scalar is an outlier: 615.6x slower than the field

abi_cross_cold_wideselect_cold_scalar (2.12 ms) is 615.6x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_wideselect_warm_null shows alternating (throttle bounce) (autocorr -0.55)

abi_cross_cold_wideselect_warm_null's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (3.44 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} (27256% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} with a 27256% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 615.6x the fastest

Fastest abi_cross_cold_wideselect_warm_null (3.44 us) to slowest abi_cross_cold_wideselect_cold_scalar (2.12 ms): 615.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 3435.6 ns median
- 3 variants significantly slower than baseline
- Spread: 615.63x (fastest 3435.6 ns, slowest 2115078.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 9905ns | 9984ns | 9364ns | 9871ns | 10228ns | +73.14% |
| abi_cross_cold_wideselect_cold_scalar | 2145408ns | 2118779ns | 2109905ns | 2118127ns | 2204082ns | +37401.23% |
| abi_cross_cold_wideselect_warm_null | 5721ns | 5707ns | 5681ns | 5702ns | 5770ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2132652ns | 2115175ns | 2104229ns | 2112227ns | 2177501ns | +37178.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 7630ns | 7128ns | 7908ns | +122.63% | 0.000 |
| abi_cross_cold_wideselect_cold_scalar | 2141791ns | 2106366ns | 2200520ns | +62392.39% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 3427ns | 3400ns | 3446ns | base | 0.001 |
| abi_cross_cold_wideselect_warm_scalar | 2128801ns | 2100631ns | 2173307ns | +62013.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 35396.9 | 11007.8 | 7630.1 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 82047.9 | 2113881.8 | 2141791.3 | n/a |
| abi_cross_cold_wideselect_warm_null | 29957.5 | 3534.2 | 3427.3 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 83121.7 | 2188676.4 | 2128801.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.000 | 44.1% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_wideselect_warm_null | 0.001 | 99.0% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 9905ns | 9905ns | +73.14% |
| abi_cross_cold_wideselect_cold_scalar | 2145408ns | 2145408ns | +37401.23% |
| abi_cross_cold_wideselect_warm_null | 5721ns | 5721ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2132652ns | 2132652ns | +37178.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 3436ns | base | --- | [3400, 3446] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 7718ns | +4282.5ns (+124.6%) | [+3842, +4484]ns | [7264, 7908] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2115078ns | +2111633.1ns (+61462.4%) | [+2106354, +2197105]ns | [2109776, 2200520] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2111341ns | +2107916.9ns (+61354.2%) | [+2098320, +2169885]ns | [2101756, 2173307] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 3442ns | +125.9% | +61375.9% | +60934.7% |
| 2 | 3400ns | +117.6% | +66752.1% | +63529.1% |
| 3 | 3443ns | +107.0% | +61072.9% | +63297.6% |
| 4 | 3430ns | +123.4% | +61942.2% | +61215.6% |
| 5 | 3449ns | +130.5% | +61208.3% | +61262.6% |
| 6 | 3400ns | +131.4% | +62052.5% | +61855.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.313 | moderate+ |
| abi_cross_cold_wideselect_cold_scalar | -0.305 | moderate- |
| abi_cross_cold_wideselect_warm_null | -0.549 | HIGH- (thermal bounce) |
| abi_cross_cold_wideselect_warm_scalar | 0.017 | ok |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 117970.0ns | 7630.1ns | 1546.1% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6437330.8ns | 2141791.3ns | 300.6% | HIGH |
| abi_cross_cold_wideselect_warm_null | 122243.3ns | 3427.3ns | 3566.8% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6564687.0ns | 2128801.3ns | 308.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 7127.5-7908.3 ns)
   7127.5 |########################################
   7166.5 |
   7205.6 |
   7244.6 |
   7283.7 |
   7322.7 |
   7361.7 |########################################
   7400.8 |
   7439.8 |
   7478.9 |
   7517.9 |
   7556.9 |
   7596.0 |
   7635.0 |########################################
   7674.1 |
   7713.1 |
   7752.1 |########################################
   7791.2 |
   7830.2 |########################################
   7869.3 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2106365.8-2200519.8 ns)
  2106365.8 |####################
  2111073.5 |########################################
  2115781.2 |####################
  2120488.9 |
  2125196.6 |####################
  2129904.3 |
  2134612.0 |
  2139319.7 |
  2144027.4 |
  2148735.1 |
  2153442.8 |
  2158150.5 |
  2162858.2 |
  2167565.9 |
  2172273.6 |
  2176981.3 |
  2181689.0 |
  2186396.7 |
  2191104.4 |
  2195812.1 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 3400.0-3446.0 ns)
   3400.0 |########################################
   3402.3 |
   3404.6 |
   3406.9 |
   3409.2 |
   3411.5 |
   3413.8 |
   3416.1 |
   3418.4 |
   3420.7 |
   3423.0 |
   3425.3 |
   3427.6 |####################
   3429.9 |
   3432.2 |
   3434.5 |
   3436.8 |
   3439.1 |
   3441.4 |########################################
   3443.7 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2100631.2-2173306.6 ns)
  2100631.2 |########################################
  2104265.0 |####################
  2107898.7 |
  2111532.5 |
  2115166.3 |####################
  2118800.1 |
  2122433.8 |
  2126067.6 |
  2129701.4 |
  2133335.2 |
  2136968.9 |
  2140602.7 |
  2144236.5 |
  2147870.2 |
  2151504.0 |
  2155137.8 |
  2158771.6 |
  2162405.3 |####################
  2166039.1 |
  2169672.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=1536.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=3565.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=304.4% of algo (FFI overhead may distort results)

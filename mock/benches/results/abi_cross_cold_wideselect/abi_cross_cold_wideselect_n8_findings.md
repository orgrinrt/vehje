# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_null dominates: 13% faster than the next best (abi_cross_cold_wideselect_cold_null)

abi_cross_cold_wideselect_warm_null (3.10 us) leads abi_cross_cold_wideselect_cold_null (3.50 us) by 13%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_wideselect_cold_scalar is an outlier: 681.5x slower than the field

abi_cross_cold_wideselect_cold_scalar (2.11 ms) is 681.5x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_wideselect_warm_null shows alternating (throttle bounce) (autocorr -0.56)

abi_cross_cold_wideselect_warm_null's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (3.10 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} (60321% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_warm_scalar, abi_cross_cold_wideselect_cold_scalar} with a 60321% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 681.5x the fastest

Fastest abi_cross_cold_wideselect_warm_null (3.10 us) to slowest abi_cross_cold_wideselect_cold_scalar (2.11 ms): 681.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 3102.3 ns median
- 3 variants significantly slower than baseline
- Spread: 681.49x (fastest 3102.3 ns, slowest 2114193.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 6613ns | 5826ns | 5569ns | 5777ns | 8391ns | +15.85% |
| abi_cross_cold_wideselect_cold_scalar | 2155751ns | 2117921ns | 2097583ns | 2115294ns | 2245520ns | +37663.10% |
| abi_cross_cold_wideselect_warm_null | 5709ns | 5472ns | 5292ns | 5438ns | 6323ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2125925ns | 2117962ns | 2103236ns | 2113555ns | 2155823ns | +37140.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 3855ns | 3383ns | 4646ns | +22.89% | 0.002 |
| abi_cross_cold_wideselect_cold_scalar | 2151811ns | 2094003ns | 2241008ns | +68505.11% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 3137ns | 3042ns | 3261ns | base | 0.003 |
| abi_cross_cold_wideselect_warm_scalar | 2121813ns | 2099437ns | 2151511ns | +67548.72% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 40555.9 | 4321.0 | 3854.6 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 88712.9 | 2130573.7 | 2151810.6 | n/a |
| abi_cross_cold_wideselect_warm_null | 31065.4 | 3181.2 | 3136.5 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 84897.8 | 2133838.0 | 2121813.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.002 | 86.9% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_wideselect_warm_null | 0.003 | 98.0% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 6613ns | 6613ns | +15.85% |
| abi_cross_cold_wideselect_cold_scalar | 2155751ns | 2155751ns | +37663.10% |
| abi_cross_cold_wideselect_warm_null | 5709ns | 5709ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2125925ns | 2125925ns | +37140.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 3102ns | base | --- | [3046, 3261] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 3498ns | +393.3ns (+12.7%) | [+243, +1518]ns | [3419, 4646] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2114194ns | +2111115.0ns (+68050.0%) | [+2097160, +2237747]ns | [2100230, 2241008] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2113727ns | +2110651.8ns (+68035.1%) | [+2097129, +2148250]ns | [2100202, 2151511] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 3106ns | +14.0% | +68132.1% | +67546.6% |
| 2 | 3042ns | +11.2% | +69152.6% | +68921.8% |
| 3 | 3371ns | +4.3% | +64676.8% | +64489.6% |
| 4 | 3051ns | +14.1% | +69028.1% | +69346.5% |
| 5 | 3150ns | +82.6% | +72851.3% | +67370.1% |
| 6 | 3099ns | +11.5% | +67474.7% | +67942.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | -0.238 | moderate- |
| abi_cross_cold_wideselect_cold_scalar | -0.521 | HIGH- (thermal bounce) |
| abi_cross_cold_wideselect_warm_null | -0.556 | HIGH- (thermal bounce) |
| abi_cross_cold_wideselect_warm_scalar | -0.236 | moderate- |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 133293.1ns | 3854.6ns | 3458.0% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6497440.7ns | 2151810.6ns | 302.0% | HIGH |
| abi_cross_cold_wideselect_warm_null | 123377.4ns | 3136.5ns | 3933.6% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6468376.4ns | 2121813.5ns | 304.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 3383.3-4646.2 ns)
   3383.3 |####################
   3446.4 |########################################
   3509.6 |########################################
   3572.7 |
   3635.9 |
   3699.0 |
   3762.2 |
   3825.3 |
   3888.5 |
   3951.6 |
   4014.8 |
   4077.9 |
   4141.1 |
   4204.2 |
   4267.4 |
   4330.5 |
   4393.7 |
   4456.8 |
   4520.0 |
   4583.1 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2094003.3-2241008.2 ns)
  2094003.3 |########################################
  2101353.5 |########################################
  2108703.8 |########################################
  2116054.0 |########################################
  2123404.3 |
  2130754.5 |
  2138104.8 |
  2145455.0 |
  2152805.2 |
  2160155.5 |
  2167505.7 |
  2174856.0 |
  2182206.2 |########################################
  2189556.5 |
  2196906.7 |
  2204256.9 |
  2211607.2 |
  2218957.4 |
  2226307.7 |
  2233657.9 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 3041.7-3260.8 ns)
   3041.7 |########################################
   3052.7 |
   3063.6 |
   3074.6 |
   3085.5 |
   3096.5 |########################################
   3107.4 |
   3118.4 |
   3129.3 |
   3140.3 |####################
   3151.2 |
   3162.2 |
   3173.2 |
   3184.1 |
   3195.1 |
   3206.0 |
   3217.0 |
   3227.9 |
   3238.9 |
   3249.8 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2099437.1-2151511.0 ns)
  2099437.1 |########################################
  2102040.8 |
  2104644.5 |
  2107248.2 |####################
  2109851.9 |
  2112455.6 |
  2115059.3 |
  2117663.0 |####################
  2120266.7 |
  2122870.4 |
  2125474.1 |####################
  2128077.8 |
  2130681.5 |
  2133285.2 |
  2135888.9 |
  2138492.6 |
  2141096.3 |
  2143700.0 |
  2146303.7 |
  2148907.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: CV=22.1% (high variance, measurements may be unstable)
- **abi_cross_cold_wideselect_cold_null**: bridge=3629.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=3982.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=303.6% of algo (FFI overhead may distort results)

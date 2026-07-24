# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_null dominates: 146% faster than the next best (abi_cross_cold_leaf_cold_null)

abi_cross_cold_leaf_warm_null (4.53 us) leads abi_cross_cold_leaf_cold_null (11.16 us) by 146%, a clear separation rather than a photo finish. CV 15.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_leaf_cold_scalar is an outlier: 464.1x slower than the field

abi_cross_cold_leaf_cold_scalar (2.10 ms) is 464.1x the fastest (4.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_leaf_warm_null is fastest but the noisiest (CV 15.7%)

abi_cross_cold_leaf_warm_null wins on median (4.53 us) yet has the highest variance (CV 15.7%), while abi_cross_cold_leaf_cold_scalar is the steadiest (CV 4.6%, 2.10 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_cross_cold_leaf_warm_scalar shows alternating (throttle bounce) (autocorr -0.62)

abi_cross_cold_leaf_warm_scalar's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (4.53 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} (18237% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_warm_scalar, abi_cross_cold_leaf_cold_scalar} with a 18237% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 464.1x the fastest

Fastest abi_cross_cold_leaf_warm_null (4.53 us) to slowest abi_cross_cold_leaf_cold_scalar (2.10 ms): 464.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 4528.9 ns median
- 3 variants significantly slower than baseline
- Spread: 464.09x (fastest 4528.9 ns, slowest 2101820.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 14757ns | 14292ns | 12396ns | 14133ns | 16873ns | +84.91% |
| abi_cross_cold_leaf_cold_scalar | 2147604ns | 2108156ns | 2053735ns | 2094783ns | 2273769ns | +26809.86% |
| abi_cross_cold_leaf_warm_null | 7981ns | 7466ns | 7116ns | 7352ns | 9356ns | base |
| abi_cross_cold_leaf_warm_scalar | 2089138ns | 2052202ns | 1828458ns | 2002486ns | 2349457ns | +26077.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 11585ns | 9633ns | 13430ns | +140.16% | 0.000 |
| abi_cross_cold_leaf_cold_scalar | 2141198ns | 2047862ns | 2267166ns | +44288.65% | 0.000 |
| abi_cross_cold_leaf_warm_null | 4824ns | 4178ns | 5731ns | base | 0.000 |
| abi_cross_cold_leaf_warm_scalar | 2082301ns | 1823344ns | 2340019ns | +43067.68% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 54816.5 | 15488.9 | 11584.6 | n/a |
| abi_cross_cold_leaf_cold_scalar | 136406.3 | 2117147.4 | 2141197.7 | n/a |
| abi_cross_cold_leaf_warm_null | 45943.3 | 5008.8 | 4823.8 | n/a |
| abi_cross_cold_leaf_warm_scalar | 119326.3 | 2086049.5 | 2082300.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.000 | 37.4% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_leaf_warm_null | 0.000 | 92.3% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 14757ns | 14757ns | +84.91% |
| abi_cross_cold_leaf_cold_scalar | 2147604ns | 2147604ns | +26809.86% |
| abi_cross_cold_leaf_warm_null | 7981ns | 7981ns | base |
| abi_cross_cold_leaf_warm_scalar | 2089138ns | 2089138ns | +26077.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 4529ns | base | --- | [4212, 5731] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 11160ns | +6756.0ns (+149.2%) | [+5635, +7891]ns | [10164, 13430] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_cold_scalar | 2101821ns | +2096597.3ns (+46293.2%) | [+2049687, +2262838]ns | [2054606, 2267166] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 2046405ns | +2042025.8ns (+45088.3%) | [+1856117, +2334288]ns | [1860479, 2340019] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 5259ns | +132.2% | +38838.7% | +43311.5% |
| 2 | 4178ns | +175.7% | +53097.9% | +46289.6% |
| 3 | 6202ns | +136.1% | +33412.9% | +38547.2% |
| 4 | 4245ns | +154.5% | +49962.2% | +44602.3% |
| 5 | 4579ns | +133.6% | +44915.5% | +46950.0% |
| 6 | 4479ns | +115.1% | +51512.4% | +40611.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.014 | ok |
| abi_cross_cold_leaf_cold_scalar | -0.437 | moderate- |
| abi_cross_cold_leaf_warm_null | -0.577 | HIGH- (thermal bounce) |
| abi_cross_cold_leaf_warm_scalar | -0.617 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 157279.9ns | 11584.6ns | 1357.7% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 6484601.9ns | 2141197.7ns | 302.8% | HIGH |
| abi_cross_cold_leaf_warm_null | 146677.7ns | 4823.8ns | 3040.7% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 6356359.8ns | 2082300.8ns | 305.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 9632.9-13429.5 ns)
   9632.9 |########################################
   9822.7 |
  10012.6 |
  10202.4 |
  10392.2 |
  10582.1 |########################################
  10771.9 |########################################
  10961.7 |
  11151.6 |
  11341.4 |########################################
  11531.2 |
  11721.1 |
  11910.9 |
  12100.7 |########################################
  12290.6 |
  12480.4 |
  12670.2 |
  12860.1 |
  13049.9 |
  13239.7 |
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 2047862.5-2267166.2 ns)
  2047862.5 |########################################
  2058827.7 |########################################
  2069792.9 |########################################
  2080758.1 |
  2091723.2 |
  2102688.4 |
  2113653.6 |
  2124618.8 |########################################
  2135584.0 |
  2146549.2 |
  2157514.4 |
  2168479.6 |
  2179444.8 |
  2190409.9 |
  2201375.1 |
  2212340.3 |########################################
  2223305.5 |
  2234270.7 |
  2245235.9 |
  2256201.1 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 4178.3-5730.6 ns)
   4178.3 |########################################
   4255.9 |
   4333.5 |
   4411.2 |####################
   4488.8 |
   4566.4 |####################
   4644.0 |
   4721.6 |
   4799.2 |
   4876.9 |
   4954.5 |
   5032.1 |
   5109.7 |
   5187.3 |####################
   5264.9 |
   5342.6 |
   5420.2 |
   5497.8 |
   5575.4 |
   5653.0 |
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1823343.8-2340018.8 ns)
  1823343.8 |########################################
  1849177.5 |
  1875011.3 |########################################
  1900845.0 |
  1926678.8 |########################################
  1952512.5 |
  1978346.3 |
  2004180.0 |
  2030013.8 |
  2055847.5 |
  2081681.3 |
  2107515.0 |
  2133348.8 |########################################
  2159182.5 |
  2185016.3 |
  2210850.0 |
  2236683.8 |
  2262517.5 |########################################
  2288351.3 |
  2314185.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=1398.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=2969.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=310.0% of algo (FFI overhead may distort results)

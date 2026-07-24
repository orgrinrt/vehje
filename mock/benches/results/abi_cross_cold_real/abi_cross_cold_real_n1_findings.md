# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 511% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (5.43 us) leads abi_cross_cold_real_cold_null (33.18 us) by 511%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_cold_scalar is an outlier: 408.0x slower than the field

abi_cross_cold_real_cold_scalar (2.22 ms) is 408.0x the fastest (5.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (5.43 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (6488% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 6488% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 408.0x the fastest

Fastest abi_cross_cold_real_warm_null (5.43 us) to slowest abi_cross_cold_real_cold_scalar (2.22 ms): 408.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 5433.1 ns median
- 3 variants significantly slower than baseline
- Spread: 408.03x (fastest 5433.1 ns, slowest 2216865.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 36036ns | 35445ns | 34767ns | 35386ns | 37646ns | +366.13% |
| abi_cross_cold_real_cold_scalar | 2219517ns | 2220208ns | 2212440ns | 2218511ns | 2224562ns | +28609.25% |
| abi_cross_cold_real_warm_null | 7731ns | 7716ns | 7610ns | 7686ns | 7858ns | base |
| abi_cross_cold_real_warm_scalar | 2190614ns | 2189459ns | 2184219ns | 2188951ns | 2196306ns | +28235.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33730ns | 32440ns | 35311ns | +521.85% | 0.000 |
| abi_cross_cold_real_cold_scalar | 2215973ns | 2208948ns | 2220708ns | +40754.34% | 0.000 |
| abi_cross_cold_real_warm_null | 5424ns | 5345ns | 5475ns | base | 0.000 |
| abi_cross_cold_real_warm_scalar | 2187127ns | 2180891ns | 2192639ns | +40222.51% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 34670.4 | 37766.3 | 33729.7 | n/a |
| abi_cross_cold_real_cold_scalar | 76924.5 | 2214296.3 | 2215973.4 | n/a |
| abi_cross_cold_real_warm_null | 28926.7 | 5468.6 | 5424.1 | n/a |
| abi_cross_cold_real_warm_scalar | 71963.7 | 2187985.1 | 2187126.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.000 | 16.1% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_real_warm_null | 0.000 | 98.4% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 36036ns | 36036ns | +366.13% |
| abi_cross_cold_real_cold_scalar | 2219517ns | 2219517ns | +28609.25% |
| abi_cross_cold_real_warm_null | 7731ns | 7731ns | base |
| abi_cross_cold_real_warm_scalar | 2190614ns | 2190614ns | +28235.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 5433ns | base | --- | [5365, 5475] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 33184ns | +27806.0ns (+511.8%) | [+27223, +29888]ns | [32693, 35311] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2216865ns | +2211461.2ns (+40703.5%) | [+2204877, +2215310]ns | [2210348, 2220708] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2186030ns | +2180665.2ns (+40136.7%) | [+2177274, +2187169]ns | [2182711, 2192639] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 5411ns | +512.0% | +40918.3% | +40270.5% |
| 2 | 5486ns | +491.3% | +40214.7% | +39771.0% |
| 3 | 5463ns | +539.1% | +40448.5% | +39821.9% |
| 4 | 5345ns | +522.1% | +41408.1% | +40817.1% |
| 5 | 5384ns | +563.2% | +41165.8% | +40482.4% |
| 6 | 5455ns | +504.0% | +40394.0% | +40191.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.463 | moderate- |
| abi_cross_cold_real_cold_scalar | -0.324 | moderate- |
| abi_cross_cold_real_warm_null | 0.032 | ok |
| abi_cross_cold_real_warm_scalar | -0.145 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 215784.9ns | 33729.7ns | 639.7% | HIGH |
| abi_cross_cold_real_cold_scalar | 6725894.8ns | 2215973.4ns | 303.5% | HIGH |
| abi_cross_cold_real_warm_null | 127457.3ns | 5424.1ns | 2349.8% | HIGH |
| abi_cross_cold_real_warm_scalar | 6644677.5ns | 2187126.6ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 32439.6-35311.4 ns)
  32439.6 |########################################
  32583.2 |
  32726.8 |
  32870.4 |########################################
  33014.0 |########################################
  33157.6 |########################################
  33301.2 |
  33444.7 |
  33588.3 |
  33731.9 |
  33875.5 |
  34019.1 |
  34162.7 |
  34306.3 |
  34449.9 |
  34593.5 |
  34737.1 |
  34880.7 |########################################
  35024.3 |
  35167.9 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2208947.9-2220707.5 ns)
  2208947.9 |########################################
  2209535.9 |
  2210123.9 |
  2210711.8 |
  2211299.8 |########################################
  2211887.8 |
  2212475.8 |
  2213063.8 |
  2213651.7 |
  2214239.7 |
  2214827.7 |########################################
  2215415.7 |
  2216003.7 |
  2216591.6 |
  2217179.6 |
  2217767.6 |
  2218355.6 |########################################
  2218943.6 |
  2219531.5 |########################################
  2220119.5 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 5345.0-5474.5 ns)
   5345.0 |########################################
   5351.5 |
   5358.0 |
   5364.4 |
   5370.9 |
   5377.4 |
   5383.9 |########################################
   5390.3 |
   5396.8 |
   5403.3 |
   5409.8 |########################################
   5416.3 |
   5422.7 |
   5429.2 |
   5435.7 |
   5442.2 |
   5448.6 |########################################
   5455.1 |
   5461.6 |########################################
   5468.1 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2180891.2-2192639.1 ns)
  2180891.2 |########################################
  2181478.6 |
  2182066.0 |
  2182653.4 |
  2183240.8 |
  2183828.2 |
  2184415.6 |########################################
  2185003.0 |########################################
  2185590.4 |
  2186177.8 |
  2186765.2 |########################################
  2187352.6 |########################################
  2187940.0 |
  2188527.4 |
  2189114.8 |
  2189702.2 |
  2190289.6 |
  2190877.0 |
  2191464.4 |
  2192051.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=649.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=2340.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=304.1% of algo (FFI overhead may distort results)

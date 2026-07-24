# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_null dominates: 30% faster than the next best (abi_cross_cold_scatter_cold_null)

abi_cross_cold_scatter_warm_null (4.00 us) leads abi_cross_cold_scatter_cold_null (5.19 us) by 30%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_scatter_cold_scalar is an outlier: 549.1x slower than the field

abi_cross_cold_scatter_cold_scalar (2.19 ms) is 549.1x the fastest (4.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_scatter_warm_null is fastest but the noisiest (CV 6.4%)

abi_cross_cold_scatter_warm_null wins on median (4.00 us) yet has the highest variance (CV 6.4%), while abi_cross_cold_scatter_cold_scalar is the steadiest (CV 1.4%, 2.19 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (4.00 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} (41920% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} with a 41920% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 549.1x the fastest

Fastest abi_cross_cold_scatter_warm_null (4.00 us) to slowest abi_cross_cold_scatter_cold_scalar (2.19 ms): 549.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 3995.0 ns median
- 3 variants significantly slower than baseline
- Spread: 549.12x (fastest 3995.0 ns, slowest 2193752.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 7519ns | 7570ns | 7242ns | 7548ns | 7615ns | +16.09% |
| abi_cross_cold_scatter_cold_scalar | 2211171ns | 2197303ns | 2183778ns | 2193447ns | 2251452ns | +34037.32% |
| abi_cross_cold_scatter_warm_null | 6477ns | 6299ns | 6096ns | 6282ns | 6961ns | base |
| abi_cross_cold_scatter_warm_scalar | 2222669ns | 2186194ns | 2181173ns | 2185773ns | 2298760ns | +34214.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5192ns | 5056ns | 5298ns | +26.79% | 0.001 |
| abi_cross_cold_scatter_cold_scalar | 2207464ns | 2180358ns | 2247354ns | +53805.44% | 0.000 |
| abi_cross_cold_scatter_warm_null | 4095ns | 3916ns | 4348ns | base | 0.001 |
| abi_cross_cold_scatter_warm_scalar | 2219007ns | 2177798ns | 2294687ns | +54087.33% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 33498.4 | 7016.5 | 5192.1 | n/a |
| abi_cross_cold_scatter_cold_scalar | 84603.8 | 2200957.2 | 2207463.8 | n/a |
| abi_cross_cold_scatter_warm_null | 29622.2 | 4282.1 | 4095.1 | n/a |
| abi_cross_cold_scatter_warm_scalar | 73773.8 | 2210782.0 | 2219007.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.001 | 75.4% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_scatter_warm_null | 0.001 | 98.0% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 7519ns | 7519ns | +16.09% |
| abi_cross_cold_scatter_cold_scalar | 2211171ns | 2211171ns | +34037.32% |
| abi_cross_cold_scatter_warm_null | 6477ns | 6477ns | base |
| abi_cross_cold_scatter_warm_scalar | 2222669ns | 2222669ns | +34214.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 3995ns | base | --- | [3943, 4348] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 5194ns | +1199.4ns (+30.0%) | [+737, +1355]ns | [5084, 5298] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2193752ns | +2189787.5ns (+54813.2%) | [+2177281, +2243037]ns | [2181285, 2247354] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2182690ns | +2178695.4ns (+54535.6%) | [+2175671, +2290370]ns | [2179644, 2294687] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 4663ns | +9.6% | +48600.7% | +47896.7% |
| 2 | 3970ns | +34.7% | +55916.9% | +59128.7% |
| 3 | 3976ns | +30.8% | +54740.7% | +54780.9% |
| 4 | 4032ns | +25.4% | +54026.3% | +54008.5% |
| 5 | 4014ns | +29.3% | +54591.7% | +54292.6% |
| 6 | 3916ns | +34.0% | +55880.1% | +55515.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | -0.230 | moderate- |
| abi_cross_cold_scatter_cold_scalar | 0.291 | moderate+ |
| abi_cross_cold_scatter_warm_null | -0.074 | ok |
| abi_cross_cold_scatter_warm_scalar | 0.078 | ok |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 117733.9ns | 5192.1ns | 2267.6% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6693215.3ns | 2207463.8ns | 303.2% | HIGH |
| abi_cross_cold_scatter_warm_null | 124330.8ns | 4095.1ns | 3036.1% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6731931.8ns | 2219007.2ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 5055.8-5297.7 ns)
   5055.8 |####################
   5067.9 |
   5080.0 |
   5092.1 |
   5104.2 |####################
   5116.3 |
   5128.4 |
   5140.5 |
   5152.6 |
   5164.7 |
   5176.8 |
   5188.8 |########################################
   5200.9 |
   5213.0 |
   5225.1 |
   5237.2 |####################
   5249.3 |
   5261.4 |
   5273.5 |
   5285.6 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2180358.3-2247353.8 ns)
  2180358.3 |########################################
  2183708.1 |
  2187057.8 |
  2190407.6 |####################
  2193757.4 |####################
  2197107.2 |
  2200456.9 |
  2203806.7 |
  2207156.5 |
  2210506.3 |
  2213856.0 |
  2217205.8 |
  2220555.6 |####################
  2223905.3 |
  2227255.1 |
  2230604.9 |
  2233954.7 |
  2237304.4 |
  2240654.2 |
  2244004.0 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 3915.8-4347.5 ns)
   3915.8 |####################
   3937.4 |
   3959.0 |########################################
   3980.6 |
   4002.1 |####################
   4023.7 |####################
   4045.3 |
   4066.9 |
   4088.5 |
   4110.1 |
   4131.6 |
   4153.2 |
   4174.8 |
   4196.4 |
   4218.0 |
   4239.6 |
   4261.2 |
   4282.7 |
   4304.3 |
   4325.9 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2177797.5-2294686.8 ns)
  2177797.5 |########################################
  2183642.0 |
  2189486.4 |
  2195330.9 |
  2201175.4 |
  2207019.8 |
  2212864.3 |
  2218708.8 |
  2224553.2 |
  2230397.7 |
  2236242.2 |##########
  2242086.6 |
  2247931.1 |
  2253775.6 |
  2259620.0 |
  2265464.5 |
  2271309.0 |
  2277153.4 |
  2282997.9 |
  2288842.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=2271.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=3046.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=304.6% of algo (FFI overhead may distort results)

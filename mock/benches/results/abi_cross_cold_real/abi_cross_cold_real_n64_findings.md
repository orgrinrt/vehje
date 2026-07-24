# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_scalar is an outlier: 940.3x slower than the field

abi_cross_cold_real_warm_scalar (2.20 ms) is 940.3x the fastest (2.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (88379% apart)

The field splits into a fast tier {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 88379% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 940.3x the fastest

Fastest abi_cross_cold_real_cold_null (2.33 us) to slowest abi_cross_cold_real_warm_scalar (2.20 ms): 940.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_real_cold_null** at 2334.8 ns median (-5.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 940.28x (fastest 2334.8 ns, slowest 2195370.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4677ns | 4690ns | 4450ns | 4679ns | 4786ns | -2.23% |
| abi_cross_cold_real_cold_scalar | 2200162ns | 2189025ns | 2179945ns | 2186776ns | 2230350ns | +45895.62% |
| abi_cross_cold_real_warm_null | 4783ns | 4786ns | 4692ns | 4757ns | 4870ns | base |
| abi_cross_cold_real_warm_scalar | 2201161ns | 2199004ns | 2183482ns | 2194930ns | 2219345ns | +45916.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2348ns | 2268ns | 2410ns | -5.39% | 0.027 |
| abi_cross_cold_real_cold_scalar | 2196558ns | 2176779ns | 2226342ns | +88406.65% | 0.000 |
| abi_cross_cold_real_warm_null | 2482ns | 2429ns | 2537ns | base | 0.026 |
| abi_cross_cold_real_warm_scalar | 2197466ns | 2179952ns | 2215424ns | +88443.22% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 33803.7 | 2474.1 | 2348.0 | 30 |
| abi_cross_cold_real_cold_scalar | 78871.1 | 2195657.1 | 2196558.1 | n/a |
| abi_cross_cold_real_warm_null | 29773.4 | 2719.6 | 2481.8 | n/a |
| abi_cross_cold_real_warm_scalar | 75076.0 | 2198780.6 | 2197465.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.028 Gops/s** (abi_cross_cold_real_cold_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.027 | 97.1% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.026 | 91.8% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 4677ns | 4677ns | -2.23% |
| abi_cross_cold_real_cold_scalar | 2200162ns | 2200162ns | +45895.62% |
| abi_cross_cold_real_warm_null | 4783ns | 4783ns | base |
| abi_cross_cold_real_warm_scalar | 2201161ns | 2201161ns | +45916.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2470ns | base | --- | [2438, 2537] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2335ns | -126.7ns (-5.1%) | [-215, -60]ns | [2299, 2410] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2185615ns | +2183130.2ns (+88378.7%) | [+2175269, +2223829]ns | [2177717, 2226342] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2195371ns | +2192919.1ns (+88775.0%) | [+2179121, +2212911]ns | [2181602, 2215424] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2578ns | -7.8% | +86276.3% | +86144.1% |
| 2 | 2447ns | -4.8% | +90865.1% | +90112.0% |
| 3 | 2467ns | -5.6% | +88146.6% | +88275.2% |
| 4 | 2429ns | -3.7% | +89586.1% | +89847.7% |
| 5 | 2496ns | -9.1% | +87567.4% | +87363.1% |
| 6 | 2474ns | -1.2% | +88143.4% | +89067.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.408 | moderate- |
| abi_cross_cold_real_cold_scalar | 0.337 | moderate+ |
| abi_cross_cold_real_warm_null | -0.213 | moderate- |
| abi_cross_cold_real_warm_scalar | 0.239 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 6/6, lost 0/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 122035.3ns | 2348.0ns | 5197.4% | HIGH |
| abi_cross_cold_real_cold_scalar | 6671698.1ns | 2196558.1ns | 303.7% | HIGH |
| abi_cross_cold_real_warm_null | 114860.6ns | 2481.8ns | 4628.1% | HIGH |
| abi_cross_cold_real_warm_scalar | 6672698.5ns | 2197465.6ns | 303.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2267.9-2410.4 ns)
   2267.9 |####################
   2275.0 |
   2282.2 |
   2289.3 |
   2296.4 |
   2303.5 |
   2310.7 |
   2317.8 |
   2324.9 |########################################
   2332.0 |
   2339.2 |####################
   2346.3 |
   2353.4 |
   2360.6 |
   2367.7 |
   2374.8 |####################
   2381.9 |
   2389.1 |
   2396.2 |
   2403.3 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2176778.8-2226341.9 ns)
  2176778.8 |########################################
  2179257.0 |
  2181735.1 |####################
  2184213.3 |
  2186691.4 |####################
  2189169.6 |
  2191647.7 |
  2194125.9 |
  2196604.0 |
  2199082.2 |
  2201560.4 |
  2204038.5 |
  2206516.7 |
  2208994.8 |
  2211473.0 |
  2213951.1 |
  2216429.3 |
  2218907.4 |
  2221385.6 |
  2223863.7 |####################
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2429.2-2537.2 ns)
   2429.2 |########################################
   2434.6 |
   2440.0 |
   2445.4 |########################################
   2450.8 |
   2456.2 |
   2461.6 |########################################
   2467.0 |
   2472.4 |########################################
   2477.8 |
   2483.2 |
   2488.6 |
   2494.0 |########################################
   2499.4 |
   2504.8 |
   2510.2 |
   2515.6 |
   2521.0 |
   2526.4 |
   2531.8 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2179951.7-2215423.8 ns)
  2179951.7 |########################################
  2181725.3 |########################################
  2183498.9 |########################################
  2185272.5 |
  2187046.1 |
  2188819.7 |
  2190593.3 |
  2192366.9 |
  2194140.5 |
  2195914.1 |
  2197687.7 |
  2199461.3 |
  2201234.9 |
  2203008.5 |
  2204782.1 |########################################
  2206555.7 |########################################
  2208329.3 |
  2210102.9 |
  2211876.5 |
  2213650.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=5216.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=4615.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.0% of algo (FFI overhead may distort results)

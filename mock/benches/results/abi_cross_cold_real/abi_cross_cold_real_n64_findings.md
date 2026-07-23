# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_scalar is an outlier: 910.8x slower than the field

abi_cross_cold_real_warm_scalar (2.16 ms) is 910.8x the fastest (2.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (85541% apart)

The field splits into a fast tier {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 85541% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 910.8x the fastest

Fastest abi_cross_cold_real_cold_null (2.37 us) to slowest abi_cross_cold_real_warm_scalar (2.16 ms): 910.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_real_cold_null** at 2368.6 ns median (-5.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 910.83x (fastest 2368.6 ns, slowest 2157357.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4661ns | 4651ns | 4500ns | 4610ns | 4817ns | -10.71% |
| abi_cross_cold_real_cold_scalar | 2297585ns | 2156149ns | 2143402ns | 2153568ns | 2590700ns | +43919.81% |
| abi_cross_cold_real_warm_null | 5219ns | 4819ns | 4571ns | 4740ns | 6263ns | base |
| abi_cross_cold_real_warm_scalar | 2266801ns | 2160344ns | 2152760ns | 2158296ns | 2486578ns | +43330.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2366ns | 2254ns | 2437ns | -14.80% | 0.027 |
| abi_cross_cold_real_cold_scalar | 2294492ns | 2140685ns | 2587189ns | +82525.82% | 0.000 |
| abi_cross_cold_real_warm_null | 2777ns | 2394ns | 3413ns | base | 0.023 |
| abi_cross_cold_real_warm_scalar | 2263788ns | 2149772ns | 2483461ns | +81420.17% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 32293.9 | 2549.0 | 2366.0 | 3 |
| abi_cross_cold_real_cold_scalar | 65399.0 | 2242010.9 | 2294491.5 | n/a |
| abi_cross_cold_real_warm_null | 30995.5 | 2955.0 | 2777.0 | n/a |
| abi_cross_cold_real_warm_scalar | 56832.4 | 2167244.1 | 2263787.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.028 Gops/s** (abi_cross_cold_real_cold_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.027 | 95.2% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.025 | 89.6% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 4661ns | 4661ns | -10.71% |
| abi_cross_cold_real_cold_scalar | 2297585ns | 2297585ns | +43919.81% |
| abi_cross_cold_real_warm_null | 5219ns | 5219ns | base |
| abi_cross_cold_real_warm_scalar | 2266801ns | 2266801ns | +43330.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2514ns | base | --- | [2404, 3413] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2369ns | -145.4ns (-5.8%) | [-1048, -40]ns | [2292, 2437] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2153146ns | +2150671.5ns (+85542.7%) | [+2140608, +2583864]ns | [2143140, 2587189] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2157358ns | +2154797.0ns (+85706.8%) | [+2148060, +2480176]ns | [2150545, 2483461] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2394ns | -5.8% | +89870.1% | +90127.2% |
| 2 | 4177ns | -42.6% | +72115.6% | +67107.7% |
| 3 | 2472ns | -0.1% | +87186.1% | +87208.5% |
| 4 | 2556ns | -5.9% | +84123.7% | +84073.9% |
| 5 | 2649ns | -12.0% | +80902.5% | +81295.9% |
| 6 | 2414ns | -3.2% | +88570.6% | +88947.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.120 | ok |
| abi_cross_cold_real_cold_scalar | -0.219 | moderate- |
| abi_cross_cold_real_warm_null | -0.342 | moderate- |
| abi_cross_cold_real_warm_scalar | -0.218 | moderate- |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 6/6, lost 0/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 119569.5ns | 2366.0ns | 5053.7% | HIGH |
| abi_cross_cold_real_cold_scalar | 6881063.5ns | 2294491.5ns | 299.9% | HIGH |
| abi_cross_cold_real_warm_null | 117898.2ns | 2777.0ns | 4245.6% | HIGH |
| abi_cross_cold_real_warm_scalar | 6559868.5ns | 2263787.9ns | 289.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2253.8-2437.1 ns)
   2253.8 |########################################
   2263.0 |
   2272.1 |
   2281.3 |
   2290.5 |
   2299.6 |
   2308.8 |
   2318.0 |
   2327.1 |########################################
   2336.3 |########################################
   2345.4 |
   2354.6 |
   2363.8 |
   2372.9 |
   2382.1 |
   2391.3 |########################################
   2400.4 |########################################
   2409.6 |
   2418.8 |
   2427.9 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2140685.0-2587188.8 ns)
  2140685.0 |########################################
  2163010.2 |
  2185335.4 |
  2207660.6 |
  2229985.8 |
  2252310.9 |
  2274636.1 |
  2296961.3 |
  2319286.5 |
  2341611.7 |
  2363936.9 |
  2386262.1 |
  2408587.2 |
  2430912.4 |
  2453237.6 |
  2475562.8 |
  2497888.0 |
  2520213.2 |
  2542538.4 |
  2564863.6 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2393.8-3412.8 ns)
   2393.8 |########################################
   2444.7 |####################
   2495.7 |
   2546.6 |####################
   2597.6 |
   2648.5 |####################
   2699.5 |
   2750.4 |
   2801.4 |
   2852.3 |
   2903.3 |
   2954.2 |
   3005.2 |
   3056.1 |
   3107.1 |
   3158.0 |
   3209.0 |
   3259.9 |
   3310.9 |
   3361.8 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2149772.5-2483460.9 ns)
  2149772.5 |########################################
  2166456.9 |
  2183141.3 |
  2199825.8 |
  2216510.2 |
  2233194.6 |
  2249879.0 |
  2266563.4 |
  2283247.8 |
  2299932.3 |
  2316616.7 |
  2333301.1 |
  2349985.5 |
  2366669.9 |
  2383354.3 |
  2400038.8 |
  2416723.2 |
  2433407.6 |
  2450092.0 |
  2466776.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=5043.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: CV=22.8% (high variance, measurements may be unstable)
- **abi_cross_cold_real_warm_null**: bridge=4520.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=302.7% of algo (FFI overhead may distort results)

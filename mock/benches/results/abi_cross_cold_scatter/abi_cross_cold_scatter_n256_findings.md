# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_scalar is an outlier: 699.3x slower than the field

abi_cross_cold_scatter_warm_scalar (2.24 ms) is 699.3x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_scatter_cold_null, abi_cross_cold_scatter_warm_null} vs {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} (68781% apart)

The field splits into a fast tier {abi_cross_cold_scatter_cold_null, abi_cross_cold_scatter_warm_null} and a slow tier {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} with a 68781% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 699.3x the fastest

Fastest abi_cross_cold_scatter_cold_null (3.20 us) to slowest abi_cross_cold_scatter_warm_scalar (2.24 ms): 699.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_scatter_cold_null's edge over baseline is significant but tiny (-46 ns, 1.41%)

abi_cross_cold_scatter_cold_null differs from baseline abi_cross_cold_scatter_warm_null by -46 ns (1.41%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_scatter_cold_null** at 3196.1 ns median (-1.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 699.33x (fastest 3196.1 ns, slowest 2235104.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5465ns | 5463ns | 5336ns | 5433ns | 5577ns | -1.93% |
| abi_cross_cold_scatter_cold_scalar | 2231052ns | 2230274ns | 2217108ns | 2228596ns | 2241706ns | +39936.34% |
| abi_cross_cold_scatter_warm_null | 5573ns | 5579ns | 5502ns | 5558ns | 5630ns | base |
| abi_cross_cold_scatter_warm_scalar | 2244705ns | 2239269ns | 2230228ns | 2237887ns | 2262169ns | +40181.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 3193ns | 3130ns | 3251ns | -1.20% | 0.080 |
| abi_cross_cold_scatter_cold_scalar | 2226898ns | 2212904ns | 2237551ns | +68793.72% | 0.000 |
| abi_cross_cold_scatter_warm_null | 3232ns | 3170ns | 3285ns | base | 0.079 |
| abi_cross_cold_scatter_warm_scalar | 2240663ns | 2226367ns | 2258275ns | +69219.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 32063.8 | 3235.3 | 3193.5 | 66 |
| abi_cross_cold_scatter_cold_scalar | 95763.1 | 2228862.1 | 2226897.7 | n/a |
| abi_cross_cold_scatter_warm_null | 29499.1 | 3241.0 | 3232.4 | n/a |
| abi_cross_cold_scatter_warm_scalar | 87331.7 | 2228841.0 | 2240663.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_cross_cold_scatter_cold_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.080 | 97.9% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_scatter_warm_null | 0.079 | 96.9% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5465ns | 5465ns | -1.93% |
| abi_cross_cold_scatter_cold_scalar | 2231052ns | 2231052ns | +39936.34% |
| abi_cross_cold_scatter_warm_null | 5573ns | 5573ns | base |
| abi_cross_cold_scatter_warm_scalar | 2244705ns | 2244705ns | +40181.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 3232ns | base | --- | [3181, 3285] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 3196ns | no significant difference | [-88, +17]ns | [3134, 3251] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2226001ns | +2222775.6ns (+68781.4%) | [+2213871, +2234349]ns | [2217142, 2237551] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2235104ns | +2231875.0ns (+69063.0%) | [+2225388, +2255029]ns | [2228611, 2258275] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 3280ns | -0.9% | +67789.4% | +68182.0% |
| 2 | 3289ns | -3.1% | +67443.8% | +67982.1% |
| 3 | 3211ns | -2.3% | +69469.1% | +70805.3% |
| 4 | 3252ns | -0.1% | +67937.0% | +68488.9% |
| 5 | 3170ns | +1.1% | +70096.8% | +70291.3% |
| 6 | 3192ns | -1.9% | +70116.3% | +69646.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | -0.236 | moderate- |
| abi_cross_cold_scatter_cold_scalar | -0.278 | moderate- |
| abi_cross_cold_scatter_warm_null | 0.195 | ok |
| abi_cross_cold_scatter_warm_scalar | -0.106 | ok |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 4/6, lost 1/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 125279.8ns | 3193.5ns | 3923.0% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6783007.9ns | 2226897.7ns | 304.6% | HIGH |
| abi_cross_cold_scatter_warm_null | 122953.3ns | 3232.4ns | 3803.8% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6785328.4ns | 2240663.2ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 3130.0-3250.8 ns)
   3130.0 |########################################
   3136.0 |########################################
   3142.1 |
   3148.1 |
   3154.2 |
   3160.2 |
   3166.2 |
   3172.3 |
   3178.3 |
   3184.4 |########################################
   3190.4 |
   3196.4 |
   3202.5 |########################################
   3208.5 |
   3214.6 |
   3220.6 |
   3226.6 |
   3232.7 |
   3238.7 |
   3244.8 |########################################
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2212903.8-2237550.6 ns)
  2212903.8 |########################################
  2214136.1 |
  2215368.5 |
  2216600.8 |
  2217833.2 |
  2219065.5 |
  2220297.8 |########################################
  2221530.2 |
  2222762.5 |
  2223994.9 |########################################
  2225227.2 |
  2226459.5 |########################################
  2227691.9 |
  2228924.2 |
  2230156.6 |
  2231388.9 |
  2232621.2 |########################################
  2233853.6 |
  2235085.9 |
  2236318.3 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 3169.6-3284.6 ns)
   3169.6 |########################################
   3175.3 |
   3181.1 |
   3186.8 |########################################
   3192.6 |
   3198.3 |
   3204.1 |
   3209.8 |########################################
   3215.6 |
   3221.3 |
   3227.1 |
   3232.9 |
   3238.6 |
   3244.4 |
   3250.1 |########################################
   3255.9 |
   3261.6 |
   3267.4 |
   3273.1 |
   3278.9 |########################################
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2226366.7-2258274.8 ns)
  2226366.7 |####################
  2227962.1 |
  2229557.5 |########################################
  2231152.9 |
  2232748.3 |
  2234343.7 |
  2235939.1 |
  2237534.5 |####################
  2239129.9 |####################
  2240725.3 |
  2242320.7 |
  2243916.1 |
  2245511.5 |
  2247106.9 |
  2248702.3 |
  2250297.7 |
  2251893.1 |
  2253488.5 |
  2255083.9 |
  2256679.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=3926.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=3794.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=303.4% of algo (FFI overhead may distort results)

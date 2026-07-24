# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_scalar is an outlier: 946.1x slower than the field

abi_cross_cold_wideselect_warm_scalar (2.15 ms) is 946.1x the fastest (2.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_wideselect_warm_null)

The baseline abi_cross_cold_wideselect_warm_null is the fastest (2.27 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} vs {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} (86526% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_warm_null, abi_cross_cold_wideselect_cold_null} and a slow tier {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} with a 86526% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 946.1x the fastest

Fastest abi_cross_cold_wideselect_warm_null (2.27 us) to slowest abi_cross_cold_wideselect_warm_scalar (2.15 ms): 946.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_wideselect_warm_null) is the fastest** at 2272.1 ns median
- 3 variants significantly slower than baseline
- Spread: 946.13x (fastest 2272.1 ns, slowest 2149700.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 4791ns | 4774ns | 4644ns | 4750ns | 4925ns | +5.28% |
| abi_cross_cold_wideselect_cold_scalar | 2148811ns | 2101429ns | 2077639ns | 2100030ns | 2257569ns | +47122.47% |
| abi_cross_cold_wideselect_warm_null | 4550ns | 4555ns | 4416ns | 4522ns | 4661ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2171603ns | 2153406ns | 2079868ns | 2129488ns | 2280643ns | +47623.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 2433ns | 2352ns | 2509ns | +6.80% | 0.013 |
| abi_cross_cold_wideselect_cold_scalar | 2145109ns | 2074402ns | 2253329ns | +94047.05% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 2278ns | 2213ns | 2331ns | base | 0.014 |
| abi_cross_cold_wideselect_warm_scalar | 2167740ns | 2076570ns | 2276015ns | +95040.30% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 36352.8 | 2574.4 | 2433.3 | n/a |
| abi_cross_cold_wideselect_cold_scalar | 77274.5 | 2130448.1 | 2145109.1 | n/a |
| abi_cross_cold_wideselect_warm_null | 29093.5 | 2397.6 | 2278.5 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 76904.6 | 2188557.2 | 2167740.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_wideselect_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.013 | 91.4% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_wideselect_warm_null | 0.014 | 97.4% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 4791ns | 4791ns | +5.28% |
| abi_cross_cold_wideselect_cold_scalar | 2148811ns | 2148811ns | +47122.47% |
| abi_cross_cold_wideselect_warm_null | 4550ns | 4550ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2171603ns | 2171603ns | +47623.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 2272ns | base | --- | [2232, 2331] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 2422ns | +133.8ns (+5.9%) | [+75, +256]ns | [2369, 2509] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2097958ns | +2095706.5ns (+92236.5%) | [+2081787, +2250998]ns | [2084039, 2253329] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2149700ns | +2147447.9ns (+94513.8%) | [+2075253, +2273684]ns | [2077505, 2276015] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 2292ns | +9.9% | +104612.1% | +96462.5% |
| 2 | 2370ns | +2.2% | +88784.1% | +98580.2% |
| 3 | 2213ns | +12.9% | +94598.3% | +93806.8% |
| 4 | 2251ns | +6.0% | +92046.5% | +97673.8% |
| 5 | 2254ns | +4.4% | +92795.4% | +93001.3% |
| 6 | 2290ns | +5.8% | +91585.3% | +90564.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | -0.008 | ok |
| abi_cross_cold_wideselect_cold_scalar | 0.019 | ok |
| abi_cross_cold_wideselect_warm_null | -0.179 | ok |
| abi_cross_cold_wideselect_warm_scalar | -0.120 | ok |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 125863.5ns | 2433.3ns | 5172.5% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6506791.9ns | 2145109.1ns | 303.3% | HIGH |
| abi_cross_cold_wideselect_warm_null | 117214.7ns | 2278.5ns | 5144.5% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6664725.6ns | 2167740.1ns | 307.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 2352.1-2508.8 ns)
   2352.1 |####################
   2359.9 |
   2367.8 |
   2375.6 |
   2383.4 |####################
   2391.3 |
   2399.1 |
   2406.9 |
   2414.8 |########################################
   2422.6 |
   2430.4 |
   2438.3 |
   2446.1 |
   2453.9 |
   2461.8 |
   2469.6 |
   2477.4 |
   2485.3 |
   2493.1 |####################
   2500.9 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2074401.7-2253329.4 ns)
  2074401.7 |#############
  2083348.1 |
  2092294.5 |########################################
  2101240.9 |#############
  2110187.2 |
  2119133.6 |
  2128080.0 |
  2137026.4 |
  2145972.8 |
  2154919.2 |
  2163865.5 |
  2172811.9 |
  2181758.3 |
  2190704.7 |
  2199651.1 |
  2208597.5 |
  2217543.9 |
  2226490.2 |
  2235436.6 |
  2244383.0 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 2213.3-2331.1 ns)
   2213.3 |####################
   2219.2 |
   2225.1 |
   2231.0 |
   2236.9 |
   2242.7 |
   2248.6 |########################################
   2254.5 |
   2260.4 |
   2266.3 |
   2272.2 |
   2278.1 |
   2284.0 |
   2289.8 |########################################
   2295.7 |
   2301.6 |
   2307.5 |
   2313.4 |
   2319.3 |
   2325.2 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2076570.0-2276015.0 ns)
  2076570.0 |########################################
  2086542.2 |
  2096514.5 |####################
  2106486.8 |
  2116459.0 |
  2126431.2 |
  2136403.5 |
  2146375.8 |
  2156348.0 |
  2166320.2 |
  2176292.5 |
  2186264.8 |
  2196237.0 |####################
  2206209.2 |####################
  2216181.5 |
  2226153.8 |
  2236126.0 |
  2246098.2 |
  2256070.5 |
  2266042.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=5196.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=5170.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=309.3% of algo (FFI overhead may distort results)

# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_scalar is an outlier: 874.8x slower than the field

abi_cross_cold_scatter_warm_scalar (2.18 ms) is 874.8x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_scatter_cold_scalar shows alternating (throttle bounce) (autocorr -0.60)

abi_cross_cold_scatter_cold_scalar's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (2.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} (83483% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} with a 83483% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 874.8x the fastest

Fastest abi_cross_cold_scatter_warm_null (2.49 us) to slowest abi_cross_cold_scatter_warm_scalar (2.18 ms): 874.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 2493.8 ns median
- 2 variants significantly slower than baseline
- Spread: 874.76x (fastest 2493.8 ns, slowest 2181439.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 4916ns | 4885ns | 4668ns | 4836ns | 5159ns | +2.21% |
| abi_cross_cold_scatter_cold_scalar | 2180189ns | 2179107ns | 2163979ns | 2177343ns | 2192562ns | +45231.51% |
| abi_cross_cold_scatter_warm_null | 4809ns | 4811ns | 4676ns | 4768ns | 4938ns | base |
| abi_cross_cold_scatter_warm_scalar | 2190590ns | 2185208ns | 2174099ns | 2182582ns | 2210849ns | +45447.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 2586ns | 2440ns | 2713ns | +3.50% | 0.025 |
| abi_cross_cold_scatter_cold_scalar | 2176676ns | 2160797ns | 2188930ns | +87005.95% | 0.000 |
| abi_cross_cold_scatter_warm_null | 2499ns | 2425ns | 2572ns | base | 0.026 |
| abi_cross_cold_scatter_warm_scalar | 2186931ns | 2170320ns | 2207112ns | +87416.32% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 32237.4 | 2810.5 | 2586.5 | n/a |
| abi_cross_cold_scatter_cold_scalar | 73813.9 | 2175476.9 | 2176676.1 | n/a |
| abi_cross_cold_scatter_warm_null | 27928.7 | 2759.2 | 2498.9 | n/a |
| abi_cross_cold_scatter_warm_scalar | 70539.6 | 2183017.8 | 2186930.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.025 | 93.2% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_scatter_warm_null | 0.026 | 97.3% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 4916ns | 4916ns | +2.21% |
| abi_cross_cold_scatter_cold_scalar | 2180189ns | 2180189ns | +45231.51% |
| abi_cross_cold_scatter_warm_null | 4809ns | 4809ns | base |
| abi_cross_cold_scatter_warm_scalar | 2190590ns | 2190590ns | +45447.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 2494ns | base | --- | [2430, 2572] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 2603ns | no significant difference | [-74, +249]ns | [2444, 2713] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2175615ns | +2173149.6ns (+87143.8%) | [+2163011, +2186371]ns | [2165484, 2188930] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2181440ns | +2178980.4ns (+87377.7%) | [+2169710, +2204605]ns | [2172241, 2207112] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 2425ns | +0.6% | +89376.8% | +89827.6% |
| 2 | 2626ns | -2.9% | +83549.6% | +82700.0% |
| 3 | 2519ns | -2.8% | +85673.1% | +86882.3% |
| 4 | 2435ns | +9.1% | +89203.3% | +89015.5% |
| 5 | 2493ns | +11.0% | +87404.0% | +87419.6% |
| 6 | 2495ns | +6.5% | +87141.9% | +89011.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.313 | moderate+ |
| abi_cross_cold_scatter_cold_scalar | -0.597 | HIGH- (thermal bounce) |
| abi_cross_cold_scatter_warm_null | -0.293 | moderate- |
| abi_cross_cold_scatter_warm_scalar | -0.084 | ok |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 2/6, lost 4/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 120170.7ns | 2586.5ns | 4646.1% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6603596.3ns | 2176676.1ns | 303.4% | HIGH |
| abi_cross_cold_scatter_warm_null | 113101.2ns | 2498.9ns | 4526.1% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6629636.5ns | 2186930.7ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 2439.6-2712.7 ns)
   2439.6 |########################################
   2453.3 |
   2466.9 |
   2480.6 |
   2494.2 |
   2507.9 |
   2521.5 |
   2535.2 |####################
   2548.8 |
   2562.5 |
   2576.1 |
   2589.8 |
   2603.5 |
   2617.1 |
   2630.8 |
   2644.4 |########################################
   2658.1 |
   2671.7 |
   2685.4 |
   2699.0 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2160797.1-2188930.0 ns)
  2160797.1 |########################################
  2162203.7 |
  2163610.4 |
  2165017.0 |
  2166423.7 |
  2167830.3 |
  2169237.0 |########################################
  2170643.6 |
  2172050.3 |
  2173456.9 |
  2174863.5 |########################################
  2176270.2 |########################################
  2177676.8 |
  2179083.5 |
  2180490.1 |########################################
  2181896.8 |
  2183303.4 |
  2184710.1 |
  2186116.7 |
  2187523.4 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 2425.4-2572.5 ns)
   2425.4 |####################
   2432.8 |####################
   2440.1 |
   2447.5 |
   2454.8 |
   2462.2 |
   2469.5 |
   2476.9 |
   2484.2 |
   2491.6 |########################################
   2498.9 |
   2506.3 |
   2513.7 |####################
   2521.0 |
   2528.4 |
   2535.7 |
   2543.1 |
   2550.4 |
   2557.8 |
   2565.1 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2170320.0-2207111.6 ns)
  2170320.0 |########################################
  2172159.6 |
  2173999.2 |########################################
  2175838.7 |
  2177678.3 |
  2179517.9 |########################################
  2181357.5 |########################################
  2183197.1 |
  2185036.7 |
  2186876.2 |
  2188715.8 |
  2190555.4 |########################################
  2192395.0 |
  2194234.6 |
  2196074.2 |
  2197913.7 |
  2199753.3 |
  2201592.9 |
  2203432.5 |
  2205272.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=4624.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=4560.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=303.3% of algo (FFI overhead may distort results)

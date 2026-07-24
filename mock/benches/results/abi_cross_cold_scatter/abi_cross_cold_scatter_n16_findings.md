# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_warm_scalar is an outlier: 853.7x slower than the field

abi_cross_cold_scatter_warm_scalar (2.18 ms) is 853.7x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_scatter_warm_scalar shows alternating (throttle bounce) (autocorr -0.67)

abi_cross_cold_scatter_warm_scalar's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_scatter_warm_null)

The baseline abi_cross_cold_scatter_warm_null is the fastest (2.55 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} vs {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} (80441% apart)

The field splits into a fast tier {abi_cross_cold_scatter_warm_null, abi_cross_cold_scatter_cold_null} and a slow tier {abi_cross_cold_scatter_cold_scalar, abi_cross_cold_scatter_warm_scalar} with a 80441% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 853.7x the fastest

Fastest abi_cross_cold_scatter_warm_null (2.55 us) to slowest abi_cross_cold_scatter_warm_scalar (2.18 ms): 853.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_scatter_warm_null) is the fastest** at 2552.5 ns median
- 3 variants significantly slower than baseline
- Spread: 853.74x (fastest 2552.5 ns, slowest 2179176.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 4984ns | 4956ns | 4879ns | 4949ns | 5088ns | +1.26% |
| abi_cross_cold_scatter_cold_scalar | 2177984ns | 2178712ns | 2170225ns | 2176352ns | 2184311ns | +44152.38% |
| abi_cross_cold_scatter_warm_null | 4922ns | 4905ns | 4780ns | 4871ns | 5068ns | base |
| abi_cross_cold_scatter_warm_scalar | 2181474ns | 2182632ns | 2169053ns | 2181453ns | 2187715ns | +44223.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 2707ns | 2636ns | 2760ns | +5.35% | 0.006 |
| abi_cross_cold_scatter_cold_scalar | 2174523ns | 2166821ns | 2180741ns | +84524.95% | 0.000 |
| abi_cross_cold_scatter_warm_null | 2570ns | 2472ns | 2666ns | base | 0.006 |
| abi_cross_cold_scatter_warm_scalar | 2177941ns | 2165905ns | 2183908ns | +84657.98% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 32051.1 | 2809.6 | 2707.1 | n/a |
| abi_cross_cold_scatter_cold_scalar | 73753.9 | 2174737.4 | 2174522.8 | n/a |
| abi_cross_cold_scatter_warm_null | 27274.3 | 2652.6 | 2569.6 | n/a |
| abi_cross_cold_scatter_warm_scalar | 70991.0 | 2179739.5 | 2177941.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.006 | 91.5% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_scatter_warm_null | 0.006 | 96.9% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 4984ns | 4984ns | +1.26% |
| abi_cross_cold_scatter_cold_scalar | 2177984ns | 2177984ns | +44152.38% |
| abi_cross_cold_scatter_warm_null | 4922ns | 4922ns | base |
| abi_cross_cold_scatter_warm_scalar | 2181474ns | 2181474ns | +44223.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 2552ns | base | --- | [2490, 2666] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 2701ns | +143.9ns (+5.6%) | [+0, +268]ns | [2660, 2760] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2175239ns | +2172646.0ns (+85118.4%) | [+2165098, +2178115]ns | [2167588, 2180741] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2179177ns | +2176593.5ns (+85273.0%) | [+2168145, +2181375]ns | [2170738, 2183908] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 2472ns | +12.7% | +87613.1% | +88139.4% |
| 2 | 2512ns | +8.9% | +86439.5% | +86118.9% |
| 3 | 2593ns | +3.5% | +83963.4% | +84224.5% |
| 4 | 2674ns | +0.8% | +81301.7% | +81266.3% |
| 5 | 2509ns | +7.8% | +86268.8% | +86695.7% |
| 6 | 2658ns | -0.8% | +81987.4% | +81950.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.156 | ok |
| abi_cross_cold_scatter_cold_scalar | -0.334 | moderate- |
| abi_cross_cold_scatter_warm_null | -0.140 | ok |
| abi_cross_cold_scatter_warm_scalar | -0.675 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 1/6, lost 5/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 122932.0ns | 2707.1ns | 4541.1% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6606767.2ns | 2174522.8ns | 303.8% | HIGH |
| abi_cross_cold_scatter_warm_null | 118181.0ns | 2569.6ns | 4599.2% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6614776.6ns | 2177941.0ns | 303.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 2635.8-2760.4 ns)
   2635.8 |########################################
   2642.0 |
   2648.3 |
   2654.5 |
   2660.7 |
   2667.0 |
   2673.2 |
   2679.4 |########################################
   2685.6 |
   2691.9 |########################################
   2698.1 |
   2704.3 |########################################
   2710.6 |
   2716.8 |
   2723.0 |
   2729.2 |
   2735.5 |########################################
   2741.7 |
   2747.9 |
   2754.2 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2166821.2-2180740.8 ns)
  2166821.2 |########################################
  2167517.2 |
  2168213.2 |########################################
  2168909.1 |
  2169605.1 |
  2170301.1 |
  2170997.1 |
  2171693.1 |
  2172389.0 |
  2173085.0 |
  2173781.0 |########################################
  2174477.0 |
  2175173.0 |
  2175868.9 |########################################
  2176564.9 |
  2177260.9 |
  2177956.9 |
  2178652.9 |
  2179348.8 |########################################
  2180044.8 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 2472.1-2665.9 ns)
   2472.1 |########################################
   2481.8 |
   2491.5 |
   2501.2 |########################################
   2510.8 |########################################
   2520.5 |
   2530.2 |
   2539.9 |
   2549.6 |
   2559.3 |
   2569.0 |
   2578.7 |
   2588.4 |########################################
   2598.0 |
   2607.7 |
   2617.4 |
   2627.1 |
   2636.8 |
   2646.5 |
   2656.2 |########################################
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2165905.0-2183907.7 ns)
  2165905.0 |########################################
  2166805.1 |
  2167705.3 |
  2168605.4 |
  2169505.5 |
  2170405.7 |
  2171305.8 |
  2172205.9 |
  2173106.1 |
  2174006.2 |
  2174906.4 |########################################
  2175806.5 |
  2176706.6 |########################################
  2177606.8 |
  2178506.9 |
  2179407.0 |
  2180307.2 |########################################
  2181207.3 |########################################
  2182107.4 |
  2183007.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=4573.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=4632.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=303.4% of algo (FFI overhead may distort results)

# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_cold_scalar is an outlier: 819.6x slower than the field

abi_cross_cold_real_cold_scalar (2.18 ms) is 819.6x the fastest (2.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_real_cold_null shows alternating (throttle bounce) (autocorr -0.61)

abi_cross_cold_real_cold_null's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (78453% apart)

The field splits into a fast tier {abi_cross_cold_real_cold_null, abi_cross_cold_real_warm_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 78453% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 819.6x the fastest

Fastest abi_cross_cold_real_cold_null (2.66 us) to slowest abi_cross_cold_real_cold_scalar (2.18 ms): 819.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_real_cold_null** at 2658.6 ns median (-3.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 819.64x (fastest 2658.6 ns, slowest 2179044.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4895ns | 4918ns | 4821ns | 4898ns | 4930ns | -3.78% |
| abi_cross_cold_real_cold_scalar | 2183147ns | 2182334ns | 2172891ns | 2179928ns | 2193105ns | +42808.61% |
| abi_cross_cold_real_warm_null | 5088ns | 5126ns | 4842ns | 5059ns | 5254ns | base |
| abi_cross_cold_real_warm_scalar | 2181263ns | 2176798ns | 2172835ns | 2176113ns | 2193201ns | +42771.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 2658ns | 2608ns | 2685ns | -3.48% | 0.048 |
| abi_cross_cold_real_cold_scalar | 2179834ns | 2169588ns | 2189665ns | +79066.43% | 0.000 |
| abi_cross_cold_real_warm_null | 2753ns | 2635ns | 2820ns | base | 0.046 |
| abi_cross_cold_real_warm_scalar | 2177921ns | 2169692ns | 2189516ns | +78996.93% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 31981.1 | 2711.7 | 2657.8 | 35 |
| abi_cross_cold_real_cold_scalar | 71380.1 | 2178569.2 | 2179834.4 | n/a |
| abi_cross_cold_real_warm_null | 28657.3 | 2815.0 | 2753.5 | n/a |
| abi_cross_cold_real_warm_scalar | 65406.7 | 2177136.0 | 2177920.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_real_cold_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.048 | 98.1% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.046 | 94.2% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 4895ns | 4895ns | -3.78% |
| abi_cross_cold_real_cold_scalar | 2183147ns | 2183147ns | +42808.61% |
| abi_cross_cold_real_warm_null | 5088ns | 5088ns | base |
| abi_cross_cold_real_warm_scalar | 2181263ns | 2181263ns | +42771.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 2767ns | base | --- | [2674, 2820] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 2659ns | -90.4ns (-3.3%) | [-190, -7]ns | [2630, 2685] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_real_cold_scalar | 2179044ns | +2176230.9ns (+78646.6%) | [+2168089, +2186923]ns | [2170793, 2189665] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2173651ns | +2170832.5ns (+78451.5%) | [+2167921, +2186748]ns | [2170595, 2189516] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 2635ns | +1.3% | +82316.3% | +82228.8% |
| 2 | 2772ns | -4.3% | +79034.6% | +78310.3% |
| 3 | 2712ns | -1.8% | +80488.5% | +79967.0% |
| 4 | 2865ns | -9.0% | +76034.5% | +75770.5% |
| 5 | 2762ns | -2.2% | +78710.9% | +79069.5% |
| 6 | 2774ns | -4.4% | +78105.9% | +78924.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.612 | HIGH- (thermal bounce) |
| abi_cross_cold_real_cold_scalar | 0.016 | ok |
| abi_cross_cold_real_warm_null | -0.223 | moderate- |
| abi_cross_cold_real_warm_scalar | 0.418 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 5/6, lost 1/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 123671.8ns | 2657.8ns | 4653.2% | HIGH |
| abi_cross_cold_real_cold_scalar | 6613967.8ns | 2179834.4ns | 303.4% | HIGH |
| abi_cross_cold_real_warm_null | 120775.3ns | 2753.5ns | 4386.3% | HIGH |
| abi_cross_cold_real_warm_scalar | 6606377.4ns | 2177920.7ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 2607.9-2684.8 ns)
   2607.9 |####################
   2611.7 |
   2615.6 |
   2619.4 |
   2623.3 |
   2627.1 |
   2631.0 |
   2634.8 |
   2638.7 |
   2642.5 |
   2646.4 |
   2650.2 |########################################
   2654.0 |
   2657.9 |
   2661.7 |####################
   2665.6 |####################
   2669.4 |
   2673.3 |
   2677.1 |
   2681.0 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2169587.9-2189665.4 ns)
  2169587.9 |########################################
  2170591.8 |
  2171595.6 |########################################
  2172599.5 |
  2173603.4 |
  2174607.3 |
  2175611.1 |
  2176615.0 |########################################
  2177618.9 |
  2178622.8 |
  2179626.6 |
  2180630.5 |########################################
  2181634.4 |
  2182638.3 |
  2183642.1 |
  2184646.0 |########################################
  2185649.9 |
  2186653.8 |
  2187657.6 |
  2188661.5 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2635.4-2819.6 ns)
   2635.4 |########################################
   2644.6 |
   2653.8 |
   2663.0 |
   2672.2 |
   2681.4 |
   2690.7 |
   2699.9 |
   2709.1 |########################################
   2718.3 |
   2727.5 |
   2736.7 |
   2745.9 |
   2755.1 |########################################
   2764.3 |########################################
   2773.6 |########################################
   2782.8 |
   2792.0 |
   2801.2 |
   2810.4 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2169692.1-2189516.2 ns)
  2169692.1 |########################################
  2170683.3 |########################################
  2171674.5 |
  2172665.7 |########################################
  2173656.9 |########################################
  2174648.1 |
  2175639.3 |
  2176630.5 |
  2177621.7 |
  2178612.9 |
  2179604.2 |
  2180595.4 |
  2181586.6 |
  2182577.8 |
  2183569.0 |
  2184560.2 |
  2185551.4 |
  2186542.6 |########################################
  2187533.8 |
  2188525.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=4651.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=4403.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.3% of algo (FFI overhead may distort results)

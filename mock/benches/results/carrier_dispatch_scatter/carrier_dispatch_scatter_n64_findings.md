# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 41% faster than the next best (carrier_disp_scatter_switch)

carrier_disp_scatter_nullfloor (1.77 us) leads carrier_disp_scatter_switch (2.51 us) by 41%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 30% (significant)

carrier_disp_scatter_nullfloor is -747 ns (30%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 3.3x slower than the field

carrier_disp_scatter_ifchainlin (5.93 us) is 3.3x the fastest (1.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_ifchainlin shows alternating (throttle bounce) (autocorr -0.81)

carrier_disp_scatter_ifchainlin's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} vs {carrier_disp_scatter_ifchainlin} (83% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} and a slow tier {carrier_disp_scatter_ifchainlin} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest carrier_disp_scatter_nullfloor (1.77 us) to slowest carrier_disp_scatter_ifchainlin (5.93 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_scatter_ifchainasc's edge over baseline is significant but tiny (32 ns, 1.26%)

carrier_disp_scatter_ifchainasc differs from baseline carrier_disp_scatter_switch by 32 ns (1.26%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 1775.0 ns median (-29.2% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 3.34x (fastest 1775.0 ns, slowest 5930.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 5628ns | 5555ns | 5489ns | 5540ns | 5830ns | +11.44% |
| carrier_disp_scatter_fntable | 5853ns | 5864ns | 5748ns | 5854ns | 5904ns | +15.89% |
| carrier_disp_scatter_ifchain | 5168ns | 5198ns | 5014ns | 5159ns | 5260ns | +2.34% |
| carrier_disp_scatter_ifchainasc | 5047ns | 5149ns | 4500ns | 5124ns | 5206ns | -0.06% |
| carrier_disp_scatter_ifchainlin | 8506ns | 8520ns | 8416ns | 8497ns | 8565ns | +68.43% |
| carrier_disp_scatter_nullfloor | 4360ns | 4352ns | 4195ns | 4346ns | 4464ns | -13.66% |
| carrier_disp_scatter_switch | 5050ns | 5045ns | 4935ns | 5033ns | 5133ns | base |
| carrier_disp_scatter_threaded | 5369ns | 5324ns | 5285ns | 5324ns | 5480ns | +6.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 3036ns | 3010ns | 3062ns | +20.96% | 0.021 |
| carrier_disp_scatter_fntable | 3251ns | 3235ns | 3269ns | +29.54% | 0.020 |
| carrier_disp_scatter_ifchain | 2565ns | 2534ns | 2594ns | +2.20% | 0.025 |
| carrier_disp_scatter_ifchainasc | 2498ns | 2219ns | 2569ns | -0.46% | 0.026 |
| carrier_disp_scatter_ifchainlin | 5933ns | 5888ns | 5976ns | +136.38% | 0.011 |
| carrier_disp_scatter_nullfloor | 1770ns | 1748ns | 1783ns | -29.49% | 0.036 |
| carrier_disp_scatter_switch | 2510ns | 2490ns | 2528ns | base | 0.026 |
| carrier_disp_scatter_threaded | 2882ns | 2855ns | 2901ns | +14.83% | 0.022 |

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.021 | 57.6% |
| carrier_disp_scatter_fntable | 0.020 | 53.8% |
| carrier_disp_scatter_ifchain | 0.025 | 68.2% |
| carrier_disp_scatter_ifchainasc | 0.025 | 68.5% |
| carrier_disp_scatter_ifchainlin | 0.011 | 29.5% |
| carrier_disp_scatter_nullfloor | 0.036 | 98.5% |
| carrier_disp_scatter_switch | 0.026 | 69.7% |
| carrier_disp_scatter_threaded | 0.022 | 60.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 5628ns | 5628ns | +11.44% |
| carrier_disp_scatter_fntable | 5853ns | 5853ns | +15.89% |
| carrier_disp_scatter_ifchain | 5168ns | 5168ns | +2.34% |
| carrier_disp_scatter_ifchainasc | 5047ns | 5047ns | -0.06% |
| carrier_disp_scatter_ifchainlin | 8506ns | 8506ns | +68.43% |
| carrier_disp_scatter_nullfloor | 4360ns | 4360ns | -13.66% |
| carrier_disp_scatter_switch | 5050ns | 5050ns | base |
| carrier_disp_scatter_threaded | 5369ns | 5369ns | +6.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 2508ns | base | --- | [2494, 2528] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 3036ns | +522.7ns (+20.8%) | [+516, +540]ns | [3010, 3062] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 3249ns | +745.8ns (+29.7%) | [+709, +769]ns | [3236, 3269] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 2562ns | +57.3ns (+2.3%) | [+32, +76]ns | [2540, 2594] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_scatter_ifchainasc | 2552ns | no significant difference | [-128, +61]ns | [2374, 2569] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_scatter_ifchainlin | 5931ns | +3420.7ns (+136.4%) | [+3380, +3469]ns | [5892, 5976] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 1775ns | -746.6ns (-29.8%) | [-759, -714]ns | [1751, 1783] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 2879ns | +377.0ns (+15.0%) | [+351, +388]ns | [2865, 2901] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2490ns | +20.9% | +31.3% | +1.8% | -10.9% | +138.0% | -28.4% | +15.5% |
| 2 | 2510ns | +20.7% | +30.2% | +1.4% | +2.6% | +138.1% | -30.4% | +13.7% |
| 3 | 2497ns | +20.6% | +29.6% | +2.8% | +1.3% | +135.8% | -28.8% | +15.3% |
| 4 | 2505ns | +21.5% | +30.0% | +3.0% | +2.3% | +138.6% | -30.0% | +14.8% |
| 5 | 2527ns | +20.8% | +28.0% | +1.1% | +1.2% | +133.3% | -29.4% | +15.4% |
| 6 | 2530ns | +21.4% | +28.1% | +3.0% | +0.6% | +134.6% | -29.9% | +14.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | 0.305 | moderate+ |
| carrier_disp_scatter_fntable | 0.058 | ok |
| carrier_disp_scatter_ifchain | 0.010 | ok |
| carrier_disp_scatter_ifchainasc | -0.108 | ok |
| carrier_disp_scatter_ifchainlin | -0.809 | HIGH- (thermal bounce) |
| carrier_disp_scatter_nullfloor | -0.628 | HIGH- (thermal bounce) |
| carrier_disp_scatter_switch | 0.261 | moderate+ |
| carrier_disp_scatter_threaded | 0.077 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 0/6, lost 6/6
- **carrier_disp_scatter_fntable**: won 0/6, lost 6/6
- **carrier_disp_scatter_ifchain**: won 0/6, lost 6/6
- **carrier_disp_scatter_ifchainasc**: won 1/6, lost 5/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 86550.2ns | 3036.0ns | 2850.8% | HIGH |
| carrier_disp_scatter_fntable | 87091.4ns | 3251.1ns | 2678.8% | HIGH |
| carrier_disp_scatter_ifchain | 86996.3ns | 2565.1ns | 3391.5% | HIGH |
| carrier_disp_scatter_ifchainasc | 86651.4ns | 2498.3ns | 3468.5% | HIGH |
| carrier_disp_scatter_ifchainlin | 87826.0ns | 5932.8ns | 1480.4% | HIGH |
| carrier_disp_scatter_nullfloor | 86295.7ns | 1769.7ns | 4876.2% | HIGH |
| carrier_disp_scatter_switch | 87346.6ns | 2509.8ns | 3480.2% | HIGH |
| carrier_disp_scatter_threaded | 88313.5ns | 2881.9ns | 3064.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 3010.4-3061.6 ns)
   3010.4 |########################################
   3013.0 |
   3015.5 |
   3018.1 |
   3020.7 |
   3023.2 |
   3025.8 |####################
   3028.3 |
   3030.9 |
   3033.5 |
   3036.0 |
   3038.6 |
   3041.1 |####################
   3043.7 |
   3046.3 |
   3048.8 |
   3051.4 |####################
   3054.0 |
   3056.5 |
   3059.1 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 3235.0-3268.9 ns)
   3235.0 |########################################
   3236.7 |########################################
   3238.4 |########################################
   3240.1 |
   3241.8 |
   3243.5 |
   3245.2 |
   3246.9 |
   3248.6 |
   3250.3 |
   3252.0 |
   3253.7 |
   3255.4 |
   3257.1 |########################################
   3258.8 |
   3260.5 |
   3262.2 |
   3263.9 |
   3265.6 |########################################
   3267.3 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 2534.2-2593.8 ns)
   2534.2 |########################################
   2537.2 |
   2540.2 |
   2543.1 |########################################
   2546.1 |
   2549.1 |
   2552.1 |
   2555.0 |########################################
   2558.0 |
   2561.0 |
   2564.0 |
   2567.0 |########################################
   2569.9 |
   2572.9 |
   2575.9 |
   2578.9 |########################################
   2581.8 |
   2584.8 |
   2587.8 |
   2590.8 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 2219.2-2568.9 ns)
   2219.2 |####################
   2236.7 |
   2254.2 |
   2271.7 |
   2289.1 |
   2306.6 |
   2324.1 |
   2341.6 |
   2359.1 |
   2376.6 |
   2394.1 |
   2411.6 |
   2429.0 |
   2446.5 |
   2464.0 |
   2481.5 |
   2499.0 |
   2516.5 |####################
   2534.0 |####################
   2551.5 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 5888.3-5976.1 ns)
   5888.3 |########################################
   5892.7 |########################################
   5897.1 |
   5901.5 |
   5905.9 |
   5910.2 |
   5914.6 |
   5919.0 |
   5923.4 |########################################
   5927.8 |
   5932.2 |########################################
   5936.6 |
   5940.9 |
   5945.3 |
   5949.7 |
   5954.1 |
   5958.5 |
   5962.9 |
   5967.3 |
   5971.7 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 1747.5-1783.3 ns)
   1747.5 |########################################
   1749.3 |
   1751.1 |
   1752.9 |########################################
   1754.7 |
   1756.5 |
   1758.3 |
   1760.0 |
   1761.8 |
   1763.6 |
   1765.4 |
   1767.2 |
   1769.0 |
   1770.8 |
   1772.6 |########################################
   1774.4 |
   1776.2 |########################################
   1778.0 |
   1779.8 |
   1781.6 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 2490.4-2528.1 ns)
   2490.4 |########################################
   2492.3 |
   2494.2 |
   2496.1 |########################################
   2497.9 |
   2499.8 |
   2501.7 |
   2503.6 |########################################
   2505.5 |
   2507.4 |
   2509.3 |########################################
   2511.2 |
   2513.0 |
   2514.9 |
   2516.8 |
   2518.7 |
   2520.6 |
   2522.5 |
   2524.4 |
   2526.3 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 2854.6-2901.4 ns)
   2854.6 |####################
   2856.9 |
   2859.3 |
   2861.6 |
   2864.0 |
   2866.3 |
   2868.7 |
   2871.0 |
   2873.3 |
   2875.7 |########################################
   2878.0 |
   2880.4 |####################
   2882.7 |
   2885.1 |####################
   2887.4 |
   2889.7 |
   2892.1 |
   2894.4 |
   2896.8 |
   2899.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=2839.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=2677.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=3389.8% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=3396.8% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=1479.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=4861.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=3491.8% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=3069.8% of algo (FFI overhead may distort results)

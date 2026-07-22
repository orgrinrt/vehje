# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 31% faster than the next best (carrier_disp_wideselect_ifchainasc)

carrier_disp_wideselect_nullfloor (1.80 us) leads carrier_disp_wideselect_ifchainasc (2.37 us) by 31%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 24% (significant)

carrier_disp_wideselect_nullfloor is -575 ns (24%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 2.9x slower than the field

carrier_disp_wideselect_ifchainlin (5.29 us) is 2.9x the fastest (1.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_fntable shows alternating (throttle bounce) (autocorr -0.74)

carrier_disp_wideselect_fntable's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_bittree, carrier_disp_wideselect_threaded, carrier_disp_wideselect_fntable} vs {carrier_disp_wideselect_ifchainlin} (73% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_bittree, carrier_disp_wideselect_threaded, carrier_disp_wideselect_fntable} and a slow tier {carrier_disp_wideselect_ifchainlin} with a 73% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_wideselect_ifchainasc's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_disp_wideselect_ifchainasc are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### carrier_disp_wideselect_ifchainasc's edge over baseline is significant but tiny (-6 ns, 0.24%)

carrier_disp_wideselect_ifchainasc differs from baseline carrier_disp_wideselect_switch by -6 ns (0.24%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 1802.9 ns median (-24.1% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 2.94x (fastest 1802.9 ns, slowest 5291.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 5395ns | 5425ns | 5268ns | 5382ns | 5477ns | +8.17% |
| carrier_disp_wideselect_fntable | 5658ns | 5657ns | 5523ns | 5624ns | 5775ns | +13.45% |
| carrier_disp_wideselect_ifchain | 5083ns | 5086ns | 4998ns | 5077ns | 5135ns | +1.93% |
| carrier_disp_wideselect_ifchainasc | 4835ns | 4879ns | 4438ns | 4867ns | 4986ns | -3.04% |
| carrier_disp_wideselect_ifchainlin | 7879ns | 7873ns | 7736ns | 7859ns | 7981ns | +58.00% |
| carrier_disp_wideselect_nullfloor | 4392ns | 4379ns | 4292ns | 4362ns | 4488ns | -11.93% |
| carrier_disp_wideselect_switch | 4987ns | 4969ns | 4910ns | 4955ns | 5073ns | base |
| carrier_disp_wideselect_threaded | 5520ns | 5519ns | 5399ns | 5480ns | 5642ns | +10.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 2815ns | 2760ns | 2873ns | +18.37% | 0.023 |
| carrier_disp_wideselect_fntable | 3068ns | 3002ns | 3129ns | +29.00% | 0.021 |
| carrier_disp_wideselect_ifchain | 2513ns | 2491ns | 2534ns | +5.66% | 0.025 |
| carrier_disp_wideselect_ifchainasc | 2329ns | 2074ns | 2398ns | -2.05% | 0.027 |
| carrier_disp_wideselect_ifchainlin | 5306ns | 5249ns | 5366ns | +123.13% | 0.012 |
| carrier_disp_wideselect_nullfloor | 1802ns | 1780ns | 1820ns | -24.24% | 0.036 |
| carrier_disp_wideselect_switch | 2378ns | 2360ns | 2396ns | base | 0.027 |
| carrier_disp_wideselect_threaded | 2898ns | 2856ns | 2944ns | +21.88% | 0.022 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.023 | 63.5% |
| carrier_disp_wideselect_fntable | 0.021 | 58.1% |
| carrier_disp_wideselect_ifchain | 0.026 | 70.9% |
| carrier_disp_wideselect_ifchainasc | 0.027 | 75.1% |
| carrier_disp_wideselect_ifchainlin | 0.012 | 33.6% |
| carrier_disp_wideselect_nullfloor | 0.035 | 98.7% |
| carrier_disp_wideselect_switch | 0.027 | 75.0% |
| carrier_disp_wideselect_threaded | 0.022 | 61.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 5395ns | 5395ns | +8.17% |
| carrier_disp_wideselect_fntable | 5658ns | 5658ns | +13.45% |
| carrier_disp_wideselect_ifchain | 5083ns | 5083ns | +1.93% |
| carrier_disp_wideselect_ifchainasc | 4835ns | 4835ns | -3.04% |
| carrier_disp_wideselect_ifchainlin | 7879ns | 7879ns | +58.00% |
| carrier_disp_wideselect_nullfloor | 4392ns | 4392ns | -11.93% |
| carrier_disp_wideselect_switch | 4987ns | 4987ns | base |
| carrier_disp_wideselect_threaded | 5520ns | 5520ns | +10.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 2374ns | base | --- | [2364, 2396] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 2805ns | +410.5ns (+17.3%) | [+393, +507]ns | [2767, 2873] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 3060ns | +681.9ns (+28.7%) | [+649, +737]ns | [3013, 3129] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 2509ns | +138.5ns (+5.8%) | [+105, +160]ns | [2495, 2534] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 2370ns | no significant difference | [-175, +34]ns | [2220, 2398] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| carrier_disp_wideselect_ifchainlin | 5292ns | +2905.8ns (+122.4%) | [+2884, +2994]ns | [5260, 5366] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 1803ns | -574.8ns (-24.2%) | [-595, -560]ns | [1782, 1820] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 2886ns | +512.1ns (+21.6%) | [+471, +578]ns | [2865, 2944] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2381ns | +15.9% | +28.0% | +5.5% | -12.9% | +121.4% | -25.3% | +21.4% |
| 2 | 2376ns | +17.3% | +29.3% | +6.8% | -0.5% | +124.2% | -23.6% | +20.2% |
| 3 | 2368ns | +17.2% | +26.8% | +5.9% | +0.8% | +128.3% | -23.6% | +21.8% |
| 4 | 2412ns | +17.0% | +30.8% | +3.3% | -1.8% | +119.3% | -24.4% | +19.1% |
| 5 | 2360ns | +21.1% | +28.1% | +5.9% | +2.1% | +122.4% | -23.9% | +24.9% |
| 6 | 2372ns | +21.8% | +30.9% | +6.7% | +0.0% | +123.2% | -24.7% | +24.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.454 | moderate+ |
| carrier_disp_wideselect_fntable | -0.744 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_ifchain | 0.022 | ok |
| carrier_disp_wideselect_ifchainasc | 0.022 | ok |
| carrier_disp_wideselect_ifchainlin | 0.100 | ok |
| carrier_disp_wideselect_nullfloor | -0.053 | ok |
| carrier_disp_wideselect_switch | -0.513 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_threaded | 0.325 | moderate+ |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 0/6, lost 6/6
- **carrier_disp_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchain**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchainasc**: won 3/6, lost 2/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 87574.4ns | 2814.9ns | 3111.1% | HIGH |
| carrier_disp_wideselect_fntable | 87199.0ns | 3067.6ns | 2842.6% | HIGH |
| carrier_disp_wideselect_ifchain | 86424.5ns | 2512.5ns | 3439.7% | HIGH |
| carrier_disp_wideselect_ifchainasc | 86079.7ns | 2329.2ns | 3695.6% | HIGH |
| carrier_disp_wideselect_ifchainlin | 86776.2ns | 5306.1ns | 1635.4% | HIGH |
| carrier_disp_wideselect_nullfloor | 86608.3ns | 1801.6ns | 4807.3% | HIGH |
| carrier_disp_wideselect_switch | 85998.4ns | 2378.1ns | 3616.3% | HIGH |
| carrier_disp_wideselect_threaded | 88324.5ns | 2898.4ns | 3047.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 2760.0-2873.1 ns)
   2760.0 |########################################
   2765.7 |
   2771.3 |########################################
   2777.0 |
   2782.6 |########################################
   2788.3 |
   2793.9 |
   2799.6 |
   2805.2 |
   2810.9 |
   2816.6 |########################################
   2822.2 |
   2827.9 |
   2833.5 |
   2839.2 |
   2844.8 |
   2850.5 |
   2856.1 |########################################
   2861.8 |
   2867.4 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 3001.7-3129.3 ns)
   3001.7 |########################################
   3008.1 |
   3014.5 |
   3020.8 |########################################
   3027.2 |
   3033.6 |
   3040.0 |
   3046.4 |########################################
   3052.8 |
   3059.1 |
   3065.5 |
   3071.9 |########################################
   3078.3 |
   3084.7 |
   3091.1 |
   3097.4 |########################################
   3103.8 |
   3110.2 |
   3116.6 |
   3123.0 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 2490.8-2533.8 ns)
   2490.8 |########################################
   2492.9 |
   2495.1 |
   2497.2 |########################################
   2499.4 |
   2501.5 |
   2503.7 |
   2505.8 |########################################
   2508.0 |
   2510.1 |
   2512.3 |########################################
   2514.4 |
   2516.6 |
   2518.7 |
   2520.9 |
   2523.0 |
   2525.2 |
   2527.3 |
   2529.5 |########################################
   2531.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 2074.2-2398.1 ns)
   2074.2 |####################
   2090.4 |
   2106.6 |
   2122.8 |
   2139.0 |
   2155.2 |
   2171.4 |
   2187.6 |
   2203.8 |
   2220.0 |
   2236.2 |
   2252.4 |
   2268.6 |
   2284.8 |
   2301.0 |
   2317.2 |
   2333.4 |
   2349.6 |####################
   2365.8 |########################################
   2382.0 |####################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 5249.2-5366.2 ns)
   5249.2 |####################
   5255.1 |
   5260.9 |
   5266.8 |####################
   5272.6 |
   5278.5 |
   5284.3 |
   5290.2 |########################################
   5296.0 |
   5301.9 |
   5307.7 |
   5313.6 |
   5319.4 |
   5325.3 |####################
   5331.1 |
   5337.0 |
   5342.8 |
   5348.7 |
   5354.5 |
   5360.4 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 1779.6-1819.5 ns)
   1779.6 |########################################
   1781.6 |
   1783.6 |########################################
   1785.6 |
   1787.6 |
   1789.6 |
   1791.6 |
   1793.6 |
   1795.6 |########################################
   1797.6 |
   1799.6 |
   1801.6 |
   1803.6 |
   1805.6 |
   1807.6 |########################################
   1809.6 |
   1811.6 |
   1813.6 |
   1815.6 |########################################
   1817.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 2360.0-2396.4 ns)
   2360.0 |########################################
   2361.8 |
   2363.6 |
   2365.5 |
   2367.3 |########################################
   2369.1 |
   2370.9 |########################################
   2372.8 |
   2374.6 |########################################
   2376.4 |
   2378.2 |
   2380.0 |########################################
   2381.9 |
   2383.7 |
   2385.5 |
   2387.3 |
   2389.2 |
   2391.0 |
   2392.8 |
   2394.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 2856.2-2944.2 ns)
   2856.2 |########################################
   2860.6 |
   2865.0 |
   2869.4 |########################################
   2873.8 |
   2878.2 |########################################
   2882.6 |
   2887.0 |########################################
   2891.4 |
   2895.8 |
   2900.2 |
   2904.6 |
   2909.0 |
   2913.4 |
   2917.8 |
   2922.2 |
   2926.6 |
   2931.0 |
   2935.4 |
   2939.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=3111.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=2849.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=3445.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=3632.5% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=1636.7% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=4806.0% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=3630.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=3059.6% of algo (FFI overhead may distort results)

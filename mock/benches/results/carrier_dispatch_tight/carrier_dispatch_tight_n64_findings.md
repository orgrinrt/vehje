# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 23% faster than the next best (carrier_disp_tight_switch)

carrier_disp_tight_nullfloor (1.83 us) leads carrier_disp_tight_switch (2.25 us) by 23%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_ifchainlin is an outlier: 2.2x slower than the field

carrier_disp_tight_ifchainlin (3.94 us) is 2.2x the fastest (1.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_tight_nullfloor, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_ifchainasc, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable} vs {carrier_disp_tight_ifchainlin} (30% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_ifchainasc, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable} and a slow tier {carrier_disp_tight_ifchainlin} with a 30% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_tight_ifchain's edge over baseline is significant but tiny (17 ns, 0.77%)

carrier_disp_tight_ifchain differs from baseline carrier_disp_tight_switch by 17 ns (0.77%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 1828.8 ns median (-18.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.16x (fastest 1828.8 ns, slowest 3942.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 5124ns | 5092ns | 5030ns | 5076ns | 5243ns | +6.06% |
| carrier_disp_tight_fntable | 5642ns | 5663ns | 5535ns | 5647ns | 5689ns | +16.79% |
| carrier_disp_tight_ifchain | 4854ns | 4861ns | 4756ns | 4845ns | 4918ns | +0.47% |
| carrier_disp_tight_ifchainasc | 4998ns | 5048ns | 4388ns | 4979ns | 5332ns | +3.46% |
| carrier_disp_tight_ifchainlin | 6479ns | 6495ns | 6353ns | 6464ns | 6565ns | +34.11% |
| carrier_disp_tight_nullfloor | 4479ns | 4520ns | 4361ns | 4478ns | 4538ns | -7.29% |
| carrier_disp_tight_switch | 4831ns | 4816ns | 4745ns | 4810ns | 4906ns | base |
| carrier_disp_tight_threaded | 5435ns | 5440ns | 5321ns | 5402ns | 5541ns | +12.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 2552ns | 2532ns | 2576ns | +12.83% | 0.025 |
| carrier_disp_tight_fntable | 3031ns | 2996ns | 3069ns | +33.98% | 0.021 |
| carrier_disp_tight_ifchain | 2273ns | 2261ns | 2287ns | +0.48% | 0.028 |
| carrier_disp_tight_ifchainasc | 2392ns | 2152ns | 2590ns | +5.72% | 0.027 |
| carrier_disp_tight_ifchainlin | 3944ns | 3879ns | 3986ns | +74.35% | 0.016 |
| carrier_disp_tight_nullfloor | 1832ns | 1821ns | 1845ns | -19.03% | 0.035 |
| carrier_disp_tight_switch | 2262ns | 2234ns | 2292ns | base | 0.028 |
| carrier_disp_tight_threaded | 2920ns | 2880ns | 2981ns | +29.06% | 0.022 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.025 | 71.6% |
| carrier_disp_tight_fntable | 0.021 | 60.2% |
| carrier_disp_tight_ifchain | 0.028 | 80.2% |
| carrier_disp_tight_ifchainasc | 0.027 | 76.6% |
| carrier_disp_tight_ifchainlin | 0.016 | 46.2% |
| carrier_disp_tight_nullfloor | 0.035 | 99.6% |
| carrier_disp_tight_switch | 0.028 | 80.8% |
| carrier_disp_tight_threaded | 0.022 | 62.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 5124ns | 5124ns | +6.06% |
| carrier_disp_tight_fntable | 5642ns | 5642ns | +16.79% |
| carrier_disp_tight_ifchain | 4854ns | 4854ns | +0.47% |
| carrier_disp_tight_ifchainasc | 4998ns | 4998ns | +3.46% |
| carrier_disp_tight_ifchainlin | 6479ns | 6479ns | +34.11% |
| carrier_disp_tight_nullfloor | 4479ns | 4479ns | -7.29% |
| carrier_disp_tight_switch | 4831ns | 4831ns | base |
| carrier_disp_tight_threaded | 5435ns | 5435ns | +12.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 2255ns | base | --- | [2239, 2292] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 2544ns | +287.3ns (+12.7%) | [+246, +337]ns | [2537, 2576] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 3025ns | +773.8ns (+34.3%) | [+728, +804]ns | [2998, 3069] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 2270ns | no significant difference | [-25, +40]ns | [2262, 2287] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_tight_ifchainasc | 2376ns | no significant difference | [-62, +341]ns | [2209, 2590] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_tight_ifchainlin | 3943ns | +1703.7ns (+75.6%) | [+1610, +1732]ns | [3903, 3986] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 1829ns | -422.1ns (-18.7%) | [-467, -403]ns | [1821, 1845] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 2895ns | +641.0ns (+28.4%) | [+609, +722]ns | [2882, 2981] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2296ns | +10.3% | +30.5% | -1.5% | -6.3% | +69.0% | -20.3% | +26.2% |
| 2 | 2250ns | +13.0% | +34.3% | +1.4% | +2.2% | +76.7% | -18.0% | +28.6% |
| 3 | 2234ns | +16.2% | +35.5% | +1.2% | +21.8% | +76.1% | -18.2% | +28.9% |
| 4 | 2289ns | +11.2% | +33.9% | -0.7% | +7.4% | +71.5% | -20.4% | +26.9% |
| 5 | 2259ns | +12.5% | +36.1% | +0.3% | +8.6% | +76.9% | -19.4% | +35.3% |
| 6 | 2245ns | +14.0% | +33.7% | +2.1% | +0.9% | +76.1% | -17.9% | +28.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.203 | moderate- |
| carrier_disp_tight_fntable | 0.061 | ok |
| carrier_disp_tight_ifchain | -0.444 | moderate- |
| carrier_disp_tight_ifchainasc | 0.051 | ok |
| carrier_disp_tight_ifchainlin | -0.326 | moderate- |
| carrier_disp_tight_nullfloor | -0.129 | ok |
| carrier_disp_tight_switch | -0.274 | moderate- |
| carrier_disp_tight_threaded | -0.202 | moderate- |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 0/6, lost 6/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 2/6, lost 4/6
- **carrier_disp_tight_ifchainasc**: won 1/6, lost 5/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 86798.9ns | 2552.5ns | 3400.5% | HIGH |
| carrier_disp_tight_fntable | 83872.8ns | 3030.8ns | 2767.3% | HIGH |
| carrier_disp_tight_ifchain | 85930.3ns | 2273.0ns | 3780.5% | HIGH |
| carrier_disp_tight_ifchainasc | 86436.4ns | 2391.5ns | 3614.3% | HIGH |
| carrier_disp_tight_ifchainlin | 87772.9ns | 3944.1ns | 2225.4% | HIGH |
| carrier_disp_tight_nullfloor | 86432.1ns | 1831.6ns | 4718.9% | HIGH |
| carrier_disp_tight_switch | 85947.0ns | 2262.2ns | 3799.3% | HIGH |
| carrier_disp_tight_threaded | 88125.4ns | 2919.5ns | 3018.5% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 2532.5-2576.4 ns)
   2532.5 |####################
   2534.7 |
   2536.9 |
   2539.1 |
   2541.3 |########################################
   2543.5 |####################
   2545.7 |
   2547.9 |
   2550.1 |
   2552.3 |
   2554.5 |
   2556.7 |####################
   2558.9 |
   2561.1 |
   2563.3 |
   2565.5 |
   2567.7 |
   2569.9 |
   2572.1 |
   2574.3 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 2995.8-3069.4 ns)
   2995.8 |########################################
   2999.5 |########################################
   3003.2 |
   3006.8 |
   3010.5 |
   3014.2 |
   3017.9 |
   3021.6 |########################################
   3025.2 |########################################
   3028.9 |
   3032.6 |
   3036.3 |
   3040.0 |
   3043.6 |
   3047.3 |
   3051.0 |
   3054.7 |
   3058.4 |
   3062.0 |########################################
   3065.7 |
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 2261.2-2287.3 ns)
   2261.2 |########################################
   2262.5 |
   2263.8 |
   2265.1 |
   2266.4 |####################
   2267.7 |
   2269.0 |
   2270.3 |
   2271.6 |####################
   2272.9 |
   2274.2 |
   2275.6 |
   2276.9 |
   2278.2 |
   2279.5 |
   2280.8 |
   2282.1 |####################
   2283.4 |
   2284.7 |
   2286.0 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 2151.7-2590.0 ns)
   2151.7 |####################
   2173.6 |
   2195.5 |
   2217.4 |
   2239.4 |
   2261.3 |####################
   2283.2 |####################
   2305.1 |
   2327.0 |
   2348.9 |
   2370.8 |
   2392.8 |
   2414.7 |
   2436.6 |########################################
   2458.5 |
   2480.4 |
   2502.3 |
   2524.3 |
   2546.2 |
   2568.1 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 3879.2-3986.4 ns)
   3879.2 |########################################
   3884.6 |
   3889.9 |
   3895.3 |
   3900.6 |
   3906.0 |
   3911.4 |
   3916.7 |
   3922.1 |########################################
   3927.5 |
   3932.8 |########################################
   3938.2 |
   3943.5 |
   3948.9 |########################################
   3954.3 |
   3959.6 |
   3965.0 |
   3970.4 |
   3975.7 |########################################
   3981.1 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 1820.8-1844.6 ns)
   1820.8 |########################################
   1822.0 |########################################
   1823.2 |
   1824.4 |
   1825.6 |
   1826.8 |########################################
   1827.9 |
   1829.1 |########################################
   1830.3 |
   1831.5 |
   1832.7 |
   1833.9 |
   1835.1 |
   1836.3 |
   1837.5 |
   1838.6 |
   1839.8 |
   1841.0 |
   1842.2 |
   1843.4 |########################################
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 2233.8-2292.5 ns)
   2233.8 |########################################
   2236.7 |
   2239.7 |
   2242.6 |########################################
   2245.5 |
   2248.5 |########################################
   2251.4 |
   2254.3 |
   2257.3 |########################################
   2260.2 |
   2263.2 |
   2266.1 |
   2269.0 |
   2272.0 |
   2274.9 |
   2277.8 |
   2280.8 |
   2283.7 |
   2286.6 |########################################
   2289.6 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 2880.4-2981.1 ns)
   2880.4 |########################################
   2885.4 |
   2890.5 |####################
   2895.5 |####################
   2900.5 |####################
   2905.6 |
   2910.6 |
   2915.6 |
   2920.7 |
   2925.7 |
   2930.7 |
   2935.8 |
   2940.8 |
   2945.8 |
   2950.9 |
   2955.9 |
   2960.9 |
   2966.0 |
   2971.0 |
   2976.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=3402.0% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=2776.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=3779.0% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=3640.9% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=2226.8% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=4725.8% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=3811.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=3043.7% of algo (FFI overhead may distort results)

# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_real**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_real**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_nullfloor_real dominates: 31% faster than the next best (carrier_disp_ifchainasc_real)

carrier_disp_nullfloor_real (1.90 us) leads carrier_disp_ifchainasc_real (2.49 us) by 31%, a clear separation rather than a photo finish. CV 10.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_nullfloor_real beats baseline by 28% (significant)

carrier_disp_nullfloor_real is -739 ns (28%) faster than baseline carrier_disp_switch_real, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_ifchainlin_real is an outlier: 3.0x slower than the field

carrier_disp_ifchainlin_real (5.61 us) is 3.0x the fastest (1.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_ifchain_real shows alternating (throttle bounce) (autocorr -0.58)

carrier_disp_ifchain_real's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_nullfloor_real, carrier_disp_ifchainasc_real, carrier_disp_ifchain_real, carrier_disp_switch_real, carrier_disp_threaded_real, carrier_disp_bittree_real, carrier_disp_fntable_real} vs {carrier_disp_ifchainlin_real} (67% apart)

The field splits into a fast tier {carrier_disp_nullfloor_real, carrier_disp_ifchainasc_real, carrier_disp_ifchain_real, carrier_disp_switch_real, carrier_disp_threaded_real, carrier_disp_bittree_real, carrier_disp_fntable_real} and a slow tier {carrier_disp_ifchainlin_real} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_ifchain_real's edge over baseline is significant but tiny (-38 ns, 1.41%)

carrier_disp_ifchain_real differs from baseline carrier_disp_switch_real by -38 ns (1.41%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_nullfloor_real** at 1896.5 ns median (-28.8% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.96x (fastest 1896.5 ns, slowest 5605.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_bittree_real | 5646ns | 5972ns | 4764ns | 5577ns | 6190ns | +10.55% |
| carrier_disp_fntable_real | 5785ns | 5944ns | 4968ns | 5621ns | 6439ns | +13.27% |
| carrier_disp_ifchain_real | 5092ns | 5345ns | 4305ns | 5012ns | 5608ns | -0.29% |
| carrier_disp_ifchainasc_real | 4891ns | 4996ns | 4303ns | 4776ns | 5358ns | -4.23% |
| carrier_disp_ifchainlin_real | 8018ns | 8190ns | 6823ns | 7755ns | 9012ns | +57.00% |
| carrier_disp_nullfloor_real | 4381ns | 4607ns | 3704ns | 4308ns | 4828ns | -14.22% |
| carrier_disp_switch_real | 5107ns | 5312ns | 4368ns | 5003ns | 5634ns | base |
| carrier_disp_threaded_real | 5377ns | 5600ns | 4691ns | 5303ns | 5833ns | +5.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_bittree_real | 3070ns | 2590ns | 3366ns | +20.10% | 0.021 |
| carrier_disp_fntable_real | 3259ns | 2803ns | 3625ns | +27.47% | 0.020 |
| carrier_disp_ifchain_real | 2536ns | 2128ns | 2810ns | -0.81% | 0.025 |
| carrier_disp_ifchainasc_real | 2432ns | 2146ns | 2661ns | -4.86% | 0.026 |
| carrier_disp_ifchainlin_real | 5475ns | 4659ns | 6158ns | +114.17% | 0.012 |
| carrier_disp_nullfloor_real | 1802ns | 1530ns | 1975ns | -29.52% | 0.036 |
| carrier_disp_switch_real | 2557ns | 2177ns | 2825ns | base | 0.025 |
| carrier_disp_threaded_real | 2876ns | 2508ns | 3120ns | +12.48% | 0.022 |

## Performance model

- Peak throughput: **0.042 Gops/s** (carrier_disp_nullfloor_real; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_bittree_real | 0.020 | 47.1% |
| carrier_disp_fntable_real | 0.019 | 45.7% |
| carrier_disp_ifchain_real | 0.024 | 57.6% |
| carrier_disp_ifchainasc_real | 0.026 | 61.5% |
| carrier_disp_ifchainlin_real | 0.011 | 27.3% |
| carrier_disp_nullfloor_real | 0.034 | 80.7% |
| carrier_disp_switch_real | 0.024 | 57.4% |
| carrier_disp_threaded_real | 0.021 | 51.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_bittree_real | 5646ns | 5646ns | +10.55% |
| carrier_disp_fntable_real | 5785ns | 5785ns | +13.27% |
| carrier_disp_ifchain_real | 5092ns | 5092ns | -0.29% |
| carrier_disp_ifchainasc_real | 4891ns | 4891ns | -4.23% |
| carrier_disp_ifchainlin_real | 8018ns | 8018ns | +57.00% |
| carrier_disp_nullfloor_real | 4381ns | 4381ns | -14.22% |
| carrier_disp_switch_real | 5107ns | 5107ns | base |
| carrier_disp_threaded_real | 5377ns | 5377ns | +5.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_real | 2664ns | base | --- | [2181, 2825] | --- | --- | --- | --- |
| carrier_disp_bittree_real | 3249ns | +477.9ns (+17.9%) | [+416, +648]ns | [2597, 3366] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_fntable_real | 3348ns | +684.2ns (+25.7%) | [+623, +800]ns | [2804, 3625] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_ifchain_real | 2654ns | no significant difference | [-121, +96]ns | [2143, 2810] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_ifchainasc_real | 2487ns | -82.5ns (-3.1%) | [-259, -31]ns | [2150, 2661] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_ifchainlin_real | 5606ns | +2863.8ns (+107.5%) | [+2481, +3411]ns | [4662, 6158] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_nullfloor_real | 1896ns | -738.7ns (-27.7%) | [-878, -647]ns | [1534, 1975] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_threaded_real | 2993ns | +332.9ns (+12.5%) | [+218, +406]ns | [2514, 3120] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_real | carrier_disp_bittree_real | carrier_disp_fntable_real | carrier_disp_ifchain_real | carrier_disp_ifchainasc_real | carrier_disp_ifchainlin_real | carrier_disp_nullfloor_real | carrier_disp_threaded_real |
|---|---|---|---|---|---|---|---|---|
| 1 | 2177ns | +19.6% | +28.8% | -2.2% | -1.1% | +114.3% | -29.4% | +15.7% |
| 2 | 2853ns | +18.0% | +27.0% | -3.1% | -2.9% | +95.7% | -30.8% | +5.2% |
| 3 | 2697ns | +16.5% | +23.9% | -5.7% | -5.4% | +130.9% | -32.5% | +10.7% |
| 4 | 2797ns | +20.4% | +29.6% | +1.1% | -13.3% | +117.7% | -29.3% | +15.5% |
| 5 | 2185ns | +18.6% | +28.3% | -1.2% | -1.8% | +113.3% | -30.0% | +14.8% |
| 6 | 2631ns | +27.6% | +27.6% | +6.2% | -3.2% | +113.9% | -25.0% | +14.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_bittree_real | -0.530 | HIGH- (thermal bounce) |
| carrier_disp_fntable_real | -0.456 | moderate- |
| carrier_disp_ifchain_real | -0.580 | HIGH- (thermal bounce) |
| carrier_disp_ifchainasc_real | -0.286 | moderate- |
| carrier_disp_ifchainlin_real | -0.074 | ok |
| carrier_disp_nullfloor_real | -0.571 | HIGH- (thermal bounce) |
| carrier_disp_switch_real | -0.341 | moderate- |
| carrier_disp_threaded_real | -0.396 | moderate- |

**Consistency summary:**

- **carrier_disp_bittree_real**: won 0/6, lost 6/6
- **carrier_disp_fntable_real**: won 0/6, lost 6/6
- **carrier_disp_ifchain_real**: won 4/6, lost 2/6
- **carrier_disp_ifchainasc_real**: won 6/6, lost 0/6
- **carrier_disp_ifchainlin_real**: won 0/6, lost 6/6
- **carrier_disp_nullfloor_real**: won 6/6, lost 0/6
- **carrier_disp_threaded_real**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_bittree_real | 58.4ns | 3070.5ns | 1.9% |  |
| carrier_disp_fntable_real | 61.1ns | 3259.0ns | 1.9% |  |
| carrier_disp_ifchain_real | 63.6ns | 2535.9ns | 2.5% |  |
| carrier_disp_ifchainasc_real | 59.3ns | 2432.4ns | 2.4% |  |
| carrier_disp_ifchainlin_real | 68.4ns | 5475.4ns | 1.2% |  |
| carrier_disp_nullfloor_real | 59.1ns | 1801.8ns | 3.3% |  |
| carrier_disp_switch_real | 62.3ns | 2556.6ns | 2.4% |  |
| carrier_disp_threaded_real | 63.6ns | 2875.6ns | 2.2% |  |

## Distribution (algo ns)

```
carrier_disp_bittree_real (n=6, range 2590.4-3365.8 ns)
   2590.4 |########################################
   2629.2 |
   2667.9 |
   2706.7 |
   2745.5 |
   2784.3 |
   2823.0 |
   2861.8 |
   2900.6 |
   2939.4 |
   2978.1 |
   3016.9 |
   3055.7 |
   3094.4 |
   3133.2 |####################
   3172.0 |
   3210.8 |
   3249.5 |
   3288.3 |
   3327.1 |########################################
  (0 below, 1 above range)

carrier_disp_fntable_real (n=6, range 2802.9-3625.0 ns)
   2802.9 |########################################
   2844.0 |
   2885.1 |
   2926.2 |
   2967.3 |
   3008.4 |
   3049.5 |
   3090.6 |
   3131.7 |
   3172.8 |
   3213.9 |
   3255.1 |
   3296.2 |
   3337.3 |########################################
   3378.4 |
   3419.5 |
   3460.6 |
   3501.7 |
   3542.8 |
   3583.9 |####################
  (0 below, 1 above range)

carrier_disp_ifchain_real (n=6, range 2128.3-2810.4 ns)
   2128.3 |########################################
   2162.4 |
   2196.5 |
   2230.6 |
   2264.7 |
   2298.8 |
   2332.9 |
   2367.0 |
   2401.1 |
   2435.2 |
   2469.3 |
   2503.5 |
   2537.6 |####################
   2571.7 |
   2605.8 |
   2639.9 |
   2674.0 |
   2708.1 |
   2742.2 |####################
   2776.3 |####################
  (0 below, 1 above range)

carrier_disp_ifchainasc_real (n=6, range 2146.2-2661.0 ns)
   2146.2 |########################################
   2171.9 |
   2197.7 |
   2223.4 |
   2249.2 |
   2274.9 |
   2300.6 |
   2326.4 |
   2352.1 |
   2377.9 |
   2403.6 |####################
   2429.3 |
   2455.1 |
   2480.8 |
   2506.6 |
   2532.3 |########################################
   2558.0 |
   2583.8 |
   2609.5 |
   2635.3 |
  (0 below, 1 above range)

carrier_disp_ifchainlin_real (n=6, range 4659.2-6158.1 ns)
   4659.2 |########################################
   4734.1 |
   4809.1 |
   4884.0 |
   4959.0 |
   5033.9 |
   5108.9 |
   5183.8 |
   5258.8 |
   5333.7 |
   5408.6 |
   5483.6 |
   5558.5 |########################################
   5633.5 |
   5708.4 |
   5783.4 |
   5858.3 |
   5933.3 |
   6008.2 |
   6083.2 |####################
  (0 below, 1 above range)

carrier_disp_nullfloor_real (n=6, range 1529.6-1975.4 ns)
   1529.6 |########################################
   1551.9 |
   1574.2 |
   1596.5 |
   1618.8 |
   1641.0 |
   1663.3 |
   1685.6 |
   1707.9 |
   1730.2 |
   1752.5 |
   1774.8 |
   1797.1 |
   1819.4 |####################
   1841.7 |
   1864.0 |
   1886.2 |
   1908.5 |
   1930.8 |
   1953.1 |########################################
  (0 below, 1 above range)

carrier_disp_switch_real (n=6, range 2177.1-2825.0 ns)
   2177.1 |########################################
   2209.5 |
   2241.9 |
   2274.3 |
   2306.7 |
   2339.1 |
   2371.5 |
   2403.9 |
   2436.3 |
   2468.7 |
   2501.1 |
   2533.4 |
   2565.8 |
   2598.2 |
   2630.6 |####################
   2663.0 |
   2695.4 |####################
   2727.8 |
   2760.2 |
   2792.6 |####################
  (0 below, 1 above range)

carrier_disp_threaded_real (n=6, range 2508.3-3120.4 ns)
   2508.3 |########################################
   2538.9 |
   2569.5 |
   2600.1 |
   2630.7 |
   2661.3 |
   2691.9 |
   2722.5 |
   2753.1 |
   2783.7 |
   2814.4 |
   2845.0 |
   2875.6 |
   2906.2 |
   2936.8 |
   2967.4 |####################
   2998.0 |########################################
   3028.6 |
   3059.2 |
   3089.8 |
  (0 below, 1 above range)

```

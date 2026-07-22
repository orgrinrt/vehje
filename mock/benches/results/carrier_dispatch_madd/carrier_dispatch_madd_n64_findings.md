# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_nullfloor dominates: 27% faster than the next best (carrier_disp_madd_ifchainlin)

carrier_disp_madd_nullfloor (1.98 us) leads carrier_disp_madd_ifchainlin (2.52 us) by 27%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_madd_nullfloor beats baseline by 27% (significant)

carrier_disp_madd_nullfloor is -720 ns (27%) faster than baseline carrier_disp_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_madd_fntable shows alternating (throttle bounce) (autocorr -0.76)

carrier_disp_madd_fntable's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_madd_nullfloor} vs {carrier_disp_madd_ifchainlin, carrier_disp_madd_bittree, carrier_disp_madd_ifchainasc, carrier_disp_madd_ifchain, carrier_disp_madd_switch, carrier_disp_madd_threaded, carrier_disp_madd_fntable} (27% apart)

The field splits into a fast tier {carrier_disp_madd_nullfloor} and a slow tier {carrier_disp_madd_ifchainlin, carrier_disp_madd_bittree, carrier_disp_madd_ifchainasc, carrier_disp_madd_ifchain, carrier_disp_madd_switch, carrier_disp_madd_threaded, carrier_disp_madd_fntable} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_madd_bittree's edge over baseline is significant but tiny (-6 ns, 0.21%)

carrier_disp_madd_bittree differs from baseline carrier_disp_madd_switch by -6 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 1976.5 ns median (-26.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.50x (fastest 1976.5 ns, slowest 2968.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 5210ns | 5184ns | 5075ns | 5156ns | 5357ns | +0.30% |
| carrier_disp_madd_fntable | 5520ns | 5515ns | 5415ns | 5512ns | 5584ns | +6.27% |
| carrier_disp_madd_ifchain | 5288ns | 5279ns | 5126ns | 5251ns | 5423ns | +1.80% |
| carrier_disp_madd_ifchainasc | 5082ns | 5259ns | 4578ns | 5051ns | 5381ns | -2.16% |
| carrier_disp_madd_ifchainlin | 5063ns | 5039ns | 4974ns | 5033ns | 5152ns | -2.53% |
| carrier_disp_madd_nullfloor | 4595ns | 4551ns | 4490ns | 4542ns | 4728ns | -11.53% |
| carrier_disp_madd_switch | 5194ns | 5303ns | 4793ns | 5212ns | 5367ns | base |
| carrier_disp_madd_threaded | 5512ns | 5522ns | 5369ns | 5496ns | 5608ns | +6.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 2631ns | 2531ns | 2733ns | -0.15% | 0.024 |
| carrier_disp_madd_fntable | 2960ns | 2913ns | 2987ns | +12.34% | 0.022 |
| carrier_disp_madd_ifchain | 2716ns | 2672ns | 2784ns | +3.09% | 0.024 |
| carrier_disp_madd_ifchainasc | 2570ns | 2295ns | 2702ns | -2.48% | 0.025 |
| carrier_disp_madd_ifchainlin | 2526ns | 2502ns | 2555ns | -4.16% | 0.025 |
| carrier_disp_madd_nullfloor | 1971ns | 1925ns | 1995ns | -25.20% | 0.032 |
| carrier_disp_madd_switch | 2635ns | 2378ns | 2718ns | base | 0.024 |
| carrier_disp_madd_threaded | 2949ns | 2909ns | 2984ns | +11.90% | 0.022 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.024 | 73.5% |
| carrier_disp_madd_fntable | 0.022 | 64.9% |
| carrier_disp_madd_ifchain | 0.024 | 71.5% |
| carrier_disp_madd_ifchainasc | 0.024 | 72.1% |
| carrier_disp_madd_ifchainlin | 0.025 | 76.4% |
| carrier_disp_madd_nullfloor | 0.032 | 97.4% |
| carrier_disp_madd_switch | 0.024 | 71.5% |
| carrier_disp_madd_threaded | 0.022 | 65.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 5210ns | 5210ns | +0.30% |
| carrier_disp_madd_fntable | 5520ns | 5520ns | +6.27% |
| carrier_disp_madd_ifchain | 5288ns | 5288ns | +1.80% |
| carrier_disp_madd_ifchainasc | 5082ns | 5082ns | -2.16% |
| carrier_disp_madd_ifchainlin | 5063ns | 5063ns | -2.53% |
| carrier_disp_madd_nullfloor | 4595ns | 4595ns | -11.53% |
| carrier_disp_madd_switch | 5194ns | 5194ns | base |
| carrier_disp_madd_threaded | 5512ns | 5512ns | +6.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 2692ns | base | --- | [2495, 2718] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 2620ns | no significant difference | [-132, +126]ns | [2540, 2733] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_madd_fntable | 2968ns | +291.2ns (+10.8%) | [+219, +466]ns | [2926, 2987] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 2692ns | no significant difference | [-45, +237]ns | [2673, 2784] | no | 0.9625 | 0.6875 | 0 |
| carrier_disp_madd_ifchainasc | 2669ns | no significant difference | [-235, +51]ns | [2338, 2702] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_madd_ifchainlin | 2519ns | no significant difference | [-203, +37]ns | [2502, 2555] | no | 0.3828 | 0.2188 | 0 |
| carrier_disp_madd_nullfloor | 1976ns | -719.8ns (-26.7%) | [-769, -504]ns | [1942, 1995] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 2952ns | +273.8ns (+10.2%) | [+232, +435]ns | [2911, 2984] | YES (adj: no) | 0.0729 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2729ns | -7.3% | +8.5% | -2.1% | -15.9% | -7.3% | -29.5% | +10.4% |
| 2 | 2611ns | +0.2% | +14.2% | +6.1% | +3.0% | -1.9% | -23.3% | +11.4% |
| 3 | 2378ns | +7.2% | +23.6% | +13.2% | +0.1% | +5.3% | -16.8% | +24.0% |
| 4 | 2694ns | +3.0% | +11.0% | +3.8% | -1.0% | -6.8% | -26.3% | +9.7% |
| 5 | 2708ns | -0.6% | +7.6% | -1.2% | -1.3% | -7.6% | -27.1% | +7.6% |
| 6 | 2690ns | -2.5% | +10.6% | +0.1% | +0.8% | -5.2% | -27.2% | +9.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.028 | ok |
| carrier_disp_madd_fntable | -0.762 | HIGH- (thermal bounce) |
| carrier_disp_madd_ifchain | -0.559 | HIGH- (thermal bounce) |
| carrier_disp_madd_ifchainasc | -0.300 | moderate- |
| carrier_disp_madd_ifchainlin | -0.166 | ok |
| carrier_disp_madd_nullfloor | -0.292 | moderate- |
| carrier_disp_madd_switch | -0.035 | ok |
| carrier_disp_madd_threaded | -0.416 | moderate- |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 3/6, lost 3/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 2/6, lost 3/6
- **carrier_disp_madd_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_madd_ifchainlin**: won 5/6, lost 1/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 86864.3ns | 2631.1ns | 3301.4% | HIGH |
| carrier_disp_madd_fntable | 83599.7ns | 2960.3ns | 2824.0% | HIGH |
| carrier_disp_madd_ifchain | 86095.5ns | 2716.4ns | 3169.5% | HIGH |
| carrier_disp_madd_ifchainasc | 86284.1ns | 2569.6ns | 3357.8% | HIGH |
| carrier_disp_madd_ifchainlin | 87885.6ns | 2525.6ns | 3479.9% | HIGH |
| carrier_disp_madd_nullfloor | 86428.2ns | 1971.0ns | 4385.1% | HIGH |
| carrier_disp_madd_switch | 86775.7ns | 2635.1ns | 3293.1% | HIGH |
| carrier_disp_madd_threaded | 88131.8ns | 2948.7ns | 2988.8% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 2530.8-2732.9 ns)
   2530.8 |########################################
   2540.9 |########################################
   2551.0 |
   2561.1 |
   2571.2 |
   2581.3 |
   2591.4 |
   2601.5 |
   2611.6 |########################################
   2621.7 |########################################
   2631.9 |
   2642.0 |
   2652.1 |
   2662.2 |
   2672.3 |
   2682.4 |########################################
   2692.5 |
   2702.6 |
   2712.7 |
   2722.8 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 2913.3-2987.1 ns)
   2913.3 |########################################
   2917.0 |
   2920.7 |
   2924.4 |
   2928.1 |
   2931.8 |
   2935.4 |########################################
   2939.1 |
   2942.8 |
   2946.5 |
   2950.2 |
   2953.9 |
   2957.6 |########################################
   2961.3 |
   2965.0 |
   2968.7 |
   2972.3 |########################################
   2976.0 |
   2979.7 |########################################
   2983.4 |
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 2672.5-2783.5 ns)
   2672.5 |########################################
   2678.1 |
   2683.6 |
   2689.2 |########################################
   2694.7 |
   2700.2 |
   2705.8 |
   2711.3 |
   2716.9 |
   2722.4 |
   2728.0 |
   2733.6 |
   2739.1 |
   2744.7 |
   2750.2 |
   2755.8 |
   2761.3 |
   2766.8 |####################
   2772.4 |
   2777.9 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 2295.0-2701.9 ns)
   2295.0 |####################
   2315.3 |
   2335.7 |
   2356.0 |
   2376.4 |####################
   2396.7 |
   2417.1 |
   2437.4 |
   2457.7 |
   2478.1 |
   2498.4 |
   2518.8 |
   2539.1 |
   2559.5 |
   2579.8 |
   2600.1 |
   2620.5 |
   2640.8 |
   2661.2 |########################################
   2681.5 |####################
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 2501.7-2554.8 ns)
   2501.7 |########################################
   2504.4 |
   2507.0 |
   2509.7 |####################
   2512.3 |
   2515.0 |
   2517.6 |
   2520.3 |
   2522.9 |
   2525.6 |
   2528.2 |####################
   2530.9 |
   2533.6 |
   2536.2 |
   2538.9 |
   2541.5 |
   2544.2 |
   2546.8 |####################
   2549.5 |
   2552.1 |
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 1925.4-1994.6 ns)
   1925.4 |########################################
   1928.9 |
   1932.3 |
   1935.8 |
   1939.2 |
   1942.7 |
   1946.2 |
   1949.6 |
   1953.1 |
   1956.5 |########################################
   1960.0 |
   1963.5 |
   1966.9 |
   1970.4 |########################################
   1973.8 |
   1977.3 |########################################
   1980.8 |
   1984.2 |########################################
   1987.7 |
   1991.1 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 2377.9-2718.3 ns)
   2377.9 |####################
   2394.9 |
   2411.9 |
   2429.0 |
   2446.0 |
   2463.0 |
   2480.0 |
   2497.1 |
   2514.1 |
   2531.1 |
   2548.1 |
   2565.1 |
   2582.2 |
   2599.2 |####################
   2616.2 |
   2633.2 |
   2650.3 |
   2667.3 |
   2684.3 |########################################
   2701.3 |####################
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 2909.2-2983.9 ns)
   2909.2 |########################################
   2912.9 |
   2916.7 |
   2920.4 |
   2924.1 |
   2927.9 |
   2931.6 |
   2935.4 |
   2939.1 |
   2942.8 |
   2946.6 |####################
   2950.3 |####################
   2954.0 |####################
   2957.8 |
   2961.5 |
   2965.3 |
   2969.0 |
   2972.7 |
   2976.5 |
   2980.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=3302.7% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=2817.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=3197.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=3237.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=3494.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=4371.6% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=3222.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=2985.8% of algo (FFI overhead may distort results)

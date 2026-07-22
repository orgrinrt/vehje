# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_real**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_real**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_nullfloor_real dominates: 333% faster than the next best (carrier_disp_fntable_real)

carrier_disp_nullfloor_real (528.80 us) leads carrier_disp_fntable_real (2.29 ms) by 333%, a clear separation rather than a photo finish. CV 9.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_nullfloor_real beats baseline by 79% (significant)

carrier_disp_nullfloor_real is -2.07 ms (79%) faster than baseline carrier_disp_switch_real, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_ifchainlin_real is an outlier: 6.2x slower than the field

carrier_disp_ifchainlin_real (3.26 ms) is 6.2x the fastest (528.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_nullfloor_real is fastest but the noisiest (CV 9.5%)

carrier_disp_nullfloor_real wins on median (528.80 us) yet has the highest variance (CV 9.5%), while carrier_disp_fntable_real is the steadiest (CV 0.4%, 2.29 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_switch_real shows alternating (throttle bounce) (autocorr -0.61)

carrier_disp_switch_real's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_nullfloor_real} vs {carrier_disp_fntable_real, carrier_disp_ifchain_real, carrier_disp_switch_real, carrier_disp_ifchainasc_real, carrier_disp_bittree_real, carrier_disp_threaded_real, carrier_disp_ifchainlin_real} (333% apart)

The field splits into a fast tier {carrier_disp_nullfloor_real} and a slow tier {carrier_disp_fntable_real, carrier_disp_ifchain_real, carrier_disp_switch_real, carrier_disp_ifchainasc_real, carrier_disp_bittree_real, carrier_disp_threaded_real, carrier_disp_ifchainlin_real} with a 333% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.2x the fastest

Fastest carrier_disp_nullfloor_real (528.80 us) to slowest carrier_disp_ifchainlin_real (3.26 ms): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_nullfloor_real** at 528801.2 ns median (-79.8% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 6.16x (fastest 528801.2 ns, slowest 3255800.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_bittree_real | 2665217ns | 2700072ns | 2389435ns | 2682396ns | 2777341ns | +2.07% |
| carrier_disp_fntable_real | 2294223ns | 2295206ns | 2278155ns | 2292723ns | 2304508ns | -12.13% |
| carrier_disp_ifchain_real | 2611454ns | 2615480ns | 2569901ns | 2611806ns | 2631704ns | +0.02% |
| carrier_disp_ifchainasc_real | 2645764ns | 2646553ns | 2597901ns | 2639731ns | 2678747ns | +1.33% |
| carrier_disp_ifchainlin_real | 3272987ns | 3259085ns | 3229498ns | 3249505ns | 3329956ns | +25.35% |
| carrier_disp_nullfloor_real | 551220ns | 531511ns | 491254ns | 527095ns | 617392ns | -78.89% |
| carrier_disp_switch_real | 2611039ns | 2620682ns | 2580190ns | 2609997ns | 2628027ns | base |
| carrier_disp_threaded_real | 2931586ns | 2932734ns | 2903690ns | 2930071ns | 2947805ns | +12.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_bittree_real | 2661729ns | 2386028ns | 2773708ns | +2.08% | 0.006 |
| carrier_disp_fntable_real | 2290576ns | 2275120ns | 2300883ns | -12.15% | 0.007 |
| carrier_disp_ifchain_real | 2607858ns | 2566112ns | 2628642ns | +0.02% | 0.006 |
| carrier_disp_ifchainasc_real | 2642529ns | 2594308ns | 2675963ns | +1.35% | 0.006 |
| carrier_disp_ifchainlin_real | 3269501ns | 3226296ns | 3326276ns | +25.39% | 0.005 |
| carrier_disp_nullfloor_real | 548542ns | 488701ns | 614509ns | -78.96% | 0.030 |
| carrier_disp_switch_real | 2607377ns | 2576415ns | 2624381ns | base | 0.006 |
| carrier_disp_threaded_real | 2927832ns | 2899868ns | 2944180ns | +12.29% | 0.006 |

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_disp_nullfloor_real; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_bittree_real | 0.006 | 18.1% |
| carrier_disp_fntable_real | 0.007 | 21.3% |
| carrier_disp_ifchain_real | 0.006 | 18.7% |
| carrier_disp_ifchainasc_real | 0.006 | 18.5% |
| carrier_disp_ifchainlin_real | 0.005 | 15.0% |
| carrier_disp_nullfloor_real | 0.031 | 92.4% |
| carrier_disp_switch_real | 0.006 | 18.7% |
| carrier_disp_threaded_real | 0.006 | 16.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_bittree_real | 2665217ns | 2665217ns | +2.07% |
| carrier_disp_fntable_real | 2294223ns | 2294223ns | -12.13% |
| carrier_disp_ifchain_real | 2611454ns | 2611454ns | +0.02% |
| carrier_disp_ifchainasc_real | 2645764ns | 2645764ns | +1.33% |
| carrier_disp_ifchainlin_real | 3272987ns | 3272987ns | +25.35% |
| carrier_disp_nullfloor_real | 551220ns | 551220ns | -78.89% |
| carrier_disp_switch_real | 2611039ns | 2611039ns | base |
| carrier_disp_threaded_real | 2931586ns | 2931586ns | +12.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_real | 2617098ns | base | --- | [2580651, 2624381] | --- | --- | --- | --- |
| carrier_disp_bittree_real | 2696587ns | no significant difference | [-95521, +168046]ns | [2514893, 2773708] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_fntable_real | 2291101ns | -316862.0ns (-12.1%) | [-344637, -288903]ns | [2279744, 2300883] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_ifchain_real | 2611535ns | no significant difference | [-40193, +43146]ns | [2583396, 2628642] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_ifchainasc_real | 2643151ns | +40010.5ns (+1.5%) | [+13865, +51582]ns | [2608474, 2675963] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_ifchainlin_real | 3255800ns | +645055.2ns (+24.6%) | [+614046, +727270]ns | [3226426, 3326276] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_nullfloor_real | 528801ns | -2072475.2ns (-79.2%) | [-2100925, -2003104]ns | [502316, 614509] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_threaded_real | 2929277ns | +311664.4ns (+11.9%) | [+286171, +363528]ns | [2910038, 2944180] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_real | carrier_disp_bittree_real | carrier_disp_fntable_real | carrier_disp_ifchain_real | carrier_disp_ifchainasc_real | carrier_disp_ifchainlin_real | carrier_disp_nullfloor_real | carrier_disp_threaded_real |
|---|---|---|---|---|---|---|---|---|
| 1 | 2622625ns | +6.7% | -12.9% | -2.2% | +1.6% | +24.7% | -75.8% | +11.7% |
| 2 | 2612600ns | +1.7% | -11.7% | +0.7% | +1.6% | +24.0% | -77.3% | +12.2% |
| 3 | 2584888ns | +2.3% | -11.4% | +1.6% | +1.5% | +24.8% | -81.1% | +14.3% |
| 4 | 2621595ns | -9.0% | -12.5% | -0.8% | +0.4% | +28.0% | -80.3% | +11.4% |
| 5 | 2576415ns | +6.2% | -11.0% | +1.7% | +0.7% | +28.0% | -79.6% | +13.9% |
| 6 | 2626138ns | +4.7% | -13.4% | -0.9% | +2.3% | +22.9% | -79.8% | +10.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_bittree_real | -0.089 | ok |
| carrier_disp_fntable_real | -0.299 | moderate- |
| carrier_disp_ifchain_real | -0.285 | moderate- |
| carrier_disp_ifchainasc_real | -0.242 | moderate- |
| carrier_disp_ifchainlin_real | -0.103 | ok |
| carrier_disp_nullfloor_real | 0.285 | moderate+ |
| carrier_disp_switch_real | -0.605 | HIGH- (thermal bounce) |
| carrier_disp_threaded_real | -0.236 | moderate- |

**Consistency summary:**

- **carrier_disp_bittree_real**: won 1/6, lost 5/6
- **carrier_disp_fntable_real**: won 6/6, lost 0/6
- **carrier_disp_ifchain_real**: won 3/6, lost 3/6
- **carrier_disp_ifchainasc_real**: won 0/6, lost 6/6
- **carrier_disp_ifchainlin_real**: won 0/6, lost 6/6
- **carrier_disp_nullfloor_real**: won 6/6, lost 0/6
- **carrier_disp_threaded_real**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_bittree_real | 2612.9ns | 2661729.0ns | 0.1% |  |
| carrier_disp_fntable_real | 2595.9ns | 2290575.9ns | 0.1% |  |
| carrier_disp_ifchain_real | 2466.5ns | 2607857.5ns | 0.1% |  |
| carrier_disp_ifchainasc_real | 2618.1ns | 2642529.3ns | 0.1% |  |
| carrier_disp_ifchainlin_real | 2703.2ns | 3269500.6ns | 0.1% |  |
| carrier_disp_nullfloor_real | 2215.0ns | 548542.2ns | 0.4% |  |
| carrier_disp_switch_real | 2550.4ns | 2607376.8ns | 0.1% |  |
| carrier_disp_threaded_real | 2618.3ns | 2927831.5ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_bittree_real (n=6, range 2386028.3-2773707.5 ns)
  2386028.3 |####################
  2405412.3 |
  2424796.2 |
  2444180.2 |
  2463564.1 |
  2482948.1 |
  2502332.1 |
  2521716.0 |
  2541100.0 |
  2560483.9 |
  2579867.9 |
  2599251.9 |
  2618635.8 |
  2638019.8 |########################################
  2657403.7 |
  2676787.7 |
  2696171.7 |
  2715555.6 |
  2734939.6 |########################################
  2754323.5 |
  (0 below, 1 above range)

carrier_disp_fntable_real (n=6, range 2275120.4-2300883.2 ns)
  2275120.4 |########################################
  2276408.5 |
  2277696.7 |
  2278984.8 |
  2280273.0 |
  2281561.1 |
  2282849.2 |
  2284137.4 |########################################
  2285425.5 |
  2286713.6 |
  2288001.8 |
  2289289.9 |########################################
  2290578.1 |
  2291866.2 |########################################
  2293154.3 |########################################
  2294442.5 |
  2295730.6 |
  2297018.7 |
  2298306.9 |
  2299595.0 |
  (0 below, 1 above range)

carrier_disp_ifchain_real (n=6, range 2566111.7-2628641.9 ns)
  2566111.7 |####################
  2569238.2 |
  2572364.7 |
  2575491.2 |
  2578617.7 |
  2581744.2 |
  2584870.8 |
  2587997.3 |
  2591123.8 |
  2594250.3 |
  2597376.8 |
  2600503.3 |########################################
  2603629.8 |
  2606756.3 |
  2609882.8 |
  2613009.4 |
  2616135.9 |
  2619262.4 |####################
  2622388.9 |
  2625515.4 |####################
  (0 below, 1 above range)

carrier_disp_ifchainasc_real (n=6, range 2594308.3-2675962.7 ns)
  2594308.3 |########################################
  2598391.0 |
  2602473.7 |
  2606556.5 |
  2610639.2 |
  2614721.9 |
  2618804.6 |########################################
  2622887.3 |
  2626970.1 |
  2631052.8 |########################################
  2635135.5 |
  2639218.2 |
  2643300.9 |
  2647383.7 |
  2651466.4 |########################################
  2655549.1 |
  2659631.8 |
  2663714.5 |########################################
  2667797.3 |
  2671880.0 |
  (0 below, 1 above range)

carrier_disp_ifchainlin_real (n=6, range 3226296.2-3326275.7 ns)
  3226296.2 |########################################
  3231295.2 |
  3236294.1 |####################
  3241293.1 |
  3246292.1 |
  3251291.1 |
  3256290.0 |
  3261289.0 |
  3266288.0 |####################
  3271287.0 |
  3276285.9 |
  3281284.9 |
  3286283.9 |
  3291282.8 |
  3296281.8 |####################
  3301280.8 |
  3306279.8 |
  3311278.7 |
  3316277.7 |
  3321276.7 |
  (0 below, 1 above range)

carrier_disp_nullfloor_real (n=6, range 488700.8-614508.9 ns)
  488700.8 |########################################
  494991.2 |
  501281.6 |
  507572.0 |
  513862.4 |########################################
  520152.8 |########################################
  526443.2 |########################################
  532733.7 |
  539024.1 |
  545314.5 |
  551604.9 |
  557895.3 |
  564185.7 |
  570476.1 |
  576766.5 |
  583056.9 |
  589347.3 |########################################
  595637.7 |
  601928.1 |
  608218.5 |
  (0 below, 1 above range)

carrier_disp_switch_real (n=6, range 2576415.4-2624381.0 ns)
  2576415.4 |########################################
  2578813.7 |
  2581212.0 |
  2583610.2 |########################################
  2586008.5 |
  2588406.8 |
  2590805.1 |
  2593203.4 |
  2595601.7 |
  2597999.9 |
  2600398.2 |
  2602796.5 |
  2605194.8 |
  2607593.1 |
  2609991.4 |
  2612389.6 |########################################
  2614787.9 |
  2617186.2 |
  2619584.5 |########################################
  2621982.8 |########################################
  (0 below, 1 above range)

carrier_disp_threaded_real (n=6, range 2899868.3-2944179.8 ns)
  2899868.3 |########################################
  2902083.9 |
  2904299.4 |
  2906515.0 |
  2908730.6 |
  2910946.2 |
  2913161.8 |
  2915377.3 |
  2917592.9 |
  2919808.5 |########################################
  2922024.0 |
  2924239.6 |
  2926455.2 |########################################
  2928670.8 |########################################
  2930886.3 |
  2933101.9 |########################################
  2935317.5 |
  2937533.1 |
  2939748.6 |
  2941964.2 |
  (0 below, 1 above range)

```

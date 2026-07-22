# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_disp_switch_v4)

The baseline carrier_disp_switch_v4 is the fastest (2.53 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_disp_switch_v4 vs stability leader carrier_disp_threaded_v4 (+9% speed for 1.2x steadier)

carrier_disp_switch_v4 is fastest (2.53 us, CV 6.3%); carrier_disp_threaded_v4 gives up 8.6% median for 1.2x lower variance (CV 5.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (carrier_disp_switch_v4) is the fastest** at 2531.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 2531.9 ns, slowest 2929.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 5472ns | 5626ns | 4364ns | 5544ns | 5918ns | +5.32% |
| carrier_disp_switch_v4 | 5196ns | 5235ns | 4673ns | 5171ns | 5494ns | base |
| carrier_disp_threaded_v4 | 5360ns | 5538ns | 4785ns | 5417ns | 5561ns | +3.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 2860ns | 2270ns | 3111ns | +13.33% | 0.022 |
| carrier_disp_switch_v4 | 2524ns | 2274ns | 2688ns | base | 0.025 |
| carrier_disp_threaded_v4 | 2666ns | 2375ns | 2758ns | +5.64% | 0.024 |

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_disp_fntable_v4; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.022 | 77.5% |
| carrier_disp_switch_v4 | 0.025 | 89.7% |
| carrier_disp_threaded_v4 | 0.023 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 5472ns | 5472ns | +5.32% |
| carrier_disp_switch_v4 | 5196ns | 5196ns | base |
| carrier_disp_threaded_v4 | 5360ns | 5360ns | +3.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 2532ns | base | --- | [2352, 2688] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 2930ns | +380.4ns (+15.0%) | [+153, +476]ns | [2540, 3111] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_disp_threaded_v4 | 2751ns | no significant difference | [-21, +327]ns | [2490, 2758] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 | carrier_disp_threaded_v4 |
|---|---|---|---|
| 1 | 2274ns | -0.2% | +14.6% |
| 2 | 2634ns | +15.9% | +4.4% |
| 3 | 2637ns | +20.2% | +4.8% |
| 4 | 2430ns | +15.6% | +13.3% |
| 5 | 2739ns | +11.3% | +0.5% |
| 6 | 2430ns | +15.7% | -2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | -0.170 | ok |
| carrier_disp_switch_v4 | -0.437 | moderate- |
| carrier_disp_threaded_v4 | -0.054 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 1/6, lost 5/6
- **carrier_disp_threaded_v4**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 65.6ns | 2860.1ns | 2.3% |  |
| carrier_disp_switch_v4 | 66.4ns | 2523.8ns | 2.6% |  |
| carrier_disp_threaded_v4 | 66.1ns | 2666.2ns | 2.5% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 2270.0-3111.1 ns)
   2270.0 |####################
   2312.1 |
   2354.1 |
   2396.2 |
   2438.2 |
   2480.3 |
   2522.3 |
   2564.4 |
   2606.4 |
   2648.5 |
   2690.5 |
   2732.6 |
   2774.6 |########################################
   2816.7 |
   2858.7 |
   2900.8 |
   2942.8 |
   2984.9 |
   3026.9 |########################################
   3069.0 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 2273.8-2687.8 ns)
   2273.8 |####################
   2294.5 |
   2315.2 |
   2335.9 |
   2356.6 |
   2377.3 |
   2398.0 |
   2418.7 |########################################
   2439.4 |
   2460.1 |
   2480.8 |
   2501.5 |
   2522.2 |
   2542.9 |
   2563.6 |
   2584.3 |
   2605.0 |
   2625.7 |########################################
   2646.4 |
   2667.1 |
  (0 below, 1 above range)

carrier_disp_threaded_v4 (n=6, range 2375.4-2757.5 ns)
   2375.4 |#############
   2394.5 |
   2413.6 |
   2432.7 |
   2451.8 |
   2470.9 |
   2490.0 |
   2509.1 |
   2528.2 |
   2547.3 |
   2566.4 |
   2585.6 |
   2604.7 |#############
   2623.8 |
   2642.9 |
   2662.0 |
   2681.1 |
   2700.2 |
   2719.3 |
   2738.4 |########################################
  (0 below, 1 above range)

```

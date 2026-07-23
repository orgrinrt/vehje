# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_scatter_rec24 shows alternating (throttle bounce) (autocorr -0.54)

carrier_lay_scatter_rec24's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_lay_scatter_rec24)

The baseline carrier_lay_scatter_rec24 is the fastest (2.60 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.4% of the fastest

All 5 variants sit between 2.60 ms and 2.63 ms - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_scatter_rec24) is the fastest** at 2596911.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 2596911.7 ns, slowest 2634372.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2614227ns | 2611452ns | 2597499ns | 2609223ns | 2630096ns | +0.38% |
| carrier_lay_scatter_rec16 | 2646266ns | 2637980ns | 2610349ns | 2630826ns | 2687385ns | +1.61% |
| carrier_lay_scatter_rec20 | 2691673ns | 2624298ns | 2606589ns | 2620689ns | 2840690ns | +3.35% |
| carrier_lay_scatter_rec24 | 2604367ns | 2599849ns | 2584412ns | 2597160ns | 2625155ns | base |
| carrier_lay_scatter_rec32 | 2630672ns | 2625562ns | 2617386ns | 2622947ns | 2648904ns | +1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2611475ns | 2594831ns | 2626907ns | +0.38% | 0.006 |
| carrier_lay_scatter_rec16 | 2643203ns | 2607898ns | 2684301ns | +1.60% | 0.006 |
| carrier_lay_scatter_rec20 | 2688408ns | 2604019ns | 2837078ns | +3.34% | 0.006 |
| carrier_lay_scatter_rec24 | 2601481ns | 2582052ns | 2621855ns | base | 0.006 |
| carrier_lay_scatter_rec32 | 2627701ns | 2615069ns | 2645780ns | +1.01% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 16361868 | 18990098 | 0.862 | 1.00× |
| carrier_lay_scatter_rec16 | 16459912 | 18769931 | 0.877 | 1.01× |
| carrier_lay_scatter_rec20 | 16463786 | 18780256 | 0.877 | 1.01× |
| carrier_lay_scatter_rec24 | 16325053 | 18759836 | 0.870 | 1.00× |
| carrier_lay_scatter_rec32 | 16443113 | 18765389 | 0.876 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_lay_scatter_rec24; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.006 | 99.0% |
| carrier_lay_scatter_rec16 | 0.006 | 98.0% |
| carrier_lay_scatter_rec20 | 0.006 | 98.5% |
| carrier_lay_scatter_rec24 | 0.006 | 99.4% |
| carrier_lay_scatter_rec32 | 0.006 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 2614227ns | 2614227ns | +0.38% |
| carrier_lay_scatter_rec16 | 2646266ns | 2646266ns | +1.61% |
| carrier_lay_scatter_rec20 | 2691673ns | 2691673ns | +3.35% |
| carrier_lay_scatter_rec24 | 2604367ns | 2604367ns | base |
| carrier_lay_scatter_rec32 | 2630672ns | 2630672ns | +1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 2596912ns | base | --- | [2585677, 2621855] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 2608860ns | no significant difference | [-8608, +30490]ns | [2598657, 2626907] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 2634372ns | no significant difference | [-4495, +98624]ns | [2610937, 2684301] | no | 0.2917 | 0.2188 | 0 |
| carrier_lay_scatter_rec20 | 2621020ns | no significant difference | [-6688, +250145]ns | [2607127, 2837078] | no | 0.2917 | 0.2188 | 0 |
| carrier_lay_scatter_rec32 | 2622252ns | +25532.3ns (+1.0%) | [+1723, +51405]ns | [2615072, 2645780] | YES (adj: no) | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 2602009ns | +1.2% | +0.8% | +0.8% | +0.7% |
| 2 | 2604661ns | -0.0% | +1.5% | +0.2% | +1.1% |
| 3 | 2591814ns | +0.4% | +0.6% | +2.0% | +0.9% |
| 4 | 2589302ns | +0.2% | +2.7% | +0.6% | +2.7% |
| 5 | 2639049ns | -0.7% | -1.0% | -0.7% | -0.6% |
| 6 | 2582052ns | +1.2% | +4.9% | +17.4% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.091 | ok |
| carrier_lay_scatter_rec16 | -0.431 | moderate- |
| carrier_lay_scatter_rec20 | -0.035 | ok |
| carrier_lay_scatter_rec24 | -0.538 | HIGH- (thermal bounce) |
| carrier_lay_scatter_rec32 | -0.403 | moderate- |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 1/6, lost 4/6
- **carrier_lay_scatter_rec16**: won 1/6, lost 5/6
- **carrier_lay_scatter_rec20**: won 1/6, lost 5/6
- **carrier_lay_scatter_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2612205.6ns | 2611474.6ns | 100.0% | HIGH |
| carrier_lay_scatter_rec16 | 2645152.2ns | 2643203.2ns | 100.1% | HIGH |
| carrier_lay_scatter_rec20 | 2691126.9ns | 2688408.2ns | 100.1% | HIGH |
| carrier_lay_scatter_rec24 | 2604390.3ns | 2601481.3ns | 100.1% | HIGH |
| carrier_lay_scatter_rec32 | 2628184.3ns | 2627701.4ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 2594830.8-2626906.6 ns)
  2594830.8 |########################################
  2596434.6 |
  2598038.4 |
  2599642.2 |
  2601246.0 |########################################
  2602849.8 |
  2604453.6 |########################################
  2606057.3 |
  2607661.1 |
  2609264.9 |
  2610868.7 |
  2612472.5 |########################################
  2614076.3 |
  2615680.1 |
  2617283.9 |
  2618887.7 |
  2620491.5 |########################################
  2622095.3 |
  2623699.1 |
  2625302.9 |
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 2607898.3-2684300.6 ns)
  2607898.3 |########################################
  2611718.4 |########################################
  2615538.5 |
  2619358.6 |
  2623178.8 |########################################
  2626998.9 |
  2630819.0 |
  2634639.1 |
  2638459.2 |
  2642279.3 |########################################
  2646099.5 |
  2649919.6 |
  2653739.7 |
  2657559.8 |########################################
  2661379.9 |
  2665200.0 |
  2669020.1 |
  2672840.3 |
  2676660.4 |
  2680480.5 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 2604018.7-2837077.9 ns)
  2604018.7 |########################################
  2615671.7 |########################################
  2627324.6 |
  2638977.6 |####################
  2650630.5 |
  2662283.5 |
  2673936.5 |
  2685589.4 |
  2697242.4 |
  2708895.3 |
  2720548.3 |
  2732201.3 |
  2743854.2 |
  2755507.2 |
  2767160.1 |
  2778813.1 |
  2790466.1 |
  2802119.0 |
  2813772.0 |
  2825424.9 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 2582051.7-2621855.0 ns)
  2582051.7 |########################################
  2584041.9 |
  2586032.0 |
  2588022.2 |########################################
  2590012.4 |########################################
  2592002.5 |
  2593992.7 |
  2595982.9 |
  2597973.0 |
  2599963.2 |
  2601953.4 |########################################
  2603943.5 |########################################
  2605933.7 |
  2607923.8 |
  2609914.0 |
  2611904.2 |
  2613894.3 |
  2615884.5 |
  2617874.7 |
  2619864.8 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 2615069.2-2645779.5 ns)
  2615069.2 |########################################
  2616604.7 |
  2618140.2 |
  2619675.8 |####################
  2621211.3 |
  2622746.8 |####################
  2624282.3 |
  2625817.8 |
  2627353.3 |
  2628888.9 |
  2630424.4 |
  2631959.9 |####################
  2633495.4 |
  2635030.9 |
  2636566.4 |
  2638102.0 |
  2639637.5 |
  2641173.0 |
  2642708.5 |
  2644244.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=99.9% of algo (FFI overhead may distort results)

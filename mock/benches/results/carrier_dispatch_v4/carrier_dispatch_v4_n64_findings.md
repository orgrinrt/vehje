# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_switch_v4 dominates: 24% faster than the next best (carrier_disp_fntable_v4)

carrier_disp_switch_v4 (2.20 us) leads carrier_disp_fntable_v4 (2.72 us) by 24%, a clear separation rather than a photo finish. CV 7.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_disp_switch_v4)

The baseline carrier_disp_switch_v4 is the fastest (2.20 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_disp_switch_v4) is the fastest** at 2201.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 2201.0 ns, slowest 2721.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 5044ns | 5230ns | 4464ns | 4997ns | 5405ns | +9.18% |
| carrier_disp_switch_v4 | 4620ns | 4546ns | 4229ns | 4480ns | 5025ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 2628ns | 2325ns | 2820ns | +17.44% | 0.024 |
| carrier_disp_switch_v4 | 2238ns | 2042ns | 2439ns | base | 0.029 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_switch_v4; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.024 | 75.0% |
| carrier_disp_switch_v4 | 0.029 | 92.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 5044ns | 5044ns | +9.18% |
| carrier_disp_switch_v4 | 4620ns | 4620ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 2201ns | base | --- | [2074, 2439] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 2721ns | +357.3ns (+16.2%) | [+269, +544]ns | [2344, 2820] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 |
|---|---|---|
| 1 | 2042ns | +15.7% |
| 2 | 2106ns | +10.4% |
| 3 | 2440ns | +15.6% |
| 4 | 2437ns | +15.2% |
| 5 | 2111ns | +33.6% |
| 6 | 2291ns | +15.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.346 | moderate+ |
| carrier_disp_switch_v4 | 0.048 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 53.1ns | 2628.3ns | 2.0% |  |
| carrier_disp_switch_v4 | 55.3ns | 2237.9ns | 2.5% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 2325.4-2820.0 ns)
   2325.4 |####################
   2350.1 |####################
   2374.9 |
   2399.6 |
   2424.3 |
   2449.1 |
   2473.8 |
   2498.5 |
   2523.2 |
   2548.0 |
   2572.7 |
   2597.4 |
   2622.2 |####################
   2646.9 |
   2671.6 |
   2696.3 |
   2721.1 |
   2745.8 |
   2770.5 |
   2795.3 |########################################
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 2042.1-2438.6 ns)
   2042.1 |####################
   2061.9 |
   2081.7 |
   2101.6 |########################################
   2121.4 |
   2141.2 |
   2161.0 |
   2180.9 |
   2200.7 |
   2220.5 |
   2240.3 |
   2260.1 |
   2280.0 |####################
   2299.8 |
   2319.6 |
   2339.4 |
   2359.3 |
   2379.1 |
   2398.9 |
   2418.7 |####################
  (0 below, 1 above range)

```

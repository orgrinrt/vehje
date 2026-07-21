# Interpreter dispatch shapes: switch vs fnptr-table vs computed

2 variants, 6 samples per variant.
Baseline: **hx_dispatch__switch**

## Highlights

Baseline for all deltas below: **hx_dispatch__switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_dispatch__switch dominates: 115% faster than the next best (hx_dispatch__fnptr)

hx_dispatch__switch (1.23 us) leads hx_dispatch__fnptr (2.64 us) by 115%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_dispatch__switch)

The baseline hx_dispatch__switch is the fastest (1.23 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_dispatch__switch) is the fastest** at 1228.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.15x (fastest 1228.3 ns, slowest 2636.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_dispatch__fnptr | 4875ns | 4846ns | 4548ns | 4776ns | 5187ns | +37.85% |
| hx_dispatch__switch | 3537ns | 3443ns | 3378ns | 3433ns | 3772ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_dispatch__fnptr | 2661ns | 2430ns | 2905ns | +109.92% | 0.385 |
| hx_dispatch__switch | 1268ns | 1201ns | 1368ns | base | 0.808 |

## Performance model

- Peak throughput: **0.852 Gops/s** (hx_dispatch__switch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_dispatch__fnptr | 0.388 | 45.6% |
| hx_dispatch__switch | 0.834 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_dispatch__fnptr | 4875ns | 4875ns | +37.85% |
| hx_dispatch__switch | 3537ns | 3537ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_dispatch__switch | 1228ns | base | --- | [1207, 1368] | --- | --- | --- | --- |
| hx_dispatch__fnptr | 2637ns | +1305.2ns (+106.3%) | [+1220, +1655]ns | [2441, 2905] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_dispatch__switch | hx_dispatch__fnptr |
|---|---|---|
| 1 | 1201ns | +102.3% |
| 2 | 1450ns | +90.2% |
| 3 | 1285ns | +117.6% |
| 4 | 1241ns | +97.7% |
| 5 | 1212ns | +107.4% |
| 6 | 1216ns | +147.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_dispatch__fnptr | -0.217 | moderate- |
| hx_dispatch__switch | -0.116 | ok |

**Consistency summary:**

- **hx_dispatch__fnptr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_dispatch__fnptr | 3.1ns | 2660.8ns | 0.1% |  |
| hx_dispatch__switch | 2.5ns | 1267.5ns | 0.2% |  |

## Distribution (algo ns)

```
hx_dispatch__fnptr (n=6, range 2429.6-2904.8 ns)
   2429.6 |########################################
   2453.4 |
   2477.1 |
   2500.9 |####################
   2524.6 |
   2548.4 |
   2572.2 |
   2595.9 |
   2619.7 |
   2643.4 |
   2667.2 |
   2691.0 |
   2714.7 |
   2738.5 |####################
   2762.2 |
   2786.0 |####################
   2809.8 |
   2833.5 |
   2857.3 |
   2881.0 |
  (0 below, 1 above range)

hx_dispatch__switch (n=6, range 1201.2-1367.5 ns)
   1201.2 |####################
   1209.5 |########################################
   1217.8 |
   1226.1 |
   1234.5 |####################
   1242.8 |
   1251.1 |
   1259.4 |
   1267.7 |
   1276.0 |
   1284.3 |####################
   1292.7 |
   1301.0 |
   1309.3 |
   1317.6 |
   1325.9 |
   1334.2 |
   1342.6 |
   1350.9 |
   1359.2 |
  (0 below, 1 above range)

```

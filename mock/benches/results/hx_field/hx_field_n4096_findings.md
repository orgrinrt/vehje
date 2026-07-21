# Field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache

3 variants, 6 samples per variant.
Baseline: **hx_field__direct**

## Highlights

Baseline for all deltas below: **hx_field__direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_field__direct dominates: 150% faster than the next best (hx_field__hash)

hx_field__direct (1.12 us) leads hx_field__hash (2.81 us) by 150%, a clear separation rather than a photo finish. CV 5.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_field__linear is an outlier: 2.6x slower than the field

hx_field__linear (2.93 us) is 2.6x the fastest (1.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (hx_field__direct)

The baseline hx_field__direct is the fastest (1.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_field__direct) is the fastest** at 1124.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.61x (fastest 1124.2 ns, slowest 2933.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_field__direct | 3569ns | 3696ns | 3118ns | 3619ns | 3721ns | base |
| hx_field__hash | 5245ns | 5410ns | 4449ns | 5113ns | 5842ns | +46.95% |
| hx_field__linear | 5431ns | 5643ns | 4518ns | 5445ns | 5867ns | +52.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_field__direct | 1085ns | 949ns | 1130ns | base | 3.776 |
| hx_field__hash | 2732ns | 2318ns | 3046ns | +151.86% | 1.499 |
| hx_field__linear | 2831ns | 2362ns | 3060ns | +160.99% | 1.447 |

## Performance model

- Peak throughput: **4.317 Gops/s** (hx_field__direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_field__direct | 3.644 | 84.4% |
| hx_field__hash | 1.457 | 33.8% |
| hx_field__linear | 1.396 | 32.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_field__direct | 3569ns | 3569ns | base |
| hx_field__hash | 5245ns | 5245ns | +46.95% |
| hx_field__linear | 5431ns | 5431ns | +52.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_field__direct | 1124ns | base | --- | [1000, 1130] | --- | --- | --- | --- |
| hx_field__hash | 2810ns | +1721.7ns (+153.2%) | [+1303, +1917]ns | [2339, 3046] | YES | 0.0313 | 0.0313 | 0 |
| hx_field__linear | 2934ns | +1842.5ns (+163.9%) | [+1462, +1934]ns | [2498, 3060] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_field__direct | hx_field__hash | hx_field__linear |
|---|---|---|---|
| 1 | 1123ns | +106.3% | +134.5% |
| 2 | 1125ns | +170.7% | +172.1% |
| 3 | 1133ns | +168.8% | +170.0% |
| 4 | 1126ns | +150.3% | +171.5% |
| 5 | 1052ns | +166.5% | +167.2% |
| 6 | 949ns | +148.7% | +149.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_field__direct | 0.323 | moderate+ |
| hx_field__hash | -0.046 | ok |
| hx_field__linear | 0.154 | ok |

**Consistency summary:**

- **hx_field__hash**: won 0/6, lost 6/6
- **hx_field__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_field__direct | 3.2ns | 1084.6ns | 0.3% |  |
| hx_field__hash | 3.1ns | 2731.7ns | 0.1% |  |
| hx_field__linear | 3.8ns | 2830.8ns | 0.1% |  |

## Distribution (algo ns)

```
hx_field__direct (n=6, range 948.7-1129.5 ns)
    948.7 |#############
    957.7 |
    966.8 |
    975.8 |
    984.9 |
    993.9 |
   1003.0 |
   1012.0 |
   1021.0 |
   1030.1 |
   1039.1 |
   1048.2 |#############
   1057.2 |
   1066.3 |
   1075.3 |
   1084.3 |
   1093.4 |
   1102.4 |
   1111.5 |
   1120.5 |########################################
  (0 below, 1 above range)

hx_field__hash (n=6, range 2317.9-3046.2 ns)
   2317.9 |####################
   2354.3 |####################
   2390.7 |
   2427.2 |
   2463.6 |
   2500.0 |
   2536.4 |
   2572.8 |
   2609.2 |
   2645.7 |
   2682.1 |
   2718.5 |
   2754.9 |
   2791.3 |########################################
   2827.7 |
   2864.2 |
   2900.6 |
   2937.0 |
   2973.4 |
   3009.8 |####################
  (0 below, 1 above range)

hx_field__linear (n=6, range 2362.5-3060.4 ns)
   2362.5 |####################
   2397.4 |
   2432.3 |
   2467.2 |
   2502.1 |
   2537.0 |
   2571.9 |
   2606.8 |####################
   2641.7 |
   2676.6 |
   2711.4 |
   2746.3 |
   2781.2 |####################
   2816.1 |
   2851.0 |
   2885.9 |
   2920.8 |
   2955.7 |
   2990.6 |
   3025.5 |########################################
  (0 below, 1 above range)

```

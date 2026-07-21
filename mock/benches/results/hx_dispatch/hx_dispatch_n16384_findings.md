# Interpreter dispatch shapes: switch vs fnptr-table vs computed

2 variants, 6 samples per variant.
Baseline: **hx_dispatch__switch**

## Highlights

Baseline for all deltas below: **hx_dispatch__switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_dispatch__switch dominates: 83% faster than the next best (hx_dispatch__fnptr)

hx_dispatch__switch (66.91 us) leads hx_dispatch__fnptr (122.45 us) by 83%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_dispatch__switch)

The baseline hx_dispatch__switch is the fastest (66.91 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_dispatch__switch) is the fastest** at 66908.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.83x (fastest 66908.9 ns, slowest 122448.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_dispatch__fnptr | 124679ns | 124957ns | 116212ns | 122510ns | 132166ns | +76.33% |
| hx_dispatch__switch | 70707ns | 69365ns | 66540ns | 68991ns | 75366ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_dispatch__fnptr | 122145ns | 113810ns | 129361ns | +78.87% | 0.134 |
| hx_dispatch__switch | 68288ns | 64276ns | 72875ns | base | 0.240 |

## Performance model

- Peak throughput: **0.255 Gops/s** (hx_dispatch__switch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_dispatch__fnptr | 0.134 | 52.5% |
| hx_dispatch__switch | 0.245 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_dispatch__fnptr | 124679ns | 124679ns | +76.33% |
| hx_dispatch__switch | 70707ns | 70707ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_dispatch__switch | 66909ns | base | --- | [65079, 72875] | --- | --- | --- | --- |
| hx_dispatch__fnptr | 122449ns | +54136.5ns (+80.9%) | [+44625, +62810]ns | [114626, 129361] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_dispatch__switch | hx_dispatch__fnptr |
|---|---|---|
| 1 | 74864ns | +52.0% |
| 2 | 65883ns | +96.2% |
| 3 | 66600ns | +75.5% |
| 4 | 64276ns | +79.6% |
| 5 | 67218ns | +92.6% |
| 6 | 70886ns | +80.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_dispatch__fnptr | -0.242 | moderate- |
| hx_dispatch__switch | -0.046 | ok |

**Consistency summary:**

- **hx_dispatch__fnptr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_dispatch__fnptr | 3.2ns | 122145.0ns | 0.0% |  |
| hx_dispatch__switch | 3.1ns | 68287.7ns | 0.0% |  |

## Distribution (algo ns)

```
hx_dispatch__fnptr (n=6, range 113809.6-129360.6 ns)
  113809.6 |########################################
  114587.2 |
  115364.7 |########################################
  116142.3 |########################################
  116919.8 |
  117697.4 |
  118474.9 |
  119252.5 |
  120030.0 |
  120807.6 |
  121585.1 |
  122362.7 |
  123140.2 |
  123917.8 |
  124695.3 |
  125472.9 |
  126250.4 |
  127028.0 |
  127805.5 |########################################
  128583.1 |########################################
  (0 below, 1 above range)

hx_dispatch__switch (n=6, range 64275.8-72874.8 ns)
  64275.8 |########################################
  64705.8 |
  65135.7 |
  65565.7 |########################################
  65995.6 |
  66425.6 |########################################
  66855.5 |########################################
  67285.4 |
  67715.4 |
  68145.4 |
  68575.3 |
  69005.2 |
  69435.2 |
  69865.2 |
  70295.1 |
  70725.1 |########################################
  71155.0 |
  71584.9 |
  72014.9 |
  72444.9 |
  (0 below, 1 above range)

```

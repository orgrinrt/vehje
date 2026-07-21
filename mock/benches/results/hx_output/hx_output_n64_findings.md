# Interpolation output: format-to-temp+copy vs format-in-place

2 variants, 6 samples per variant.
Baseline: **hx_output__inplace**

## Highlights

Baseline for all deltas below: **hx_output__inplace**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_output__inplace is fastest but the noisiest (CV 9.8%)

hx_output__inplace wins on median (130 ns) yet has the highest variance (CV 9.8%), while hx_output__temp is the steadiest (CV 5.5%, 132 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (13 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (hx_output__inplace)

The baseline hx_output__inplace is the fastest (130 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader hx_output__inplace vs stability leader hx_output__temp (+1% speed for 1.8x steadier)

hx_output__inplace is fastest (130 ns, CV 9.8%); hx_output__temp gives up 1.4% median for 1.8x lower variance (CV 5.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (hx_output__inplace) is the fastest** at 130.0 ns median
- Spread: 1.01x (fastest 130.0 ns, slowest 131.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_output__inplace | 2705ns | 2561ns | 2531ns | 2552ns | 3023ns | base |
| hx_output__temp | 2682ns | 2668ns | 2548ns | 2629ns | 2830ns | -0.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_output__inplace | 136ns | 128ns | 150ns | base | 0.469 |
| hx_output__temp | 133ns | 124ns | 142ns | -2.24% | 0.480 |

## Performance model

- Peak throughput: **0.515 Gops/s** (hx_output__temp; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_output__inplace | 0.492 | 95.5% |
| hx_output__temp | 0.485 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_output__inplace | 2705ns | 2705ns | base |
| hx_output__temp | 2682ns | 2682ns | -0.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_output__inplace | 130ns | base | --- | [129, 150] | --- | --- | --- | --- |
| hx_output__temp | 132ns | no significant difference | [-11, +5]ns | [126, 142] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_output__inplace | hx_output__temp |
|---|---|---|
| 1 | 164ns | -10.7% |
| 2 | 130ns | +1.9% |
| 3 | 128ns | -3.2% |
| 4 | 129ns | -1.3% |
| 5 | 136ns | -3.7% |
| 6 | 130ns | +5.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_output__inplace | -0.069 | ok |
| hx_output__temp | 0.163 | ok |

**Consistency summary:**

- **hx_output__temp**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_output__inplace | 3.3ns | 136.3ns | 2.4% |  |
| hx_output__temp | 3.5ns | 133.3ns | 2.6% |  |

## Distribution (algo ns)

```
hx_output__inplace (n=6, range 128.3-150.2 ns)
    128.3 |########################################
    129.4 |########################################
    130.5 |
    131.6 |
    132.7 |
    133.8 |
    134.9 |
    136.0 |####################
    137.1 |
    138.2 |
    139.2 |
    140.3 |
    141.4 |
    142.5 |
    143.6 |
    144.7 |
    145.8 |
    146.9 |
    148.0 |
    149.1 |
  (0 below, 1 above range)

hx_output__temp (n=6, range 124.2-142.1 ns)
    124.2 |########################################
    125.1 |
    126.0 |
    126.9 |########################################
    127.8 |
    128.7 |
    129.6 |
    130.5 |########################################
    131.4 |
    132.3 |########################################
    133.2 |
    134.0 |
    134.9 |
    135.8 |
    136.7 |########################################
    137.6 |
    138.5 |
    139.4 |
    140.3 |
    141.2 |
  (0 below, 1 above range)

```

# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reuse__reuse is fastest but the noisiest (CV 9.2%)

hx_reuse__reuse wins on median (73 ns) yet has the highest variance (CV 9.2%), while hx_reuse__copy is the steadiest (CV 6.5%, 75 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (7 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (73 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader hx_reuse__reuse vs stability leader hx_reuse__copy (+3% speed for 1.4x steadier)

hx_reuse__reuse is fastest (73 ns, CV 9.2%); hx_reuse__copy gives up 2.5% median for 1.4x lower variance (CV 6.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 72.7 ns median
- Spread: 1.03x (fastest 72.7 ns, slowest 74.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 3031ns | 2876ns | 2712ns | 2875ns | 3424ns | +3.43% |
| hx_reuse__reuse | 2930ns | 2872ns | 2308ns | 2868ns | 3335ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 75ns | 70ns | 80ns | +5.55% | 0.856 |
| hx_reuse__reuse | 71ns | 60ns | 78ns | base | 0.904 |

## Performance model

- Peak throughput: **1.074 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 0.858 | 79.9% |
| hx_reuse__reuse | 0.880 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 3031ns | 3031ns | +3.43% |
| hx_reuse__reuse | 2930ns | 2930ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 73ns | base | --- | [62, 78] | --- | --- | --- | --- |
| hx_reuse__copy | 75ns | no significant difference | [-1, +8]ns | [70, 80] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 60ns | +18.1% |
| 2 | 78ns | +4.2% |
| 3 | 78ns | +1.5% |
| 4 | 73ns | +8.0% |
| 5 | 65ns | +7.7% |
| 6 | 72ns | -3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.121 | ok |
| hx_reuse__reuse | -0.157 | ok |

**Consistency summary:**

- **hx_reuse__copy**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 2.9ns | 74.8ns | 3.9% |  |
| hx_reuse__reuse | 2.9ns | 70.8ns | 4.2% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 69.6-80.0 ns)
     69.6 |########################################
     70.1 |####################
     70.6 |
     71.2 |
     71.7 |
     72.2 |
     72.7 |
     73.2 |
     73.7 |
     74.3 |
     74.8 |
     75.3 |
     75.8 |
     76.3 |
     76.8 |
     77.4 |
     77.9 |
     78.4 |########################################
     78.9 |
     79.4 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 59.6-77.7 ns)
     59.6 |####################
     60.5 |
     61.4 |
     62.3 |
     63.2 |
     64.1 |####################
     65.0 |
     65.9 |
     66.8 |
     67.7 |
     68.7 |
     69.6 |
     70.5 |
     71.4 |
     72.3 |########################################
     73.2 |
     74.1 |
     75.0 |
     75.9 |
     76.8 |####################
  (0 below, 1 above range)

```

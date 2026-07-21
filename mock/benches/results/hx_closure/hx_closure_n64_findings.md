# Closure representation: flat-capture vs linked-env

2 variants, 6 samples per variant.
Baseline: **hx_closure__linked**

## Highlights

Baseline for all deltas below: **hx_closure__linked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_closure__linked is fastest but the noisiest (CV 8.5%)

hx_closure__linked wins on median (57 ns) yet has the highest variance (CV 8.5%), while hx_closure__flat is the steadiest (CV 5.2%, 60 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (hx_closure__linked)

The baseline hx_closure__linked is the fastest (57 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader hx_closure__linked vs stability leader hx_closure__flat (+6% speed for 1.6x steadier)

hx_closure__linked is fastest (57 ns, CV 8.5%); hx_closure__flat gives up 5.5% median for 1.6x lower variance (CV 5.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (hx_closure__linked) is the fastest** at 56.9 ns median
- Spread: 1.06x (fastest 56.9 ns, slowest 60.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_closure__flat | 2637ns | 2625ns | 2458ns | 2623ns | 2748ns | -0.13% |
| hx_closure__linked | 2641ns | 2630ns | 2170ns | 2626ns | 2899ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_closure__flat | 60ns | 55ns | 63ns | +2.40% | 1.073 |
| hx_closure__linked | 58ns | 51ns | 64ns | base | 1.099 |

## Performance model

- Peak throughput: **1.260 Gops/s** (hx_closure__linked; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_closure__flat | 1.067 | 84.7% |
| hx_closure__linked | 1.126 | 89.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_closure__flat | 2637ns | 2637ns | -0.13% |
| hx_closure__linked | 2641ns | 2641ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_closure__linked | 57ns | base | --- | [54, 64] | --- | --- | --- | --- |
| hx_closure__flat | 60ns | no significant difference | [-7, +7]ns | [56, 63] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_closure__linked | hx_closure__flat |
|---|---|---|
| 1 | 51ns | +10.6% |
| 2 | 56ns | +9.8% |
| 3 | 64ns | -6.0% |
| 4 | 56ns | +14.9% |
| 5 | 65ns | -14.8% |
| 6 | 58ns | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_closure__flat | -0.461 | moderate- |
| hx_closure__linked | -0.187 | ok |

**Consistency summary:**

- **hx_closure__flat**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_closure__flat | 2.7ns | 59.6ns | 4.5% |  |
| hx_closure__linked | 2.8ns | 58.2ns | 4.8% |  |

## Distribution (algo ns)

```
hx_closure__flat (n=6, range 55.4-63.1 ns)
     55.4 |####################
     55.8 |
     56.2 |####################
     56.6 |
     56.9 |
     57.3 |
     57.7 |
     58.1 |
     58.5 |
     58.9 |
     59.3 |
     59.7 |########################################
     60.0 |
     60.4 |
     60.8 |
     61.2 |
     61.6 |####################
     62.0 |
     62.4 |
     62.8 |
  (0 below, 1 above range)

hx_closure__linked (n=6, range 50.8-64.4 ns)
     50.8 |####################
     51.5 |
     52.2 |
     52.8 |
     53.5 |
     54.2 |
     54.9 |
     55.6 |########################################
     56.2 |
     56.9 |####################
     57.6 |
     58.3 |
     59.0 |
     59.6 |
     60.3 |
     61.0 |
     61.7 |
     62.4 |
     63.0 |
     63.7 |####################
  (0 below, 1 above range)

```

# Name resolution: linear scope-chain vs flat shadow-stack

2 variants, 6 samples per variant.
Baseline: **hx_resolve__flat**

## Highlights

Baseline for all deltas below: **hx_resolve__flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_resolve__flat dominates: 728% faster than the next best (hx_resolve__linear)

hx_resolve__flat (362 ns) leads hx_resolve__linear (3.00 us) by 728%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_resolve__flat is fastest but the noisiest (CV 8.3%)

hx_resolve__flat wins on median (362 ns) yet has the highest variance (CV 8.3%), while hx_resolve__linear is the steadiest (CV 3.9%, 3.00 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_resolve__flat)

The baseline hx_resolve__flat is the fastest (362 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.3x the fastest

Fastest hx_resolve__flat (362 ns) to slowest hx_resolve__linear (3.00 us): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_resolve__flat) is the fastest** at 362.5 ns median
- 1 variant significantly slower than baseline
- Spread: 8.28x (fastest 362.5 ns, slowest 3003.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_resolve__flat | 2910ns | 2942ns | 2430ns | 2879ns | 3198ns | base |
| hx_resolve__linear | 5544ns | 5592ns | 5296ns | 5560ns | 5644ns | +90.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_resolve__flat | 364ns | 310ns | 398ns | base | 0.704 |
| hx_resolve__linear | 3015ns | 2882ns | 3143ns | +728.90% | 0.085 |

## Performance model

- Peak throughput: **0.826 Gops/s** (hx_resolve__flat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_resolve__flat | 0.706 | 85.5% |
| hx_resolve__linear | 0.085 | 10.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_resolve__flat | 2910ns | 2910ns | base |
| hx_resolve__linear | 5544ns | 5544ns | +90.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_resolve__flat | 362ns | base | --- | [330, 398] | --- | --- | --- | --- |
| hx_resolve__linear | 3003ns | +2616.8ns (+721.9%) | [+2541, +2795]ns | [2897, 3143] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_resolve__flat | hx_resolve__linear |
|---|---|---|
| 1 | 310ns | +945.5% |
| 2 | 398ns | +665.4% |
| 3 | 363ns | +732.5% |
| 4 | 399ns | +648.5% |
| 5 | 350ns | +722.4% |
| 6 | 362ns | +704.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_resolve__flat | -0.428 | moderate- |
| hx_resolve__linear | 0.301 | moderate+ |

**Consistency summary:**

- **hx_resolve__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_resolve__flat | 4.7ns | 363.7ns | 1.3% |  |
| hx_resolve__linear | 3.8ns | 3014.6ns | 0.1% |  |

## Distribution (algo ns)

```
hx_resolve__flat (n=6, range 310.0-398.4 ns)
    310.0 |####################
    314.4 |
    318.8 |
    323.3 |
    327.7 |
    332.1 |
    336.5 |
    340.9 |
    345.3 |
    349.8 |####################
    354.2 |
    358.6 |########################################
    363.0 |
    367.4 |
    371.8 |
    376.3 |
    380.7 |
    385.1 |
    389.5 |
    393.9 |####################
  (0 below, 1 above range)

hx_resolve__linear (n=6, range 2881.7-3143.3 ns)
   2881.7 |########################################
   2894.8 |
   2907.9 |########################################
   2920.9 |
   2934.0 |
   2947.1 |
   2960.2 |
   2973.3 |########################################
   2986.3 |
   2999.4 |
   3012.5 |########################################
   3025.6 |
   3038.7 |########################################
   3051.7 |
   3064.8 |
   3077.9 |
   3091.0 |
   3104.1 |
   3117.1 |
   3130.2 |
  (0 below, 1 above range)

```

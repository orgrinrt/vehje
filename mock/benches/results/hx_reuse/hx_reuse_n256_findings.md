# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reuse__reuse dominates: 35% faster than the next best (hx_reuse__copy)

hx_reuse__reuse (150 ns) leads hx_reuse__copy (203 ns) by 35%, a clear separation rather than a photo finish. CV 31.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_reuse__reuse is fastest but the noisiest (CV 31.9%)

hx_reuse__reuse wins on median (150 ns) yet has the highest variance (CV 31.9%), while hx_reuse__copy is the steadiest (CV 8.7%, 203 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_reuse__reuse is inconsistent: worst-20% is 1.5x its best-20%

hx_reuse__reuse's best 20% of batches run at 144 ns but its worst 20% at 217 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 150.2 ns median
- Spread: 1.35x (fastest 150.2 ns, slowest 202.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 2518ns | 2460ns | 2432ns | 2453ns | 2658ns | +2.57% |
| hx_reuse__reuse | 2455ns | 2416ns | 2380ns | 2407ns | 2565ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 205ns | 182ns | 224ns | +20.22% | 1.249 |
| hx_reuse__reuse | 171ns | 144ns | 217ns | base | 1.501 |

## Performance model

- Peak throughput: **1.780 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.262 | 70.9% |
| hx_reuse__reuse | 1.704 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 2518ns | 2518ns | +2.57% |
| hx_reuse__reuse | 2455ns | 2455ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 150ns | base | --- | [144, 217] | --- | --- | --- | --- |
| hx_reuse__copy | 203ns | no significant difference | [-15, +71]ns | [188, 224] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 277ns | -24.8% |
| 2 | 154ns | +26.5% |
| 3 | 144ns | +42.2% |
| 4 | 147ns | +36.9% |
| 5 | 144ns | +26.4% |
| 6 | 158ns | +52.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | -0.396 | moderate- |
| hx_reuse__reuse | 0.020 | ok |

**Consistency summary:**

- **hx_reuse__copy**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.5ns | 205.0ns | 1.7% |  |
| hx_reuse__reuse | 3.3ns | 170.5ns | 1.9% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 181.7-223.9 ns)
    181.7 |########################################
    183.8 |
    185.9 |
    188.0 |
    190.1 |
    192.3 |
    194.4 |########################################
    196.5 |
    198.6 |
    200.7 |########################################
    202.8 |
    204.9 |########################################
    207.0 |########################################
    209.2 |
    211.3 |
    213.4 |
    215.5 |
    217.6 |
    219.7 |
    221.8 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 143.8-217.3 ns)
    143.8 |########################################
    147.5 |
    151.2 |#############
    154.8 |#############
    158.5 |
    162.2 |
    165.9 |
    169.5 |
    173.2 |
    176.9 |
    180.6 |
    184.2 |
    187.9 |
    191.6 |
    195.2 |
    198.9 |
    202.6 |
    206.3 |
    209.9 |
    213.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **hx_reuse__reuse**: CV=28.1% (high variance, measurements may be unstable)

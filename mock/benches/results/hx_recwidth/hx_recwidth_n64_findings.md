# Node record width: 16B pool-spill vs 24B inline operands

2 variants, 6 samples per variant.
Baseline: **hx_recwidth__rec24**

## Highlights

Baseline for all deltas below: **hx_recwidth__rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_recwidth__rec24 dominates: 70% faster than the next best (hx_recwidth__rec16)

hx_recwidth__rec24 (112 ns) leads hx_recwidth__rec16 (192 ns) by 70%, a clear separation rather than a photo finish. CV 17.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_recwidth__rec24 is fastest but the noisiest (CV 17.9%)

hx_recwidth__rec24 wins on median (112 ns) yet has the highest variance (CV 17.9%), while hx_recwidth__rec16 is the steadiest (CV 11.1%, 192 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_recwidth__rec24)

The baseline hx_recwidth__rec24 is the fastest (112 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_recwidth__rec24 is inconsistent: worst-20% is 1.5x its best-20%

hx_recwidth__rec24's best 20% of batches run at 95 ns but its worst 20% at 146 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (hx_recwidth__rec24) is the fastest** at 112.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.70x (fastest 112.5 ns, slowest 191.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_recwidth__rec16 | 3117ns | 3120ns | 2743ns | 2998ns | 3481ns | +2.31% |
| hx_recwidth__rec24 | 3046ns | 2804ns | 2316ns | 2764ns | 3834ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_recwidth__rec16 | 192ns | 165ns | 216ns | +60.20% | 0.334 |
| hx_recwidth__rec24 | 120ns | 95ns | 146ns | base | 0.535 |

## Performance model

- Peak throughput: **0.671 Gops/s** (hx_recwidth__rec24; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_recwidth__rec16 | 0.334 | 49.8% |
| hx_recwidth__rec24 | 0.569 | 84.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_recwidth__rec16 | 3117ns | 3117ns | +2.31% |
| hx_recwidth__rec24 | 3046ns | 3046ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_recwidth__rec24 | 112ns | base | --- | [101, 146] | --- | --- | --- | --- |
| hx_recwidth__rec16 | 192ns | +58.5ns (+52.0%) | [+51, +106]ns | [168, 216] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_recwidth__rec24 | hx_recwidth__rec16 |
|---|---|---|
| 1 | 95ns | +139.3% |
| 2 | 156ns | +30.7% |
| 3 | 135ns | +45.6% |
| 4 | 115ns | +47.7% |
| 5 | 110ns | +50.5% |
| 6 | 107ns | +74.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_recwidth__rec16 | 0.409 | moderate+ |
| hx_recwidth__rec24 | -0.084 | ok |

**Consistency summary:**

- **hx_recwidth__rec16**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_recwidth__rec16 | 4.4ns | 191.8ns | 2.3% |  |
| hx_recwidth__rec24 | 2.2ns | 119.7ns | 1.8% |  |

## Distribution (algo ns)

```
hx_recwidth__rec16 (n=6, range 165.0-216.0 ns)
    165.0 |########################################
    167.6 |
    170.1 |########################################
    172.7 |
    175.2 |
    177.8 |
    180.3 |
    182.8 |
    185.4 |########################################
    187.9 |
    190.5 |
    193.1 |
    195.6 |########################################
    198.2 |
    200.7 |
    203.2 |########################################
    205.8 |
    208.3 |
    210.9 |
    213.4 |
  (0 below, 1 above range)

hx_recwidth__rec24 (n=6, range 95.4-145.6 ns)
     95.4 |########################################
     97.9 |
    100.4 |
    102.9 |
    105.4 |########################################
    108.0 |########################################
    110.5 |
    113.0 |########################################
    115.5 |
    118.0 |
    120.5 |
    123.0 |
    125.5 |
    128.0 |
    130.5 |
    133.1 |########################################
    135.6 |
    138.1 |
    140.6 |
    143.1 |
  (0 below, 1 above range)

```

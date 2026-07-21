# Effect lattice: thermometer-OR join vs per-lane branch max

2 variants, 6 samples per variant.
Baseline: **hx_effect__thermo**

## Highlights

Baseline for all deltas below: **hx_effect__thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_effect__thermo dominates: 30% faster than the next best (hx_effect__branchmax)

hx_effect__thermo (95 ns) leads hx_effect__branchmax (123 ns) by 30%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_effect__thermo)

The baseline hx_effect__thermo is the fastest (95 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_effect__branchmax is inconsistent: worst-20% is 1.6x its best-20%

hx_effect__branchmax's best 20% of batches run at 88 ns but its worst 20% at 140 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (hx_effect__thermo) is the fastest** at 94.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.30x (fastest 94.8 ns, slowest 123.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_effect__branchmax | 2790ns | 2910ns | 2221ns | 2876ns | 2944ns | +1.43% |
| hx_effect__thermo | 2750ns | 2776ns | 2491ns | 2738ns | 2899ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_effect__branchmax | 122ns | 88ns | 140ns | +30.27% | 0.523 |
| hx_effect__thermo | 94ns | 85ns | 100ns | base | 0.681 |

## Performance model

- Peak throughput: **0.749 Gops/s** (hx_effect__thermo; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_effect__branchmax | 0.519 | 69.2% |
| hx_effect__thermo | 0.675 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_effect__branchmax | 2790ns | 2790ns | +1.43% |
| hx_effect__thermo | 2750ns | 2750ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_effect__thermo | 95ns | base | --- | [88, 100] | --- | --- | --- | --- |
| hx_effect__branchmax | 123ns | +31.9ns (+33.7%) | [+13, +40]ns | [104, 140] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_effect__thermo | hx_effect__branchmax |
|---|---|---|
| 1 | 85ns | +3.4% |
| 2 | 93ns | +33.5% |
| 3 | 90ns | +36.3% |
| 4 | 103ns | +32.8% |
| 5 | 96ns | +24.7% |
| 6 | 96ns | +48.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_effect__branchmax | -0.091 | ok |
| hx_effect__thermo | -0.029 | ok |

**Consistency summary:**

- **hx_effect__branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_effect__branchmax | 2.8ns | 122.4ns | 2.3% |  |
| hx_effect__thermo | 3.1ns | 93.9ns | 3.3% |  |

## Distribution (algo ns)

```
hx_effect__branchmax (n=6, range 88.3-139.6 ns)
     88.3 |########################################
     90.9 |
     93.4 |
     96.0 |
     98.6 |
    101.1 |
    103.7 |
    106.3 |
    108.8 |
    111.4 |
    113.9 |
    116.5 |
    119.1 |########################################
    121.6 |########################################
    124.2 |########################################
    126.8 |
    129.3 |
    131.9 |
    134.5 |########################################
    137.0 |
  (0 below, 1 above range)

hx_effect__thermo (n=6, range 85.4-99.6 ns)
     85.4 |####################
     86.1 |
     86.8 |
     87.5 |
     88.2 |
     88.9 |####################
     89.6 |
     90.4 |
     91.1 |
     91.8 |
     92.5 |
     93.2 |####################
     93.9 |
     94.6 |
     95.3 |
     96.0 |########################################
     96.7 |
     97.4 |
     98.1 |
     98.8 |
  (0 below, 1 above range)

```

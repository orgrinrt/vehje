# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reuse__reuse dominates: 16% faster than the next best (hx_reuse__copy)

hx_reuse__reuse (56 ns) leads hx_reuse__copy (65 ns) by 16%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (56 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 56.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 56.5 ns, slowest 65.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 2399ns | 2309ns | 2298ns | 2308ns | 2584ns | +2.11% |
| hx_reuse__reuse | 2349ns | 2298ns | 2221ns | 2296ns | 2493ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 67ns | 61ns | 74ns | +15.34% | 0.960 |
| hx_reuse__reuse | 58ns | 54ns | 62ns | base | 1.108 |

## Performance model

- Peak throughput: **1.190 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 0.982 | 82.5% |
| hx_reuse__reuse | 1.134 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 2399ns | 2399ns | +2.11% |
| hx_reuse__reuse | 2349ns | 2349ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 56ns | base | --- | [55, 62] | --- | --- | --- | --- |
| hx_reuse__copy | 65ns | +8.7ns (+15.4%) | [+4, +14]ns | [61, 74] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 60ns | +2.7% |
| 2 | 56ns | +20.3% |
| 3 | 65ns | +16.1% |
| 4 | 56ns | +29.0% |
| 5 | 57ns | +11.6% |
| 6 | 54ns | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.280 | moderate+ |
| hx_reuse__reuse | -0.299 | moderate- |

**Consistency summary:**

- **hx_reuse__copy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.0ns | 66.6ns | 4.5% |  |
| hx_reuse__reuse | 2.8ns | 57.8ns | 4.9% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 60.8-73.8 ns)
     60.8 |########################################
     61.4 |
     62.1 |
     62.7 |####################
     63.4 |
     64.0 |
     64.7 |
     65.3 |
     66.0 |
     66.6 |####################
     67.3 |
     67.9 |
     68.6 |
     69.2 |
     69.9 |
     70.5 |
     71.2 |
     71.8 |
     72.5 |####################
     73.1 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 53.8-62.1 ns)
     53.8 |########################################
     54.2 |
     54.6 |
     55.0 |
     55.5 |########################################
     55.9 |########################################
     56.3 |########################################
     56.7 |
     57.1 |
     57.5 |
     57.9 |
     58.4 |
     58.8 |
     59.2 |########################################
     59.6 |
     60.0 |
     60.4 |
     60.9 |
     61.3 |
     61.7 |
  (0 below, 1 above range)

```

# Reachability fixpoint: whole-column OR vs delta semi-naive

2 variants, 6 samples per variant.
Baseline: **hx_reach__whole**

## Highlights

Baseline for all deltas below: **hx_reach__whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reach__whole dominates: 31% faster than the next best (hx_reach__delta)

hx_reach__whole (174 ns) leads hx_reach__delta (228 ns) by 31%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_reach__whole)

The baseline hx_reach__whole is the fastest (174 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reach__whole) is the fastest** at 174.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.31x (fastest 174.2 ns, slowest 228.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reach__delta | 2481ns | 2431ns | 2340ns | 2415ns | 2651ns | +3.25% |
| hx_reach__whole | 2403ns | 2371ns | 2326ns | 2357ns | 2511ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reach__delta | 230ns | 215ns | 246ns | +30.06% | 17.797 |
| hx_reach__whole | 177ns | 172ns | 185ns | base | 23.148 |

## Performance model

- Peak throughput: **23.856 Gops/s** (hx_reach__whole; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reach__delta | 17.953 | 75.3% |
| hx_reach__whole | 23.513 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reach__delta | 2481ns | 2481ns | +3.25% |
| hx_reach__whole | 2403ns | 2403ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reach__whole | 174ns | base | --- | [172, 185] | --- | --- | --- | --- |
| hx_reach__delta | 228ns | +53.9ns (+31.0%) | [+43, +63]ns | [216, 246] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reach__whole | hx_reach__delta |
|---|---|---|
| 1 | 175ns | +22.9% |
| 2 | 193ns | +35.6% |
| 3 | 172ns | +33.1% |
| 4 | 172ns | +26.4% |
| 5 | 176ns | +29.0% |
| 6 | 174ns | +32.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reach__delta | -0.321 | moderate- |
| hx_reach__whole | -0.236 | moderate- |

**Consistency summary:**

- **hx_reach__delta**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reach__delta | 3.1ns | 230.2ns | 1.4% |  |
| hx_reach__whole | 3.6ns | 177.0ns | 2.0% |  |

## Distribution (algo ns)

```
hx_reach__delta (n=6, range 214.6-246.5 ns)
    214.6 |########################################
    216.2 |########################################
    217.8 |
    219.4 |
    221.0 |
    222.6 |
    224.2 |
    225.7 |########################################
    227.3 |
    228.9 |########################################
    230.5 |########################################
    232.1 |
    233.7 |
    235.3 |
    236.9 |
    238.5 |
    240.1 |
    241.7 |
    243.3 |
    244.9 |
  (0 below, 1 above range)

hx_reach__whole (n=6, range 171.7-184.6 ns)
    171.7 |########################################
    172.3 |########################################
    173.0 |
    173.6 |########################################
    174.3 |########################################
    174.9 |
    175.6 |########################################
    176.2 |
    176.8 |
    177.5 |
    178.1 |
    178.8 |
    179.4 |
    180.1 |
    180.7 |
    181.3 |
    182.0 |
    182.6 |
    183.3 |
    183.9 |
  (0 below, 1 above range)

```

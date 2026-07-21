# Reachability fixpoint: whole-column OR vs delta semi-naive

2 variants, 6 samples per variant.
Baseline: **hx_reach__whole**

## Highlights

Baseline for all deltas below: **hx_reach__whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reach__whole dominates: 36% faster than the next best (hx_reach__delta)

hx_reach__whole (196 ns) leads hx_reach__delta (267 ns) by 36%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_reach__whole)

The baseline hx_reach__whole is the fastest (196 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### hx_reach__delta is inconsistent: worst-20% is 2.7x its best-20%

hx_reach__delta's best 20% of batches run at 220 ns but its worst 20% at 593 ns (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (hx_reach__whole) is the fastest** at 195.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.36x (fastest 195.9 ns, slowest 266.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reach__delta | 3138ns | 2834ns | 2343ns | 2770ns | 4089ns | +17.14% |
| hx_reach__whole | 2679ns | 2681ns | 2303ns | 2651ns | 2908ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reach__delta | 363ns | 220ns | 593ns | +83.54% | 0.176 |
| hx_reach__whole | 198ns | 175ns | 216ns | base | 0.323 |

## Performance model

- Peak throughput: **0.367 Gops/s** (hx_reach__whole; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reach__delta | 0.240 | 65.5% |
| hx_reach__whole | 0.327 | 89.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reach__delta | 3138ns | 3138ns | +17.14% |
| hx_reach__whole | 2679ns | 2679ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reach__whole | 196ns | base | --- | [182, 216] | --- | --- | --- | --- |
| hx_reach__delta | 267ns | +55.0ns (+28.1%) | [+38, +402]ns | [230, 593] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reach__whole | hx_reach__delta |
|---|---|---|
| 1 | 189ns | +16.5% |
| 2 | 198ns | +32.2% |
| 3 | 231ns | +20.1% |
| 4 | 175ns | +420.4% |
| 5 | 194ns | +23.6% |
| 6 | 201ns | +35.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reach__delta | -0.222 | moderate- |
| hx_reach__whole | -0.394 | moderate- |

**Consistency summary:**

- **hx_reach__delta**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reach__delta | 3.9ns | 363.1ns | 1.1% |  |
| hx_reach__whole | 4.4ns | 197.8ns | 2.2% |  |

## Distribution (algo ns)

```
hx_reach__delta (n=6, range 220.0-592.9 ns)
    220.0 |####################
    238.6 |####################
    257.3 |########################################
    275.9 |####################
    294.6 |
    313.2 |
    331.9 |
    350.5 |
    369.2 |
    387.8 |
    406.5 |
    425.1 |
    443.7 |
    462.4 |
    481.0 |
    499.7 |
    518.3 |
    537.0 |
    555.6 |
    574.3 |
  (0 below, 1 above range)

hx_reach__whole (n=6, range 174.6-216.0 ns)
    174.6 |########################################
    176.7 |
    178.7 |
    180.8 |
    182.9 |
    184.9 |
    187.0 |########################################
    189.1 |
    191.2 |
    193.2 |########################################
    195.3 |
    197.4 |########################################
    199.4 |########################################
    201.5 |
    203.6 |
    205.7 |
    207.7 |
    209.8 |
    211.9 |
    213.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **hx_reach__delta**: CV=67.4% (high variance, measurements may be unstable)

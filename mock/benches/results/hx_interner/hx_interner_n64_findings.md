# Interner intern hot path: FNV vs FxHash

2 variants, 6 samples per variant.
Baseline: **hx_interner__fnv**

## Highlights

Baseline for all deltas below: **hx_interner__fnv**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_interner__fnv dominates: 20% faster than the next best (hx_interner__fx)

hx_interner__fnv (405 ns) leads hx_interner__fx (486 ns) by 20%, a clear separation rather than a photo finish. CV 9.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_interner__fnv is fastest but the noisiest (CV 9.9%)

hx_interner__fnv wins on median (405 ns) yet has the highest variance (CV 9.9%), while hx_interner__fx is the steadiest (CV 9.2%, 486 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_interner__fnv)

The baseline hx_interner__fnv is the fastest (405 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_interner__fnv) is the fastest** at 404.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 404.8 ns, slowest 485.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_interner__fnv | 2792ns | 2813ns | 2534ns | 2734ns | 3009ns | base |
| hx_interner__fx | 2901ns | 2906ns | 2603ns | 2818ns | 3175ns | +3.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_interner__fnv | 409ns | 366ns | 456ns | base | 0.156 |
| hx_interner__fx | 493ns | 446ns | 545ns | +20.39% | 0.130 |

## Performance model

- Peak throughput: **0.175 Gops/s** (hx_interner__fnv; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_interner__fnv | 0.158 | 90.5% |
| hx_interner__fx | 0.132 | 75.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_interner__fnv | 2792ns | 2792ns | base |
| hx_interner__fx | 2901ns | 2901ns | +3.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_interner__fnv | 405ns | base | --- | [367, 456] | --- | --- | --- | --- |
| hx_interner__fx | 486ns | +79.8ns (+19.7%) | [+52, +118]ns | [447, 545] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_interner__fnv | hx_interner__fx |
|---|---|---|
| 1 | 410ns | +10.5% |
| 2 | 399ns | +29.7% |
| 3 | 486ns | +12.8% |
| 4 | 425ns | +27.6% |
| 5 | 366ns | +21.8% |
| 6 | 369ns | +21.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_interner__fnv | 0.155 | ok |
| hx_interner__fx | 0.244 | moderate+ |

**Consistency summary:**

- **hx_interner__fx**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_interner__fnv | 3.2ns | 409.3ns | 0.8% |  |
| hx_interner__fx | 2.8ns | 492.8ns | 0.6% |  |

## Distribution (algo ns)

```
hx_interner__fnv (n=6, range 366.2-455.6 ns)
    366.2 |########################################
    370.7 |
    375.1 |
    379.6 |
    384.1 |
    388.6 |
    393.0 |
    397.5 |####################
    402.0 |
    406.4 |####################
    410.9 |
    415.4 |
    419.8 |
    424.3 |####################
    428.8 |
    433.2 |
    437.7 |
    442.2 |
    446.7 |
    451.1 |
  (0 below, 1 above range)

hx_interner__fx (n=6, range 446.2-545.4 ns)
    446.2 |########################################
    451.2 |####################
    456.1 |
    461.1 |
    466.0 |
    471.0 |
    476.0 |
    480.9 |
    485.9 |
    490.8 |
    495.8 |
    500.8 |
    505.7 |
    510.7 |
    515.6 |####################
    520.6 |
    525.6 |
    530.5 |
    535.5 |
    540.4 |####################
  (0 below, 1 above range)

```

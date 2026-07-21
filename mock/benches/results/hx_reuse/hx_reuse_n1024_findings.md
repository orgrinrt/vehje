# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reuse__reuse dominates: 34% faster than the next best (hx_reuse__copy)

hx_reuse__reuse (601 ns) leads hx_reuse__copy (807 ns) by 34%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_reuse__reuse is fastest but the noisiest (CV 7.2%)

hx_reuse__reuse wins on median (601 ns) yet has the highest variance (CV 7.2%), while hx_reuse__copy is the steadiest (CV 2.4%, 807 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (601 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 600.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.34x (fastest 600.6 ns, slowest 806.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 3351ns | 3380ns | 3186ns | 3375ns | 3398ns | +8.31% |
| hx_reuse__reuse | 3094ns | 3175ns | 2650ns | 3105ns | 3300ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 804ns | 767ns | 823ns | +37.07% | 1.274 |
| hx_reuse__reuse | 587ns | 505ns | 625ns | base | 1.746 |

## Performance model

- Peak throughput: **2.028 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.269 | 62.6% |
| hx_reuse__reuse | 1.705 | 84.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 3351ns | 3351ns | +8.31% |
| hx_reuse__reuse | 3094ns | 3094ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 601ns | base | --- | [534, 625] | --- | --- | --- | --- |
| hx_reuse__copy | 807ns | +220.4ns (+36.7%) | [+179, +253]ns | [782, 823] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 505ns | +51.9% |
| 2 | 563ns | +43.2% |
| 3 | 607ns | +37.1% |
| 4 | 598ns | +36.0% |
| 5 | 643ns | +24.0% |
| 6 | 603ns | +33.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.069 | ok |
| hx_reuse__reuse | 0.291 | moderate+ |

**Consistency summary:**

- **hx_reuse__copy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.5ns | 803.9ns | 0.4% |  |
| hx_reuse__reuse | 3.5ns | 586.5ns | 0.6% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 767.1-822.9 ns)
    767.1 |####################
    769.9 |
    772.7 |
    775.5 |
    778.3 |
    781.0 |
    783.8 |
    786.6 |
    789.4 |
    792.2 |
    795.0 |####################
    797.8 |
    800.6 |
    803.4 |
    806.2 |########################################
    809.0 |
    811.7 |####################
    814.5 |
    817.3 |
    820.1 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 505.0-625.0 ns)
    505.0 |########################################
    511.0 |
    517.0 |
    523.0 |
    529.0 |
    535.0 |
    541.0 |
    547.0 |
    553.0 |
    559.0 |########################################
    565.0 |
    571.0 |
    577.0 |
    583.0 |
    589.0 |
    595.0 |########################################
    601.0 |########################################
    607.0 |########################################
    613.0 |
    619.0 |
  (0 below, 1 above range)

```

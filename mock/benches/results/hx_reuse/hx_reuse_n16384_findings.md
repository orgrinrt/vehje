# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reuse__reuse dominates: 37% faster than the next best (hx_reuse__copy)

hx_reuse__reuse (8.80 us) leads hx_reuse__copy (12.07 us) by 37%, a clear separation rather than a photo finish. CV 7.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_reuse__reuse is fastest but the noisiest (CV 7.4%)

hx_reuse__reuse wins on median (8.80 us) yet has the highest variance (CV 7.4%), while hx_reuse__copy is the steadiest (CV 7.3%, 12.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_reuse__reuse shows warm-up / thermal drift (autocorr +0.55)

hx_reuse__reuse's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (8.80 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 8801.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.37x (fastest 8801.9 ns, slowest 12070.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 14044ns | 14598ns | 12527ns | 13937ns | 14964ns | +28.40% |
| hx_reuse__reuse | 10938ns | 11320ns | 9629ns | 10875ns | 11688ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 11609ns | 10346ns | 12372ns | +36.39% | 1.411 |
| hx_reuse__reuse | 8512ns | 7504ns | 9093ns | base | 1.925 |

## Performance model

- Peak throughput: **2.183 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.357 | 62.2% |
| hx_reuse__reuse | 1.861 | 85.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 14044ns | 14044ns | +28.40% |
| hx_reuse__reuse | 10938ns | 10938ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 8802ns | base | --- | [7640, 9093] | --- | --- | --- | --- |
| hx_reuse__copy | 12071ns | +2977.5ns (+33.8%) | [+2745, +3571]ns | [10385, 12372] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 7504ns | +37.9% |
| 2 | 7775ns | +34.1% |
| 3 | 8559ns | +44.7% |
| 4 | 9045ns | +36.7% |
| 5 | 9093ns | +31.6% |
| 6 | 9093ns | +33.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | 0.354 | moderate+ |
| hx_reuse__reuse | 0.548 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **hx_reuse__copy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 3.2ns | 11609.2ns | 0.0% |  |
| hx_reuse__reuse | 3.2ns | 8511.5ns | 0.0% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 10346.2-12372.5 ns)
  10346.2 |########################################
  10447.5 |
  10548.8 |
  10650.1 |
  10751.5 |
  10852.8 |
  10954.1 |
  11055.4 |
  11156.7 |
  11258.0 |
  11359.4 |
  11460.7 |
  11562.0 |
  11663.3 |
  11764.6 |
  11865.9 |####################
  11967.2 |
  12068.6 |
  12169.9 |####################
  12271.2 |####################
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 7504.2-9093.1 ns)
   7504.2 |####################
   7583.6 |
   7663.1 |
   7742.5 |####################
   7822.0 |
   7901.4 |
   7980.9 |
   8060.3 |
   8139.8 |
   8219.2 |
   8298.6 |
   8378.1 |
   8457.5 |
   8537.0 |####################
   8616.4 |
   8695.9 |
   8775.3 |
   8854.8 |
   8934.2 |
   9013.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **hx_reuse__reuse**: autocorrelation=0.55 (measurement drift or warm-up artifact)

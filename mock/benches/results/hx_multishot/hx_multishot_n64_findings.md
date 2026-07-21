# Bounded multi-shot enumeration vs single-shot

2 variants, 6 samples per variant.
Baseline: **hx_multishot__single**

## Highlights

Baseline for all deltas below: **hx_multishot__single**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_multishot__single dominates: 1354% faster than the next best (hx_multishot__multishot)

hx_multishot__single (5 ns) leads hx_multishot__multishot (67 ns) by 1354%, a clear separation rather than a photo finish. CV 45.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_multishot__single is fastest but the noisiest (CV 45.1%)

hx_multishot__single wins on median (5 ns) yet has the highest variance (CV 45.1%), while hx_multishot__multishot is the steadiest (CV 15.1%, 67 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_multishot__single)

The baseline hx_multishot__single is the fastest (5 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 14.5x the fastest

Fastest hx_multishot__single (5 ns) to slowest hx_multishot__multishot (67 ns): 14.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hx_multishot__single is inconsistent: worst-20% is 3.5x its best-20%

hx_multishot__single's best 20% of batches run at 2 ns but its worst 20% at 7 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (hx_multishot__single) is the fastest** at 4.6 ns median
- 1 variant significantly slower than baseline
- Spread: 14.54x (fastest 4.6 ns, slowest 66.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_multishot__multishot | 2861ns | 2887ns | 2290ns | 2748ns | 3317ns | -4.71% |
| hx_multishot__single | 3002ns | 2954ns | 2603ns | 2838ns | 3448ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_multishot__multishot | 66ns | 52ns | 78ns | +1243.24% | 0.966 |
| hx_multishot__single | 5ns | 2ns | 7ns | base | 12.973 |

## Performance model

- Peak throughput: **30.476 Gops/s** (hx_multishot__single; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_multishot__multishot | 0.957 | 3.1% |
| hx_multishot__single | 13.913 | 45.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_multishot__multishot | 2861ns | 2861ns | -4.71% |
| hx_multishot__single | 3002ns | 3002ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_multishot__single | 5ns | base | --- | [3, 7] | --- | --- | --- | --- |
| hx_multishot__multishot | 67ns | +63.3ns (+1377.2%) | [+48, +72]ns | [54, 78] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_multishot__single | hx_multishot__multishot |
|---|---|---|
| 1 | 9ns | +487.5% |
| 2 | 6ns | +1272.4% |
| 3 | 5ns | +1342.0% |
| 4 | 4ns | +1695.2% |
| 5 | 4ns | +1443.2% |
| 6 | 2ns | +2838.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_multishot__multishot | -0.172 | ok |
| hx_multishot__single | 0.301 | moderate+ |

**Consistency summary:**

- **hx_multishot__multishot**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_multishot__multishot | 3.2ns | 66.3ns | 4.9% |  |
| hx_multishot__single | 3.8ns | 4.9ns | 76.0% | HIGH |

## Distribution (algo ns)

```
hx_multishot__multishot (n=6, range 51.7-77.5 ns)
     51.7 |########################################
     53.0 |
     54.3 |
     55.6 |
     56.9 |########################################
     58.2 |
     59.4 |
     60.7 |########################################
     62.0 |
     63.3 |
     64.6 |
     65.9 |
     67.2 |
     68.5 |
     69.8 |
     71.0 |########################################
     72.3 |
     73.6 |
     74.9 |########################################
     76.2 |
  (0 below, 1 above range)

hx_multishot__single (n=6, range 2.1-7.3 ns)
      2.1 |########################################
      2.4 |
      2.6 |
      2.9 |
      3.1 |
      3.4 |
      3.7 |########################################
      3.9 |
      4.2 |########################################
      4.4 |
      4.7 |
      5.0 |########################################
      5.2 |
      5.5 |
      5.7 |########################################
      6.0 |
      6.3 |
      6.5 |
      6.8 |
      7.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **hx_multishot__single**: CV=42.0% (high variance, measurements may be unstable)
- **hx_multishot__single**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **hx_multishot__single**: bridge=81.5% of algo (FFI overhead may distort results)

# Bounded multi-shot enumeration vs single-shot

2 variants, 6 samples per variant.
Baseline: **hx_multishot__single**

## Highlights

Baseline for all deltas below: **hx_multishot__single**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_multishot__single dominates: 454% faster than the next best (hx_multishot__multishot)

hx_multishot__single (490 ns) leads hx_multishot__multishot (2.71 us) by 454%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_multishot__single is fastest but the noisiest (CV 7.0%)

hx_multishot__single wins on median (490 ns) yet has the highest variance (CV 7.0%), while hx_multishot__multishot is the steadiest (CV 6.9%, 2.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_multishot__single)

The baseline hx_multishot__single is the fastest (490 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.5x the fastest

Fastest hx_multishot__single (490 ns) to slowest hx_multishot__multishot (2.71 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_multishot__single) is the fastest** at 489.8 ns median
- 1 variant significantly slower than baseline
- Spread: 5.54x (fastest 489.8 ns, slowest 2712.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_multishot__multishot | 5061ns | 4916ns | 4735ns | 4888ns | 5485ns | +80.68% |
| hx_multishot__single | 2801ns | 2732ns | 2650ns | 2708ns | 3017ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_multishot__multishot | 2793ns | 2616ns | 3024ns | +450.01% | 1.467 |
| hx_multishot__single | 508ns | 485ns | 548ns | base | 8.067 |

## Performance model

- Peak throughput: **8.452 Gops/s** (hx_multishot__single; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_multishot__multishot | 1.510 | 17.9% |
| hx_multishot__single | 8.363 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_multishot__multishot | 5061ns | 5061ns | +80.68% |
| hx_multishot__single | 2801ns | 2801ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_multishot__single | 490ns | base | --- | [486, 548] | --- | --- | --- | --- |
| hx_multishot__multishot | 2712ns | +2225.8ns (+454.5%) | [+2140, +2489]ns | [2642, 3024] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_multishot__single | hx_multishot__multishot |
|---|---|---|
| 1 | 514ns | +408.8% |
| 2 | 581ns | +445.6% |
| 3 | 488ns | +490.4% |
| 4 | 485ns | +451.1% |
| 5 | 488ns | +464.0% |
| 6 | 491ns | +443.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_multishot__multishot | -0.166 | ok |
| hx_multishot__single | 0.033 | ok |

**Consistency summary:**

- **hx_multishot__multishot**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_multishot__multishot | 3.9ns | 2792.8ns | 0.1% |  |
| hx_multishot__single | 2.8ns | 507.8ns | 0.5% |  |

## Distribution (algo ns)

```
hx_multishot__multishot (n=6, range 2616.2-3023.5 ns)
   2616.2 |####################
   2636.6 |
   2656.9 |########################################
   2677.3 |
   2697.7 |
   2718.0 |
   2738.4 |####################
   2758.8 |
   2779.1 |
   2799.5 |
   2819.8 |
   2840.2 |
   2860.6 |####################
   2880.9 |
   2901.3 |
   2921.7 |
   2942.0 |
   2962.4 |
   2982.8 |
   3003.1 |
  (0 below, 1 above range)

hx_multishot__single (n=6, range 484.6-547.5 ns)
    484.6 |########################################
    487.7 |####################
    490.9 |####################
    494.0 |
    497.2 |
    500.3 |
    503.5 |
    506.6 |
    509.8 |
    512.9 |####################
    516.0 |
    519.2 |
    522.3 |
    525.5 |
    528.6 |
    531.8 |
    534.9 |
    538.1 |
    541.2 |
    544.4 |
  (0 below, 1 above range)

```

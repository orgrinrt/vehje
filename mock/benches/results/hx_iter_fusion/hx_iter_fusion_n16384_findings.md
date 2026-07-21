# Iterator fusion: materialized intermediates vs fused pipeline

2 variants, 6 samples per variant.
Baseline: **hx_iter_fusion__fused**

## Highlights

Baseline for all deltas below: **hx_iter_fusion__fused**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_iter_fusion__fused dominates: 1054% faster than the next best (hx_iter_fusion__materialized)

hx_iter_fusion__fused (4.12 us) leads hx_iter_fusion__materialized (47.52 us) by 1054%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_iter_fusion__fused is fastest but the noisiest (CV 5.5%)

hx_iter_fusion__fused wins on median (4.12 us) yet has the highest variance (CV 5.5%), while hx_iter_fusion__materialized is the steadiest (CV 4.6%, 47.52 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_iter_fusion__fused)

The baseline hx_iter_fusion__fused is the fastest (4.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 11.5x the fastest

Fastest hx_iter_fusion__fused (4.12 us) to slowest hx_iter_fusion__materialized (47.52 us): 11.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_iter_fusion__fused) is the fastest** at 4116.6 ns median
- 1 variant significantly slower than baseline
- Spread: 11.54x (fastest 4116.6 ns, slowest 47523.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 6483ns | 6528ns | 5780ns | 6527ns | 6768ns | base |
| hx_iter_fusion__materialized | 49511ns | 50131ns | 45646ns | 49398ns | 51614ns | +663.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_iter_fusion__fused | 4086ns | 3642ns | 4262ns | base | 4.010 |
| hx_iter_fusion__materialized | 46900ns | 43028ns | 49005ns | +1047.82% | 0.349 |

## Performance model

- Peak throughput: **4.499 Gops/s** (hx_iter_fusion__fused; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_iter_fusion__fused | 3.980 | 88.5% |
| hx_iter_fusion__materialized | 0.345 | 7.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_iter_fusion__fused | 6483ns | 6483ns | base |
| hx_iter_fusion__materialized | 49511ns | 49511ns | +663.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 4117ns | base | --- | [3879, 4262] | --- | --- | --- | --- |
| hx_iter_fusion__materialized | 47523ns | +43404.6ns (+1054.4%) | [+40292, +44744]ns | [44171, 49005] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_iter_fusion__fused | hx_iter_fusion__materialized |
|---|---|---|
| 1 | 3642ns | +1081.4% |
| 2 | 4117ns | +1049.8% |
| 3 | 4405ns | +991.9% |
| 4 | 4115ns | +1001.1% |
| 5 | 4120ns | +1058.0% |
| 6 | 4116ns | +1112.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_iter_fusion__fused | 0.025 | ok |
| hx_iter_fusion__materialized | -0.066 | ok |

**Consistency summary:**

- **hx_iter_fusion__materialized**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_iter_fusion__fused | 4.3ns | 4086.0ns | 0.1% |  |
| hx_iter_fusion__materialized | 26.9ns | 46899.7ns | 0.1% |  |

## Distribution (algo ns)

```
hx_iter_fusion__fused (n=6, range 3642.1-4262.5 ns)
   3642.1 |##########
   3673.1 |
   3704.1 |
   3735.2 |
   3766.2 |
   3797.2 |
   3828.2 |
   3859.2 |
   3890.3 |
   3921.3 |
   3952.3 |
   3983.3 |
   4014.3 |
   4045.4 |
   4076.4 |
   4107.4 |########################################
   4138.4 |
   4169.4 |
   4200.5 |
   4231.5 |
  (0 below, 1 above range)

hx_iter_fusion__materialized (n=6, range 43027.9-49004.8 ns)
  43027.9 |########################################
  43326.7 |
  43625.6 |
  43924.4 |
  44223.3 |
  44522.1 |
  44821.0 |
  45119.8 |########################################
  45418.7 |
  45717.5 |
  46016.4 |
  46315.2 |
  46614.0 |
  46912.9 |
  47211.7 |########################################
  47510.6 |########################################
  47809.4 |########################################
  48108.3 |
  48407.1 |
  48706.0 |
  (0 below, 1 above range)

```

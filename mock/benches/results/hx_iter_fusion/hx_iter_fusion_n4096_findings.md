# Iterator fusion: materialized intermediates vs fused pipeline

2 variants, 6 samples per variant.
Baseline: **hx_iter_fusion__fused**

## Highlights

Baseline for all deltas below: **hx_iter_fusion__fused**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_iter_fusion__fused dominates: 371% faster than the next best (hx_iter_fusion__materialized)

hx_iter_fusion__fused (1.03 us) leads hx_iter_fusion__materialized (4.86 us) by 371%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_iter_fusion__fused is fastest but the noisiest (CV 8.9%)

hx_iter_fusion__fused wins on median (1.03 us) yet has the highest variance (CV 8.9%), while hx_iter_fusion__materialized is the steadiest (CV 6.2%, 4.86 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_iter_fusion__fused)

The baseline hx_iter_fusion__fused is the fastest (1.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.7x the fastest

Fastest hx_iter_fusion__fused (1.03 us) to slowest hx_iter_fusion__materialized (4.86 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_iter_fusion__fused) is the fastest** at 1032.7 ns median
- 1 variant significantly slower than baseline
- Spread: 4.71x (fastest 1032.7 ns, slowest 4862.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 3391ns | 3407ns | 3043ns | 3307ns | 3691ns | base |
| hx_iter_fusion__materialized | 7197ns | 7362ns | 6543ns | 7097ns | 7673ns | +112.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_iter_fusion__fused | 1029ns | 925ns | 1121ns | base | 3.982 |
| hx_iter_fusion__materialized | 4779ns | 4373ns | 5085ns | +364.55% | 0.857 |

## Performance model

- Peak throughput: **4.430 Gops/s** (hx_iter_fusion__fused; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_iter_fusion__fused | 3.966 | 89.5% |
| hx_iter_fusion__materialized | 0.842 | 19.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_iter_fusion__fused | 3391ns | 3391ns | base |
| hx_iter_fusion__materialized | 7197ns | 7197ns | +112.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 1033ns | base | --- | [932, 1121] | --- | --- | --- | --- |
| hx_iter_fusion__materialized | 4862ns | +3839.8ns (+371.8%) | [+3446, +3964]ns | [4389, 5085] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_iter_fusion__fused | hx_iter_fusion__materialized |
|---|---|---|
| 1 | 925ns | +409.6% |
| 2 | 1121ns | +355.8% |
| 3 | 1120ns | +347.4% |
| 4 | 1121ns | +351.4% |
| 5 | 945ns | +366.1% |
| 6 | 940ns | +365.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_iter_fusion__fused | 0.137 | ok |
| hx_iter_fusion__materialized | 0.303 | moderate+ |

**Consistency summary:**

- **hx_iter_fusion__materialized**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_iter_fusion__fused | 5.3ns | 1028.7ns | 0.5% |  |
| hx_iter_fusion__materialized | 22.6ns | 4778.7ns | 0.5% |  |

## Distribution (algo ns)

```
hx_iter_fusion__fused (n=6, range 924.6-1121.0 ns)
    924.6 |####################
    934.4 |####################
    944.2 |####################
    954.1 |
    963.9 |
    973.7 |
    983.5 |
    993.3 |
   1003.2 |
   1013.0 |
   1022.8 |
   1032.6 |
   1042.4 |
   1052.3 |
   1062.1 |
   1071.9 |
   1081.7 |
   1091.5 |
   1101.4 |
   1111.2 |########################################
  (0 below, 1 above range)

hx_iter_fusion__materialized (n=6, range 4372.9-5084.8 ns)
   4372.9 |########################################
   4408.5 |
   4444.1 |
   4479.7 |
   4515.3 |
   4550.9 |
   4586.5 |
   4622.0 |
   4657.6 |
   4693.2 |####################
   4728.8 |
   4764.4 |
   4800.0 |
   4835.6 |
   4871.2 |
   4906.8 |
   4942.4 |
   4978.0 |####################
   5013.6 |
   5049.2 |####################
  (0 below, 1 above range)

```

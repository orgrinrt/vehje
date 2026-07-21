# Iterator fusion: materialized intermediates vs fused pipeline

2 variants, 6 samples per variant.
Baseline: **hx_iter_fusion__fused**

## Highlights

Baseline for all deltas below: **hx_iter_fusion__fused**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_iter_fusion__fused dominates: 1603% faster than the next best (hx_iter_fusion__materialized)

hx_iter_fusion__fused (84 ns) leads hx_iter_fusion__materialized (1.43 us) by 1603%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_iter_fusion__fused is fastest but the noisiest (CV 8.2%)

hx_iter_fusion__fused wins on median (84 ns) yet has the highest variance (CV 8.2%), while hx_iter_fusion__materialized is the steadiest (CV 3.0%, 1.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_iter_fusion__fused)

The baseline hx_iter_fusion__fused is the fastest (84 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 17.0x the fastest

Fastest hx_iter_fusion__fused (84 ns) to slowest hx_iter_fusion__materialized (1.43 us): 17.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_iter_fusion__fused) is the fastest** at 84.2 ns median
- 1 variant significantly slower than baseline
- Spread: 17.03x (fastest 84.2 ns, slowest 1433.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 2306ns | 2242ns | 2198ns | 2238ns | 2463ns | base |
| hx_iter_fusion__materialized | 3754ns | 3632ns | 3619ns | 3630ns | 4008ns | +62.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_iter_fusion__fused | 87ns | 83ns | 94ns | base | 2.938 |
| hx_iter_fusion__materialized | 1458ns | 1416ns | 1516ns | +1573.20% | 0.176 |

## Performance model

- Peak throughput: **3.073 Gops/s** (hx_iter_fusion__fused; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_iter_fusion__fused | 3.042 | 99.0% |
| hx_iter_fusion__materialized | 0.179 | 5.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_iter_fusion__fused | 2306ns | 2306ns | base |
| hx_iter_fusion__materialized | 3754ns | 3754ns | +62.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 84ns | base | --- | [83, 94] | --- | --- | --- | --- |
| hx_iter_fusion__materialized | 1433ns | +1349.0ns (+1603.1%) | [+1340, +1424]ns | [1424, 1516] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_iter_fusion__fused | hx_iter_fusion__materialized |
|---|---|---|
| 1 | 83ns | +1742.3% |
| 2 | 85ns | +1576.9% |
| 3 | 83ns | +1600.1% |
| 4 | 84ns | +1610.5% |
| 5 | 85ns | +1595.7% |
| 6 | 102ns | +1361.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_iter_fusion__fused | -0.013 | ok |
| hx_iter_fusion__materialized | -0.013 | ok |

**Consistency summary:**

- **hx_iter_fusion__materialized**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_iter_fusion__fused | 4.4ns | 87.1ns | 5.0% | HIGH |
| hx_iter_fusion__materialized | 18.9ns | 1457.9ns | 1.3% |  |

## Distribution (algo ns)

```
hx_iter_fusion__fused (n=6, range 83.3-94.0 ns)
     83.3 |########################################
     83.8 |
     84.4 |#############
     84.9 |#############
     85.4 |
     86.0 |
     86.5 |
     87.0 |
     87.6 |
     88.1 |
     88.6 |
     89.2 |
     89.7 |
     90.2 |
     90.8 |
     91.3 |
     91.8 |
     92.4 |
     92.9 |
     93.4 |
  (0 below, 1 above range)

hx_iter_fusion__materialized (n=6, range 1416.2-1516.4 ns)
   1416.2 |#############
   1421.2 |
   1426.2 |
   1431.2 |########################################
   1436.2 |
   1441.3 |
   1446.3 |
   1451.3 |
   1456.3 |
   1461.3 |
   1466.3 |
   1471.3 |
   1476.3 |
   1481.4 |
   1486.4 |
   1491.4 |
   1496.4 |#############
   1501.4 |
   1506.4 |
   1511.4 |
  (0 below, 1 above range)

```

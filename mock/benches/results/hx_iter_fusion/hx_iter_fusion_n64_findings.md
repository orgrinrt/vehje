# Iterator fusion: materialized intermediates vs fused pipeline

2 variants, 6 samples per variant.
Baseline: **hx_iter_fusion__fused**

## Highlights

Baseline for all deltas below: **hx_iter_fusion__fused**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_iter_fusion__fused dominates: 7319% faster than the next best (hx_iter_fusion__materialized)

hx_iter_fusion__fused (18 ns) leads hx_iter_fusion__materialized (1.34 us) by 7319%, a clear separation rather than a photo finish. CV 9.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_iter_fusion__fused is fastest but the noisiest (CV 9.8%)

hx_iter_fusion__fused wins on median (18 ns) yet has the highest variance (CV 9.8%), while hx_iter_fusion__materialized is the steadiest (CV 2.5%, 1.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_iter_fusion__fused)

The baseline hx_iter_fusion__fused is the fastest (18 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 74.2x the fastest

Fastest hx_iter_fusion__fused (18 ns) to slowest hx_iter_fusion__materialized (1.34 us): 74.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_iter_fusion__fused) is the fastest** at 18.1 ns median
- 1 variant significantly slower than baseline
- Spread: 74.19x (fastest 18.1 ns, slowest 1342.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 2630ns | 2757ns | 2148ns | 2656ns | 2833ns | base |
| hx_iter_fusion__materialized | 3950ns | 3926ns | 3712ns | 3918ns | 4117ns | +50.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_iter_fusion__fused | 18ns | 16ns | 20ns | base | 3.507 |
| hx_iter_fusion__materialized | 1345ns | 1300ns | 1383ns | +7270.59% | 0.048 |

## Performance model

- Peak throughput: **4.051 Gops/s** (hx_iter_fusion__fused; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_iter_fusion__fused | 3.536 | 87.3% |
| hx_iter_fusion__materialized | 0.048 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_iter_fusion__fused | 2630ns | 2630ns | base |
| hx_iter_fusion__materialized | 3950ns | 3950ns | +50.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 18ns | base | --- | [16, 20] | --- | --- | --- | --- |
| hx_iter_fusion__materialized | 1343ns | +1323.2ns (+7310.2%) | [+1291, +1366]ns | [1309, 1383] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_iter_fusion__fused | hx_iter_fusion__materialized |
|---|---|---|
| 1 | 16ns | +8125.3% |
| 2 | 18ns | +7140.4% |
| 3 | 17ns | +8248.5% |
| 4 | 21ns | +6318.9% |
| 5 | 20ns | +6628.1% |
| 6 | 18ns | +7567.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_iter_fusion__fused | -0.066 | ok |
| hx_iter_fusion__materialized | -0.067 | ok |

**Consistency summary:**

- **hx_iter_fusion__materialized**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_iter_fusion__fused | 4.2ns | 18.3ns | 23.2% | HIGH |
| hx_iter_fusion__materialized | 22.0ns | 1345.1ns | 1.6% |  |

## Distribution (algo ns)

```
hx_iter_fusion__fused (n=6, range 15.8-20.4 ns)
     15.8 |########################################
     16.0 |
     16.3 |
     16.5 |########################################
     16.7 |
     16.9 |
     17.2 |
     17.4 |
     17.6 |
     17.9 |########################################
     18.1 |########################################
     18.3 |
     18.6 |
     18.8 |
     19.0 |
     19.2 |
     19.5 |########################################
     19.7 |
     19.9 |
     20.2 |
  (0 below, 1 above range)

hx_iter_fusion__materialized (n=6, range 1299.6-1383.3 ns)
   1299.6 |########################################
   1303.8 |
   1308.0 |
   1312.2 |
   1316.3 |########################################
   1320.5 |
   1324.7 |########################################
   1328.9 |
   1333.1 |
   1337.3 |
   1341.5 |
   1345.7 |
   1349.8 |
   1354.0 |
   1358.2 |########################################
   1362.4 |
   1366.6 |
   1370.8 |########################################
   1375.0 |
   1379.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **hx_iter_fusion__fused**: bridge=23.2% of algo (FFI overhead may distort results)

# Node record width: 16B pool-spill vs 24B inline operands

2 variants, 6 samples per variant.
Baseline: **hx_recwidth__rec24**

## Highlights

Baseline for all deltas below: **hx_recwidth__rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_recwidth__rec24 dominates: 17% faster than the next best (hx_recwidth__rec16)

hx_recwidth__rec24 (1.30 us) leads hx_recwidth__rec16 (1.52 us) by 17%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_recwidth__rec24)

The baseline hx_recwidth__rec24 is the fastest (1.30 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_recwidth__rec24) is the fastest** at 1295.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 1295.8 ns, slowest 1522.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_recwidth__rec16 | 3897ns | 4103ns | 3450ns | 3890ns | 4131ns | +5.88% |
| hx_recwidth__rec24 | 3680ns | 3871ns | 3250ns | 3667ns | 3915ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_recwidth__rec16 | 1446ns | 1281ns | 1533ns | +17.74% | 0.708 |
| hx_recwidth__rec24 | 1228ns | 1088ns | 1300ns | base | 0.834 |

## Performance model

- Peak throughput: **0.941 Gops/s** (hx_recwidth__rec24; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_recwidth__rec16 | 0.673 | 71.5% |
| hx_recwidth__rec24 | 0.790 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_recwidth__rec16 | 3897ns | 3897ns | +5.88% |
| hx_recwidth__rec24 | 3680ns | 3680ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_recwidth__rec24 | 1296ns | base | --- | [1090, 1300] | --- | --- | --- | --- |
| hx_recwidth__rec16 | 1522ns | +224.4ns (+17.3%) | [+194, +236]ns | [1284, 1533] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_recwidth__rec24 | hx_recwidth__rec16 |
|---|---|---|
| 1 | 1091ns | +17.4% |
| 2 | 1088ns | +18.2% |
| 3 | 1296ns | +18.3% |
| 4 | 1296ns | +16.8% |
| 5 | 1300ns | +17.8% |
| 6 | 1300ns | +18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_recwidth__rec16 | 0.395 | moderate+ |
| hx_recwidth__rec24 | 0.419 | moderate+ |

**Consistency summary:**

- **hx_recwidth__rec16**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_recwidth__rec16 | 2.6ns | 1446.3ns | 0.2% |  |
| hx_recwidth__rec24 | 2.5ns | 1228.4ns | 0.2% |  |

## Distribution (algo ns)

```
hx_recwidth__rec16 (n=6, range 1280.8-1533.3 ns)
   1280.8 |########################################
   1293.4 |
   1306.1 |
   1318.7 |
   1331.3 |
   1343.9 |
   1356.6 |
   1369.2 |
   1381.8 |
   1394.4 |
   1407.1 |
   1419.7 |
   1432.3 |
   1445.0 |
   1457.6 |
   1470.2 |
   1482.8 |
   1495.5 |
   1508.1 |####################
   1520.7 |########################################
  (0 below, 1 above range)

hx_recwidth__rec24 (n=6, range 1088.3-1299.6 ns)
   1088.3 |########################################
   1098.9 |
   1109.4 |
   1120.0 |
   1130.6 |
   1141.1 |
   1151.7 |
   1162.3 |
   1172.8 |
   1183.4 |
   1193.9 |
   1204.5 |
   1215.1 |
   1225.6 |
   1236.2 |
   1246.8 |
   1257.3 |
   1267.9 |
   1278.5 |
   1289.0 |########################################
  (0 below, 2 above range)

```

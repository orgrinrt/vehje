# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 450% faster than the next best (resolve_linear)

resolve_flat (276 ns) leads resolve_linear (1.52 us) by 450%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 6.4x slower than the field

resolve_hashed (1.78 us) is 6.4x the fastest (276 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### resolve_flat shows warm-up / thermal drift (autocorr +0.51)

resolve_flat's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (276 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.4x the fastest

Fastest resolve_flat (276 ns) to slowest resolve_hashed (1.78 us): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 276.1 ns median
- 2 variants significantly slower than baseline
- Spread: 6.44x (fastest 276.1 ns, slowest 1778.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 2598ns | 2590ns | 2411ns | 2531ns | 2792ns | base |
| resolve_hashed | 4165ns | 4192ns | 3812ns | 4087ns | 4458ns | +60.31% |
| resolve_linear | 4007ns | 4109ns | 3453ns | 3986ns | 4315ns | +54.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 277ns | 257ns | 299ns | base | 0.923 |
| resolve_hashed | 1747ns | 1561ns | 1870ns | +529.69% | 0.147 |
| resolve_linear | 1525ns | 1279ns | 1693ns | +449.75% | 0.168 |

## Performance model

- Peak throughput: **0.997 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 0.927 | 93.0% |
| resolve_hashed | 0.144 | 14.4% |
| resolve_linear | 0.169 | 16.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 2598ns | 2598ns | base |
| resolve_hashed | 4165ns | 4165ns | +60.31% |
| resolve_linear | 4007ns | 4007ns | +54.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 276ns | base | --- | [258, 299] | --- | --- | --- | --- |
| resolve_hashed | 1779ns | +1492.5ns (+540.6%) | [+1335, +1582]ns | [1592, 1870] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 1518ns | +1231.2ns (+446.0%) | [+1108, +1405]ns | [1365, 1693] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 283ns | +525.2% | +435.4% |
| 2 | 308ns | +499.4% | +435.6% |
| 3 | 289ns | +517.6% | +425.0% |
| 4 | 269ns | +604.1% | +545.5% |
| 5 | 257ns | +508.2% | +398.3% |
| 6 | 258ns | +528.3% | +461.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | 0.510 | HIGH+ (drift/warm-up) |
| resolve_hashed | 0.097 | ok |
| resolve_linear | -0.291 | moderate- |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 4.1ns | 277.4ns | 1.5% |  |
| resolve_hashed | 4.0ns | 1747.0ns | 0.2% |  |
| resolve_linear | 4.9ns | 1525.2ns | 0.3% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 256.7-298.8 ns)
    256.7 |########################################
    258.8 |
    260.9 |
    263.0 |
    265.1 |
    267.2 |####################
    269.3 |
    271.4 |
    273.5 |
    275.6 |
    277.7 |
    279.8 |
    281.9 |####################
    284.0 |
    286.1 |
    288.2 |####################
    290.3 |
    292.4 |
    294.5 |
    296.6 |
  (0 below, 1 above range)

resolve_hashed (n=6, range 1561.2-1870.2 ns)
   1561.2 |########################################
   1576.7 |
   1592.1 |
   1607.5 |########################################
   1623.0 |
   1638.5 |
   1653.9 |
   1669.4 |
   1684.8 |
   1700.2 |
   1715.7 |
   1731.2 |
   1746.6 |
   1762.0 |########################################
   1777.5 |########################################
   1793.0 |
   1808.4 |
   1823.8 |
   1839.3 |########################################
   1854.8 |
  (0 below, 1 above range)

resolve_linear (n=6, range 1279.2-1693.1 ns)
   1279.2 |####################
   1299.9 |
   1320.6 |
   1341.3 |
   1362.0 |
   1382.7 |
   1403.4 |
   1424.1 |
   1444.8 |####################
   1465.5 |
   1486.2 |
   1506.8 |########################################
   1527.5 |
   1548.2 |
   1568.9 |
   1589.6 |
   1610.3 |
   1631.0 |####################
   1651.7 |
   1672.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **resolve_flat**: autocorrelation=0.51 (measurement drift or warm-up artifact)

# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 409% faster than the next best (resolve_linear)

resolve_flat (287 ns) leads resolve_linear (1.46 us) by 409%, a clear separation rather than a photo finish. CV 7.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 6.0x slower than the field

resolve_hashed (1.72 us) is 6.0x the fastest (287 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### resolve_flat shows warm-up / thermal drift (autocorr +0.50)

resolve_flat's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (287 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.0x the fastest

Fastest resolve_flat (287 ns) to slowest resolve_hashed (1.72 us): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 286.9 ns median
- 2 variants significantly slower than baseline
- Spread: 5.98x (fastest 286.9 ns, slowest 1715.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 2684ns | 2699ns | 2410ns | 2638ns | 2892ns | base |
| resolve_hashed | 4160ns | 4099ns | 3689ns | 3976ns | 4673ns | +54.97% |
| resolve_linear | 3830ns | 3800ns | 3436ns | 3724ns | 4187ns | +42.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 285ns | 254ns | 308ns | base | 0.898 |
| resolve_hashed | 1715ns | 1497ns | 1927ns | +501.83% | 0.149 |
| resolve_linear | 1460ns | 1274ns | 1624ns | +412.30% | 0.175 |

## Performance model

- Peak throughput: **1.009 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 0.892 | 88.4% |
| resolve_hashed | 0.149 | 14.8% |
| resolve_linear | 0.175 | 17.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 2684ns | 2684ns | base |
| resolve_hashed | 4160ns | 4160ns | +54.97% |
| resolve_linear | 3830ns | 3830ns | +42.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 287ns | base | --- | [260, 308] | --- | --- | --- | --- |
| resolve_hashed | 1716ns | +1435.0ns (+500.2%) | [+1235, +1621]ns | [1503, 1927] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 1460ns | +1181.5ns (+411.8%) | [+1028, +1316]ns | [1296, 1624] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 270ns | +458.8% | +371.9% |
| 2 | 254ns | +515.4% | +445.9% |
| 3 | 267ns | +461.3% | +394.5% |
| 4 | 308ns | +515.5% | +414.7% |
| 5 | 304ns | +544.1% | +405.4% |
| 6 | 308ns | +508.1% | +440.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | 0.502 | HIGH+ (drift/warm-up) |
| resolve_hashed | 0.467 | moderate+ |
| resolve_linear | 0.256 | moderate+ |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 5.1ns | 285.0ns | 1.8% |  |
| resolve_hashed | 4.4ns | 1715.2ns | 0.3% |  |
| resolve_linear | 4.1ns | 1460.1ns | 0.3% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 253.7-307.9 ns)
    253.7 |########################################
    256.4 |
    259.1 |
    261.8 |
    264.5 |########################################
    267.2 |
    270.0 |########################################
    272.7 |
    275.4 |
    278.1 |
    280.8 |
    283.5 |
    286.2 |
    288.9 |
    291.6 |
    294.3 |
    297.1 |
    299.8 |
    302.5 |########################################
    305.2 |########################################
  (0 below, 1 above range)

resolve_hashed (n=6, range 1497.1-1927.1 ns)
   1497.1 |########################################
   1518.6 |
   1540.1 |####################
   1561.6 |
   1583.1 |
   1604.6 |
   1626.1 |
   1647.6 |
   1669.1 |
   1690.6 |
   1712.1 |
   1733.6 |
   1755.1 |
   1776.6 |
   1798.1 |
   1819.6 |
   1841.1 |
   1862.6 |####################
   1884.1 |####################
   1905.6 |
  (0 below, 1 above range)

resolve_linear (n=6, range 1274.2-1623.6 ns)
   1274.2 |########################################
   1291.7 |
   1309.1 |########################################
   1326.6 |
   1344.1 |
   1361.5 |
   1379.0 |########################################
   1396.5 |
   1413.9 |
   1431.4 |
   1448.9 |
   1466.3 |
   1483.8 |
   1501.3 |
   1518.7 |########################################
   1536.2 |
   1553.7 |
   1571.1 |########################################
   1588.6 |
   1606.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **resolve_flat**: autocorrelation=0.50 (measurement drift or warm-up artifact)

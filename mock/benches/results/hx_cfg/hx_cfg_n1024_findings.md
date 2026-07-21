# Interpreter model: recursive tree-walk vs CFG-of-blocks linear scan

2 variants, 6 samples per variant.
Baseline: **hx_cfg__cfgblock**

## Highlights

Baseline for all deltas below: **hx_cfg__cfgblock**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_cfg__cfgblock dominates: 862% faster than the next best (hx_cfg__treewalk)

hx_cfg__cfgblock (1.58 us) leads hx_cfg__treewalk (15.21 us) by 862%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_cfg__cfgblock)

The baseline hx_cfg__cfgblock is the fastest (1.58 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.6x the fastest

Fastest hx_cfg__cfgblock (1.58 us) to slowest hx_cfg__treewalk (15.21 us): 9.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_cfg__cfgblock) is the fastest** at 1581.7 ns median
- 1 variant significantly slower than baseline
- Spread: 9.62x (fastest 1581.7 ns, slowest 15214.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 3952ns | 4043ns | 3532ns | 3918ns | 4214ns | base |
| hx_cfg__treewalk | 17321ns | 17704ns | 15070ns | 17379ns | 18359ns | +338.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_cfg__cfgblock | 1550ns | 1396ns | 1641ns | base | 0.661 |
| hx_cfg__treewalk | 14860ns | 12943ns | 15692ns | +859.05% | 0.069 |

## Performance model

- Peak throughput: **0.733 Gops/s** (hx_cfg__cfgblock; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_cfg__cfgblock | 0.647 | 88.3% |
| hx_cfg__treewalk | 0.067 | 9.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_cfg__cfgblock | 3952ns | 3952ns | base |
| hx_cfg__treewalk | 17321ns | 17321ns | +338.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_cfg__cfgblock | 1582ns | base | --- | [1426, 1641] | --- | --- | --- | --- |
| hx_cfg__treewalk | 15214ns | +13726.1ns (+867.8%) | [+12156, +14051]ns | [13675, 15692] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_cfg__cfgblock | hx_cfg__treewalk |
|---|---|---|
| 1 | 1396ns | +967.3% |
| 2 | 1586ns | +881.1% |
| 3 | 1580ns | +882.5% |
| 4 | 1697ns | +832.8% |
| 5 | 1583ns | +717.7% |
| 6 | 1455ns | +890.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_cfg__cfgblock | 0.032 | ok |
| hx_cfg__treewalk | 0.027 | ok |

**Consistency summary:**

- **hx_cfg__treewalk**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_cfg__cfgblock | 2.6ns | 1549.5ns | 0.2% |  |
| hx_cfg__treewalk | 3.1ns | 14860.5ns | 0.0% |  |

## Distribution (algo ns)

```
hx_cfg__cfgblock (n=6, range 1396.2-1641.2 ns)
   1396.2 |#############
   1408.5 |
   1420.7 |
   1433.0 |
   1445.2 |#############
   1457.5 |
   1469.7 |
   1482.0 |
   1494.2 |
   1506.5 |
   1518.7 |
   1531.0 |
   1543.2 |
   1555.5 |
   1567.7 |
   1580.0 |########################################
   1592.2 |
   1604.5 |
   1616.7 |
   1629.0 |
  (0 below, 1 above range)

hx_cfg__treewalk (n=6, range 12942.9-15692.3 ns)
  12942.9 |########################################
  13080.4 |
  13217.8 |
  13355.3 |
  13492.8 |
  13630.2 |
  13767.7 |
  13905.2 |
  14042.7 |
  14180.1 |
  14317.6 |########################################
  14455.1 |
  14592.5 |
  14730.0 |
  14867.5 |########################################
  15004.9 |
  15142.4 |
  15279.9 |
  15417.4 |########################################
  15554.8 |########################################
  (0 below, 1 above range)

```

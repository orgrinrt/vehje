# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 62391% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (2.33 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 62391%, a clear separation rather than a photo finish. CV 7.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 666.2x slower than the field

abi_lifecycle_leaf_fresh_per_batch (1.55 ms) is 666.2x the fastest (2.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (62391% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 62391% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 666.2x the fastest

Fastest abi_lifecycle_leaf_null_entry (2.33 us) to slowest abi_lifecycle_leaf_fresh_per_batch (1.55 ms): 666.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 2332.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 666.18x (fastest 2332.1 ns, slowest 1553605.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1560532ns | 1556347ns | 1551298ns | 1554832ns | 1573698ns | +1.08% |
| abi_lifecycle_leaf_fresh_per_column | 1505237ns | 1494883ns | 1468646ns | 1486867ns | 1551088ns | -2.50% |
| abi_lifecycle_leaf_held_handle | 1543815ns | 1460218ns | 1457003ns | 1459199ns | 1714145ns | base |
| abi_lifecycle_leaf_null_entry | 4729ns | 4646ns | 4389ns | 4586ns | 5113ns | -99.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1557676ns | 1548482ns | 1570652ns | +1.11% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1501954ns | 1465882ns | 1547323ns | -2.50% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1540531ns | 1454182ns | 1709904ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 2386ns | 2213ns | 2597ns | -99.85% | 0.013 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 49939.0 | 1558728.7 | 1557675.7 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 62234.9 | 1519329.1 | 1501953.7 | 0 |
| abi_lifecycle_leaf_held_handle | 58182.9 | 1549078.9 | 1540530.8 | n/a |
| abi_lifecycle_leaf_null_entry | 32073.9 | 2469.5 | 2386.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.014 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1560532ns | 1560532ns | +1.08% |
| abi_lifecycle_leaf_fresh_per_column | 1505237ns | 1505237ns | -2.50% |
| abi_lifecycle_leaf_held_handle | 1543815ns | 1543815ns | base |
| abi_lifecycle_leaf_null_entry | 4729ns | 4729ns | -99.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1457362ns | base | --- | [1454326, 1709904] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 1553606ns | no significant difference | [-141420, +98368]ns | [1548769, 1570652] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1491543ns | no significant difference | [-178043, +47732]ns | [1466995, 1547323] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_null_entry | 2332ns | -1454909.4ns (-99.8%) | [-1707445, -1452080]ns | [2229, 2597] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1456006ns | +6.5% | +3.8% | -99.8% |
| 2 | 1454182ns | +6.5% | +2.7% | -99.8% |
| 3 | 1454470ns | +7.0% | +0.8% | -99.8% |
| 4 | 1458719ns | +6.2% | +0.6% | -99.8% |
| 5 | 1471557ns | +6.4% | +1.2% | -99.8% |
| 6 | 1948250ns | -19.2% | -18.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.278 | moderate+ |
| abi_lifecycle_leaf_fresh_per_column | 0.090 | ok |
| abi_lifecycle_leaf_held_handle | -0.004 | ok |
| abi_lifecycle_leaf_null_entry | -0.396 | moderate- |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_leaf_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4727016.1ns | 1557675.7ns | 303.5% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4592657.3ns | 1501953.7ns | 305.8% | HIGH |
| abi_lifecycle_leaf_held_handle | 4701284.3ns | 1540530.8ns | 305.2% | HIGH |
| abi_lifecycle_leaf_null_entry | 123015.1ns | 2386.0ns | 5155.7% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 1548482.5-1570652.1 ns)
  1548482.5 |########################################
  1549591.0 |
  1550699.5 |####################
  1551807.9 |
  1552916.4 |
  1554024.9 |
  1555133.4 |
  1556241.8 |####################
  1557350.3 |
  1558458.8 |
  1559567.3 |
  1560675.8 |
  1561784.2 |
  1562892.7 |
  1564001.2 |
  1565109.7 |
  1566218.1 |####################
  1567326.6 |
  1568435.1 |
  1569543.6 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1465881.7-1547323.3 ns)
  1465881.7 |########################################
  1469953.8 |
  1474025.9 |
  1478097.9 |
  1482170.0 |
  1486242.1 |####################
  1490314.2 |####################
  1494386.3 |
  1498458.3 |
  1502530.4 |
  1506602.5 |
  1510674.6 |####################
  1514746.7 |
  1518818.7 |
  1522890.8 |
  1526962.9 |
  1531035.0 |
  1535107.1 |
  1539179.1 |
  1543251.2 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1454182.5-1709903.6 ns)
  1454182.5 |########################################
  1466968.6 |##########
  1479754.6 |
  1492540.7 |
  1505326.7 |
  1518112.8 |
  1530898.8 |
  1543684.9 |
  1556470.9 |
  1569257.0 |
  1582043.0 |
  1594829.1 |
  1607615.1 |
  1620401.2 |
  1633187.2 |
  1645973.3 |
  1658759.3 |
  1671545.4 |
  1684331.4 |
  1697117.5 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 2213.3-2597.3 ns)
   2213.3 |####################
   2232.5 |########################################
   2251.7 |
   2270.9 |
   2290.1 |
   2309.3 |
   2328.5 |
   2347.7 |
   2366.9 |
   2386.1 |
   2405.3 |####################
   2424.5 |
   2443.7 |
   2462.9 |
   2482.1 |####################
   2501.3 |
   2520.5 |
   2539.7 |
   2558.9 |
   2578.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=5179.1% of algo (FFI overhead may distort results)

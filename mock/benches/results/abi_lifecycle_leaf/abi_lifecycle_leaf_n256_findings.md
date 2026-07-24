# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 45439% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (3.21 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 45439%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 470.6x slower than the field

abi_lifecycle_leaf_fresh_per_batch (1.51 ms) is 470.6x the fastest (3.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (45439% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 45439% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 470.6x the fastest

Fastest abi_lifecycle_leaf_null_entry (3.21 us) to slowest abi_lifecycle_leaf_fresh_per_batch (1.51 ms): 470.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 3205.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 470.60x (fastest 3205.6 ns, slowest 1508548.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1559465ns | 1511845ns | 1470096ns | 1499175ns | 1694584ns | +2.04% |
| abi_lifecycle_leaf_fresh_per_column | 1491230ns | 1475930ns | 1472581ns | 1475065ns | 1524802ns | -2.43% |
| abi_lifecycle_leaf_held_handle | 1528347ns | 1462627ns | 1451308ns | 1460139ns | 1669180ns | base |
| abi_lifecycle_leaf_null_entry | 5485ns | 5516ns | 5281ns | 5473ns | 5604ns | -99.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1556191ns | 1467485ns | 1690771ns | +2.04% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1488249ns | 1469842ns | 1521308ns | -2.41% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1525025ns | 1448706ns | 1664682ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 3200ns | 3077ns | 3279ns | -99.79% | 0.080 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 59010.9 | 1594696.2 | 1556191.2 | n/a |
| abi_lifecycle_leaf_fresh_per_column | 50976.2 | 1485482.6 | 1488248.5 | n/a |
| abi_lifecycle_leaf_held_handle | 56330.4 | 1506488.2 | 1525025.5 | n/a |
| abi_lifecycle_leaf_null_entry | 27735.9 | 3220.6 | 3200.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.2% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.080 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1559465ns | 1559465ns | +2.04% |
| abi_lifecycle_leaf_fresh_per_column | 1491230ns | 1491230ns | -2.43% |
| abi_lifecycle_leaf_held_handle | 1528347ns | 1528347ns | base |
| abi_lifecycle_leaf_null_entry | 5485ns | 5485ns | -99.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1459810ns | base | --- | [1450584, 1664682] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 1508548ns | no significant difference | [-126076, +204867]ns | [1469255, 1690771] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1473246ns | no significant difference | [-144560, +20978]ns | [1470191, 1521308] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_leaf_null_entry | 3206ns | -1456544.5ns (-99.8%) | [-1661463, -1447468]ns | [3116, 3279] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1448706ns | +1.5% | +1.5% | -99.8% |
| 2 | 1567465ns | -1.6% | -6.0% | -99.8% |
| 3 | 1452462ns | +2.0% | +1.4% | -99.8% |
| 4 | 1459229ns | +26.1% | +1.2% | -99.8% |
| 5 | 1761899ns | -12.8% | -11.1% | -99.8% |
| 6 | 1460392ns | +0.5% | +0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.225 | moderate- |
| abi_lifecycle_leaf_fresh_per_column | -0.232 | moderate- |
| abi_lifecycle_leaf_held_handle | -0.419 | moderate- |
| abi_lifecycle_leaf_null_entry | 0.165 | ok |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 2/6, lost 4/6
- **abi_lifecycle_leaf_fresh_per_column**: won 2/6, lost 4/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4813794.8ns | 1556191.2ns | 309.3% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4512280.5ns | 1488248.5ns | 303.2% | HIGH |
| abi_lifecycle_leaf_held_handle | 4635482.2ns | 1525025.5ns | 304.0% | HIGH |
| abi_lifecycle_leaf_null_entry | 121258.5ns | 3200.3ns | 3788.9% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 1467485.4-1690771.2 ns)
  1467485.4 |########################################
  1478649.7 |####################
  1489814.0 |
  1500978.3 |
  1512142.6 |
  1523306.8 |
  1534471.1 |########################################
  1545635.4 |
  1556799.7 |
  1567964.0 |
  1579128.3 |
  1590292.6 |
  1601456.9 |
  1612621.2 |
  1623785.5 |
  1634949.8 |
  1646114.0 |
  1657278.3 |
  1668442.6 |
  1679606.9 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1469842.5-1521307.9 ns)
  1469842.5 |########################################
  1472415.8 |########################################
  1474989.0 |####################
  1477562.3 |
  1480135.6 |
  1482708.9 |
  1485282.1 |
  1487855.4 |
  1490428.7 |
  1493001.9 |
  1495575.2 |
  1498148.5 |
  1500721.7 |
  1503295.0 |
  1505868.3 |
  1508441.5 |
  1511014.8 |
  1513588.1 |
  1516161.4 |
  1518734.6 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1448705.8-1664682.3 ns)
  1448705.8 |########################################
  1459504.6 |#############
  1470303.4 |
  1481102.3 |
  1491901.1 |
  1502699.9 |
  1513498.8 |
  1524297.6 |
  1535096.4 |
  1545895.2 |
  1556694.0 |#############
  1567492.9 |
  1578291.7 |
  1589090.5 |
  1599889.3 |
  1610688.2 |
  1621487.0 |
  1632285.8 |
  1643084.6 |
  1653883.5 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 3076.7-3279.0 ns)
   3076.7 |########################################
   3086.8 |
   3096.9 |
   3107.0 |
   3117.2 |
   3127.3 |
   3137.4 |
   3147.5 |########################################
   3157.6 |
   3167.7 |
   3177.8 |########################################
   3188.0 |
   3198.1 |
   3208.2 |
   3218.3 |########################################
   3228.4 |
   3238.5 |
   3248.7 |########################################
   3258.8 |
   3268.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=3791.2% of algo (FFI overhead may distort results)

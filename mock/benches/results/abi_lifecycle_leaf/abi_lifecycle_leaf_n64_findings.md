# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 59322% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (2.46 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 59322%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 619.5x slower than the field

abi_lifecycle_leaf_fresh_per_batch (1.52 ms) is 619.5x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (59322% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 59322% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 619.5x the fastest

Fastest abi_lifecycle_leaf_null_entry (2.46 us) to slowest abi_lifecycle_leaf_fresh_per_batch (1.52 ms): 619.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 2459.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 619.55x (fastest 2459.6 ns, slowest 1523843.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1539809ns | 1527090ns | 1506991ns | 1520559ns | 1585091ns | +3.59% |
| abi_lifecycle_leaf_fresh_per_column | 1495616ns | 1473215ns | 1470735ns | 1472395ns | 1542887ns | +0.62% |
| abi_lifecycle_leaf_held_handle | 1486439ns | 1464507ns | 1454838ns | 1461587ns | 1539519ns | base |
| abi_lifecycle_leaf_null_entry | 4718ns | 4713ns | 4602ns | 4679ns | 4835ns | -99.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1536556ns | 1504135ns | 1581477ns | +3.59% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1492597ns | 1467979ns | 1539392ns | +0.63% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1483291ns | 1451952ns | 1535862ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 2469ns | 2398ns | 2536ns | -99.83% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 58577.6 | 1533211.7 | 1536556.1 | n/a |
| abi_lifecycle_leaf_fresh_per_column | 54901.2 | 1494614.0 | 1492596.6 | 0 |
| abi_lifecycle_leaf_held_handle | 56090.6 | 1489568.1 | 1483291.4 | n/a |
| abi_lifecycle_leaf_null_entry | 29173.9 | 2700.2 | 2469.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.2% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.026 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1539809ns | 1539809ns | +3.59% |
| abi_lifecycle_leaf_fresh_per_column | 1495616ns | 1495616ns | +0.62% |
| abi_lifecycle_leaf_held_handle | 1486439ns | 1486439ns | base |
| abi_lifecycle_leaf_null_entry | 4718ns | 4718ns | -99.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1461543ns | base | --- | [1452469, 1535862] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 1523844ns | +45586.4ns (+3.1%) | [+9241, +104966]ns | [1504348, 1581477] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1470387ns | no significant difference | [-538, +19609]ns | [1468011, 1539392] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_null_entry | 2460ns | -1459110.2ns (-99.8%) | [-1533397, -1449959]ns | [2412, 2536] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1462859ns | +2.8% | +0.4% | -99.8% |
| 2 | 1460227ns | +3.0% | +0.8% | -99.8% |
| 3 | 1451952ns | +5.5% | +1.7% | -99.8% |
| 4 | 1469180ns | +3.2% | -0.1% | -99.8% |
| 5 | 1602545ns | -1.4% | +0.0% | -99.8% |
| 6 | 1452985ns | +9.0% | +1.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.369 | moderate+ |
| abi_lifecycle_leaf_fresh_per_column | -0.287 | moderate- |
| abi_lifecycle_leaf_held_handle | -0.212 | moderate- |
| abi_lifecycle_leaf_null_entry | 0.072 | ok |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_leaf_fresh_per_column**: won 0/6, lost 4/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4667331.7ns | 1536556.1ns | 303.8% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4531610.5ns | 1492596.6ns | 303.6% | HIGH |
| abi_lifecycle_leaf_held_handle | 4549097.0ns | 1483291.4ns | 306.7% | HIGH |
| abi_lifecycle_leaf_null_entry | 114402.8ns | 2469.4ns | 4632.9% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 1504134.6-1581476.9 ns)
  1504134.6 |########################################
  1508001.7 |
  1511868.8 |
  1515735.9 |####################
  1519603.1 |
  1523470.2 |
  1527337.3 |
  1531204.4 |####################
  1535071.5 |
  1538938.6 |
  1542805.7 |
  1546672.8 |
  1550540.0 |
  1554407.1 |
  1558274.2 |
  1562141.3 |
  1566008.4 |
  1569875.5 |
  1573742.6 |
  1577609.7 |####################
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1467979.2-1539391.6 ns)
  1467979.2 |########################################
  1471549.8 |#############
  1475120.4 |#############
  1478691.1 |
  1482261.7 |
  1485832.3 |
  1489402.9 |
  1492973.6 |
  1496544.2 |
  1500114.8 |
  1503685.4 |
  1507256.0 |
  1510826.7 |
  1514397.3 |
  1517967.9 |
  1521538.5 |
  1525109.2 |
  1528679.8 |
  1532250.4 |
  1535821.0 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1451952.1-1535862.5 ns)
  1451952.1 |########################################
  1456147.6 |####################
  1460343.1 |####################
  1464538.7 |
  1468734.2 |####################
  1472929.7 |
  1477125.2 |
  1481320.7 |
  1485516.3 |
  1489711.8 |
  1493907.3 |
  1498102.8 |
  1502298.3 |
  1506493.9 |
  1510689.4 |
  1514884.9 |
  1519080.4 |
  1523275.9 |
  1527471.5 |
  1531667.0 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 2398.3-2536.3 ns)
   2398.3 |########################################
   2405.2 |
   2412.1 |
   2419.0 |
   2425.9 |########################################
   2432.8 |
   2439.7 |
   2446.6 |########################################
   2453.5 |
   2460.4 |########################################
   2467.3 |
   2474.2 |########################################
   2481.1 |
   2488.0 |
   2494.9 |
   2501.8 |
   2508.7 |
   2515.6 |
   2522.5 |
   2529.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=4619.4% of algo (FFI overhead may distort results)

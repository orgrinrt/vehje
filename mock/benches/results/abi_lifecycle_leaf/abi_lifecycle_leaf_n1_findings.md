# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 29976% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (4.86 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 29976%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 898.3x slower than the field

abi_lifecycle_leaf_fresh_per_batch (4.37 ms) is 898.3x the fastest (4.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (29976% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 29976% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 898.3x the fastest

Fastest abi_lifecycle_leaf_null_entry (4.86 us) to slowest abi_lifecycle_leaf_fresh_per_batch (4.37 ms): 898.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 4863.7 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 898.28x (fastest 4863.7 ns, slowest 4368955.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4442847ns | 4372186ns | 4349805ns | 4370099ns | 4598491ns | +198.31% |
| abi_lifecycle_leaf_fresh_per_column | 1493610ns | 1483161ns | 1478566ns | 1482208ns | 1518234ns | +0.29% |
| abi_lifecycle_leaf_held_handle | 1489316ns | 1465530ns | 1456341ns | 1463391ns | 1544691ns | base |
| abi_lifecycle_leaf_null_entry | 7169ns | 7135ns | 6982ns | 7102ns | 7363ns | -99.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4439439ns | 4346375ns | 4594987ns | +198.69% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1490804ns | 1476042ns | 1515190ns | +0.30% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1486326ns | 1453711ns | 1541062ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 4897ns | 4798ns | 5030ns | -99.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 73497.4 | 4392853.5 | 4439438.6 | n/a |
| abi_lifecycle_leaf_fresh_per_column | 48769.8 | 1484609.0 | 1490803.6 | 1 |
| abi_lifecycle_leaf_held_handle | 55270.2 | 1483777.0 | 1486325.7 | n/a |
| abi_lifecycle_leaf_null_entry | 28007.7 | 4953.2 | 4897.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.3% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.3% |
| abi_lifecycle_leaf_null_entry | 0.000 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4442847ns | 4442847ns | +198.31% |
| abi_lifecycle_leaf_fresh_per_column | 1493610ns | 1493610ns | +0.29% |
| abi_lifecycle_leaf_held_handle | 1489316ns | 1489316ns | base |
| abi_lifecycle_leaf_null_entry | 7169ns | 7169ns | -99.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1462803ns | base | --- | [1455112, 1541062] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 4368955ns | +2901856.7ns (+198.4%) | [+2821522, +3135960]ns | [4354373, 4594987] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1480374ns | no significant difference | [-62614, +58476]ns | [1476846, 1515190] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_leaf_null_entry | 4864ns | -1457795.2ns (-99.7%) | [-1536176, -1450314]ns | [4798, 5030] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1453711ns | +229.2% | +6.6% | -99.7% |
| 2 | 1604439ns | +171.9% | -7.7% | -99.7% |
| 3 | 1464343ns | +200.7% | +1.1% | -99.7% |
| 4 | 1461264ns | +197.4% | +1.3% | -99.6% |
| 5 | 1456513ns | +199.9% | +1.5% | -99.7% |
| 6 | 1477685ns | +195.8% | -0.1% | -99.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.061 | ok |
| abi_lifecycle_leaf_fresh_per_column | -0.010 | ok |
| abi_lifecycle_leaf_held_handle | -0.286 | moderate- |
| abi_lifecycle_leaf_null_entry | -0.284 | moderate- |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_fresh_per_column**: won 2/6, lost 4/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 13309583.1ns | 4439438.6ns | 299.8% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4566361.9ns | 1490803.6ns | 306.3% | HIGH |
| abi_lifecycle_leaf_held_handle | 4501410.6ns | 1486325.7ns | 302.9% | HIGH |
| abi_lifecycle_leaf_null_entry | 124514.3ns | 4897.1ns | 2542.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 4346375.4-4594987.1 ns)
  4346375.4 |#############
  4358806.0 |########################################
  4371236.6 |
  4383667.2 |
  4396097.7 |#############
  4408528.3 |
  4420958.9 |
  4433389.5 |
  4445820.1 |
  4458250.7 |
  4470681.2 |
  4483111.8 |
  4495542.4 |
  4507973.0 |
  4520403.6 |
  4532834.2 |
  4545264.8 |
  4557695.3 |
  4570125.9 |
  4582556.5 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1476042.1-1515190.4 ns)
  1476042.1 |##########################
  1477999.5 |
  1479956.9 |########################################
  1481914.4 |
  1483871.8 |
  1485829.2 |
  1487786.6 |
  1489744.0 |
  1491701.4 |
  1493658.9 |
  1495616.3 |
  1497573.7 |
  1499531.1 |
  1501488.5 |
  1503445.9 |
  1505403.4 |
  1507360.8 |
  1509318.2 |
  1511275.6 |
  1513233.0 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1453711.2-1541061.6 ns)
  1453711.2 |########################################
  1458078.7 |####################
  1462446.2 |####################
  1466813.8 |
  1471181.3 |
  1475548.8 |####################
  1479916.3 |
  1484283.9 |
  1488651.4 |
  1493018.9 |
  1497386.4 |
  1501753.9 |
  1506121.5 |
  1510489.0 |
  1514856.5 |
  1519224.0 |
  1523591.6 |
  1527959.1 |
  1532326.6 |
  1536694.1 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 4797.9-5029.6 ns)
   4797.9 |########################################
   4809.5 |
   4821.1 |
   4832.7 |####################
   4844.2 |
   4855.8 |
   4867.4 |
   4879.0 |
   4890.6 |####################
   4902.2 |
   4913.8 |
   4925.3 |####################
   4936.9 |
   4948.5 |
   4960.1 |
   4971.7 |
   4983.3 |
   4994.8 |
   5006.4 |
   5018.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=2549.7% of algo (FFI overhead may distort results)

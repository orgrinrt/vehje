# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 52653% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (2.76 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 52653%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 537.5x slower than the field

abi_lifecycle_leaf_fresh_per_batch (1.49 ms) is 537.5x the fastest (2.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (52653% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 52653% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 537.5x the fastest

Fastest abi_lifecycle_leaf_null_entry (2.76 us) to slowest abi_lifecycle_leaf_fresh_per_batch (1.49 ms): 537.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 2764.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 537.52x (fastest 2764.2 ns, slowest 1485793.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1488752ns | 1488545ns | 1482249ns | 1487645ns | 1493666ns | +1.19% |
| abi_lifecycle_leaf_fresh_per_column | 1474431ns | 1474713ns | 1470791ns | 1473752ns | 1477271ns | +0.22% |
| abi_lifecycle_leaf_held_handle | 1471245ns | 1460981ns | 1458803ns | 1460595ns | 1493441ns | base |
| abi_lifecycle_leaf_null_entry | 5056ns | 5066ns | 4813ns | 5022ns | 5227ns | -99.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1485947ns | 1479625ns | 1490731ns | +1.20% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1471619ns | 1467922ns | 1474406ns | +0.22% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1468395ns | 1456211ns | 1490428ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 2758ns | 2640ns | 2837ns | -99.81% | 0.046 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 47197.1 | 1485518.4 | 1485947.3 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 47224.9 | 1471681.6 | 1471619.2 | 1 |
| abi_lifecycle_leaf_held_handle | 49046.7 | 1469455.0 | 1468395.1 | n/a |
| abi_lifecycle_leaf_null_entry | 28575.6 | 2786.7 | 2757.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.2% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.046 | 95.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1488752ns | 1488752ns | +1.19% |
| abi_lifecycle_leaf_fresh_per_column | 1474431ns | 1474431ns | +0.22% |
| abi_lifecycle_leaf_held_handle | 1471245ns | 1471245ns | base |
| abi_lifecycle_leaf_null_entry | 5056ns | 5056ns | -99.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1458171ns | base | --- | [1456586, 1490428] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 1485793ns | no significant difference | [-4635, +33179]ns | [1481318, 1490731] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1471980ns | no significant difference | [-20374, +16737]ns | [1468471, 1474406] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_null_entry | 2764ns | -1455406.9ns (-99.8%) | [-1487719, -1453787]ns | [2671, 2837] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1456211ns | +2.2% | +0.8% | -99.8% |
| 2 | 1456962ns | +1.8% | +1.2% | -99.8% |
| 3 | 1515295ns | -2.0% | -3.1% | -99.8% |
| 4 | 1465561ns | +1.4% | +0.4% | -99.8% |
| 5 | 1457450ns | +1.5% | +1.1% | -99.8% |
| 6 | 1458892ns | +2.4% | +1.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.480 | moderate- |
| abi_lifecycle_leaf_fresh_per_column | -0.465 | moderate- |
| abi_lifecycle_leaf_held_handle | -0.146 | ok |
| abi_lifecycle_leaf_null_entry | -0.189 | ok |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_leaf_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 4505143.7ns | 1485947.3ns | 303.2% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4462776.7ns | 1471619.2ns | 303.3% | HIGH |
| abi_lifecycle_leaf_held_handle | 4459282.4ns | 1468395.1ns | 303.7% | HIGH |
| abi_lifecycle_leaf_null_entry | 120815.1ns | 2757.5ns | 4381.3% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 1479625.0-1490730.9 ns)
  1479625.0 |########################################
  1480180.3 |
  1480735.6 |
  1481290.9 |
  1481846.2 |
  1482401.5 |
  1482956.8 |########################################
  1483512.0 |
  1484067.3 |
  1484622.6 |
  1485177.9 |########################################
  1485733.2 |########################################
  1486288.5 |
  1486843.8 |
  1487399.1 |
  1487954.4 |########################################
  1488509.7 |
  1489065.0 |
  1489620.3 |
  1490175.6 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1467921.7-1474406.4 ns)
  1467921.7 |########################################
  1468245.9 |
  1468570.2 |
  1468894.4 |########################################
  1469218.6 |
  1469542.9 |
  1469867.1 |
  1470191.4 |
  1470515.6 |
  1470839.8 |########################################
  1471164.1 |
  1471488.3 |
  1471812.6 |
  1472136.8 |
  1472461.0 |
  1472785.3 |########################################
  1473109.5 |
  1473433.7 |
  1473758.0 |########################################
  1474082.2 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1456210.8-1490427.9 ns)
  1456210.8 |########################################
  1457921.7 |#############
  1459632.5 |
  1461343.4 |
  1463054.2 |
  1464765.1 |#############
  1466475.9 |
  1468186.8 |
  1469897.6 |
  1471608.5 |
  1473319.4 |
  1475030.2 |
  1476741.1 |
  1478451.9 |
  1480162.8 |
  1481873.6 |
  1483584.5 |
  1485295.3 |
  1487006.2 |
  1488717.0 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 2640.0-2837.1 ns)
   2640.0 |####################
   2649.9 |
   2659.7 |
   2669.6 |
   2679.4 |
   2689.3 |
   2699.1 |####################
   2709.0 |
   2718.8 |
   2728.7 |
   2738.6 |
   2748.4 |####################
   2758.3 |
   2768.1 |########################################
   2778.0 |
   2787.8 |
   2797.7 |
   2807.5 |
   2817.4 |
   2827.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=4351.3% of algo (FFI overhead may distort results)

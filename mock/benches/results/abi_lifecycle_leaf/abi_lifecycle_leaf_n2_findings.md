# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 42155% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (3.50 us) leads abi_lifecycle_leaf_held_handle (1.48 ms) by 42155%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.47 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 836.5x slower than the field

abi_lifecycle_leaf_fresh_per_batch (2.93 ms) is 836.5x the fastest (3.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (42155% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 42155% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 836.5x the fastest

Fastest abi_lifecycle_leaf_null_entry (3.50 us) to slowest abi_lifecycle_leaf_fresh_per_batch (2.93 ms): 836.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 3498.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 836.46x (fastest 3498.8 ns, slowest 2926554.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 2986325ns | 2929950ns | 2910520ns | 2928702ns | 3110661ns | +100.48% |
| abi_lifecycle_leaf_fresh_per_column | 1500307ns | 1485881ns | 1478812ns | 1485295ns | 1533571ns | +0.72% |
| abi_lifecycle_leaf_held_handle | 1489575ns | 1481636ns | 1463170ns | 1476950ns | 1521714ns | base |
| abi_lifecycle_leaf_null_entry | 5803ns | 5844ns | 5616ns | 5799ns | 5901ns | -99.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 2982834ns | 2907478ns | 3107215ns | +100.69% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1496981ns | 1475952ns | 1529432ns | +0.72% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1486300ns | 1460493ns | 1517957ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 3483ns | 3376ns | 3542ns | -99.77% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 72655.4 | 2942020.2 | 2982834.0 | n/a |
| abi_lifecycle_leaf_fresh_per_column | 57814.3 | 1510805.3 | 1496980.6 | 0 |
| abi_lifecycle_leaf_held_handle | 60733.7 | 1490963.0 | 1486299.8 | n/a |
| abi_lifecycle_leaf_null_entry | 28337.0 | 3589.7 | 3482.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.001 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 2986325ns | 2986325ns | +100.48% |
| abi_lifecycle_leaf_fresh_per_column | 1500307ns | 1500307ns | +0.72% |
| abi_lifecycle_leaf_held_handle | 1489575ns | 1489575ns | base |
| abi_lifecycle_leaf_null_entry | 5803ns | 5803ns | -99.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1478382ns | base | --- | [1462560, 1517957] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 2926554ns | +1450653.1ns (+98.1%) | [+1408598, +1630351]ns | [2914732, 3107215] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1482903ns | no significant difference | [-30363, +49855]ns | [1478607, 1529432] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_leaf_null_entry | 3499ns | -1474953.0ns (-99.8%) | [-1514468, -1459030]ns | [3408, 3542] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1474732ns | +98.6% | +0.7% | -99.8% |
| 2 | 1464628ns | +99.6% | +1.1% | -99.8% |
| 3 | 1482032ns | +121.5% | +5.6% | -99.8% |
| 4 | 1496324ns | +95.3% | -0.2% | -99.8% |
| 5 | 1539590ns | +90.4% | -3.8% | -99.8% |
| 6 | 1460493ns | +99.1% | +1.1% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.237 | moderate- |
| abi_lifecycle_leaf_fresh_per_column | -0.125 | ok |
| abi_lifecycle_leaf_held_handle | -0.128 | ok |
| abi_lifecycle_leaf_null_entry | -0.499 | moderate- |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_fresh_per_column**: won 2/6, lost 4/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 8892987.4ns | 2982834.0ns | 298.1% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4654170.9ns | 1496980.6ns | 310.9% | HIGH |
| abi_lifecycle_leaf_held_handle | 4532751.9ns | 1486299.8ns | 305.0% | HIGH |
| abi_lifecycle_leaf_null_entry | 121338.4ns | 3482.9ns | 3483.8% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 2907477.5-3107215.2 ns)
  2907477.5 |####################
  2917464.4 |########################################
  2927451.3 |########################################
  2937438.2 |
  2947425.0 |
  2957411.9 |
  2967398.8 |
  2977385.7 |
  2987372.6 |
  2997359.5 |
  3007346.4 |
  3017333.2 |
  3027320.1 |
  3037307.0 |
  3047293.9 |
  3057280.8 |
  3067267.7 |
  3077254.5 |
  3087241.4 |
  3097228.3 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1475951.7-1529432.2 ns)
  1475951.7 |########################################
  1478625.7 |########################################
  1481299.8 |########################################
  1483973.8 |########################################
  1486647.8 |
  1489321.8 |
  1491995.9 |########################################
  1494669.9 |
  1497343.9 |
  1500017.9 |
  1502692.0 |
  1505366.0 |
  1508040.0 |
  1510714.1 |
  1513388.1 |
  1516062.1 |
  1518736.1 |
  1521410.2 |
  1524084.2 |
  1526758.2 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1460493.3-1517956.9 ns)
  1460493.3 |########################################
  1463366.5 |########################################
  1466239.7 |
  1469112.8 |
  1471986.0 |########################################
  1474859.2 |
  1477732.4 |
  1480605.5 |########################################
  1483478.7 |
  1486351.9 |
  1489225.1 |
  1492098.3 |
  1494971.4 |########################################
  1497844.6 |
  1500717.8 |
  1503591.0 |
  1506464.1 |
  1509337.3 |
  1512210.5 |
  1515083.7 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 3376.2-3542.2 ns)
   3376.2 |########################################
   3384.5 |
   3392.8 |
   3401.1 |
   3409.4 |
   3417.7 |
   3426.0 |
   3434.3 |########################################
   3442.6 |
   3450.9 |
   3459.2 |
   3467.5 |
   3475.8 |########################################
   3484.1 |
   3492.4 |
   3500.7 |
   3509.0 |########################################
   3517.3 |
   3525.6 |
   3533.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=306.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=3475.7% of algo (FFI overhead may distort results)

# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.51 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 3.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 42753% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (3.42 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.47 ms) by 42753%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.50 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 440.3x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.51 ms) is 440.3x the fastest (3.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (42753% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 42753% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 440.3x the fastest

Fastest abi_cross_scalar_leaf_null_entry (3.42 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.51 ms): 440.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 3422.9 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 440.28x (fastest 3422.9 ns, slowest 1507017.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1484941ns | 1485115ns | 1471462ns | 1482538ns | 1495285ns | -1.57% |
| abi_cross_scalar_leaf_inproc_direct | 1508576ns | 1509922ns | 1499231ns | 1508358ns | 1513574ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1471881ns | 1469849ns | 1465236ns | 1468355ns | 1480491ns | -2.43% |
| abi_cross_scalar_leaf_null_entry | 5692ns | 5670ns | 5565ns | 5653ns | 5813ns | -99.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1481652ns | 1468470ns | 1491682ns | -1.59% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1505561ns | 1496191ns | 1510691ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1468934ns | 1462286ns | 1477554ns | -2.43% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 3426ns | 3353ns | 3482ns | -99.77% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 58980.1 | 1480770.3 | 1481651.6 | 2 |
| abi_cross_scalar_leaf_inproc_direct | 8162.1 | 1504428.3 | 1505560.8 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 8002.3 | 1469519.4 | 1468934.3 | n/a |
| abi_cross_scalar_leaf_null_entry | 28153.9 | 3543.0 | 3426.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.001 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1484941ns | 1484941ns | -1.57% |
| abi_cross_scalar_leaf_inproc_direct | 1508576ns | 1508576ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1471881ns | 1471881ns | -2.43% |
| abi_cross_scalar_leaf_null_entry | 5692ns | 5692ns | -99.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1507018ns | base | --- | [1498974, 1510691] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1481846ns | -24294.8ns (-1.6%) | [-32998, -14435]ns | [1471427, 1491682] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1466820ns | -40715.4ns (-2.7%) | [-44423, -24741]ns | [1462429, 1477554] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_leaf_null_entry | 3423ns | -1503546.2ns (-99.8%) | [-1507291, -1495565]ns | [3374, 3482] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1501756ns | -1.1% | -2.6% | -99.8% |
| 2 | 1511007ns | -2.1% | -2.8% | -99.8% |
| 3 | 1510375ns | -0.8% | -3.0% | -99.8% |
| 4 | 1505638ns | -1.4% | -2.9% | -99.8% |
| 5 | 1508398ns | -2.3% | -2.2% | -99.8% |
| 6 | 1496191ns | -1.9% | -1.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.136 | ok |
| abi_cross_scalar_leaf_inproc_direct | -0.126 | ok |
| abi_cross_scalar_leaf_inproc_fnptr | 0.211 | moderate+ |
| abi_cross_scalar_leaf_null_entry | 0.164 | ok |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4509479.5ns | 1481651.6ns | 304.4% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4525829.2ns | 1505560.8ns | 300.6% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4414351.3ns | 1468934.3ns | 300.5% | HIGH |
| abi_cross_scalar_leaf_null_entry | 120905.1ns | 3426.5ns | 3528.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1468470.4-1491681.6 ns)
  1468470.4 |########################################
  1469631.0 |
  1470791.5 |
  1471952.1 |
  1473112.6 |
  1474273.2 |########################################
  1475433.8 |
  1476594.3 |
  1477754.9 |
  1478915.5 |########################################
  1480076.0 |
  1481236.6 |
  1482397.1 |
  1483557.7 |########################################
  1484718.3 |########################################
  1485878.8 |
  1487039.4 |
  1488200.0 |
  1489360.5 |
  1490521.1 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1496191.2-1510690.9 ns)
  1496191.2 |########################################
  1496916.2 |
  1497641.2 |
  1498366.1 |
  1499091.1 |
  1499816.1 |
  1500541.1 |
  1501266.1 |########################################
  1501991.1 |
  1502716.0 |
  1503441.0 |
  1504166.0 |
  1504891.0 |
  1505616.0 |########################################
  1506341.0 |
  1507065.9 |
  1507790.9 |########################################
  1508515.9 |
  1509240.9 |
  1509965.9 |########################################
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1462286.2-1477554.1 ns)
  1462286.2 |########################################
  1463049.6 |
  1463813.0 |
  1464576.4 |####################
  1465339.8 |
  1466103.2 |
  1466866.6 |
  1467630.0 |
  1468393.4 |####################
  1469156.8 |
  1469920.2 |
  1470683.6 |
  1471447.0 |
  1472210.4 |
  1472973.8 |
  1473737.2 |
  1474500.6 |####################
  1475264.0 |
  1476027.4 |
  1476790.8 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 3353.3-3482.3 ns)
   3353.3 |########################################
   3359.8 |
   3366.2 |
   3372.7 |
   3379.1 |
   3385.6 |
   3392.0 |########################################
   3398.5 |
   3404.9 |
   3411.4 |
   3417.8 |########################################
   3424.2 |########################################
   3430.7 |
   3437.2 |
   3443.6 |########################################
   3450.1 |
   3456.5 |
   3463.0 |
   3469.4 |
   3475.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=3539.1% of algo (FFI overhead may distort results)

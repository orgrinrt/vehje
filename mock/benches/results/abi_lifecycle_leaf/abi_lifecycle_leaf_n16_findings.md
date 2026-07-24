# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 57141% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (2.56 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 57141%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 644.0x slower than the field

abi_lifecycle_leaf_fresh_per_batch (1.65 ms) is 644.0x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (57141% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 57141% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 644.0x the fastest

Fastest abi_lifecycle_leaf_null_entry (2.56 us) to slowest abi_lifecycle_leaf_fresh_per_batch (1.65 ms): 644.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 2556.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 644.03x (fastest 2556.1 ns, slowest 1646184.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1652950ns | 1649170ns | 1642422ns | 1647499ns | 1666392ns | +11.99% |
| abi_lifecycle_leaf_fresh_per_column | 1477913ns | 1477454ns | 1474355ns | 1476900ns | 1481211ns | +0.13% |
| abi_lifecycle_leaf_held_handle | 1475993ns | 1466236ns | 1459058ns | 1464028ns | 1502407ns | base |
| abi_lifecycle_leaf_null_entry | 4899ns | 4837ns | 4678ns | 4810ns | 5143ns | -99.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1649897ns | 1639549ns | 1663140ns | +12.03% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1474887ns | 1471338ns | 1478076ns | +0.15% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1472706ns | 1456273ns | 1498708ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 2579ns | 2468ns | 2699ns | -99.82% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 55152.8 | 1653091.7 | 1649896.5 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 52728.9 | 1476721.6 | 1474887.3 | 1 |
| abi_lifecycle_leaf_held_handle | 55651.0 | 1472009.4 | 1472706.4 | n/a |
| abi_lifecycle_leaf_null_entry | 29734.8 | 2660.3 | 2578.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.006 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1652950ns | 1652950ns | +11.99% |
| abi_lifecycle_leaf_fresh_per_column | 1477913ns | 1477913ns | +0.13% |
| abi_lifecycle_leaf_held_handle | 1475993ns | 1475993ns | base |
| abi_lifecycle_leaf_null_entry | 4899ns | 4899ns | -99.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1463106ns | base | --- | [1456305, 1498708] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 1646185ns | +180564.8ns (+12.3%) | [+149570, +201436]ns | [1640365, 1663140] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1474521ns | no significant difference | [-23521, +19896]ns | [1472064, 1478076] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_leaf_null_entry | 2556ns | -1460566.6ns (-99.8%) | [-1496041, -1453776]ns | [2480, 2699] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1456337ns | +13.1% | +1.4% | -99.8% |
| 2 | 1456273ns | +12.6% | +1.3% | -99.8% |
| 3 | 1527516ns | +8.1% | -3.7% | -99.8% |
| 4 | 1469901ns | +11.9% | +0.6% | -99.8% |
| 5 | 1462884ns | +14.5% | +0.7% | -99.8% |
| 6 | 1463328ns | +12.2% | +0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.392 | moderate- |
| abi_lifecycle_leaf_fresh_per_column | -0.433 | moderate- |
| abi_lifecycle_leaf_held_handle | -0.178 | ok |
| abi_lifecycle_leaf_null_entry | -0.450 | moderate- |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 5007190.1ns | 1649896.5ns | 303.5% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4480449.0ns | 1474887.3ns | 303.8% | HIGH |
| abi_lifecycle_leaf_held_handle | 4476583.5ns | 1472706.4ns | 304.0% | HIGH |
| abi_lifecycle_leaf_null_entry | 120442.7ns | 2578.5ns | 4671.0% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 1639549.2-1663139.6 ns)
  1639549.2 |########################################
  1640728.7 |########################################
  1641908.2 |
  1643087.8 |
  1644267.3 |########################################
  1645446.8 |
  1646626.3 |########################################
  1647805.8 |
  1648985.4 |
  1650164.9 |
  1651344.4 |########################################
  1652523.9 |
  1653703.4 |
  1654883.0 |
  1656062.5 |
  1657242.0 |
  1658421.5 |
  1659601.0 |
  1660780.6 |
  1661960.1 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1471338.3-1478076.4 ns)
  1471338.3 |########################################
  1471675.2 |
  1472012.1 |
  1472349.0 |
  1472685.9 |########################################
  1473022.8 |
  1473359.7 |
  1473696.7 |########################################
  1474033.6 |
  1474370.5 |
  1474707.4 |
  1475044.3 |########################################
  1475381.2 |
  1475718.1 |
  1476055.0 |
  1476391.9 |
  1476728.8 |
  1477065.7 |########################################
  1477402.6 |
  1477739.5 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1456273.3-1498708.5 ns)
  1456273.3 |########################################
  1458395.1 |
  1460516.8 |
  1462638.6 |########################################
  1464760.3 |
  1466882.1 |
  1469003.9 |####################
  1471125.6 |
  1473247.4 |
  1475369.1 |
  1477490.9 |
  1479612.7 |
  1481734.4 |
  1483856.2 |
  1485977.9 |
  1488099.7 |
  1490221.5 |
  1492343.2 |
  1494465.0 |
  1496586.7 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 2467.9-2699.1 ns)
   2467.9 |####################
   2479.5 |
   2491.0 |####################
   2502.6 |
   2514.2 |
   2525.7 |####################
   2537.3 |
   2548.8 |
   2560.4 |
   2572.0 |
   2583.5 |########################################
   2595.1 |
   2606.6 |
   2618.2 |
   2629.8 |
   2641.3 |
   2652.9 |
   2664.5 |
   2676.0 |
   2687.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=4665.5% of algo (FFI overhead may distort results)

# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 58396% faster than the next best (abi_entry_form_leaf_per_w_set)

abi_entry_form_leaf_null_entry (2.55 us) leads abi_entry_form_leaf_per_w_set (1.49 ms) by 58396%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.50 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_scalar_anchor is an outlier: 591.5x slower than the field

abi_entry_form_leaf_scalar_anchor (1.51 ms) is 591.5x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_leaf_runtime_w shows warm-up / thermal drift (autocorr +0.56)

abi_entry_form_leaf_runtime_w's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_scalar_anchor} (58396% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_scalar_anchor} with a 58396% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 591.5x the fastest

Fastest abi_entry_form_leaf_null_entry (2.55 us) to slowest abi_entry_form_leaf_scalar_anchor (1.51 ms): 591.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 2551.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 591.49x (fastest 2551.2 ns, slowest 1509037.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1505301ns | 1498703ns | 1472030ns | 1492247ns | 1541519ns | -0.31% |
| abi_entry_form_leaf_null_entry | 4868ns | 4849ns | 4780ns | 4835ns | 4961ns | -99.68% |
| abi_entry_form_leaf_per_w_set | 1501165ns | 1495724ns | 1463755ns | 1487794ns | 1539926ns | -0.58% |
| abi_entry_form_leaf_runtime_w | 1509970ns | 1508645ns | 1477962ns | 1498701ns | 1542879ns | base |
| abi_entry_form_leaf_scalar_anchor | 1514037ns | 1512616ns | 1463435ns | 1505536ns | 1552089ns | +0.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1501789ns | 1468886ns | 1537611ns | -0.30% | 0.000 |
| abi_entry_form_leaf_null_entry | 2562ns | 2493ns | 2622ns | -99.83% | 0.006 |
| abi_entry_form_leaf_per_w_set | 1497678ns | 1460548ns | 1535959ns | -0.57% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1506325ns | 1474894ns | 1538569ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1510546ns | 1460556ns | 1548275ns | +0.28% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 72221.5 | 1505617.5 | 1501789.2 | 0 |
| abi_entry_form_leaf_null_entry | 28894.0 | 2668.7 | 2562.4 | n/a |
| abi_entry_form_leaf_per_w_set | 69674.6 | 1495816.1 | 1497677.9 | n/a |
| abi_entry_form_leaf_runtime_w | 70538.6 | 1505699.2 | 1506325.4 | n/a |
| abi_entry_form_leaf_scalar_anchor | 69738.1 | 1511711.3 | 1510546.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_leaf_null_entry | 0.006 | 97.7% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.2% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.2% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1505301ns | 1505301ns | -0.31% |
| abi_entry_form_leaf_null_entry | 4868ns | 4868ns | -99.68% |
| abi_entry_form_leaf_per_w_set | 1501165ns | 1501165ns | -0.58% |
| abi_entry_form_leaf_runtime_w | 1509970ns | 1509970ns | base |
| abi_entry_form_leaf_scalar_anchor | 1514037ns | 1514037ns | +0.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1505029ns | base | --- | [1475378, 1538569] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1495342ns | no significant difference | [-19674, +11275]ns | [1472414, 1537611] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_null_entry | 2551ns | -1502514.6ns (-99.8%) | [-1535995, -1472780]ns | [2514, 2622] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1492376ns | no significant difference | [-29620, +12376]ns | [1464699, 1535959] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1509037ns | no significant difference | [-15069, +31351]ns | [1474326, 1548275] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1545928ns | -0.9% | -99.8% | -2.2% | -1.0% |
| 2 | 1531210ns | +0.8% | -99.8% | +1.1% | -0.2% |
| 3 | 1515955ns | -0.7% | -99.8% | +0.5% | +3.3% |
| 4 | 1494102ns | -1.7% | -99.8% | -1.7% | -0.3% |
| 5 | 1474894ns | +0.7% | -99.8% | -0.1% | -1.0% |
| 6 | 1475863ns | +0.0% | -99.8% | -1.0% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.482 | moderate+ |
| abi_entry_form_leaf_null_entry | -0.474 | moderate- |
| abi_entry_form_leaf_per_w_set | 0.467 | moderate+ |
| abi_entry_form_leaf_runtime_w | 0.564 | HIGH+ (drift/warm-up) |
| abi_entry_form_leaf_scalar_anchor | 0.321 | moderate+ |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 3/6, lost 2/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_leaf_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4580029.0ns | 1501789.2ns | 305.0% | HIGH |
| abi_entry_form_leaf_null_entry | 119345.3ns | 2562.4ns | 4657.6% | HIGH |
| abi_entry_form_leaf_per_w_set | 4563416.8ns | 1497677.9ns | 304.7% | HIGH |
| abi_entry_form_leaf_runtime_w | 4595163.1ns | 1506325.4ns | 305.1% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4605435.8ns | 1510546.2ns | 304.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1468885.8-1537611.4 ns)
  1468885.8 |########################################
  1472322.1 |
  1475758.4 |########################################
  1479194.6 |
  1482630.9 |########################################
  1486067.2 |
  1489503.5 |
  1492939.8 |
  1496376.1 |
  1499812.3 |
  1503248.6 |########################################
  1506684.9 |
  1510121.2 |
  1513557.5 |
  1516993.8 |
  1520430.0 |
  1523866.3 |
  1527302.6 |
  1530738.9 |########################################
  1534175.2 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 2492.9-2621.9 ns)
   2492.9 |########################################
   2499.3 |
   2505.8 |
   2512.2 |
   2518.7 |
   2525.2 |
   2531.6 |########################################
   2538.0 |########################################
   2544.5 |
   2550.9 |
   2557.4 |########################################
   2563.8 |
   2570.3 |
   2576.8 |
   2583.2 |########################################
   2589.6 |
   2596.1 |
   2602.5 |
   2609.0 |
   2615.4 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1460548.3-1535958.9 ns)
  1460548.3 |########################################
  1464318.8 |
  1468089.4 |########################################
  1471859.9 |########################################
  1475630.4 |
  1479401.0 |
  1483171.5 |
  1486942.0 |
  1490712.6 |
  1494483.1 |
  1498253.6 |
  1502024.2 |
  1505794.7 |
  1509565.2 |########################################
  1513335.8 |
  1517106.3 |
  1520876.8 |########################################
  1524647.4 |
  1528417.9 |
  1532188.4 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1474893.7-1538569.1 ns)
  1474893.7 |########################################
  1478077.5 |
  1481261.2 |
  1484445.0 |
  1487628.8 |
  1490812.6 |
  1493996.3 |####################
  1497180.1 |
  1500363.9 |
  1503547.7 |
  1506731.4 |
  1509915.2 |
  1513099.0 |####################
  1516282.7 |
  1519466.5 |
  1522650.3 |
  1525834.1 |
  1529017.8 |####################
  1532201.6 |
  1535385.4 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1460556.2-1548275.4 ns)
  1460556.2 |####################
  1464942.2 |
  1469328.1 |
  1473714.1 |
  1478100.0 |
  1482486.0 |
  1486872.0 |########################################
  1491257.9 |
  1495643.9 |
  1500029.8 |
  1504415.8 |
  1508801.8 |
  1513187.7 |
  1517573.7 |
  1521959.6 |
  1526345.6 |########################################
  1530731.6 |
  1535117.5 |
  1539503.5 |
  1543889.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=4683.9% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **abi_entry_form_leaf_runtime_w**: bridge=305.7% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=305.4% of algo (FFI overhead may distort results)

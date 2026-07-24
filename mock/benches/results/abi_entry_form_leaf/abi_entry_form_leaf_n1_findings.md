# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_leaf_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_leaf_runtime_w has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_leaf_null_entry at 4.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_leaf_null_entry dominates: 29908% faster than the next best (abi_entry_form_leaf_per_w_set)

abi_entry_form_leaf_null_entry (4.93 us) leads abi_entry_form_leaf_per_w_set (1.48 ms) by 29908%, a clear separation rather than a photo finish. CV 7.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.50 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_runtime_w is an outlier: 304.6x slower than the field

abi_entry_form_leaf_runtime_w (1.50 ms) is 304.6x the fastest (4.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w} (29908% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w} with a 29908% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 304.6x the fastest

Fastest abi_entry_form_leaf_null_entry (4.93 us) to slowest abi_entry_form_leaf_runtime_w (1.50 ms): 304.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 4933.1 ns median (-99.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 304.58x (fastest 4933.1 ns, slowest 1502510.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1589314ns | 1485248ns | 1468944ns | 1483455ns | 1808286ns | -2.45% |
| abi_entry_form_leaf_null_entry | 7509ns | 7271ns | 6992ns | 7220ns | 8201ns | -99.54% |
| abi_entry_form_leaf_per_w_set | 1521447ns | 1483903ns | 1473414ns | 1482417ns | 1604008ns | -6.62% |
| abi_entry_form_leaf_runtime_w | 1629265ns | 1505961ns | 1493792ns | 1502198ns | 1887602ns | base |
| abi_entry_form_leaf_scalar_anchor | 1522125ns | 1489931ns | 1468005ns | 1483676ns | 1606858ns | -6.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1585728ns | 1466063ns | 1803745ns | -2.45% | 0.000 |
| abi_entry_form_leaf_null_entry | 5115ns | 4787ns | 5602ns | -99.69% | 0.000 |
| abi_entry_form_leaf_per_w_set | 1517984ns | 1470367ns | 1600201ns | -6.61% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1625484ns | 1490183ns | 1883178ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1518896ns | 1465326ns | 1603204ns | -6.56% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 71246.2 | 1551892.1 | 1585728.1 | n/a |
| abi_entry_form_leaf_null_entry | 30438.5 | 5179.0 | 5115.5 | n/a |
| abi_entry_form_leaf_per_w_set | 67622.2 | 1518838.0 | 1517983.7 | n/a |
| abi_entry_form_leaf_runtime_w | 72313.5 | 1604982.8 | 1625483.6 | n/a |
| abi_entry_form_leaf_scalar_anchor | 59423.8 | 1515638.1 | 1518895.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.3% |
| abi_entry_form_leaf_null_entry | 0.000 | 97.0% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.3% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.3% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1589314ns | 1589314ns | -2.45% |
| abi_entry_form_leaf_null_entry | 7509ns | 7509ns | -99.54% |
| abi_entry_form_leaf_per_w_set | 1521447ns | 1521447ns | -6.62% |
| abi_entry_form_leaf_runtime_w | 1629265ns | 1629265ns | base |
| abi_entry_form_leaf_scalar_anchor | 1522125ns | 1522125ns | -6.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1502511ns | base | --- | [1490763, 1883178] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1482101ns | no significant difference | [-116853, +17822]ns | [1471338, 1803745] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_leaf_null_entry | 4933ns | -1497663.4ns (-99.7%) | [-1877575, -1485866]ns | [4811, 5602] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1480318ns | -20938.9ns (-1.4%) | [-298218, -3343]ns | [1473433, 1600201] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1486761ns | no significant difference | [-320366, +23439]ns | [1466723, 1603204] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1499999ns | -1.0% | -99.7% | -2.0% | -2.1% |
| 2 | 1490183ns | -0.9% | -99.7% | -0.9% | -0.9% |
| 3 | 1491342ns | -1.7% | -99.7% | -0.5% | +0.4% |
| 4 | 1505022ns | -1.7% | -99.7% | -1.9% | -2.6% |
| 5 | 2159334ns | -9.6% | -99.7% | -26.2% | -27.8% |
| 6 | 1607021ns | +3.1% | -99.7% | +0.1% | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.127 | ok |
| abi_entry_form_leaf_null_entry | 0.167 | ok |
| abi_entry_form_leaf_per_w_set | 0.412 | moderate+ |
| abi_entry_form_leaf_runtime_w | -0.065 | ok |
| abi_entry_form_leaf_scalar_anchor | 0.278 | moderate+ |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 5/6, lost 1/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 5/6, lost 0/6
- **abi_entry_form_leaf_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4768212.1ns | 1585728.1ns | 300.7% | HIGH |
| abi_entry_form_leaf_null_entry | 128071.5ns | 5115.5ns | 2503.6% | HIGH |
| abi_entry_form_leaf_per_w_set | 4633846.0ns | 1517983.7ns | 305.3% | HIGH |
| abi_entry_form_leaf_runtime_w | 4835465.1ns | 1625483.6ns | 297.5% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4611981.9ns | 1518895.8ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1466062.9-1803745.4 ns)
  1466062.9 |########################################
  1482947.0 |#############
  1499831.1 |
  1516715.3 |
  1533599.4 |
  1550483.5 |
  1567367.6 |
  1584251.8 |
  1601135.9 |
  1618020.0 |
  1634904.1 |
  1651788.3 |#############
  1668672.4 |
  1685556.5 |
  1702440.6 |
  1719324.8 |
  1736208.9 |
  1753093.0 |
  1769977.1 |
  1786861.3 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 4786.7-5602.3 ns)
   4786.7 |########################################
   4827.5 |########################################
   4868.3 |########################################
   4909.0 |
   4949.8 |########################################
   4990.6 |
   5031.4 |
   5072.2 |
   5112.9 |
   5153.7 |
   5194.5 |
   5235.3 |
   5276.1 |
   5316.8 |
   5357.6 |
   5398.4 |########################################
   5439.2 |
   5480.0 |
   5520.7 |
   5561.5 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1470367.1-1600200.6 ns)
  1470367.1 |########################################
  1476858.8 |
  1483350.5 |#############
  1489842.1 |
  1496333.8 |
  1502825.5 |
  1509317.2 |
  1515808.8 |
  1522300.5 |
  1528792.2 |
  1535283.9 |
  1541775.5 |
  1548267.2 |
  1554758.9 |
  1561250.6 |
  1567742.2 |
  1574233.9 |
  1580725.6 |
  1587217.2 |#############
  1593708.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1490183.3-1883177.5 ns)
  1490183.3 |########################################
  1509833.0 |
  1529482.7 |
  1549132.4 |
  1568782.1 |
  1588431.9 |##########
  1608081.6 |
  1627731.3 |
  1647381.0 |
  1667030.7 |
  1686680.4 |
  1706330.1 |
  1725979.8 |
  1745629.5 |
  1765279.2 |
  1784928.9 |
  1804578.7 |
  1824228.4 |
  1843878.1 |
  1863527.8 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1465325.8-1603203.9 ns)
  1465325.8 |########################################
  1472219.7 |####################
  1479113.6 |
  1486007.5 |
  1492901.4 |####################
  1499795.3 |
  1506689.2 |
  1513583.2 |
  1520477.1 |
  1527371.0 |
  1534264.9 |
  1541158.8 |
  1548052.7 |
  1554946.6 |####################
  1561840.5 |
  1568734.4 |
  1575628.3 |
  1582522.2 |
  1589416.1 |
  1596310.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=2586.6% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: bridge=307.2% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=303.6% of algo (FFI overhead may distort results)

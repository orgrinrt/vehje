# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 54502% faster than the next best (abi_entry_form_leaf_scalar_anchor)

abi_entry_form_leaf_null_entry (2.71 us) leads abi_entry_form_leaf_scalar_anchor (1.48 ms) by 54502%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.48 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_dispatch_table is an outlier: 548.6x slower than the field

abi_entry_form_leaf_dispatch_table (1.48 ms) is 548.6x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table} (54502% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table} with a 54502% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 548.6x the fastest

Fastest abi_entry_form_leaf_null_entry (2.71 us) to slowest abi_entry_form_leaf_dispatch_table (1.48 ms): 548.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 2705.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 548.60x (fastest 2705.4 ns, slowest 1484181.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1508832ns | 1487555ns | 1468050ns | 1485265ns | 1564575ns | -0.88% |
| abi_entry_form_leaf_null_entry | 5021ns | 4971ns | 4911ns | 4960ns | 5168ns | -99.67% |
| abi_entry_form_leaf_per_w_set | 1559930ns | 1485944ns | 1470840ns | 1482383ns | 1720796ns | +2.48% |
| abi_entry_form_leaf_runtime_w | 1522239ns | 1484685ns | 1469805ns | 1481866ns | 1609017ns | base |
| abi_entry_form_leaf_scalar_anchor | 1607914ns | 1480203ns | 1467904ns | 1476811ns | 1874573ns | +5.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1505382ns | 1464546ns | 1560882ns | -0.79% | 0.000 |
| abi_entry_form_leaf_null_entry | 2718ns | 2645ns | 2801ns | -99.82% | 0.047 |
| abi_entry_form_leaf_per_w_set | 1556524ns | 1467712ns | 1717002ns | +2.58% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1517303ns | 1466602ns | 1600651ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1604561ns | 1465209ns | 1870333ns | +5.75% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 63427.5 | 1488413.8 | 1505382.5 | 0 |
| abi_entry_form_leaf_null_entry | 28137.1 | 2766.2 | 2718.1 | n/a |
| abi_entry_form_leaf_per_w_set | 68847.3 | 1553227.5 | 1556523.8 | n/a |
| abi_entry_form_leaf_runtime_w | 63192.7 | 1520313.1 | 1517302.8 | n/a |
| abi_entry_form_leaf_scalar_anchor | 67251.0 | 1548986.1 | 1604560.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_leaf_null_entry | 0.047 | 97.8% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.2% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.2% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1508832ns | 1508832ns | -0.88% |
| abi_entry_form_leaf_null_entry | 5021ns | 5021ns | -99.67% |
| abi_entry_form_leaf_per_w_set | 1559930ns | 1559930ns | +2.48% |
| abi_entry_form_leaf_runtime_w | 1522239ns | 1522239ns | base |
| abi_entry_form_leaf_scalar_anchor | 1607914ns | 1607914ns | +5.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1481559ns | base | --- | [1469698, 1600651] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1484182ns | no significant difference | [-47307, +13022]ns | [1471084, 1560882] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_null_entry | 2705ns | -1478887.9ns (-99.8%) | [-1597850, -1467016]ns | [2648, 2801] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1482546ns | no significant difference | [-12818, +120450]ns | [1470022, 1717002] | no | 1.0000 | 0.6875 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1477193ns | no significant difference | [-15015, +273781]ns | [1466156, 1870333] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1466602ns | +0.8% | -99.8% | +0.4% | +0.6% |
| 2 | 1481767ns | +0.4% | -99.8% | -0.9% | -0.2% |
| 3 | 1489548ns | -0.6% | -99.8% | -0.8% | -1.6% |
| 4 | 1711755ns | -4.5% | -99.8% | +13.1% | +25.7% |
| 5 | 1472795ns | +1.0% | -99.8% | +1.0% | -0.4% |
| 6 | 1481350ns | -1.1% | -99.8% | +1.1% | +7.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | -0.187 | ok |
| abi_entry_form_leaf_null_entry | -0.215 | moderate- |
| abi_entry_form_leaf_per_w_set | -0.217 | moderate- |
| abi_entry_form_leaf_runtime_w | -0.212 | moderate- |
| abi_entry_form_leaf_scalar_anchor | -0.312 | moderate- |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 2/6, lost 4/6
- **abi_entry_form_leaf_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4558495.8ns | 1505382.5ns | 302.8% | HIGH |
| abi_entry_form_leaf_null_entry | 120192.8ns | 2718.1ns | 4421.9% | HIGH |
| abi_entry_form_leaf_per_w_set | 4668355.9ns | 1556523.8ns | 299.9% | HIGH |
| abi_entry_form_leaf_runtime_w | 4596219.6ns | 1517302.8ns | 302.9% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4729360.7ns | 1604560.7ns | 294.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1464546.2-1560881.7 ns)
  1464546.2 |####################
  1469363.0 |
  1474179.8 |####################
  1478996.5 |####################
  1483813.3 |########################################
  1488630.1 |
  1493446.9 |
  1498263.6 |
  1503080.4 |
  1507897.2 |
  1512714.0 |
  1517530.7 |
  1522347.5 |
  1527164.3 |
  1531981.1 |
  1536797.8 |
  1541614.6 |
  1546431.4 |
  1551248.2 |
  1556064.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 2644.6-2801.2 ns)
   2644.6 |########################################
   2652.4 |
   2660.3 |
   2668.1 |
   2675.9 |
   2683.8 |####################
   2691.6 |
   2699.4 |
   2707.3 |
   2715.1 |####################
   2722.9 |
   2730.8 |
   2738.6 |####################
   2746.4 |
   2754.3 |
   2762.1 |
   2769.9 |
   2777.8 |
   2785.6 |
   2793.4 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1467712.1-1717002.5 ns)
  1467712.1 |########################################
  1480176.6 |#############
  1492641.1 |#############
  1505105.7 |
  1517570.2 |
  1530034.7 |
  1542499.2 |
  1554963.7 |
  1567428.3 |
  1579892.8 |
  1592357.3 |
  1604821.8 |
  1617286.3 |
  1629750.9 |
  1642215.4 |
  1654679.9 |
  1667144.4 |
  1679608.9 |
  1692073.5 |
  1704538.0 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1466602.1-1600651.5 ns)
  1466602.1 |########################################
  1473304.6 |
  1480007.0 |########################################
  1486709.5 |####################
  1493412.0 |
  1500114.4 |
  1506816.9 |
  1513519.4 |
  1520221.8 |
  1526924.3 |
  1533626.8 |
  1540329.2 |
  1547031.7 |
  1553734.2 |
  1560436.6 |
  1567139.1 |
  1573841.6 |
  1580544.0 |
  1587246.5 |
  1593949.0 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1465209.2-1870333.1 ns)
  1465209.2 |########################################
  1485465.4 |
  1505721.6 |
  1525977.8 |
  1546234.0 |
  1566490.2 |
  1586746.4 |##########
  1607002.6 |
  1627258.8 |
  1647515.0 |
  1667771.1 |
  1688027.3 |
  1708283.5 |
  1728539.7 |
  1748795.9 |
  1769052.1 |
  1789308.3 |
  1809564.5 |
  1829820.7 |
  1850076.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=4437.8% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=304.6% of algo (FFI overhead may distort results)

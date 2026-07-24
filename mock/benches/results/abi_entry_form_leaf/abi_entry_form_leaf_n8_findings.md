# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 51668% faster than the next best (abi_entry_form_leaf_scalar_anchor)

abi_entry_form_leaf_null_entry (3.06 us) leads abi_entry_form_leaf_scalar_anchor (1.59 ms) by 51668%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.59 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_per_w_set is an outlier: 523.3x slower than the field

abi_entry_form_leaf_per_w_set (1.60 ms) is 523.3x the fastest (3.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_per_w_set} (51668% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_per_w_set} with a 51668% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 523.3x the fastest

Fastest abi_entry_form_leaf_null_entry (3.06 us) to slowest abi_entry_form_leaf_per_w_set (1.60 ms): 523.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 3063.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 523.28x (fastest 3063.3 ns, slowest 1602993.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1592183ns | 1604610ns | 1519115ns | 1579606ns | 1647584ns | -0.13% |
| abi_entry_form_leaf_null_entry | 5541ns | 5516ns | 5232ns | 5427ns | 5866ns | -99.65% |
| abi_entry_form_leaf_per_w_set | 1626598ns | 1606695ns | 1508371ns | 1597828ns | 1728868ns | +2.03% |
| abi_entry_form_leaf_runtime_w | 1594190ns | 1597520ns | 1567841ns | 1591942ns | 1610735ns | base |
| abi_entry_form_leaf_scalar_anchor | 1605469ns | 1589536ns | 1537565ns | 1581698ns | 1675078ns | +0.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1588061ns | 1515597ns | 1642881ns | -0.15% | 0.000 |
| abi_entry_form_leaf_null_entry | 3110ns | 2965ns | 3294ns | -99.80% | 0.003 |
| abi_entry_form_leaf_per_w_set | 1622626ns | 1504459ns | 1724503ns | +2.03% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1590399ns | 1564117ns | 1606702ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1601591ns | 1533355ns | 1670984ns | +0.70% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 77633.3 | 1678166.2 | 1588060.6 | 2 |
| abi_entry_form_leaf_null_entry | 29437.0 | 3212.5 | 3110.2 | n/a |
| abi_entry_form_leaf_per_w_set | 82906.2 | 1618447.3 | 1622626.1 | n/a |
| abi_entry_form_leaf_runtime_w | 72533.8 | 1592143.8 | 1590399.1 | n/a |
| abi_entry_form_leaf_scalar_anchor | 81821.4 | 1605493.2 | 1601591.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_leaf_null_entry | 0.003 | 96.8% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.2% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.2% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1592183ns | 1592183ns | -0.13% |
| abi_entry_form_leaf_null_entry | 5541ns | 5541ns | -99.65% |
| abi_entry_form_leaf_per_w_set | 1626598ns | 1626598ns | +2.03% |
| abi_entry_form_leaf_runtime_w | 1594190ns | 1594190ns | base |
| abi_entry_form_leaf_scalar_anchor | 1605469ns | 1605469ns | +0.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1593745ns | base | --- | [1570750, 1606702] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1600896ns | no significant difference | [-81579, +59704]ns | [1520405, 1642881] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_leaf_null_entry | 3063ns | -1590621.5ns (-99.8%) | [-1603639, -1567606]ns | [2974, 3294] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1602994ns | no significant difference | [-60802, +153753]ns | [1540382, 1724503] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1585836ns | no significant difference | [-58749, +100235]ns | [1547954, 1670984] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1603983ns | +0.9% | -99.8% | +1.0% | -2.6% |
| 2 | 1577382ns | +5.7% | -99.8% | +14.4% | +10.3% |
| 3 | 1564117ns | +1.9% | -99.8% | +5.2% | +2.4% |
| 4 | 1594546ns | -5.0% | -99.8% | -0.5% | -0.6% |
| 5 | 1592945ns | +1.0% | -99.8% | -1.0% | -0.4% |
| 6 | 1609422ns | -5.2% | -99.8% | -6.5% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | -0.016 | ok |
| abi_entry_form_leaf_null_entry | -0.439 | moderate- |
| abi_entry_form_leaf_per_w_set | 0.195 | ok |
| abi_entry_form_leaf_runtime_w | 0.081 | ok |
| abi_entry_form_leaf_scalar_anchor | -0.159 | ok |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 2/6, lost 4/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_leaf_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4961363.3ns | 1588060.6ns | 312.4% | HIGH |
| abi_entry_form_leaf_null_entry | 122233.0ns | 3110.2ns | 3930.0% | HIGH |
| abi_entry_form_leaf_per_w_set | 4949399.9ns | 1622626.1ns | 305.0% | HIGH |
| abi_entry_form_leaf_runtime_w | 4855782.1ns | 1590399.1ns | 305.3% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4901903.6ns | 1601591.2ns | 306.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1515596.7-1642880.9 ns)
  1515596.7 |########################################
  1521960.9 |########################################
  1528325.1 |
  1534689.3 |
  1541053.5 |
  1547417.7 |
  1553781.9 |
  1560146.2 |
  1566510.4 |
  1572874.6 |
  1579238.8 |
  1585603.0 |
  1591967.2 |########################################
  1598331.4 |
  1604695.6 |########################################
  1611059.8 |
  1617424.0 |########################################
  1623788.2 |
  1630152.4 |
  1636516.6 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 2965.0-3293.6 ns)
   2965.0 |########################################
   2981.4 |########################################
   2997.9 |
   3014.3 |
   3030.7 |
   3047.1 |########################################
   3063.6 |########################################
   3080.0 |
   3096.4 |
   3112.8 |
   3129.3 |
   3145.7 |
   3162.1 |
   3178.6 |
   3195.0 |
   3211.4 |
   3227.8 |
   3244.3 |
   3260.7 |########################################
   3277.1 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1504458.8-1724502.7 ns)
  1504458.8 |########################################
  1515461.0 |
  1526463.2 |
  1537465.4 |
  1548467.6 |
  1559469.8 |
  1570472.0 |########################################
  1581474.2 |########################################
  1592476.4 |
  1603478.6 |
  1614480.8 |########################################
  1625482.9 |
  1636485.1 |########################################
  1647487.3 |
  1658489.5 |
  1669491.7 |
  1680493.9 |
  1691496.1 |
  1702498.3 |
  1713500.5 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1564116.7-1606702.3 ns)
  1564116.7 |########################################
  1566246.0 |
  1568375.3 |
  1570504.5 |
  1572633.8 |
  1574763.1 |
  1576892.4 |########################################
  1579021.7 |
  1581150.9 |
  1583280.2 |
  1585409.5 |
  1587538.8 |
  1589668.1 |
  1591797.3 |########################################
  1593926.6 |########################################
  1596055.9 |
  1598185.2 |
  1600314.5 |
  1602443.7 |########################################
  1604573.0 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1533354.6-1670984.1 ns)
  1533354.6 |####################
  1540236.1 |
  1547117.6 |
  1553999.0 |
  1560880.5 |####################
  1567762.0 |
  1574643.5 |
  1581524.9 |########################################
  1588406.4 |
  1595287.9 |####################
  1602169.4 |
  1609050.9 |
  1615932.3 |
  1622813.8 |
  1629695.3 |
  1636576.8 |
  1643458.2 |
  1650339.7 |
  1657221.2 |
  1664102.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=306.1% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=4010.5% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=304.5% of algo (FFI overhead may distort results)

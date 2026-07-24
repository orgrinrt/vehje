# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_leaf_null_entry dominates: 42244% faster than the next best (abi_boundary_w_leaf_zig_runtime_w)

abi_boundary_w_leaf_null_entry (3.41 us) leads abi_boundary_w_leaf_zig_runtime_w (1.44 ms) by 42244%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_soa_runtime_w is an outlier: 428.7x slower than the field

abi_boundary_w_leaf_soa_runtime_w (1.46 ms) is 428.7x the fastest (3.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_zig_runtime_w shows alternating (throttle bounce) (autocorr -0.81)

abi_boundary_w_leaf_zig_runtime_w's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_soa_runtime_w} (42244% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_soa_runtime_w} with a 42244% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 428.7x the fastest

Fastest abi_boundary_w_leaf_null_entry (3.41 us) to slowest abi_boundary_w_leaf_soa_runtime_w (1.46 ms): 428.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 3410.9 ns median (-99.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 428.66x (fastest 3410.9 ns, slowest 1462078.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5739ns | 5688ns | 5568ns | 5653ns | 5952ns | -99.61% |
| abi_boundary_w_leaf_scalar_anchor | 1460132ns | 1460194ns | 1456971ns | 1459620ns | 1462480ns | -0.18% |
| abi_boundary_w_leaf_scalar_dispatch | 1463453ns | 1461935ns | 1457489ns | 1461360ns | 1469576ns | +0.05% |
| abi_boundary_w_leaf_scalar_per_w | 1457423ns | 1457627ns | 1455793ns | 1457162ns | 1458630ns | -0.36% |
| abi_boundary_w_leaf_scalar_runtime_w | 1462738ns | 1462163ns | 1459103ns | 1461514ns | 1466391ns | base |
| abi_boundary_w_leaf_soa_dispatch | 1457815ns | 1457440ns | 1454957ns | 1456836ns | 1460713ns | -0.34% |
| abi_boundary_w_leaf_soa_per_w | 1456612ns | 1455711ns | 1453078ns | 1455340ns | 1460286ns | -0.42% |
| abi_boundary_w_leaf_soa_runtime_w | 1464956ns | 1464870ns | 1457370ns | 1464238ns | 1469825ns | +0.15% |
| abi_boundary_w_leaf_zig_runtime_w | 1446658ns | 1447088ns | 1440945ns | 1445799ns | 1450801ns | -1.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 3454ns | 3363ns | 3588ns | -99.76% | 0.001 |
| abi_boundary_w_leaf_scalar_anchor | 1457400ns | 1454384ns | 1459608ns | -0.18% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1460526ns | 1454848ns | 1466460ns | +0.04% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1454732ns | 1453095ns | 1455915ns | -0.36% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1460015ns | 1456271ns | 1463565ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 1455173ns | 1452219ns | 1458063ns | -0.33% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 1453937ns | 1450453ns | 1457344ns | -0.42% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 1462150ns | 1454728ns | 1466887ns | +0.15% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1443794ns | 1438125ns | 1447818ns | -1.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 27133.5 | 3528.3 | 3454.5 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 46430.6 | 1455177.3 | 1457400.3 | 1 |
| abi_boundary_w_leaf_scalar_dispatch | 49343.5 | 1461728.4 | 1460526.3 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 45284.7 | 1454328.9 | 1454731.9 | 0 |
| abi_boundary_w_leaf_scalar_runtime_w | 44726.0 | 1459717.0 | 1460014.7 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 41602.5 | 1455063.4 | 1455173.1 | n/a |
| abi_boundary_w_leaf_soa_per_w | 42662.1 | 1454411.8 | 1453937.3 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 48098.5 | 1461488.2 | 1462149.5 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 189617.7 | 1443226.5 | 1443794.2 | 9 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.001 | 98.6% |
| abi_boundary_w_leaf_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5739ns | 5739ns | -99.61% |
| abi_boundary_w_leaf_scalar_anchor | 1460132ns | 1460132ns | -0.18% |
| abi_boundary_w_leaf_scalar_dispatch | 1463453ns | 1463453ns | +0.05% |
| abi_boundary_w_leaf_scalar_per_w | 1457423ns | 1457423ns | -0.36% |
| abi_boundary_w_leaf_scalar_runtime_w | 1462738ns | 1462738ns | base |
| abi_boundary_w_leaf_soa_dispatch | 1457815ns | 1457815ns | -0.34% |
| abi_boundary_w_leaf_soa_per_w | 1456612ns | 1456612ns | -0.42% |
| abi_boundary_w_leaf_soa_runtime_w | 1464956ns | 1464956ns | +0.15% |
| abi_boundary_w_leaf_zig_runtime_w | 1446658ns | 1446658ns | -1.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1459565ns | base | --- | [1456914, 1463565] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 3411ns | -1456089.4ns (-99.8%) | [-1460200, -1453391]ns | [3365, 3588] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1457508ns | no significant difference | [-7012, +1709]ns | [1455085, 1459608] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1459114ns | no significant difference | [-2304, +4471]ns | [1456004, 1466460] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1454902ns | -6186.2ns (-0.4%) | [-8663, -999]ns | [1453379, 1455915] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 1454764ns | no significant difference | [-9740, +1149]ns | [1452693, 1458063] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_soa_per_w | 1453154ns | -6053.4ns (-0.4%) | [-11104, -1075]ns | [1451314, 1457344] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 1462078ns | no significant difference | [-2628, +8839]ns | [1457483, 1466887] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1444283ns | -16811.3ns (-1.2%) | [-22754, -9096]ns | [1439281, 1447818] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1456271ns | -99.8% | +0.3% | +0.2% | +0.0% | +0.2% | +0.1% | +0.9% | -0.5% |
| 2 | 1460568ns | -99.8% | -0.3% | -0.1% | -0.5% | -0.4% | -0.3% | -0.0% | -1.5% |
| 3 | 1457557ns | -99.7% | -0.0% | -0.0% | -0.1% | -0.1% | -0.5% | -0.2% | -0.7% |
| 4 | 1463503ns | -99.8% | -0.3% | +0.5% | -0.6% | -0.6% | -0.7% | +0.0% | -1.6% |
| 5 | 1458562ns | -99.8% | -0.0% | -0.3% | -0.4% | -0.4% | -0.3% | +0.3% | -0.9% |
| 6 | 1463627ns | -99.8% | -0.6% | -0.1% | -0.6% | -0.8% | -0.8% | -0.2% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.374 | moderate- |
| abi_boundary_w_leaf_scalar_anchor | -0.239 | moderate- |
| abi_boundary_w_leaf_scalar_dispatch | -0.652 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_per_w | -0.388 | moderate- |
| abi_boundary_w_leaf_scalar_runtime_w | -0.468 | moderate- |
| abi_boundary_w_leaf_soa_dispatch | -0.013 | ok |
| abi_boundary_w_leaf_soa_per_w | 0.179 | ok |
| abi_boundary_w_leaf_soa_runtime_w | -0.121 | ok |
| abi_boundary_w_leaf_zig_runtime_w | -0.814 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 3/6, lost 1/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 1/6, lost 2/6
- **abi_boundary_w_leaf_scalar_per_w**: won 5/6, lost 0/6
- **abi_boundary_w_leaf_soa_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_leaf_soa_per_w**: won 5/6, lost 1/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 2/6, lost 2/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 119931.5ns | 3454.5ns | 3471.8% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4416363.8ns | 1457400.3ns | 303.0% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4434799.8ns | 1460526.3ns | 303.6% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4411805.7ns | 1454731.9ns | 303.3% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4428396.6ns | 1460014.7ns | 303.3% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 4408412.0ns | 1455173.1ns | 302.9% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 4406847.4ns | 1453937.3ns | 303.1% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 4432576.6ns | 1462149.5ns | 303.2% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4590717.8ns | 1443794.2ns | 318.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 3363.3-3587.5 ns)
   3363.3 |########################################
   3374.5 |
   3385.7 |
   3396.9 |
   3408.1 |
   3419.4 |
   3430.6 |
   3441.8 |
   3453.0 |#############
   3464.2 |
   3475.4 |
   3486.6 |#############
   3497.8 |
   3509.0 |
   3520.2 |
   3531.4 |
   3542.7 |
   3553.9 |
   3565.1 |
   3576.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1454383.7-1459607.9 ns)
  1454383.7 |########################################
  1454644.9 |
  1454906.1 |
  1455167.3 |
  1455428.5 |
  1455689.8 |########################################
  1455951.0 |
  1456212.2 |
  1456473.4 |
  1456734.6 |########################################
  1456995.8 |
  1457257.0 |
  1457518.2 |
  1457779.4 |
  1458040.6 |########################################
  1458301.8 |
  1458563.1 |
  1458824.3 |########################################
  1459085.5 |
  1459346.7 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1454847.5-1466460.2 ns)
  1454847.5 |########################################
  1455428.1 |
  1456008.8 |
  1456589.4 |########################################
  1457170.1 |
  1457750.7 |
  1458331.3 |########################################
  1458912.0 |
  1459492.6 |########################################
  1460073.2 |
  1460653.9 |
  1461234.5 |
  1461815.1 |
  1462395.8 |########################################
  1462976.4 |
  1463557.1 |
  1464137.7 |
  1464718.3 |
  1465299.0 |
  1465879.6 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1453095.0-1455915.2 ns)
  1453095.0 |########################################
  1453236.0 |
  1453377.0 |
  1453518.0 |
  1453659.0 |########################################
  1453800.1 |
  1453941.1 |
  1454082.1 |
  1454223.1 |
  1454364.1 |
  1454505.1 |
  1454646.1 |########################################
  1454787.1 |
  1454928.1 |
  1455069.1 |########################################
  1455210.1 |
  1455351.2 |########################################
  1455492.2 |
  1455633.2 |
  1455774.2 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1456271.2-1463565.0 ns)
  1456271.2 |########################################
  1456635.9 |
  1457000.6 |
  1457365.3 |########################################
  1457730.0 |
  1458094.6 |
  1458459.3 |########################################
  1458824.0 |
  1459188.7 |
  1459553.4 |
  1459918.1 |
  1460282.8 |########################################
  1460647.5 |
  1461012.2 |
  1461376.9 |
  1461741.6 |
  1462106.2 |
  1462470.9 |
  1462835.6 |
  1463200.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 1452219.2-1458062.7 ns)
  1452219.2 |########################################
  1452511.4 |
  1452803.6 |
  1453095.7 |########################################
  1453387.9 |
  1453680.1 |
  1453972.2 |########################################
  1454264.4 |
  1454556.6 |
  1454848.8 |
  1455140.9 |########################################
  1455433.1 |
  1455725.3 |
  1456017.5 |
  1456309.6 |
  1456601.8 |########################################
  1456894.0 |
  1457186.2 |
  1457478.3 |
  1457770.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 1450452.9-1457344.4 ns)
  1450452.9 |########################################
  1450797.5 |
  1451142.0 |
  1451486.6 |
  1451831.2 |########################################
  1452175.8 |
  1452520.3 |########################################
  1452864.9 |
  1453209.5 |
  1453554.1 |########################################
  1453898.6 |
  1454243.2 |
  1454587.8 |
  1454932.3 |
  1455276.9 |
  1455621.5 |
  1455966.1 |########################################
  1456310.6 |
  1456655.2 |
  1456999.8 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 1454727.5-1466887.3 ns)
  1454727.5 |########################################
  1455335.5 |
  1455943.5 |
  1456551.5 |
  1457159.5 |
  1457767.4 |
  1458375.4 |
  1458983.4 |
  1459591.4 |
  1460199.4 |########################################
  1460807.4 |########################################
  1461415.4 |
  1462023.4 |
  1462631.4 |########################################
  1463239.4 |
  1463847.3 |########################################
  1464455.3 |
  1465063.3 |
  1465671.3 |
  1466279.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1438125.0-1447817.9 ns)
  1438125.0 |########################################
  1438609.6 |
  1439094.3 |
  1439578.9 |
  1440063.6 |########################################
  1440548.2 |
  1441032.9 |
  1441517.5 |
  1442002.2 |
  1442486.8 |########################################
  1442971.4 |
  1443456.1 |
  1443940.7 |
  1444425.4 |
  1444910.0 |
  1445394.7 |
  1445879.3 |########################################
  1446364.0 |
  1446848.6 |########################################
  1447333.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=3507.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=318.0% of algo (FFI overhead may distort results)

# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 56104% faster than the next best (abi_zig_entry_leaf_zig_runtime_w)

abi_zig_entry_leaf_zig_null (2.55 us) leads abi_zig_entry_leaf_zig_runtime_w (1.43 ms) by 56104%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.43 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_dispatch is an outlier: 1275.6x slower than the field

abi_zig_entry_leaf_zig_tail_dispatch (3.25 ms) is 1275.6x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_leaf_zig_tail_runtime_w shows alternating (throttle bounce) (autocorr -0.53)

abi_zig_entry_leaf_zig_tail_runtime_w's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch} (56104% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch} with a 56104% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1275.6x the fastest

Fastest abi_zig_entry_leaf_zig_null (2.55 us) to slowest abi_zig_entry_leaf_zig_tail_dispatch (3.25 ms): 1275.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 2550.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1275.55x (fastest 2550.2 ns, slowest 3252916.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1437882ns | 1438470ns | 1433755ns | 1438275ns | 1439356ns | -3.45% |
| abi_zig_entry_leaf_zig_dispatch | 1457597ns | 1437003ns | 1435294ns | 1436494ns | 1500402ns | -2.13% |
| abi_zig_entry_leaf_zig_null | 4874ns | 4872ns | 4725ns | 4852ns | 4983ns | -99.67% |
| abi_zig_entry_leaf_zig_per_w_set | 1437514ns | 1437280ns | 1434591ns | 1436952ns | 1439819ns | -3.48% |
| abi_zig_entry_leaf_zig_runtime_w | 1489280ns | 1435868ns | 1433570ns | 1435311ns | 1598088ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3204320ns | 3255850ns | 3079394ns | 3197936ns | 3276359ns | +115.16% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3255327ns | 3255536ns | 3251923ns | 3254552ns | 3258192ns | +118.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1435338ns | 1431347ns | 1436746ns | -3.45% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1454641ns | 1432733ns | 1496734ns | -2.15% | 0.000 |
| abi_zig_entry_leaf_zig_null | 2544ns | 2467ns | 2586ns | -99.83% | 0.006 |
| abi_zig_entry_leaf_zig_per_w_set | 1434851ns | 1432033ns | 1437049ns | -3.48% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1486596ns | 1430846ns | 1595201ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3201410ns | 3076828ns | 3273247ns | +115.35% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3252473ns | 3249329ns | 3255263ns | +118.79% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 173129.8 | 1434397.2 | 1435338.2 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 187442.2 | 1453806.0 | 1454640.6 | 0 |
| abi_zig_entry_leaf_zig_null | 159919.0 | 2718.7 | 2544.0 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 179236.3 | 1434941.2 | 1434850.6 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 182619.6 | 1447489.0 | 1486595.8 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 198646.0 | 3211774.9 | 3201409.6 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 193353.8 | 3256079.2 | 3252472.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_null | 0.006 | 96.7% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1437882ns | 1437882ns | -3.45% |
| abi_zig_entry_leaf_zig_dispatch | 1457597ns | 1457597ns | -2.13% |
| abi_zig_entry_leaf_zig_null | 4874ns | 4874ns | -99.67% |
| abi_zig_entry_leaf_zig_per_w_set | 1437514ns | 1437514ns | -3.48% |
| abi_zig_entry_leaf_zig_runtime_w | 1489280ns | 1489280ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3204320ns | 3204320ns | +115.16% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3255327ns | 3255327ns | +118.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1433316ns | base | --- | [1431270, 1595201] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1435918ns | no significant difference | [-161850, +4647]ns | [1433351, 1436746] | no | 1.0000 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1434332ns | no significant difference | [-104275, +8336]ns | [1432857, 1496734] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_null | 2550ns | -1430819.8ns (-99.8%) | [-1592638, -1428697]ns | [2496, 2586] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1434650ns | no significant difference | [-161431, +5779]ns | [1432853, 1437049] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3252916ns | +1733479.2ns (+120.9%) | [+1585703, +1825259]ns | [3078066, 3273247] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3252657ns | +1819178.8ns (+126.9%) | [+1656489, +1821964]ns | [3249499, 3255263] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1439342ns | -0.6% | -0.5% | -99.8% | -0.4% | +113.8% | +125.8% |
| 2 | 1432906ns | +0.3% | +0.1% | -99.8% | +0.2% | +114.9% | +127.3% |
| 3 | 1430846ns | +0.3% | +0.3% | -99.8% | +0.5% | +127.2% | +127.2% |
| 4 | 1751060ns | -18.0% | -11.5% | -99.9% | -18.1% | +87.6% | +85.8% |
| 5 | 1433726ns | +0.2% | -0.1% | -99.8% | -0.1% | +127.5% | +127.0% |
| 6 | 1431695ns | +0.3% | +0.9% | -99.8% | +0.3% | +127.3% | +127.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.166 | ok |
| abi_zig_entry_leaf_zig_dispatch | -0.261 | moderate- |
| abi_zig_entry_leaf_zig_null | -0.312 | moderate- |
| abi_zig_entry_leaf_zig_per_w_set | -0.179 | ok |
| abi_zig_entry_leaf_zig_runtime_w | -0.242 | moderate- |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.464 | moderate+ |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 2/6, lost 4/6
- **abi_zig_entry_leaf_zig_dispatch**: won 2/6, lost 2/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 3/6, lost 3/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4541475.0ns | 1435338.2ns | 316.4% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4668610.8ns | 1454640.6ns | 320.9% | HIGH |
| abi_zig_entry_leaf_zig_null | 308285.5ns | 2544.0ns | 12118.1% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4553371.8ns | 1434850.6ns | 317.3% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4603547.2ns | 1486595.8ns | 309.7% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9899901.2ns | 3201409.6ns | 309.2% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10029560.4ns | 3252472.9ns | 308.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1431347.1-1436746.2 ns)
  1431347.1 |########################################
  1431617.1 |
  1431887.0 |
  1432157.0 |
  1432426.9 |
  1432696.9 |
  1432966.8 |
  1433236.8 |
  1433506.8 |
  1433776.7 |
  1434046.7 |
  1434316.6 |
  1434586.6 |
  1434856.5 |
  1435126.5 |########################################
  1435396.5 |
  1435666.4 |########################################
  1435936.4 |########################################
  1436206.3 |
  1436476.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1432733.3-1496733.6 ns)
  1432733.3 |########################################
  1435933.3 |
  1439133.3 |
  1442333.3 |##########
  1445533.4 |
  1448733.4 |
  1451933.4 |
  1455133.4 |
  1458333.4 |
  1461533.4 |
  1464733.4 |
  1467933.4 |
  1471133.4 |
  1474333.5 |
  1477533.5 |
  1480733.5 |
  1483933.5 |
  1487133.5 |
  1490333.5 |
  1493533.5 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 2466.7-2585.6 ns)
   2466.7 |########################################
   2472.6 |
   2478.6 |
   2484.5 |
   2490.5 |
   2496.4 |
   2502.4 |
   2508.3 |
   2514.3 |
   2520.2 |########################################
   2526.2 |
   2532.1 |
   2538.0 |########################################
   2544.0 |
   2549.9 |
   2555.9 |########################################
   2561.8 |
   2567.8 |
   2573.7 |
   2579.7 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1432033.3-1437048.8 ns)
  1432033.3 |########################################
  1432284.1 |
  1432534.8 |
  1432785.6 |
  1433036.4 |
  1433287.2 |
  1433537.9 |########################################
  1433788.7 |########################################
  1434039.5 |
  1434290.3 |
  1434541.0 |
  1434791.8 |
  1435042.6 |
  1435293.3 |########################################
  1435544.1 |
  1435794.9 |
  1436045.7 |
  1436296.4 |
  1436547.2 |########################################
  1436798.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1430845.8-1595201.2 ns)
  1430845.8 |########################################
  1439063.6 |##########
  1447281.3 |
  1455499.1 |
  1463716.9 |
  1471934.7 |
  1480152.4 |
  1488370.2 |
  1496588.0 |
  1504805.8 |
  1513023.5 |
  1521241.3 |
  1529459.1 |
  1537676.8 |
  1545894.6 |
  1554112.4 |
  1562330.2 |
  1570547.9 |
  1578765.7 |
  1586983.5 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3076828.3-3273247.1 ns)
  3076828.3 |########################################
  3086649.2 |
  3096470.2 |
  3106291.1 |
  3116112.1 |
  3125933.0 |
  3135753.9 |
  3145574.9 |
  3155395.8 |
  3165216.8 |
  3175037.7 |
  3184858.6 |
  3194679.6 |
  3204500.5 |
  3214321.5 |
  3224142.4 |
  3233963.3 |
  3243784.3 |####################
  3253605.2 |########################################
  3263426.2 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3249329.2-3255262.7 ns)
  3249329.2 |########################################
  3249625.9 |########################################
  3249922.6 |
  3250219.2 |
  3250515.9 |
  3250812.6 |
  3251109.2 |########################################
  3251405.9 |
  3251702.6 |
  3251999.3 |
  3252296.0 |
  3252592.6 |
  3252889.3 |
  3253186.0 |
  3253482.7 |
  3253779.3 |########################################
  3254076.0 |########################################
  3254372.7 |
  3254669.4 |
  3254966.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=316.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=316.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=11912.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=317.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=308.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=308.5% of algo (FFI overhead may distort results)

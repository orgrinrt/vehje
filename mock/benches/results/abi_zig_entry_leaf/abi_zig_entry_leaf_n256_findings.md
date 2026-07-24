# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 45595% faster than the next best (abi_zig_entry_leaf_zig_per_w_set)

abi_zig_entry_leaf_zig_null (3.15 us) leads abi_zig_entry_leaf_zig_per_w_set (1.44 ms) by 45595%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.44 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_dispatch is an outlier: 1044.6x slower than the field

abi_zig_entry_leaf_zig_tail_dispatch (3.29 ms) is 1044.6x the fastest (3.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch} (45595% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch} with a 45595% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1044.6x the fastest

Fastest abi_zig_entry_leaf_zig_null (3.15 us) to slowest abi_zig_entry_leaf_zig_tail_dispatch (3.29 ms): 1044.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 3145.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1044.62x (fastest 3145.6 ns, slowest 3285949.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1535882ns | 1450013ns | 1437095ns | 1447647ns | 1717628ns | +5.41% |
| abi_zig_entry_leaf_zig_dispatch | 1467928ns | 1449503ns | 1436694ns | 1445912ns | 1516569ns | +0.75% |
| abi_zig_entry_leaf_zig_null | 5522ns | 5446ns | 5388ns | 5434ns | 5721ns | -99.62% |
| abi_zig_entry_leaf_zig_per_w_set | 1476922ns | 1440185ns | 1435785ns | 1439247ns | 1554003ns | +1.36% |
| abi_zig_entry_leaf_zig_runtime_w | 1457055ns | 1443012ns | 1442174ns | 1442750ns | 1485954ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3359604ns | 3289437ns | 3262563ns | 3284970ns | 3520075ns | +130.57% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3303281ns | 3279788ns | 3258535ns | 3276873ns | 3365266ns | +126.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1532359ns | 1434430ns | 1713141ns | +5.39% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1464787ns | 1434133ns | 1512959ns | +0.75% | 0.000 |
| abi_zig_entry_leaf_zig_null | 3157ns | 3086ns | 3230ns | -99.78% | 0.081 |
| abi_zig_entry_leaf_zig_per_w_set | 1473851ns | 1433157ns | 1550239ns | +1.37% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1453944ns | 1439268ns | 1482327ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3356108ns | 3259646ns | 3516192ns | +130.83% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3299843ns | 3255394ns | 3361688ns | +126.96% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 230927.8 | 1531943.2 | 1532359.2 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 222271.5 | 1464412.8 | 1464787.0 | n/a |
| abi_zig_entry_leaf_zig_null | 162716.8 | 3237.6 | 3156.8 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 209221.2 | 1471836.5 | 1473851.2 | 0 |
| abi_zig_entry_leaf_zig_runtime_w | 210917.7 | 1454414.0 | 1453944.0 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 256116.7 | 3404517.5 | 3356107.8 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 232822.1 | 3311198.1 | 3299842.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_null | 0.081 | 98.1% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1535882ns | 1535882ns | +5.41% |
| abi_zig_entry_leaf_zig_dispatch | 1467928ns | 1467928ns | +0.75% |
| abi_zig_entry_leaf_zig_null | 5522ns | 5522ns | -99.62% |
| abi_zig_entry_leaf_zig_per_w_set | 1476922ns | 1476922ns | +1.36% |
| abi_zig_entry_leaf_zig_runtime_w | 1457055ns | 1457055ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3359604ns | 3359604ns | +130.57% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3303281ns | 3303281ns | +126.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1440155ns | base | --- | [1439350, 1482327] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1446740ns | no significant difference | [-2327, +233646]ns | [1437196, 1713141] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1446315ns | no significant difference | [-4263, +34004]ns | [1435088, 1512959] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_null | 3146ns | -1437015.0ns (-99.8%) | [-1479142, -1436205]ns | [3095, 3230] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1437381ns | no significant difference | [-5417, +68695]ns | [1433933, 1550239] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3285949ns | +1843053.9ns (+128.0%) | [+1826201, +2037236]ns | [3266182, 3516192] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3276220ns | +1836156.0ns (+127.5%) | [+1782839, +1918701]ns | [3261621, 3361688] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1518296ns | +30.0% | +1.0% | -99.8% | +9.2% | +123.2% | +115.2% |
| 2 | 1440695ns | +0.9% | +0.1% | -99.8% | -0.2% | +127.2% | +127.6% |
| 3 | 1446358ns | +0.1% | +0.3% | -99.8% | -0.3% | +127.5% | +135.4% |
| 4 | 1439268ns | +0.4% | -0.4% | -99.8% | -0.4% | +126.5% | +126.2% |
| 5 | 1439615ns | -0.4% | +3.7% | -99.8% | -0.2% | +153.1% | +130.5% |
| 6 | 1439432ns | +0.0% | -0.2% | -99.8% | -0.3% | +127.9% | +127.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.014 | ok |
| abi_zig_entry_leaf_zig_dispatch | -0.301 | moderate- |
| abi_zig_entry_leaf_zig_null | 0.040 | ok |
| abi_zig_entry_leaf_zig_per_w_set | -0.030 | ok |
| abi_zig_entry_leaf_zig_runtime_w | -0.044 | ok |
| abi_zig_entry_leaf_zig_tail_dispatch | -0.367 | moderate- |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.482 | moderate- |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 1/6, lost 4/6
- **abi_zig_entry_leaf_zig_dispatch**: won 2/6, lost 4/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 5/6, lost 1/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4913208.6ns | 1532359.2ns | 320.6% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4694376.4ns | 1464787.0ns | 320.5% | HIGH |
| abi_zig_entry_leaf_zig_null | 314933.5ns | 3156.8ns | 9976.4% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4698423.2ns | 1473851.2ns | 318.8% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4646296.2ns | 1453944.0ns | 319.6% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 10477769.9ns | 3356107.8ns | 312.2% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10231515.8ns | 3299842.8ns | 310.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1434430.0-1713141.2 ns)
  1434430.0 |########################################
  1448365.6 |##########
  1462301.1 |
  1476236.7 |
  1490172.2 |
  1504107.8 |
  1518043.4 |
  1531978.9 |
  1545914.5 |
  1559850.0 |
  1573785.6 |
  1587721.2 |
  1601656.7 |
  1615592.3 |
  1629527.8 |
  1643463.4 |
  1657399.0 |
  1671334.5 |
  1685270.1 |
  1699205.6 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1434133.3-1512959.0 ns)
  1434133.3 |########################################
  1438074.6 |
  1442015.9 |####################
  1445957.2 |
  1449898.4 |####################
  1453839.7 |
  1457781.0 |
  1461722.3 |
  1465663.6 |
  1469604.9 |
  1473546.1 |
  1477487.4 |
  1481428.7 |
  1485370.0 |
  1489311.3 |####################
  1493252.6 |
  1497193.9 |
  1501135.1 |
  1505076.4 |
  1509017.7 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 3085.8-3229.8 ns)
   3085.8 |####################
   3093.0 |
   3100.2 |####################
   3107.4 |
   3114.6 |####################
   3121.8 |
   3129.0 |
   3136.2 |
   3143.4 |
   3150.6 |
   3157.8 |
   3165.0 |
   3172.2 |########################################
   3179.4 |
   3186.6 |
   3193.8 |
   3201.0 |
   3208.2 |
   3215.4 |
   3222.6 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1433156.7-1550238.9 ns)
  1433156.7 |########################################
  1439010.8 |##########
  1444864.9 |
  1450719.0 |
  1456573.1 |
  1462427.3 |
  1468281.4 |
  1474135.5 |
  1479989.6 |
  1485843.7 |
  1491697.8 |
  1497551.9 |
  1503406.1 |
  1509260.2 |
  1515114.3 |
  1520968.4 |
  1526822.5 |
  1532676.6 |
  1538530.7 |
  1544384.8 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1439268.3-1482326.9 ns)
  1439268.3 |########################################
  1441421.2 |
  1443574.2 |
  1445727.1 |##########
  1447880.0 |
  1450032.9 |
  1452185.9 |
  1454338.8 |
  1456491.7 |
  1458644.6 |
  1460797.6 |
  1462950.5 |
  1465103.4 |
  1467256.4 |
  1469409.3 |
  1471562.2 |
  1473715.1 |
  1475868.1 |
  1478021.0 |
  1480173.9 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3259645.8-3516191.6 ns)
  3259645.8 |####################
  3272473.1 |########################################
  3285300.4 |####################
  3298127.7 |
  3310955.0 |
  3323782.3 |
  3336609.6 |
  3349436.8 |
  3362264.1 |
  3375091.4 |
  3387918.7 |####################
  3400746.0 |
  3413573.3 |
  3426400.6 |
  3439227.9 |
  3452055.2 |
  3464882.5 |
  3477709.8 |
  3490537.1 |
  3503364.4 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3255393.7-3361687.7 ns)
  3255393.7 |########################################
  3260708.4 |
  3266023.1 |########################################
  3271337.8 |########################################
  3276652.5 |########################################
  3281967.2 |
  3287281.9 |
  3292596.6 |
  3297911.3 |
  3303226.0 |
  3308540.7 |
  3313855.4 |########################################
  3319170.1 |
  3324484.8 |
  3329799.5 |
  3335114.2 |
  3340428.9 |
  3345743.6 |
  3351058.3 |
  3356373.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=320.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=319.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=9733.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=317.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=318.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=310.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=309.8% of algo (FFI overhead may distort results)

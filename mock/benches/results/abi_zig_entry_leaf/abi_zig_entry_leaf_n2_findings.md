# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 41582% faster than the next best (abi_zig_entry_leaf_zig_runtime_w)

abi_zig_entry_leaf_zig_null (3.44 us) leads abi_zig_entry_leaf_zig_runtime_w (1.44 ms) by 41582%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.43 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 945.5x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.26 ms) is 945.5x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_leaf_zig_tail_runtime_w shows alternating (throttle bounce) (autocorr -0.56)

abi_zig_entry_leaf_zig_tail_runtime_w's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (41582% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 41582% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 945.5x the fastest

Fastest abi_zig_entry_leaf_zig_null (3.44 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.26 ms): 945.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 3443.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 945.49x (fastest 3443.6 ns, slowest 3255827.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1483245ns | 1440740ns | 1436872ns | 1439573ns | 1571940ns | +3.11% |
| abi_zig_entry_leaf_zig_dispatch | 1448880ns | 1439336ns | 1437425ns | 1439019ns | 1469399ns | +0.73% |
| abi_zig_entry_leaf_zig_null | 5777ns | 5775ns | 5700ns | 5758ns | 5844ns | -99.60% |
| abi_zig_entry_leaf_zig_per_w_set | 1442698ns | 1440798ns | 1434893ns | 1440247ns | 1450278ns | +0.30% |
| abi_zig_entry_leaf_zig_runtime_w | 1438440ns | 1437977ns | 1435840ns | 1437518ns | 1441123ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3090555ns | 3085017ns | 3081516ns | 3084576ns | 3104043ns | +114.85% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3261384ns | 3258745ns | 3255219ns | 3258263ns | 3269148ns | +126.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1480481ns | 1434328ns | 1568769ns | +3.11% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1446274ns | 1434922ns | 1466785ns | +0.73% | 0.000 |
| abi_zig_entry_leaf_zig_null | 3450ns | 3415ns | 3478ns | -99.76% | 0.001 |
| abi_zig_entry_leaf_zig_per_w_set | 1439978ns | 1432297ns | 1447464ns | +0.29% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1435800ns | 1433235ns | 1438394ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3087673ns | 3078798ns | 3101055ns | +115.05% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3258487ns | 3252452ns | 3266226ns | +126.95% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 194977.9 | 1561251.9 | 1480481.2 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 176230.1 | 1517467.8 | 1446273.8 | n/a |
| abi_zig_entry_leaf_zig_null | 157425.9 | 3716.9 | 3449.7 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 179551.5 | 1439010.8 | 1439978.3 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 174256.1 | 1436688.9 | 1435800.5 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 192218.1 | 3088374.8 | 3087673.2 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 193156.3 | 3261100.3 | 3258487.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_null | 0.001 | 99.2% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1483245ns | 1483245ns | +3.11% |
| abi_zig_entry_leaf_zig_dispatch | 1448880ns | 1448880ns | +0.73% |
| abi_zig_entry_leaf_zig_null | 5777ns | 5777ns | -99.60% |
| abi_zig_entry_leaf_zig_per_w_set | 1442698ns | 1442698ns | +0.30% |
| abi_zig_entry_leaf_zig_runtime_w | 1438440ns | 1438440ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3090555ns | 3090555ns | +114.85% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3261384ns | 3261384ns | +126.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1435353ns | base | --- | [1433654, 1438394] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1438107ns | no significant difference | [-966, +133656]ns | [1434568, 1568769] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1436702ns | no significant difference | [-2729, +31253]ns | [1435334, 1466785] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_null | 3444ns | -1431909.6ns (-99.8%) | [-1434967, -1430176]ns | [3427, 3478] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1438100ns | no significant difference | [-369, +10528]ns | [1434371, 1447464] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3082199ns | +1647532.0ns (+114.8%) | [+1643208, +1664878]ns | [3079766, 3101055] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3255828ns | +1820872.9ns (+126.9%) | [+1816315, +1830873]ns | [3253408, 3266226] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1439797ns | -0.0% | -0.3% | -99.8% | +0.5% | +114.4% | +125.9% |
| 2 | 1436329ns | -0.1% | -0.0% | -99.8% | +0.2% | +114.4% | +127.3% |
| 3 | 1433235ns | +0.7% | +0.3% | -99.8% | -0.1% | +114.9% | +127.1% |
| 4 | 1434377ns | -0.0% | +0.1% | -99.8% | +0.1% | +115.0% | +127.8% |
| 5 | 1436991ns | +17.9% | +4.0% | -99.8% | +0.0% | +114.4% | +126.7% |
| 6 | 1434074ns | +0.2% | +0.4% | -99.8% | +1.0% | +117.2% | +127.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.253 | moderate- |
| abi_zig_entry_leaf_zig_dispatch | -0.177 | ok |
| abi_zig_entry_leaf_zig_null | -0.139 | ok |
| abi_zig_entry_leaf_zig_per_w_set | 0.080 | ok |
| abi_zig_entry_leaf_zig_runtime_w | 0.023 | ok |
| abi_zig_entry_leaf_zig_tail_dispatch | -0.065 | ok |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.559 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 1/6, lost 3/6
- **abi_zig_entry_leaf_zig_dispatch**: won 1/6, lost 4/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 0/6, lost 4/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4809921.2ns | 1480481.2ns | 324.9% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4679111.5ns | 1446273.8ns | 323.5% | HIGH |
| abi_zig_entry_leaf_zig_null | 308021.4ns | 3449.7ns | 8929.0% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4565386.2ns | 1439978.3ns | 317.0% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4549359.3ns | 1435800.5ns | 316.9% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9528200.8ns | 3087673.2ns | 308.6% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10042831.3ns | 3258487.4ns | 308.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1434327.5-1568769.1 ns)
  1434327.5 |########################################
  1441049.6 |##########
  1447771.7 |
  1454493.7 |
  1461215.8 |
  1467937.9 |
  1474660.0 |
  1481382.1 |
  1488104.2 |
  1494826.2 |
  1501548.3 |
  1508270.4 |
  1514992.5 |
  1521714.6 |
  1528436.7 |
  1535158.7 |
  1541880.8 |
  1548602.9 |
  1555325.0 |
  1562047.1 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1434922.1-1466785.4 ns)
  1434922.1 |########################################
  1436515.3 |#############
  1438108.4 |
  1439701.6 |#############
  1441294.8 |
  1442887.9 |
  1444481.1 |
  1446074.3 |
  1447667.4 |
  1449260.6 |
  1450853.8 |
  1452446.9 |
  1454040.1 |
  1455633.2 |
  1457226.4 |
  1458819.6 |
  1460412.7 |
  1462005.9 |
  1463599.1 |
  1465192.2 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 3415.4-3478.1 ns)
   3415.4 |####################
   3418.5 |
   3421.7 |
   3424.8 |
   3427.9 |
   3431.1 |
   3434.2 |
   3437.4 |########################################
   3440.5 |
   3443.6 |####################
   3446.8 |
   3449.9 |
   3453.0 |
   3456.2 |
   3459.3 |
   3462.5 |####################
   3465.6 |
   3468.7 |
   3471.9 |
   3475.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1432297.1-1447464.0 ns)
  1432297.1 |########################################
  1433055.4 |
  1433813.8 |
  1434572.1 |
  1435330.5 |
  1436088.8 |########################################
  1436847.2 |########################################
  1437605.5 |
  1438363.9 |########################################
  1439122.2 |
  1439880.6 |
  1440638.9 |
  1441397.2 |
  1442155.6 |
  1442913.9 |
  1443672.3 |
  1444430.6 |
  1445189.0 |
  1445947.3 |
  1446705.7 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1433234.6-1438394.0 ns)
  1433234.6 |########################################
  1433492.6 |
  1433750.5 |
  1434008.5 |########################################
  1434266.5 |########################################
  1434524.4 |
  1434782.4 |
  1435040.4 |
  1435298.3 |
  1435556.3 |
  1435814.3 |
  1436072.2 |########################################
  1436330.2 |
  1436588.2 |
  1436846.1 |########################################
  1437104.1 |
  1437362.1 |
  1437620.0 |
  1437878.0 |
  1438136.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3078797.5-3101054.8 ns)
  3078797.5 |####################
  3079910.4 |########################################
  3081023.2 |
  3082136.1 |
  3083249.0 |####################
  3084361.8 |
  3085474.7 |
  3086587.6 |####################
  3087700.4 |
  3088813.3 |
  3089926.1 |
  3091039.0 |
  3092151.9 |
  3093264.7 |
  3094377.6 |
  3095490.5 |
  3096603.3 |
  3097716.2 |
  3098829.1 |
  3099941.9 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3252452.5-3266226.0 ns)
  3252452.5 |########################################
  3253141.2 |
  3253829.9 |########################################
  3254518.5 |########################################
  3255207.2 |
  3255895.9 |
  3256584.6 |########################################
  3257273.2 |
  3257961.9 |
  3258650.6 |
  3259339.3 |
  3260028.0 |
  3260716.6 |
  3261405.3 |
  3262094.0 |
  3262782.7 |
  3263471.3 |
  3264160.0 |########################################
  3264848.7 |
  3265537.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=316.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=317.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=8924.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=317.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=308.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=308.4% of algo (FFI overhead may distort results)

# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 36746% faster than the next best (abi_zig_entry_leaf_zig_per_w_set)

abi_zig_entry_leaf_zig_null (3.90 us) leads abi_zig_entry_leaf_zig_per_w_set (1.44 ms) by 36746%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.44 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 836.7x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.26 ms) is 836.7x the fastest (3.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (36746% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 36746% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 836.7x the fastest

Fastest abi_zig_entry_leaf_zig_null (3.90 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.26 ms): 836.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_zig_entry_leaf_zig_dispatch's edge over baseline is significant but tiny (-12 ns, 0.00%)

abi_zig_entry_leaf_zig_dispatch differs from baseline abi_zig_entry_leaf_zig_runtime_w by -12 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 3896.9 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 836.71x (fastest 3896.9 ns, slowest 3260558.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1444128ns | 1443421ns | 1435459ns | 1440897ns | 1453308ns | -0.00% |
| abi_zig_entry_leaf_zig_dispatch | 1452277ns | 1442684ns | 1432789ns | 1439970ns | 1480482ns | +0.56% |
| abi_zig_entry_leaf_zig_null | 6190ns | 6184ns | 6103ns | 6168ns | 6268ns | -99.57% |
| abi_zig_entry_leaf_zig_per_w_set | 1443880ns | 1438476ns | 1432035ns | 1436453ns | 1460945ns | -0.02% |
| abi_zig_entry_leaf_zig_runtime_w | 1444146ns | 1444947ns | 1434680ns | 1442276ns | 1451684ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3114996ns | 3107619ns | 3077002ns | 3099214ns | 3157666ns | +115.70% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3270357ns | 3263446ns | 3252397ns | 3262252ns | 3291495ns | +126.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1441583ns | 1432835ns | 1450810ns | +0.00% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1449622ns | 1430166ns | 1477573ns | +0.56% | 0.000 |
| abi_zig_entry_leaf_zig_null | 3906ns | 3859ns | 3962ns | -99.73% | 0.002 |
| abi_zig_entry_leaf_zig_per_w_set | 1441287ns | 1429601ns | 1458246ns | -0.02% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1441536ns | 1432173ns | 1449121ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3111986ns | 3074072ns | 3154320ns | +115.88% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3267488ns | 3249615ns | 3288550ns | +126.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 176413.9 | 1439893.7 | 1441583.0 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 182132.8 | 1446714.3 | 1449621.7 | n/a |
| abi_zig_entry_leaf_zig_null | 155125.1 | 4119.1 | 3906.3 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 177264.8 | 1441338.9 | 1441286.8 | 14 |
| abi_zig_entry_leaf_zig_runtime_w | 173808.3 | 1440779.7 | 1441535.6 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 208213.7 | 3116340.0 | 3111985.5 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 192553.0 | 3268683.1 | 3267487.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_null | 0.002 | 99.0% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1444128ns | 1444128ns | -0.00% |
| abi_zig_entry_leaf_zig_dispatch | 1452277ns | 1452277ns | +0.56% |
| abi_zig_entry_leaf_zig_null | 6190ns | 6190ns | -99.57% |
| abi_zig_entry_leaf_zig_per_w_set | 1443880ns | 1443880ns | -0.02% |
| abi_zig_entry_leaf_zig_runtime_w | 1444146ns | 1444146ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3114996ns | 3114996ns | +115.70% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3270357ns | 3270357ns | +126.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1442272ns | base | --- | [1433214, 1449121] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1440832ns | no significant difference | [-3869, +3484]ns | [1433107, 1450810] | no | 1.0000 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1440175ns | no significant difference | [-9236, +33506]ns | [1431116, 1477573] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_null | 3897ns | -1438309.4ns (-99.7%) | [-1445235, -1429343]ns | [3859, 3962] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1435865ns | no significant difference | [-8307, +9125]ns | [1429750, 1458246] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3104788ns | +1665640.4ns (+115.5%) | [+1635456, +1710253]ns | [3076848, 3154320] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3260559ns | +1825790.0ns (+126.6%) | [+1811433, +1840633]ns | [3253355, 3288550] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1449712ns | -0.5% | +2.6% | -99.7% | +0.8% | +118.7% | +127.1% |
| 2 | 1446121ns | +0.2% | +0.1% | -99.7% | -0.6% | +114.9% | +127.2% |
| 3 | 1448530ns | +0.3% | -1.1% | -99.7% | +0.4% | +112.6% | +124.3% |
| 4 | 1432173ns | +0.0% | -0.1% | -99.7% | +0.1% | +116.6% | +127.4% |
| 5 | 1434255ns | -0.1% | -0.1% | -99.7% | -0.3% | +114.3% | +127.4% |
| 6 | 1438422ns | +0.0% | +2.0% | -99.7% | -0.6% | +118.2% | +126.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.269 | moderate+ |
| abi_zig_entry_leaf_zig_dispatch | 0.120 | ok |
| abi_zig_entry_leaf_zig_null | -0.067 | ok |
| abi_zig_entry_leaf_zig_per_w_set | 0.010 | ok |
| abi_zig_entry_leaf_zig_runtime_w | 0.331 | moderate+ |
| abi_zig_entry_leaf_zig_tail_dispatch | -0.068 | ok |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.292 | moderate+ |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 1/6, lost 2/6
- **abi_zig_entry_leaf_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 3/6, lost 3/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4566666.9ns | 1441583.0ns | 316.8% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4645387.8ns | 1449621.7ns | 320.5% | HIGH |
| abi_zig_entry_leaf_zig_null | 305554.9ns | 3906.3ns | 7822.2% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4567305.3ns | 1441286.8ns | 316.9% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4563414.0ns | 1441535.6ns | 316.6% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9677271.4ns | 3111985.5ns | 311.0% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10069409.2ns | 3267487.7ns | 308.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1432835.4-1450810.0 ns)
  1432835.4 |########################################
  1433734.1 |
  1434632.9 |
  1435531.6 |
  1436430.3 |
  1437329.0 |
  1438227.8 |####################
  1439126.5 |
  1440025.2 |
  1440924.0 |
  1441822.7 |
  1442721.4 |####################
  1443620.2 |
  1444518.9 |
  1445417.6 |
  1446316.4 |
  1447215.1 |
  1448113.8 |####################
  1449012.5 |
  1449911.3 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1430166.2-1477573.4 ns)
  1430166.2 |########################################
  1432536.6 |
  1434906.9 |
  1437277.3 |
  1439647.6 |
  1442018.0 |
  1444388.3 |
  1446758.7 |#############
  1449129.1 |
  1451499.4 |
  1453869.8 |
  1456240.1 |
  1458610.5 |
  1460980.8 |
  1463351.2 |
  1465721.6 |#############
  1468091.9 |
  1470462.3 |
  1472832.6 |
  1475203.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 3859.2-3962.5 ns)
   3859.2 |########################################
   3864.4 |
   3869.5 |
   3874.7 |
   3879.9 |####################
   3885.0 |
   3890.2 |
   3895.4 |
   3900.5 |
   3905.7 |
   3910.8 |####################
   3916.0 |
   3921.2 |
   3926.3 |####################
   3931.5 |
   3936.7 |
   3941.8 |
   3947.0 |
   3952.2 |
   3957.3 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1429601.2-1458246.0 ns)
  1429601.2 |########################################
  1431033.4 |
  1432465.7 |####################
  1433897.9 |
  1435330.2 |
  1436762.4 |####################
  1438194.6 |
  1439626.9 |
  1441059.1 |
  1442491.4 |
  1443923.6 |
  1445355.8 |
  1446788.1 |
  1448220.3 |
  1449652.6 |
  1451084.8 |
  1452517.0 |
  1453949.3 |####################
  1455381.5 |
  1456813.8 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1432173.3-1449121.0 ns)
  1432173.3 |########################################
  1433020.7 |
  1433868.1 |########################################
  1434715.5 |
  1435562.9 |
  1436410.2 |
  1437257.6 |
  1438105.0 |########################################
  1438952.4 |
  1439799.8 |
  1440647.2 |
  1441494.6 |
  1442341.9 |
  1443189.3 |
  1444036.7 |
  1444884.1 |
  1445731.5 |########################################
  1446578.9 |
  1447426.3 |
  1448273.7 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3074072.5-3154320.5 ns)
  3074072.5 |########################################
  3078084.9 |########################################
  3082097.3 |
  3086109.7 |
  3090122.1 |
  3094134.5 |
  3098146.9 |
  3102159.3 |########################################
  3106171.7 |########################################
  3110184.1 |
  3114196.5 |
  3118208.9 |
  3122221.3 |
  3126233.7 |
  3130246.1 |
  3134258.5 |########################################
  3138270.9 |
  3142283.3 |
  3146295.7 |
  3150308.1 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3249615.0-3288549.8 ns)
  3249615.0 |####################
  3251561.7 |
  3253508.5 |
  3255455.2 |####################
  3257402.0 |
  3259348.7 |########################################
  3261295.4 |
  3263242.2 |
  3265188.9 |
  3267135.7 |
  3269082.4 |
  3271029.1 |
  3272975.9 |
  3274922.6 |
  3276869.4 |
  3278816.1 |
  3280762.8 |
  3282709.6 |
  3284656.3 |####################
  3286603.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=317.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=316.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=7838.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=316.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=309.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=308.3% of algo (FFI overhead may distort results)

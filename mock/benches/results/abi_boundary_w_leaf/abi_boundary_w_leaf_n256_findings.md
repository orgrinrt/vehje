# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_leaf_null_entry dominates: 24810% faster than the next best (abi_boundary_w_leaf_soa_dispatch)

abi_boundary_w_leaf_null_entry (3.27 us) leads abi_boundary_w_leaf_soa_dispatch (813.53 us) by 24810%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_dispatch is an outlier: 444.2x slower than the field

abi_boundary_w_leaf_scalar_dispatch (1.45 ms) is 444.2x the fastest (3.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_scalar_dispatch shows alternating (throttle bounce) (autocorr -0.52)

abi_boundary_w_leaf_scalar_dispatch's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_dispatch} (24810% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_dispatch} with a 24810% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 444.2x the fastest

Fastest abi_boundary_w_leaf_null_entry (3.27 us) to slowest abi_boundary_w_leaf_scalar_dispatch (1.45 ms): 444.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 3265.8 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 444.16x (fastest 3265.8 ns, slowest 1450545.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5501ns | 5548ns | 5290ns | 5466ns | 5660ns | -99.62% |
| abi_boundary_w_leaf_scalar_anchor | 1451397ns | 1450943ns | 1449190ns | 1450624ns | 1453660ns | -0.16% |
| abi_boundary_w_leaf_scalar_dispatch | 1454964ns | 1453032ns | 1449658ns | 1452577ns | 1461198ns | +0.09% |
| abi_boundary_w_leaf_scalar_per_w | 1451450ns | 1450948ns | 1448841ns | 1450391ns | 1454343ns | -0.15% |
| abi_boundary_w_leaf_scalar_runtime_w | 1453696ns | 1452145ns | 1447769ns | 1451278ns | 1460287ns | base |
| abi_boundary_w_leaf_soa_dispatch | 815528ns | 816052ns | 810631ns | 815288ns | 818336ns | -43.90% |
| abi_boundary_w_leaf_soa_per_w | 819902ns | 819377ns | 814260ns | 818170ns | 825322ns | -43.60% |
| abi_boundary_w_leaf_soa_runtime_w | 816547ns | 817000ns | 811490ns | 816366ns | 819346ns | -43.83% |
| abi_boundary_w_leaf_zig_runtime_w | 1434825ns | 1435825ns | 1431793ns | 1434712ns | 1436511ns | -1.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 3229ns | 3085ns | 3332ns | -99.78% | 0.079 |
| abi_boundary_w_leaf_scalar_anchor | 1448962ns | 1446812ns | 1451170ns | -0.16% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1452462ns | 1447212ns | 1458552ns | +0.09% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1449010ns | 1446398ns | 1451874ns | -0.15% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1451228ns | 1445368ns | 1457712ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 813130ns | 808323ns | 815976ns | -43.97% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 817526ns | 811882ns | 822866ns | -43.67% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 814211ns | 809200ns | 816983ns | -43.90% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1432287ns | 1429381ns | 1433865ns | -1.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 26878.3 | 3223.7 | 3229.5 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 35343.8 | 1448420.2 | 1448961.7 | n/a |
| abi_boundary_w_leaf_scalar_dispatch | 35392.8 | 1452654.2 | 1452462.1 | 1 |
| abi_boundary_w_leaf_scalar_per_w | 34888.7 | 1448419.8 | 1449009.9 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 36168.0 | 1450926.7 | 1451228.3 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 30837.3 | 812893.8 | 813130.0 | n/a |
| abi_boundary_w_leaf_soa_per_w | 32583.2 | 817304.0 | 817525.6 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 31252.9 | 814640.9 | 814210.6 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 172071.0 | 1432985.5 | 1432286.9 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.078 | 94.5% |
| abi_boundary_w_leaf_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_dispatch | 0.000 | 0.4% |
| abi_boundary_w_leaf_soa_per_w | 0.000 | 0.4% |
| abi_boundary_w_leaf_soa_runtime_w | 0.000 | 0.4% |
| abi_boundary_w_leaf_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5501ns | 5501ns | -99.62% |
| abi_boundary_w_leaf_scalar_anchor | 1451397ns | 1451397ns | -0.16% |
| abi_boundary_w_leaf_scalar_dispatch | 1454964ns | 1454964ns | +0.09% |
| abi_boundary_w_leaf_scalar_per_w | 1451450ns | 1451450ns | -0.15% |
| abi_boundary_w_leaf_scalar_runtime_w | 1453696ns | 1453696ns | base |
| abi_boundary_w_leaf_soa_dispatch | 815528ns | 815528ns | -43.90% |
| abi_boundary_w_leaf_soa_per_w | 819902ns | 819902ns | -43.60% |
| abi_boundary_w_leaf_soa_runtime_w | 816547ns | 816547ns | -43.83% |
| abi_boundary_w_leaf_zig_runtime_w | 1434825ns | 1434825ns | -1.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1449731ns | base | --- | [1446241, 1457712] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 3266ns | -1446451.6ns (-99.8%) | [-1454394, -1443151]ns | [3090, 3332] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1448478ns | no significant difference | [-9258, +3940]ns | [1447237, 1451170] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1450546ns | no significant difference | [-7319, +11323]ns | [1448288, 1458552] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1448454ns | no significant difference | [-7646, +2519]ns | [1446702, 1451874] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 813526ns | -637982.8ns (-44.0%) | [-641944, -634368]ns | [809887, 815976] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 817072ns | -634131.7ns (-43.7%) | [-642126, -624850]ns | [812639, 822866] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 814706ns | -636286.7ns (-43.9%) | [-641323, -633443]ns | [810943, 816983] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1433284ns | -18092.7ns (-1.2%) | [-26355, -12376]ns | [1429711, 1433865] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1457777ns | -99.8% | -0.6% | -0.4% | -0.3% | -44.0% | -44.2% | -44.1% | -1.9% |
| 2 | 1449091ns | -99.8% | +0.3% | +0.7% | +0.1% | -43.9% | -44.0% | -43.9% | -1.3% |
| 3 | 1447114ns | -99.8% | -0.0% | +0.2% | +0.0% | -43.7% | -42.9% | -43.6% | -0.9% |
| 4 | 1457648ns | -99.8% | -0.7% | -0.6% | -0.8% | -44.1% | -43.9% | -43.9% | -1.7% |
| 5 | 1445368ns | -99.8% | +0.3% | +0.9% | +0.3% | -44.1% | -43.5% | -44.0% | -0.8% |
| 6 | 1450372ns | -99.8% | -0.2% | -0.2% | -0.2% | -44.1% | -43.5% | -43.8% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.227 | moderate- |
| abi_boundary_w_leaf_scalar_anchor | -0.235 | moderate- |
| abi_boundary_w_leaf_scalar_dispatch | -0.521 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_per_w | 0.138 | ok |
| abi_boundary_w_leaf_scalar_runtime_w | -0.456 | moderate- |
| abi_boundary_w_leaf_soa_dispatch | 0.009 | ok |
| abi_boundary_w_leaf_soa_per_w | -0.204 | moderate- |
| abi_boundary_w_leaf_soa_runtime_w | -0.454 | moderate- |
| abi_boundary_w_leaf_zig_runtime_w | 0.427 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 3/6, lost 2/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_leaf_scalar_per_w**: won 3/6, lost 1/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 121162.3ns | 3229.5ns | 3751.8% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4382421.5ns | 1448961.7ns | 302.5% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4392988.7ns | 1452462.1ns | 302.5% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4381594.9ns | 1449009.9ns | 302.4% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4391545.8ns | 1451228.3ns | 302.6% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 2471391.2ns | 813130.0ns | 303.9% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 2486103.7ns | 817525.6ns | 304.1% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 2476200.4ns | 814210.6ns | 304.1% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4535545.7ns | 1432286.9ns | 316.7% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 3084.6-3332.1 ns)
   3084.6 |########################################
   3097.0 |
   3109.3 |
   3121.7 |
   3134.1 |
   3146.5 |
   3158.8 |
   3171.2 |
   3183.6 |
   3196.0 |
   3208.3 |
   3220.7 |
   3233.1 |
   3245.5 |
   3257.8 |########################################
   3270.2 |
   3282.6 |####################
   3295.0 |
   3307.3 |
   3319.7 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1446812.1-1451170.0 ns)
  1446812.1 |####################
  1447030.0 |
  1447247.9 |
  1447465.8 |####################
  1447683.7 |####################
  1447901.6 |
  1448119.5 |
  1448337.4 |
  1448555.3 |
  1448773.2 |
  1448991.1 |
  1449208.9 |########################################
  1449426.8 |
  1449644.7 |
  1449862.6 |
  1450080.5 |
  1450298.4 |
  1450516.3 |
  1450734.2 |
  1450952.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1447211.7-1458552.3 ns)
  1447211.7 |########################################
  1447778.7 |
  1448345.8 |
  1448912.8 |########################################
  1449479.8 |########################################
  1450046.9 |
  1450613.9 |
  1451180.9 |########################################
  1451747.9 |
  1452315.0 |
  1452882.0 |
  1453449.0 |
  1454016.1 |
  1454583.1 |
  1455150.1 |
  1455717.1 |
  1456284.2 |
  1456851.2 |
  1457418.2 |
  1457985.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1446397.9-1451873.8 ns)
  1446397.9 |########################################
  1446671.7 |
  1446945.5 |########################################
  1447219.3 |########################################
  1447493.1 |
  1447766.9 |
  1448040.7 |
  1448314.4 |
  1448588.2 |
  1448862.0 |
  1449135.8 |
  1449409.6 |########################################
  1449683.4 |
  1449957.2 |########################################
  1450231.0 |
  1450504.8 |
  1450778.6 |
  1451052.4 |
  1451326.2 |
  1451600.0 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1445368.3-1457712.3 ns)
  1445368.3 |########################################
  1445985.5 |
  1446602.7 |########################################
  1447219.9 |
  1447837.1 |
  1448454.3 |
  1449071.5 |########################################
  1449688.7 |
  1450305.9 |########################################
  1450923.1 |
  1451540.3 |
  1452157.5 |
  1452774.7 |
  1453391.9 |
  1454009.1 |
  1454626.3 |
  1455243.5 |
  1455860.7 |
  1456477.9 |
  1457095.1 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 808323.3-815976.4 ns)
  808323.3 |########################################
  808706.0 |
  809088.6 |
  809471.3 |
  809853.9 |
  810236.6 |
  810619.2 |
  811001.9 |
  811384.6 |########################################
  811767.2 |
  812149.9 |########################################
  812532.5 |
  812915.2 |
  813297.8 |
  813680.5 |
  814063.2 |
  814445.8 |########################################
  814828.5 |########################################
  815211.1 |
  815593.8 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 811882.1-822866.1 ns)
  811882.1 |########################################
  812431.3 |
  812980.5 |########################################
  813529.7 |
  814078.9 |
  814628.1 |
  815177.3 |
  815726.5 |
  816275.7 |########################################
  816824.9 |
  817374.1 |########################################
  817923.3 |
  818472.5 |
  819021.7 |########################################
  819570.9 |
  820120.1 |
  820669.3 |
  821218.5 |
  821767.7 |
  822316.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 809199.6-816982.7 ns)
  809199.6 |########################################
  809588.8 |
  809977.9 |
  810367.1 |
  810756.2 |
  811145.4 |
  811534.5 |
  811923.7 |
  812312.8 |########################################
  812702.0 |
  813091.1 |
  813480.3 |
  813869.5 |
  814258.6 |########################################
  814647.8 |########################################
  815036.9 |
  815426.1 |########################################
  815815.2 |
  816204.4 |
  816593.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1429380.8-1433865.2 ns)
  1429380.8 |####################
  1429605.0 |
  1429829.2 |####################
  1430053.5 |
  1430277.7 |
  1430501.9 |
  1430726.1 |
  1430950.4 |
  1431174.6 |
  1431398.8 |
  1431623.0 |
  1431847.2 |
  1432071.5 |
  1432295.7 |
  1432519.9 |
  1432744.1 |
  1432968.4 |
  1433192.6 |########################################
  1433416.8 |####################
  1433641.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=3724.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=316.7% of algo (FFI overhead may distort results)

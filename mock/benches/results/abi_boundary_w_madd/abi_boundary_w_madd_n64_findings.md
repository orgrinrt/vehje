# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_madd_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_madd_scalar_runtime_w has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_madd_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_madd_null_entry dominates: 41663% faster than the next best (abi_boundary_w_madd_soa_dispatch)

abi_boundary_w_madd_null_entry (2.56 us) leads abi_boundary_w_madd_soa_dispatch (1.07 ms) by 41663%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.68 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_runtime_w is an outlier: 1047.4x slower than the field

abi_boundary_w_madd_scalar_runtime_w (2.68 ms) is 1047.4x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_soa_per_w shows alternating (throttle bounce) (autocorr -0.53)

abi_boundary_w_madd_soa_per_w's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_runtime_w} (41663% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_runtime_w} with a 41663% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1047.4x the fastest

Fastest abi_boundary_w_madd_null_entry (2.56 us) to slowest abi_boundary_w_madd_scalar_runtime_w (2.68 ms): 1047.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 2557.3 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 1047.44x (fastest 2557.3 ns, slowest 2678610.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 4785ns | 4830ns | 4480ns | 4732ns | 5018ns | -99.82% |
| abi_boundary_w_madd_scalar_anchor | 2669484ns | 2665224ns | 2655263ns | 2664438ns | 2684164ns | -0.55% |
| abi_boundary_w_madd_scalar_dispatch | 2671517ns | 2671139ns | 2667232ns | 2669965ns | 2675987ns | -0.47% |
| abi_boundary_w_madd_scalar_per_w | 2663320ns | 2666020ns | 2648680ns | 2664034ns | 2669569ns | -0.78% |
| abi_boundary_w_madd_scalar_runtime_w | 2684138ns | 2681160ns | 2673305ns | 2679605ns | 2696354ns | base |
| abi_boundary_w_madd_soa_dispatch | 1069560ns | 1070332ns | 1060829ns | 1069054ns | 1074684ns | -60.15% |
| abi_boundary_w_madd_soa_per_w | 1074790ns | 1075982ns | 1070819ns | 1074658ns | 1076973ns | -59.96% |
| abi_boundary_w_madd_soa_runtime_w | 1072267ns | 1073005ns | 1062080ns | 1071716ns | 1078186ns | -60.05% |
| abi_boundary_w_madd_zig_runtime_w | 2669710ns | 2671988ns | 2660362ns | 2668864ns | 2675652ns | -0.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 2523ns | 2352ns | 2624ns | -99.91% | 0.025 |
| abi_boundary_w_madd_scalar_anchor | 2666923ns | 2652888ns | 2681546ns | -0.55% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2668990ns | 2664914ns | 2673444ns | -0.47% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2660845ns | 2646324ns | 2667040ns | -0.77% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2681570ns | 2670508ns | 2693784ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 1067149ns | 1058387ns | 1072216ns | -60.20% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 1072390ns | 1068498ns | 1074535ns | -60.01% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 1069967ns | 1059805ns | 1075873ns | -60.10% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2667188ns | 2657954ns | 2673118ns | -0.54% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 26730.0 | 2739.2 | 2522.7 | n/a |
| abi_boundary_w_madd_scalar_anchor | 38659.0 | 2669451.8 | 2666923.0 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 38314.6 | 2670461.1 | 2668990.5 | n/a |
| abi_boundary_w_madd_scalar_per_w | 38951.8 | 2662421.0 | 2660845.0 | 0 |
| abi_boundary_w_madd_scalar_runtime_w | 38485.5 | 2681243.2 | 2681569.7 | n/a |
| abi_boundary_w_madd_soa_dispatch | 31110.1 | 1066614.6 | 1067148.9 | n/a |
| abi_boundary_w_madd_soa_per_w | 32508.2 | 1071297.6 | 1072390.1 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 30219.8 | 1070187.8 | 1069967.4 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 178866.2 | 2670886.2 | 2667188.1 | 10 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.025 | 92.0% |
| abi_boundary_w_madd_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_madd_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_madd_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_madd_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_madd_null_entry | 4785ns | 4785ns | -99.82% |
| abi_boundary_w_madd_scalar_anchor | 2669484ns | 2669484ns | -0.55% |
| abi_boundary_w_madd_scalar_dispatch | 2671517ns | 2671517ns | -0.47% |
| abi_boundary_w_madd_scalar_per_w | 2663320ns | 2663320ns | -0.78% |
| abi_boundary_w_madd_scalar_runtime_w | 2684138ns | 2684138ns | base |
| abi_boundary_w_madd_soa_dispatch | 1069560ns | 1069560ns | -60.15% |
| abi_boundary_w_madd_soa_per_w | 1074790ns | 1074790ns | -59.96% |
| abi_boundary_w_madd_soa_runtime_w | 1072267ns | 1072267ns | -60.05% |
| abi_boundary_w_madd_zig_runtime_w | 2669710ns | 2669710ns | -0.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2678611ns | base | --- | [2672314, 2693784] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 2557ns | -2676024.4ns (-99.9%) | [-2691323, -2669794]ns | [2386, 2624] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2662552ns | no significant difference | [-36977, +9232]ns | [2656671, 2681546] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2668552ns | no significant difference | [-28809, +1130]ns | [2664975, 2673444] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2663482ns | -21302.9ns (-0.8%) | [-27076, -13795]ns | [2652013, 2667040] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_dispatch | 1068015ns | -1613920.2ns (-60.3%) | [-1625558, -1603784]ns | [1061216, 1072216] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 1073515ns | -1604911.5ns (-59.9%) | [-1624663, -1597964]ns | [1069121, 1074535] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 1070745ns | -1613658.1ns (-60.2%) | [-1622265, -1598883]ns | [1063284, 1075873] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2669506ns | -11371.4ns (-0.4%) | [-28018, -3756]ns | [2658940, 2673118] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2677457ns | -99.9% | -0.5% | -0.2% | -0.4% | -59.9% | -59.9% | -59.8% | -0.2% |
| 2 | 2696430ns | -99.9% | -1.6% | -1.2% | -1.1% | -60.3% | -60.4% | -60.1% | -1.4% |
| 3 | 2679765ns | -99.9% | -0.7% | -0.5% | -0.7% | -60.5% | -59.9% | -60.5% | -0.3% |
| 4 | 2691139ns | -99.9% | -1.1% | -1.0% | -0.9% | -60.4% | -60.2% | -60.4% | -0.7% |
| 5 | 2670508ns | -99.9% | +0.1% | +0.2% | -0.9% | -60.2% | -59.7% | -59.8% | -0.1% |
| 6 | 2674120ns | -99.9% | +0.6% | -0.1% | -0.6% | -60.0% | -59.9% | -60.1% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | -0.349 | moderate- |
| abi_boundary_w_madd_scalar_anchor | 0.310 | moderate+ |
| abi_boundary_w_madd_scalar_dispatch | 0.034 | ok |
| abi_boundary_w_madd_scalar_per_w | 0.062 | ok |
| abi_boundary_w_madd_scalar_runtime_w | -0.252 | moderate- |
| abi_boundary_w_madd_soa_dispatch | 0.003 | ok |
| abi_boundary_w_madd_soa_per_w | -0.529 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_runtime_w | -0.116 | ok |
| abi_boundary_w_madd_zig_runtime_w | -0.277 | moderate- |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 4/6, lost 1/6
- **abi_boundary_w_madd_scalar_dispatch**: won 5/6, lost 1/6
- **abi_boundary_w_madd_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 114031.4ns | 2522.7ns | 4520.3% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8037699.4ns | 2666923.0ns | 301.4% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8049597.8ns | 2668990.5ns | 301.6% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8030555.8ns | 2660845.0ns | 301.8% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8084548.1ns | 2681569.7ns | 301.5% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 3233966.9ns | 1067148.9ns | 303.0% | HIGH |
| abi_boundary_w_madd_soa_per_w | 3249003.0ns | 1072390.1ns | 303.0% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 3241523.9ns | 1069967.4ns | 303.0% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8257316.2ns | 2667188.1ns | 309.6% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 2352.5-2624.2 ns)
   2352.5 |########################################
   2366.1 |
   2379.7 |
   2393.3 |
   2406.8 |########################################
   2420.4 |
   2434.0 |
   2447.6 |
   2461.2 |
   2474.8 |
   2488.3 |
   2501.9 |
   2515.5 |
   2529.1 |
   2542.7 |########################################
   2556.3 |
   2569.9 |########################################
   2583.4 |
   2597.0 |
   2610.6 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2652888.3-2681545.9 ns)
  2652888.3 |####################
  2654321.2 |
  2655754.1 |
  2657186.9 |
  2658619.8 |
  2660052.7 |########################################
  2661485.6 |
  2662918.4 |
  2664351.3 |####################
  2665784.2 |
  2667217.1 |
  2668650.0 |
  2670082.8 |
  2671515.7 |####################
  2672948.6 |
  2674381.5 |
  2675814.3 |
  2677247.2 |
  2678680.1 |
  2680113.0 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2664914.2-2673443.5 ns)
  2664914.2 |########################################
  2665340.7 |
  2665767.1 |
  2666193.6 |####################
  2666620.1 |
  2667046.5 |
  2667473.0 |
  2667899.5 |
  2668325.9 |
  2668752.4 |
  2669178.9 |
  2669605.3 |
  2670031.8 |
  2670458.3 |####################
  2670884.7 |
  2671311.2 |####################
  2671737.7 |
  2672164.1 |
  2672590.6 |
  2673017.1 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2646324.2-2667040.2 ns)
  2646324.2 |########################################
  2647360.0 |
  2648395.8 |
  2649431.6 |
  2650467.4 |
  2651503.2 |
  2652539.0 |
  2653574.8 |
  2654610.6 |
  2655646.4 |
  2656682.2 |########################################
  2657718.0 |
  2658753.8 |
  2659789.6 |
  2660825.4 |########################################
  2661861.2 |
  2662897.0 |
  2663932.8 |
  2664968.6 |########################################
  2666004.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2670507.9-2693784.4 ns)
  2670507.9 |########################################
  2671671.7 |
  2672835.5 |
  2673999.4 |########################################
  2675163.2 |
  2676327.0 |########################################
  2677490.9 |
  2678654.7 |########################################
  2679818.5 |
  2680982.3 |
  2682146.1 |
  2683310.0 |
  2684473.8 |
  2685637.6 |
  2686801.4 |
  2687965.3 |
  2689129.1 |
  2690292.9 |########################################
  2691456.8 |
  2692620.6 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 1058386.7-1072215.9 ns)
  1058386.7 |########################################
  1059078.2 |
  1059769.6 |
  1060461.1 |
  1061152.5 |
  1061844.0 |
  1062535.4 |
  1063226.9 |
  1063918.4 |########################################
  1064609.8 |########################################
  1065301.3 |
  1065992.7 |
  1066684.2 |
  1067375.6 |
  1068067.1 |
  1068758.6 |
  1069450.0 |
  1070141.5 |########################################
  1070832.9 |########################################
  1071524.4 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 1068498.3-1074534.8 ns)
  1068498.3 |####################
  1068800.1 |
  1069101.9 |
  1069403.8 |
  1069705.6 |####################
  1070007.4 |
  1070309.2 |
  1070611.1 |
  1070912.9 |
  1071214.7 |
  1071516.5 |
  1071818.3 |
  1072120.2 |
  1072422.0 |
  1072723.8 |
  1073025.6 |
  1073327.5 |########################################
  1073629.3 |
  1073931.1 |####################
  1074232.9 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 1059804.6-1075872.9 ns)
  1059804.6 |####################
  1060608.0 |
  1061411.4 |
  1062214.8 |
  1063018.3 |
  1063821.7 |
  1064625.1 |
  1065428.5 |
  1066231.9 |########################################
  1067035.3 |
  1067838.8 |
  1068642.2 |
  1069445.6 |
  1070249.0 |
  1071052.4 |
  1071855.8 |
  1072659.2 |
  1073462.7 |
  1074266.1 |####################
  1075069.5 |####################
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2657954.2-2673117.5 ns)
  2657954.2 |########################################
  2658712.4 |
  2659470.5 |########################################
  2660228.7 |
  2660986.9 |
  2661745.0 |
  2662503.2 |
  2663261.4 |
  2664019.5 |
  2664777.7 |
  2665535.9 |
  2666294.0 |
  2667052.2 |########################################
  2667810.3 |
  2668568.5 |
  2669326.7 |
  2670084.8 |
  2670843.0 |########################################
  2671601.2 |
  2672359.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=4440.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.3% of algo (FFI overhead may distort results)

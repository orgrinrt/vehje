# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 100942% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (2.67 us) leads abi_zig_entry_madd_zig_anchor (2.70 ms) by 100942%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.76 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_dispatch is an outlier: 1198.6x slower than the field

abi_zig_entry_madd_zig_tail_dispatch (3.20 ms) is 1198.6x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_tail_runtime_w, abi_zig_entry_madd_zig_tail_dispatch} (100942% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_tail_runtime_w, abi_zig_entry_madd_zig_tail_dispatch} with a 100942% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1198.6x the fastest

Fastest abi_zig_entry_madd_zig_null (2.67 us) to slowest abi_zig_entry_madd_zig_tail_dispatch (3.20 ms): 1198.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 2669.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1198.65x (fastest 2669.8 ns, slowest 3200149.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2732011ns | 2700710ns | 2692836ns | 2699928ns | 2799724ns | -2.29% |
| abi_zig_entry_madd_zig_dispatch | 2743509ns | 2720100ns | 2717071ns | 2719381ns | 2792922ns | -1.88% |
| abi_zig_entry_madd_zig_null | 4985ns | 4960ns | 4903ns | 4944ns | 5088ns | -99.82% |
| abi_zig_entry_madd_zig_per_w_set | 2725097ns | 2717482ns | 2711124ns | 2717149ns | 2744007ns | -2.54% |
| abi_zig_entry_madd_zig_runtime_w | 2796001ns | 2768770ns | 2708444ns | 2748820ns | 2910550ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3206501ns | 3203491ns | 3185698ns | 3199419ns | 3227525ns | +14.68% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3214781ns | 3202562ns | 3182495ns | 3197801ns | 3256394ns | +14.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2728608ns | 2689897ns | 2795671ns | -2.28% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2739989ns | 2713954ns | 2788895ns | -1.88% | 0.000 |
| abi_zig_entry_madd_zig_null | 2674ns | 2641ns | 2703ns | -99.90% | 0.048 |
| abi_zig_entry_madd_zig_per_w_set | 2721798ns | 2708080ns | 2740527ns | -2.53% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2792350ns | 2705366ns | 2906329ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3203130ns | 3182745ns | 3223885ns | +14.71% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3211495ns | 3179768ns | 3252768ns | +15.01% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 226694.9 | 2735937.6 | 2728607.5 | n/a |
| abi_zig_entry_madd_zig_dispatch | 229660.7 | 2737029.6 | 2739988.6 | n/a |
| abi_zig_entry_madd_zig_null | 157558.2 | 2763.3 | 2673.7 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 224967.7 | 2721282.1 | 2721798.2 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 255550.2 | 2813906.5 | 2792350.1 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 231492.6 | 3201885.8 | 3203129.9 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 223564.4 | 3224890.6 | 3211495.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.048 | 98.9% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2732011ns | 2732011ns | -2.29% |
| abi_zig_entry_madd_zig_dispatch | 2743509ns | 2743509ns | -1.88% |
| abi_zig_entry_madd_zig_null | 4985ns | 4985ns | -99.82% |
| abi_zig_entry_madd_zig_per_w_set | 2725097ns | 2725097ns | -2.54% |
| abi_zig_entry_madd_zig_runtime_w | 2796001ns | 2796001ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3206501ns | 3206501ns | +14.68% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3214781ns | 3214781ns | +14.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2765106ns | base | --- | [2705616, 2906329] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2697611ns | -23547.1ns (-0.9%) | [-159409, -8272]ns | [2692540, 2795671] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2716759ns | no significant difference | [-155212, +10876]ns | [2714311, 2788895] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_madd_zig_null | 2670ns | -2762446.7ns (-99.9%) | [-2903637, -2702946]ns | [2648, 2703] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2714160ns | no significant difference | [-167062, +9341]ns | [2710707, 2740527] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3200150ns | +433885.4ns (+15.7%) | [+303920, +494534]ns | [3185355, 3223885] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3199202ns | +432177.1ns (+15.6%) | [+331673, +493586]ns | [3182516, 3252768] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2926365ns | -1.2% | -7.2% | -99.9% | -7.2% | +10.6% | +10.5% |
| 2 | 2886292ns | -6.4% | -1.2% | -99.9% | -4.2% | +10.3% | +13.4% |
| 3 | 2705366ns | -0.3% | +0.3% | -99.9% | +0.3% | +18.1% | +18.0% |
| 4 | 2706400ns | -0.3% | +0.5% | -99.9% | +0.4% | +17.8% | +17.7% |
| 5 | 2705865ns | -0.4% | +0.3% | -99.9% | +0.1% | +18.5% | +18.5% |
| 6 | 2823812ns | -4.7% | -3.5% | -99.9% | -3.9% | +13.7% | +12.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.009 | ok |
| abi_zig_entry_madd_zig_dispatch | -0.275 | moderate- |
| abi_zig_entry_madd_zig_null | -0.196 | ok |
| abi_zig_entry_madd_zig_per_w_set | -0.205 | moderate- |
| abi_zig_entry_madd_zig_runtime_w | 0.331 | moderate+ |
| abi_zig_entry_madd_zig_tail_dispatch | -0.207 | moderate- |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.160 | ok |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 3/6, lost 2/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8506239.9ns | 2728607.5ns | 311.7% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8524362.8ns | 2739988.6ns | 311.1% | HIGH |
| abi_zig_entry_madd_zig_null | 307224.4ns | 2673.7ns | 11490.5% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8468894.4ns | 2721798.2ns | 311.2% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8755574.5ns | 2792350.1ns | 313.6% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9928299.8ns | 3203129.9ns | 310.0% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 9958110.6ns | 3211495.4ns | 310.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2689897.1-2795670.9 ns)
  2689897.1 |########################################
  2695185.8 |########################################
  2700474.5 |####################
  2705763.2 |
  2711051.9 |
  2716340.5 |
  2721629.2 |
  2726917.9 |
  2732206.6 |
  2737495.3 |
  2742784.0 |
  2748072.7 |
  2753361.4 |
  2758650.0 |
  2763938.7 |
  2769227.4 |
  2774516.1 |
  2779804.8 |
  2785093.5 |
  2790382.2 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2713954.2-2788895.5 ns)
  2713954.2 |########################################
  2717701.3 |#############
  2721448.3 |#############
  2725195.4 |
  2728942.5 |
  2732689.5 |
  2736436.6 |
  2740183.6 |
  2743930.7 |
  2747677.8 |
  2751424.8 |
  2755171.9 |
  2758919.0 |
  2762666.0 |
  2766413.1 |
  2770160.1 |
  2773907.2 |
  2777654.3 |
  2781401.3 |
  2785148.4 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 2640.8-2703.3 ns)
   2640.8 |########################################
   2643.9 |
   2647.1 |
   2650.2 |
   2653.3 |########################################
   2656.4 |
   2659.6 |
   2662.7 |
   2665.8 |########################################
   2668.9 |
   2672.1 |########################################
   2675.2 |
   2678.3 |########################################
   2681.4 |
   2684.6 |
   2687.7 |
   2690.8 |
   2693.9 |
   2697.1 |
   2700.2 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2708080.4-2740527.3 ns)
  2708080.4 |####################
  2709702.7 |
  2711325.1 |
  2712947.4 |########################################
  2714569.8 |####################
  2716192.1 |####################
  2717814.5 |
  2719436.8 |
  2721059.2 |
  2722681.5 |
  2724303.8 |
  2725926.2 |
  2727548.5 |
  2729170.9 |
  2730793.2 |
  2732415.6 |
  2734037.9 |
  2735660.3 |
  2737282.6 |
  2738905.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2705365.8-2906328.5 ns)
  2705365.8 |########################################
  2715413.9 |
  2725462.1 |
  2735510.2 |
  2745558.3 |
  2755606.5 |
  2765654.6 |
  2775702.8 |
  2785750.9 |
  2795799.0 |
  2805847.2 |
  2815895.3 |#############
  2825943.4 |
  2835991.6 |
  2846039.7 |
  2856087.9 |
  2866136.0 |
  2876184.1 |
  2886232.3 |#############
  2896280.4 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3182745.4-3223885.2 ns)
  3182745.4 |########################################
  3184802.4 |
  3186859.4 |########################################
  3188916.4 |
  3190973.4 |
  3193030.4 |########################################
  3195087.3 |
  3197144.3 |
  3199201.3 |
  3201258.3 |
  3203315.3 |
  3205372.3 |########################################
  3207429.3 |
  3209486.3 |########################################
  3211543.3 |
  3213600.2 |
  3215657.2 |
  3217714.2 |
  3219771.2 |
  3221828.2 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3179767.5-3252768.1 ns)
  3179767.5 |########################################
  3183417.5 |########################################
  3187067.6 |
  3190717.6 |########################################
  3194367.6 |
  3198017.6 |
  3201667.7 |
  3205317.7 |########################################
  3208967.7 |
  3212617.8 |
  3216267.8 |
  3219917.8 |
  3223567.9 |
  3227217.9 |
  3230867.9 |########################################
  3234518.0 |
  3238168.0 |
  3241818.0 |
  3245468.0 |
  3249118.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=11457.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=310.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=309.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=309.7% of algo (FFI overhead may distort results)

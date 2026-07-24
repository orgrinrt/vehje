# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_madd_null_entry dominates: 32212% faster than the next best (abi_boundary_w_madd_soa_runtime_w)

abi_boundary_w_madd_null_entry (3.31 us) leads abi_boundary_w_madd_soa_runtime_w (1.07 ms) by 32212%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.67 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_per_w is an outlier: 812.6x slower than the field

abi_boundary_w_madd_scalar_per_w (2.69 ms) is 812.6x the fastest (3.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_per_w} (32212% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_per_w} with a 32212% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 812.6x the fastest

Fastest abi_boundary_w_madd_null_entry (3.31 us) to slowest abi_boundary_w_madd_scalar_per_w (2.69 ms): 812.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 3306.9 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 812.62x (fastest 3306.9 ns, slowest 2687263.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5561ns | 5650ns | 5250ns | 5533ns | 5757ns | -99.79% |
| abi_boundary_w_madd_scalar_anchor | 2662464ns | 2663234ns | 2652112ns | 2660353ns | 2670806ns | -0.66% |
| abi_boundary_w_madd_scalar_dispatch | 2666325ns | 2667252ns | 2654575ns | 2664297ns | 2675242ns | -0.51% |
| abi_boundary_w_madd_scalar_per_w | 2685598ns | 2689925ns | 2655522ns | 2685704ns | 2700478ns | +0.21% |
| abi_boundary_w_madd_scalar_runtime_w | 2680039ns | 2676153ns | 2669840ns | 2674794ns | 2693006ns | base |
| abi_boundary_w_madd_soa_dispatch | 1075251ns | 1076468ns | 1066455ns | 1073495ns | 1082284ns | -59.88% |
| abi_boundary_w_madd_soa_per_w | 1076493ns | 1077880ns | 1064475ns | 1077226ns | 1081404ns | -59.83% |
| abi_boundary_w_madd_soa_runtime_w | 1068549ns | 1070774ns | 1060675ns | 1068401ns | 1072709ns | -60.13% |
| abi_boundary_w_madd_zig_runtime_w | 2676486ns | 2677522ns | 2665358ns | 2674496ns | 2685034ns | -0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 3248ns | 3049ns | 3364ns | -99.88% | 0.079 |
| abi_boundary_w_madd_scalar_anchor | 2659968ns | 2649697ns | 2668141ns | -0.66% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2663711ns | 2651912ns | 2672583ns | -0.52% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2683035ns | 2653086ns | 2697823ns | +0.20% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2677580ns | 2667544ns | 2690470ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 1072907ns | 1064050ns | 1079919ns | -59.93% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 1074203ns | 1062225ns | 1079116ns | -59.88% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 1066267ns | 1058438ns | 1070341ns | -60.18% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2673842ns | 2662750ns | 2682371ns | -0.14% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 26797.3 | 3262.6 | 3247.9 | n/a |
| abi_boundary_w_madd_scalar_anchor | 37663.1 | 2663958.0 | 2659968.0 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 38878.2 | 2664511.5 | 2663711.0 | 0 |
| abi_boundary_w_madd_scalar_per_w | 38389.9 | 2682096.2 | 2683035.0 | n/a |
| abi_boundary_w_madd_scalar_runtime_w | 37016.2 | 2676373.1 | 2677580.3 | n/a |
| abi_boundary_w_madd_soa_dispatch | 31762.3 | 1072083.8 | 1072906.5 | n/a |
| abi_boundary_w_madd_soa_per_w | 29919.2 | 1074083.3 | 1074203.2 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 29584.6 | 1066564.2 | 1066267.3 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 179374.8 | 2674056.0 | 2673842.5 | 38 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.084 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.077 | 92.2% |
| abi_boundary_w_madd_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_madd_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_madd_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_madd_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5561ns | 5561ns | -99.79% |
| abi_boundary_w_madd_scalar_anchor | 2662464ns | 2662464ns | -0.66% |
| abi_boundary_w_madd_scalar_dispatch | 2666325ns | 2666325ns | -0.51% |
| abi_boundary_w_madd_scalar_per_w | 2685598ns | 2685598ns | +0.21% |
| abi_boundary_w_madd_scalar_runtime_w | 2680039ns | 2680039ns | base |
| abi_boundary_w_madd_soa_dispatch | 1075251ns | 1075251ns | -59.88% |
| abi_boundary_w_madd_soa_per_w | 1076493ns | 1076493ns | -59.83% |
| abi_boundary_w_madd_soa_runtime_w | 1068549ns | 1068549ns | -60.13% |
| abi_boundary_w_madd_zig_runtime_w | 2676486ns | 2676486ns | -0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2673698ns | base | --- | [2668572, 2690470] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 3307ns | -2670470.2ns (-99.9%) | [-2687157, -2665370]ns | [3073, 3364] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2660876ns | -18441.2ns (-0.7%) | [-27805, -6590]ns | [2650887, 2668141] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2664572ns | -5962.4ns (-0.2%) | [-30914, -4731]ns | [2653978, 2672583] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2687264ns | no significant difference | [-22407, +25658]ns | [2664018, 2697823] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_madd_soa_dispatch | 1074158ns | -1601021.2ns (-59.9%) | [-1622813, -1590187]ns | [1064643, 1079919] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 1075582ns | -1602224.8ns (-59.9%) | [-1614916, -1592990]ns | [1067911, 1079116] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 1068512ns | -1608623.9ns (-60.2%) | [-1620730, -1604585]ns | [1059949, 1070341] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2674890ns | no significant difference | [-23190, +13799]ns | [2664266, 2682371] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2700183ns | -99.9% | -1.1% | -1.6% | -1.7% | -60.6% | -60.0% | -60.4% | -1.3% |
| 2 | 2669601ns | -99.9% | -0.1% | -0.7% | +1.0% | -59.7% | -59.7% | -60.2% | +0.6% |
| 3 | 2667544ns | -99.9% | -0.6% | -0.2% | +0.6% | -59.5% | -59.6% | -60.3% | +0.4% |
| 4 | 2680758ns | -99.9% | -0.8% | -0.2% | +0.4% | -60.0% | -60.0% | -60.1% | -0.2% |
| 5 | 2674729ns | -99.9% | -0.9% | -0.2% | +1.0% | -60.2% | -59.7% | -60.1% | -0.4% |
| 6 | 2672668ns | -99.9% | -0.4% | -0.2% | +0.1% | -59.6% | -60.3% | -60.0% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | -0.134 | ok |
| abi_boundary_w_madd_scalar_anchor | -0.006 | ok |
| abi_boundary_w_madd_scalar_dispatch | 0.450 | moderate+ |
| abi_boundary_w_madd_scalar_per_w | -0.238 | moderate- |
| abi_boundary_w_madd_scalar_runtime_w | -0.177 | ok |
| abi_boundary_w_madd_soa_dispatch | -0.191 | ok |
| abi_boundary_w_madd_soa_per_w | -0.193 | ok |
| abi_boundary_w_madd_soa_runtime_w | 0.053 | ok |
| abi_boundary_w_madd_zig_runtime_w | -0.181 | ok |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_per_w**: won 1/6, lost 4/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_zig_runtime_w**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 121104.5ns | 3247.9ns | 3728.7% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8031516.4ns | 2659968.0ns | 301.9% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8034268.3ns | 2663711.0ns | 301.6% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8087267.6ns | 2683035.0ns | 301.4% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8064656.0ns | 2677580.3ns | 301.2% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 3250986.6ns | 1072906.5ns | 303.0% | HIGH |
| abi_boundary_w_madd_soa_per_w | 3254206.8ns | 1074203.2ns | 302.9% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 3231247.3ns | 1066267.3ns | 303.0% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8269749.1ns | 2673842.5ns | 309.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 3048.8-3363.8 ns)
   3048.8 |####################
   3064.5 |
   3080.3 |
   3096.0 |####################
   3111.8 |
   3127.5 |
   3143.3 |
   3159.0 |
   3174.8 |
   3190.5 |
   3206.3 |
   3222.0 |
   3237.8 |
   3253.5 |####################
   3269.3 |
   3285.0 |
   3300.8 |
   3316.5 |
   3332.3 |
   3348.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2649697.1-2668141.0 ns)
  2649697.1 |########################################
  2650619.3 |
  2651541.5 |########################################
  2652463.7 |
  2653385.9 |
  2654308.1 |
  2655230.3 |
  2656152.5 |
  2657074.7 |
  2657996.9 |
  2658919.1 |########################################
  2659841.3 |
  2660763.5 |
  2661685.7 |########################################
  2662607.9 |
  2663530.1 |
  2664452.3 |
  2665374.5 |
  2666296.7 |########################################
  2667218.9 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2651912.1-2672582.8 ns)
  2651912.1 |########################################
  2652945.6 |
  2653979.2 |
  2655012.7 |########################################
  2656046.2 |
  2657079.8 |
  2658113.3 |
  2659146.8 |
  2660180.4 |
  2661213.9 |
  2662247.4 |########################################
  2663281.0 |
  2664314.5 |
  2665348.0 |########################################
  2666381.6 |
  2667415.1 |
  2668448.6 |########################################
  2669482.2 |
  2670515.7 |
  2671549.2 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2653085.8-2697823.1 ns)
  2653085.8 |########################################
  2655322.7 |
  2657559.5 |
  2659796.4 |
  2662033.3 |
  2664270.1 |
  2666507.0 |
  2668743.9 |
  2670980.7 |
  2673217.6 |########################################
  2675454.4 |
  2677691.3 |
  2679928.2 |
  2682165.0 |########################################
  2684401.9 |
  2686638.8 |
  2688875.6 |
  2691112.5 |########################################
  2693349.4 |########################################
  2695586.2 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2667544.2-2690470.2 ns)
  2667544.2 |########################################
  2668690.5 |########################################
  2669836.8 |
  2670983.1 |
  2672129.4 |########################################
  2673275.7 |
  2674422.0 |########################################
  2675568.3 |
  2676714.6 |
  2677860.9 |
  2679007.2 |
  2680153.5 |########################################
  2681299.8 |
  2682446.1 |
  2683592.4 |
  2684738.7 |
  2685885.0 |
  2687031.3 |
  2688177.6 |
  2689323.9 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 1064049.6-1079918.9 ns)
  1064049.6 |########################################
  1064843.1 |########################################
  1065636.5 |
  1066430.0 |
  1067223.5 |
  1068016.9 |
  1068810.4 |
  1069603.9 |
  1070397.3 |
  1071190.8 |
  1071984.3 |
  1072777.7 |
  1073571.2 |########################################
  1074364.7 |########################################
  1075158.1 |
  1075951.6 |
  1076745.1 |
  1077538.5 |
  1078332.0 |########################################
  1079125.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 1062225.0-1079116.1 ns)
  1062225.0 |########################################
  1063069.6 |
  1063914.1 |
  1064758.7 |
  1065603.2 |
  1066447.8 |
  1067292.3 |
  1068136.9 |
  1068981.4 |
  1069826.0 |
  1070670.5 |
  1071515.1 |
  1072359.6 |
  1073204.2 |########################################
  1074048.7 |########################################
  1074893.3 |
  1075737.8 |
  1076582.4 |########################################
  1077426.9 |########################################
  1078271.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 1058437.5-1070341.4 ns)
  1058437.5 |########################################
  1059032.7 |
  1059627.9 |
  1060223.1 |
  1060818.3 |
  1061413.5 |########################################
  1062008.7 |
  1062603.9 |
  1063199.1 |
  1063794.3 |
  1064389.5 |
  1064984.7 |
  1065579.9 |
  1066175.1 |
  1066770.3 |
  1067365.5 |########################################
  1067960.7 |
  1068555.9 |########################################
  1069151.1 |
  1069746.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2662749.6-2682371.0 ns)
  2662749.6 |########################################
  2663730.7 |
  2664711.7 |
  2665692.8 |########################################
  2666673.9 |
  2667655.0 |
  2668636.0 |
  2669617.1 |
  2670598.2 |
  2671579.3 |
  2672560.3 |
  2673541.4 |########################################
  2674522.5 |
  2675503.5 |########################################
  2676484.6 |
  2677465.7 |########################################
  2678446.8 |
  2679427.8 |
  2680408.9 |
  2681390.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=3692.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.1% of algo (FFI overhead may distort results)

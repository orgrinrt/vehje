# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_madd_null_entry dominates: 40774% faster than the next best (abi_boundary_w_madd_soa_runtime_w)

abi_boundary_w_madd_null_entry (2.62 us) leads abi_boundary_w_madd_soa_runtime_w (1.07 ms) by 40774%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.67 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_zig_runtime_w is an outlier: 1023.3x slower than the field

abi_boundary_w_madd_zig_runtime_w (2.68 ms) is 1023.3x the fastest (2.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_soa_per_w shows alternating (throttle bounce) (autocorr -0.84)

abi_boundary_w_madd_soa_per_w's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_zig_runtime_w} (40774% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_zig_runtime_w} with a 40774% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1023.3x the fastest

Fastest abi_boundary_w_madd_null_entry (2.62 us) to slowest abi_boundary_w_madd_zig_runtime_w (2.68 ms): 1023.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 2615.8 ns median (-99.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1023.34x (fastest 2615.8 ns, slowest 2676850.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 4927ns | 4912ns | 4778ns | 4894ns | 5051ns | -99.82% |
| abi_boundary_w_madd_scalar_anchor | 2671675ns | 2676224ns | 2650440ns | 2671271ns | 2682897ns | -0.35% |
| abi_boundary_w_madd_scalar_dispatch | 2675741ns | 2677276ns | 2664231ns | 2675100ns | 2682457ns | -0.19% |
| abi_boundary_w_madd_scalar_per_w | 2680312ns | 2674281ns | 2665347ns | 2672872ns | 2698953ns | -0.02% |
| abi_boundary_w_madd_scalar_runtime_w | 2680953ns | 2676824ns | 2673398ns | 2675853ns | 2692380ns | base |
| abi_boundary_w_madd_soa_dispatch | 1078029ns | 1078494ns | 1073166ns | 1077299ns | 1081556ns | -59.79% |
| abi_boundary_w_madd_soa_per_w | 1076595ns | 1077934ns | 1071608ns | 1075890ns | 1080145ns | -59.84% |
| abi_boundary_w_madd_soa_runtime_w | 1071352ns | 1071450ns | 1062406ns | 1070214ns | 1077533ns | -60.04% |
| abi_boundary_w_madd_zig_runtime_w | 2780735ns | 2679418ns | 2672107ns | 2678323ns | 2988666ns | +3.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 2618ns | 2542ns | 2682ns | -99.90% | 0.006 |
| abi_boundary_w_madd_scalar_anchor | 2669175ns | 2648109ns | 2680469ns | -0.34% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2673154ns | 2661674ns | 2679880ns | -0.20% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2677785ns | 2662675ns | 2696419ns | -0.02% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2678413ns | 2671017ns | 2689694ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 1075619ns | 1070708ns | 1079178ns | -59.84% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 1074227ns | 1069227ns | 1077727ns | -59.89% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 1069079ns | 1060202ns | 1075226ns | -60.09% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2777896ns | 2669668ns | 2985295ns | +3.71% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 26790.5 | 2669.6 | 2618.2 | n/a |
| abi_boundary_w_madd_scalar_anchor | 38461.9 | 2672854.9 | 2669174.6 | n/a |
| abi_boundary_w_madd_scalar_dispatch | 38505.8 | 2676867.4 | 2673153.8 | n/a |
| abi_boundary_w_madd_scalar_per_w | 39629.1 | 2678460.0 | 2677785.1 | 0 |
| abi_boundary_w_madd_scalar_runtime_w | 39473.3 | 2679417.9 | 2678413.1 | n/a |
| abi_boundary_w_madd_soa_dispatch | 32017.6 | 1075867.2 | 1075618.7 | n/a |
| abi_boundary_w_madd_soa_per_w | 31416.8 | 1074038.7 | 1074227.2 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 30636.5 | 1069665.0 | 1069079.2 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 206086.8 | 2822334.5 | 2777896.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.006 | 97.2% |
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
| abi_boundary_w_madd_null_entry | 4927ns | 4927ns | -99.82% |
| abi_boundary_w_madd_scalar_anchor | 2671675ns | 2671675ns | -0.35% |
| abi_boundary_w_madd_scalar_dispatch | 2675741ns | 2675741ns | -0.19% |
| abi_boundary_w_madd_scalar_per_w | 2680312ns | 2680312ns | -0.02% |
| abi_boundary_w_madd_scalar_runtime_w | 2680953ns | 2680953ns | base |
| abi_boundary_w_madd_soa_dispatch | 1078029ns | 1078029ns | -59.79% |
| abi_boundary_w_madd_soa_per_w | 1076595ns | 1076595ns | -59.84% |
| abi_boundary_w_madd_soa_runtime_w | 1071352ns | 1071352ns | -60.04% |
| abi_boundary_w_madd_zig_runtime_w | 2780735ns | 2780735ns | +3.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2674275ns | base | --- | [2671271, 2689694] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 2616ns | -2671656.7ns (-99.9%) | [-2687049, -2668679]ns | [2557, 2682] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2673617ns | no significant difference | [-22906, +4030]ns | [2653438, 2680469] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2674710ns | no significant difference | [-21234, +8608]ns | [2664872, 2679880] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2671812ns | no significant difference | [-14311, +14879]ns | [2665124, 2696419] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_soa_dispatch | 1076096ns | -1600074.8ns (-59.8%) | [-1614963, -1593346]ns | [1071581, 1079178] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 1075541ns | -1600755.9ns (-59.9%) | [-1616210, -1595591]ns | [1069414, 1077727] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 1069181ns | -1607362.3ns (-60.1%) | [-1623871, -1596768]ns | [1062830, 1075226] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2676850ns | no significant difference | [-4069, +300875]ns | [2671544, 2985295] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2672972ns | -99.9% | +0.2% | -0.2% | -0.1% | -59.5% | -59.8% | -59.8% | +0.2% |
| 2 | 2687833ns | -99.9% | -0.2% | -0.5% | -0.6% | -59.9% | -59.9% | -60.1% | -0.2% |
| 3 | 2671525ns | -99.9% | +0.1% | +0.4% | +0.0% | -59.9% | -60.0% | -60.1% | -0.1% |
| 4 | 2675577ns | -99.9% | -1.0% | -0.1% | -0.5% | -59.8% | -59.7% | -60.2% | +0.0% |
| 5 | 2691554ns | -99.9% | -0.7% | -1.1% | +1.1% | -60.2% | -60.3% | -60.6% | +22.2% |
| 6 | 2671017ns | -99.9% | -0.5% | +0.2% | -0.1% | -59.7% | -59.7% | -59.7% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | -0.184 | ok |
| abi_boundary_w_madd_scalar_anchor | -0.048 | ok |
| abi_boundary_w_madd_scalar_dispatch | -0.174 | ok |
| abi_boundary_w_madd_scalar_per_w | -0.416 | moderate- |
| abi_boundary_w_madd_scalar_runtime_w | -0.576 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_dispatch | -0.071 | ok |
| abi_boundary_w_madd_soa_per_w | -0.837 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_runtime_w | -0.149 | ok |
| abi_boundary_w_madd_zig_runtime_w | -0.236 | moderate- |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 4/6, lost 1/6
- **abi_boundary_w_madd_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_madd_scalar_per_w**: won 3/6, lost 1/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_zig_runtime_w**: won 1/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 118157.5ns | 2618.2ns | 4512.8% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8054780.5ns | 2669174.6ns | 301.8% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8068677.6ns | 2673153.8ns | 301.8% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8077553.0ns | 2677785.1ns | 301.7% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8074627.4ns | 2678413.1ns | 301.5% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 3261098.4ns | 1075618.7ns | 303.2% | HIGH |
| abi_boundary_w_madd_soa_per_w | 3254411.5ns | 1074227.2ns | 303.0% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 3240335.0ns | 1069079.2ns | 303.1% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8703224.4ns | 2777896.3ns | 313.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 2541.7-2681.8 ns)
   2541.7 |########################################
   2548.7 |
   2555.7 |
   2562.7 |
   2569.7 |########################################
   2576.7 |
   2583.7 |########################################
   2590.8 |
   2597.8 |
   2604.8 |
   2611.8 |
   2618.8 |
   2625.8 |
   2632.8 |
   2639.8 |########################################
   2646.8 |########################################
   2653.8 |
   2660.8 |
   2667.8 |
   2674.8 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2648109.2-2680468.8 ns)
  2648109.2 |########################################
  2649727.2 |
  2651345.2 |
  2652963.1 |
  2654581.1 |
  2656199.1 |
  2657817.1 |########################################
  2659435.0 |
  2661053.0 |
  2662671.0 |
  2664289.0 |
  2665907.0 |
  2667524.9 |
  2669142.9 |
  2670760.9 |
  2672378.9 |########################################
  2673996.8 |########################################
  2675614.8 |
  2677232.8 |########################################
  2678850.8 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2661674.2-2679879.5 ns)
  2661674.2 |########################################
  2662584.5 |
  2663494.7 |
  2664405.0 |
  2665315.3 |
  2666225.5 |
  2667135.8 |
  2668046.1 |########################################
  2668956.3 |
  2669866.6 |
  2670776.9 |
  2671687.1 |
  2672597.4 |
  2673507.7 |########################################
  2674417.9 |########################################
  2675328.2 |
  2676238.5 |
  2677148.7 |########################################
  2678059.0 |
  2678969.3 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2662674.6-2696418.8 ns)
  2662674.6 |#############
  2664361.8 |
  2666049.0 |#############
  2667736.2 |
  2669423.4 |
  2671110.6 |########################################
  2672797.8 |
  2674485.1 |
  2676172.3 |
  2677859.5 |
  2679546.7 |
  2681233.9 |
  2682921.1 |
  2684608.3 |
  2686295.5 |
  2687982.7 |
  2689669.9 |
  2691357.1 |
  2693044.3 |
  2694731.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2671016.7-2689693.5 ns)
  2671016.7 |########################################
  2671950.5 |
  2672884.4 |####################
  2673818.2 |
  2674752.1 |####################
  2675685.9 |
  2676619.8 |
  2677553.6 |
  2678487.4 |
  2679421.3 |
  2680355.1 |
  2681289.0 |
  2682222.8 |
  2683156.7 |
  2684090.5 |
  2685024.3 |
  2685958.2 |
  2686892.0 |
  2687825.9 |####################
  2688759.7 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 1070707.9-1079178.1 ns)
  1070707.9 |########################################
  1071131.4 |
  1071554.9 |
  1071978.4 |
  1072401.9 |########################################
  1072825.5 |
  1073249.0 |
  1073672.5 |
  1074096.0 |
  1074519.5 |
  1074943.0 |
  1075366.5 |
  1075790.0 |########################################
  1076213.6 |########################################
  1076637.1 |########################################
  1077060.6 |
  1077484.1 |
  1077907.6 |
  1078331.1 |
  1078754.6 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 1069227.1-1077727.1 ns)
  1069227.1 |########################################
  1069652.1 |
  1070077.1 |
  1070502.1 |
  1070927.1 |
  1071352.1 |
  1071777.1 |
  1072202.1 |
  1072627.1 |
  1073052.1 |####################
  1073477.1 |
  1073902.1 |
  1074327.1 |
  1074752.1 |
  1075177.1 |
  1075602.1 |
  1076027.1 |
  1076452.1 |
  1076877.1 |
  1077302.1 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 1060202.1-1075226.4 ns)
  1060202.1 |########################################
  1060953.3 |
  1061704.5 |
  1062455.8 |
  1063207.0 |
  1063958.2 |
  1064709.4 |########################################
  1065460.6 |
  1066211.8 |########################################
  1066963.1 |
  1067714.3 |
  1068465.5 |
  1069216.7 |
  1069967.9 |
  1070719.1 |########################################
  1071470.4 |
  1072221.6 |
  1072972.8 |########################################
  1073724.0 |
  1074475.2 |
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2669667.9-2985294.6 ns)
  2669667.9 |########################################
  2685449.2 |
  2701230.6 |
  2717011.9 |
  2732793.2 |
  2748574.6 |
  2764355.9 |
  2780137.2 |
  2795918.6 |
  2811699.9 |
  2827481.2 |
  2843262.6 |
  2859043.9 |
  2874825.3 |
  2890606.6 |
  2906387.9 |
  2922169.3 |
  2937950.6 |
  2953731.9 |
  2969513.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=4518.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.6% of algo (FFI overhead may distort results)

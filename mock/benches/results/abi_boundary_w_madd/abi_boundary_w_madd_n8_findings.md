# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_madd_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_madd_scalar_runtime_w has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_madd_null_entry at 3.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_madd_null_entry dominates: 34587% faster than the next best (abi_boundary_w_madd_soa_per_w)

abi_boundary_w_madd_null_entry (3.09 us) leads abi_boundary_w_madd_soa_per_w (1.07 ms) by 34587%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.68 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_runtime_w is an outlier: 867.4x slower than the field

abi_boundary_w_madd_scalar_runtime_w (2.68 ms) is 867.4x the fastest (3.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_soa_dispatch shows alternating (throttle bounce) (autocorr -0.53)

abi_boundary_w_madd_soa_dispatch's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_runtime_w} (34587% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_runtime_w} with a 34587% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 867.4x the fastest

Fastest abi_boundary_w_madd_null_entry (3.09 us) to slowest abi_boundary_w_madd_scalar_runtime_w (2.68 ms): 867.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 3092.5 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 867.43x (fastest 3092.5 ns, slowest 2682514.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5323ns | 5378ns | 5067ns | 5336ns | 5432ns | -99.80% |
| abi_boundary_w_madd_scalar_anchor | 2668911ns | 2668384ns | 2656503ns | 2666125ns | 2679295ns | -0.56% |
| abi_boundary_w_madd_scalar_dispatch | 2680138ns | 2681223ns | 2665058ns | 2679938ns | 2687978ns | -0.14% |
| abi_boundary_w_madd_scalar_per_w | 2675218ns | 2672148ns | 2669827ns | 2671748ns | 2683118ns | -0.33% |
| abi_boundary_w_madd_scalar_runtime_w | 2683968ns | 2684992ns | 2669058ns | 2684762ns | 2690231ns | base |
| abi_boundary_w_madd_soa_dispatch | 1080455ns | 1080508ns | 1076146ns | 1080080ns | 1083172ns | -59.74% |
| abi_boundary_w_madd_soa_per_w | 1075095ns | 1075051ns | 1069394ns | 1073680ns | 1080068ns | -59.94% |
| abi_boundary_w_madd_soa_runtime_w | 1082255ns | 1084579ns | 1068925ns | 1082846ns | 1088034ns | -59.68% |
| abi_boundary_w_madd_zig_runtime_w | 2674683ns | 2676670ns | 2656958ns | 2670856ns | 2689286ns | -0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 3063ns | 2938ns | 3113ns | -99.89% | 0.003 |
| abi_boundary_w_madd_scalar_anchor | 2666404ns | 2654058ns | 2676733ns | -0.56% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2677609ns | 2662717ns | 2685382ns | -0.14% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2672694ns | 2667224ns | 2680709ns | -0.33% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2681471ns | 2666398ns | 2687840ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 1078155ns | 1073922ns | 1080864ns | -59.79% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 1072702ns | 1067070ns | 1077678ns | -60.00% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 1079856ns | 1066587ns | 1085666ns | -59.73% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2672084ns | 2654470ns | 2686664ns | -0.35% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 26508.9 | 3134.5 | 3063.3 | n/a |
| abi_boundary_w_madd_scalar_anchor | 39185.8 | 2669670.1 | 2666403.8 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 37274.5 | 2679154.2 | 2677609.1 | 0 |
| abi_boundary_w_madd_scalar_per_w | 39610.3 | 2674356.9 | 2672693.6 | 0 |
| abi_boundary_w_madd_scalar_runtime_w | 36265.5 | 2682825.5 | 2681470.7 | n/a |
| abi_boundary_w_madd_soa_dispatch | 29995.8 | 1078103.8 | 1078155.1 | n/a |
| abi_boundary_w_madd_soa_per_w | 32746.6 | 1073480.1 | 1072701.9 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 31703.5 | 1080709.2 | 1079855.9 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 179195.2 | 2672374.3 | 2672083.8 | 15 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.003 | 95.0% |
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
| abi_boundary_w_madd_null_entry | 5323ns | 5323ns | -99.80% |
| abi_boundary_w_madd_scalar_anchor | 2668911ns | 2668911ns | -0.56% |
| abi_boundary_w_madd_scalar_dispatch | 2680138ns | 2680138ns | -0.14% |
| abi_boundary_w_madd_scalar_per_w | 2675218ns | 2675218ns | -0.33% |
| abi_boundary_w_madd_scalar_runtime_w | 2683968ns | 2683968ns | base |
| abi_boundary_w_madd_soa_dispatch | 1080455ns | 1080455ns | -59.74% |
| abi_boundary_w_madd_soa_per_w | 1075095ns | 1075095ns | -59.94% |
| abi_boundary_w_madd_soa_runtime_w | 1082255ns | 1082255ns | -59.68% |
| abi_boundary_w_madd_zig_runtime_w | 2674683ns | 2674683ns | -0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2682514ns | base | --- | [2674058, 2687840] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 3092ns | -2679403.1ns (-99.9%) | [-2684769, -2671050]ns | [2985, 3113] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2665772ns | -17573.3ns (-0.7%) | [-27154, -473]ns | [2656706, 2676733] | YES (adj: no) | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2678662ns | no significant difference | [-11985, +2868]ns | [2668783, 2685382] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2669581ns | no significant difference | [-15514, +2663]ns | [2667791, 2680709] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_soa_dispatch | 1078194ns | -1605372.1ns (-59.8%) | [-1610364, -1594211]ns | [1075407, 1080864] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 1072705ns | -1612468.3ns (-60.1%) | [-1616138, -1597700]ns | [1067723, 1077678] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 1082056ns | -1602771.0ns (-59.7%) | [-1611433, -1590641]ns | [1071846, 1085666] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2674039ns | no significant difference | [-28313, +11408]ns | [2655548, 2686664] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2684838ns | -99.9% | -1.1% | -0.4% | +0.1% | -59.9% | -60.2% | -59.9% | -1.1% |
| 2 | 2666398ns | -99.9% | +0.1% | -0.1% | +0.1% | -59.6% | -59.6% | -59.4% | +0.8% |
| 3 | 2681719ns | -99.9% | -0.1% | -0.0% | -0.5% | -59.8% | -59.9% | -60.2% | +0.1% |
| 4 | 2690841ns | -99.9% | -0.6% | -0.5% | -0.6% | -59.9% | -59.9% | -59.7% | -0.2% |
| 5 | 2682883ns | -99.9% | -0.9% | +0.0% | -0.5% | -60.0% | -60.2% | -59.5% | -1.0% |
| 6 | 2682145ns | -99.9% | -0.7% | +0.2% | -0.6% | -59.6% | -60.1% | -59.6% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.147 | ok |
| abi_boundary_w_madd_scalar_anchor | 0.154 | ok |
| abi_boundary_w_madd_scalar_dispatch | 0.133 | ok |
| abi_boundary_w_madd_scalar_per_w | -0.091 | ok |
| abi_boundary_w_madd_scalar_runtime_w | -0.116 | ok |
| abi_boundary_w_madd_soa_dispatch | -0.525 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_per_w | 0.005 | ok |
| abi_boundary_w_madd_soa_runtime_w | -0.183 | ok |
| abi_boundary_w_madd_zig_runtime_w | -0.003 | ok |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 5/6, lost 0/6
- **abi_boundary_w_madd_scalar_dispatch**: won 3/6, lost 1/6
- **abi_boundary_w_madd_scalar_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_zig_runtime_w**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 119585.6ns | 3063.3ns | 3903.8% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8047678.8ns | 2666403.8ns | 301.8% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8076265.7ns | 2677609.1ns | 301.6% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8062922.7ns | 2672693.6ns | 301.7% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8084596.8ns | 2681470.7ns | 301.5% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 3265835.6ns | 1078155.1ns | 302.9% | HIGH |
| abi_boundary_w_madd_soa_per_w | 3255152.1ns | 1072701.9ns | 303.5% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 3271941.4ns | 1079855.9ns | 303.0% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8270208.4ns | 2672083.8ns | 309.5% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 2937.9-3112.9 ns)
   2937.9 |####################
   2946.7 |
   2955.4 |
   2964.2 |
   2972.9 |
   2981.7 |
   2990.4 |
   2999.2 |
   3007.9 |
   3016.7 |
   3025.4 |####################
   3034.2 |
   3042.9 |
   3051.7 |
   3060.4 |
   3069.2 |
   3077.9 |####################
   3086.7 |
   3095.4 |
   3104.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2654057.5-2676732.9 ns)
  2654057.5 |########################################
  2655191.3 |
  2656325.0 |
  2657458.8 |
  2658592.6 |########################################
  2659726.4 |
  2660860.1 |
  2661993.9 |########################################
  2663127.7 |
  2664261.4 |
  2665395.2 |
  2666529.0 |
  2667662.7 |########################################
  2668796.5 |
  2669930.3 |
  2671064.1 |
  2672197.8 |
  2673331.6 |
  2674465.4 |########################################
  2675599.1 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2662716.7-2685382.2 ns)
  2662716.7 |########################################
  2663850.0 |
  2664983.3 |
  2666116.5 |
  2667249.8 |
  2668383.1 |
  2669516.4 |
  2670649.6 |
  2671782.9 |
  2672916.2 |
  2674049.5 |########################################
  2675182.8 |
  2676316.0 |########################################
  2677449.3 |
  2678582.6 |
  2679715.9 |########################################
  2680849.1 |
  2681982.4 |
  2683115.7 |########################################
  2684249.0 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2667223.8-2680708.8 ns)
  2667223.8 |####################
  2667898.0 |####################
  2668572.3 |
  2669246.5 |########################################
  2669920.8 |
  2670595.0 |
  2671269.3 |
  2671943.5 |
  2672617.8 |
  2673292.0 |
  2673966.3 |
  2674640.5 |####################
  2675314.8 |
  2675989.0 |
  2676663.3 |
  2677337.5 |
  2678011.8 |
  2678686.0 |
  2679360.3 |
  2680034.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2666397.9-2687839.5 ns)
  2666397.9 |####################
  2667470.0 |
  2668542.1 |
  2669614.1 |
  2670686.2 |
  2671758.3 |
  2672830.4 |
  2673902.5 |
  2674974.6 |
  2676046.6 |
  2677118.7 |
  2678190.8 |
  2679262.9 |
  2680335.0 |
  2681407.1 |########################################
  2682479.1 |####################
  2683551.2 |
  2684623.3 |####################
  2685695.4 |
  2686767.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 1073921.7-1080864.4 ns)
  1073921.7 |####################
  1074268.8 |
  1074616.0 |
  1074963.1 |
  1075310.2 |
  1075657.4 |
  1076004.5 |
  1076351.6 |
  1076698.8 |####################
  1077045.9 |
  1077393.0 |####################
  1077740.2 |
  1078087.3 |
  1078434.5 |
  1078781.6 |########################################
  1079128.7 |
  1079475.9 |
  1079823.0 |
  1080170.1 |
  1080517.3 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 1067070.0-1077678.3 ns)
  1067070.0 |########################################
  1067600.4 |
  1068130.8 |########################################
  1068661.2 |
  1069191.7 |########################################
  1069722.1 |
  1070252.5 |
  1070782.9 |
  1071313.3 |
  1071843.7 |
  1072374.1 |
  1072904.6 |
  1073435.0 |
  1073965.4 |
  1074495.8 |
  1075026.2 |
  1075556.6 |########################################
  1076087.1 |
  1076617.5 |########################################
  1077147.9 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 1066587.1-1085665.6 ns)
  1066587.1 |########################################
  1067541.0 |
  1068495.0 |
  1069448.9 |
  1070402.8 |
  1071356.7 |
  1072310.7 |
  1073264.6 |
  1074218.5 |
  1075172.4 |
  1076126.4 |
  1077080.3 |########################################
  1078034.2 |
  1078988.1 |
  1079942.1 |
  1080896.0 |########################################
  1081849.9 |########################################
  1082803.8 |
  1083757.8 |
  1084711.7 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2654469.6-2686664.2 ns)
  2654469.6 |########################################
  2656079.3 |########################################
  2657689.1 |
  2659298.8 |
  2660908.5 |
  2662518.2 |########################################
  2664128.0 |
  2665737.7 |
  2667347.4 |
  2668957.2 |
  2670566.9 |
  2672176.6 |
  2673786.4 |
  2675396.1 |
  2677005.8 |
  2678615.6 |
  2680225.3 |
  2681835.0 |
  2683444.7 |########################################
  2685054.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=3865.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.4% of algo (FFI overhead may distort results)

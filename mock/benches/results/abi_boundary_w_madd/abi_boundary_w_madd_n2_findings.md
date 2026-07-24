# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_madd_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_madd_scalar_runtime_w has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_madd_null_entry at 3.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_madd_null_entry dominates: 76186% faster than the next best (abi_boundary_w_madd_scalar_anchor)

abi_boundary_w_madd_null_entry (3.49 us) leads abi_boundary_w_madd_scalar_anchor (2.66 ms) by 76186%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.72 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_runtime_w is an outlier: 780.1x slower than the field

abi_boundary_w_madd_scalar_runtime_w (2.72 ms) is 780.1x the fastest (3.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_runtime_w} (76186% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_runtime_w} with a 76186% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 780.1x the fastest

Fastest abi_boundary_w_madd_null_entry (3.49 us) to slowest abi_boundary_w_madd_scalar_runtime_w (2.72 ms): 780.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 3491.7 ns median (-99.9% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 780.07x (fastest 3491.7 ns, slowest 2723767.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5812ns | 5777ns | 5548ns | 5718ns | 6085ns | -99.79% |
| abi_boundary_w_madd_scalar_anchor | 2667478ns | 2666234ns | 2656833ns | 2664029ns | 2677976ns | -2.16% |
| abi_boundary_w_madd_scalar_dispatch | 2673633ns | 2675364ns | 2655700ns | 2672712ns | 2683981ns | -1.94% |
| abi_boundary_w_madd_scalar_per_w | 2683172ns | 2677102ns | 2667031ns | 2675424ns | 2702866ns | -1.59% |
| abi_boundary_w_madd_scalar_runtime_w | 2726450ns | 2726290ns | 2716716ns | 2723674ns | 2735481ns | base |
| abi_boundary_w_madd_soa_dispatch | 2679482ns | 2676342ns | 2666963ns | 2675619ns | 2691538ns | -1.72% |
| abi_boundary_w_madd_soa_per_w | 2675994ns | 2676246ns | 2670078ns | 2674519ns | 2681165ns | -1.85% |
| abi_boundary_w_madd_soa_runtime_w | 2721345ns | 2722419ns | 2704182ns | 2720218ns | 2731616ns | -0.19% |
| abi_boundary_w_madd_zig_runtime_w | 2708593ns | 2707860ns | 2703156ns | 2707467ns | 2713002ns | -0.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 3512ns | 3359ns | 3682ns | -99.87% | 0.001 |
| abi_boundary_w_madd_scalar_anchor | 2664941ns | 2654412ns | 2675434ns | -2.17% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2671028ns | 2653248ns | 2681460ns | -1.94% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2680655ns | 2664610ns | 2700417ns | -1.59% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2723923ns | 2714139ns | 2733007ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 2676924ns | 2664545ns | 2688997ns | -1.73% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 2673476ns | 2667523ns | 2678804ns | -1.85% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 2718787ns | 2701525ns | 2728987ns | -0.19% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2706015ns | 2700737ns | 2710365ns | -0.66% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 27140.8 | 3573.4 | 3511.8 | n/a |
| abi_boundary_w_madd_scalar_anchor | 39080.7 | 2668073.1 | 2664941.3 | n/a |
| abi_boundary_w_madd_scalar_dispatch | 38195.9 | 2672121.5 | 2671028.2 | n/a |
| abi_boundary_w_madd_scalar_per_w | 39680.2 | 2678583.9 | 2680654.6 | n/a |
| abi_boundary_w_madd_scalar_runtime_w | 39837.2 | 2721204.4 | 2723923.1 | n/a |
| abi_boundary_w_madd_soa_dispatch | 37670.4 | 2676591.4 | 2676923.8 | n/a |
| abi_boundary_w_madd_soa_per_w | 38928.9 | 2674422.2 | 2673476.1 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 37485.4 | 2718317.7 | 2718786.9 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 179189.3 | 2706371.7 | 2706015.0 | 8 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.001 | 96.2% |
| abi_boundary_w_madd_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_dispatch | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_per_w | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_madd_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5812ns | 5812ns | -99.79% |
| abi_boundary_w_madd_scalar_anchor | 2667478ns | 2667478ns | -2.16% |
| abi_boundary_w_madd_scalar_dispatch | 2673633ns | 2673633ns | -1.94% |
| abi_boundary_w_madd_scalar_per_w | 2683172ns | 2683172ns | -1.59% |
| abi_boundary_w_madd_scalar_runtime_w | 2726450ns | 2726450ns | base |
| abi_boundary_w_madd_soa_dispatch | 2679482ns | 2679482ns | -1.72% |
| abi_boundary_w_madd_soa_per_w | 2675994ns | 2675994ns | -1.85% |
| abi_boundary_w_madd_soa_runtime_w | 2721345ns | 2721345ns | -0.19% |
| abi_boundary_w_madd_zig_runtime_w | 2708593ns | 2708593ns | -0.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2723768ns | base | --- | [2714995, 2733007] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 3492ns | -2720348.3ns (-99.9%) | [-2729492, -2711393]ns | [3362, 3682] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2663670ns | -56607.0ns (-2.1%) | [-76673, -43665]ns | [2655719, 2675434] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2672584ns | -55576.0ns (-2.0%) | [-60622, -42486]ns | [2659041, 2681460] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2674530ns | -47977.7ns (-1.8%) | [-57863, -23965]ns | [2667017, 2700417] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_dispatch | 2673681ns | -48311.7ns (-1.8%) | [-55818, -36868]ns | [2668093, 2688997] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 2673670ns | -46835.6ns (-1.7%) | [-61935, -42570]ns | [2667955, 2678804] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 2719905ns | no significant difference | [-25216, +7896]ns | [2707469, 2728987] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2705280ns | -13724.7ns (-0.5%) | [-30089, -9911]ns | [2702401, 2710365] | YES | 0.0357 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2723475ns | -99.9% | -2.5% | -2.2% | -1.8% | -2.2% | -1.6% | +0.0% | -0.5% |
| 2 | 2714139ns | -99.9% | -1.6% | -1.2% | -1.8% | -1.4% | -1.5% | +0.3% | -0.3% |
| 3 | 2724703ns | -99.9% | -2.1% | -1.9% | -0.1% | -1.9% | -1.7% | +0.3% | -0.6% |
| 4 | 2724060ns | -99.9% | -1.7% | -1.9% | -1.6% | -1.3% | -1.8% | -0.4% | -0.7% |
| 5 | 2741310ns | -99.9% | -3.1% | -2.1% | -2.4% | -1.9% | -2.7% | -1.5% | -1.5% |
| 6 | 2715851ns | -99.9% | -2.1% | -2.3% | -1.7% | -1.6% | -1.7% | +0.1% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | -0.342 | moderate- |
| abi_boundary_w_madd_scalar_anchor | -0.220 | moderate- |
| abi_boundary_w_madd_scalar_dispatch | -0.401 | moderate- |
| abi_boundary_w_madd_scalar_per_w | -0.240 | moderate- |
| abi_boundary_w_madd_scalar_runtime_w | -0.304 | moderate- |
| abi_boundary_w_madd_soa_dispatch | 0.099 | ok |
| abi_boundary_w_madd_soa_per_w | 0.241 | moderate+ |
| abi_boundary_w_madd_soa_runtime_w | 0.132 | ok |
| abi_boundary_w_madd_zig_runtime_w | 0.188 | ok |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 2/6, lost 2/6
- **abi_boundary_w_madd_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 120623.7ns | 3511.8ns | 3434.8% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8046473.8ns | 2664941.3ns | 301.9% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8056247.2ns | 2671028.2ns | 301.6% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8080084.0ns | 2680654.6ns | 301.4% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8201536.5ns | 2723923.1ns | 301.1% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 8068857.0ns | 2676923.8ns | 301.4% | HIGH |
| abi_boundary_w_madd_soa_per_w | 8062329.8ns | 2673476.1ns | 301.6% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 8195257.0ns | 2718786.9ns | 301.4% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8369131.0ns | 2706015.0ns | 309.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 3359.2-3682.3 ns)
   3359.2 |########################################
   3375.4 |
   3391.5 |
   3407.7 |
   3423.8 |
   3440.0 |
   3456.1 |
   3472.3 |####################
   3488.4 |####################
   3504.6 |
   3520.8 |
   3536.9 |
   3553.1 |
   3569.2 |
   3585.4 |
   3601.5 |
   3617.7 |
   3633.8 |
   3650.0 |####################
   3666.1 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2654411.7-2675434.2 ns)
  2654411.7 |########################################
  2655462.8 |
  2656514.0 |########################################
  2657565.1 |
  2658616.2 |
  2659667.3 |########################################
  2660718.5 |
  2661769.6 |
  2662820.7 |
  2663871.8 |
  2664923.0 |
  2665974.1 |
  2667025.2 |########################################
  2668076.3 |
  2669127.5 |
  2670178.6 |
  2671229.7 |########################################
  2672280.8 |
  2673332.0 |
  2674383.1 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2653247.5-2681459.5 ns)
  2653247.5 |########################################
  2654658.1 |
  2656068.7 |
  2657479.3 |
  2658889.9 |
  2660300.5 |
  2661711.1 |
  2663121.7 |
  2664532.3 |########################################
  2665942.9 |
  2667353.5 |
  2668764.1 |
  2670174.7 |########################################
  2671585.3 |
  2672995.9 |########################################
  2674406.5 |
  2675817.1 |
  2677227.7 |
  2678638.3 |
  2680048.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2664609.6-2700416.8 ns)
  2664609.6 |####################
  2666400.0 |
  2668190.3 |####################
  2669980.7 |
  2671771.0 |
  2673561.4 |########################################
  2675351.8 |
  2677142.1 |
  2678932.5 |####################
  2680722.9 |
  2682513.2 |
  2684303.6 |
  2686093.9 |
  2687884.3 |
  2689674.7 |
  2691465.0 |
  2693255.4 |
  2695045.8 |
  2696836.1 |
  2698626.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2714138.8-2733006.8 ns)
  2714138.8 |########################################
  2715082.2 |########################################
  2716025.6 |
  2716969.0 |
  2717912.4 |
  2718855.8 |
  2719799.2 |
  2720742.6 |
  2721686.0 |
  2722629.4 |########################################
  2723572.8 |########################################
  2724516.2 |########################################
  2725459.6 |
  2726403.0 |
  2727346.4 |
  2728289.8 |
  2729233.2 |
  2730176.6 |
  2731120.0 |
  2732063.4 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 2664545.0-2688997.1 ns)
  2664545.0 |########################################
  2665767.6 |
  2666990.2 |
  2668212.8 |
  2669435.4 |
  2670658.0 |########################################
  2671880.6 |########################################
  2673103.2 |
  2674325.8 |########################################
  2675548.4 |
  2676771.0 |
  2677993.7 |
  2679216.3 |
  2680438.9 |
  2681661.5 |
  2682884.1 |
  2684106.7 |
  2685329.3 |
  2686551.9 |
  2687774.5 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 2667522.9-2678804.0 ns)
  2667522.9 |########################################
  2668087.0 |########################################
  2668651.0 |
  2669215.1 |
  2669779.1 |
  2670343.2 |
  2670907.2 |
  2671471.3 |
  2672035.3 |
  2672599.4 |
  2673163.4 |########################################
  2673727.5 |########################################
  2674291.5 |
  2674855.6 |
  2675419.6 |
  2675983.7 |
  2676547.7 |
  2677111.8 |
  2677675.8 |
  2678239.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 2701525.0-2728986.8 ns)
  2701525.0 |########################################
  2702898.1 |
  2704271.2 |
  2705644.3 |
  2707017.4 |
  2708390.5 |
  2709763.6 |
  2711136.6 |
  2712509.7 |########################################
  2713882.8 |
  2715255.9 |
  2716629.0 |
  2718002.1 |########################################
  2719375.2 |
  2720748.3 |########################################
  2722121.4 |
  2723494.5 |########################################
  2724867.6 |
  2726240.7 |
  2727613.8 |
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2700737.1-2710364.6 ns)
  2700737.1 |########################################
  2701218.5 |
  2701699.9 |
  2702181.2 |
  2702662.6 |
  2703144.0 |
  2703625.4 |########################################
  2704106.7 |########################################
  2704588.1 |
  2705069.5 |
  2705550.9 |
  2706032.2 |########################################
  2706513.6 |
  2706995.0 |
  2707476.4 |
  2707957.7 |
  2708439.1 |
  2708920.5 |
  2709401.9 |########################################
  2709883.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=3465.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.6% of algo (FFI overhead may distort results)

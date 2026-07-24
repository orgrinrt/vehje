# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_madd_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_madd_scalar_runtime_w has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_madd_null_entry at 2.26 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_madd_null_entry dominates: 47150% faster than the next best (abi_boundary_w_madd_soa_per_w)

abi_boundary_w_madd_null_entry (2.26 us) leads abi_boundary_w_madd_soa_per_w (1.07 ms) by 47150%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.68 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_runtime_w is an outlier: 1184.6x slower than the field

abi_boundary_w_madd_scalar_runtime_w (2.68 ms) is 1184.6x the fastest (2.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_null_entry shows alternating (throttle bounce) (autocorr -0.55)

abi_boundary_w_madd_null_entry's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_runtime_w} (47150% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_runtime_w} with a 47150% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1184.6x the fastest

Fastest abi_boundary_w_madd_null_entry (2.26 us) to slowest abi_boundary_w_madd_scalar_runtime_w (2.68 ms): 1184.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 2264.8 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 1184.56x (fastest 2264.8 ns, slowest 2682794.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 4569ns | 4495ns | 4368ns | 4466ns | 4823ns | -99.83% |
| abi_boundary_w_madd_scalar_anchor | 2667668ns | 2665350ns | 2656652ns | 2664502ns | 2677925ns | -0.68% |
| abi_boundary_w_madd_scalar_dispatch | 2672290ns | 2669889ns | 2659979ns | 2667345ns | 2685865ns | -0.50% |
| abi_boundary_w_madd_scalar_per_w | 2679865ns | 2677022ns | 2669312ns | 2675249ns | 2692067ns | -0.22% |
| abi_boundary_w_madd_scalar_runtime_w | 2685801ns | 2685341ns | 2674585ns | 2684967ns | 2692658ns | base |
| abi_boundary_w_madd_soa_dispatch | 1077117ns | 1079769ns | 1064486ns | 1075474ns | 1085898ns | -59.90% |
| abi_boundary_w_madd_soa_per_w | 1070954ns | 1072437ns | 1059849ns | 1071253ns | 1076058ns | -60.13% |
| abi_boundary_w_madd_soa_runtime_w | 1078895ns | 1080789ns | 1066867ns | 1079970ns | 1083295ns | -59.83% |
| abi_boundary_w_madd_zig_runtime_w | 2677281ns | 2680058ns | 2658371ns | 2677566ns | 2686310ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 2296ns | 2197ns | 2415ns | -99.91% | 0.014 |
| abi_boundary_w_madd_scalar_anchor | 2665183ns | 2654162ns | 2675505ns | -0.67% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2669681ns | 2657435ns | 2683218ns | -0.51% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2677463ns | 2666900ns | 2689565ns | -0.22% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2683255ns | 2672135ns | 2690013ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 1074749ns | 1062108ns | 1083443ns | -59.95% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 1068658ns | 1057632ns | 1073729ns | -60.17% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 1076365ns | 1064498ns | 1080776ns | -59.89% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2674693ns | 2655828ns | 2683732ns | -0.32% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 26514.7 | 2391.6 | 2296.1 | n/a |
| abi_boundary_w_madd_scalar_anchor | 35927.5 | 2668699.5 | 2665182.8 | n/a |
| abi_boundary_w_madd_scalar_dispatch | 38863.9 | 2669758.0 | 2669680.6 | n/a |
| abi_boundary_w_madd_scalar_per_w | 36201.8 | 2681034.6 | 2677462.7 | n/a |
| abi_boundary_w_madd_scalar_runtime_w | 39189.2 | 2684082.8 | 2683254.9 | n/a |
| abi_boundary_w_madd_soa_dispatch | 31275.6 | 1074304.7 | 1074749.1 | n/a |
| abi_boundary_w_madd_soa_per_w | 30630.7 | 1068794.8 | 1068658.3 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 32215.6 | 1076470.6 | 1076365.2 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 180247.9 | 2678478.1 | 2674692.9 | 16 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.014 | 97.0% |
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
| abi_boundary_w_madd_null_entry | 4569ns | 4569ns | -99.83% |
| abi_boundary_w_madd_scalar_anchor | 2667668ns | 2667668ns | -0.68% |
| abi_boundary_w_madd_scalar_dispatch | 2672290ns | 2672290ns | -0.50% |
| abi_boundary_w_madd_scalar_per_w | 2679865ns | 2679865ns | -0.22% |
| abi_boundary_w_madd_scalar_runtime_w | 2685801ns | 2685801ns | base |
| abi_boundary_w_madd_soa_dispatch | 1077117ns | 1077117ns | -59.90% |
| abi_boundary_w_madd_soa_per_w | 1070954ns | 1070954ns | -60.13% |
| abi_boundary_w_madd_soa_runtime_w | 1078895ns | 1078895ns | -59.83% |
| abi_boundary_w_madd_zig_runtime_w | 2677281ns | 2677281ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2682795ns | base | --- | [2676956, 2690013] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 2265ns | -2680485.0ns (-99.9%) | [-2687748, -2674643]ns | [2209, 2415] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2662815ns | -19208.6ns (-0.7%) | [-29686, -5322]ns | [2657228, 2675505] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2667187ns | -15482.2ns (-0.6%) | [-22587, -2654]ns | [2658637, 2683218] | YES (adj: no) | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2674658ns | no significant difference | [-18201, +5571]ns | [2668165, 2689565] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_madd_soa_dispatch | 1077450ns | -1606078.4ns (-59.9%) | [-1622294, -1597145]ns | [1063354, 1083443] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 1070108ns | -1612806.6ns (-60.1%) | [-1621917, -1609066]ns | [1062138, 1073729] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 1078150ns | -1606665.9ns (-59.9%) | [-1616768, -1597235]ns | [1070169, 1080776] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2677455ns | no significant difference | [-23550, +6776]ns | [2662892, 2683732] | no | 0.2500 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2689518ns | -99.9% | -0.4% | -0.5% | +0.1% | -60.5% | -60.3% | -59.9% | -1.3% |
| 2 | 2681778ns | -99.9% | -0.6% | -0.8% | -0.4% | -60.3% | -60.1% | -59.7% | -0.0% |
| 3 | 2690508ns | -99.9% | -1.4% | -0.6% | -0.8% | -59.8% | -60.2% | -60.0% | -0.5% |
| 4 | 2682224ns | -99.9% | -0.8% | +0.3% | -0.6% | -59.7% | -60.0% | -59.7% | -0.2% |
| 5 | 2683365ns | -99.9% | -0.9% | -0.9% | +0.1% | -59.5% | -59.9% | -60.3% | -0.5% |
| 6 | 2672135ns | -99.9% | -0.0% | -0.6% | +0.3% | -59.8% | -60.4% | -59.7% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | -0.548 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_scalar_anchor | 0.117 | ok |
| abi_boundary_w_madd_scalar_dispatch | -0.121 | ok |
| abi_boundary_w_madd_scalar_per_w | -0.083 | ok |
| abi_boundary_w_madd_scalar_runtime_w | -0.132 | ok |
| abi_boundary_w_madd_soa_dispatch | 0.358 | moderate+ |
| abi_boundary_w_madd_soa_per_w | -0.282 | moderate- |
| abi_boundary_w_madd_soa_runtime_w | -0.267 | moderate- |
| abi_boundary_w_madd_zig_runtime_w | -0.266 | moderate- |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 5/6, lost 0/6
- **abi_boundary_w_madd_scalar_dispatch**: won 5/6, lost 1/6
- **abi_boundary_w_madd_scalar_per_w**: won 3/6, lost 2/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_zig_runtime_w**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 116029.0ns | 2296.1ns | 5053.4% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8042058.1ns | 2665182.8ns | 301.7% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8053557.8ns | 2669680.6ns | 301.7% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8079731.0ns | 2677462.7ns | 301.8% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8094199.6ns | 2683254.9ns | 301.7% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 3257211.6ns | 1074749.1ns | 303.1% | HIGH |
| abi_boundary_w_madd_soa_per_w | 3237728.6ns | 1068658.3ns | 303.0% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 3262760.7ns | 1076365.2ns | 303.1% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8284423.6ns | 2674692.9ns | 309.7% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 2197.1-2414.8 ns)
   2197.1 |########################################
   2208.0 |
   2218.9 |########################################
   2229.8 |
   2240.6 |
   2251.5 |########################################
   2262.4 |########################################
   2273.3 |
   2284.2 |
   2295.1 |
   2305.9 |
   2316.8 |
   2327.7 |
   2338.6 |
   2349.5 |
   2360.4 |
   2371.3 |
   2382.1 |
   2393.0 |
   2403.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2654161.7-2675505.2 ns)
  2654161.7 |####################
  2655228.9 |
  2656296.1 |
  2657363.2 |
  2658430.4 |
  2659497.6 |########################################
  2660564.8 |
  2661631.9 |
  2662699.1 |
  2663766.3 |
  2664833.5 |####################
  2665900.6 |
  2666967.8 |
  2668035.0 |
  2669102.2 |
  2670169.3 |####################
  2671236.5 |
  2672303.7 |
  2673370.9 |
  2674438.0 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2657434.6-2683217.7 ns)
  2657434.6 |########################################
  2658723.8 |########################################
  2660012.9 |########################################
  2661302.1 |
  2662591.2 |
  2663880.4 |
  2665169.5 |
  2666458.7 |
  2667747.8 |
  2669037.0 |
  2670326.2 |
  2671615.3 |
  2672904.5 |
  2674193.6 |########################################
  2675482.8 |
  2676771.9 |########################################
  2678061.1 |
  2679350.2 |
  2680639.4 |
  2681928.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2666900.0-2689565.0 ns)
  2666900.0 |####################
  2668033.2 |
  2669166.5 |########################################
  2670299.7 |
  2671433.0 |
  2672566.2 |
  2673699.5 |
  2674832.7 |
  2675966.0 |
  2677099.2 |
  2678232.5 |
  2679365.7 |####################
  2680499.0 |
  2681632.2 |
  2682765.5 |
  2683898.7 |
  2685032.0 |####################
  2686165.2 |
  2687298.5 |
  2688431.7 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2672135.4-2690013.3 ns)
  2672135.4 |########################################
  2673029.3 |
  2673923.2 |
  2674817.1 |
  2675711.0 |
  2676604.9 |
  2677498.8 |
  2678392.7 |
  2679286.6 |
  2680180.5 |
  2681074.3 |########################################
  2681968.2 |########################################
  2682862.1 |########################################
  2683756.0 |
  2684649.9 |
  2685543.8 |
  2686437.7 |
  2687331.6 |
  2688225.5 |
  2689119.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 1062107.5-1083443.1 ns)
  1062107.5 |########################################
  1063174.3 |
  1064241.1 |########################################
  1065307.8 |
  1066374.6 |
  1067441.4 |
  1068508.2 |
  1069575.0 |
  1070641.8 |
  1071708.5 |
  1072775.3 |
  1073842.1 |########################################
  1074908.9 |
  1075975.7 |
  1077042.5 |
  1078109.2 |
  1079176.0 |########################################
  1080242.8 |########################################
  1081309.6 |
  1082376.4 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 1057631.7-1073728.8 ns)
  1057631.7 |########################################
  1058436.6 |
  1059241.4 |
  1060046.3 |
  1060851.1 |
  1061656.0 |
  1062460.8 |
  1063265.7 |
  1064070.5 |
  1064875.4 |
  1065680.2 |
  1066485.1 |########################################
  1067289.9 |
  1068094.8 |
  1068899.6 |########################################
  1069704.5 |
  1070509.3 |########################################
  1071314.2 |########################################
  1072119.0 |
  1072923.9 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 1064497.9-1080776.2 ns)
  1064497.9 |########################################
  1065311.8 |
  1066125.7 |
  1066939.7 |
  1067753.6 |
  1068567.5 |
  1069381.4 |
  1070195.3 |
  1071009.2 |
  1071823.2 |
  1072637.1 |
  1073451.0 |
  1074264.9 |
  1075078.8 |########################################
  1075892.7 |
  1076706.7 |
  1077520.6 |########################################
  1078334.5 |########################################
  1079148.4 |########################################
  1079962.3 |
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2655827.5-2683732.3 ns)
  2655827.5 |########################################
  2657222.7 |
  2658618.0 |
  2660013.2 |
  2661408.5 |
  2662803.7 |
  2664198.9 |
  2665594.2 |
  2666989.4 |
  2668384.7 |
  2669779.9 |########################################
  2671175.1 |
  2672570.4 |
  2673965.6 |
  2675360.9 |########################################
  2676756.1 |
  2678151.3 |########################################
  2679546.6 |########################################
  2680941.8 |
  2682337.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=5102.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.6% of algo (FFI overhead may distort results)

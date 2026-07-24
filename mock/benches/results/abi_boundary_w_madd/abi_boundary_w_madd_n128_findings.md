# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_madd_null_entry dominates: 38102% faster than the next best (abi_boundary_w_madd_soa_dispatch)

abi_boundary_w_madd_null_entry (2.81 us) leads abi_boundary_w_madd_soa_dispatch (1.07 ms) by 38102%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.67 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_per_w is an outlier: 955.7x slower than the field

abi_boundary_w_madd_scalar_per_w (2.68 ms) is 955.7x the fastest (2.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_soa_per_w shows alternating (throttle bounce) (autocorr -0.58)

abi_boundary_w_madd_soa_per_w's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_scalar_per_w} (38102% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_scalar_per_w} with a 38102% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 955.7x the fastest

Fastest abi_boundary_w_madd_null_entry (2.81 us) to slowest abi_boundary_w_madd_scalar_per_w (2.68 ms): 955.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 2805.2 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 955.74x (fastest 2805.2 ns, slowest 2681085.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5126ns | 5145ns | 4884ns | 5110ns | 5273ns | -99.81% |
| abi_boundary_w_madd_scalar_anchor | 2666927ns | 2667424ns | 2656593ns | 2666628ns | 2672544ns | -0.56% |
| abi_boundary_w_madd_scalar_dispatch | 2671218ns | 2671840ns | 2654161ns | 2670985ns | 2680097ns | -0.40% |
| abi_boundary_w_madd_scalar_per_w | 2681661ns | 2683635ns | 2663212ns | 2680745ns | 2692258ns | -0.01% |
| abi_boundary_w_madd_scalar_runtime_w | 2681952ns | 2677318ns | 2673822ns | 2676211ns | 2694628ns | base |
| abi_boundary_w_madd_soa_dispatch | 1074808ns | 1073994ns | 1067469ns | 1072068ns | 1082587ns | -59.92% |
| abi_boundary_w_madd_soa_per_w | 1077415ns | 1076865ns | 1067762ns | 1076022ns | 1084330ns | -59.83% |
| abi_boundary_w_madd_soa_runtime_w | 1074098ns | 1077083ns | 1064341ns | 1073522ns | 1079840ns | -59.95% |
| abi_boundary_w_madd_zig_runtime_w | 2668266ns | 2668309ns | 2660202ns | 2667013ns | 2674178ns | -0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 2802ns | 2691ns | 2874ns | -99.90% | 0.046 |
| abi_boundary_w_madd_scalar_anchor | 2664442ns | 2654312ns | 2669985ns | -0.56% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2668612ns | 2651638ns | 2677460ns | -0.40% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2679113ns | 2660815ns | 2689633ns | -0.01% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2679447ns | 2671297ns | 2692118ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 1072447ns | 1065115ns | 1080151ns | -59.98% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 1075007ns | 1065343ns | 1081804ns | -59.88% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 1071742ns | 1062041ns | 1077523ns | -60.00% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2665694ns | 2657629ns | 2671523ns | -0.51% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 27044.1 | 2820.2 | 2802.1 | n/a |
| abi_boundary_w_madd_scalar_anchor | 39331.8 | 2668694.6 | 2664442.1 | n/a |
| abi_boundary_w_madd_scalar_dispatch | 38085.2 | 2670247.8 | 2668611.5 | n/a |
| abi_boundary_w_madd_scalar_per_w | 39984.5 | 2682296.4 | 2679112.8 | n/a |
| abi_boundary_w_madd_scalar_runtime_w | 40021.9 | 2679125.3 | 2679447.1 | n/a |
| abi_boundary_w_madd_soa_dispatch | 31297.1 | 1072936.8 | 1072447.0 | n/a |
| abi_boundary_w_madd_soa_per_w | 31920.7 | 1074616.0 | 1075007.4 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 30171.9 | 1072240.0 | 1071742.4 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 182517.5 | 2669182.6 | 2665694.2 | 10 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.046 | 95.9% |
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
| abi_boundary_w_madd_null_entry | 5126ns | 5126ns | -99.81% |
| abi_boundary_w_madd_scalar_anchor | 2666927ns | 2666927ns | -0.56% |
| abi_boundary_w_madd_scalar_dispatch | 2671218ns | 2671218ns | -0.40% |
| abi_boundary_w_madd_scalar_per_w | 2681661ns | 2681661ns | -0.01% |
| abi_boundary_w_madd_scalar_runtime_w | 2681952ns | 2681952ns | base |
| abi_boundary_w_madd_soa_dispatch | 1074808ns | 1074808ns | -59.92% |
| abi_boundary_w_madd_soa_per_w | 1077415ns | 1077415ns | -59.83% |
| abi_boundary_w_madd_soa_runtime_w | 1074098ns | 1074098ns | -59.95% |
| abi_boundary_w_madd_zig_runtime_w | 2668266ns | 2668266ns | -0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2674744ns | base | --- | [2671479, 2692118] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 2805ns | -2671970.6ns (-99.9%) | [-2689257, -2668707]ns | [2727, 2874] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2664908ns | -14559.8ns (-0.5%) | [-28590, -1865]ns | [2658434, 2669985] | YES (adj: no) | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2669238ns | no significant difference | [-24407, +2999]ns | [2659137, 2677460] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2681086ns | no significant difference | [-17427, +17463]ns | [2666620, 2689633] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_madd_soa_dispatch | 1071654ns | -1607127.7ns (-60.1%) | [-1620121, -1593752]ns | [1065536, 1080151] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_per_w | 1074503ns | -1604560.8ns (-60.0%) | [-1611147, -1597611]ns | [1068715, 1081804] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 1074663ns | -1600081.4ns (-59.8%) | [-1629077, -1593956]ns | [1063041, 1077523] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2665773ns | -8604.2ns (-0.3%) | [-29433, -3221]ns | [2659786, 2671523] | YES (adj: no) | 0.2500 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2693228ns | -99.9% | -1.1% | -0.9% | -0.2% | -60.5% | -60.0% | -60.6% | -0.9% |
| 2 | 2672402ns | -99.9% | -0.2% | +0.3% | +0.2% | -59.8% | -59.7% | -59.8% | +0.0% |
| 3 | 2671661ns | -99.9% | -0.6% | -0.1% | +0.8% | -59.5% | -60.1% | -59.7% | -0.4% |
| 4 | 2691008ns | -99.9% | -1.0% | -0.7% | -0.7% | -59.9% | -59.6% | -60.5% | -1.2% |
| 5 | 2677086ns | -99.9% | -0.4% | -1.0% | -0.6% | -60.1% | -59.9% | -59.9% | -0.3% |
| 6 | 2671297ns | -99.9% | +0.0% | -0.2% | +0.5% | -60.1% | -59.9% | -59.6% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.075 | ok |
| abi_boundary_w_madd_scalar_anchor | -0.210 | moderate- |
| abi_boundary_w_madd_scalar_dispatch | -0.068 | ok |
| abi_boundary_w_madd_scalar_per_w | -0.188 | ok |
| abi_boundary_w_madd_scalar_runtime_w | -0.277 | moderate- |
| abi_boundary_w_madd_soa_dispatch | 0.256 | moderate+ |
| abi_boundary_w_madd_soa_per_w | -0.576 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_runtime_w | -0.243 | moderate- |
| abi_boundary_w_madd_zig_runtime_w | -0.157 | ok |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 5/6, lost 0/6
- **abi_boundary_w_madd_scalar_dispatch**: won 5/6, lost 1/6
- **abi_boundary_w_madd_scalar_per_w**: won 3/6, lost 3/6
- **abi_boundary_w_madd_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_zig_runtime_w**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 120174.9ns | 2802.1ns | 4288.7% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8042963.8ns | 2664442.1ns | 301.9% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8049352.4ns | 2668611.5ns | 301.6% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8087662.3ns | 2679112.8ns | 301.9% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8076670.6ns | 2679447.1ns | 301.4% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 3251527.1ns | 1072447.0ns | 303.2% | HIGH |
| abi_boundary_w_madd_soa_per_w | 3258574.1ns | 1075007.4ns | 303.1% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 3249225.6ns | 1071742.4ns | 303.2% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8263849.9ns | 2665694.2ns | 310.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 2690.8-2873.9 ns)
   2690.8 |########################################
   2700.0 |
   2709.1 |
   2718.3 |
   2727.4 |
   2736.6 |
   2745.7 |
   2754.9 |########################################
   2764.1 |
   2773.2 |
   2782.4 |########################################
   2791.5 |
   2800.7 |
   2809.8 |
   2819.0 |########################################
   2828.2 |
   2837.3 |
   2846.5 |########################################
   2855.6 |
   2864.8 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2654311.7-2669985.0 ns)
  2654311.7 |########################################
  2655095.4 |
  2655879.0 |
  2656662.7 |
  2657446.4 |
  2658230.0 |
  2659013.7 |
  2659797.3 |
  2660581.0 |
  2661364.7 |
  2662148.3 |########################################
  2662932.0 |
  2663715.7 |
  2664499.3 |########################################
  2665283.0 |########################################
  2666066.6 |
  2666850.3 |
  2667634.0 |########################################
  2668417.6 |
  2669201.3 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2651637.9-2677459.6 ns)
  2651637.9 |########################################
  2652929.0 |
  2654220.1 |
  2655511.2 |
  2656802.2 |
  2658093.3 |
  2659384.4 |
  2660675.5 |
  2661966.6 |
  2663257.7 |
  2664548.8 |
  2665839.8 |########################################
  2667130.9 |
  2668422.0 |########################################
  2669713.1 |########################################
  2671004.2 |
  2672295.3 |########################################
  2673586.3 |
  2674877.4 |
  2676168.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2660815.0-2689632.7 ns)
  2660815.0 |########################################
  2662255.9 |
  2663696.8 |
  2665137.7 |
  2666578.5 |
  2668019.4 |
  2669460.3 |
  2670901.2 |
  2672342.1 |########################################
  2673783.0 |
  2675223.9 |
  2676664.7 |########################################
  2678105.6 |
  2679546.5 |
  2680987.4 |
  2682428.3 |
  2683869.2 |########################################
  2685310.0 |########################################
  2686750.9 |
  2688191.8 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2671297.1-2692118.1 ns)
  2671297.1 |########################################
  2672338.1 |####################
  2673379.2 |
  2674420.2 |
  2675461.3 |
  2676502.4 |####################
  2677543.4 |
  2678584.4 |
  2679625.5 |
  2680666.5 |
  2681707.6 |
  2682748.6 |
  2683789.7 |
  2684830.8 |
  2685871.8 |
  2686912.8 |
  2687953.9 |
  2688994.9 |
  2690036.0 |####################
  2691077.0 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 1065115.0-1080151.0 ns)
  1065115.0 |########################################
  1065866.8 |########################################
  1066618.6 |
  1067370.4 |
  1068122.2 |########################################
  1068874.0 |
  1069625.8 |
  1070377.6 |
  1071129.4 |
  1071881.2 |
  1072633.0 |
  1073384.8 |
  1074136.6 |
  1074888.4 |########################################
  1075640.2 |
  1076392.0 |
  1077143.8 |
  1077895.6 |
  1078647.4 |########################################
  1079399.2 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 1065342.9-1081804.4 ns)
  1065342.9 |####################
  1066166.0 |
  1066989.0 |
  1067812.1 |
  1068635.2 |
  1069458.3 |
  1070281.3 |
  1071104.4 |
  1071927.5 |########################################
  1072750.6 |
  1073573.6 |
  1074396.7 |
  1075219.8 |
  1076042.9 |####################
  1076865.9 |####################
  1077689.0 |
  1078512.1 |
  1079335.2 |
  1080158.2 |
  1080981.3 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 1062041.2-1077522.9 ns)
  1062041.2 |########################################
  1062815.3 |
  1063589.4 |########################################
  1064363.5 |
  1065137.6 |
  1065911.6 |
  1066685.7 |
  1067459.8 |
  1068233.9 |
  1069008.0 |
  1069782.1 |
  1070556.2 |
  1071330.2 |
  1072104.3 |
  1072878.4 |
  1073652.5 |########################################
  1074426.6 |########################################
  1075200.7 |
  1075974.8 |
  1076748.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2657629.2-2671523.1 ns)
  2657629.2 |########################################
  2658323.9 |
  2659018.6 |
  2659713.3 |
  2660408.0 |
  2661102.7 |
  2661797.4 |########################################
  2662492.1 |
  2663186.8 |########################################
  2663881.5 |
  2664576.2 |
  2665270.9 |
  2665965.6 |
  2666660.3 |
  2667355.0 |########################################
  2668049.7 |
  2668744.4 |
  2669439.1 |########################################
  2670133.8 |
  2670828.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=4298.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.9% of algo (FFI overhead may distort results)

# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 107592% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (2.51 us) leads abi_zig_entry_madd_zig_anchor (2.70 ms) by 107592%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.71 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 1297.4x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.25 ms) is 1297.4x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (107592% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 107592% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1297.4x the fastest

Fastest abi_zig_entry_madd_zig_null (2.51 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.25 ms): 1297.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 2507.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1297.39x (fastest 2507.1 ns, slowest 3252617.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2771791ns | 2703105ns | 2694940ns | 2702619ns | 2913974ns | +1.40% |
| abi_zig_entry_madd_zig_dispatch | 2800266ns | 2719170ns | 2702658ns | 2716404ns | 2974862ns | +2.44% |
| abi_zig_entry_madd_zig_null | 4980ns | 4859ns | 4791ns | 4845ns | 5277ns | -99.82% |
| abi_zig_entry_madd_zig_per_w_set | 2725667ns | 2718331ns | 2711927ns | 2716685ns | 2746011ns | -0.29% |
| abi_zig_entry_madd_zig_runtime_w | 2733630ns | 2712854ns | 2708650ns | 2712375ns | 2778002ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3279314ns | 3194351ns | 3133548ns | 3192060ns | 3483079ns | +19.96% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3384511ns | 3256543ns | 3197687ns | 3237689ns | 3698156ns | +23.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2768227ns | 2691805ns | 2909712ns | +1.39% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2796850ns | 2699973ns | 2970590ns | +2.44% | 0.000 |
| abi_zig_entry_madd_zig_null | 2591ns | 2495ns | 2768ns | -99.91% | 0.025 |
| abi_zig_entry_madd_zig_per_w_set | 2722358ns | 2708650ns | 2742438ns | -0.29% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2730272ns | 2705538ns | 2774370ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3275660ns | 3129902ns | 3478688ns | +19.98% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3380635ns | 3194128ns | 3693878ns | +23.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 250068.5 | 2881038.3 | 2768227.0 | n/a |
| abi_zig_entry_madd_zig_dispatch | 231867.5 | 2783743.3 | 2796849.6 | 0 |
| abi_zig_entry_madd_zig_null | 168909.9 | 2814.1 | 2590.5 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 223985.5 | 2721339.7 | 2722357.5 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 233267.0 | 2733267.1 | 2730271.6 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 256690.8 | 3291329.6 | 3275659.7 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 278353.4 | 3307697.9 | 3380635.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.026 | 99.5% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2771791ns | 2771791ns | +1.40% |
| abi_zig_entry_madd_zig_dispatch | 2800266ns | 2800266ns | +2.44% |
| abi_zig_entry_madd_zig_null | 4980ns | 4980ns | -99.82% |
| abi_zig_entry_madd_zig_per_w_set | 2725667ns | 2725667ns | -0.29% |
| abi_zig_entry_madd_zig_runtime_w | 2733630ns | 2733630ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3279314ns | 3279314ns | +19.96% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3384511ns | 3384511ns | +23.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2709638ns | base | --- | [2706806, 2774370] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2699885ns | no significant difference | [-19759, +146213]ns | [2695083, 2909712] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2715994ns | no significant difference | [-7272, +204256]ns | [2703965, 2970590] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_madd_zig_null | 2507ns | -2707078.1ns (-99.9%) | [-2771659, -2704306]ns | [2497, 2768] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2715248ns | no significant difference | [-51620, +23872]ns | [2709386, 2742438] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3191266ns | +482168.6ns (+17.8%) | [+444298, +709698]ns | [3157025, 3478688] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3252617ns | +545811.1ns (+20.1%) | [+479179, +926100]ns | [3195410, 3693878] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2827130ns | -0.5% | +13.9% | -99.9% | -3.7% | +32.7% | +22.9% |
| 2 | 2708425ns | +11.0% | +0.3% | -99.9% | +0.1% | +15.6% | +44.5% |
| 3 | 2705538ns | -0.2% | +0.5% | -99.9% | +0.1% | +17.7% | +18.3% |
| 4 | 2710851ns | -0.4% | -0.1% | -99.9% | +0.2% | +18.3% | +17.9% |
| 5 | 2721611ns | -0.9% | -0.2% | -99.9% | +1.5% | +17.2% | +17.4% |
| 6 | 2708075ns | -0.6% | -0.3% | -99.9% | +0.2% | +17.9% | +22.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.117 | ok |
| abi_zig_entry_madd_zig_dispatch | -0.027 | ok |
| abi_zig_entry_madd_zig_null | 0.199 | ok |
| abi_zig_entry_madd_zig_per_w_set | -0.163 | ok |
| abi_zig_entry_madd_zig_runtime_w | -0.064 | ok |
| abi_zig_entry_madd_zig_tail_dispatch | -0.133 | ok |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.090 | ok |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_madd_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 1/6, lost 4/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8736943.8ns | 2768227.0ns | 315.6% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8667981.2ns | 2796849.6ns | 309.9% | HIGH |
| abi_zig_entry_madd_zig_null | 318528.0ns | 2590.5ns | 12295.8% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8470139.2ns | 2722357.5ns | 311.1% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8509168.7ns | 2730271.6ns | 311.7% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 10252391.8ns | 3275659.7ns | 313.0% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 10327832.8ns | 3380635.1ns | 305.5% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2691805.4-2909712.3 ns)
  2691805.4 |########################################
  2702700.7 |
  2713596.1 |
  2724491.4 |
  2735386.8 |
  2746282.1 |
  2757177.5 |
  2768072.8 |
  2778968.2 |
  2789863.5 |
  2800758.8 |
  2811654.2 |##########
  2822549.5 |
  2833444.9 |
  2844340.2 |
  2855235.6 |
  2866130.9 |
  2877026.3 |
  2887921.6 |
  2898817.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2699973.3-2970590.2 ns)
  2699973.3 |##########################
  2713504.1 |########################################
  2727035.0 |
  2740565.8 |
  2754096.7 |
  2767627.5 |
  2781158.4 |
  2794689.2 |
  2808220.1 |
  2821750.9 |
  2835281.8 |
  2848812.6 |
  2862343.4 |
  2875874.3 |
  2889405.1 |
  2902936.0 |
  2916466.8 |
  2929997.7 |
  2943528.5 |
  2957059.4 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 2495.0-2767.9 ns)
   2495.0 |########################################
   2508.6 |
   2522.3 |
   2535.9 |
   2549.6 |
   2563.2 |
   2576.9 |
   2590.5 |
   2604.2 |##########
   2617.8 |
   2631.5 |
   2645.1 |
   2658.8 |
   2672.4 |
   2686.1 |
   2699.7 |
   2713.4 |
   2727.0 |
   2740.7 |
   2754.3 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2708650.4-2742438.4 ns)
  2708650.4 |########################################
  2710339.8 |
  2712029.2 |####################
  2713718.6 |
  2715408.0 |
  2717097.4 |####################
  2718786.8 |
  2720476.2 |
  2722165.6 |####################
  2723855.0 |
  2725544.4 |
  2727233.8 |
  2728923.2 |
  2730612.6 |
  2732302.0 |
  2733991.4 |
  2735680.8 |
  2737370.2 |
  2739059.6 |
  2740749.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2705537.9-2774370.4 ns)
  2705537.9 |########################################
  2708979.5 |#############
  2712421.1 |
  2715862.8 |
  2719304.4 |#############
  2722746.0 |
  2726187.6 |
  2729629.3 |
  2733070.9 |
  2736512.5 |
  2739954.1 |
  2743395.8 |
  2746837.4 |
  2750279.0 |
  2753720.6 |
  2757162.3 |
  2760603.9 |
  2764045.5 |
  2767487.1 |
  2770928.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3129902.1-3478688.1 ns)
  3129902.1 |#############
  3147341.4 |
  3164780.7 |
  3182220.0 |########################################
  3199659.3 |#############
  3217098.6 |
  3234537.9 |
  3251977.2 |
  3269416.5 |
  3286855.8 |
  3304295.1 |
  3321734.4 |
  3339173.7 |
  3356613.0 |
  3374052.3 |
  3391491.6 |
  3408930.9 |
  3426370.2 |
  3443809.5 |
  3461248.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3194127.5-3693877.9 ns)
  3194127.5 |########################################
  3219115.0 |
  3244102.5 |
  3269090.1 |
  3294077.6 |#############
  3319065.1 |
  3344052.6 |
  3369040.1 |
  3394027.7 |
  3419015.2 |
  3444002.7 |
  3468990.2 |#############
  3493977.7 |
  3518965.3 |
  3543952.8 |
  3568940.3 |
  3593927.8 |
  3618915.3 |
  3643902.9 |
  3668890.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=311.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=311.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=12380.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=311.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=309.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=308.5% of algo (FFI overhead may distort results)

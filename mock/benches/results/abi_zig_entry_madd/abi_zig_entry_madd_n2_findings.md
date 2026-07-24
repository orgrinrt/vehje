# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 77572% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (3.46 us) leads abi_zig_entry_madd_zig_anchor (2.69 ms) by 77572%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.72 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 918.8x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.18 ms) is 918.8x the fastest (3.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_madd_zig_per_w_set shows alternating (throttle bounce) (autocorr -0.55)

abi_zig_entry_madd_zig_per_w_set's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (77572% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 77572% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 918.8x the fastest

Fastest abi_zig_entry_madd_zig_null (3.46 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.18 ms): 918.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 3462.5 ns median (-99.9% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 918.82x (fastest 3462.5 ns, slowest 3181401.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2692390ns | 2692387ns | 2681949ns | 2691176ns | 2699432ns | -1.21% |
| abi_zig_entry_madd_zig_dispatch | 2700110ns | 2701129ns | 2689618ns | 2698339ns | 2708012ns | -0.92% |
| abi_zig_entry_madd_zig_null | 5798ns | 5784ns | 5640ns | 5765ns | 5926ns | -99.79% |
| abi_zig_entry_madd_zig_per_w_set | 2698476ns | 2696038ns | 2686983ns | 2694207ns | 2710626ns | -0.98% |
| abi_zig_entry_madd_zig_runtime_w | 2725318ns | 2725749ns | 2711766ns | 2725450ns | 2731896ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3079986ns | 3068175ns | 3064798ns | 3067439ns | 3106399ns | +13.01% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3201817ns | 3184467ns | 3180624ns | 3183859ns | 3239351ns | +17.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2689400ns | 2679004ns | 2696384ns | -1.21% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2697104ns | 2686560ns | 2704909ns | -0.93% | 0.000 |
| abi_zig_entry_madd_zig_null | 3456ns | 3370ns | 3511ns | -99.87% | 0.001 |
| abi_zig_entry_madd_zig_per_w_set | 2695504ns | 2683994ns | 2707600ns | -0.99% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2722342ns | 2708812ns | 2728872ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3076943ns | 3061922ns | 3103080ns | +13.03% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3198701ns | 3177685ns | 3236029ns | +17.50% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 201522.1 | 2690341.8 | 2689399.6 | 0 |
| abi_zig_entry_madd_zig_dispatch | 199930.9 | 2697660.2 | 2697104.3 | n/a |
| abi_zig_entry_madd_zig_null | 162282.0 | 3623.3 | 3455.8 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 198792.7 | 2691884.5 | 2695504.4 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 201434.7 | 2721583.9 | 2722341.5 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 206709.4 | 3111376.5 | 3076943.1 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 215146.5 | 3197887.5 | 3198700.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.001 | 97.3% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2692390ns | 2692390ns | -1.21% |
| abi_zig_entry_madd_zig_dispatch | 2700110ns | 2700110ns | -0.92% |
| abi_zig_entry_madd_zig_null | 5798ns | 5798ns | -99.79% |
| abi_zig_entry_madd_zig_per_w_set | 2698476ns | 2698476ns | -0.98% |
| abi_zig_entry_madd_zig_runtime_w | 2725318ns | 2725318ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3079986ns | 3079986ns | +13.01% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3201817ns | 3201817ns | +17.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2722886ns | base | --- | [2715267, 2728872] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2689405ns | -33284.9ns (-1.2%) | [-39469, -26071]ns | [2682410, 2696384] | YES | 0.0313 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2698222ns | -26755.6ns (-1.0%) | [-34708, -14248]ns | [2688182, 2704909] | YES | 0.0313 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_null | 3462ns | -2719443.3ns (-99.9%) | [-2725430, -2711784]ns | [3394, 3511] | YES | 0.0313 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2693095ns | -28187.9ns (-1.0%) | [-42277, -10046]ns | [2685818, 2707600] | YES | 0.0313 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3065311ns | +341942.4ns (+12.6%) | [+338030, +383832]ns | [3062438, 3103080] | YES | 0.0313 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3181401ns | +458400.2ns (+16.8%) | [+450888, +519789]ns | [3178672, 3236029] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2722113ns | -0.8% | -1.2% | -99.9% | -1.3% | +12.6% | +16.7% |
| 2 | 2723668ns | -1.4% | -1.4% | -99.9% | -0.2% | +15.1% | +20.6% |
| 3 | 2734077ns | -1.5% | -1.0% | -99.9% | -1.8% | +12.4% | +16.3% |
| 4 | 2721721ns | -1.1% | -0.9% | -99.9% | -1.1% | +12.5% | +16.9% |
| 5 | 2708812ns | -1.1% | -0.2% | -99.9% | -0.6% | +13.2% | +17.6% |
| 6 | 2723658ns | -1.3% | -0.9% | -99.9% | -1.0% | +12.4% | +16.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.171 | ok |
| abi_zig_entry_madd_zig_dispatch | 0.027 | ok |
| abi_zig_entry_madd_zig_null | -0.065 | ok |
| abi_zig_entry_madd_zig_per_w_set | -0.552 | HIGH- (thermal bounce) |
| abi_zig_entry_madd_zig_runtime_w | -0.004 | ok |
| abi_zig_entry_madd_zig_tail_dispatch | -0.149 | ok |
| abi_zig_entry_madd_zig_tail_runtime_w | -0.298 | moderate- |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_dispatch**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8345314.7ns | 2689399.6ns | 310.3% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8365944.4ns | 2697104.3ns | 310.2% | HIGH |
| abi_zig_entry_madd_zig_null | 314332.0ns | 3455.8ns | 9095.9% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8349523.6ns | 2695504.4ns | 309.8% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8438922.6ns | 2722341.5ns | 310.0% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9563197.1ns | 3076943.1ns | 310.8% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 9968914.2ns | 3198700.7ns | 311.7% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2679004.2-2696384.0 ns)
  2679004.2 |########################################
  2679873.2 |
  2680742.2 |
  2681611.2 |
  2682480.2 |
  2683349.1 |
  2684218.1 |
  2685087.1 |########################################
  2685956.1 |
  2686825.1 |########################################
  2687694.1 |
  2688563.1 |
  2689432.1 |
  2690301.0 |
  2691170.0 |########################################
  2692039.0 |
  2692908.0 |########################################
  2693777.0 |
  2694646.0 |
  2695515.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2686559.6-2704908.8 ns)
  2686559.6 |####################
  2687477.1 |
  2688394.5 |
  2689312.0 |####################
  2690229.4 |
  2691146.9 |
  2692064.3 |
  2692981.8 |
  2693899.3 |
  2694816.7 |
  2695734.2 |
  2696651.6 |
  2697569.1 |########################################
  2698486.5 |
  2699404.0 |
  2700321.5 |
  2701238.9 |
  2702156.4 |
  2703073.8 |####################
  2703991.3 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 3369.6-3511.2 ns)
   3369.6 |########################################
   3376.7 |
   3383.8 |
   3390.8 |
   3397.9 |
   3405.0 |
   3412.1 |########################################
   3419.2 |
   3426.3 |
   3433.3 |
   3440.4 |
   3447.5 |
   3454.6 |########################################
   3461.7 |########################################
   3468.8 |
   3475.8 |
   3482.9 |
   3490.0 |
   3497.1 |
   3504.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2683993.8-2707600.4 ns)
  2683993.8 |########################################
  2685174.1 |
  2686354.5 |
  2687534.8 |########################################
  2688715.1 |
  2689895.4 |
  2691075.8 |
  2692256.1 |########################################
  2693436.4 |########################################
  2694616.8 |
  2695797.1 |########################################
  2696977.4 |
  2698157.8 |
  2699338.1 |
  2700518.4 |
  2701698.8 |
  2702879.1 |
  2704059.4 |
  2705239.7 |
  2706420.1 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2708812.5-2728872.1 ns)
  2708812.5 |####################
  2709815.5 |
  2710818.5 |
  2711821.4 |
  2712824.4 |
  2713827.4 |
  2714830.4 |
  2715833.4 |
  2716836.3 |
  2717839.3 |
  2718842.3 |
  2719845.3 |
  2720848.3 |####################
  2721851.2 |####################
  2722854.2 |########################################
  2723857.2 |
  2724860.2 |
  2725863.2 |
  2726866.1 |
  2727869.1 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3061921.7-3103080.2 ns)
  3061921.7 |########################################
  3063979.6 |########################################
  3066037.6 |
  3068095.5 |
  3070153.4 |####################
  3072211.3 |
  3074269.2 |
  3076327.2 |
  3078385.1 |
  3080443.0 |
  3082501.0 |
  3084558.9 |
  3086616.8 |
  3088674.7 |
  3090732.7 |
  3092790.6 |
  3094848.5 |
  3096906.4 |
  3098964.4 |
  3101022.3 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3177685.0-3236029.0 ns)
  3177685.0 |########################################
  3180602.2 |#############
  3183519.4 |
  3186436.6 |#############
  3189353.8 |
  3192271.0 |
  3195188.2 |
  3198105.4 |
  3201022.6 |
  3203939.8 |
  3206857.0 |
  3209774.2 |
  3212691.4 |
  3215608.6 |
  3218525.8 |
  3221443.0 |
  3224360.2 |
  3227277.4 |
  3230194.6 |
  3233111.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=310.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=309.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=8963.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=309.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=309.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=309.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=309.4% of algo (FFI overhead may distort results)

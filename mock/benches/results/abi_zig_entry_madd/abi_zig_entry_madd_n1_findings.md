# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 51690% faster than the next best (abi_zig_entry_madd_zig_runtime_w)

abi_zig_entry_madd_zig_null (5.19 us) leads abi_zig_entry_madd_zig_runtime_w (2.69 ms) by 51690%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.68 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 632.0x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.28 ms) is 632.0x the fastest (5.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (51690% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 51690% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 632.0x the fastest

Fastest abi_zig_entry_madd_zig_null (5.19 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.28 ms): 632.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 5192.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 631.97x (fastest 5192.5 ns, slowest 3281496.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2691725ns | 2692655ns | 2682241ns | 2690540ns | 2698246ns | -3.14% |
| abi_zig_entry_madd_zig_dispatch | 2767451ns | 2710047ns | 2701715ns | 2707743ns | 2889881ns | -0.41% |
| abi_zig_entry_madd_zig_null | 7501ns | 7510ns | 7426ns | 7485ns | 7564ns | -99.73% |
| abi_zig_entry_madd_zig_per_w_set | 2696608ns | 2696656ns | 2690531ns | 2696369ns | 2700003ns | -2.96% |
| abi_zig_entry_madd_zig_runtime_w | 2778898ns | 2692200ns | 2674296ns | 2690046ns | 2964476ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3075241ns | 3072108ns | 3066321ns | 3070367ns | 3087013ns | +10.66% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3294494ns | 3284579ns | 3263787ns | 3280908ns | 3330227ns | +18.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2688836ns | 2679412ns | 2695291ns | -3.13% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2764441ns | 2698831ns | 2886732ns | -0.41% | 0.000 |
| abi_zig_entry_madd_zig_null | 5175ns | 5109ns | 5206ns | -99.81% | 0.000 |
| abi_zig_entry_madd_zig_per_w_set | 2693728ns | 2687677ns | 2697140ns | -2.96% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2775797ns | 2671598ns | 2961012ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3072198ns | 3063385ns | 3083698ns | +10.68% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3291386ns | 3260924ns | 3326860ns | +18.57% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 195572.4 | 2689159.0 | 2688836.3 | n/a |
| abi_zig_entry_madd_zig_dispatch | 312831.5 | 2722849.1 | 2764441.1 | 9 |
| abi_zig_entry_madd_zig_null | 158122.0 | 5205.8 | 5174.9 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 196717.3 | 2693693.9 | 2693727.5 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 212956.9 | 2793177.3 | 2775796.9 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 201408.0 | 3071689.4 | 3072198.5 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 221225.2 | 3248434.6 | 3291385.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_madd_zig_null | 0.000 | 98.4% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.2% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2691725ns | 2691725ns | -3.14% |
| abi_zig_entry_madd_zig_dispatch | 2767451ns | 2767451ns | -0.41% |
| abi_zig_entry_madd_zig_null | 7501ns | 7501ns | -99.73% |
| abi_zig_entry_madd_zig_per_w_set | 2696608ns | 2696608ns | -2.96% |
| abi_zig_entry_madd_zig_runtime_w | 2778898ns | 2778898ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3075241ns | 3075241ns | +10.66% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3294494ns | 3294494ns | +18.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2689172ns | base | --- | [2677206, 2961012] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2689796ns | no significant difference | [-272062, +6686]ns | [2681421, 2695291] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2707145ns | no significant difference | [-78173, +28722]ns | [2699445, 2886732] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_null | 5192ns | -2683997.5ns (-99.8%) | [-2955861, -2672008]ns | [5127, 5206] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2693725ns | no significant difference | [-267458, +16052]ns | [2690318, 2697140] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3069176ns | +380235.2ns (+14.1%) | [+105946, +403024]ns | [3063722, 3083698] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3281496ns | +592202.3ns (+22.0%) | [+348193, +606371]ns | [3265801, 3326860] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2689751ns | +0.2% | +0.5% | -99.8% | +0.2% | +14.3% | +22.5% |
| 2 | 3230183ns | -16.9% | -5.2% | -99.8% | -16.6% | -5.1% | +3.9% |
| 3 | 2691842ns | +0.1% | +0.7% | -99.8% | +0.0% | +14.0% | +21.1% |
| 4 | 2682813ns | +0.0% | +0.6% | -99.8% | +0.2% | +14.2% | +22.6% |
| 5 | 2671598ns | +0.3% | +1.4% | -99.8% | +0.8% | +15.8% | +22.4% |
| 6 | 2688594ns | +0.2% | +0.4% | -99.8% | +0.4% | +14.1% | +21.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.334 | moderate- |
| abi_zig_entry_madd_zig_dispatch | -0.217 | moderate- |
| abi_zig_entry_madd_zig_null | -0.445 | moderate- |
| abi_zig_entry_madd_zig_per_w_set | 0.078 | ok |
| abi_zig_entry_madd_zig_runtime_w | -0.204 | moderate- |
| abi_zig_entry_madd_zig_tail_dispatch | -0.385 | moderate- |
| abi_zig_entry_madd_zig_tail_runtime_w | -0.200 | moderate- |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 1/6, lost 4/6
- **abi_zig_entry_madd_zig_dispatch**: won 1/6, lost 5/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 1/6, lost 4/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 1/6, lost 5/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8328443.3ns | 2688836.3ns | 309.7% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8542340.3ns | 2764441.1ns | 309.0% | HIGH |
| abi_zig_entry_madd_zig_null | 316717.8ns | 5174.9ns | 6120.2% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8352556.4ns | 2693727.5ns | 310.1% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8591369.2ns | 2775796.9ns | 309.5% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9497844.1ns | 3072198.5ns | 309.2% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 10010856.6ns | 3291385.6ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2679411.7-2695291.0 ns)
  2679411.7 |########################################
  2680205.7 |
  2680999.6 |
  2681793.6 |
  2682587.6 |
  2683381.5 |########################################
  2684175.5 |
  2684969.5 |########################################
  2685763.4 |
  2686557.4 |
  2687351.4 |
  2688145.3 |
  2688939.3 |
  2689733.3 |
  2690527.2 |
  2691321.2 |
  2692115.2 |
  2692909.1 |
  2693703.1 |########################################
  2694497.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2698831.2-2886732.3 ns)
  2698831.2 |########################################
  2708226.3 |##########################
  2717621.3 |
  2727016.4 |
  2736411.4 |
  2745806.5 |
  2755201.5 |
  2764596.6 |
  2773991.6 |
  2783386.7 |
  2792781.8 |
  2802176.8 |
  2811571.9 |
  2820966.9 |
  2830362.0 |
  2839757.0 |
  2849152.1 |
  2858547.1 |
  2867942.2 |
  2877337.2 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 5108.8-5205.6 ns)
   5108.8 |########################################
   5113.6 |
   5118.5 |
   5123.3 |
   5128.2 |
   5133.0 |
   5137.8 |
   5142.7 |########################################
   5147.5 |
   5152.4 |
   5157.2 |
   5162.0 |
   5166.9 |
   5171.7 |
   5176.6 |
   5181.4 |
   5186.2 |########################################
   5191.1 |########################################
   5195.9 |
   5200.8 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2687677.1-2697139.8 ns)
  2687677.1 |####################
  2688150.2 |
  2688623.4 |
  2689096.5 |
  2689569.6 |
  2690042.8 |
  2690515.9 |
  2690989.0 |
  2691462.2 |
  2691935.3 |
  2692408.5 |
  2692881.6 |########################################
  2693354.7 |
  2693827.9 |####################
  2694301.0 |
  2694774.1 |
  2695247.3 |####################
  2695720.4 |
  2696193.5 |
  2696666.7 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2671597.9-2961012.5 ns)
  2671597.9 |##########################
  2686068.6 |########################################
  2700539.4 |
  2715010.1 |
  2729480.8 |
  2743951.5 |
  2758422.3 |
  2772893.0 |
  2787363.7 |
  2801834.5 |
  2816305.2 |
  2830775.9 |
  2845246.7 |
  2859717.4 |
  2874188.1 |
  2888658.9 |
  2903129.6 |
  2917600.3 |
  2932071.0 |
  2946541.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3063385.0-3083698.1 ns)
  3063385.0 |########################################
  3064400.7 |
  3065416.3 |
  3066432.0 |
  3067447.6 |
  3068463.3 |####################
  3069478.9 |####################
  3070494.6 |
  3071510.3 |
  3072525.9 |
  3073541.6 |####################
  3074557.2 |
  3075572.9 |
  3076588.5 |
  3077604.2 |
  3078619.9 |
  3079635.5 |
  3080651.2 |
  3081666.8 |
  3082682.5 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3260924.2-3326860.2 ns)
  3260924.2 |########################################
  3264221.0 |
  3267517.8 |########################################
  3270814.6 |########################################
  3274111.4 |
  3277408.2 |
  3280705.0 |
  3284001.8 |
  3287298.6 |########################################
  3290595.4 |
  3293892.2 |########################################
  3297189.0 |
  3300485.8 |
  3303782.6 |
  3307079.4 |
  3310376.2 |
  3313673.0 |
  3316969.8 |
  3320266.6 |
  3323563.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=309.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=310.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=6073.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=309.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=310.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=308.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=302.8% of algo (FFI overhead may distort results)

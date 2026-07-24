# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 86108% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (3.13 us) leads abi_zig_entry_madd_zig_anchor (2.70 ms) by 86108%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.71 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_dispatch is an outlier: 1035.6x slower than the field

abi_zig_entry_madd_zig_tail_dispatch (3.24 ms) is 1035.6x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_tail_runtime_w, abi_zig_entry_madd_zig_tail_dispatch} (86108% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_tail_runtime_w, abi_zig_entry_madd_zig_tail_dispatch} with a 86108% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1035.6x the fastest

Fastest abi_zig_entry_madd_zig_null (3.13 us) to slowest abi_zig_entry_madd_zig_tail_dispatch (3.24 ms): 1035.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 3128.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1035.60x (fastest 3128.1 ns, slowest 3239464.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2699158ns | 2699688ns | 2690602ns | 2696757ns | 2707039ns | -0.67% |
| abi_zig_entry_madd_zig_dispatch | 2861536ns | 2752053ns | 2710232ns | 2738542ns | 3121680ns | +5.31% |
| abi_zig_entry_madd_zig_null | 5716ns | 5500ns | 5395ns | 5473ns | 6242ns | -99.79% |
| abi_zig_entry_madd_zig_per_w_set | 2895836ns | 2737004ns | 2703292ns | 2730116ns | 3240687ns | +6.57% |
| abi_zig_entry_madd_zig_runtime_w | 2717288ns | 2713533ns | 2699398ns | 2711580ns | 2734794ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3354001ns | 3243544ns | 3177533ns | 3227268ns | 3632333ns | +23.43% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3316337ns | 3197154ns | 3184555ns | 3194006ns | 3565725ns | +22.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2695940ns | 2687714ns | 2703350ns | -0.67% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2856455ns | 2707101ns | 3112862ns | +5.24% | 0.000 |
| abi_zig_entry_madd_zig_null | 3191ns | 3098ns | 3339ns | -99.88% | 0.080 |
| abi_zig_entry_madd_zig_per_w_set | 2892093ns | 2700510ns | 3236253ns | +6.56% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2714150ns | 2696819ns | 2731229ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3350265ns | 3174775ns | 3628245ns | +23.44% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3312722ns | 3181669ns | 3561229ns | +22.05% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 213778.3 | 2699765.3 | 2695940.0 | n/a |
| abi_zig_entry_madd_zig_dispatch | 247501.1 | 2826069.1 | 2856455.3 | n/a |
| abi_zig_entry_madd_zig_null | 168862.5 | 3369.4 | 3191.2 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 272825.0 | 2821767.3 | 2892093.1 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 215226.1 | 2715019.2 | 2714149.5 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 251566.8 | 3263183.5 | 3350264.7 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 242879.2 | 3266722.9 | 3312721.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.082 | 99.0% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2699158ns | 2699158ns | -0.67% |
| abi_zig_entry_madd_zig_dispatch | 2861536ns | 2861536ns | +5.31% |
| abi_zig_entry_madd_zig_null | 5716ns | 5716ns | -99.79% |
| abi_zig_entry_madd_zig_per_w_set | 2895836ns | 2895836ns | +6.57% |
| abi_zig_entry_madd_zig_runtime_w | 2717288ns | 2717288ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3354001ns | 3354001ns | +23.43% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3316337ns | 3316337ns | +22.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2710515ns | base | --- | [2700705, 2731229] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2696657ns | -12546.9ns (-0.5%) | [-32807, -9275]ns | [2687812, 2703350] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2748738ns | no significant difference | [-18049, +408260]ns | [2707766, 3112862] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_null | 3128ns | -2707271.7ns (-99.9%) | [-2728005, -2697598]ns | [3107, 3339] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2733309ns | +22959.1ns (+0.8%) | [+4331, +506541]ns | [2706718, 3236253] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3239465ns | +509753.0ns (+18.8%) | [+476466, +922126]ns | [3183084, 3628245] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3193710ns | +487579.8ns (+18.0%) | [+478137, +830000]ns | [3183226, 3561229] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2712385ns | -0.4% | +26.9% | -99.9% | +24.7% | +19.2% | +17.7% |
| 2 | 2747038ns | -1.8% | -1.4% | -99.9% | +12.5% | +18.2% | +28.3% |
| 3 | 2715419ns | -0.3% | +2.5% | -99.9% | +0.2% | +20.2% | +32.5% |
| 4 | 2704591ns | -0.6% | +0.1% | -99.9% | +0.3% | +18.0% | +18.1% |
| 5 | 2696819ns | -0.3% | +3.3% | -99.9% | +0.1% | +48.0% | +18.0% |
| 6 | 2708644ns | -0.5% | +0.2% | -99.9% | +1.4% | +17.2% | +17.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.090 | ok |
| abi_zig_entry_madd_zig_dispatch | -0.106 | ok |
| abi_zig_entry_madd_zig_null | -0.288 | moderate- |
| abi_zig_entry_madd_zig_per_w_set | 0.392 | moderate+ |
| abi_zig_entry_madd_zig_runtime_w | 0.154 | ok |
| abi_zig_entry_madd_zig_tail_dispatch | -0.360 | moderate- |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.178 | ok |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_dispatch**: won 1/6, lost 4/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8389391.8ns | 2695940.0ns | 311.2% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 9088666.9ns | 2856455.3ns | 318.2% | HIGH |
| abi_zig_entry_madd_zig_null | 322027.8ns | 3191.2ns | 10091.0% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8853663.5ns | 2892093.1ns | 306.1% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8432075.5ns | 2714149.5ns | 310.7% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 10239143.7ns | 3350264.7ns | 305.6% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 10156262.3ns | 3312721.7ns | 306.6% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2687714.2-2703350.4 ns)
  2687714.2 |########################################
  2688496.0 |
  2689277.8 |
  2690059.6 |
  2690841.4 |
  2691623.2 |
  2692405.1 |
  2693186.9 |
  2693968.7 |
  2694750.5 |####################
  2695532.3 |
  2696314.1 |
  2697095.9 |
  2697877.7 |####################
  2698659.5 |
  2699441.4 |
  2700223.2 |####################
  2701005.0 |
  2701786.8 |
  2702568.6 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2707100.8-3112862.1 ns)
  2707100.8 |########################################
  2727388.9 |
  2747676.9 |
  2767965.0 |##########################
  2788253.1 |
  2808541.1 |
  2828829.2 |
  2849117.3 |
  2869405.3 |
  2889693.4 |
  2909981.5 |
  2930269.5 |
  2950557.6 |
  2970845.6 |
  2991133.7 |
  3011421.8 |
  3031709.8 |
  3051997.9 |
  3072286.0 |
  3092574.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 3097.5-3338.9 ns)
   3097.5 |####################
   3109.6 |########################################
   3121.6 |
   3133.7 |####################
   3145.8 |
   3157.9 |
   3169.9 |
   3182.0 |
   3194.1 |
   3206.2 |
   3218.2 |
   3230.3 |
   3242.4 |
   3254.4 |
   3266.5 |
   3278.6 |
   3290.7 |
   3302.7 |
   3314.8 |
   3326.9 |####################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2700510.4-3236252.9 ns)
  2700510.4 |########################################
  2727297.5 |#############
  2754084.6 |
  2780871.8 |
  2807658.9 |
  2834446.0 |
  2861233.1 |
  2888020.3 |
  2914807.4 |
  2941594.5 |
  2968381.6 |
  2995168.8 |
  3021955.9 |
  3048743.0 |
  3075530.1 |#############
  3102317.3 |
  3129104.4 |
  3155891.5 |
  3182678.6 |
  3209465.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2696818.8-2731228.8 ns)
  2696818.8 |########################################
  2698539.3 |
  2700259.8 |
  2701980.3 |
  2703700.8 |########################################
  2705421.3 |
  2707141.8 |########################################
  2708862.3 |
  2710582.8 |
  2712303.3 |########################################
  2714023.8 |########################################
  2715744.3 |
  2717464.8 |
  2719185.3 |
  2720905.8 |
  2722626.3 |
  2724346.8 |
  2726067.3 |
  2727787.8 |
  2729508.3 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3174775.0-3628245.2 ns)
  3174775.0 |########################################
  3197448.5 |
  3220122.0 |####################
  3242795.5 |########################################
  3265469.0 |
  3288142.5 |
  3310816.1 |
  3333489.6 |
  3356163.1 |
  3378836.6 |
  3401510.1 |
  3424183.6 |
  3446857.1 |
  3469530.6 |
  3492204.1 |
  3514877.7 |
  3537551.2 |
  3560224.7 |
  3582898.2 |
  3605571.7 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3181669.2-3561229.0 ns)
  3181669.2 |########################################
  3200647.2 |
  3219625.2 |
  3238603.2 |
  3257581.2 |
  3276559.1 |
  3295537.1 |
  3314515.1 |
  3333493.1 |
  3352471.1 |
  3371449.1 |
  3390427.1 |
  3409405.1 |
  3428383.0 |
  3447361.0 |
  3466339.0 |
  3485317.0 |
  3504295.0 |
  3523273.0 |##########
  3542251.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=310.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=310.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=10156.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=310.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=310.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=309.9% of algo (FFI overhead may distort results)

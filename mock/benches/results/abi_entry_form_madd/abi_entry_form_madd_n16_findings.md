# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 110696% faster than the next best (abi_entry_form_madd_scalar_anchor)

abi_entry_form_madd_null_entry (2.67 us) leads abi_entry_form_madd_scalar_anchor (2.96 ms) by 110696%, a clear separation rather than a photo finish. CV 11.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.99 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_dispatch_table is an outlier: 1157.4x slower than the field

abi_entry_form_madd_dispatch_table (3.09 ms) is 1157.4x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_madd_per_w_set shows alternating (throttle bounce) (autocorr -0.58)

abi_entry_form_madd_per_w_set's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_per_w_set, abi_entry_form_madd_dispatch_table} (110696% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_per_w_set, abi_entry_form_madd_dispatch_table} with a 110696% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1157.4x the fastest

Fastest abi_entry_form_madd_null_entry (2.67 us) to slowest abi_entry_form_madd_dispatch_table (3.09 ms): 1157.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 2672.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1157.42x (fastest 2672.7 ns, slowest 3093441.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3157033ns | 3098808ns | 2769395ns | 3082916ns | 3462028ns | +0.52% |
| abi_entry_form_madd_null_entry | 5349ns | 5128ns | 4902ns | 5053ns | 6017ns | -99.83% |
| abi_entry_form_madd_per_w_set | 3020391ns | 3037678ns | 2870764ns | 2999191ns | 3127006ns | -3.83% |
| abi_entry_form_madd_runtime_w | 3140659ns | 3002418ns | 2845585ns | 2953192ns | 3569397ns | base |
| abi_entry_form_madd_scalar_anchor | 2955841ns | 2965918ns | 2799271ns | 2914824ns | 3095652ns | -5.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3151434ns | 2765595ns | 3455565ns | +0.51% | 0.000 |
| abi_entry_form_madd_null_entry | 2832ns | 2578ns | 3244ns | -99.91% | 0.006 |
| abi_entry_form_madd_per_w_set | 3015486ns | 2866249ns | 3121817ns | -3.83% | 0.000 |
| abi_entry_form_madd_runtime_w | 3135516ns | 2840951ns | 3563551ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 2951261ns | 2795164ns | 3090863ns | -5.88% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 114818.8 | 3137040.5 | 3151434.1 | 3 |
| abi_entry_form_madd_null_entry | 33979.0 | 3012.0 | 2831.9 | n/a |
| abi_entry_form_madd_per_w_set | 108675.5 | 3018118.4 | 3015485.8 | n/a |
| abi_entry_form_madd_runtime_w | 157041.3 | 3131350.7 | 3135515.6 | n/a |
| abi_entry_form_madd_scalar_anchor | 101867.9 | 2945314.7 | 2951260.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.006 | 96.5% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3157033ns | 3157033ns | +0.52% |
| abi_entry_form_madd_null_entry | 5349ns | 5349ns | -99.83% |
| abi_entry_form_madd_per_w_set | 3020391ns | 3020391ns | -3.83% |
| abi_entry_form_madd_runtime_w | 3140659ns | 3140659ns | base |
| abi_entry_form_madd_scalar_anchor | 2955841ns | 2955841ns | -5.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2997315ns | base | --- | [2845680, 3563551] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 3093441ns | no significant difference | [-288458, +240088]ns | [2905296, 3455565] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_madd_null_entry | 2673ns | -2994486.9ns (-99.9%) | [-3560536, -2843029]ns | [2579, 3244] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 3032800ns | no significant difference | [-576070, +219853]ns | [2891840, 3121817] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_scalar_anchor | 2961247ns | no significant difference | [-571972, +44853]ns | [2801671, 3090863] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2953518ns | +3.4% | -99.9% | +6.4% | -0.3% |
| 2 | 2840951ns | -2.7% | -99.9% | +0.9% | -1.6% |
| 3 | 4076196ns | -12.3% | -99.9% | -25.0% | -26.9% |
| 4 | 3041112ns | +3.0% | -99.9% | -1.1% | +1.5% |
| 5 | 3050907ns | +9.4% | -99.9% | -4.4% | +1.5% |
| 6 | 2850410ns | +6.8% | -99.9% | +8.8% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.408 | moderate- |
| abi_entry_form_madd_null_entry | 0.055 | ok |
| abi_entry_form_madd_per_w_set | -0.581 | HIGH- (thermal bounce) |
| abi_entry_form_madd_runtime_w | -0.254 | moderate- |
| abi_entry_form_madd_scalar_anchor | -0.007 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 2/6, lost 4/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_madd_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 9540792.2ns | 3151434.1ns | 302.7% | HIGH |
| abi_entry_form_madd_null_entry | 126438.0ns | 2831.9ns | 4464.8% | HIGH |
| abi_entry_form_madd_per_w_set | 9164893.8ns | 3015485.8ns | 303.9% | HIGH |
| abi_entry_form_madd_runtime_w | 9471714.8ns | 3135515.6ns | 302.1% | HIGH |
| abi_entry_form_madd_scalar_anchor | 9057126.1ns | 2951260.5ns | 306.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2765595.4-3455564.8 ns)
  2765595.4 |####################
  2800093.9 |
  2834592.3 |
  2869090.8 |
  2903589.3 |
  2938087.8 |
  2972586.2 |
  3007084.7 |
  3041583.2 |########################################
  3076081.6 |
  3110580.1 |####################
  3145078.6 |
  3179577.0 |
  3214075.5 |
  3248574.0 |
  3283072.4 |
  3317570.9 |####################
  3352069.4 |
  3386567.9 |
  3421066.3 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 2578.3-3243.8 ns)
   2578.3 |########################################
   2611.6 |####################
   2644.8 |
   2678.1 |
   2711.4 |####################
   2744.7 |
   2777.9 |
   2811.2 |
   2844.5 |
   2877.8 |
   2911.0 |
   2944.3 |
   2977.6 |
   3010.8 |
   3044.1 |
   3077.4 |####################
   3110.7 |
   3143.9 |
   3177.2 |
   3210.5 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2866249.2-3121816.7 ns)
  2866249.2 |########################################
  2879027.6 |
  2891805.9 |
  2904584.3 |
  2917362.7 |########################################
  2930141.1 |
  2942919.4 |
  2955697.8 |
  2968476.2 |
  2981254.6 |
  2994032.9 |
  3006811.3 |########################################
  3019589.7 |
  3032368.0 |
  3045146.4 |########################################
  3057924.8 |
  3070703.2 |
  3083481.5 |
  3096259.9 |########################################
  3109038.3 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2840950.8-3563551.5 ns)
  2840950.8 |########################################
  2877080.8 |
  2913210.9 |
  2949340.9 |####################
  2985470.9 |
  3021601.0 |########################################
  3057731.0 |
  3093861.0 |
  3129991.1 |
  3166121.1 |
  3202251.1 |
  3238381.2 |
  3274511.2 |
  3310641.2 |
  3346771.3 |
  3382901.3 |
  3419031.3 |
  3455161.4 |
  3491291.4 |
  3527421.4 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2795164.2-3090863.1 ns)
  2795164.2 |########################################
  2809949.1 |
  2824734.1 |
  2839519.0 |
  2854304.0 |
  2869088.9 |
  2883873.9 |
  2898658.8 |
  2913443.8 |
  2928228.7 |
  2943013.6 |####################
  2957798.6 |
  2972583.5 |####################
  2987368.5 |
  3002153.4 |
  3016938.4 |
  3031723.3 |
  3046508.3 |
  3061293.2 |
  3076078.2 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=4637.2% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=309.2% of algo (FFI overhead may distort results)

# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 87150% faster than the next best (abi_entry_form_madd_dispatch_table)

abi_entry_form_madd_null_entry (3.20 us) leads abi_entry_form_madd_dispatch_table (2.79 ms) by 87150%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.90 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_scalar_anchor is an outlier: 920.5x slower than the field

abi_entry_form_madd_scalar_anchor (2.95 ms) is 920.5x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set, abi_entry_form_madd_runtime_w, abi_entry_form_madd_scalar_anchor} (87150% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set, abi_entry_form_madd_runtime_w, abi_entry_form_madd_scalar_anchor} with a 87150% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 920.5x the fastest

Fastest abi_entry_form_madd_null_entry (3.20 us) to slowest abi_entry_form_madd_scalar_anchor (2.95 ms): 920.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 3200.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 920.53x (fastest 3200.7 ns, slowest 2946296.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2895584ns | 2796685ns | 2752655ns | 2783498ns | 3135176ns | +0.64% |
| abi_entry_form_madd_null_entry | 5540ns | 5483ns | 5396ns | 5475ns | 5709ns | -99.81% |
| abi_entry_form_madd_per_w_set | 2926061ns | 2864146ns | 2744232ns | 2827193ns | 3165277ns | +1.70% |
| abi_entry_form_madd_runtime_w | 2877204ns | 2911102ns | 2768029ns | 2863575ns | 2952237ns | base |
| abi_entry_form_madd_scalar_anchor | 2947973ns | 2951087ns | 2734831ns | 2887566ns | 3145155ns | +2.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2891240ns | 2748785ns | 3130216ns | +0.65% | 0.000 |
| abi_entry_form_madd_null_entry | 3197ns | 3136ns | 3250ns | -99.89% | 0.080 |
| abi_entry_form_madd_per_w_set | 2921519ns | 2740731ns | 3159636ns | +1.70% | 0.000 |
| abi_entry_form_madd_runtime_w | 2872610ns | 2764160ns | 2947259ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 2943359ns | 2731384ns | 3139825ns | +2.46% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 88295.3 | 2876044.8 | 2891239.6 | 0 |
| abi_entry_form_madd_null_entry | 30214.2 | 3265.2 | 3196.8 | n/a |
| abi_entry_form_madd_per_w_set | 92078.9 | 2915436.2 | 2921519.0 | 0 |
| abi_entry_form_madd_runtime_w | 92409.2 | 2911686.7 | 2872610.3 | n/a |
| abi_entry_form_madd_scalar_anchor | 96435.8 | 2943197.9 | 2943359.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.080 | 98.0% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2895584ns | 2895584ns | +0.64% |
| abi_entry_form_madd_null_entry | 5540ns | 5540ns | -99.81% |
| abi_entry_form_madd_per_w_set | 2926061ns | 2926061ns | +1.70% |
| abi_entry_form_madd_runtime_w | 2877204ns | 2877204ns | base |
| abi_entry_form_madd_scalar_anchor | 2947973ns | 2947973ns | +2.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2906099ns | base | --- | [2764472, 2947259] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2792574ns | no significant difference | [-129985, +199416]ns | [2750929, 3130216] | no | 1.0000 | 0.6875 | 0 |
| abi_entry_form_madd_null_entry | 3201ns | -2902861.4ns (-99.9%) | [-2944119, -2761260]ns | [3140, 3250] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2859828ns | no significant difference | [-121705, +238515]ns | [2745093, 3159636] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_scalar_anchor | 2946297ns | no significant difference | [-20516, +209024]ns | [2743956, 3139825] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2764785ns | -0.6% | -99.9% | -0.9% | -1.2% |
| 2 | 2886674ns | -4.4% | -99.9% | -1.2% | -0.2% |
| 3 | 2764160ns | -0.4% | -99.9% | +3.7% | -0.3% |
| 4 | 2925525ns | +12.2% | -99.9% | +12.8% | +4.4% |
| 5 | 2958442ns | -4.5% | -99.9% | -7.1% | +1.8% |
| 6 | 2936076ns | +1.4% | -99.9% | +2.9% | +9.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.222 | moderate- |
| abi_entry_form_madd_null_entry | 0.419 | moderate+ |
| abi_entry_form_madd_per_w_set | -0.387 | moderate- |
| abi_entry_form_madd_runtime_w | 0.032 | ok |
| abi_entry_form_madd_scalar_anchor | 0.172 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_madd_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 8720495.5ns | 2891239.6ns | 301.6% | HIGH |
| abi_entry_form_madd_null_entry | 123136.4ns | 3196.8ns | 3851.8% | HIGH |
| abi_entry_form_madd_per_w_set | 8845505.5ns | 2921519.0ns | 302.8% | HIGH |
| abi_entry_form_madd_runtime_w | 8754259.2ns | 2872610.3ns | 304.7% | HIGH |
| abi_entry_form_madd_scalar_anchor | 8891820.2ns | 2943359.3ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2748784.6-3130216.5 ns)
  2748784.6 |########################################
  2767856.2 |
  2786927.8 |
  2805999.4 |#############
  2825071.0 |
  2844142.6 |
  2863214.2 |
  2882285.7 |
  2901357.3 |
  2920428.9 |
  2939500.5 |
  2958572.1 |
  2977643.7 |#############
  2996715.3 |
  3015786.9 |
  3034858.5 |
  3053930.1 |
  3073001.7 |
  3092073.3 |
  3111144.9 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 3135.8-3249.6 ns)
   3135.8 |########################################
   3141.5 |########################################
   3147.2 |
   3152.9 |
   3158.6 |
   3164.2 |
   3169.9 |
   3175.6 |
   3181.3 |
   3187.0 |
   3192.7 |########################################
   3198.4 |
   3204.1 |########################################
   3209.8 |
   3215.5 |########################################
   3221.2 |
   3226.8 |
   3232.5 |
   3238.2 |
   3243.9 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2740730.8-3159636.0 ns)
  2740730.8 |########################################
  2761676.1 |
  2782621.3 |
  2803566.6 |
  2824511.8 |
  2845457.1 |####################
  2866402.4 |####################
  2887347.6 |
  2908292.9 |
  2929238.2 |
  2950183.4 |
  2971128.7 |
  2992073.9 |
  3013019.2 |####################
  3033964.5 |
  3054909.7 |
  3075855.0 |
  3096800.3 |
  3117745.5 |
  3138690.8 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2764159.6-2947259.4 ns)
  2764159.6 |########################################
  2773314.6 |
  2782469.6 |
  2791624.6 |
  2800779.6 |
  2809934.5 |
  2819089.5 |
  2828244.5 |
  2837399.5 |
  2846554.5 |
  2855709.5 |
  2864864.5 |
  2874019.5 |
  2883174.4 |####################
  2892329.4 |
  2901484.4 |
  2910639.4 |
  2919794.4 |####################
  2928949.4 |####################
  2938104.4 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2731384.2-3139824.8 ns)
  2731384.2 |########################################
  2751806.2 |########################################
  2772228.3 |
  2792650.3 |
  2813072.3 |
  2833494.4 |
  2853916.4 |
  2874338.4 |########################################
  2894760.4 |
  2915182.5 |
  2935604.5 |
  2956026.5 |
  2976448.6 |
  2996870.6 |########################################
  3017292.6 |
  3037714.6 |########################################
  3058136.7 |
  3078558.7 |
  3098980.7 |
  3119402.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=3841.9% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=299.1% of algo (FFI overhead may distort results)

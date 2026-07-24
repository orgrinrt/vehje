# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_madd_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_madd_runtime_w has the worst median (3.13 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_madd_null_entry at 3.83 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_madd_null_entry dominates: 76857% faster than the next best (abi_entry_form_madd_per_w_set)

abi_entry_form_madd_null_entry (3.83 us) leads abi_entry_form_madd_per_w_set (2.95 ms) by 76857%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -3.13 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_runtime_w is an outlier: 818.8x slower than the field

abi_entry_form_madd_runtime_w (3.13 ms) is 818.8x the fastest (3.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_per_w_set, abi_entry_form_madd_dispatch_table, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w} (76857% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_per_w_set, abi_entry_form_madd_dispatch_table, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w} with a 76857% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 818.8x the fastest

Fastest abi_entry_form_madd_null_entry (3.83 us) to slowest abi_entry_form_madd_runtime_w (3.13 ms): 818.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 3828.1 ns median (-99.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 818.77x (fastest 3828.1 ns, slowest 3134329.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3015821ns | 2985549ns | 2869981ns | 2953418ns | 3182345ns | -7.00% |
| abi_entry_form_madd_null_entry | 6467ns | 6514ns | 5726ns | 6450ns | 6864ns | -99.80% |
| abi_entry_form_madd_per_w_set | 3015631ns | 2951149ns | 2892200ns | 2931951ns | 3202865ns | -7.00% |
| abi_entry_form_madd_runtime_w | 3242705ns | 3139827ns | 2915142ns | 3116443ns | 3595879ns | base |
| abi_entry_form_madd_scalar_anchor | 3058833ns | 3028606ns | 2919943ns | 2994789ns | 3224345ns | -5.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3010424ns | 2865102ns | 3175629ns | -7.01% | 0.000 |
| abi_entry_form_madd_null_entry | 3785ns | 3442ns | 3929ns | -99.88% | 0.001 |
| abi_entry_form_madd_per_w_set | 3010428ns | 2887226ns | 3197215ns | -7.01% | 0.000 |
| abi_entry_form_madd_runtime_w | 3237213ns | 2910381ns | 3589941ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 3053691ns | 2914627ns | 3219249ns | -5.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 109934.3 | 3026427.1 | 3010423.8 | n/a |
| abi_entry_form_madd_null_entry | 35639.5 | 3958.6 | 3785.1 | n/a |
| abi_entry_form_madd_per_w_set | 115219.6 | 3008609.4 | 3010428.3 | 0 |
| abi_entry_form_madd_runtime_w | 113297.8 | 3245423.2 | 3237212.8 | n/a |
| abi_entry_form_madd_scalar_anchor | 112817.3 | 3039113.5 | 3053690.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.001 | 89.9% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3015821ns | 3015821ns | -7.00% |
| abi_entry_form_madd_null_entry | 6467ns | 6467ns | -99.80% |
| abi_entry_form_madd_per_w_set | 3015631ns | 3015631ns | -7.00% |
| abi_entry_form_madd_runtime_w | 3242705ns | 3242705ns | base |
| abi_entry_form_madd_scalar_anchor | 3058833ns | 3058833ns | -5.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 3134329ns | base | --- | [2987368, 3589941] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2980778ns | -180207.1ns (-5.7%) | [-438337, -61823]ns | [2874865, 3175629] | YES | 0.0417 | 0.0313 | 0 |
| abi_entry_form_madd_null_entry | 3828ns | -3130731.0ns (-99.9%) | [-3586058, -2983494]ns | [3598, 3929] | YES | 0.0417 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2945990ns | -208551.6ns (-6.7%) | [-423398, -48404]ns | [2888080, 3197215] | YES | 0.0417 | 0.0313 | 0 |
| abi_entry_form_madd_scalar_anchor | 3023485ns | no significant difference | [-455262, +34649]ns | [2918338, 3219249] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2910381ns | -1.6% | -99.9% | -0.8% | +0.4% |
| 2 | 3064355ns | -3.0% | -99.9% | -2.4% | +1.9% |
| 3 | 3907049ns | -15.2% | -99.9% | -15.2% | -17.2% |
| 4 | 3272834ns | -8.7% | -99.9% | -5.8% | -2.1% |
| 5 | 3115960ns | -2.5% | -99.9% | -7.3% | -6.1% |
| 6 | 3152698ns | -8.5% | -99.9% | -8.0% | -7.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.125 | ok |
| abi_entry_form_madd_null_entry | 0.201 | moderate+ |
| abi_entry_form_madd_per_w_set | 0.165 | ok |
| abi_entry_form_madd_runtime_w | -0.048 | ok |
| abi_entry_form_madd_scalar_anchor | 0.257 | moderate+ |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 6/6, lost 0/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 6/6, lost 0/6
- **abi_entry_form_madd_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 9213735.5ns | 3010423.8ns | 306.1% | HIGH |
| abi_entry_form_madd_null_entry | 130701.6ns | 3785.1ns | 3453.0% | HIGH |
| abi_entry_form_madd_per_w_set | 9173357.2ns | 3010428.3ns | 304.7% | HIGH |
| abi_entry_form_madd_runtime_w | 9878287.1ns | 3237212.8ns | 305.1% | HIGH |
| abi_entry_form_madd_scalar_anchor | 9227910.5ns | 3053690.6ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2865101.7-3175628.5 ns)
  2865101.7 |########################################
  2880628.0 |########################################
  2896154.4 |
  2911680.7 |
  2927207.1 |
  2942733.4 |
  2958259.8 |########################################
  2973786.1 |
  2989312.4 |########################################
  3004838.8 |
  3020365.1 |
  3035891.5 |########################################
  3051417.8 |
  3066944.2 |
  3082470.5 |
  3097996.8 |
  3113523.2 |
  3129049.5 |
  3144575.9 |
  3160102.2 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 3441.7-3928.9 ns)
   3441.7 |########################################
   3466.1 |
   3490.4 |
   3514.8 |
   3539.1 |
   3563.5 |
   3587.9 |
   3612.2 |
   3636.6 |
   3661.0 |
   3685.3 |
   3709.7 |
   3734.0 |########################################
   3758.4 |
   3782.8 |
   3807.1 |########################################
   3831.5 |########################################
   3855.9 |
   3880.2 |########################################
   3904.6 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2887225.8-3197214.6 ns)
  2887225.8 |########################################
  2902725.2 |
  2918224.7 |
  2933724.1 |
  2949223.6 |
  2964723.0 |
  2980222.4 |#############
  2995721.9 |
  3011221.3 |
  3026720.8 |
  3042220.2 |
  3057719.6 |
  3073219.1 |#############
  3088718.5 |
  3104218.0 |
  3119717.4 |
  3135216.8 |
  3150716.3 |
  3166215.7 |
  3181715.2 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2910381.2-3589941.2 ns)
  2910381.2 |########################################
  2944359.2 |
  2978337.2 |
  3012315.2 |
  3046293.2 |########################################
  3080271.2 |
  3114249.2 |########################################
  3148227.2 |########################################
  3182205.2 |
  3216183.2 |
  3250161.2 |########################################
  3284139.2 |
  3318117.2 |
  3352095.2 |
  3386073.2 |
  3420051.2 |
  3454029.2 |
  3488007.2 |
  3521985.2 |
  3555963.2 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2914626.7-3219248.8 ns)
  2914626.7 |########################################
  2929857.8 |
  2945088.9 |
  2960320.0 |
  2975551.1 |
  2990782.2 |
  3006013.3 |
  3021244.4 |
  3036475.5 |
  3051706.6 |
  3066937.7 |
  3082168.8 |
  3097399.9 |
  3112631.0 |#############
  3127862.1 |
  3143093.2 |
  3158324.3 |
  3173555.4 |
  3188786.5 |#############
  3204017.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=3431.7% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=299.8% of algo (FFI overhead may distort results)

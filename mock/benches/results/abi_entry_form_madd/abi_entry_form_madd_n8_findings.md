# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 91416% faster than the next best (abi_entry_form_madd_scalar_anchor)

abi_entry_form_madd_null_entry (3.16 us) leads abi_entry_form_madd_scalar_anchor (2.89 ms) by 91416%, a clear separation rather than a photo finish. CV 30.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.91 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_per_w_set is an outlier: 994.4x slower than the field

abi_entry_form_madd_per_w_set (3.14 ms) is 994.4x the fastest (3.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_madd_null_entry is fastest but the noisiest (CV 30.7%)

abi_entry_form_madd_null_entry wins on median (3.16 us) yet has the highest variance (CV 30.7%), while abi_entry_form_madd_dispatch_table is the steadiest (CV 6.1%, 2.93 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set} (91416% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set} with a 91416% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 994.4x the fastest

Fastest abi_entry_form_madd_null_entry (3.16 us) to slowest abi_entry_form_madd_per_w_set (3.14 ms): 994.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 3156.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 994.36x (fastest 3156.4 ns, slowest 3138634.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2993988ns | 2932137ns | 2773535ns | 2918094ns | 3218055ns | -3.28% |
| abi_entry_form_madd_null_entry | 6224ns | 5633ns | 5362ns | 5572ns | 7633ns | -99.80% |
| abi_entry_form_madd_per_w_set | 3219357ns | 3143546ns | 2935838ns | 3090806ns | 3553945ns | +4.00% |
| abi_entry_form_madd_runtime_w | 3095505ns | 2917875ns | 2825682ns | 2897962ns | 3526732ns | base |
| abi_entry_form_madd_scalar_anchor | 3065358ns | 2893408ns | 2804284ns | 2884422ns | 3467299ns | -0.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2989188ns | 2769760ns | 3212885ns | -3.28% | 0.000 |
| abi_entry_form_madd_null_entry | 3565ns | 3027ns | 4480ns | -99.88% | 0.002 |
| abi_entry_form_madd_per_w_set | 3214093ns | 2930902ns | 3548060ns | +4.00% | 0.000 |
| abi_entry_form_madd_runtime_w | 3090519ns | 2821092ns | 3521066ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 3060195ns | 2800035ns | 3461049ns | -0.98% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 99711.2 | 2985300.4 | 2989187.7 | n/a |
| abi_entry_form_madd_null_entry | 34641.3 | 3769.1 | 3565.3 | n/a |
| abi_entry_form_madd_per_w_set | 112068.0 | 3244360.7 | 3214093.2 | n/a |
| abi_entry_form_madd_runtime_w | 111088.2 | 3089418.9 | 3090519.4 | n/a |
| abi_entry_form_madd_scalar_anchor | 117283.3 | 3076546.5 | 3060194.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.003 | 95.9% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2993988ns | 2993988ns | -3.28% |
| abi_entry_form_madd_null_entry | 6224ns | 6224ns | -99.80% |
| abi_entry_form_madd_per_w_set | 3219357ns | 3219357ns | +4.00% |
| abi_entry_form_madd_runtime_w | 3095505ns | 3095505ns | base |
| abi_entry_form_madd_scalar_anchor | 3065358ns | 3065358ns | -0.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2913093ns | base | --- | [2837399, 3521066] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2927162ns | no significant difference | [-446153, +177109]ns | [2827516, 3212885] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_null_entry | 3156ns | -2909971.5ns (-99.9%) | [-3516586, -2834305]ns | [3059, 4480] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 3138634ns | no significant difference | [-141402, +334468]ns | [2955586, 3548060] | no | 1.0000 | 0.6875 | 0 |
| abi_entry_form_madd_scalar_anchor | 2888641ns | no significant difference | [-191303, +128870]ns | [2830894, 3461049] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 4039915ns | -17.8% | -99.9% | -5.2% | -6.0% |
| 2 | 2821092ns | +4.7% | -99.9% | +5.6% | +2.7% |
| 3 | 3002218ns | -3.9% | -99.9% | -2.4% | -4.7% |
| 4 | 2943124ns | -5.9% | -99.9% | +11.0% | +6.2% |
| 5 | 2853706ns | +1.6% | -99.9% | +6.9% | +0.9% |
| 6 | 2883061ns | +7.7% | -99.9% | +11.9% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.125 | ok |
| abi_entry_form_madd_null_entry | -0.057 | ok |
| abi_entry_form_madd_per_w_set | -0.191 | ok |
| abi_entry_form_madd_runtime_w | -0.122 | ok |
| abi_entry_form_madd_scalar_anchor | -0.091 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 2/6, lost 4/6
- **abi_entry_form_madd_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 9073788.8ns | 2989187.7ns | 303.6% | HIGH |
| abi_entry_form_madd_null_entry | 133193.0ns | 3565.3ns | 3735.8% | HIGH |
| abi_entry_form_madd_per_w_set | 9847406.3ns | 3214093.2ns | 306.4% | HIGH |
| abi_entry_form_madd_runtime_w | 9480905.3ns | 3090519.4ns | 306.8% | HIGH |
| abi_entry_form_madd_scalar_anchor | 9366080.1ns | 3060194.7ns | 306.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2769760.4-3212885.0 ns)
  2769760.4 |####################
  2791916.6 |
  2814072.9 |
  2836229.1 |
  2858385.3 |
  2880541.5 |########################################
  2902697.8 |
  2924854.0 |
  2947010.2 |####################
  2969166.5 |
  2991322.7 |
  3013478.9 |
  3035635.2 |
  3057791.4 |
  3079947.6 |
  3102103.9 |####################
  3124260.1 |
  3146416.3 |
  3168572.5 |
  3190728.8 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 3027.1-4480.4 ns)
   3027.1 |########################################
   3099.8 |
   3172.4 |##########################
   3245.1 |
   3317.8 |
   3390.4 |
   3463.1 |
   3535.8 |
   3608.4 |
   3681.1 |
   3753.8 |
   3826.4 |
   3899.1 |
   3971.7 |
   4044.4 |
   4117.1 |
   4189.7 |
   4262.4 |
   4335.1 |
   4407.7 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2930902.1-3548060.0 ns)
  2930902.1 |########################################
  2961760.0 |########################################
  2992617.9 |
  3023475.8 |########################################
  3054333.7 |
  3085191.6 |
  3116049.5 |
  3146907.3 |
  3177765.2 |
  3208623.1 |########################################
  3239481.0 |########################################
  3270338.9 |
  3301196.8 |
  3332054.7 |
  3362912.6 |
  3393770.5 |
  3424628.4 |
  3455486.3 |
  3486344.2 |
  3517202.1 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2821092.1-3521066.5 ns)
  2821092.1 |########################################
  2856090.8 |####################
  2891089.5 |
  2926088.3 |####################
  2961087.0 |
  2996085.7 |####################
  3031084.4 |
  3066083.1 |
  3101081.8 |
  3136080.6 |
  3171079.3 |
  3206078.0 |
  3241076.7 |
  3276075.4 |
  3311074.1 |
  3346072.9 |
  3381071.6 |
  3416070.3 |
  3451069.0 |
  3486067.7 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2800035.4-3461049.1 ns)
  2800035.4 |####################
  2833086.1 |####################
  2866136.8 |########################################
  2899187.5 |
  2932238.1 |
  2965288.8 |
  2998339.5 |
  3031390.2 |
  3064440.9 |
  3097491.6 |####################
  3130542.3 |
  3163593.0 |
  3196643.6 |
  3229694.3 |
  3262745.0 |
  3295795.7 |
  3328846.4 |
  3361897.1 |
  3394947.8 |
  3427998.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: CV=27.2% (high variance, measurements may be unstable)
- **abi_entry_form_madd_null_entry**: bridge=3858.9% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=304.3% of algo (FFI overhead may distort results)

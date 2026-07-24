# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 107979% faster than the next best (abi_entry_form_madd_scalar_anchor)

abi_entry_form_madd_null_entry (2.55 us) leads abi_entry_form_madd_scalar_anchor (2.75 ms) by 107979%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.76 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_per_w_set is an outlier: 1134.0x slower than the field

abi_entry_form_madd_per_w_set (2.89 ms) is 1134.0x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_madd_dispatch_table shows warm-up / thermal drift (autocorr +0.56)

abi_entry_form_madd_dispatch_table's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set} (107979% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set} with a 107979% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1134.0x the fastest

Fastest abi_entry_form_madd_null_entry (2.55 us) to slowest abi_entry_form_madd_per_w_set (2.89 ms): 1134.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 2545.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1134.01x (fastest 2545.4 ns, slowest 2886506.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2931145ns | 2880808ns | 2753055ns | 2839954ns | 3156977ns | -1.45% |
| abi_entry_form_madd_null_entry | 5077ns | 5037ns | 4723ns | 4954ns | 5438ns | -99.83% |
| abi_entry_form_madd_per_w_set | 3001710ns | 2890766ns | 2747146ns | 2850163ns | 3356311ns | +0.92% |
| abi_entry_form_madd_runtime_w | 2974356ns | 2767608ns | 2749267ns | 2762298ns | 3404989ns | base |
| abi_entry_form_madd_scalar_anchor | 2966144ns | 2754936ns | 2746895ns | 2754294ns | 3393545ns | -0.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2926638ns | 2749165ns | 3151790ns | -1.45% | 0.000 |
| abi_entry_form_madd_null_entry | 2617ns | 2436ns | 2844ns | -99.91% | 0.024 |
| abi_entry_form_madd_per_w_set | 2997250ns | 2743362ns | 3351101ns | +0.92% | 0.000 |
| abi_entry_form_madd_runtime_w | 2969808ns | 2745458ns | 3398918ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 2961794ns | 2743163ns | 3388308ns | -0.27% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 89917.1 | 2908183.1 | 2926638.3 | 0 |
| abi_entry_form_madd_null_entry | 31511.5 | 2963.7 | 2617.3 | n/a |
| abi_entry_form_madd_per_w_set | 95055.6 | 3022300.6 | 2997249.6 | n/a |
| abi_entry_form_madd_runtime_w | 87952.1 | 2977058.4 | 2969808.2 | n/a |
| abi_entry_form_madd_scalar_anchor | 90117.8 | 2991342.4 | 2961793.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.025 | 95.7% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2931145ns | 2931145ns | -1.45% |
| abi_entry_form_madd_null_entry | 5077ns | 5077ns | -99.83% |
| abi_entry_form_madd_per_w_set | 3001710ns | 3001710ns | +0.92% |
| abi_entry_form_madd_runtime_w | 2974356ns | 2974356ns | base |
| abi_entry_form_madd_scalar_anchor | 2966144ns | 2966144ns | -0.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2763751ns | base | --- | [2746755, 3398918] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2876518ns | no significant difference | [-344863, +227498]ns | [2751607, 3151790] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_null_entry | 2545ns | -2761254.2ns (-99.9%) | [-3396074, -2744245]ns | [2462, 2844] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2886506ns | no significant difference | [-99300, +139751]ns | [2754141, 3351101] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_scalar_anchor | 2751049ns | no significant difference | [-284986, +264049]ns | [2746024, 3388308] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2748052ns | +15.9% | -99.9% | +3.8% | -0.2% |
| 2 | 3251381ns | -4.1% | -99.9% | +2.8% | +16.0% |
| 3 | 3546455ns | -15.7% | -99.9% | -5.3% | -15.3% |
| 4 | 2776523ns | -1.0% | -99.9% | -0.4% | -1.0% |
| 5 | 2750980ns | +0.1% | -99.9% | -0.3% | -0.0% |
| 6 | 2745458ns | +0.7% | -99.9% | +6.4% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.560 | HIGH+ (drift/warm-up) |
| abi_entry_form_madd_null_entry | 0.307 | moderate+ |
| abi_entry_form_madd_per_w_set | 0.176 | ok |
| abi_entry_form_madd_runtime_w | 0.134 | ok |
| abi_entry_form_madd_scalar_anchor | -0.074 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_madd_scalar_anchor**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 8849285.2ns | 2926638.3ns | 302.4% | HIGH |
| abi_entry_form_madd_null_entry | 117697.0ns | 2617.3ns | 4496.9% | HIGH |
| abi_entry_form_madd_per_w_set | 9133154.1ns | 2997249.6ns | 304.7% | HIGH |
| abi_entry_form_madd_runtime_w | 9049003.5ns | 2969808.2ns | 304.7% | HIGH |
| abi_entry_form_madd_scalar_anchor | 9023799.1ns | 2961793.6ns | 304.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2749165.0-3151790.2 ns)
  2749165.0 |########################################
  2769296.3 |
  2789427.5 |
  2809558.8 |
  2829690.0 |
  2849821.3 |
  2869952.6 |
  2890083.8 |
  2910215.1 |
  2930346.3 |
  2950477.6 |
  2970608.9 |#############
  2990740.1 |
  3010871.4 |
  3031002.6 |
  3051133.9 |
  3071265.2 |
  3091396.4 |
  3111527.7 |#############
  3131658.9 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 2436.2-2844.0 ns)
   2436.2 |########################################
   2456.6 |
   2477.0 |########################################
   2497.4 |########################################
   2517.8 |
   2538.1 |
   2558.5 |
   2578.9 |########################################
   2599.3 |
   2619.7 |
   2640.1 |
   2660.5 |
   2680.9 |
   2701.3 |
   2721.7 |
   2742.1 |
   2762.4 |
   2782.8 |
   2803.2 |########################################
   2823.6 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2743362.1-3351101.5 ns)
  2743362.1 |########################################
  2773749.1 |
  2804136.0 |
  2834523.0 |####################
  2864910.0 |
  2895296.9 |####################
  2925683.9 |
  2956070.9 |
  2986457.8 |
  3016844.8 |
  3047231.8 |
  3077618.7 |
  3108005.7 |
  3138392.7 |
  3168779.6 |
  3199166.6 |
  3229553.6 |
  3259940.5 |
  3290327.5 |
  3320714.5 |####################
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2745457.9-3398918.1 ns)
  2745457.9 |########################################
  2778130.9 |
  2810803.9 |
  2843476.9 |
  2876149.9 |
  2908823.0 |
  2941496.0 |
  2974169.0 |
  3006842.0 |
  3039515.0 |
  3072188.0 |
  3104861.0 |
  3137534.0 |
  3170207.0 |
  3202880.0 |
  3235553.0 |##########
  3268226.1 |
  3300899.1 |
  3333572.1 |
  3366245.1 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2743163.3-3388307.7 ns)
  2743163.3 |########################################
  2775420.5 |
  2807677.7 |
  2839935.0 |
  2872192.2 |
  2904449.4 |
  2936706.6 |
  2968963.8 |
  3001221.1 |##########
  3033478.3 |
  3065735.5 |
  3097992.7 |
  3130249.9 |
  3162507.2 |
  3194764.4 |
  3227021.6 |
  3259278.8 |
  3291536.0 |
  3323793.3 |
  3356050.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **abi_entry_form_madd_dispatch_table**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=4562.9% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=304.0% of algo (FFI overhead may distort results)

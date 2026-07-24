# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 55729% faster than the next best (abi_entry_form_madd_per_w_set)

abi_entry_form_madd_null_entry (5.02 us) leads abi_entry_form_madd_per_w_set (2.80 ms) by 55729%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.81 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_dispatch_table is an outlier: 562.4x slower than the field

abi_entry_form_madd_dispatch_table (2.82 ms) is 562.4x the fastest (5.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_madd_runtime_w shows alternating (throttle bounce) (autocorr -0.56)

abi_entry_form_madd_runtime_w's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_per_w_set, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table} (55729% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_per_w_set, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table} with a 55729% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 562.4x the fastest

Fastest abi_entry_form_madd_null_entry (5.02 us) to slowest abi_entry_form_madd_dispatch_table (2.82 ms): 562.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 5020.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 562.38x (fastest 5020.2 ns, slowest 2823253.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2947531ns | 2827503ns | 2764922ns | 2825744ns | 3221515ns | +4.20% |
| abi_entry_form_madd_null_entry | 7528ns | 7464ns | 7092ns | 7433ns | 7888ns | -99.73% |
| abi_entry_form_madd_per_w_set | 2883286ns | 2806828ns | 2740168ns | 2804034ns | 3073724ns | +1.93% |
| abi_entry_form_madd_runtime_w | 2828713ns | 2821347ns | 2749094ns | 2818788ns | 2883410ns | base |
| abi_entry_form_madd_scalar_anchor | 2827646ns | 2814757ns | 2760828ns | 2810267ns | 2887124ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2943087ns | 2760925ns | 3216551ns | +4.20% | 0.000 |
| abi_entry_form_madd_null_entry | 5031ns | 4861ns | 5178ns | -99.82% | 0.000 |
| abi_entry_form_madd_per_w_set | 2879123ns | 2736662ns | 3069295ns | +1.94% | 0.000 |
| abi_entry_form_madd_runtime_w | 2824449ns | 2745147ns | 2879014ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 2823305ns | 2756798ns | 2882449ns | -0.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 99628.2 | 2932671.1 | 2943087.2 | n/a |
| abi_entry_form_madd_null_entry | 31204.3 | 5127.4 | 5031.4 | n/a |
| abi_entry_form_madd_per_w_set | 95635.7 | 2814654.5 | 2879123.5 | n/a |
| abi_entry_form_madd_runtime_w | 92022.8 | 2845288.2 | 2824449.1 | n/a |
| abi_entry_form_madd_scalar_anchor | 99326.0 | 2826017.8 | 2823304.9 | 6 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_madd_null_entry | 0.000 | 96.8% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.2% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.2% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2947531ns | 2947531ns | +4.20% |
| abi_entry_form_madd_null_entry | 7528ns | 7528ns | -99.73% |
| abi_entry_form_madd_per_w_set | 2883286ns | 2883286ns | +1.93% |
| abi_entry_form_madd_runtime_w | 2828713ns | 2828713ns | base |
| abi_entry_form_madd_scalar_anchor | 2827646ns | 2827646ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2817138ns | base | --- | [2777196, 2879014] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2823253ns | +15162.5ns (+0.5%) | [+3214, +337538]ns | [2789457, 3216551] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_madd_null_entry | 5020ns | -2812241.7ns (-99.8%) | [-2873849, -2772162]ns | [4896, 5178] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2802705ns | no significant difference | [-21741, +196691]ns | [2765370, 3069295] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_madd_scalar_anchor | 2810522ns | no significant difference | [-67078, +58987]ns | [2776944, 2882449] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2816975ns | +0.2% | -99.8% | -0.7% | +1.0% |
| 2 | 2829950ns | +8.9% | -99.8% | -0.5% | +3.2% |
| 3 | 2817300ns | +0.0% | -99.8% | -0.8% | -0.1% |
| 4 | 2809245ns | +0.5% | -99.8% | -0.0% | -0.1% |
| 5 | 2928078ns | +14.4% | -99.8% | +13.5% | -4.5% |
| 6 | 2745147ns | +0.6% | -99.8% | -0.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.540 | HIGH- (thermal bounce) |
| abi_entry_form_madd_null_entry | -0.009 | ok |
| abi_entry_form_madd_per_w_set | -0.325 | moderate- |
| abi_entry_form_madd_runtime_w | -0.561 | HIGH- (thermal bounce) |
| abi_entry_form_madd_scalar_anchor | 0.239 | moderate+ |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 0/6, lost 5/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 4/6, lost 1/6
- **abi_entry_form_madd_scalar_anchor**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 8975883.3ns | 2943087.2ns | 305.0% | HIGH |
| abi_entry_form_madd_null_entry | 128896.6ns | 5031.4ns | 2561.9% | HIGH |
| abi_entry_form_madd_per_w_set | 8556879.1ns | 2879123.5ns | 297.2% | HIGH |
| abi_entry_form_madd_runtime_w | 8605278.8ns | 2824449.1ns | 304.7% | HIGH |
| abi_entry_form_madd_scalar_anchor | 8567600.6ns | 2823304.9ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2760924.6-3216551.5 ns)
  2760924.6 |#############
  2783705.9 |
  2806487.3 |########################################
  2829268.6 |
  2852050.0 |
  2874831.3 |
  2897612.7 |
  2920394.0 |
  2943175.3 |
  2965956.7 |
  2988738.0 |
  3011519.4 |
  3034300.7 |
  3057082.1 |
  3079863.4 |#############
  3102644.7 |
  3125426.1 |
  3148207.4 |
  3170988.8 |
  3193770.1 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 4861.2-5178.1 ns)
   4861.2 |########################################
   4877.0 |
   4892.9 |
   4908.7 |
   4924.6 |########################################
   4940.4 |
   4956.3 |
   4972.1 |
   4988.0 |
   5003.8 |########################################
   5019.7 |########################################
   5035.5 |
   5051.4 |########################################
   5067.2 |
   5083.1 |
   5098.9 |
   5114.8 |
   5130.6 |
   5146.5 |
   5162.3 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2736661.7-3069295.2 ns)
  2736661.7 |####################
  2753293.4 |
  2769925.1 |
  2786556.7 |########################################
  2803188.4 |########################################
  2819820.1 |
  2836451.8 |
  2853083.4 |
  2869715.1 |
  2886346.8 |
  2902978.5 |
  2919610.1 |
  2936241.8 |
  2952873.5 |
  2969505.2 |
  2986136.8 |
  3002768.5 |
  3019400.2 |
  3036031.9 |
  3052663.5 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2745147.1-2879014.0 ns)
  2745147.1 |####################
  2751840.4 |
  2758533.8 |
  2765227.1 |
  2771920.5 |
  2778613.8 |
  2785307.2 |
  2792000.5 |
  2798693.8 |
  2805387.2 |####################
  2812080.5 |########################################
  2818773.9 |
  2825467.2 |####################
  2832160.6 |
  2838853.9 |
  2845547.2 |
  2852240.6 |
  2858933.9 |
  2865627.3 |
  2872320.6 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2756797.9-2882449.2 ns)
  2756797.9 |########################################
  2763080.5 |
  2769363.0 |
  2775645.6 |
  2781928.1 |
  2788210.7 |
  2794493.3 |########################################
  2800775.8 |########################################
  2807058.4 |
  2813341.0 |########################################
  2819623.5 |
  2825906.1 |
  2832188.7 |
  2838471.2 |
  2844753.8 |########################################
  2851036.3 |
  2857318.9 |
  2863601.5 |
  2869884.0 |
  2876166.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=2578.4% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=304.3% of algo (FFI overhead may distort results)

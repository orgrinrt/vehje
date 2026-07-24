# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 100272% faster than the next best (abi_entry_form_madd_scalar_anchor)

abi_entry_form_madd_null_entry (2.74 us) leads abi_entry_form_madd_scalar_anchor (2.75 ms) by 100272%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.76 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_dispatch_table is an outlier: 1011.5x slower than the field

abi_entry_form_madd_dispatch_table (2.77 ms) is 1011.5x the fastest (2.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_per_w_set, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table} (100272% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_per_w_set, abi_entry_form_madd_runtime_w, abi_entry_form_madd_dispatch_table} with a 100272% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1011.5x the fastest

Fastest abi_entry_form_madd_null_entry (2.74 us) to slowest abi_entry_form_madd_dispatch_table (2.77 ms): 1011.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 2738.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1011.46x (fastest 2738.9 ns, slowest 2770325.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2851281ns | 2774535ns | 2741601ns | 2766886ns | 3032713ns | +2.46% |
| abi_entry_form_madd_null_entry | 5099ns | 5074ns | 4975ns | 5052ns | 5231ns | -99.82% |
| abi_entry_form_madd_per_w_set | 2827906ns | 2762046ns | 2753281ns | 2759767ns | 2967426ns | +1.62% |
| abi_entry_form_madd_runtime_w | 2782782ns | 2771381ns | 2763127ns | 2771132ns | 2810086ns | base |
| abi_entry_form_madd_scalar_anchor | 2769715ns | 2752845ns | 2740340ns | 2750657ns | 2812992ns | -0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2847148ns | 2737813ns | 3028245ns | +2.46% | 0.000 |
| abi_entry_form_madd_null_entry | 2769ns | 2711ns | 2844ns | -99.90% | 0.046 |
| abi_entry_form_madd_per_w_set | 2823916ns | 2749623ns | 2962879ns | +1.63% | 0.000 |
| abi_entry_form_madd_runtime_w | 2778732ns | 2759058ns | 2805786ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 2765937ns | 2736765ns | 2809047ns | -0.46% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 86221.1 | 2813434.2 | 2847147.8 | n/a |
| abi_entry_form_madd_null_entry | 29036.6 | 2836.5 | 2768.7 | n/a |
| abi_entry_form_madd_per_w_set | 83178.5 | 2890929.9 | 2823915.8 | 0 |
| abi_entry_form_madd_runtime_w | 84461.2 | 2772116.6 | 2778732.0 | n/a |
| abi_entry_form_madd_scalar_anchor | 82295.8 | 2760845.3 | 2765937.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.047 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.047 | 99.0% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2851281ns | 2851281ns | +2.46% |
| abi_entry_form_madd_null_entry | 5099ns | 5099ns | -99.82% |
| abi_entry_form_madd_per_w_set | 2827906ns | 2827906ns | +1.62% |
| abi_entry_form_madd_runtime_w | 2782782ns | 2782782ns | base |
| abi_entry_form_madd_scalar_anchor | 2769715ns | 2769715ns | -0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2767525ns | base | --- | [2762885, 2805786] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2770326ns | no significant difference | [-47765, +246368]ns | [2742873, 3028245] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_null_entry | 2739ns | -2764801.3ns (-99.9%) | [-2803022, -2760067]ns | [2724, 2844] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2758342ns | no significant difference | [-55260, +197933]ns | [2750526, 2962879] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_madd_scalar_anchor | 2749133ns | no significant difference | [-59228, +44439]ns | [2739631, 2809047] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2766744ns | +17.0% | -99.9% | +14.3% | +3.4% |
| 2 | 2768305ns | +0.6% | -99.9% | -0.4% | -0.8% |
| 3 | 2797010ns | +0.8% | -99.9% | -1.7% | -2.2% |
| 4 | 2814563ns | -2.4% | -99.9% | -2.2% | -2.1% |
| 5 | 2759058ns | -0.1% | -99.9% | -0.0% | -0.2% |
| 6 | 2766712ns | -1.0% | -99.9% | -0.1% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.007 | ok |
| abi_entry_form_madd_null_entry | 0.091 | ok |
| abi_entry_form_madd_per_w_set | -0.025 | ok |
| abi_entry_form_madd_runtime_w | 0.050 | ok |
| abi_entry_form_madd_scalar_anchor | -0.059 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 2/6, lost 3/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 4/6, lost 1/6
- **abi_entry_form_madd_scalar_anchor**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 8521148.7ns | 2847147.8ns | 299.3% | HIGH |
| abi_entry_form_madd_null_entry | 121663.0ns | 2768.7ns | 4394.3% | HIGH |
| abi_entry_form_madd_per_w_set | 8689875.8ns | 2823915.8ns | 307.7% | HIGH |
| abi_entry_form_madd_runtime_w | 8402411.2ns | 2778732.0ns | 302.4% | HIGH |
| abi_entry_form_madd_scalar_anchor | 8378097.9ns | 2765937.1ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2737812.9-3028245.2 ns)
  2737812.9 |########################################
  2752334.5 |####################
  2766856.1 |
  2781377.7 |####################
  2795899.4 |
  2810421.0 |####################
  2824942.6 |
  2839464.2 |
  2853985.8 |
  2868507.4 |
  2883029.0 |
  2897550.7 |
  2912072.3 |
  2926593.9 |
  2941115.5 |
  2955637.1 |
  2970158.7 |
  2984680.4 |
  2999202.0 |
  3013723.6 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 2710.8-2843.5 ns)
   2710.8 |####################
   2717.4 |
   2724.1 |
   2730.7 |####################
   2737.3 |########################################
   2744.0 |
   2750.6 |
   2757.2 |
   2763.9 |
   2770.5 |
   2777.2 |
   2783.8 |
   2790.4 |####################
   2797.1 |
   2803.7 |
   2810.3 |
   2817.0 |
   2823.6 |
   2830.2 |
   2836.9 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2749622.9-2962879.1 ns)
  2749622.9 |########################################
  2760285.7 |##########
  2770948.5 |
  2781611.3 |
  2792274.1 |
  2802937.0 |
  2813599.8 |
  2824262.6 |
  2834925.4 |
  2845588.2 |
  2856251.0 |
  2866913.8 |
  2877576.6 |
  2888239.5 |
  2898902.3 |
  2909565.1 |
  2920227.9 |
  2930890.7 |
  2941553.5 |
  2952216.3 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2759057.5-2805786.5 ns)
  2759057.5 |#############
  2761393.9 |
  2763730.4 |
  2766066.8 |########################################
  2768403.3 |
  2770739.7 |
  2773076.2 |
  2775412.6 |
  2777749.1 |
  2780085.5 |
  2782422.0 |
  2784758.4 |
  2787094.9 |
  2789431.3 |
  2791767.8 |
  2794104.2 |
  2796440.7 |#############
  2798777.1 |
  2801113.6 |
  2803450.0 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2736764.6-2809046.9 ns)
  2736764.6 |########################################
  2740378.7 |########################################
  2743992.8 |########################################
  2747606.9 |
  2751221.1 |########################################
  2754835.2 |########################################
  2758449.3 |
  2762063.4 |
  2765677.5 |
  2769291.6 |
  2772905.8 |
  2776519.9 |
  2780134.0 |
  2783748.1 |
  2787362.2 |
  2790976.3 |
  2794590.4 |
  2798204.6 |
  2801818.7 |
  2805432.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=4437.7% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=302.5% of algo (FFI overhead may distort results)

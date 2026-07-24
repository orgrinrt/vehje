# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_madd_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_madd_runtime_w has the worst median (2.85 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_madd_null_entry at 2.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_madd_null_entry dominates: 118554% faster than the next best (abi_entry_form_madd_dispatch_table)

abi_entry_form_madd_null_entry (2.34 us) leads abi_entry_form_madd_dispatch_table (2.78 ms) by 118554%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -2.85 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_runtime_w is an outlier: 1215.8x slower than the field

abi_entry_form_madd_runtime_w (2.85 ms) is 1215.8x the fastest (2.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w} (118554% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_dispatch_table, abi_entry_form_madd_per_w_set, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_runtime_w} with a 118554% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1215.8x the fastest

Fastest abi_entry_form_madd_null_entry (2.34 us) to slowest abi_entry_form_madd_runtime_w (2.85 ms): 1215.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 2343.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1215.84x (fastest 2343.6 ns, slowest 2849374.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2940697ns | 2784905ns | 2763133ns | 2782995ns | 3266032ns | +1.43% |
| abi_entry_form_madd_null_entry | 4671ns | 4680ns | 4525ns | 4663ns | 4756ns | -99.84% |
| abi_entry_form_madd_per_w_set | 2862906ns | 2798016ns | 2767492ns | 2792379ns | 3016404ns | -1.25% |
| abi_entry_form_madd_runtime_w | 2899264ns | 2854130ns | 2747377ns | 2829448ns | 3079931ns | base |
| abi_entry_form_madd_scalar_anchor | 2834795ns | 2810599ns | 2731732ns | 2784335ns | 2962017ns | -2.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2936302ns | 2759233ns | 3261054ns | +1.44% | 0.000 |
| abi_entry_form_madd_null_entry | 2330ns | 2257ns | 2370ns | -99.92% | 0.014 |
| abi_entry_form_madd_per_w_set | 2858715ns | 2764076ns | 3011637ns | -1.24% | 0.000 |
| abi_entry_form_madd_runtime_w | 2894632ns | 2743908ns | 3074536ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 2830732ns | 2728037ns | 2957561ns | -2.21% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 91144.8 | 2940370.3 | 2936302.5 | 0 |
| abi_entry_form_madd_null_entry | 29263.7 | 2450.3 | 2329.5 | n/a |
| abi_entry_form_madd_per_w_set | 86897.6 | 2848377.9 | 2858715.0 | n/a |
| abi_entry_form_madd_runtime_w | 91359.8 | 2934723.7 | 2894631.9 | n/a |
| abi_entry_form_madd_scalar_anchor | 86244.0 | 2836054.3 | 2830731.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.014 | 96.3% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 2940697ns | 2940697ns | +1.43% |
| abi_entry_form_madd_null_entry | 4671ns | 4671ns | -99.84% |
| abi_entry_form_madd_per_w_set | 2862906ns | 2862906ns | -1.25% |
| abi_entry_form_madd_runtime_w | 2899264ns | 2899264ns | base |
| abi_entry_form_madd_scalar_anchor | 2834795ns | 2834795ns | -2.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 2849374ns | base | --- | [2759986, 3074536] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 2780709ns | no significant difference | [-111983, +250707]ns | [2767145, 3261054] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_null_entry | 2344ns | -2847014.5ns (-99.9%) | [-3072182, -2757711]ns | [2275, 2370] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2793912ns | no significant difference | [-182789, +83897]ns | [2770596, 3011637] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_scalar_anchor | 2806323ns | no significant difference | [-178759, +23229]ns | [2728310, 2957561] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2812778ns | -1.2% | -99.9% | +4.9% | -1.4% |
| 2 | 2885971ns | -3.8% | -99.9% | -3.6% | -1.2% |
| 3 | 3111192ns | +15.6% | -99.9% | -1.2% | -1.5% |
| 4 | 2776064ns | +0.2% | -99.9% | +1.1% | +2.2% |
| 5 | 2743908ns | +0.6% | -99.9% | +0.7% | -0.6% |
| 6 | 3037880ns | -3.7% | -99.9% | -8.6% | -10.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.284 | moderate- |
| abi_entry_form_madd_null_entry | 0.023 | ok |
| abi_entry_form_madd_per_w_set | -0.283 | moderate- |
| abi_entry_form_madd_runtime_w | -0.275 | moderate- |
| abi_entry_form_madd_scalar_anchor | 0.194 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_madd_scalar_anchor**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 8997620.6ns | 2936302.5ns | 306.4% | HIGH |
| abi_entry_form_madd_null_entry | 118316.9ns | 2329.5ns | 5079.0% | HIGH |
| abi_entry_form_madd_per_w_set | 8693984.0ns | 2858715.0ns | 304.1% | HIGH |
| abi_entry_form_madd_runtime_w | 8811035.7ns | 2894631.9ns | 304.4% | HIGH |
| abi_entry_form_madd_scalar_anchor | 8593529.7ns | 2830731.7ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2759233.3-3261053.8 ns)
  2759233.3 |########################################
  2784324.3 |
  2809415.3 |
  2834506.4 |
  2859597.4 |
  2884688.4 |
  2909779.4 |##########
  2934870.5 |
  2959961.5 |
  2985052.5 |
  3010143.5 |
  3035234.5 |
  3060325.6 |
  3085416.6 |
  3110507.6 |
  3135598.6 |
  3160689.7 |
  3185780.7 |
  3210871.7 |
  3235962.7 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 2256.7-2370.0 ns)
   2256.7 |########################################
   2262.4 |
   2268.0 |
   2273.7 |
   2279.4 |
   2285.0 |
   2290.7 |########################################
   2296.4 |
   2302.0 |
   2307.7 |
   2313.3 |
   2319.0 |
   2324.7 |
   2330.3 |########################################
   2336.0 |
   2341.7 |
   2347.3 |########################################
   2353.0 |
   2358.7 |
   2364.3 |########################################
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2764076.2-3011637.0 ns)
  2764076.2 |####################
  2776454.2 |########################################
  2788832.3 |
  2801210.3 |####################
  2813588.4 |
  2825966.4 |
  2838344.5 |
  2850722.5 |
  2863100.5 |
  2875478.6 |
  2887856.6 |
  2900234.7 |
  2912612.7 |
  2924990.8 |
  2937368.8 |
  2949746.8 |####################
  2962124.9 |
  2974502.9 |
  2986881.0 |
  2999259.0 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2743907.5-3074535.7 ns)
  2743907.5 |########################################
  2760438.9 |########################################
  2776970.3 |
  2793501.7 |
  2810033.1 |########################################
  2826564.5 |
  2843095.9 |
  2859627.4 |
  2876158.8 |########################################
  2892690.2 |
  2909221.6 |
  2925753.0 |
  2942284.4 |
  2958815.8 |
  2975347.2 |
  2991878.6 |
  3008410.0 |
  3024941.4 |########################################
  3041472.8 |
  3058004.2 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2728036.7-2957561.5 ns)
  2728036.7 |########################################
  2739512.9 |
  2750989.2 |
  2762465.4 |
  2773941.7 |####################
  2785417.9 |
  2796894.1 |
  2808370.4 |
  2819846.6 |
  2831322.8 |####################
  2842799.1 |####################
  2854275.3 |
  2865751.6 |
  2877227.8 |
  2888704.0 |
  2900180.3 |
  2911656.5 |
  2923132.7 |
  2934609.0 |
  2946085.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=5074.0% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=308.3% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=304.2% of algo (FFI overhead may distort results)

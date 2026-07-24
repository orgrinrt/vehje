# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 107538% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (2.51 us) leads abi_zig_entry_madd_zig_anchor (2.70 ms) by 107538%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.71 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 1275.0x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.20 ms) is 1275.0x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (107538% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 107538% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1275.0x the fastest

Fastest abi_zig_entry_madd_zig_null (2.51 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.20 ms): 1275.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_zig_entry_madd_zig_dispatch's edge over baseline is significant but tiny (-1 ns, 0.00%)

abi_zig_entry_madd_zig_dispatch differs from baseline abi_zig_entry_madd_zig_runtime_w by -1 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 2506.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1274.95x (fastest 2506.4 ns, slowest 3195606.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2787883ns | 2701089ns | 2696459ns | 2699810ns | 2965703ns | +2.74% |
| abi_zig_entry_madd_zig_dispatch | 2720474ns | 2711586ns | 2703697ns | 2710290ns | 2744139ns | +0.25% |
| abi_zig_entry_madd_zig_null | 4860ns | 4843ns | 4736ns | 4813ns | 4994ns | -99.82% |
| abi_zig_entry_madd_zig_per_w_set | 2733605ns | 2729707ns | 2724732ns | 2728539ns | 2745639ns | +0.74% |
| abi_zig_entry_madd_zig_runtime_w | 2713601ns | 2711132ns | 2704764ns | 2709676ns | 2723906ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3158415ns | 3193035ns | 3068765ns | 3155268ns | 3207960ns | +16.39% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3198694ns | 3199018ns | 3183623ns | 3198046ns | 3207200ns | +17.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2784556ns | 2693203ns | 2962089ns | +2.74% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2717036ns | 2700384ns | 2740446ns | +0.25% | 0.000 |
| abi_zig_entry_madd_zig_null | 2518ns | 2478ns | 2562ns | -99.91% | 0.006 |
| abi_zig_entry_madd_zig_per_w_set | 2730216ns | 2721583ns | 2742019ns | +0.73% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2710338ns | 2701749ns | 2720242ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3154974ns | 3065716ns | 3204353ns | +16.41% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3195213ns | 3180370ns | 3203498ns | +17.89% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 233172.4 | 2737789.0 | 2784555.6 | n/a |
| abi_zig_entry_madd_zig_dispatch | 225494.1 | 2719237.4 | 2717035.7 | n/a |
| abi_zig_entry_madd_zig_null | 157636.6 | 2717.5 | 2517.9 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 231916.6 | 2729473.2 | 2730215.5 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 222328.3 | 2710634.8 | 2710337.6 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 234176.0 | 3156416.4 | 3154974.4 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 231066.7 | 3194920.3 | 3195212.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.006 | 98.9% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2787883ns | 2787883ns | +2.74% |
| abi_zig_entry_madd_zig_dispatch | 2720474ns | 2720474ns | +0.25% |
| abi_zig_entry_madd_zig_null | 4860ns | 4860ns | -99.82% |
| abi_zig_entry_madd_zig_per_w_set | 2733605ns | 2733605ns | +0.74% |
| abi_zig_entry_madd_zig_runtime_w | 2713601ns | 2713601ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3158415ns | 3158415ns | +16.39% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3198694ns | 3198694ns | +17.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2708022ns | base | --- | [2702749, 2720242] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2697884ns | no significant difference | [-24911, +255333]ns | [2693694, 2962089] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2708281ns | no significant difference | [-11403, +31498]ns | [2702381, 2740446] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_madd_zig_null | 2506ns | -2705484.1ns (-99.9%) | [-2717742, -2700233]ns | [2485, 2562] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2726479ns | +19637.0ns (+0.7%) | [+7088, +32909]ns | [2722148, 2742019] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3189608ns | +469365.4ns (+17.3%) | [+368214, +496331]ns | [3070962, 3204353] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3195606ns | +483162.5ns (+17.8%) | [+474486, +496977]ns | [3186535, 3203498] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2701749ns | -0.1% | +0.1% | -99.9% | +0.7% | +13.5% | +18.3% |
| 2 | 2703748ns | +18.9% | -0.1% | -99.9% | +0.7% | +13.8% | +18.5% |
| 3 | 2728545ns | -1.3% | -0.6% | -99.9% | +0.0% | +16.9% | +17.4% |
| 4 | 2709763ns | -0.1% | +2.1% | -99.9% | +0.5% | +18.1% | +17.8% |
| 5 | 2711940ns | -0.5% | -0.3% | -99.9% | +1.0% | +17.6% | +17.8% |
| 6 | 2706280ns | -0.4% | +0.2% | -99.9% | +1.4% | +18.6% | +17.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.247 | moderate- |
| abi_zig_entry_madd_zig_dispatch | -0.156 | ok |
| abi_zig_entry_madd_zig_null | -0.465 | moderate- |
| abi_zig_entry_madd_zig_per_w_set | 0.277 | moderate+ |
| abi_zig_entry_madd_zig_runtime_w | -0.174 | ok |
| abi_zig_entry_madd_zig_tail_dispatch | 0.432 | moderate+ |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.142 | ok |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 4/6, lost 1/6
- **abi_zig_entry_madd_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 0/6, lost 5/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8516316.5ns | 2784555.6ns | 305.8% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8474990.8ns | 2717035.7ns | 311.9% | HIGH |
| abi_zig_entry_madd_zig_null | 303808.5ns | 2517.9ns | 12065.9% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8506629.6ns | 2730215.5ns | 311.6% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8436607.6ns | 2710337.6ns | 311.3% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9792000.1ns | 3154974.4ns | 310.4% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 9899694.4ns | 3195212.9ns | 309.8% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2693202.9-2962088.8 ns)
  2693202.9 |########################################
  2706647.2 |##########
  2720091.5 |
  2733535.8 |
  2746980.1 |
  2760424.4 |
  2773868.7 |
  2787312.9 |
  2800757.2 |
  2814201.5 |
  2827645.8 |
  2841090.1 |
  2854534.4 |
  2867978.7 |
  2881423.0 |
  2894867.3 |
  2908311.6 |
  2921755.9 |
  2935200.2 |
  2948644.5 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2700383.8-2740445.7 ns)
  2700383.8 |########################################
  2702386.9 |########################################
  2704390.0 |########################################
  2706393.1 |
  2708396.2 |
  2710399.3 |########################################
  2712402.4 |########################################
  2714405.4 |
  2716408.5 |
  2718411.6 |
  2720414.7 |
  2722417.8 |
  2724420.9 |
  2726424.0 |
  2728427.1 |
  2730430.2 |
  2732433.3 |
  2734436.4 |
  2736439.5 |
  2738442.6 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 2477.9-2562.5 ns)
   2477.9 |########################################
   2482.1 |
   2486.4 |
   2490.6 |########################################
   2494.8 |
   2499.1 |########################################
   2503.3 |
   2507.5 |########################################
   2511.7 |
   2516.0 |
   2520.2 |
   2524.4 |
   2528.7 |
   2532.9 |
   2537.1 |
   2541.3 |
   2545.6 |
   2549.8 |########################################
   2554.0 |
   2558.3 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2721583.3-2742019.2 ns)
  2721583.3 |####################
  2722605.1 |########################################
  2723626.9 |
  2724648.7 |
  2725670.5 |
  2726692.3 |
  2727714.1 |
  2728735.9 |
  2729757.7 |####################
  2730779.5 |
  2731801.2 |
  2732823.0 |
  2733844.8 |
  2734866.6 |
  2735888.4 |
  2736910.2 |
  2737932.0 |
  2738953.8 |####################
  2739975.6 |
  2740997.4 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2701749.2-2720242.5 ns)
  2701749.2 |########################################
  2702673.9 |
  2703598.5 |########################################
  2704523.2 |
  2705447.9 |########################################
  2706372.5 |
  2707297.2 |
  2708221.9 |
  2709146.5 |########################################
  2710071.2 |
  2710995.9 |
  2711920.5 |########################################
  2712845.2 |
  2713769.8 |
  2714694.5 |
  2715619.2 |
  2716543.8 |
  2717468.5 |
  2718393.2 |
  2719317.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3065715.8-3204352.9 ns)
  3065715.8 |####################
  3072647.7 |####################
  3079579.5 |
  3086511.4 |
  3093443.2 |
  3100375.1 |
  3107306.9 |
  3114238.8 |
  3121170.6 |
  3128102.5 |
  3135034.3 |
  3141966.2 |
  3148898.1 |
  3155829.9 |
  3162761.8 |
  3169693.6 |
  3176625.5 |
  3183557.3 |########################################
  3190489.2 |
  3197421.0 |####################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3180369.6-3203497.9 ns)
  3180369.6 |########################################
  3181526.0 |
  3182682.4 |
  3183838.8 |
  3184995.3 |
  3186151.7 |
  3187308.1 |
  3188464.5 |
  3189620.9 |
  3190777.3 |
  3191933.8 |########################################
  3193090.2 |
  3194246.6 |########################################
  3195403.0 |########################################
  3196559.4 |
  3197715.8 |
  3198872.2 |
  3200028.7 |
  3201185.1 |
  3202341.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=311.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=12015.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=311.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=310.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=309.7% of algo (FFI overhead may distort results)

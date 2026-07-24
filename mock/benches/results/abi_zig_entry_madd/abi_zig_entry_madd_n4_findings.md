# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 68248% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (3.95 us) leads abi_zig_entry_madd_zig_anchor (2.70 ms) by 68248%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.72 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 808.0x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.19 ms) is 808.0x the fastest (3.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_madd_zig_tail_dispatch shows alternating (throttle bounce) (autocorr -0.61)

abi_zig_entry_madd_zig_tail_dispatch's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (68248% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 68248% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 808.0x the fastest

Fastest abi_zig_entry_madd_zig_null (3.95 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.19 ms): 808.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 3953.8 ns median (-99.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 808.02x (fastest 3953.8 ns, slowest 3194695.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2704761ns | 2705259ns | 2693585ns | 2701736ns | 2714886ns | -0.78% |
| abi_zig_entry_madd_zig_dispatch | 2759633ns | 2707947ns | 2699669ns | 2706831ns | 2868818ns | +1.23% |
| abi_zig_entry_madd_zig_null | 6313ns | 6286ns | 6272ns | 6282ns | 6380ns | -99.77% |
| abi_zig_entry_madd_zig_per_w_set | 2709469ns | 2708310ns | 2701654ns | 2707629ns | 2716136ns | -0.61% |
| abi_zig_entry_madd_zig_runtime_w | 2726078ns | 2728366ns | 2705190ns | 2722052ns | 2742560ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3076763ns | 3077114ns | 3066013ns | 3076059ns | 3083194ns | +12.86% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3213926ns | 3198285ns | 3180963ns | 3196987ns | 3255816ns | +17.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2701673ns | 2690666ns | 2711543ns | -0.78% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2756383ns | 2696693ns | 2865195ns | +1.23% | 0.000 |
| abi_zig_entry_madd_zig_null | 3968ns | 3927ns | 4016ns | -99.85% | 0.001 |
| abi_zig_entry_madd_zig_per_w_set | 2706274ns | 2698734ns | 2712543ns | -0.61% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2722807ns | 2702078ns | 2739311ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3073389ns | 3063007ns | 3079689ns | +12.88% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3210497ns | 3177781ns | 3252335ns | +17.91% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 209938.7 | 2703896.7 | 2701673.3 | n/a |
| abi_zig_entry_madd_zig_dispatch | 223534.7 | 2749616.0 | 2756383.2 | n/a |
| abi_zig_entry_madd_zig_null | 158645.5 | 4227.8 | 3967.7 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 217262.4 | 2706234.2 | 2706273.6 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 219946.4 | 2723409.8 | 2722806.6 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 223323.9 | 3073973.8 | 3073388.7 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 235212.2 | 3224347.8 | 3210497.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.001 | 99.3% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2704761ns | 2704761ns | -0.78% |
| abi_zig_entry_madd_zig_dispatch | 2759633ns | 2759633ns | +1.23% |
| abi_zig_entry_madd_zig_null | 6313ns | 6313ns | -99.77% |
| abi_zig_entry_madd_zig_per_w_set | 2709469ns | 2709469ns | -0.61% |
| abi_zig_entry_madd_zig_runtime_w | 2726078ns | 2726078ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3076763ns | 3076763ns | +12.86% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3213926ns | 3213926ns | +17.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2724749ns | base | --- | [2704360, 2739311] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2702292ns | -16180.0ns (-0.6%) | [-43691, -3529]ns | [2691186, 2711543] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2704880ns | no significant difference | [-32647, +153246]ns | [2699075, 2865195] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_null | 3954ns | -2720797.5ns (-99.9%) | [-2735333, -2700386]ns | [3934, 4016] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2705351ns | -15948.4ns (-0.6%) | [-31355, -2296]ns | [2700926, 2712543] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3073620ns | +349036.8ns (+12.8%) | [+332800, +369909]ns | [3066857, 3079689] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3194695ns | +470446.2ns (+17.3%) | [+452513, +540113]ns | [3184461, 3252335] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2702078ns | -0.4% | -0.2% | -99.9% | -0.1% | +13.4% | +17.6% |
| 2 | 2706641ns | -0.1% | +11.5% | -99.9% | -0.0% | +14.0% | +21.9% |
| 3 | 2737230ns | -1.7% | -1.3% | -99.9% | -0.9% | +12.2% | +16.6% |
| 4 | 2717803ns | -0.2% | -0.5% | -99.9% | -0.5% | +13.1% | +17.9% |
| 5 | 2731695ns | -0.8% | -1.0% | -99.9% | -0.7% | +12.5% | +17.0% |
| 6 | 2741392ns | -1.5% | -1.1% | -99.9% | -1.4% | +12.1% | +16.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.216 | moderate- |
| abi_zig_entry_madd_zig_dispatch | -0.270 | moderate- |
| abi_zig_entry_madd_zig_null | 0.094 | ok |
| abi_zig_entry_madd_zig_per_w_set | -0.205 | moderate- |
| abi_zig_entry_madd_zig_runtime_w | 0.112 | ok |
| abi_zig_entry_madd_zig_tail_dispatch | -0.606 | HIGH- (thermal bounce) |
| abi_zig_entry_madd_zig_tail_runtime_w | -0.412 | moderate- |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 5/6, lost 0/6
- **abi_zig_entry_madd_zig_dispatch**: won 5/6, lost 1/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 5/6, lost 0/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8395723.6ns | 2701673.3ns | 310.8% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8592394.8ns | 2756383.2ns | 311.7% | HIGH |
| abi_zig_entry_madd_zig_null | 309589.0ns | 3967.7ns | 7802.7% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8414755.3ns | 2706273.6ns | 310.9% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8463109.8ns | 2722806.6ns | 310.8% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9533308.6ns | 3073388.7ns | 310.2% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 10052899.6ns | 3210497.4ns | 313.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2690665.8-2711542.7 ns)
  2690665.8 |########################################
  2691709.6 |
  2692753.5 |
  2693797.3 |
  2694841.2 |
  2695885.0 |
  2696928.9 |
  2697972.7 |
  2699016.6 |
  2700060.4 |####################
  2701104.2 |
  2702148.1 |
  2703191.9 |####################
  2704235.8 |
  2705279.6 |
  2706323.5 |
  2707367.3 |
  2708411.2 |
  2709455.0 |####################
  2710498.9 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2696693.3-2865195.2 ns)
  2696693.3 |########################################
  2705118.4 |##########################
  2713543.5 |
  2721968.6 |
  2730393.7 |
  2738818.8 |
  2747243.9 |
  2755669.0 |
  2764094.1 |
  2772519.2 |
  2780944.2 |
  2789369.3 |
  2797794.4 |
  2806219.5 |
  2814644.6 |
  2823069.7 |
  2831494.8 |
  2839919.9 |
  2848345.0 |
  2856770.1 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 3927.1-4015.6 ns)
   3927.1 |########################################
   3931.5 |
   3936.0 |
   3940.4 |########################################
   3944.8 |########################################
   3949.2 |
   3953.7 |
   3958.1 |########################################
   3962.5 |
   3966.9 |
   3971.4 |
   3975.8 |
   3980.2 |
   3984.7 |
   3989.1 |
   3993.5 |
   3997.9 |
   4002.4 |
   4006.8 |
   4011.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2698733.8-2712543.4 ns)
  2698733.8 |####################
  2699424.3 |
  2700114.8 |
  2700805.2 |
  2701495.7 |
  2702186.2 |
  2702876.7 |####################
  2703567.1 |
  2704257.6 |
  2704948.1 |########################################
  2705638.6 |
  2706329.1 |
  2707019.5 |
  2707710.0 |
  2708400.5 |
  2709091.0 |
  2709781.4 |
  2710471.9 |
  2711162.4 |
  2711852.9 |####################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2702077.9-2739311.5 ns)
  2702077.9 |########################################
  2703939.6 |
  2705801.3 |########################################
  2707662.9 |
  2709524.6 |
  2711386.3 |
  2713248.0 |
  2715109.6 |
  2716971.3 |########################################
  2718833.0 |
  2720694.7 |
  2722556.4 |
  2724418.0 |
  2726279.7 |
  2728141.4 |
  2730003.1 |########################################
  2731864.7 |
  2733726.4 |
  2735588.1 |########################################
  2737449.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3063007.1-3079689.1 ns)
  3063007.1 |#############
  3063841.2 |
  3064675.3 |
  3065509.4 |
  3066343.5 |
  3067177.6 |
  3068011.7 |
  3068845.8 |
  3069679.9 |
  3070514.0 |#############
  3071348.1 |
  3072182.2 |
  3073016.3 |########################################
  3073850.4 |
  3074684.5 |
  3075518.6 |
  3076352.7 |
  3077186.8 |
  3078020.9 |
  3078855.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3177780.8-3252335.4 ns)
  3177780.8 |####################
  3181508.5 |
  3185236.3 |
  3188964.0 |########################################
  3192691.7 |
  3196419.5 |####################
  3200147.2 |
  3203874.9 |####################
  3207602.6 |
  3211330.4 |
  3215058.1 |
  3218785.8 |
  3222513.6 |
  3226241.3 |
  3229969.0 |
  3233696.8 |
  3237424.5 |
  3241152.2 |
  3244879.9 |
  3248607.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=310.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=310.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=7827.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=310.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=310.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=310.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=310.3% of algo (FFI overhead may distort results)

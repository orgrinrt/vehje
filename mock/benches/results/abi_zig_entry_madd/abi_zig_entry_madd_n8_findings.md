# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 68857% faster than the next best (abi_zig_entry_madd_zig_anchor)

abi_zig_entry_madd_zig_null (3.92 us) leads abi_zig_entry_madd_zig_anchor (2.70 ms) by 68857%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.72 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 817.0x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.20 ms) is 817.0x the fastest (3.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_madd_zig_runtime_w shows alternating (throttle bounce) (autocorr -0.54)

abi_zig_entry_madd_zig_runtime_w's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (68857% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 68857% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 817.0x the fastest

Fastest abi_zig_entry_madd_zig_null (3.92 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.20 ms): 817.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 3917.5 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 817.02x (fastest 3917.5 ns, slowest 3200660.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2706616ns | 2704708ns | 2698657ns | 2702833ns | 2716271ns | -0.66% |
| abi_zig_entry_madd_zig_dispatch | 2806641ns | 2714215ns | 2703198ns | 2711812ns | 3000606ns | +3.02% |
| abi_zig_entry_madd_zig_null | 6263ns | 6229ns | 6187ns | 6221ns | 6366ns | -99.77% |
| abi_zig_entry_madd_zig_per_w_set | 2785660ns | 2724925ns | 2708048ns | 2719564ns | 2923612ns | +2.25% |
| abi_zig_entry_madd_zig_runtime_w | 2724474ns | 2723291ns | 2717396ns | 2721450ns | 2732549ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3149359ns | 3091376ns | 3073725ns | 3087182ns | 3280441ns | +15.60% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3223913ns | 3204181ns | 3188036ns | 3201524ns | 3275435ns | +18.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2703264ns | 2695549ns | 2712669ns | -0.66% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2803095ns | 2700161ns | 2996487ns | +3.01% | 0.000 |
| abi_zig_entry_madd_zig_null | 3928ns | 3856ns | 3988ns | -99.86% | 0.002 |
| abi_zig_entry_madd_zig_per_w_set | 2782246ns | 2705150ns | 2919947ns | +2.25% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2721132ns | 2714305ns | 2729100ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3145712ns | 3070446ns | 3276351ns | +15.60% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3220307ns | 3184684ns | 3271580ns | +18.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 226869.8 | 2714321.1 | 2703264.5 | n/a |
| abi_zig_entry_madd_zig_dispatch | 240621.4 | 2770145.3 | 2803095.1 | n/a |
| abi_zig_entry_madd_zig_null | 158068.3 | 4139.8 | 3927.6 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 233112.9 | 2731405.4 | 2782245.8 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 227068.8 | 2723284.8 | 2721132.1 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 247871.5 | 3147289.2 | 3145711.8 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 245523.5 | 3258905.5 | 3220306.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.002 | 98.4% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2706616ns | 2706616ns | -0.66% |
| abi_zig_entry_madd_zig_dispatch | 2806641ns | 2806641ns | +3.02% |
| abi_zig_entry_madd_zig_null | 6263ns | 6263ns | -99.77% |
| abi_zig_entry_madd_zig_per_w_set | 2785660ns | 2785660ns | +2.25% |
| abi_zig_entry_madd_zig_runtime_w | 2724474ns | 2724474ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3149359ns | 3149359ns | +15.60% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3223913ns | 3223913ns | +18.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2719815ns | base | --- | [2714481, 2729100] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2701374ns | -14768.6ns (-0.5%) | [-32164, -6670]ns | [2695751, 2712669] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2710870ns | no significant difference | [-23637, +277836]ns | [2701928, 2996487] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_madd_zig_null | 3918ns | -2715827.2ns (-99.9%) | [-2725195, -2710591]ns | [3877, 3988] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2721407ns | no significant difference | [-22150, +202459]ns | [2705384, 2919947] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3087938ns | +370882.2ns (+13.6%) | [+346391, +556466]ns | [3072846, 3276351] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3200660ns | +483428.3ns (+17.8%) | [+462402, +551694]ns | [3188681, 3271580] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2719807ns | -0.8% | -0.1% | -99.9% | +0.6% | +13.6% | +17.6% |
| 2 | 2719824ns | -0.1% | +20.4% | -99.9% | -0.5% | +16.1% | +18.4% |
| 3 | 2714305ns | -0.4% | +0.0% | -99.9% | +0.7% | +13.7% | +17.3% |
| 4 | 2738253ns | -1.6% | -1.1% | -99.9% | -1.1% | +12.1% | +16.6% |
| 5 | 2714657ns | -0.7% | -0.5% | -99.9% | -0.3% | +13.3% | +18.0% |
| 6 | 2719948ns | -0.4% | -0.6% | -99.9% | +14.2% | +24.9% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.132 | ok |
| abi_zig_entry_madd_zig_dispatch | -0.200 | moderate- |
| abi_zig_entry_madd_zig_null | -0.268 | moderate- |
| abi_zig_entry_madd_zig_per_w_set | -0.065 | ok |
| abi_zig_entry_madd_zig_runtime_w | -0.542 | HIGH- (thermal bounce) |
| abi_zig_entry_madd_zig_tail_dispatch | -0.114 | ok |
| abi_zig_entry_madd_zig_tail_runtime_w | -0.022 | ok |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_dispatch**: won 3/6, lost 1/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 3/6, lost 3/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8510730.6ns | 2703264.5ns | 314.8% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8643692.4ns | 2803095.1ns | 308.4% | HIGH |
| abi_zig_entry_madd_zig_null | 308724.2ns | 3927.6ns | 7860.4% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8582044.2ns | 2782245.8ns | 308.5% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8469841.6ns | 2721132.1ns | 311.3% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9837665.3ns | 3145711.8ns | 312.7% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 10102375.6ns | 3220306.9ns | 313.7% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2695549.2-2712668.5 ns)
  2695549.2 |########################################
  2696405.2 |
  2697261.1 |
  2698117.1 |####################
  2698973.1 |
  2699829.0 |
  2700685.0 |
  2701541.0 |
  2702396.9 |
  2703252.9 |
  2704108.9 |####################
  2704964.8 |
  2705820.8 |
  2706676.8 |
  2707532.7 |
  2708388.7 |####################
  2709244.7 |
  2710100.6 |
  2710956.6 |
  2711812.6 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2700160.8-2996487.1 ns)
  2700160.8 |########################################
  2714977.1 |##########
  2729793.4 |
  2744609.7 |
  2759426.1 |
  2774242.4 |
  2789058.7 |
  2803875.0 |
  2818691.3 |
  2833507.6 |
  2848324.0 |
  2863140.3 |
  2877956.6 |
  2892772.9 |
  2907589.2 |
  2922405.5 |
  2937221.8 |
  2952038.2 |
  2966854.5 |
  2981670.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 3856.2-3988.1 ns)
   3856.2 |########################################
   3862.8 |
   3869.4 |
   3876.0 |
   3882.6 |
   3889.2 |
   3895.8 |########################################
   3902.4 |
   3909.0 |########################################
   3915.6 |
   3922.2 |########################################
   3928.8 |
   3935.4 |########################################
   3942.0 |
   3948.6 |
   3955.2 |
   3961.8 |
   3968.4 |
   3975.0 |
   3981.6 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2705150.0-2919946.6 ns)
  2705150.0 |########################################
  2715889.8 |
  2726629.7 |##########################
  2737369.5 |
  2748109.3 |
  2758849.2 |
  2769589.0 |
  2780328.8 |
  2791068.7 |
  2801808.5 |
  2812548.3 |
  2823288.2 |
  2834028.0 |
  2844767.8 |
  2855507.7 |
  2866247.5 |
  2876987.3 |
  2887727.2 |
  2898467.0 |
  2909206.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2714304.6-2729100.2 ns)
  2714304.6 |##########################
  2715044.4 |
  2715784.2 |
  2716523.9 |
  2717263.7 |
  2718003.5 |
  2718743.3 |
  2719483.1 |########################################
  2720222.8 |
  2720962.6 |
  2721702.4 |
  2722442.2 |
  2723182.0 |
  2723921.7 |
  2724661.5 |
  2725401.3 |
  2726141.1 |
  2726880.9 |
  2727620.6 |
  2728360.4 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3070446.2-3276351.2 ns)
  3070446.2 |########################################
  3080741.5 |########################################
  3091036.7 |
  3101332.0 |
  3111627.2 |
  3121922.5 |
  3132217.7 |
  3142513.0 |
  3152808.2 |####################
  3163103.5 |
  3173398.7 |
  3183694.0 |
  3193989.2 |
  3204284.5 |
  3214579.7 |
  3224875.0 |
  3235170.2 |
  3245465.5 |
  3255760.7 |
  3266056.0 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3184683.8-3271579.5 ns)
  3184683.8 |########################################
  3189028.6 |########################################
  3193373.4 |
  3197718.2 |########################################
  3202062.9 |########################################
  3206407.7 |
  3210752.5 |
  3215097.3 |
  3219442.1 |########################################
  3223786.9 |
  3228131.7 |
  3232476.5 |
  3236821.2 |
  3241166.0 |
  3245510.8 |
  3249855.6 |
  3254200.4 |
  3258545.2 |
  3262890.0 |
  3267234.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=311.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=7830.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=310.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=310.3% of algo (FFI overhead may distort results)

# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 103600% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (2.63 us) leads abi_lifecycle_madd_held_handle (2.73 ms) by 103600%, a clear separation rather than a photo finish. CV 39.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.72 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1115.1x slower than the field

abi_lifecycle_madd_fresh_per_batch (2.93 ms) is 1115.1x the fastest (2.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_madd_null_entry is fastest but the noisiest (CV 39.4%)

abi_lifecycle_madd_null_entry wins on median (2.63 us) yet has the highest variance (CV 39.4%), while abi_lifecycle_madd_held_handle is the steadiest (CV 0.5%, 2.73 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (103600% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 103600% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1115.1x the fastest

Fastest abi_lifecycle_madd_null_entry (2.63 us) to slowest abi_lifecycle_madd_fresh_per_batch (2.93 ms): 1115.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_lifecycle_madd_null_entry is inconsistent: worst-20% is 1.6x its best-20%

abi_lifecycle_madd_null_entry's best 20% of batches run at 2.47 us but its worst 20% at 4.02 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 2629.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1115.06x (fastest 2629.8 ns, slowest 2932392.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2954971ns | 2935494ns | 2929731ns | 2934708ns | 2997985ns | +8.12% |
| abi_lifecycle_madd_fresh_per_column | 2835049ns | 2745902ns | 2738678ns | 2744834ns | 3018556ns | +3.73% |
| abi_lifecycle_madd_held_handle | 2732976ns | 2730575ns | 2718012ns | 2728935ns | 2746520ns | base |
| abi_lifecycle_madd_null_entry | 5398ns | 5017ns | 4685ns | 4941ns | 6440ns | -99.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2951755ns | 2926655ns | 2994446ns | +8.14% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2831480ns | 2735660ns | 3013999ns | +3.73% | 0.000 |
| abi_lifecycle_madd_held_handle | 2729568ns | 2714801ns | 2743025ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 3051ns | 2470ns | 4020ns | -99.89% | 0.005 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 66087.7 | 2952835.6 | 2951755.1 | 0 |
| abi_lifecycle_madd_fresh_per_column | 72220.7 | 2813747.1 | 2831480.2 | n/a |
| abi_lifecycle_madd_held_handle | 70148.5 | 2729553.6 | 2729568.4 | n/a |
| abi_lifecycle_madd_null_entry | 29894.9 | 2725.9 | 3050.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.006 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2954971ns | 2954971ns | +8.12% |
| abi_lifecycle_madd_fresh_per_column | 2835049ns | 2835049ns | +3.73% |
| abi_lifecycle_madd_held_handle | 2732976ns | 2732976ns | base |
| abi_lifecycle_madd_null_entry | 5398ns | 5398ns | -99.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2727095ns | base | --- | [2718586, 2743025] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 2932392ns | +206146.7ns (+7.6%) | [+203102, +257312]ns | [2928427, 2994446] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2742792ns | +16676.5ns (+0.6%) | [+10647, +278412]ns | [2737649, 3013999] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_madd_null_entry | 2630ns | -2724487.1ns (-99.9%) | [-2739052, -2716013]ns | [2503, 4020] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2728759ns | +7.6% | +0.4% | -99.9% |
| 2 | 2756372ns | +10.7% | +8.0% | -99.8% |
| 3 | 2722370ns | +7.5% | +0.5% | -99.9% |
| 4 | 2729677ns | +7.4% | +0.4% | -99.9% |
| 5 | 2725431ns | +7.5% | +0.7% | -99.9% |
| 6 | 2714801ns | +8.0% | +12.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.224 | moderate- |
| abi_lifecycle_madd_fresh_per_column | -0.287 | moderate- |
| abi_lifecycle_madd_held_handle | -0.154 | ok |
| abi_lifecycle_madd_null_entry | -0.303 | moderate- |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_madd_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 8925429.3ns | 2951755.1ns | 302.4% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8543704.7ns | 2831480.2ns | 301.7% | HIGH |
| abi_lifecycle_madd_held_handle | 8258236.2ns | 2729568.4ns | 302.5% | HIGH |
| abi_lifecycle_madd_null_entry | 133050.5ns | 3050.8ns | 4361.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 2926655.4-2994445.9 ns)
  2926655.4 |#############
  2930044.9 |########################################
  2933434.4 |#############
  2936824.0 |
  2940213.5 |
  2943603.0 |
  2946992.5 |
  2950382.1 |
  2953771.6 |
  2957161.1 |
  2960550.6 |
  2963940.1 |
  2967329.7 |
  2970719.2 |
  2974108.7 |
  2977498.2 |
  2980887.8 |
  2984277.3 |
  2987666.8 |
  2991056.3 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2735660.4-3013998.8 ns)
  2735660.4 |########################################
  2749577.3 |
  2763494.2 |
  2777411.2 |
  2791328.1 |
  2805245.0 |
  2819161.9 |
  2833078.8 |
  2846995.7 |
  2860912.7 |
  2874829.6 |
  2888746.5 |
  2902663.4 |
  2916580.3 |
  2930497.2 |
  2944414.2 |
  2958331.1 |
  2972248.0 |##########
  2986164.9 |
  3000081.8 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2714800.8-2743024.6 ns)
  2714800.8 |########################################
  2716212.0 |
  2717623.2 |
  2719034.4 |
  2720445.6 |
  2721856.8 |########################################
  2723267.9 |
  2724679.1 |########################################
  2726090.3 |
  2727501.5 |########################################
  2728912.7 |########################################
  2730323.9 |
  2731735.1 |
  2733146.3 |
  2734557.5 |
  2735968.6 |
  2737379.8 |
  2738791.0 |
  2740202.2 |
  2741613.4 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 2470.4-4019.6 ns)
   2470.4 |########################################
   2547.9 |####################
   2625.3 |########################################
   2702.8 |
   2780.2 |
   2857.7 |
   2935.2 |
   3012.6 |
   3090.1 |
   3167.5 |
   3245.0 |
   3322.5 |
   3399.9 |
   3477.4 |
   3554.8 |
   3632.3 |
   3709.8 |
   3787.2 |
   3864.7 |
   3942.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: CV=33.9% (high variance, measurements may be unstable)
- **abi_lifecycle_madd_null_entry**: bridge=4513.4% of algo (FFI overhead may distort results)

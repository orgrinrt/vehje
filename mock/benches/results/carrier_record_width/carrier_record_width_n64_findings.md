# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (323 ns) is smaller than the fastest variant's own run-to-run std-dev (441 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### carrier_rec12's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_rec12 are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader carrier_rec16 vs stability leader carrier_rec12 (+5% speed for 1.2x steadier)

carrier_rec16 is fastest (3.65 us, CV 12.1%); carrier_rec12 gives up 4.7% median for 1.2x lower variance (CV 10.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_rec12's edge over baseline is significant but tiny (8 ns, 0.21%)

carrier_rec12 differs from baseline carrier_rec24 by 8 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_rec16** at 3646.6 ns median (-4.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.09x (fastest 3646.6 ns, slowest 3969.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 6473ns | 6762ns | 5194ns | 6499ns | 7074ns | -0.73% |
| carrier_rec16 | 6343ns | 6548ns | 4946ns | 6353ns | 7027ns | -2.72% |
| carrier_rec20 | 6555ns | 7037ns | 5167ns | 6684ns | 7056ns | +0.53% |
| carrier_rec24 | 6521ns | 6773ns | 4990ns | 6665ns | 7069ns | base |
| carrier_rec32 | 6468ns | 6696ns | 4957ns | 6439ns | 7266ns | -0.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 3665ns | 2941ns | 4012ns | -0.49% | 0.017 |
| carrier_rec16 | 3549ns | 2778ns | 3956ns | -3.64% | 0.018 |
| carrier_rec20 | 3701ns | 2916ns | 3983ns | +0.49% | 0.017 |
| carrier_rec24 | 3683ns | 2809ns | 3997ns | base | 0.017 |
| carrier_rec32 | 3637ns | 2786ns | 4093ns | -1.24% | 0.018 |

## Performance model

- Peak throughput: **0.023 Gops/s** (carrier_rec16; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.017 | 72.7% |
| carrier_rec16 | 0.018 | 76.2% |
| carrier_rec20 | 0.016 | 70.0% |
| carrier_rec24 | 0.017 | 72.7% |
| carrier_rec32 | 0.017 | 73.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 6473ns | 6473ns | -0.73% |
| carrier_rec16 | 6343ns | 6343ns | -2.72% |
| carrier_rec20 | 6555ns | 6555ns | +0.53% |
| carrier_rec24 | 6521ns | 6521ns | base |
| carrier_rec32 | 6468ns | 6468ns | -0.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 3822ns | base | --- | [3230, 3997] | --- | --- | --- | --- |
| carrier_rec12 | 3819ns | no significant difference | [-144, +82]ns | [3163, 4012] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| carrier_rec16 | 3647ns | -55.9ns (-1.5%) | [-316, -30]ns | [3044, 3956] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_rec20 | 3969ns | no significant difference | [-149, +221]ns | [3150, 3983] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec32 | 3758ns | no significant difference | [-359, +255]ns | [3060, 4093] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 3992ns | -0.5% | -0.7% | -0.8% | +6.1% |
| 2 | 3995ns | +0.8% | -1.5% | -0.5% | -1.1% |
| 3 | 3998ns | +0.0% | -1.3% | -0.4% | -10.0% |
| 4 | 2809ns | +4.7% | -1.1% | +3.8% | -0.8% |
| 5 | 3652ns | -7.3% | -8.0% | -7.3% | -8.7% |
| 6 | 3650ns | +0.4% | -9.3% | +9.2% | +7.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | 0.205 | moderate+ |
| carrier_rec16 | 0.168 | ok |
| carrier_rec20 | 0.086 | ok |
| carrier_rec24 | -0.050 | ok |
| carrier_rec32 | 0.282 | moderate+ |

**Consistency summary:**

- **carrier_rec12**: won 2/6, lost 3/6
- **carrier_rec16**: won 6/6, lost 0/6
- **carrier_rec20**: won 4/6, lost 2/6
- **carrier_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 58.6ns | 3664.8ns | 1.6% |  |
| carrier_rec16 | 57.9ns | 3548.9ns | 1.6% |  |
| carrier_rec20 | 60.2ns | 3700.9ns | 1.6% |  |
| carrier_rec24 | 62.0ns | 3682.9ns | 1.7% |  |
| carrier_rec32 | 63.4ns | 3637.1ns | 1.7% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 2940.8-4012.5 ns)
   2940.8 |####################
   2994.4 |
   3048.0 |
   3101.6 |
   3155.1 |
   3208.7 |
   3262.3 |
   3315.9 |
   3369.5 |####################
   3423.1 |
   3476.7 |
   3530.2 |
   3583.8 |
   3637.4 |####################
   3691.0 |
   3744.6 |
   3798.2 |
   3851.7 |
   3905.3 |
   3958.9 |########################################
  (0 below, 1 above range)

carrier_rec16 (n=6, range 2777.5-3956.1 ns)
   2777.5 |####################
   2836.4 |
   2895.4 |
   2954.3 |
   3013.2 |
   3072.1 |
   3131.1 |
   3190.0 |
   3248.9 |
   3307.8 |########################################
   3366.8 |
   3425.7 |
   3484.6 |
   3543.6 |
   3602.5 |
   3661.4 |
   3720.3 |
   3779.3 |
   3838.2 |
   3897.1 |########################################
  (0 below, 1 above range)

carrier_rec20 (n=6, range 2916.2-3982.9 ns)
   2916.2 |#############
   2969.5 |
   3022.9 |
   3076.2 |
   3129.5 |
   3182.9 |
   3236.2 |
   3289.5 |
   3342.9 |#############
   3396.2 |
   3449.6 |
   3502.9 |
   3556.2 |
   3609.6 |
   3662.9 |
   3716.2 |
   3769.6 |
   3822.9 |
   3876.2 |
   3929.6 |########################################
  (0 below, 1 above range)

carrier_rec24 (n=6, range 2809.2-3996.7 ns)
   2809.2 |####################
   2868.6 |
   2927.9 |
   2987.3 |
   3046.7 |
   3106.1 |
   3165.4 |
   3224.8 |
   3284.2 |
   3343.6 |
   3402.9 |
   3462.3 |
   3521.7 |
   3581.0 |
   3640.4 |########################################
   3699.8 |
   3759.2 |
   3818.5 |
   3877.9 |
   3937.3 |########################################
  (0 below, 1 above range)

carrier_rec32 (n=6, range 2786.2-4092.7 ns)
   2786.2 |####################
   2851.5 |
   2916.8 |
   2982.2 |
   3047.5 |
   3112.8 |
   3178.1 |
   3243.5 |
   3308.8 |####################
   3374.1 |
   3439.4 |
   3504.8 |
   3570.1 |####################
   3635.4 |
   3700.8 |
   3766.1 |
   3831.4 |
   3896.7 |########################################
   3962.1 |
   4027.4 |
  (0 below, 1 above range)

```

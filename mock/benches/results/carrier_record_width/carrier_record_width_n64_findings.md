# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (51 ns) is smaller than the fastest variant's own run-to-run std-dev (333 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_rec32 vs stability leader carrier_rec20 (+0% speed for 1.2x steadier)

carrier_rec32 is fastest (3.36 us, CV 9.9%); carrier_rec20 gives up 0.3% median for 1.2x lower variance (CV 8.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.5% of the fastest

All 5 variants sit between 3.36 us and 3.41 us - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_rec12's edge over baseline is significant but tiny (-11 ns, 0.33%)

carrier_rec12 differs from baseline carrier_rec24 by -11 ns (0.33%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_rec32** at 3357.5 ns median (-0.1% vs baseline)
- Spread: 1.02x (fastest 3357.5 ns, slowest 3408.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 6832ns | 6898ns | 5568ns | 6778ns | 7545ns | +0.50% |
| carrier_rec16 | 6816ns | 6894ns | 5574ns | 6621ns | 7730ns | +0.27% |
| carrier_rec20 | 6726ns | 6794ns | 6080ns | 6565ns | 7291ns | -1.05% |
| carrier_rec24 | 6798ns | 6790ns | 6082ns | 6556ns | 7518ns | base |
| carrier_rec32 | 6657ns | 6796ns | 5574ns | 6554ns | 7352ns | -2.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 3356ns | 2750ns | 3739ns | -0.78% | 0.019 |
| carrier_rec16 | 3361ns | 2744ns | 3802ns | -0.61% | 0.019 |
| carrier_rec20 | 3330ns | 3005ns | 3605ns | -1.53% | 0.019 |
| carrier_rec24 | 3382ns | 3012ns | 3771ns | base | 0.019 |
| carrier_rec32 | 3277ns | 2752ns | 3594ns | -3.11% | 0.020 |

## Performance model

- Peak throughput: **0.023 Gops/s** (carrier_rec16; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.019 | 81.6% |
| carrier_rec16 | 0.019 | 80.5% |
| carrier_rec20 | 0.019 | 81.5% |
| carrier_rec24 | 0.019 | 81.6% |
| carrier_rec32 | 0.019 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 6832ns | 6832ns | +0.50% |
| carrier_rec16 | 6816ns | 6816ns | +0.27% |
| carrier_rec20 | 6726ns | 6726ns | -1.05% |
| carrier_rec24 | 6798ns | 6798ns | base |
| carrier_rec32 | 6657ns | 6657ns | -2.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 3361ns | base | --- | [3014, 3771] | --- | --- | --- | --- |
| carrier_rec12 | 3361ns | no significant difference | [-305, +237]ns | [2967, 3739] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec16 | 3409ns | no significant difference | [-531, +441]ns | [2873, 3802] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec20 | 3369ns | no significant difference | [-168, +12]ns | [3016, 3605] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec32 | 3358ns | no significant difference | [-301, +2]ns | [2878, 3594] | no | 0.8750 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 3016ns | +5.6% | -0.5% | +0.4% | -0.4% |
| 2 | 3012ns | -8.7% | -8.9% | -0.2% | -8.6% |
| 3 | 3365ns | +0.1% | +13.2% | +0.2% | -0.2% |
| 4 | 3380ns | -0.7% | +2.1% | -0.4% | -0.7% |
| 5 | 4162ns | -8.4% | -19.1% | -7.8% | -8.2% |
| 6 | 3358ns | +9.1% | +13.1% | +0.4% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | 0.339 | moderate+ |
| carrier_rec16 | -0.012 | ok |
| carrier_rec20 | 0.272 | moderate+ |
| carrier_rec24 | 0.138 | ok |
| carrier_rec32 | 0.302 | moderate+ |

**Consistency summary:**

- **carrier_rec12**: won 3/6, lost 2/6
- **carrier_rec16**: won 3/6, lost 3/6
- **carrier_rec20**: won 3/6, lost 3/6
- **carrier_rec32**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 75.2ns | 3355.7ns | 2.2% |  |
| carrier_rec16 | 77.5ns | 3361.5ns | 2.3% |  |
| carrier_rec20 | 72.0ns | 3330.2ns | 2.2% |  |
| carrier_rec24 | 71.6ns | 3382.0ns | 2.1% |  |
| carrier_rec32 | 72.8ns | 3276.7ns | 2.2% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 2749.6-3738.8 ns)
   2749.6 |####################
   2799.1 |
   2848.5 |
   2898.0 |
   2947.4 |
   2996.9 |
   3046.3 |
   3095.8 |
   3145.3 |####################
   3194.7 |
   3244.2 |
   3293.6 |
   3343.1 |########################################
   3392.5 |
   3442.0 |
   3491.5 |
   3540.9 |
   3590.4 |
   3639.8 |####################
   3689.3 |
  (0 below, 1 above range)

carrier_rec16 (n=6, range 2744.2-3802.5 ns)
   2744.2 |########################################
   2797.1 |
   2850.0 |
   2902.9 |
   2955.9 |########################################
   3008.8 |
   3061.7 |
   3114.6 |
   3167.5 |
   3220.4 |
   3273.3 |
   3326.3 |########################################
   3379.2 |
   3432.1 |########################################
   3485.0 |
   3537.9 |
   3590.8 |
   3643.8 |
   3696.7 |
   3749.6 |########################################
  (0 below, 1 above range)

carrier_rec20 (n=6, range 3004.6-3605.4 ns)
   3004.6 |##########################
   3034.6 |
   3064.7 |
   3094.7 |
   3124.8 |
   3154.8 |
   3184.9 |
   3214.9 |
   3244.9 |
   3275.0 |
   3305.0 |
   3335.1 |
   3365.1 |########################################
   3395.2 |
   3425.2 |
   3455.2 |
   3485.3 |
   3515.3 |
   3545.4 |
   3575.4 |
  (0 below, 1 above range)

carrier_rec24 (n=6, range 3011.7-3771.2 ns)
   3011.7 |##########################
   3049.7 |
   3087.7 |
   3125.6 |
   3163.6 |
   3201.6 |
   3239.6 |
   3277.5 |
   3315.5 |
   3353.5 |########################################
   3391.5 |
   3429.5 |
   3467.4 |
   3505.4 |
   3543.4 |
   3581.4 |
   3619.3 |
   3657.3 |
   3695.3 |
   3733.3 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 2752.5-3594.4 ns)
   2752.5 |#############
   2794.6 |
   2836.7 |
   2878.8 |
   2920.9 |
   2963.0 |#############
   3005.1 |
   3047.2 |
   3089.3 |
   3131.4 |
   3173.4 |
   3215.5 |
   3257.6 |
   3299.7 |
   3341.8 |########################################
   3383.9 |
   3426.0 |
   3468.1 |
   3510.2 |
   3552.3 |
  (0 below, 1 above range)

```

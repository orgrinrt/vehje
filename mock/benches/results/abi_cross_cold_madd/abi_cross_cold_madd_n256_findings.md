# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_scalar is an outlier: 883.9x slower than the field

abi_cross_cold_madd_warm_scalar (2.78 ms) is 883.9x the fastest (3.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null) are a dead heat (<1%)

abi_cross_cold_madd_warm_null (3.15 us) and abi_cross_cold_madd_cold_null (3.16 us) differ by 0.24%, inside the noise, even though the wider field spreads 88285.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (3.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} (87467% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} with a 87467% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 883.9x the fastest

Fastest abi_cross_cold_madd_warm_null (3.15 us) to slowest abi_cross_cold_madd_warm_scalar (2.78 ms): 883.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_madd_cold_null's edge over baseline is significant but tiny (36 ns, 1.16%)

abi_cross_cold_madd_cold_null differs from baseline abi_cross_cold_madd_warm_null by 36 ns (1.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 3148.9 ns median
- 2 variants significantly slower than baseline
- Spread: 883.85x (fastest 3148.9 ns, slowest 2783207.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5564ns | 5470ns | 5367ns | 5455ns | 5827ns | +2.27% |
| abi_cross_cold_madd_cold_scalar | 2795995ns | 2767917ns | 2744839ns | 2765180ns | 2867795ns | +51290.51% |
| abi_cross_cold_madd_warm_null | 5441ns | 5466ns | 5295ns | 5415ns | 5552ns | base |
| abi_cross_cold_madd_warm_scalar | 2875120ns | 2787130ns | 2762842ns | 2779947ns | 3074018ns | +52744.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 3225ns | 3129ns | 3388ns | +2.19% | 0.079 |
| abi_cross_cold_madd_cold_scalar | 2792129ns | 2741411ns | 2863564ns | +88377.03% | 0.000 |
| abi_cross_cold_madd_warm_null | 3156ns | 3085ns | 3223ns | base | 0.081 |
| abi_cross_cold_madd_warm_scalar | 2870930ns | 2759268ns | 3069049ns | +90874.09% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 33926.2 | 3205.0 | 3224.9 | n/a |
| abi_cross_cold_madd_cold_scalar | 84510.5 | 2806643.1 | 2792128.7 | n/a |
| abi_cross_cold_madd_warm_null | 28248.3 | 3173.6 | 3155.8 | n/a |
| abi_cross_cold_madd_warm_scalar | 81140.8 | 2811190.8 | 2870930.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.081 | 97.7% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.081 | 98.0% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5564ns | 5564ns | +2.27% |
| abi_cross_cold_madd_cold_scalar | 2795995ns | 2795995ns | +51290.51% |
| abi_cross_cold_madd_warm_null | 5441ns | 5441ns | base |
| abi_cross_cold_madd_warm_scalar | 2875120ns | 2875120ns | +52744.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 3149ns | base | --- | [3095, 3223] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 3157ns | no significant difference | [-4, +175]ns | [3130, 3388] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_madd_cold_scalar | 2764172ns | +2761048.0ns (+87681.5%) | [+2745522, +2860349]ns | [2748650, 2863564] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2783207ns | +2780093.5ns (+88286.4%) | [+2757396, +3065834]ns | [2760535, 3069049] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 3155ns | +1.3% | +88011.1% | +102481.4% |
| 2 | 3274ns | +9.4% | +89902.8% | +88509.3% |
| 3 | 3172ns | -1.1% | +86779.0% | +86885.5% |
| 4 | 3105ns | +0.8% | +89077.1% | +88835.5% |
| 5 | 3142ns | +1.1% | +87697.5% | +88887.3% |
| 6 | 3085ns | +1.4% | +88762.6% | +89688.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.156 | ok |
| abi_cross_cold_madd_cold_scalar | -0.141 | ok |
| abi_cross_cold_madd_warm_null | 0.122 | ok |
| abi_cross_cold_madd_warm_scalar | 0.203 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 1/6, lost 5/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 128150.4ns | 3224.9ns | 3973.8% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8489075.7ns | 2792128.7ns | 304.0% | HIGH |
| abi_cross_cold_madd_warm_null | 121468.0ns | 3155.8ns | 3849.1% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8514949.2ns | 2870930.1ns | 296.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 3129.2-3387.7 ns)
   3129.2 |########################################
   3142.1 |
   3155.0 |
   3168.0 |#############
   3180.9 |
   3193.8 |#############
   3206.8 |
   3219.7 |
   3232.6 |
   3245.5 |
   3258.4 |
   3271.4 |
   3284.3 |
   3297.2 |
   3310.1 |
   3323.1 |
   3336.0 |
   3348.9 |
   3361.8 |
   3374.8 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2741411.2-2863564.0 ns)
  2741411.2 |####################
  2747518.8 |
  2753626.5 |########################################
  2759734.1 |
  2765841.8 |####################
  2771949.4 |
  2778057.0 |####################
  2784164.7 |
  2790272.3 |
  2796379.9 |
  2802487.6 |
  2808595.2 |
  2814702.9 |
  2820810.5 |
  2826918.1 |
  2833025.8 |
  2839133.4 |
  2845241.0 |
  2851348.7 |
  2857456.3 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 3085.0-3223.1 ns)
   3085.0 |########################################
   3091.9 |
   3098.8 |########################################
   3105.7 |
   3112.6 |
   3119.5 |
   3126.4 |
   3133.4 |
   3140.3 |########################################
   3147.2 |
   3154.1 |########################################
   3161.0 |
   3167.9 |########################################
   3174.8 |
   3181.7 |
   3188.6 |
   3195.5 |
   3202.4 |
   3209.3 |
   3216.2 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2759267.5-3069048.5 ns)
  2759267.5 |########################################
  2774756.6 |
  2790245.6 |#############
  2805734.7 |
  2821223.7 |
  2836712.8 |
  2852201.8 |
  2867690.9 |
  2883179.9 |
  2898669.0 |#############
  2914158.0 |
  2929647.1 |
  2945136.1 |
  2960625.2 |
  2976114.2 |
  2991603.3 |
  3007092.3 |
  3022581.4 |
  3038070.4 |
  3053559.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=3972.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=3860.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=302.2% of algo (FFI overhead may distort results)

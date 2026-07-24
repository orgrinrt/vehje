# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_cold_scalar is an outlier: 1033.4x slower than the field

abi_cross_cold_madd_cold_scalar (2.76 ms) is 1033.4x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_madd_cold_null is fastest but the noisiest (CV 8.0%)

abi_cross_cold_madd_cold_null wins on median (2.67 us) yet has the highest variance (CV 8.0%), while abi_cross_cold_madd_cold_scalar is the steadiest (CV 1.0%, 2.76 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (abi_cross_cold_madd_cold_null, abi_cross_cold_madd_warm_null) are a dead heat (<1%)

abi_cross_cold_madd_cold_null (2.67 us) and abi_cross_cold_madd_warm_null (2.68 us) differ by 0.36%, inside the noise, even though the wider field spreads 103235.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {abi_cross_cold_madd_cold_null, abi_cross_cold_madd_warm_null} vs {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} (102679% apart)

The field splits into a fast tier {abi_cross_cold_madd_cold_null, abi_cross_cold_madd_warm_null} and a slow tier {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} with a 102679% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1033.4x the fastest

Fastest abi_cross_cold_madd_cold_null (2.67 us) to slowest abi_cross_cold_madd_cold_scalar (2.76 ms): 1033.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_madd_cold_null's edge over baseline is significant but tiny (3 ns, 0.10%)

abi_cross_cold_madd_cold_null differs from baseline abi_cross_cold_madd_warm_null by 3 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_madd_cold_null** at 2670.6 ns median (-0.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1033.35x (fastest 2670.6 ns, slowest 2759665.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5155ns | 4938ns | 4828ns | 4918ns | 5674ns | +4.46% |
| abi_cross_cold_madd_cold_scalar | 2770318ns | 2763485ns | 2742275ns | 2759825ns | 2800079ns | +56039.36% |
| abi_cross_cold_madd_warm_null | 4935ns | 4942ns | 4811ns | 4922ns | 5015ns | base |
| abi_cross_cold_madd_warm_scalar | 2862792ns | 2758362ns | 2748542ns | 2755470ns | 3080900ns | +57913.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 2756ns | 2617ns | 2971ns | +2.66% | 0.046 |
| abi_cross_cold_madd_cold_scalar | 2766529ns | 2738584ns | 2796280ns | +102957.57% | 0.000 |
| abi_cross_cold_madd_warm_null | 2684ns | 2638ns | 2722ns | base | 0.048 |
| abi_cross_cold_madd_warm_scalar | 2858700ns | 2744988ns | 3075932ns | +106391.07% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 34408.8 | 2833.8 | 2755.8 | n/a |
| abi_cross_cold_madd_cold_scalar | 83170.0 | 2763352.1 | 2766528.9 | n/a |
| abi_cross_cold_madd_warm_null | 27209.7 | 2732.8 | 2684.5 | n/a |
| abi_cross_cold_madd_warm_scalar | 84167.3 | 2843358.2 | 2858699.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_madd_cold_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.048 | 98.0% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.048 | 97.6% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5155ns | 5155ns | +4.46% |
| abi_cross_cold_madd_cold_scalar | 2770318ns | 2770318ns | +56039.36% |
| abi_cross_cold_madd_warm_null | 4935ns | 4935ns | base |
| abi_cross_cold_madd_warm_scalar | 2862792ns | 2862792ns | +57913.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 2680ns | base | --- | [2651, 2722] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 2671ns | no significant difference | [-60, +271]ns | [2626, 2971] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_cold_madd_cold_scalar | 2759665ns | +2756969.8ns (+102864.3%) | [+2740985, +2793578]ns | [2743641, 2796280] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2754683ns | +2752020.9ns (+102679.7%) | [+2742788, +3073236]ns | [2745484, 3075932] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 2727ns | +18.4% | +101113.2% | +119788.7% |
| 2 | 2638ns | -0.8% | +103716.8% | +104242.3% |
| 3 | 2673ns | -1.4% | +102720.4% | +102618.7% |
| 4 | 2717ns | -3.0% | +101690.8% | +100926.4% |
| 5 | 2687ns | +1.0% | +105099.0% | +102498.3% |
| 6 | 2665ns | +1.5% | +103463.4% | +108091.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.098 | ok |
| abi_cross_cold_madd_cold_scalar | 0.048 | ok |
| abi_cross_cold_madd_warm_null | -0.321 | moderate- |
| abi_cross_cold_madd_warm_scalar | -0.045 | ok |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 3/6, lost 3/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 127784.4ns | 2755.8ns | 4636.9% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8385278.2ns | 2766528.9ns | 303.1% | HIGH |
| abi_cross_cold_madd_warm_null | 118812.7ns | 2684.5ns | 4426.0% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8638272.4ns | 2858699.7ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 2617.1-2970.6 ns)
   2617.1 |####################
   2634.8 |########################################
   2652.4 |
   2670.1 |
   2687.8 |
   2705.5 |########################################
   2723.2 |
   2740.8 |
   2758.5 |
   2776.2 |
   2793.9 |
   2811.5 |
   2829.2 |
   2846.9 |
   2864.6 |
   2882.2 |
   2899.9 |
   2917.6 |
   2935.3 |
   2952.9 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2738583.7-2796280.4 ns)
  2738583.7 |####################
  2741468.5 |
  2744353.4 |
  2747238.2 |####################
  2750123.0 |
  2753007.9 |
  2755892.7 |
  2758777.5 |########################################
  2761662.4 |
  2764547.2 |####################
  2767432.0 |
  2770316.9 |
  2773201.7 |
  2776086.6 |
  2778971.4 |
  2781856.2 |
  2784741.1 |
  2787625.9 |
  2790510.7 |
  2793395.6 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 2637.9-2721.9 ns)
   2637.9 |########################################
   2642.1 |
   2646.3 |
   2650.5 |
   2654.7 |
   2658.9 |
   2663.1 |########################################
   2667.3 |
   2671.5 |########################################
   2675.7 |
   2679.9 |
   2684.1 |########################################
   2688.3 |
   2692.5 |
   2696.7 |
   2700.9 |
   2705.1 |
   2709.3 |
   2713.5 |########################################
   2717.7 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2744988.3-3075932.0 ns)
  2744988.3 |########################################
  2761535.5 |
  2778082.7 |
  2794629.9 |
  2811177.0 |
  2827724.2 |
  2844271.4 |
  2860818.6 |
  2877365.8 |##########
  2893913.0 |
  2910460.2 |
  2927007.4 |
  2943554.5 |
  2960101.7 |
  2976648.9 |
  2993196.1 |
  3009743.3 |
  3026290.5 |
  3042837.7 |
  3059384.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=4651.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=4437.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=303.5% of algo (FFI overhead may distort results)

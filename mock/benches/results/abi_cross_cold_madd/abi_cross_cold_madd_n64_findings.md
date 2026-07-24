# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_scalar is an outlier: 1110.8x slower than the field

abi_cross_cold_madd_warm_scalar (2.76 ms) is 1110.8x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null) are a dead heat (<1%)

abi_cross_cold_madd_warm_null (2.49 us) and abi_cross_cold_madd_cold_null (2.51 us) differ by 0.99%, inside the noise, even though the wider field spreads 110978.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### abi_cross_cold_madd_warm_null shows alternating (throttle bounce) (autocorr -0.59)

abi_cross_cold_madd_warm_null's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (2.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} (109387% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} with a 109387% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1110.8x the fastest

Fastest abi_cross_cold_madd_warm_null (2.49 us) to slowest abi_cross_cold_madd_warm_scalar (2.76 ms): 1110.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_madd_cold_null's edge over baseline is significant but tiny (18 ns, 0.73%)

abi_cross_cold_madd_cold_null differs from baseline abi_cross_cold_madd_warm_null by 18 ns (0.73%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 2485.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1110.79x (fastest 2485.4 ns, slowest 2760754.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 4820ns | 4812ns | 4706ns | 4779ns | 4939ns | +0.74% |
| abi_cross_cold_madd_cold_scalar | 2784121ns | 2751840ns | 2742648ns | 2750001ns | 2856038ns | +58092.05% |
| abi_cross_cold_madd_warm_null | 4784ns | 4799ns | 4599ns | 4751ns | 4927ns | base |
| abi_cross_cold_madd_warm_scalar | 2773776ns | 2764674ns | 2749122ns | 2760567ns | 2805917ns | +57875.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 2506ns | 2442ns | 2546ns | +0.40% | 0.026 |
| abi_cross_cold_madd_cold_scalar | 2780422ns | 2739159ns | 2852219ns | +111298.07% | 0.000 |
| abi_cross_cold_madd_warm_null | 2496ns | 2426ns | 2569ns | base | 0.026 |
| abi_cross_cold_madd_warm_scalar | 2769876ns | 2745451ns | 2801790ns | +110875.57% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 32314.4 | 2700.2 | 2505.8 | n/a |
| abi_cross_cold_madd_cold_scalar | 82905.5 | 2766314.3 | 2780421.6 | n/a |
| abi_cross_cold_madd_warm_null | 28761.7 | 2758.3 | 2495.9 | n/a |
| abi_cross_cold_madd_warm_scalar | 79274.5 | 2774767.1 | 2769876.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.025 | 96.7% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.026 | 97.6% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 4820ns | 4820ns | +0.74% |
| abi_cross_cold_madd_cold_scalar | 2784121ns | 2784121ns | +58092.05% |
| abi_cross_cold_madd_warm_null | 4784ns | 4784ns | base |
| abi_cross_cold_madd_warm_scalar | 2773776ns | 2773776ns | +57875.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 2485ns | base | --- | [2434, 2569] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 2510ns | no significant difference | [-65, +77]ns | [2461, 2546] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_cold_madd_cold_scalar | 2748128ns | +2745611.3ns (+110469.6%) | [+2738441, +2849724]ns | [2740918, 2852219] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2760755ns | +2758277.3ns (+110979.2%) | [+2744589, +2799274]ns | [2747084, 2801790] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 2441ns | +3.6% | +112105.4% | +112868.7% |
| 2 | 2513ns | +2.0% | +109026.6% | +109863.7% |
| 3 | 2531ns | -2.0% | +116272.0% | +108364.4% |
| 4 | 2458ns | -0.6% | +112161.6% | +111750.1% |
| 5 | 2606ns | -3.0% | +105216.0% | +107230.4% |
| 6 | 2426ns | +2.7% | +113308.1% | +115568.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.020 | ok |
| abi_cross_cold_madd_cold_scalar | -0.195 | ok |
| abi_cross_cold_madd_warm_null | -0.591 | HIGH- (thermal bounce) |
| abi_cross_cold_madd_warm_scalar | 0.351 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 3/6, lost 3/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 119139.9ns | 2505.8ns | 4754.5% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8399069.6ns | 2780421.6ns | 302.1% | HIGH |
| abi_cross_cold_madd_warm_null | 114812.6ns | 2495.9ns | 4600.0% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8407860.5ns | 2769876.1ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 2442.5-2546.4 ns)
   2442.5 |####################
   2447.7 |
   2452.9 |
   2458.1 |
   2463.3 |
   2468.5 |
   2473.7 |
   2478.9 |####################
   2484.1 |
   2489.3 |####################
   2494.5 |
   2499.7 |
   2504.9 |
   2510.1 |
   2515.3 |
   2520.5 |
   2525.7 |########################################
   2530.9 |
   2536.1 |
   2541.3 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2739158.7-2852218.8 ns)
  2739158.7 |########################################
  2744811.7 |
  2750464.7 |#############
  2756117.7 |#############
  2761770.7 |
  2767423.7 |
  2773076.7 |
  2778729.7 |
  2784382.7 |
  2790035.7 |
  2795688.7 |
  2801341.7 |
  2806994.7 |
  2812647.7 |
  2818300.7 |
  2823953.7 |
  2829606.7 |
  2835259.7 |
  2840912.7 |
  2846565.7 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 2426.2-2568.7 ns)
   2426.2 |########################################
   2433.3 |
   2440.4 |########################################
   2447.6 |
   2454.7 |########################################
   2461.8 |
   2468.9 |
   2476.1 |
   2483.2 |
   2490.3 |
   2497.4 |
   2504.6 |
   2511.7 |########################################
   2518.8 |
   2525.9 |########################################
   2533.1 |
   2540.2 |
   2547.3 |
   2554.4 |
   2561.6 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2745451.2-2801790.4 ns)
  2745451.2 |########################################
  2748268.2 |########################################
  2751085.1 |
  2753902.1 |
  2756719.0 |########################################
  2759536.0 |
  2762353.0 |########################################
  2765169.9 |
  2767986.9 |
  2770803.8 |
  2773620.8 |
  2776437.8 |
  2779254.7 |
  2782071.7 |
  2784888.6 |
  2787705.6 |
  2790522.6 |
  2793339.5 |
  2796156.5 |########################################
  2798973.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=4758.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=4601.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=304.0% of algo (FFI overhead may distort results)

# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_scalar is an outlier: 1103.0x slower than the field

abi_cross_cold_madd_warm_scalar (2.78 ms) is 1103.0x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (2.52 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} (101634% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_cold_scalar, abi_cross_cold_madd_warm_scalar} with a 101634% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1103.0x the fastest

Fastest abi_cross_cold_madd_warm_null (2.52 us) to slowest abi_cross_cold_madd_warm_scalar (2.78 ms): 1103.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 2523.3 ns median
- 3 variants significantly slower than baseline
- Spread: 1102.99x (fastest 2523.3 ns, slowest 2783176.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5069ns | 5040ns | 4912ns | 5004ns | 5244ns | +5.96% |
| abi_cross_cold_madd_cold_scalar | 2759180ns | 2755080ns | 2750239ns | 2753926ns | 2771530ns | +57579.78% |
| abi_cross_cold_madd_warm_null | 4784ns | 4795ns | 4659ns | 4773ns | 4861ns | base |
| abi_cross_cold_madd_warm_scalar | 2827143ns | 2787372ns | 2751072ns | 2780742ns | 2934779ns | +59000.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 2720ns | 2664ns | 2771ns | +7.94% | 0.006 |
| abi_cross_cold_madd_cold_scalar | 2755562ns | 2746892ns | 2767732ns | +109259.99% | 0.000 |
| abi_cross_cold_madd_warm_null | 2520ns | 2479ns | 2545ns | base | 0.006 |
| abi_cross_cold_madd_warm_scalar | 2823141ns | 2747633ns | 2930581ns | +111941.99% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 31865.4 | 2818.8 | 2719.9 | n/a |
| abi_cross_cold_madd_cold_scalar | 81310.8 | 2759938.2 | 2755562.0 | n/a |
| abi_cross_cold_madd_warm_null | 28009.9 | 2605.9 | 2519.7 | n/a |
| abi_cross_cold_madd_warm_scalar | 82476.3 | 2811493.3 | 2823140.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.006 | 91.7% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.006 | 98.2% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5069ns | 5069ns | +5.96% |
| abi_cross_cold_madd_cold_scalar | 2759180ns | 2759180ns | +57579.78% |
| abi_cross_cold_madd_warm_null | 4784ns | 4784ns | base |
| abi_cross_cold_madd_warm_scalar | 2827143ns | 2827143ns | +59000.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 2523ns | base | --- | [2490, 2545] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 2705ns | +213.7ns (+8.5%) | [+139, +248]ns | [2684, 2771] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_cold_scalar | 2751487ns | +2748943.8ns (+108942.4%) | [+2744976, +2765207]ns | [2747467, 2767732] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2783176ns | +2780650.7ns (+110199.0%) | [+2753122, +2928091]ns | [2755665, 2930581] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 2543ns | +4.8% | +108835.8% | +109339.2% |
| 2 | 2479ns | +9.1% | +110715.4% | +121382.1% |
| 3 | 2508ns | +9.8% | +110130.0% | +110851.0% |
| 4 | 2548ns | +6.2% | +107787.5% | +107756.1% |
| 5 | 2502ns | +8.1% | +109729.4% | +113798.9% |
| 6 | 2538ns | +9.8% | +108419.1% | +108779.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.104 | ok |
| abi_cross_cold_madd_cold_scalar | -0.436 | moderate- |
| abi_cross_cold_madd_warm_null | -0.431 | moderate- |
| abi_cross_cold_madd_warm_scalar | -0.322 | moderate- |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 123647.8ns | 2719.9ns | 4546.1% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8359603.3ns | 2755562.0ns | 303.4% | HIGH |
| abi_cross_cold_madd_warm_null | 119055.2ns | 2519.7ns | 4724.9% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8513851.5ns | 2823140.7ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 2664.2-2770.9 ns)
   2664.2 |#############
   2669.5 |
   2674.9 |
   2680.2 |
   2685.5 |
   2690.9 |
   2696.2 |
   2701.5 |########################################
   2706.9 |
   2712.2 |
   2717.5 |
   2722.9 |
   2728.2 |
   2733.5 |
   2738.9 |
   2744.2 |
   2749.5 |#############
   2754.9 |
   2760.2 |
   2765.5 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2746892.1-2767732.5 ns)
  2746892.1 |####################
  2747934.1 |########################################
  2748976.1 |
  2750018.2 |
  2751060.2 |
  2752102.2 |
  2753144.2 |
  2754186.2 |####################
  2755228.3 |
  2756270.3 |
  2757312.3 |
  2758354.3 |
  2759396.3 |
  2760438.4 |
  2761480.4 |
  2762522.4 |
  2763564.4 |
  2764606.4 |####################
  2765648.5 |
  2766690.5 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 2478.8-2545.4 ns)
   2478.8 |########################################
   2482.1 |
   2485.5 |
   2488.8 |
   2492.1 |
   2495.5 |
   2498.8 |########################################
   2502.1 |
   2505.4 |########################################
   2508.8 |
   2512.1 |
   2515.4 |
   2518.8 |
   2522.1 |
   2525.4 |
   2528.8 |
   2532.1 |
   2535.4 |########################################
   2538.7 |
   2542.1 |########################################
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2747633.3-2930581.0 ns)
  2747633.3 |####################
  2756780.7 |####################
  2765928.1 |
  2775075.5 |########################################
  2784222.8 |
  2793370.2 |
  2802517.6 |
  2811665.0 |
  2820812.4 |
  2829959.8 |
  2839107.1 |
  2848254.5 |####################
  2857401.9 |
  2866549.3 |
  2875696.7 |
  2884844.1 |
  2893991.5 |
  2903138.8 |
  2912286.2 |
  2921433.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=4573.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=4726.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=302.6% of algo (FFI overhead may distort results)

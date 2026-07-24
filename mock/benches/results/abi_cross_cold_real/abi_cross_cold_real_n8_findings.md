# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 15% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (3.04 us) leads abi_cross_cold_real_cold_null (3.49 us) by 15%, a clear separation rather than a photo finish. CV 21.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_warm_scalar is an outlier: 724.0x slower than the field

abi_cross_cold_real_warm_scalar (2.20 ms) is 724.0x the fastest (3.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (3.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (62633% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 62633% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 724.0x the fastest

Fastest abi_cross_cold_real_warm_null (3.04 us) to slowest abi_cross_cold_real_warm_scalar (2.20 ms): 724.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_real_cold_null is inconsistent: worst-20% is 1.9x its best-20%

abi_cross_cold_real_cold_null's best 20% of batches run at 3.43 us but its worst 20% at 6.68 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 3036.9 ns median
- 3 variants significantly slower than baseline
- Spread: 723.96x (fastest 3036.9 ns, slowest 2198553.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 7104ns | 5807ns | 5692ns | 5785ns | 9787ns | +20.88% |
| abi_cross_cold_real_cold_scalar | 2329314ns | 2192430ns | 2178867ns | 2188618ns | 2615581ns | +39537.10% |
| abi_cross_cold_real_warm_null | 5877ns | 5347ns | 5232ns | 5328ns | 7021ns | base |
| abi_cross_cold_real_warm_scalar | 2502882ns | 2202311ns | 2179049ns | 2198470ns | 3121417ns | +42490.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 4533ns | 3426ns | 6679ns | +35.60% | 0.002 |
| abi_cross_cold_real_cold_scalar | 2325394ns | 2175469ns | 2610781ns | +69454.90% | 0.000 |
| abi_cross_cold_real_warm_null | 3343ns | 2980ns | 4006ns | base | 0.002 |
| abi_cross_cold_real_warm_scalar | 2498733ns | 2175748ns | 3116290ns | +74639.64% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 45462.1 | 5133.6 | 4533.4 | n/a |
| abi_cross_cold_real_cold_scalar | 88268.3 | 2357942.4 | 2325394.1 | n/a |
| abi_cross_cold_real_warm_null | 36643.9 | 3517.4 | 3343.2 | n/a |
| abi_cross_cold_real_warm_scalar | 89199.7 | 2577297.2 | 2498733.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.002 | 85.4% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_real_warm_null | 0.003 | 98.1% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 7104ns | 7104ns | +20.88% |
| abi_cross_cold_real_cold_scalar | 2329314ns | 2329314ns | +39537.10% |
| abi_cross_cold_real_warm_null | 5877ns | 5877ns | base |
| abi_cross_cold_real_warm_scalar | 2502882ns | 2502882ns | +42490.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 3037ns | base | --- | [2987, 4006] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 3489ns | +441.8ns (+14.5%) | [+403, +2725]ns | [3432, 6679] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2188870ns | +2185783.5ns (+71975.4%) | [+2173515, +2606854]ns | [2176531, 2610781] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2198554ns | +2195516.7ns (+72295.9%) | [+2178269, +3112384]ns | [2181356, 3116290] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 4818ns | +102.1% | +62587.9% | +83087.3% |
| 2 | 2994ns | +17.7% | +72568.2% | +74201.5% |
| 3 | 3194ns | +13.3% | +68440.8% | +68024.1% |
| 4 | 3038ns | +12.8% | +71580.8% | +72149.1% |
| 5 | 3036ns | +13.8% | +72403.8% | +72442.8% |
| 6 | 2980ns | +15.4% | +73345.8% | +73288.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.029 | ok |
| abi_cross_cold_real_cold_scalar | -0.049 | ok |
| abi_cross_cold_real_warm_null | -0.080 | ok |
| abi_cross_cold_real_warm_scalar | -0.016 | ok |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 142267.3ns | 4533.4ns | 3138.2% | HIGH |
| abi_cross_cold_real_cold_scalar | 7044296.7ns | 2325394.1ns | 302.9% | HIGH |
| abi_cross_cold_real_warm_null | 129700.0ns | 3343.2ns | 3879.5% | HIGH |
| abi_cross_cold_real_warm_scalar | 7698892.7ns | 2498733.1ns | 308.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 3426.2-6679.1 ns)
   3426.2 |########################################
   3588.8 |##########
   3751.5 |
   3914.1 |
   4076.8 |
   4239.4 |
   4402.1 |
   4564.7 |
   4727.4 |
   4890.0 |
   5052.7 |
   5215.3 |
   5378.0 |
   5540.6 |
   5703.3 |
   5865.9 |
   6028.6 |
   6191.2 |
   6353.9 |
   6516.5 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2175468.8-2610781.2 ns)
  2175468.8 |########################################
  2197234.4 |##########
  2219000.0 |
  2240765.7 |
  2262531.3 |
  2284296.9 |
  2306062.5 |
  2327828.2 |
  2349593.8 |
  2371359.4 |
  2393125.0 |
  2414890.6 |
  2436656.3 |
  2458421.9 |
  2480187.5 |
  2501953.1 |
  2523718.8 |
  2545484.4 |
  2567250.0 |
  2589015.6 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 2980.0-4006.1 ns)
   2980.0 |########################################
   3031.3 |########################################
   3082.6 |
   3133.9 |
   3185.2 |####################
   3236.5 |
   3287.8 |
   3339.1 |
   3390.4 |
   3441.7 |
   3493.0 |
   3544.3 |
   3595.6 |
   3646.9 |
   3698.2 |
   3749.5 |
   3800.8 |
   3852.1 |
   3903.4 |
   3954.7 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2175747.5-3116290.0 ns)
  2175747.5 |########################################
  2222774.6 |##########
  2269801.8 |
  2316828.9 |
  2363856.0 |
  2410883.1 |
  2457910.2 |
  2504937.4 |
  2551964.5 |
  2598991.6 |
  2646018.8 |
  2693045.9 |
  2740073.0 |
  2787100.1 |
  2834127.2 |
  2881154.4 |
  2928181.5 |
  2975208.6 |
  3022235.8 |
  3069262.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: CV=51.4% (high variance, measurements may be unstable)
- **abi_cross_cold_real_cold_null**: bridge=3547.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=3991.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: CV=27.0% (high variance, measurements may be unstable)
- **abi_cross_cold_real_warm_scalar**: bridge=303.6% of algo (FFI overhead may distort results)

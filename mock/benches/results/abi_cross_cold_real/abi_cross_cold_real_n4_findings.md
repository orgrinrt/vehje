# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 24% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (4.10 us) leads abi_cross_cold_real_cold_null (5.07 us) by 24%, a clear separation rather than a photo finish. CV 34.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_warm_scalar is an outlier: 539.1x slower than the field

abi_cross_cold_real_warm_scalar (2.21 ms) is 539.1x the fastest (4.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (4.10 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} (43069% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_cold_scalar, abi_cross_cold_real_warm_scalar} with a 43069% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 539.1x the fastest

Fastest abi_cross_cold_real_warm_null (4.10 us) to slowest abi_cross_cold_real_warm_scalar (2.21 ms): 539.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_real_cold_null is inconsistent: worst-20% is 2.4x its best-20%

abi_cross_cold_real_cold_null's best 20% of batches run at 4.82 us but its worst 20% at 11.78 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 4101.5 ns median
- 3 variants significantly slower than baseline
- Spread: 539.11x (fastest 4101.5 ns, slowest 2211120.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 9917ns | 7437ns | 7005ns | 7322ns | 15266ns | +28.11% |
| abi_cross_cold_real_cold_scalar | 2697424ns | 2192950ns | 2184946ns | 2190842ns | 3713537ns | +34745.49% |
| abi_cross_cold_real_warm_null | 7741ns | 6497ns | 6133ns | 6446ns | 10488ns | base |
| abi_cross_cold_real_warm_scalar | 2780297ns | 2214823ns | 2181056ns | 2208359ns | 3937824ns | +35816.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 7230ns | 4816ns | 11776ns | +50.20% | 0.001 |
| abi_cross_cold_real_cold_scalar | 2693323ns | 2181837ns | 3708054ns | +55851.21% | 0.000 |
| abi_cross_cold_real_warm_null | 4814ns | 3909ns | 6351ns | base | 0.001 |
| abi_cross_cold_real_warm_scalar | 2775753ns | 2178101ns | 3931171ns | +57563.60% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 58046.5 | 12171.2 | 7230.4 | n/a |
| abi_cross_cold_real_cold_scalar | 104206.4 | 2601665.6 | 2693323.5 | n/a |
| abi_cross_cold_real_warm_null | 43657.3 | 4734.2 | 4813.7 | n/a |
| abi_cross_cold_real_warm_scalar | 110847.8 | 2720516.1 | 2775752.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.001 | 77.1% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_real_warm_null | 0.001 | 95.3% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 9917ns | 9917ns | +28.11% |
| abi_cross_cold_real_cold_scalar | 2697424ns | 2697424ns | +34745.49% |
| abi_cross_cold_real_warm_null | 7741ns | 7741ns | base |
| abi_cross_cold_real_warm_scalar | 2780297ns | 2780297ns | +35816.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 4101ns | base | --- | [3989, 6351] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 5071ns | +915.3ns (+22.3%) | [+572, +5763]ns | [4844, 11776] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2189315ns | +2185310.8ns (+53281.4%) | [+2178178, +3702041]ns | [2182602, 3708054] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2211120ns | +2206678.8ns (+53802.4%) | [+2180883, +3925256]ns | [2184967, 3931171] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 4103ns | +26.4% | +53418.0% | +53350.6% |
| 2 | 4100ns | +22.5% | +53264.7% | +53364.5% |
| 3 | 4069ns | +19.8% | +53523.6% | +53431.8% |
| 4 | 4779ns | +7.1% | +45584.8% | +46539.6% |
| 5 | 3909ns | +23.2% | +55950.2% | +75789.8% |
| 6 | 7922ns | +131.8% | +65789.6% | +61698.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | -0.048 | ok |
| abi_cross_cold_real_cold_scalar | -0.032 | ok |
| abi_cross_cold_real_warm_null | -0.142 | ok |
| abi_cross_cold_real_warm_scalar | 0.224 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 137845.4ns | 7230.4ns | 1906.5% | HIGH |
| abi_cross_cold_real_cold_scalar | 7947178.6ns | 2693323.5ns | 295.1% | HIGH |
| abi_cross_cold_real_warm_null | 142885.0ns | 4813.7ns | 2968.3% | HIGH |
| abi_cross_cold_real_warm_scalar | 8605983.3ns | 2775752.8ns | 310.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 4815.8-11775.7 ns)
   4815.8 |########################################
   5163.8 |##########
   5511.8 |
   5859.8 |
   6207.8 |
   6555.8 |
   6903.8 |
   7251.7 |
   7599.7 |
   7947.7 |
   8295.7 |
   8643.7 |
   8991.7 |
   9339.7 |
   9687.7 |
  10035.7 |
  10383.7 |
  10731.7 |
  11079.7 |
  11427.7 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2181836.7-3708053.8 ns)
  2181836.7 |########################################
  2258147.6 |
  2334458.4 |
  2410769.3 |
  2487080.1 |
  2563391.0 |
  2639701.8 |
  2716012.7 |
  2792323.5 |
  2868634.4 |
  2944945.2 |
  3021256.1 |
  3097566.9 |
  3173877.8 |
  3250188.6 |
  3326499.5 |
  3402810.3 |
  3479121.2 |
  3555432.0 |
  3631742.9 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 3908.8-6350.9 ns)
   3908.8 |#############
   4030.9 |########################################
   4153.0 |
   4275.1 |
   4397.2 |
   4519.3 |
   4641.4 |
   4763.5 |#############
   4885.6 |
   5007.7 |
   5129.8 |
   5251.9 |
   5374.0 |
   5496.1 |
   5618.2 |
   5740.3 |
   5862.4 |
   5984.5 |
   6106.6 |
   6228.7 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2178101.2-3931171.5 ns)
  2178101.2 |########################################
  2265754.7 |
  2353408.2 |
  2441061.7 |
  2528715.2 |
  2616368.8 |
  2704022.3 |
  2791675.8 |
  2879329.3 |##########
  2966982.8 |
  3054636.3 |
  3142289.8 |
  3229943.4 |
  3317596.9 |
  3405250.4 |
  3492903.9 |
  3580557.4 |
  3668210.9 |
  3755864.4 |
  3843517.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: CV=68.9% (high variance, measurements may be unstable)
- **abi_cross_cold_real_cold_null**: bridge=2050.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: CV=42.0% (high variance, measurements may be unstable)
- **abi_cross_cold_real_cold_scalar**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: CV=29.4% (high variance, measurements may be unstable)
- **abi_cross_cold_real_warm_null**: bridge=3057.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: CV=35.6% (high variance, measurements may be unstable)
- **abi_cross_cold_real_warm_scalar**: bridge=303.6% of algo (FFI overhead may distort results)

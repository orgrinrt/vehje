# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_null dominates: 126% faster than the next best (abi_cross_cold_madd_cold_null)

abi_cross_cold_madd_warm_null (3.49 us) leads abi_cross_cold_madd_cold_null (7.89 us) by 126%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_madd_cold_scalar is an outlier: 804.3x slower than the field

abi_cross_cold_madd_cold_scalar (2.81 ms) is 804.3x the fastest (3.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (3.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} (35377% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} with a 35377% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 804.3x the fastest

Fastest abi_cross_cold_madd_warm_null (3.49 us) to slowest abi_cross_cold_madd_cold_scalar (2.81 ms): 804.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 3494.6 ns median
- 3 variants significantly slower than baseline
- Spread: 804.34x (fastest 3494.6 ns, slowest 2810791.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 10364ns | 10275ns | 9828ns | 10169ns | 10924ns | +79.44% |
| abi_cross_cold_madd_cold_scalar | 2822416ns | 2814809ns | 2790905ns | 2811557ns | 2854461ns | +48768.36% |
| abi_cross_cold_madd_warm_null | 5776ns | 5823ns | 5519ns | 5758ns | 5931ns | base |
| abi_cross_cold_madd_warm_scalar | 2823693ns | 2803307ns | 2787209ns | 2801238ns | 2875617ns | +48790.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 8022ns | 7653ns | 8490ns | +130.87% | 0.000 |
| abi_cross_cold_madd_cold_scalar | 2818460ns | 2787351ns | 2850229ns | +81018.82% | 0.000 |
| abi_cross_cold_madd_warm_null | 3474ns | 3338ns | 3568ns | base | 0.001 |
| abi_cross_cold_madd_warm_scalar | 2819590ns | 2783420ns | 2870898ns | +81051.36% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 35431.0 | 11186.4 | 8021.6 | n/a |
| abi_cross_cold_madd_cold_scalar | 87105.3 | 2817776.6 | 2818459.9 | n/a |
| abi_cross_cold_madd_warm_null | 29664.2 | 3559.9 | 3474.5 | n/a |
| abi_cross_cold_madd_warm_scalar | 84197.0 | 2817633.4 | 2819590.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.000 | 42.3% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.001 | 95.5% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 10364ns | 10364ns | +79.44% |
| abi_cross_cold_madd_cold_scalar | 2822416ns | 2822416ns | +48768.36% |
| abi_cross_cold_madd_warm_null | 5776ns | 5776ns | base |
| abi_cross_cold_madd_warm_scalar | 2823693ns | 2823693ns | +48790.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 3495ns | base | --- | [3361, 3568] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 7891ns | +4368.1ns (+125.0%) | [+4224, +5049]ns | [7684, 8490] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_cold_scalar | 2810791ns | +2807349.8ns (+80335.1%) | [+2790903, +2846703]ns | [2794360, 2850229] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2799425ns | +2795857.1ns (+80006.2%) | [+2785087, +2867404]ns | [2788448, 2870898] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 3591ns | +124.4% | +79761.3% | +77787.0% |
| 2 | 3338ns | +151.0% | +84276.9% | +83298.4% |
| 3 | 3384ns | +126.2% | +82690.1% | +82456.8% |
| 4 | 3460ns | +123.2% | +81754.1% | +84689.8% |
| 5 | 3545ns | +142.7% | +79030.8% | +78925.5% |
| 6 | 3529ns | +118.6% | +78890.9% | +79468.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | -0.459 | moderate- |
| abi_cross_cold_madd_cold_scalar | -0.023 | ok |
| abi_cross_cold_madd_warm_null | 0.012 | ok |
| abi_cross_cold_madd_warm_scalar | -0.190 | ok |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 119168.2ns | 8021.6ns | 1485.6% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8545416.0ns | 2818459.9ns | 303.2% | HIGH |
| abi_cross_cold_madd_warm_null | 122667.4ns | 3474.5ns | 3530.5% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8544735.3ns | 2819590.5ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 7652.9-8490.4 ns)
   7652.9 |####################
   7694.8 |########################################
   7736.6 |
   7778.5 |
   7820.4 |
   7862.3 |
   7904.2 |
   7946.0 |
   7987.9 |
   8029.8 |####################
   8071.7 |
   8113.5 |
   8155.4 |
   8197.3 |
   8239.2 |
   8281.0 |
   8322.9 |
   8364.8 |####################
   8406.7 |
   8448.5 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2787350.8-2850228.8 ns)
  2787350.8 |########################################
  2790494.7 |
  2793638.6 |
  2796782.5 |
  2799926.4 |########################################
  2803070.3 |########################################
  2806214.2 |
  2809358.1 |
  2812502.0 |
  2815645.9 |########################################
  2818789.8 |
  2821933.7 |
  2825077.6 |
  2828221.5 |
  2831365.4 |########################################
  2834509.3 |
  2837653.2 |
  2840797.1 |
  2843941.0 |
  2847084.9 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 3337.5-3568.3 ns)
   3337.5 |########################################
   3349.0 |
   3360.6 |
   3372.1 |
   3383.7 |########################################
   3395.2 |
   3406.7 |
   3418.3 |
   3429.8 |
   3441.4 |
   3452.9 |########################################
   3464.4 |
   3476.0 |
   3487.5 |
   3499.1 |
   3510.6 |
   3522.1 |########################################
   3533.7 |
   3545.2 |########################################
   3556.8 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2783420.4-2870898.1 ns)
  2783420.4 |########################################
  2787794.3 |
  2792168.2 |########################################
  2796542.1 |########################################
  2800915.9 |########################################
  2805289.8 |########################################
  2809663.7 |
  2814037.6 |
  2818411.5 |
  2822785.4 |
  2827159.2 |
  2831533.1 |
  2835907.0 |
  2840280.9 |
  2844654.8 |
  2849028.7 |
  2853402.6 |
  2857776.4 |
  2862150.3 |
  2866524.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=1497.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=3508.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=303.2% of algo (FFI overhead may distort results)

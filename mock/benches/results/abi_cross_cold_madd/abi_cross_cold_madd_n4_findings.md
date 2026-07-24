# abi_cross_cold (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_madd_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_madd_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_madd_warm_null dominates: 28% faster than the next best (abi_cross_cold_madd_cold_null)

abi_cross_cold_madd_warm_null (3.96 us) leads abi_cross_cold_madd_cold_null (5.08 us) by 28%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_madd_cold_scalar is an outlier: 697.6x slower than the field

abi_cross_cold_madd_cold_scalar (2.76 ms) is 697.6x the fastest (3.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_madd_warm_null)

The baseline abi_cross_cold_madd_warm_null is the fastest (3.96 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} vs {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} (54238% apart)

The field splits into a fast tier {abi_cross_cold_madd_warm_null, abi_cross_cold_madd_cold_null} and a slow tier {abi_cross_cold_madd_warm_scalar, abi_cross_cold_madd_cold_scalar} with a 54238% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 697.6x the fastest

Fastest abi_cross_cold_madd_warm_null (3.96 us) to slowest abi_cross_cold_madd_cold_scalar (2.76 ms): 697.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_madd_warm_null) is the fastest** at 3962.3 ns median
- 3 variants significantly slower than baseline
- Spread: 697.59x (fastest 3962.3 ns, slowest 2764047.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 7337ns | 7349ns | 7101ns | 7271ns | 7555ns | +17.99% |
| abi_cross_cold_madd_cold_scalar | 2772807ns | 2767634ns | 2764105ns | 2766547ns | 2786549ns | +44486.42% |
| abi_cross_cold_madd_warm_null | 6219ns | 6216ns | 6150ns | 6206ns | 6273ns | base |
| abi_cross_cold_madd_warm_scalar | 2765977ns | 2764832ns | 2756399ns | 2764377ns | 2773167ns | +44376.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 5063ns | 4901ns | 5194ns | +27.50% | 0.001 |
| abi_cross_cold_madd_cold_scalar | 2769203ns | 2760501ns | 2782819ns | +69643.56% | 0.000 |
| abi_cross_cold_madd_warm_null | 3971ns | 3921ns | 4016ns | base | 0.001 |
| abi_cross_cold_madd_warm_scalar | 2762365ns | 2752781ns | 2769448ns | +69471.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 33123.7 | 6688.5 | 5062.6 | n/a |
| abi_cross_cold_madd_cold_scalar | 79581.0 | 2773725.1 | 2769203.0 | n/a |
| abi_cross_cold_madd_warm_null | 26986.1 | 4132.3 | 3970.5 | n/a |
| abi_cross_cold_madd_warm_scalar | 72357.5 | 2763731.6 | 2762364.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_madd_warm_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.001 | 77.2% |
| abi_cross_cold_madd_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_madd_warm_null | 0.001 | 99.0% |
| abi_cross_cold_madd_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_madd_cold_null | 7337ns | 7337ns | +17.99% |
| abi_cross_cold_madd_cold_scalar | 2772807ns | 2772807ns | +44486.42% |
| abi_cross_cold_madd_warm_null | 6219ns | 6219ns | base |
| abi_cross_cold_madd_warm_scalar | 2765977ns | 2765977ns | +44376.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_madd_warm_null | 3962ns | base | --- | [3933, 4016] | --- | --- | --- | --- |
| abi_cross_cold_madd_cold_null | 5082ns | +1077.9ns (+27.2%) | [+955, +1244]ns | [4912, 5194] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_cold_scalar | 2764047ns | +2760114.4ns (+69659.4%) | [+2756776, +2778807]ns | [2760742, 2782819] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_madd_warm_scalar | 2761280ns | +2757313.5ns (+69588.7%) | [+2752368, +2765501]ns | [2756366, 2769448] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_madd_warm_null | abi_cross_cold_madd_cold_null | abi_cross_cold_madd_cold_scalar | abi_cross_cold_madd_warm_scalar |
|---|---|---|---|---|
| 1 | 3921ns | +33.1% | +70420.6% | +70627.1% |
| 2 | 3980ns | +29.9% | +69252.4% | +69257.7% |
| 3 | 4052ns | +23.5% | +68289.8% | +67828.0% |
| 4 | 3945ns | +24.2% | +69941.1% | +69860.8% |
| 5 | 3952ns | +24.6% | +69761.2% | +69783.0% |
| 6 | 3972ns | +29.8% | +70237.1% | +69524.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_madd_cold_null | 0.311 | moderate+ |
| abi_cross_cold_madd_cold_scalar | -0.184 | ok |
| abi_cross_cold_madd_warm_null | -0.130 | ok |
| abi_cross_cold_madd_warm_scalar | 0.091 | ok |

**Consistency summary:**

- **abi_cross_cold_madd_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_madd_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_madd_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_madd_cold_null | 116734.4ns | 5062.6ns | 2305.8% | HIGH |
| abi_cross_cold_madd_cold_scalar | 8398552.7ns | 2769203.0ns | 303.3% | HIGH |
| abi_cross_cold_madd_warm_null | 121479.6ns | 3970.5ns | 3059.5% | HIGH |
| abi_cross_cold_madd_warm_scalar | 8364358.9ns | 2762364.7ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_madd_cold_null (n=6, range 4901.2-5194.2 ns)
   4901.2 |########################################
   4915.8 |########################################
   4930.5 |
   4945.1 |
   4959.8 |
   4974.4 |
   4989.1 |
   5003.8 |########################################
   5018.4 |
   5033.1 |
   5047.7 |
   5062.3 |
   5077.0 |
   5091.6 |
   5106.3 |
   5120.9 |
   5135.6 |
   5150.2 |########################################
   5164.9 |########################################
   5179.6 |
  (0 below, 1 above range)

abi_cross_cold_madd_cold_scalar (n=6, range 2760501.2-2782819.3 ns)
  2760501.2 |########################################
  2761617.1 |
  2762733.0 |####################
  2763848.9 |
  2764964.8 |####################
  2766080.7 |
  2767196.6 |
  2768312.6 |
  2769428.5 |
  2770544.4 |####################
  2771660.3 |
  2772776.2 |
  2773892.1 |
  2775008.0 |
  2776123.9 |
  2777239.8 |
  2778355.7 |
  2779471.6 |
  2780587.5 |
  2781703.4 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_null (n=6, range 3920.8-4016.4 ns)
   3920.8 |########################################
   3925.6 |
   3930.4 |
   3935.1 |
   3939.9 |
   3944.7 |########################################
   3949.5 |########################################
   3954.3 |
   3959.1 |
   3963.8 |
   3968.6 |########################################
   3973.4 |
   3978.2 |########################################
   3983.0 |
   3987.8 |
   3992.5 |
   3997.3 |
   4002.1 |
   4006.9 |
   4011.7 |
  (0 below, 1 above range)

abi_cross_cold_madd_warm_scalar (n=6, range 2752780.8-2769447.9 ns)
  2752780.8 |########################################
  2753614.2 |
  2754447.5 |
  2755280.9 |
  2756114.2 |
  2756947.6 |
  2757780.9 |
  2758614.3 |
  2759447.6 |########################################
  2760281.0 |########################################
  2761114.3 |########################################
  2761947.7 |
  2762781.1 |
  2763614.4 |
  2764447.8 |
  2765281.1 |########################################
  2766114.5 |
  2766947.8 |
  2767781.2 |
  2768614.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_madd_cold_null**: bridge=2308.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_cold_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_null**: bridge=3065.0% of algo (FFI overhead may distort results)
- **abi_cross_cold_madd_warm_scalar**: bridge=302.8% of algo (FFI overhead may distort results)

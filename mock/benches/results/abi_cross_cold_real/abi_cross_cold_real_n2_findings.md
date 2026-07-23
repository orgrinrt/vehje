# abi_cross_cold (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_real_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_real_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_real_warm_null dominates: 281% faster than the next best (abi_cross_cold_real_cold_null)

abi_cross_cold_real_warm_null (3.60 us) leads abi_cross_cold_real_cold_null (13.74 us) by 281%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_cold_real_cold_scalar is an outlier: 610.9x slower than the field

abi_cross_cold_real_cold_scalar (2.20 ms) is 610.9x the fastest (3.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_real_warm_null)

The baseline abi_cross_cold_real_warm_null is the fastest (3.60 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} vs {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} (15805% apart)

The field splits into a fast tier {abi_cross_cold_real_warm_null, abi_cross_cold_real_cold_null} and a slow tier {abi_cross_cold_real_warm_scalar, abi_cross_cold_real_cold_scalar} with a 15805% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 610.9x the fastest

Fastest abi_cross_cold_real_warm_null (3.60 us) to slowest abi_cross_cold_real_cold_scalar (2.20 ms): 610.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_real_warm_null) is the fastest** at 3601.4 ns median
- 3 variants significantly slower than baseline
- Spread: 610.94x (fastest 3601.4 ns, slowest 2200284.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 15949ns | 16030ns | 14540ns | 15884ns | 16750ns | +171.38% |
| abi_cross_cold_real_cold_scalar | 2205162ns | 2203661ns | 2196652ns | 2201512ns | 2214892ns | +37423.50% |
| abi_cross_cold_real_warm_null | 5877ns | 5819ns | 5702ns | 5797ns | 6083ns | base |
| abi_cross_cold_real_warm_scalar | 2188360ns | 2188084ns | 2181201ns | 2186907ns | 2194119ns | +37137.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 13647ns | 12161ns | 14469ns | +277.14% | 0.000 |
| abi_cross_cold_real_cold_scalar | 2201713ns | 2193346ns | 2211294ns | +60746.31% | 0.000 |
| abi_cross_cold_real_warm_null | 3618ns | 3512ns | 3726ns | base | 0.001 |
| abi_cross_cold_real_warm_scalar | 2185147ns | 2178109ns | 2190904ns | +60288.48% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 34488.2 | 19509.8 | 13646.6 | n/a |
| abi_cross_cold_real_cold_scalar | 70652.4 | 2200706.3 | 2201713.5 | n/a |
| abi_cross_cold_real_warm_null | 27969.0 | 3696.2 | 3618.5 | n/a |
| abi_cross_cold_real_warm_scalar | 63362.6 | 2184827.5 | 2185147.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_cold_real_warm_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.000 | 25.6% |
| abi_cross_cold_real_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_real_warm_null | 0.001 | 97.5% |
| abi_cross_cold_real_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_real_cold_null | 15949ns | 15949ns | +171.38% |
| abi_cross_cold_real_cold_scalar | 2205162ns | 2205162ns | +37423.50% |
| abi_cross_cold_real_warm_null | 5877ns | 5877ns | base |
| abi_cross_cold_real_warm_scalar | 2188360ns | 2188360ns | +37137.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_real_warm_null | 3601ns | base | --- | [3528, 3726] | --- | --- | --- | --- |
| abi_cross_cold_real_cold_null | 13736ns | +10159.8ns (+282.1%) | [+9097, +10827]ns | [12734, 14469] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_cold_scalar | 2200285ns | +2196723.2ns (+60995.5%) | [+2189884, +2207677]ns | [2193561, 2211294] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_cold_real_warm_scalar | 2184847ns | +2181169.8ns (+60563.7%) | [+2176040, +2187376]ns | [2179690, 2190904] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_real_warm_null | abi_cross_cold_real_cold_null | abi_cross_cold_real_cold_scalar | abi_cross_cold_real_warm_scalar |
|---|---|---|---|---|
| 1 | 3545ns | +243.1% | +61941.1% | +61624.8% |
| 2 | 3730ns | +256.8% | +58709.2% | +58521.0% |
| 3 | 3578ns | +272.1% | +61422.5% | +60769.9% |
| 4 | 3512ns | +303.2% | +62599.6% | +62374.4% |
| 5 | 3625ns | +292.0% | +60424.7% | +60137.4% |
| 6 | 3722ns | +295.7% | +59564.4% | +58503.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_real_cold_null | 0.321 | moderate+ |
| abi_cross_cold_real_cold_scalar | -0.255 | moderate- |
| abi_cross_cold_real_warm_null | -0.202 | moderate- |
| abi_cross_cold_real_warm_scalar | -0.492 | moderate- |

**Consistency summary:**

- **abi_cross_cold_real_cold_null**: won 0/6, lost 6/6
- **abi_cross_cold_real_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_real_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_real_cold_null | 147375.4ns | 13646.6ns | 1079.9% | HIGH |
| abi_cross_cold_real_cold_scalar | 6684018.9ns | 2201713.5ns | 303.6% | HIGH |
| abi_cross_cold_real_warm_null | 122000.8ns | 3618.5ns | 3371.6% | HIGH |
| abi_cross_cold_real_warm_scalar | 6625164.1ns | 2185147.2ns | 303.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_real_cold_null (n=6, range 12161.2-14469.0 ns)
  12161.2 |####################
  12276.6 |
  12392.0 |
  12507.4 |
  12622.8 |
  12738.1 |
  12853.5 |
  12968.9 |
  13084.3 |
  13199.7 |########################################
  13315.1 |
  13430.5 |
  13545.9 |
  13661.2 |
  13776.6 |
  13892.0 |
  14007.4 |
  14122.8 |########################################
  14238.2 |
  14353.6 |
  (0 below, 1 above range)

abi_cross_cold_real_cold_scalar (n=6, range 2193346.2-2211294.3 ns)
  2193346.2 |########################################
  2194243.6 |
  2195141.0 |
  2196038.4 |
  2196935.8 |
  2197833.2 |
  2198730.6 |####################
  2199628.1 |
  2200525.5 |
  2201422.9 |########################################
  2202320.3 |
  2203217.7 |
  2204115.1 |
  2205012.5 |
  2205909.9 |
  2206807.3 |
  2207704.7 |
  2208602.1 |
  2209499.5 |
  2210396.9 |
  (0 below, 1 above range)

abi_cross_cold_real_warm_null (n=6, range 3511.7-3725.8 ns)
   3511.7 |########################################
   3522.4 |
   3533.1 |
   3543.8 |########################################
   3554.5 |
   3565.2 |
   3575.9 |########################################
   3586.7 |
   3597.4 |
   3608.1 |
   3618.8 |########################################
   3629.5 |
   3640.2 |
   3650.9 |
   3661.6 |
   3672.3 |
   3683.0 |
   3693.7 |
   3704.4 |
   3715.1 |########################################
  (0 below, 1 above range)

abi_cross_cold_real_warm_scalar (n=6, range 2178108.8-2190904.3 ns)
  2178108.8 |########################################
  2178748.6 |
  2179388.4 |
  2180028.1 |
  2180667.9 |########################################
  2181307.7 |
  2181947.5 |
  2182587.2 |
  2183227.0 |########################################
  2183866.8 |
  2184506.6 |
  2185146.4 |
  2185786.1 |########################################
  2186425.9 |
  2187065.7 |
  2187705.5 |########################################
  2188345.2 |
  2188985.0 |
  2189624.8 |
  2190264.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_real_cold_null**: bridge=1070.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_cold_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_null**: bridge=3381.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_real_warm_scalar**: bridge=303.1% of algo (FFI overhead may distort results)

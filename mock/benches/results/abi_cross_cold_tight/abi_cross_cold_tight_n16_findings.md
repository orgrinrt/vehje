# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_warm_scalar is an outlier: 799.2x slower than the field

abi_cross_cold_tight_warm_scalar (2.14 ms) is 799.2x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (abi_cross_cold_tight_warm_null)

The baseline abi_cross_cold_tight_warm_null is the fastest (2.68 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} vs {abi_cross_cold_tight_cold_scalar, abi_cross_cold_tight_warm_scalar} (74329% apart)

The field splits into a fast tier {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} and a slow tier {abi_cross_cold_tight_cold_scalar, abi_cross_cold_tight_warm_scalar} with a 74329% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 799.2x the fastest

Fastest abi_cross_cold_tight_warm_null (2.68 us) to slowest abi_cross_cold_tight_warm_scalar (2.14 ms): 799.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (abi_cross_cold_tight_warm_null) is the fastest** at 2680.8 ns median
- 2 variants significantly slower than baseline
- Spread: 799.24x (fastest 2680.8 ns, slowest 2142607.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5876ns | 5195ns | 4976ns | 5133ns | 7441ns | +10.27% |
| abi_cross_cold_tight_cold_scalar | 2295043ns | 2092898ns | 2054022ns | 2080219ns | 2737789ns | +42966.78% |
| abi_cross_cold_tight_warm_null | 5329ns | 5076ns | 4908ns | 5073ns | 5924ns | base |
| abi_cross_cold_tight_warm_scalar | 2172519ns | 2148074ns | 2051184ns | 2138994ns | 2283474ns | +40667.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 2928ns | 2697ns | 3267ns | +4.32% | 0.005 |
| abi_cross_cold_tight_cold_scalar | 2290759ns | 2050451ns | 2732506ns | +81508.82% | 0.000 |
| abi_cross_cold_tight_warm_null | 2807ns | 2558ns | 3153ns | base | 0.006 |
| abi_cross_cold_tight_warm_scalar | 2167734ns | 2047287ns | 2278509ns | +77126.00% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 38694.9 | 3049.7 | 2928.3 | n/a |
| abi_cross_cold_tight_cold_scalar | 106526.7 | 2236484.7 | 2290759.4 | n/a |
| abi_cross_cold_tight_warm_null | 31582.3 | 2852.6 | 2807.0 | n/a |
| abi_cross_cold_tight_warm_scalar | 89997.8 | 2155569.4 | 2167733.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.006 | 91.1% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_tight_warm_null | 0.006 | 95.4% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5876ns | 5876ns | +10.27% |
| abi_cross_cold_tight_cold_scalar | 2295043ns | 2295043ns | +42966.78% |
| abi_cross_cold_tight_warm_null | 5329ns | 5329ns | base |
| abi_cross_cold_tight_warm_scalar | 2172519ns | 2172519ns | +40667.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 2681ns | base | --- | [2587, 3153] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 2807ns | no significant difference | [-23, +216]ns | [2712, 3267] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_tight_cold_scalar | 2088956ns | +2086368.8ns (+77826.3%) | [+2048136, +2729353]ns | [2050817, 2732506] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2142607ns | +2139954.8ns (+79825.2%) | [+2079469, +2275356]ns | [2082085, 2278509] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 2558ns | +6.6% | +80455.2% | +79950.3% |
| 2 | 2673ns | +0.9% | +76601.1% | +79086.1% |
| 3 | 2688ns | +6.4% | +76200.3% | +79058.9% |
| 4 | 2617ns | +7.3% | +80830.6% | +82339.2% |
| 5 | 3430ns | +7.1% | +76170.6% | +64486.9% |
| 6 | 2876ns | -2.4% | +98955.1% | +81318.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.162 | ok |
| abi_cross_cold_tight_cold_scalar | 0.455 | moderate+ |
| abi_cross_cold_tight_warm_null | -0.007 | ok |
| abi_cross_cold_tight_warm_scalar | 0.319 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 1/6, lost 5/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 130280.5ns | 2928.3ns | 4449.0% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6788836.2ns | 2290759.4ns | 296.4% | HIGH |
| abi_cross_cold_tight_warm_null | 126101.0ns | 2807.0ns | 4492.4% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6593744.5ns | 2167733.7ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 2697.1-3266.7 ns)
   2697.1 |####################
   2725.6 |####################
   2754.1 |
   2782.5 |########################################
   2811.0 |
   2839.5 |####################
   2868.0 |
   2896.4 |
   2924.9 |
   2953.4 |
   2981.9 |
   3010.4 |
   3038.8 |
   3067.3 |
   3095.8 |
   3124.3 |
   3152.7 |
   3181.2 |
   3209.7 |
   3238.2 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2050451.2-2732505.9 ns)
  2050451.2 |########################################
  2084553.9 |#############
  2118656.7 |
  2152759.4 |
  2186862.1 |
  2220964.9 |
  2255067.6 |
  2289170.3 |
  2323273.1 |
  2357375.8 |
  2391478.5 |
  2425581.3 |
  2459684.0 |
  2493786.7 |
  2527889.5 |
  2561992.2 |
  2596094.9 |#############
  2630197.7 |
  2664300.4 |
  2698403.1 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 2557.5-3153.1 ns)
   2557.5 |########################################
   2587.3 |########################################
   2617.1 |
   2646.8 |########################################
   2676.6 |########################################
   2706.4 |
   2736.2 |
   2766.0 |
   2795.7 |
   2825.5 |
   2855.3 |########################################
   2885.1 |
   2914.9 |
   2944.6 |
   2974.4 |
   3004.2 |
   3034.0 |
   3063.8 |
   3093.5 |
   3123.3 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2047286.7-2278509.0 ns)
  2047286.7 |####################
  2058847.8 |
  2070408.9 |
  2081970.0 |
  2093531.1 |
  2105092.3 |
  2116653.4 |########################################
  2128214.5 |
  2139775.6 |
  2151336.7 |####################
  2162897.8 |
  2174458.9 |
  2186020.1 |
  2197581.2 |
  2209142.3 |####################
  2220703.4 |
  2232264.5 |
  2243825.6 |
  2255386.7 |
  2266947.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=4476.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=305.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=4551.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=303.8% of algo (FFI overhead may distort results)

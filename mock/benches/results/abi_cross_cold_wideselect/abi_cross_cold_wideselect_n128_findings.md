# abi_cross_cold (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_wideselect_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_wideselect_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_wideselect_warm_scalar is an outlier: 774.7x slower than the field

abi_cross_cold_wideselect_warm_scalar (2.10 ms) is 774.7x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_wideselect_cold_null, abi_cross_cold_wideselect_warm_null} vs {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} (76443% apart)

The field splits into a fast tier {abi_cross_cold_wideselect_cold_null, abi_cross_cold_wideselect_warm_null} and a slow tier {abi_cross_cold_wideselect_cold_scalar, abi_cross_cold_wideselect_warm_scalar} with a 76443% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 774.7x the fastest

Fastest abi_cross_cold_wideselect_cold_null (2.71 us) to slowest abi_cross_cold_wideselect_warm_scalar (2.10 ms): 774.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_wideselect_cold_null** at 2715.0 ns median (-1.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 774.71x (fastest 2715.0 ns, slowest 2103343.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 5016ns | 5018ns | 4878ns | 5011ns | 5093ns | -1.44% |
| abi_cross_cold_wideselect_cold_scalar | 2122078ns | 2104257ns | 2088078ns | 2100867ns | 2170895ns | +41598.09% |
| abi_cross_cold_wideselect_warm_null | 5089ns | 5027ns | 4930ns | 5005ns | 5294ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2112745ns | 2106862ns | 2091192ns | 2103794ns | 2136948ns | +41414.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 2702ns | 2624ns | 2733ns | -2.48% | 0.047 |
| abi_cross_cold_wideselect_cold_scalar | 2118370ns | 2084605ns | 2166944ns | +76339.56% | 0.000 |
| abi_cross_cold_wideselect_warm_null | 2771ns | 2670ns | 2893ns | base | 0.046 |
| abi_cross_cold_wideselect_warm_scalar | 2109127ns | 2087858ns | 2132937ns | +76006.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 34671.5 | 2760.2 | 2702.5 | 71 |
| abi_cross_cold_wideselect_cold_scalar | 85635.2 | 2105628.0 | 2118369.5 | n/a |
| abi_cross_cold_wideselect_warm_null | 29820.3 | 2802.8 | 2771.3 | n/a |
| abi_cross_cold_wideselect_warm_scalar | 73602.6 | 2099233.8 | 2109126.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_wideselect_cold_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | 0.047 | 96.7% |
| abi_cross_cold_wideselect_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_wideselect_warm_null | 0.047 | 95.6% |
| abi_cross_cold_wideselect_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 5016ns | 5016ns | -1.44% |
| abi_cross_cold_wideselect_cold_scalar | 2122078ns | 2122078ns | +41598.09% |
| abi_cross_cold_wideselect_warm_null | 5089ns | 5089ns | base |
| abi_cross_cold_wideselect_warm_scalar | 2112745ns | 2112745ns | +41414.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_wideselect_warm_null | 2744ns | base | --- | [2676, 2893] | --- | --- | --- | --- |
| abi_cross_cold_wideselect_cold_null | 2715ns | no significant difference | [-167, +11]ns | [2660, 2733] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_wideselect_cold_scalar | 2100606ns | +2097886.0ns (+76443.8%) | [+2084714, +2164195]ns | [2087558, 2166944] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_wideselect_warm_scalar | 2103343ns | +2100617.7ns (+76543.4%) | [+2088399, +2130049]ns | [2091100, 2132937] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_wideselect_warm_null | abi_cross_cold_wideselect_cold_null | abi_cross_cold_wideselect_cold_scalar | abi_cross_cold_wideselect_warm_scalar |
|---|---|---|---|---|
| 1 | 2670ns | -1.7% | +78701.4% | +78841.5% |
| 2 | 3005ns | -8.9% | +69467.7% | +70256.7% |
| 3 | 2781ns | -2.4% | +79958.6% | +75380.5% |
| 4 | 2683ns | +0.5% | +77588.1% | +77709.3% |
| 5 | 2718ns | +0.4% | +77445.4% | +76968.7% |
| 6 | 2771ns | -2.0% | +75590.3% | +77543.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_wideselect_cold_null | -0.276 | moderate- |
| abi_cross_cold_wideselect_cold_scalar | -0.393 | moderate- |
| abi_cross_cold_wideselect_warm_null | -0.232 | moderate- |
| abi_cross_cold_wideselect_warm_scalar | -0.063 | ok |

**Consistency summary:**

- **abi_cross_cold_wideselect_cold_null**: won 4/6, lost 2/6
- **abi_cross_cold_wideselect_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_wideselect_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_wideselect_cold_null | 126976.4ns | 2702.5ns | 4698.5% | HIGH |
| abi_cross_cold_wideselect_cold_scalar | 6412344.4ns | 2118369.5ns | 302.7% | HIGH |
| abi_cross_cold_wideselect_warm_null | 122176.2ns | 2771.3ns | 4408.6% | HIGH |
| abi_cross_cold_wideselect_warm_scalar | 6384067.9ns | 2109126.7ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_wideselect_cold_null (n=6, range 2624.2-2732.7 ns)
   2624.2 |####################
   2629.6 |
   2635.0 |
   2640.5 |
   2645.9 |
   2651.3 |
   2656.8 |
   2662.2 |
   2667.6 |
   2673.0 |
   2678.4 |
   2683.9 |
   2689.3 |
   2694.7 |####################
   2700.1 |
   2705.6 |
   2711.0 |########################################
   2716.4 |
   2721.8 |####################
   2727.3 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_cold_scalar (n=6, range 2084605.4-2166944.0 ns)
  2084605.4 |########################################
  2088722.3 |########################################
  2092839.3 |
  2096956.2 |########################################
  2101073.1 |########################################
  2105190.0 |########################################
  2109307.0 |
  2113423.9 |
  2117540.8 |
  2121657.7 |
  2125774.7 |
  2129891.6 |
  2134008.5 |
  2138125.5 |
  2142242.4 |
  2146359.3 |
  2150476.2 |
  2154593.2 |
  2158710.1 |
  2162827.0 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_null (n=6, range 2669.6-2893.1 ns)
   2669.6 |####################
   2680.8 |####################
   2691.9 |
   2703.1 |
   2714.3 |####################
   2725.5 |
   2736.7 |
   2747.8 |
   2759.0 |
   2770.2 |########################################
   2781.3 |
   2792.5 |
   2803.7 |
   2814.9 |
   2826.0 |
   2837.2 |
   2848.4 |
   2859.6 |
   2870.8 |
   2881.9 |
  (0 below, 1 above range)

abi_cross_cold_wideselect_warm_scalar (n=6, range 2087857.9-2132937.2 ns)
  2087857.9 |########################################
  2090111.9 |
  2092365.8 |########################################
  2094619.8 |
  2096873.8 |
  2099127.7 |########################################
  2101381.7 |
  2103635.7 |
  2105889.6 |########################################
  2108143.6 |
  2110397.6 |
  2112651.5 |########################################
  2114905.5 |
  2117159.5 |
  2119413.4 |
  2121667.4 |
  2123921.4 |
  2126175.3 |
  2128429.3 |
  2130683.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_wideselect_cold_null**: bridge=4685.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_cold_scalar**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_null**: bridge=4422.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_wideselect_warm_scalar**: bridge=303.7% of algo (FFI overhead may distort results)

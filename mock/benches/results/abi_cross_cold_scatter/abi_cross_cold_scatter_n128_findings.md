# abi_cross_cold (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_scatter_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_scatter_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_scatter_cold_scalar is an outlier: 828.1x slower than the field

abi_cross_cold_scatter_cold_scalar (2.22 ms) is 828.1x the fastest (2.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_scatter_cold_null, abi_cross_cold_scatter_warm_null} vs {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} (81078% apart)

The field splits into a fast tier {abi_cross_cold_scatter_cold_null, abi_cross_cold_scatter_warm_null} and a slow tier {abi_cross_cold_scatter_warm_scalar, abi_cross_cold_scatter_cold_scalar} with a 81078% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 828.1x the fastest

Fastest abi_cross_cold_scatter_cold_null (2.69 us) to slowest abi_cross_cold_scatter_cold_scalar (2.22 ms): 828.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_scatter_cold_null's edge over baseline is significant but tiny (-45 ns, 1.66%)

abi_cross_cold_scatter_cold_null differs from baseline abi_cross_cold_scatter_warm_null by -45 ns (1.66%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_scatter_cold_null** at 2686.4 ns median (-1.7% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 828.09x (fastest 2686.4 ns, slowest 2224631.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5004ns | 4957ns | 4892ns | 4937ns | 5162ns | -0.85% |
| abi_cross_cold_scatter_cold_scalar | 2238293ns | 2228373ns | 2175072ns | 2211025ns | 2310807ns | +44245.76% |
| abi_cross_cold_scatter_warm_null | 5047ns | 5068ns | 4815ns | 5019ns | 5205ns | base |
| abi_cross_cold_scatter_warm_scalar | 2228119ns | 2222404ns | 2185052ns | 2220065ns | 2261732ns | +44044.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 2703ns | 2652ns | 2766ns | -1.50% | 0.047 |
| abi_cross_cold_scatter_cold_scalar | 2234562ns | 2171722ns | 2306776ns | +81321.11% | 0.000 |
| abi_cross_cold_scatter_warm_null | 2744ns | 2632ns | 2830ns | base | 0.047 |
| abi_cross_cold_scatter_warm_scalar | 2224272ns | 2181465ns | 2257676ns | +80946.20% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 32996.8 | 2743.2 | 2703.4 | 134 |
| abi_cross_cold_scatter_cold_scalar | 83097.1 | 2221915.5 | 2234561.7 | n/a |
| abi_cross_cold_scatter_warm_null | 27502.7 | 2768.5 | 2744.5 | n/a |
| abi_cross_cold_scatter_warm_scalar | 77117.9 | 2219530.2 | 2224272.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_scatter_warm_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.048 | 98.0% |
| abi_cross_cold_scatter_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_scatter_warm_null | 0.047 | 96.3% |
| abi_cross_cold_scatter_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 5004ns | 5004ns | -0.85% |
| abi_cross_cold_scatter_cold_scalar | 2238293ns | 2238293ns | +44245.76% |
| abi_cross_cold_scatter_warm_null | 5047ns | 5047ns | base |
| abi_cross_cold_scatter_warm_scalar | 2228119ns | 2228119ns | +44044.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_scatter_warm_null | 2733ns | base | --- | [2670, 2830] | --- | --- | --- | --- |
| abi_cross_cold_scatter_cold_null | 2686ns | no significant difference | [-130, +52]ns | [2658, 2766] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_scatter_cold_scalar | 2224632ns | +2221823.5ns (+81299.1%) | [+2169607, +2304022]ns | [2172277, 2306776] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_scatter_warm_scalar | 2218509ns | +2215754.4ns (+81077.0%) | [+2193940, +2254890]ns | [2196632, 2257676] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_scatter_warm_null | abi_cross_cold_scatter_cold_null | abi_cross_cold_scatter_cold_scalar | abi_cross_cold_scatter_warm_scalar |
|---|---|---|---|---|
| 1 | 2632ns | +1.3% | +82463.8% | +82791.9% |
| 2 | 2709ns | -1.7% | +80061.0% | +83811.7% |
| 3 | 2864ns | -7.4% | +77371.6% | +78177.3% |
| 4 | 2796ns | -1.7% | +79735.9% | +79132.4% |
| 5 | 2752ns | -1.7% | +80928.9% | +80256.0% |
| 6 | 2713ns | +2.6% | +87671.3% | +81786.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_scatter_cold_null | 0.110 | ok |
| abi_cross_cold_scatter_cold_scalar | 0.145 | ok |
| abi_cross_cold_scatter_warm_null | 0.190 | ok |
| abi_cross_cold_scatter_warm_scalar | -0.260 | moderate- |

**Consistency summary:**

- **abi_cross_cold_scatter_cold_null**: won 4/6, lost 2/6
- **abi_cross_cold_scatter_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_scatter_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_scatter_cold_null | 125168.0ns | 2703.4ns | 4630.0% | HIGH |
| abi_cross_cold_scatter_cold_scalar | 6766317.6ns | 2234561.7ns | 302.8% | HIGH |
| abi_cross_cold_scatter_warm_null | 120155.8ns | 2744.5ns | 4378.1% | HIGH |
| abi_cross_cold_scatter_warm_scalar | 6745123.8ns | 2224272.4ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_scatter_cold_null (n=6, range 2651.7-2765.8 ns)
   2651.7 |####################
   2657.4 |
   2663.1 |########################################
   2668.8 |
   2674.5 |
   2680.2 |
   2685.9 |
   2691.7 |
   2697.4 |
   2703.1 |####################
   2708.8 |
   2714.5 |
   2720.2 |
   2725.9 |
   2731.6 |
   2737.3 |
   2743.0 |
   2748.7 |####################
   2754.4 |
   2760.1 |
  (0 below, 1 above range)

abi_cross_cold_scatter_cold_scalar (n=6, range 2171722.1-2306776.0 ns)
  2171722.1 |########################################
  2178474.8 |
  2185227.5 |
  2191980.2 |
  2198732.9 |
  2205485.6 |
  2212238.3 |####################
  2218991.0 |
  2225743.7 |########################################
  2232496.4 |
  2239249.1 |
  2246001.8 |
  2252754.5 |
  2259507.2 |
  2266259.9 |
  2273012.6 |
  2279765.3 |
  2286518.0 |
  2293270.7 |
  2300023.4 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_null (n=6, range 2631.7-2830.0 ns)
   2631.7 |########################################
   2641.6 |
   2651.5 |
   2661.4 |
   2671.4 |
   2681.3 |
   2691.2 |
   2701.1 |########################################
   2711.0 |########################################
   2720.9 |
   2730.8 |
   2740.8 |
   2750.7 |########################################
   2760.6 |
   2770.5 |
   2780.4 |
   2790.3 |########################################
   2800.3 |
   2810.2 |
   2820.1 |
  (0 below, 1 above range)

abi_cross_cold_scatter_warm_scalar (n=6, range 2181465.4-2257676.2 ns)
  2181465.4 |########################################
  2185275.9 |
  2189086.5 |
  2192897.0 |
  2196707.6 |
  2200518.1 |
  2204328.7 |
  2208139.2 |########################################
  2211949.7 |########################################
  2215760.3 |
  2219570.8 |########################################
  2223381.4 |
  2227191.9 |
  2231002.5 |
  2234813.0 |
  2238623.5 |########################################
  2242434.1 |
  2246244.6 |
  2250055.2 |
  2253865.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_scatter_cold_null**: bridge=4674.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_cold_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_null**: bridge=4391.8% of algo (FFI overhead may distort results)
- **abi_cross_cold_scatter_warm_scalar**: bridge=304.2% of algo (FFI overhead may distort results)

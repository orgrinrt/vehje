# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 50939% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (4.19 us) leads abi_lifecycle_scatter_held_handle (2.14 ms) by 50939%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 696.8x slower than the field

abi_lifecycle_scatter_fresh_per_batch (2.92 ms) is 696.8x the fastest (4.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_scatter_fresh_per_batch shows alternating (throttle bounce) (autocorr -0.53)

abi_lifecycle_scatter_fresh_per_batch's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (50939% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 50939% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 696.8x the fastest

Fastest abi_lifecycle_scatter_null_entry (4.19 us) to slowest abi_lifecycle_scatter_fresh_per_batch (2.92 ms): 696.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 4189.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 696.76x (fastest 4189.6 ns, slowest 2919148.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2924894ns | 2921806ns | 2915508ns | 2919839ns | 2937170ns | +29.49% |
| abi_lifecycle_scatter_fresh_per_column | 2166484ns | 2157235ns | 2150842ns | 2156250ns | 2189657ns | -4.09% |
| abi_lifecycle_scatter_held_handle | 2258822ns | 2140794ns | 2136412ns | 2139563ns | 2498916ns | base |
| abi_lifecycle_scatter_null_entry | 6496ns | 6564ns | 6142ns | 6456ns | 6732ns | -99.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2922229ns | 2912965ns | 2934315ns | +29.52% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2163758ns | 2147973ns | 2186782ns | -4.09% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2256144ns | 2133929ns | 2495879ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 4138ns | 3948ns | 4257ns | -99.82% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 41230.7 | 3018394.4 | 2922229.1 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 59048.8 | 2254384.7 | 2163758.0 | 0 |
| abi_lifecycle_scatter_held_handle | 42758.0 | 2177275.8 | 2256144.3 | n/a |
| abi_lifecycle_scatter_null_entry | 27228.2 | 4270.5 | 4137.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.2% |
| abi_lifecycle_scatter_null_entry | 0.001 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2924894ns | 2924894ns | +29.49% |
| abi_lifecycle_scatter_fresh_per_column | 2166484ns | 2166484ns | -4.09% |
| abi_lifecycle_scatter_held_handle | 2258822ns | 2258822ns | base |
| abi_lifecycle_scatter_null_entry | 6496ns | 6496ns | -99.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2138348ns | base | --- | [2134206, 2495879] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2919148ns | +781254.5ns (+36.5%) | [+426039, +790960]ns | [2913224, 2934315] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2154674ns | no significant difference | [-333054, +41682]ns | [2149818, 2186782] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_scatter_null_entry | 4190ns | -2134254.4ns (-99.8%) | [-2491790, -2129975]ns | [3966, 4257] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2838201ns | +2.7% | -23.9% | -99.9% |
| 2 | 2153557ns | +36.4% | +2.9% | -99.8% |
| 3 | 2139919ns | +36.7% | +0.6% | -99.8% |
| 4 | 2136778ns | +36.3% | +0.7% | -99.8% |
| 5 | 2134483ns | +37.4% | +0.6% | -99.8% |
| 6 | 2133929ns | +36.5% | +1.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | -0.532 | HIGH- (thermal bounce) |
| abi_lifecycle_scatter_fresh_per_column | -0.109 | ok |
| abi_lifecycle_scatter_held_handle | -0.011 | ok |
| abi_lifecycle_scatter_null_entry | 0.234 | moderate+ |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 8904937.8ns | 2922229.1ns | 304.7% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6641930.6ns | 2163758.0ns | 307.0% | HIGH |
| abi_lifecycle_scatter_held_handle | 6528907.0ns | 2256144.3ns | 289.4% | HIGH |
| abi_lifecycle_scatter_null_entry | 122084.1ns | 4137.8ns | 2950.5% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2912965.0-2934315.0 ns)
  2912965.0 |########################################
  2914032.5 |####################
  2915100.0 |
  2916167.5 |
  2917235.0 |
  2918302.5 |
  2919370.0 |
  2920437.5 |
  2921505.0 |
  2922572.5 |
  2923640.0 |####################
  2924707.5 |
  2925775.0 |
  2926842.5 |
  2927910.0 |
  2928977.5 |
  2930045.0 |
  2931112.5 |####################
  2932180.0 |
  2933247.5 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2147972.9-2186782.2 ns)
  2147972.9 |########################################
  2149913.4 |########################################
  2151853.8 |########################################
  2153794.3 |
  2155734.8 |########################################
  2157675.2 |########################################
  2159615.7 |
  2161556.2 |
  2163496.6 |
  2165437.1 |
  2167377.6 |
  2169318.0 |
  2171258.5 |
  2173199.0 |
  2175139.4 |
  2177079.9 |
  2179020.4 |
  2180960.8 |
  2182901.3 |
  2184841.8 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2133929.2-2495878.8 ns)
  2133929.2 |########################################
  2152026.7 |##########
  2170124.2 |
  2188221.6 |
  2206319.1 |
  2224416.6 |
  2242514.1 |
  2260611.5 |
  2278709.0 |
  2296806.5 |
  2314904.0 |
  2333001.5 |
  2351098.9 |
  2369196.4 |
  2387293.9 |
  2405391.4 |
  2423488.8 |
  2441586.3 |
  2459683.8 |
  2477781.3 |
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 3947.5-4257.3 ns)
   3947.5 |####################
   3963.0 |
   3978.5 |####################
   3994.0 |
   4009.5 |
   4024.9 |
   4040.4 |
   4055.9 |
   4071.4 |
   4086.9 |
   4102.4 |
   4117.9 |
   4133.4 |
   4148.9 |
   4164.4 |
   4179.9 |########################################
   4195.3 |
   4210.8 |
   4226.3 |####################
   4241.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=2912.1% of algo (FFI overhead may distort results)

# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 90302% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (2.35 us) leads abi_lifecycle_scatter_held_handle (2.13 ms) by 90302%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.12 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 948.2x slower than the field

abi_lifecycle_scatter_fresh_per_batch (2.23 ms) is 948.2x the fastest (2.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_scatter_null_entry shows alternating (throttle bounce) (autocorr -0.50)

abi_lifecycle_scatter_null_entry's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (90302% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 90302% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 948.2x the fastest

Fastest abi_lifecycle_scatter_null_entry (2.35 us) to slowest abi_lifecycle_scatter_fresh_per_batch (2.23 ms): 948.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 2351.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 948.20x (fastest 2351.4 ns, slowest 2229636.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2232137ns | 2232242ns | 2222600ns | 2230909ns | 2238747ns | +4.92% |
| abi_lifecycle_scatter_fresh_per_column | 2145626ns | 2146146ns | 2142040ns | 2145164ns | 2148112ns | +0.86% |
| abi_lifecycle_scatter_held_handle | 2127428ns | 2128370ns | 2117445ns | 2126842ns | 2133299ns | base |
| abi_lifecycle_scatter_null_entry | 4636ns | 4677ns | 4392ns | 4607ns | 4802ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2229498ns | 2220051ns | 2236020ns | +4.92% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2143018ns | 2139460ns | 2145449ns | +0.85% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2124862ns | 2114890ns | 2130770ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 2330ns | 2213ns | 2407ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 40417.0 | 2229453.4 | 2229497.6 | n/a |
| abi_lifecycle_scatter_fresh_per_column | 37989.0 | 2142073.6 | 2143017.9 | 0 |
| abi_lifecycle_scatter_held_handle | 39573.7 | 2126520.2 | 2124861.7 | n/a |
| abi_lifecycle_scatter_null_entry | 26801.1 | 2437.8 | 2329.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.1% |
| abi_lifecycle_scatter_null_entry | 0.014 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2232137ns | 2232137ns | +4.92% |
| abi_lifecycle_scatter_fresh_per_column | 2145626ns | 2145626ns | +0.86% |
| abi_lifecycle_scatter_held_handle | 2127428ns | 2127428ns | base |
| abi_lifecycle_scatter_null_entry | 4636ns | 4636ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2125754ns | base | --- | [2118061, 2130770] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2229637ns | +100202.3ns (+4.7%) | [+98285, +115421]ns | [2222835, 2236020] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2143491ns | +17736.7ns (+0.8%) | [+11293, +25438]ns | [2140114, 2145449] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_null_entry | 2351ns | -2123372.0ns (-99.9%) | [-2128458, -2115765]ns | [2231, 2407] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2114890ns | +5.9% | +1.2% | -99.9% |
| 2 | 2126268ns | +4.7% | +0.9% | -99.9% |
| 3 | 2121232ns | +4.7% | +1.2% | -99.9% |
| 4 | 2130978ns | +4.7% | +0.6% | -99.9% |
| 5 | 2130561ns | +4.6% | +0.4% | -99.9% |
| 6 | 2125241ns | +5.0% | +0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | -0.146 | ok |
| abi_lifecycle_scatter_fresh_per_column | 0.070 | ok |
| abi_lifecycle_scatter_held_handle | -0.023 | ok |
| abi_lifecycle_scatter_null_entry | -0.505 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 6731680.7ns | 2229497.6ns | 301.9% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6467188.5ns | 2143017.9ns | 301.8% | HIGH |
| abi_lifecycle_scatter_held_handle | 6421882.6ns | 2124861.7ns | 302.2% | HIGH |
| abi_lifecycle_scatter_null_entry | 115948.2ns | 2329.9ns | 4976.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2220050.8-2236020.4 ns)
  2220050.8 |########################################
  2220849.3 |
  2221647.8 |
  2222446.2 |
  2223244.7 |
  2224043.2 |
  2224841.7 |########################################
  2225640.2 |
  2226438.6 |
  2227237.1 |
  2228035.6 |########################################
  2228834.1 |
  2229632.6 |
  2230431.0 |########################################
  2231229.5 |
  2232028.0 |########################################
  2232826.5 |
  2233625.0 |
  2234423.4 |
  2235221.9 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2139460.4-2145449.1 ns)
  2139460.4 |########################################
  2139759.8 |
  2140059.3 |
  2140358.7 |
  2140658.1 |########################################
  2140957.6 |
  2141257.0 |
  2141556.5 |
  2141855.9 |
  2142155.3 |
  2142454.8 |########################################
  2142754.2 |
  2143053.6 |
  2143353.1 |
  2143652.5 |
  2143952.0 |
  2144251.4 |########################################
  2144550.8 |########################################
  2144850.3 |
  2145149.7 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2114890.4-2130769.5 ns)
  2114890.4 |########################################
  2115684.4 |
  2116478.3 |
  2117272.3 |
  2118066.2 |
  2118860.2 |
  2119654.1 |
  2120448.1 |########################################
  2121242.1 |
  2122036.0 |
  2122830.0 |
  2123623.9 |
  2124417.9 |
  2125211.8 |########################################
  2126005.8 |########################################
  2126799.8 |
  2127593.7 |
  2128387.7 |
  2129181.6 |
  2129975.6 |########################################
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 2212.9-2406.9 ns)
   2212.9 |########################################
   2222.6 |
   2232.3 |
   2242.0 |########################################
   2251.7 |
   2261.4 |
   2271.1 |
   2280.8 |
   2290.5 |
   2300.2 |
   2309.9 |
   2319.6 |
   2329.3 |
   2339.0 |########################################
   2348.7 |
   2358.4 |########################################
   2368.1 |
   2377.8 |
   2387.5 |
   2397.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=4933.2% of algo (FFI overhead may distort results)

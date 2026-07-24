# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 43353% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (4.96 us) leads abi_lifecycle_real_held_handle (2.16 ms) by 43353%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.15 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 1059.4x slower than the field

abi_lifecycle_real_fresh_per_batch (5.26 ms) is 1059.4x the fastest (4.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (43353% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 43353% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1059.4x the fastest

Fastest abi_lifecycle_real_null_entry (4.96 us) to slowest abi_lifecycle_real_fresh_per_batch (5.26 ms): 1059.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 4961.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1059.40x (fastest 4961.2 ns, slowest 5255944.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 5270220ns | 5258727ns | 5248135ns | 5257676ns | 5300079ns | +139.97% |
| abi_lifecycle_real_fresh_per_column | 2184640ns | 2177199ns | 2169933ns | 2176603ns | 2204049ns | -0.52% |
| abi_lifecycle_real_held_handle | 2196165ns | 2158347ns | 2148912ns | 2155377ns | 2280975ns | base |
| abi_lifecycle_real_null_entry | 7314ns | 7251ns | 7199ns | 7241ns | 7481ns | -99.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 5267522ns | 5245469ns | 5297492ns | +140.15% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2182057ns | 2167385ns | 2201478ns | -0.52% | 0.000 |
| abi_lifecycle_real_held_handle | 2193444ns | 2146326ns | 2277949ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 5007ns | 4928ns | 5128ns | -99.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 47485.6 | 5271629.0 | 5267522.0 | n/a |
| abi_lifecycle_real_fresh_per_column | 40927.1 | 2175829.9 | 2182057.0 | n/a |
| abi_lifecycle_real_held_handle | 44741.2 | 2197971.9 | 2193443.6 | n/a |
| abi_lifecycle_real_null_entry | 27452.1 | 5098.5 | 5007.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_real_held_handle | 0.000 | 0.2% |
| abi_lifecycle_real_null_entry | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 5270220ns | 5270220ns | +139.97% |
| abi_lifecycle_real_fresh_per_column | 2184640ns | 2184640ns | -0.52% |
| abi_lifecycle_real_held_handle | 2196165ns | 2196165ns | base |
| abi_lifecycle_real_null_entry | 7314ns | 7314ns | -99.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2155820ns | base | --- | [2146562, 2277949] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 5255945ns | +3102361.7ns (+143.9%) | [+2973206, +3146667]ns | [5249130, 5297492] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2174592ns | no significant difference | [-78415, +28033]ns | [2170101, 2201478] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_real_null_entry | 4961ns | -2150784.8ns (-99.8%) | [-2272993, -2141531]ns | [4933, 5128] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2155323ns | +146.1% | +0.9% | -99.8% |
| 2 | 2156316ns | +143.7% | +0.5% | -99.8% |
| 3 | 2394482ns | +119.5% | -7.0% | -99.8% |
| 4 | 2161417ns | +142.7% | +0.6% | -99.8% |
| 5 | 2146326ns | +146.5% | +1.2% | -99.8% |
| 6 | 2146798ns | +144.7% | +1.4% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.338 | moderate- |
| abi_lifecycle_real_fresh_per_column | -0.322 | moderate- |
| abi_lifecycle_real_held_handle | -0.180 | ok |
| abi_lifecycle_real_null_entry | -0.285 | moderate- |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 15859862.4ns | 5267522.0ns | 301.1% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6604451.8ns | 2182057.0ns | 302.7% | HIGH |
| abi_lifecycle_real_held_handle | 6617477.6ns | 2193443.6ns | 301.7% | HIGH |
| abi_lifecycle_real_null_entry | 124941.3ns | 5007.3ns | 2495.2% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 5245468.8-5297491.7 ns)
  5245468.8 |########################################
  5248069.9 |
  5250671.1 |########################################
  5253272.2 |########################################
  5255873.4 |########################################
  5258474.5 |
  5261075.7 |
  5263676.8 |
  5266277.9 |
  5268879.1 |
  5271480.2 |
  5274081.4 |
  5276682.5 |
  5279283.7 |
  5281884.8 |
  5284485.9 |
  5287087.1 |
  5289688.2 |########################################
  5292289.4 |
  5294890.5 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2167384.6-2201478.2 ns)
  2167384.6 |####################
  2169089.3 |
  2170794.0 |
  2172498.6 |########################################
  2174203.3 |####################
  2175908.0 |####################
  2177612.7 |
  2179317.3 |
  2181022.0 |
  2182726.7 |
  2184431.4 |
  2186136.1 |
  2187840.7 |
  2189545.4 |
  2191250.1 |
  2192954.8 |
  2194659.4 |
  2196364.1 |
  2198068.8 |
  2199773.5 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2146326.2-2277949.2 ns)
  2146326.2 |########################################
  2152907.4 |########################################
  2159488.5 |####################
  2166069.7 |
  2172650.8 |
  2179232.0 |
  2185813.1 |
  2192394.2 |
  2198975.4 |
  2205556.6 |
  2212137.7 |
  2218718.9 |
  2225300.0 |
  2231881.2 |
  2238462.3 |
  2245043.5 |
  2251624.6 |
  2258205.8 |
  2264786.9 |
  2271368.1 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 4928.3-5127.5 ns)
   4928.3 |########################################
   4938.3 |########################################
   4948.2 |########################################
   4958.2 |
   4968.1 |########################################
   4978.1 |
   4988.1 |
   4998.0 |
   5008.0 |
   5017.9 |
   5027.9 |
   5037.9 |
   5047.8 |
   5057.8 |
   5067.7 |
   5077.7 |
   5087.7 |
   5097.6 |
   5107.6 |
   5117.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=2515.6% of algo (FFI overhead may distort results)

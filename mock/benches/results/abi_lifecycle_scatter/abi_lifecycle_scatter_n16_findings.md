# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 83862% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (2.54 us) leads abi_lifecycle_scatter_held_handle (2.13 ms) by 83862%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 914.3x slower than the field

abi_lifecycle_scatter_fresh_per_batch (2.32 ms) is 914.3x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_scatter_held_handle shows alternating (throttle bounce) (autocorr -0.58)

abi_lifecycle_scatter_held_handle's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (83862% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 83862% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 914.3x the fastest

Fastest abi_lifecycle_scatter_null_entry (2.54 us) to slowest abi_lifecycle_scatter_fresh_per_batch (2.32 ms): 914.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 2541.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 914.26x (fastest 2541.8 ns, slowest 2323924.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2326948ns | 2326387ns | 2322578ns | 2325582ns | 2331181ns | +8.75% |
| abi_lifecycle_scatter_fresh_per_column | 2150570ns | 2150310ns | 2144313ns | 2148981ns | 2156081ns | +0.51% |
| abi_lifecycle_scatter_held_handle | 2139723ns | 2136813ns | 2123735ns | 2136007ns | 2153289ns | base |
| abi_lifecycle_scatter_null_entry | 4873ns | 4817ns | 4716ns | 4800ns | 5061ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2324432ns | 2320109ns | 2328577ns | +8.77% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2147987ns | 2141655ns | 2153396ns | +0.51% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2137079ns | 2121067ns | 2150547ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 2555ns | 2501ns | 2620ns | -99.88% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 37385.7 | 2323993.6 | 2324431.5 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 38223.9 | 2145396.6 | 2147987.2 | 0 |
| abi_lifecycle_scatter_held_handle | 40602.0 | 2142917.8 | 2137079.3 | n/a |
| abi_lifecycle_scatter_null_entry | 26455.0 | 2631.8 | 2555.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.1% |
| abi_lifecycle_scatter_null_entry | 0.006 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2326948ns | 2326948ns | +8.75% |
| abi_lifecycle_scatter_fresh_per_column | 2150570ns | 2150570ns | +0.51% |
| abi_lifecycle_scatter_held_handle | 2139723ns | 2139723ns | base |
| abi_lifecycle_scatter_null_entry | 4873ns | 4873ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2134184ns | base | --- | [2126508, 2150547] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2323924ns | +190053.6ns (+8.9%) | [+175379, +196624]ns | [2320794, 2328577] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2147800ns | no significant difference | [-3249, +25120]ns | [2142765, 2153396] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_scatter_null_entry | 2542ns | -2131654.1ns (-99.9%) | [-2147985, -2123932]ns | [2504, 2620] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2132391ns | +8.8% | +0.7% | -99.9% |
| 2 | 2131948ns | +9.0% | +0.5% | -99.9% |
| 3 | 2142731ns | +8.4% | +0.4% | -99.9% |
| 4 | 2121067ns | +9.4% | +1.7% | -99.9% |
| 5 | 2158362ns | +7.9% | -0.7% | -99.9% |
| 6 | 2135976ns | +9.0% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.099 | ok |
| abi_lifecycle_scatter_fresh_per_column | -0.201 | moderate- |
| abi_lifecycle_scatter_held_handle | -0.581 | HIGH- (thermal bounce) |
| abi_lifecycle_scatter_null_entry | 0.454 | moderate+ |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 7012427.9ns | 2324431.5ns | 301.7% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6478488.6ns | 2147987.2ns | 301.6% | HIGH |
| abi_lifecycle_scatter_held_handle | 6466268.2ns | 2137079.3ns | 302.6% | HIGH |
| abi_lifecycle_scatter_null_entry | 116758.5ns | 2555.3ns | 4569.3% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2320109.2-2328576.6 ns)
  2320109.2 |########################################
  2320532.6 |
  2320955.9 |
  2321379.3 |########################################
  2321802.7 |
  2322226.1 |
  2322649.4 |########################################
  2323072.8 |
  2323496.2 |
  2323919.6 |
  2324342.9 |
  2324766.3 |########################################
  2325189.7 |
  2325613.0 |
  2326036.4 |
  2326459.8 |
  2326883.2 |
  2327306.5 |
  2327729.9 |
  2328153.3 |########################################
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2141655.0-2153396.5 ns)
  2141655.0 |####################
  2142242.1 |
  2142829.1 |
  2143416.2 |####################
  2144003.3 |
  2144590.4 |
  2145177.4 |
  2145764.5 |
  2146351.6 |
  2146938.7 |
  2147525.7 |########################################
  2148112.8 |
  2148699.9 |
  2149286.9 |
  2149874.0 |
  2150461.1 |####################
  2151048.2 |
  2151635.2 |
  2152222.3 |
  2152809.4 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2121067.1-2150546.7 ns)
  2121067.1 |####################
  2122541.1 |
  2124015.1 |
  2125489.0 |
  2126963.0 |
  2128437.0 |
  2129911.0 |
  2131384.9 |########################################
  2132858.9 |
  2134332.9 |
  2135806.9 |####################
  2137280.9 |
  2138754.8 |
  2140228.8 |
  2141702.8 |####################
  2143176.8 |
  2144650.7 |
  2146124.7 |
  2147598.7 |
  2149072.7 |
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 2501.2-2620.0 ns)
   2501.2 |########################################
   2507.1 |
   2513.1 |
   2519.0 |
   2525.0 |####################
   2530.9 |
   2536.8 |
   2542.8 |
   2548.7 |
   2554.7 |####################
   2560.6 |
   2566.5 |
   2572.5 |
   2578.4 |
   2584.4 |
   2590.3 |
   2596.2 |
   2602.2 |
   2608.1 |
   2614.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=4589.9% of algo (FFI overhead may distort results)

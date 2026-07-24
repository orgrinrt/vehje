# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 84302% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (2.53 us) leads abi_lifecycle_scatter_held_handle (2.13 ms) by 84302%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 863.2x slower than the field

abi_lifecycle_scatter_fresh_per_batch (2.18 ms) is 863.2x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (84302% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 84302% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 863.2x the fastest

Fastest abi_lifecycle_scatter_null_entry (2.53 us) to slowest abi_lifecycle_scatter_fresh_per_batch (2.18 ms): 863.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 2526.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 863.19x (fastest 2526.4 ns, slowest 2180797.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2185649ns | 2183329ns | 2177367ns | 2182429ns | 2194619ns | +2.47% |
| abi_lifecycle_scatter_fresh_per_column | 2151179ns | 2149126ns | 2139184ns | 2147409ns | 2162832ns | +0.86% |
| abi_lifecycle_scatter_held_handle | 2132879ns | 2134983ns | 2122329ns | 2133434ns | 2137322ns | base |
| abi_lifecycle_scatter_null_entry | 4867ns | 4809ns | 4605ns | 4800ns | 5099ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2183065ns | 2174606ns | 2191985ns | +2.48% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2148472ns | 2136505ns | 2159925ns | +0.86% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2130238ns | 2119682ns | 2134666ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 2533ns | 2411ns | 2627ns | -99.88% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 38620.0 | 2182930.6 | 2183065.4 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 41605.0 | 2147182.1 | 2148471.9 | n/a |
| abi_lifecycle_scatter_held_handle | 40017.1 | 2128647.8 | 2130238.1 | n/a |
| abi_lifecycle_scatter_null_entry | 26524.2 | 2773.4 | 2533.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.1% |
| abi_lifecycle_scatter_null_entry | 0.025 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2185649ns | 2185649ns | +2.47% |
| abi_lifecycle_scatter_fresh_per_column | 2151179ns | 2151179ns | +0.86% |
| abi_lifecycle_scatter_held_handle | 2132879ns | 2132879ns | base |
| abi_lifecycle_scatter_null_entry | 4867ns | 4867ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2132367ns | base | --- | [2123682, 2134666] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2180798ns | +49022.0ns (+2.3%) | [+42935, +66525]ns | [2176413, 2191985] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2146507ns | +14055.8ns (+0.7%) | [+8395, +32251]ns | [2138983, 2159925] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_null_entry | 2526ns | -2129784.0ns (-99.9%) | [-2132175, -2121155]ns | [2447, 2627] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2135667ns | +2.2% | +1.3% | -99.9% |
| 2 | 2131238ns | +3.2% | +0.7% | -99.9% |
| 3 | 2127681ns | +2.4% | +0.4% | -99.9% |
| 4 | 2133665ns | +2.1% | +0.6% | -99.9% |
| 5 | 2119682ns | +3.0% | +1.7% | -99.9% |
| 6 | 2133496ns | +1.9% | +0.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | -0.184 | ok |
| abi_lifecycle_scatter_fresh_per_column | -0.098 | ok |
| abi_lifecycle_scatter_held_handle | -0.448 | moderate- |
| abi_lifecycle_scatter_null_entry | -0.024 | ok |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 6586756.6ns | 2183065.4ns | 301.7% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6487540.9ns | 2148471.9ns | 302.0% | HIGH |
| abi_lifecycle_scatter_held_handle | 6430136.0ns | 2130238.1ns | 301.9% | HIGH |
| abi_lifecycle_scatter_null_entry | 112188.3ns | 2533.3ns | 4428.5% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2174605.8-2191985.2 ns)
  2174605.8 |####################
  2175474.8 |
  2176343.7 |
  2177212.7 |
  2178081.7 |########################################
  2178950.6 |
  2179819.6 |
  2180688.6 |
  2181557.6 |
  2182426.5 |####################
  2183295.5 |
  2184164.5 |####################
  2185033.4 |
  2185902.4 |
  2186771.4 |
  2187640.4 |
  2188509.3 |
  2189378.3 |
  2190247.3 |
  2191116.2 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2136505.0-2159925.4 ns)
  2136505.0 |####################
  2137676.0 |
  2138847.0 |
  2140018.1 |
  2141189.1 |####################
  2142360.1 |
  2143531.1 |
  2144702.1 |
  2145873.2 |########################################
  2147044.2 |
  2148215.2 |
  2149386.2 |
  2150557.2 |
  2151728.3 |
  2152899.3 |
  2154070.3 |
  2155241.3 |####################
  2156412.3 |
  2157583.4 |
  2158754.4 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2119682.5-2134665.9 ns)
  2119682.5 |####################
  2120431.7 |
  2121180.8 |
  2121930.0 |
  2122679.2 |
  2123428.3 |
  2124177.5 |
  2124926.7 |
  2125675.8 |
  2126425.0 |
  2127174.2 |####################
  2127923.3 |
  2128672.5 |
  2129421.7 |
  2130170.8 |
  2130920.0 |####################
  2131669.2 |
  2132418.3 |
  2133167.5 |########################################
  2133916.7 |
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 2411.2-2626.7 ns)
   2411.2 |####################
   2422.0 |
   2432.7 |
   2443.5 |
   2454.3 |
   2465.1 |
   2475.8 |####################
   2486.6 |
   2497.4 |
   2508.2 |
   2518.9 |########################################
   2529.7 |
   2540.5 |
   2551.2 |
   2562.0 |####################
   2572.8 |
   2583.6 |
   2594.3 |
   2605.1 |
   2615.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=4458.3% of algo (FFI overhead may distort results)

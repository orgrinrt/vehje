# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 67012% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (3.17 us) leads abi_lifecycle_scatter_held_handle (2.13 ms) by 67012%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_column is an outlier: 676.6x slower than the field

abi_lifecycle_scatter_fresh_per_column (2.15 ms) is 676.6x the fastest (3.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_batch, abi_lifecycle_scatter_fresh_per_column} (67012% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_batch, abi_lifecycle_scatter_fresh_per_column} with a 67012% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 676.6x the fastest

Fastest abi_lifecycle_scatter_null_entry (3.17 us) to slowest abi_lifecycle_scatter_fresh_per_column (2.15 ms): 676.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 3172.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 676.56x (fastest 3172.7 ns, slowest 2146535.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2146104ns | 2148417ns | 2125609ns | 2148294ns | 2153067ns | +0.77% |
| abi_lifecycle_scatter_fresh_per_column | 2148314ns | 2149249ns | 2138814ns | 2148734ns | 2152434ns | +0.87% |
| abi_lifecycle_scatter_held_handle | 2129802ns | 2131739ns | 2115207ns | 2128712ns | 2138735ns | base |
| abi_lifecycle_scatter_null_entry | 5438ns | 5443ns | 5143ns | 5383ns | 5667ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2143509ns | 2123179ns | 2150305ns | +0.77% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2145696ns | 2136110ns | 2149887ns | +0.87% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2127131ns | 2112406ns | 2135895ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 3177ns | 3008ns | 3314ns | -99.85% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 38501.3 | 2142450.5 | 2143509.2 | n/a |
| abi_lifecycle_scatter_fresh_per_column | 39593.4 | 2143549.7 | 2145695.5 | n/a |
| abi_lifecycle_scatter_held_handle | 37832.4 | 2127696.4 | 2127131.3 | n/a |
| abi_lifecycle_scatter_null_entry | 26342.8 | 3187.7 | 3177.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.085 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.1% |
| abi_lifecycle_scatter_null_entry | 0.081 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2146104ns | 2146104ns | +0.77% |
| abi_lifecycle_scatter_fresh_per_column | 2148314ns | 2148314ns | +0.87% |
| abi_lifecycle_scatter_held_handle | 2129802ns | 2129802ns | base |
| abi_lifecycle_scatter_null_entry | 5438ns | 5438ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2129252ns | base | --- | [2116248, 2135895] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2145890ns | +16977.1ns (+0.8%) | [+9783, +22373]ns | [2134333, 2150305] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2146535ns | +16325.9ns (+0.8%) | [+12974, +26393]ns | [2140664, 2149887] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_scatter_null_entry | 3173ns | -2126020.9ns (-99.8%) | [-2132639, -2113202]ns | [3045, 3314] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2135095ns | +0.9% | +0.8% | -99.9% |
| 2 | 2120089ns | +1.2% | +0.8% | -99.9% |
| 3 | 2130429ns | +0.7% | +0.7% | -99.8% |
| 4 | 2136694ns | +0.4% | +0.5% | -99.8% |
| 5 | 2112406ns | +0.5% | +1.6% | -99.9% |
| 6 | 2128075ns | +0.9% | +0.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | -0.128 | ok |
| abi_lifecycle_scatter_fresh_per_column | -0.440 | moderate- |
| abi_lifecycle_scatter_held_handle | -0.468 | moderate- |
| abi_lifecycle_scatter_null_entry | -0.224 | moderate- |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 6469617.9ns | 2143509.2ns | 301.8% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6473954.4ns | 2145695.5ns | 301.7% | HIGH |
| abi_lifecycle_scatter_held_handle | 6420916.2ns | 2127131.3ns | 301.9% | HIGH |
| abi_lifecycle_scatter_null_entry | 119795.5ns | 3177.4ns | 3770.3% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2123179.2-2150304.8 ns)
  2123179.2 |#############
  2124535.5 |
  2125891.8 |
  2127248.0 |
  2128604.3 |
  2129960.6 |
  2131316.9 |
  2132673.1 |
  2134029.4 |
  2135385.7 |
  2136742.0 |
  2138098.3 |
  2139454.5 |
  2140810.8 |
  2142167.1 |
  2143523.4 |
  2144879.6 |########################################
  2146235.9 |#############
  2147592.2 |
  2148948.5 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2136109.6-2149887.3 ns)
  2136109.6 |####################
  2136798.5 |
  2137487.4 |
  2138176.3 |
  2138865.1 |
  2139554.0 |
  2140242.9 |
  2140931.8 |
  2141620.7 |
  2142309.6 |
  2142998.5 |
  2143687.3 |
  2144376.2 |
  2145065.1 |########################################
  2145754.0 |
  2146442.9 |
  2147131.8 |####################
  2147820.6 |####################
  2148509.5 |
  2149198.4 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2112405.8-2135894.6 ns)
  2112405.8 |########################################
  2113580.2 |
  2114754.7 |
  2115929.1 |
  2117103.6 |
  2118278.0 |
  2119452.4 |########################################
  2120626.9 |
  2121801.3 |
  2122975.8 |
  2124150.2 |
  2125324.6 |
  2126499.1 |
  2127673.5 |########################################
  2128848.0 |
  2130022.4 |########################################
  2131196.8 |
  2132371.3 |
  2133545.7 |
  2134720.2 |########################################
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 3007.9-3314.4 ns)
   3007.9 |########################################
   3023.2 |
   3038.5 |
   3053.9 |
   3069.2 |########################################
   3084.5 |
   3099.8 |
   3115.2 |
   3130.5 |########################################
   3145.8 |
   3161.1 |
   3176.4 |
   3191.8 |########################################
   3207.1 |
   3222.4 |
   3237.7 |
   3253.1 |########################################
   3268.4 |
   3283.7 |
   3299.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=3756.6% of algo (FFI overhead may distort results)

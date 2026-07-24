# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 79034% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (2.70 us) leads abi_lifecycle_real_held_handle (2.14 ms) by 79034%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 801.4x slower than the field

abi_lifecycle_real_fresh_per_batch (2.17 ms) is 801.4x the fastest (2.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (79034% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 79034% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 801.4x the fastest

Fastest abi_lifecycle_real_null_entry (2.70 us) to slowest abi_lifecycle_real_fresh_per_batch (2.17 ms): 801.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 2704.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 801.41x (fastest 2704.6 ns, slowest 2167487.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2216689ns | 2170053ns | 2161294ns | 2169227ns | 2315579ns | +2.64% |
| abi_lifecycle_real_fresh_per_column | 2177103ns | 2159863ns | 2151367ns | 2158828ns | 2217385ns | +0.80% |
| abi_lifecycle_real_held_handle | 2159730ns | 2142858ns | 2127030ns | 2142498ns | 2201928ns | base |
| abi_lifecycle_real_null_entry | 5129ns | 4990ns | 4855ns | 4964ns | 5514ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2213436ns | 2158773ns | 2310913ns | +2.62% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2174357ns | 2148730ns | 2214378ns | +0.80% | 0.000 |
| abi_lifecycle_real_held_handle | 2157020ns | 2124471ns | 2199000ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 2760ns | 2633ns | 2940ns | -99.87% | 0.046 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 42370.7 | 2173701.8 | 2213435.7 | 0 |
| abi_lifecycle_real_fresh_per_column | 44741.7 | 2169292.7 | 2174356.5 | n/a |
| abi_lifecycle_real_held_handle | 44028.0 | 2150154.9 | 2157020.4 | n/a |
| abi_lifecycle_real_null_entry | 29560.9 | 2806.5 | 2760.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_real_held_handle | 0.000 | 0.1% |
| abi_lifecycle_real_null_entry | 0.047 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2216689ns | 2216689ns | +2.64% |
| abi_lifecycle_real_fresh_per_column | 2177103ns | 2177103ns | +0.80% |
| abi_lifecycle_real_held_handle | 2159730ns | 2159730ns | base |
| abi_lifecycle_real_null_entry | 5129ns | 5129ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2140258ns | base | --- | [2131804, 2199000] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2167488ns | +28299.9ns (+1.3%) | [+21649, +119297]ns | [2161906, 2310913] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2157177ns | +14306.4ns (+0.7%) | [+11752, +25950]ns | [2151515, 2214378] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_null_entry | 2705ns | -2137498.5ns (-99.9%) | [-2196166, -2129115]ns | [2637, 2940] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2140156ns | +1.2% | +0.8% | -99.9% |
| 2 | 2141854ns | +1.3% | +0.7% | -99.9% |
| 3 | 2139137ns | +1.3% | +0.4% | -99.9% |
| 4 | 2140359ns | +0.9% | +0.7% | -99.9% |
| 5 | 2124471ns | +2.0% | +1.6% | -99.9% |
| 6 | 2256146ns | +8.7% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.027 | ok |
| abi_lifecycle_real_fresh_per_column | 0.007 | ok |
| abi_lifecycle_real_held_handle | -0.155 | ok |
| abi_lifecycle_real_null_entry | -0.172 | ok |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 6583005.8ns | 2213435.7ns | 297.4% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6588054.5ns | 2174356.5ns | 303.0% | HIGH |
| abi_lifecycle_real_held_handle | 6507064.9ns | 2157020.4ns | 301.7% | HIGH |
| abi_lifecycle_real_null_entry | 122138.2ns | 2760.4ns | 4424.6% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2158772.9-2310912.9 ns)
  2158772.9 |##########################
  2166379.9 |########################################
  2173986.9 |
  2181593.9 |
  2189200.9 |
  2196807.9 |
  2204414.9 |
  2212021.9 |
  2219628.9 |
  2227235.9 |
  2234842.9 |
  2242449.9 |
  2250056.9 |
  2257663.9 |
  2265270.9 |
  2272877.9 |
  2280484.9 |
  2288091.9 |
  2295698.9 |
  2303305.9 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2148729.6-2214377.7 ns)
  2148729.6 |####################
  2152012.0 |####################
  2155294.4 |########################################
  2158576.8 |####################
  2161859.2 |
  2165141.6 |
  2168424.0 |
  2171706.4 |
  2174988.8 |
  2178271.2 |
  2181553.7 |
  2184836.1 |
  2188118.5 |
  2191400.9 |
  2194683.3 |
  2197965.7 |
  2201248.1 |
  2204530.5 |
  2207812.9 |
  2211095.3 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2124471.2-2198999.8 ns)
  2124471.2 |#############
  2128197.6 |
  2131924.1 |
  2135650.5 |#############
  2139376.9 |########################################
  2143103.4 |
  2146829.8 |
  2150556.2 |
  2154282.6 |
  2158009.1 |
  2161735.5 |
  2165461.9 |
  2169188.4 |
  2172914.8 |
  2176641.2 |
  2180367.6 |
  2184094.1 |
  2187820.5 |
  2191546.9 |
  2195273.4 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 2633.3-2939.8 ns)
   2633.3 |########################################
   2648.6 |
   2664.0 |####################
   2679.3 |
   2694.6 |
   2709.9 |
   2725.2 |####################
   2740.6 |
   2755.9 |
   2771.2 |
   2786.6 |
   2801.9 |
   2817.2 |
   2832.5 |
   2847.9 |
   2863.2 |
   2878.5 |####################
   2893.8 |
   2909.2 |
   2924.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=4440.0% of algo (FFI overhead may distort results)

# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_scatter_null_entry dominates: 62849% faster than the next best (abi_entry_form_scatter_dispatch_table)

abi_entry_form_scatter_null_entry (3.45 us) leads abi_entry_form_scatter_dispatch_table (2.17 ms) by 62849%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.19 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_scalar_anchor is an outlier: 637.8x slower than the field

abi_entry_form_scatter_scalar_anchor (2.20 ms) is 637.8x the fastest (3.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_scalar_anchor} (62849% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_scalar_anchor} with a 62849% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 637.8x the fastest

Fastest abi_entry_form_scatter_null_entry (3.45 us) to slowest abi_entry_form_scatter_scalar_anchor (2.20 ms): 637.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_entry_form_scatter_scalar_anchor's edge over baseline is significant but tiny (5 ns, 0.00%)

abi_entry_form_scatter_scalar_anchor differs from baseline abi_entry_form_scatter_runtime_w by 5 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 3453.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 637.81x (fastest 3453.9 ns, slowest 2202980.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2181602ns | 2177579ns | 2169058ns | 2176385ns | 2195700ns | -1.54% |
| abi_entry_form_scatter_null_entry | 5836ns | 5808ns | 5622ns | 5764ns | 6050ns | -99.74% |
| abi_entry_form_scatter_per_w_set | 2185001ns | 2184292ns | 2168730ns | 2182201ns | 2197336ns | -1.39% |
| abi_entry_form_scatter_runtime_w | 2215701ns | 2195847ns | 2177729ns | 2192057ns | 2270154ns | base |
| abi_entry_form_scatter_scalar_anchor | 2204574ns | 2206782ns | 2169895ns | 2205303ns | 2220820ns | -0.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2178094ns | 2166167ns | 2191670ns | -1.53% | 0.000 |
| abi_entry_form_scatter_null_entry | 3488ns | 3379ns | 3628ns | -99.84% | 0.001 |
| abi_entry_form_scatter_per_w_set | 2181485ns | 2165506ns | 2193474ns | -1.37% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2211872ns | 2174540ns | 2265578ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2200840ns | 2166695ns | 2216976ns | -0.50% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 66393.0 | 2179812.4 | 2178094.0 | n/a |
| abi_entry_form_scatter_null_entry | 29217.5 | 3621.3 | 3488.0 | n/a |
| abi_entry_form_scatter_per_w_set | 75192.6 | 2178267.1 | 2181485.5 | 0 |
| abi_entry_form_scatter_runtime_w | 74022.6 | 2216073.2 | 2211871.9 | n/a |
| abi_entry_form_scatter_scalar_anchor | 76875.9 | 2210631.1 | 2200839.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_scatter_null_entry | 0.001 | 97.8% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.2% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.2% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2181602ns | 2181602ns | -1.54% |
| abi_entry_form_scatter_null_entry | 5836ns | 5836ns | -99.74% |
| abi_entry_form_scatter_per_w_set | 2185001ns | 2185001ns | -1.39% |
| abi_entry_form_scatter_runtime_w | 2215701ns | 2215701ns | base |
| abi_entry_form_scatter_scalar_anchor | 2204574ns | 2204574ns | -0.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2192226ns | base | --- | [2177812, 2265578] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2174230ns | no significant difference | [-94563, +2346]ns | [2168383, 2191670] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_scatter_null_entry | 3454ns | -2188619.1ns (-99.8%) | [-2262127, -2174406]ns | [3382, 3628] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2180911ns | no significant difference | [-81252, +2850]ns | [2170072, 2193474] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2202981ns | no significant difference | [-55414, +22313]ns | [2182563, 2216976] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2193331ns | +0.1% | -99.8% | +0.4% | +0.7% |
| 2 | 2191120ns | -0.2% | -99.8% | -0.8% | +0.5% |
| 3 | 2209148ns | -1.7% | -99.8% | -1.1% | -0.5% |
| 4 | 2181084ns | -0.7% | -99.8% | -0.1% | -0.7% |
| 5 | 2174540ns | +0.1% | -99.8% | -0.4% | +1.4% |
| 6 | 2322009ns | -6.5% | -99.8% | -6.0% | -4.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.320 | moderate+ |
| abi_entry_form_scatter_null_entry | -0.216 | moderate- |
| abi_entry_form_scatter_per_w_set | -0.210 | moderate- |
| abi_entry_form_scatter_runtime_w | -0.160 | ok |
| abi_entry_form_scatter_scalar_anchor | 0.031 | ok |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 5/6, lost 1/6
- **abi_entry_form_scatter_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6607565.1ns | 2178094.0ns | 303.4% | HIGH |
| abi_entry_form_scatter_null_entry | 122147.1ns | 3488.0ns | 3501.9% | HIGH |
| abi_entry_form_scatter_per_w_set | 6620377.0ns | 2181485.5ns | 303.5% | HIGH |
| abi_entry_form_scatter_runtime_w | 6714634.3ns | 2211871.9ns | 303.6% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6704876.1ns | 2200839.8ns | 304.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2166166.7-2191669.6 ns)
  2166166.7 |########################################
  2167441.8 |
  2168717.0 |
  2169992.1 |########################################
  2171267.3 |########################################
  2172542.4 |
  2173817.6 |
  2175092.7 |
  2176367.9 |########################################
  2177643.0 |
  2178918.2 |
  2180193.3 |
  2181468.4 |
  2182743.6 |
  2184018.7 |
  2185293.9 |
  2186569.0 |########################################
  2187844.2 |
  2189119.3 |
  2190394.5 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 3378.8-3627.9 ns)
   3378.8 |########################################
   3391.3 |
   3403.7 |
   3416.2 |####################
   3428.6 |
   3441.1 |
   3453.5 |
   3466.0 |
   3478.4 |####################
   3490.9 |
   3503.3 |
   3515.8 |####################
   3528.3 |
   3540.7 |
   3553.2 |
   3565.6 |
   3578.1 |
   3590.5 |
   3603.0 |
   3615.4 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2165506.2-2193473.5 ns)
  2165506.2 |####################
  2166904.6 |
  2168302.9 |
  2169701.3 |
  2171099.7 |
  2172498.0 |
  2173896.4 |####################
  2175294.8 |
  2176693.1 |
  2178091.5 |####################
  2179489.9 |
  2180888.2 |
  2182286.6 |
  2183684.9 |########################################
  2185083.3 |
  2186481.7 |
  2187880.0 |
  2189278.4 |
  2190676.8 |
  2192075.1 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2174539.6-2265578.3 ns)
  2174539.6 |########################################
  2179091.5 |########################################
  2183643.5 |
  2188195.4 |########################################
  2192747.4 |########################################
  2197299.3 |
  2201851.2 |
  2206403.2 |########################################
  2210955.1 |
  2215507.0 |
  2220059.0 |
  2224610.9 |
  2229162.8 |
  2233714.8 |
  2238266.7 |
  2242818.7 |
  2247370.6 |
  2251922.5 |
  2256474.5 |
  2261026.4 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2166694.6-2216975.9 ns)
  2166694.6 |########################################
  2169208.7 |
  2171722.7 |
  2174236.8 |
  2176750.9 |
  2179264.9 |
  2181779.0 |
  2184293.0 |
  2186807.1 |
  2189321.2 |
  2191835.2 |
  2194349.3 |
  2196863.4 |########################################
  2199377.4 |########################################
  2201891.5 |########################################
  2204405.5 |
  2206919.6 |########################################
  2209433.7 |
  2211947.7 |
  2214461.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=3515.3% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=305.0% of algo (FFI overhead may distort results)

# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 67084% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (3.19 us) leads abi_lifecycle_real_held_handle (2.14 ms) by 67084%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 676.2x slower than the field

abi_lifecycle_real_fresh_per_batch (2.16 ms) is 676.2x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_real_null_entry shows alternating (throttle bounce) (autocorr -0.56)

abi_lifecycle_real_null_entry's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (67084% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 67084% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 676.2x the fastest

Fastest abi_lifecycle_real_null_entry (3.19 us) to slowest abi_lifecycle_real_fresh_per_batch (2.16 ms): 676.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 3187.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 676.19x (fastest 3187.9 ns, slowest 2155613.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2157155ns | 2158102ns | 2147428ns | 2157644ns | 2161284ns | +0.61% |
| abi_lifecycle_real_fresh_per_column | 2157505ns | 2158145ns | 2147388ns | 2157052ns | 2163242ns | +0.62% |
| abi_lifecycle_real_held_handle | 2144140ns | 2144379ns | 2136511ns | 2143899ns | 2148316ns | base |
| abi_lifecycle_real_null_entry | 5451ns | 5496ns | 5162ns | 5432ns | 5625ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2154629ns | 2144830ns | 2158762ns | +0.61% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2154936ns | 2145081ns | 2160602ns | +0.63% | 0.000 |
| abi_lifecycle_real_held_handle | 2141472ns | 2133982ns | 2145568ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 3171ns | 3026ns | 3259ns | -99.85% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 38634.4 | 2154376.9 | 2154629.2 | 0 |
| abi_lifecycle_real_fresh_per_column | 39297.3 | 2155064.1 | 2154935.6 | 0 |
| abi_lifecycle_real_held_handle | 40518.3 | 2140586.2 | 2141471.6 | n/a |
| abi_lifecycle_real_null_entry | 27473.6 | 3197.7 | 3171.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.085 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_real_held_handle | 0.000 | 0.1% |
| abi_lifecycle_real_null_entry | 0.080 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2157155ns | 2157155ns | +0.61% |
| abi_lifecycle_real_fresh_per_column | 2157505ns | 2157505ns | +0.62% |
| abi_lifecycle_real_held_handle | 2144140ns | 2144140ns | base |
| abi_lifecycle_real_null_entry | 5451ns | 5451ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2141743ns | base | --- | [2137103, 2145568] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2155614ns | +13381.8ns (+0.6%) | [+5261, +20830]ns | [2149511, 2158762] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2155592ns | +15275.2ns (+0.7%) | [+5551, +19566]ns | [2148612, 2160602] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_null_entry | 3188ns | -2138552.5ns (-99.9%) | [-2142367, -2133981]ns | [3067, 3259] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2146897ns | +0.3% | +0.8% | -99.9% |
| 2 | 2141884ns | +0.8% | +0.5% | -99.9% |
| 3 | 2140224ns | +0.7% | +0.7% | -99.8% |
| 4 | 2144240ns | +0.5% | +0.0% | -99.8% |
| 5 | 2133982ns | +1.2% | +1.0% | -99.9% |
| 6 | 2141603ns | +0.2% | +0.7% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.295 | moderate- |
| abi_lifecycle_real_fresh_per_column | -0.192 | ok |
| abi_lifecycle_real_held_handle | -0.247 | moderate- |
| abi_lifecycle_real_null_entry | -0.559 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 0/6, lost 5/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 6506118.7ns | 2154629.2ns | 302.0% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6503414.4ns | 2154935.6ns | 301.8% | HIGH |
| abi_lifecycle_real_held_handle | 6466484.1ns | 2141471.6ns | 302.0% | HIGH |
| abi_lifecycle_real_null_entry | 121236.0ns | 3171.4ns | 3822.8% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2144830.4-2158762.5 ns)
  2144830.4 |########################################
  2145527.0 |
  2146223.6 |
  2146920.2 |
  2147616.8 |
  2148313.4 |
  2149010.0 |
  2149706.6 |
  2150403.2 |
  2151099.8 |
  2151796.5 |
  2152493.1 |
  2153189.7 |
  2153886.3 |########################################
  2154582.9 |########################################
  2155279.5 |
  2155976.1 |########################################
  2156672.7 |
  2157369.3 |
  2158065.9 |########################################
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2145081.2-2160602.5 ns)
  2145081.2 |####################
  2145857.3 |
  2146633.3 |
  2147409.4 |
  2148185.5 |
  2148961.5 |
  2149737.6 |
  2150513.7 |
  2151289.7 |
  2152065.8 |####################
  2152841.9 |
  2153617.9 |
  2154394.0 |
  2155170.0 |########################################
  2155946.1 |
  2156722.2 |####################
  2157498.2 |
  2158274.3 |
  2159050.4 |
  2159826.4 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2133982.1-2145568.4 ns)
  2133982.1 |####################
  2134561.4 |
  2135140.7 |
  2135720.0 |
  2136299.4 |
  2136878.7 |
  2137458.0 |
  2138037.3 |
  2138616.6 |
  2139195.9 |
  2139775.2 |####################
  2140354.5 |
  2140933.9 |
  2141513.2 |########################################
  2142092.5 |
  2142671.8 |
  2143251.1 |
  2143830.4 |####################
  2144409.7 |
  2144989.0 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 3025.8-3259.3 ns)
   3025.8 |########################################
   3037.5 |
   3049.2 |
   3060.8 |
   3072.5 |
   3084.2 |
   3095.9 |
   3107.5 |########################################
   3119.2 |
   3130.9 |
   3142.6 |
   3154.3 |########################################
   3165.9 |
   3177.6 |
   3189.3 |
   3201.0 |
   3212.6 |########################################
   3224.3 |
   3236.0 |########################################
   3247.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=3807.3% of algo (FFI overhead may distort results)

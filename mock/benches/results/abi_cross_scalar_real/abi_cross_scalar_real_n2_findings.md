# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 63749% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (3.40 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 63749%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 644.6x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.19 ms) is 644.6x the fastest (3.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (63749% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 63749% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 644.6x the fastest

Fastest abi_cross_scalar_real_null_entry (3.40 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.19 ms): 644.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 3397.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 644.65x (fastest 3397.5 ns, slowest 2190194.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2211632ns | 2193649ns | 2186383ns | 2191673ns | 2254194ns | +1.81% |
| abi_cross_scalar_real_inproc_direct | 2172323ns | 2172335ns | 2161230ns | 2170402ns | 2180750ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2191454ns | 2188933ns | 2181452ns | 2187305ns | 2202680ns | +0.88% |
| abi_cross_scalar_real_null_entry | 5671ns | 5635ns | 5563ns | 5618ns | 5806ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2208155ns | 2182872ns | 2250543ns | +1.79% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2169261ns | 2158258ns | 2177615ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2188225ns | 2178321ns | 2199296ns | +0.87% | 0.000 |
| abi_cross_scalar_real_null_entry | 3407ns | 3363ns | 3451ns | -99.84% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 69499.9 | 2204013.1 | 2208155.3 | n/a |
| abi_cross_scalar_real_inproc_direct | 9850.1 | 2168208.0 | 2169261.2 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10014.6 | 2195393.6 | 2188224.6 | n/a |
| abi_cross_scalar_real_null_entry | 28042.3 | 3503.7 | 3406.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_real_null_entry | 0.001 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2211632ns | 2211632ns | +1.81% |
| abi_cross_scalar_real_inproc_direct | 2172323ns | 2172323ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2191454ns | 2191454ns | +0.88% |
| abi_cross_scalar_real_null_entry | 5671ns | 5671ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2169262ns | base | --- | [2160907, 2177615] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2190194ns | +25237.1ns (+1.2%) | [+11693, +79752]ns | [2183729, 2250543] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2185838ns | +17112.7ns (+0.8%) | [+13802, +25975]ns | [2179540, 2199296] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 3398ns | -2165879.6ns (-99.8%) | [-2174228, -2157456]ns | [3371, 3451] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2183092ns | +1.5% | +0.8% | -99.8% |
| 2 | 2172137ns | +0.5% | +0.6% | -99.8% |
| 3 | 2158258ns | +1.7% | +0.9% | -99.8% |
| 4 | 2171935ns | +0.6% | +0.7% | -99.8% |
| 5 | 2166589ns | +0.8% | +1.5% | -99.8% |
| 6 | 2163556ns | +5.6% | +0.8% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.107 | ok |
| abi_cross_scalar_real_inproc_direct | -0.036 | ok |
| abi_cross_scalar_real_inproc_fnptr | -0.203 | moderate- |
| abi_cross_scalar_real_null_entry | 0.029 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6686075.1ns | 2208155.3ns | 302.8% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6516323.1ns | 2169261.2ns | 300.4% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6589659.6ns | 2188224.6ns | 301.1% | HIGH |
| abi_cross_scalar_real_null_entry | 120536.7ns | 3406.5ns | 3538.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2182872.1-2250543.0 ns)
  2182872.1 |########################################
  2186255.6 |
  2189639.2 |
  2193022.7 |#############
  2196406.3 |
  2199789.8 |
  2203173.4 |
  2206556.9 |
  2209940.4 |
  2213324.0 |#############
  2216707.5 |
  2220091.1 |
  2223474.6 |
  2226858.2 |
  2230241.7 |
  2233625.2 |
  2237008.8 |
  2240392.3 |
  2243775.9 |
  2247159.4 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2158258.3-2177614.6 ns)
  2158258.3 |####################
  2159226.1 |
  2160193.9 |
  2161161.7 |
  2162129.6 |
  2163097.4 |####################
  2164065.2 |
  2165033.0 |
  2166000.8 |####################
  2166968.6 |
  2167936.5 |
  2168904.3 |
  2169872.1 |
  2170839.9 |
  2171807.7 |########################################
  2172775.5 |
  2173743.3 |
  2174711.2 |
  2175679.0 |
  2176646.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2178320.8-2199295.9 ns)
  2178320.8 |########################################
  2179369.6 |
  2180418.3 |########################################
  2181467.1 |
  2182515.8 |
  2183564.6 |########################################
  2184613.3 |
  2185662.1 |
  2186710.8 |########################################
  2187759.6 |
  2188808.3 |
  2189857.1 |
  2190905.8 |
  2191954.6 |
  2193003.3 |
  2194052.1 |
  2195100.8 |
  2196149.6 |
  2197198.3 |
  2198247.1 |########################################
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 3362.9-3451.2 ns)
   3362.9 |####################
   3367.3 |
   3371.7 |
   3376.2 |####################
   3380.6 |
   3385.0 |
   3389.4 |####################
   3393.8 |
   3398.2 |########################################
   3402.7 |
   3407.1 |
   3411.5 |
   3415.9 |
   3420.3 |
   3424.7 |
   3429.2 |
   3433.6 |
   3438.0 |
   3442.4 |
   3446.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3544.5% of algo (FFI overhead may distort results)

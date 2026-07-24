# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 68383% faster than the next best (abi_cross_scalar_scatter_inproc_fnptr)

abi_cross_scalar_scatter_null_entry (3.18 us) leads abi_cross_scalar_scatter_inproc_fnptr (2.18 ms) by 68383%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.18 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_ffi_batched_scalar is an outlier: 686.4x slower than the field

abi_cross_scalar_scatter_ffi_batched_scalar (2.18 ms) is 686.4x the fastest (3.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_fnptr, abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar} (68383% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_fnptr, abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar} with a 68383% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 686.4x the fastest

Fastest abi_cross_scalar_scatter_null_entry (3.18 us) to slowest abi_cross_scalar_scatter_ffi_batched_scalar (2.18 ms): 686.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 3178.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 686.42x (fastest 3178.3 ns, slowest 2181686.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2245614ns | 2185353ns | 2164374ns | 2179936ns | 2384750ns | +2.80% |
| abi_cross_scalar_scatter_inproc_direct | 2184446ns | 2184637ns | 2145040ns | 2174906ns | 2218460ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2220317ns | 2179666ns | 2168029ns | 2178459ns | 2309248ns | +1.64% |
| abi_cross_scalar_scatter_null_entry | 5543ns | 5546ns | 5373ns | 5521ns | 5661ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2241761ns | 2161198ns | 2380078ns | +2.79% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2180923ns | 2142426ns | 2214672ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2217041ns | 2165083ns | 2305479ns | +1.66% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 3179ns | 3119ns | 3217ns | -99.85% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 75830.8 | 2225215.7 | 2241761.4 | n/a |
| abi_cross_scalar_scatter_inproc_direct | 8109.0 | 2184948.0 | 2180923.0 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 7991.4 | 2204173.9 | 2217041.1 | 0 |
| abi_cross_scalar_scatter_null_entry | 28857.0 | 3386.1 | 3178.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_scatter_null_entry | 0.081 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2245614ns | 2245614ns | +2.80% |
| abi_cross_scalar_scatter_inproc_direct | 2184446ns | 2184446ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2220317ns | 2220317ns | +1.64% |
| abi_cross_scalar_scatter_null_entry | 5543ns | 5543ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2180702ns | base | --- | [2147394, 2214672] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2181687ns | no significant difference | [-7820, +166955]ns | [2163520, 2380078] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2176631ns | no significant difference | [-40346, +132896]ns | [2169014, 2305479] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_scatter_null_entry | 3178ns | -2177505.6ns (-99.9%) | [-2211531, -2144196]ns | [3141, 3217] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2152362ns | +1.3% | +1.4% | -99.9% |
| 2 | 2142426ns | +0.9% | +1.6% | -99.9% |
| 3 | 2197153ns | +5.4% | +10.5% | -99.9% |
| 4 | 2229091ns | +9.7% | -2.5% | -99.9% |
| 5 | 2200253ns | -1.6% | -1.1% | -99.9% |
| 6 | 2164252ns | +0.9% | +0.0% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.047 | ok |
| abi_cross_scalar_scatter_inproc_direct | 0.337 | moderate+ |
| abi_cross_scalar_scatter_inproc_fnptr | -0.230 | moderate- |
| abi_cross_scalar_scatter_null_entry | 0.354 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 2/6, lost 3/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6767837.4ns | 2241761.4ns | 301.9% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6660926.9ns | 2180923.0ns | 305.4% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6662939.5ns | 2217041.1ns | 300.5% | HIGH |
| abi_cross_scalar_scatter_null_entry | 121913.0ns | 3178.7ns | 3835.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2161198.3-2380077.5 ns)
  2161198.3 |########################################
  2172142.3 |####################
  2183086.2 |####################
  2194030.2 |
  2204974.1 |
  2215918.1 |
  2226862.1 |
  2237806.0 |
  2248750.0 |
  2259693.9 |
  2270637.9 |
  2281581.9 |
  2292525.8 |
  2303469.8 |
  2314413.7 |####################
  2325357.7 |
  2336301.7 |
  2347245.6 |
  2358189.6 |
  2369133.5 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2142425.8-2214672.2 ns)
  2142425.8 |########################################
  2146038.1 |
  2149650.4 |########################################
  2153262.8 |
  2156875.1 |
  2160487.4 |
  2164099.7 |########################################
  2167712.1 |
  2171324.4 |
  2174936.7 |
  2178549.0 |
  2182161.3 |
  2185773.7 |
  2189386.0 |
  2192998.3 |
  2196610.6 |########################################
  2200223.0 |########################################
  2203835.3 |
  2207447.6 |
  2211059.9 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2165082.9-2305478.8 ns)
  2165082.9 |#############
  2172102.7 |########################################
  2179122.5 |#############
  2186142.3 |
  2193162.1 |
  2200181.9 |
  2207201.7 |
  2214221.5 |
  2221241.3 |
  2228261.1 |
  2235280.8 |
  2242300.6 |
  2249320.4 |
  2256340.2 |
  2263360.0 |
  2270379.8 |
  2277399.6 |
  2284419.4 |
  2291439.2 |
  2298459.0 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 3119.2-3216.7 ns)
   3119.2 |########################################
   3124.1 |
   3128.9 |
   3133.8 |
   3138.7 |
   3143.6 |
   3148.4 |
   3153.3 |
   3158.2 |########################################
   3163.1 |########################################
   3167.9 |
   3172.8 |
   3177.7 |
   3182.6 |
   3187.4 |########################################
   3192.3 |
   3197.2 |
   3202.1 |########################################
   3206.9 |
   3211.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=3842.4% of algo (FFI overhead may distort results)

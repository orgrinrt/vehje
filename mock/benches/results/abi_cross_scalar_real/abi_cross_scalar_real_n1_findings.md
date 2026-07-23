# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 42795% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (5.04 us) leads abi_cross_scalar_real_inproc_direct (2.16 ms) by 42795%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 433.8x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.19 ms) is 433.8x the fastest (5.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry shows alternating (throttle bounce) (autocorr -0.74)

abi_cross_scalar_real_null_entry's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (42795% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 42795% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 433.8x the fastest

Fastest abi_cross_scalar_real_null_entry (5.04 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.19 ms): 433.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 5045.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 433.79x (fastest 5045.0 ns, slowest 2188454.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2197561ns | 2184988ns | 2177870ns | 2183052ns | 2229170ns | +0.66% |
| abi_cross_scalar_real_inproc_direct | 2183171ns | 2167363ns | 2161519ns | 2166036ns | 2219698ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2214027ns | 2191572ns | 2181445ns | 2188390ns | 2268773ns | +1.41% |
| abi_cross_scalar_real_null_entry | 7362ns | 7389ns | 6965ns | 7280ns | 7684ns | -99.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2194055ns | 2174622ns | 2225288ns | +0.66% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2179762ns | 2158451ns | 2215911ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2210604ns | 2178428ns | 2264726ns | +1.41% | 0.000 |
| abi_cross_scalar_real_null_entry | 5033ns | 4754ns | 5257ns | -99.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 72274.0 | 2197362.1 | 2194055.1 | n/a |
| abi_cross_scalar_real_inproc_direct | 10647.1 | 2177825.6 | 2179761.6 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 11064.7 | 2215209.6 | 2210603.8 | n/a |
| abi_cross_scalar_real_null_entry | 29995.4 | 5068.3 | 5033.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_real_null_entry | 0.000 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2197561ns | 2197561ns | +0.66% |
| abi_cross_scalar_real_inproc_direct | 2183171ns | 2183171ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2214027ns | 2214027ns | +1.41% |
| abi_cross_scalar_real_null_entry | 7362ns | 7362ns | -99.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2164051ns | base | --- | [2159323, 2215911] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2181704ns | +16355.0ns (+0.8%) | [+1294, +25232]ns | [2175174, 2225288] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2188454ns | +21329.6ns (+1.0%) | [+14758, +56439]ns | [2178631, 2264726] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 5045ns | -2159180.2ns (-99.8%) | [-2210910, -2154095]ns | [4797, 5257] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2256378ns | +0.2% | +3.6% | -99.8% |
| 2 | 2167321ns | +0.9% | +1.0% | -99.8% |
| 3 | 2160195ns | +1.4% | +1.5% | -99.8% |
| 4 | 2160780ns | +0.7% | +0.8% | -99.8% |
| 5 | 2158451ns | +0.8% | +0.9% | -99.8% |
| 6 | 2175444ns | -0.0% | +0.5% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.056 | ok |
| abi_cross_scalar_real_inproc_direct | 0.022 | ok |
| abi_cross_scalar_real_inproc_fnptr | 0.006 | ok |
| abi_cross_scalar_real_null_entry | -0.736 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 5/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6672496.4ns | 2194055.1ns | 304.1% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6560520.8ns | 2179761.6ns | 301.0% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6652120.2ns | 2210603.8ns | 300.9% | HIGH |
| abi_cross_scalar_real_null_entry | 127589.0ns | 5033.0ns | 2535.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2174621.7-2225288.1 ns)
  2174621.7 |########################################
  2177155.0 |
  2179688.3 |
  2182221.7 |
  2184755.0 |#############
  2187288.3 |
  2189821.6 |#############
  2192354.9 |
  2194888.3 |
  2197421.6 |
  2199954.9 |
  2202488.2 |
  2205021.5 |
  2207554.9 |
  2210088.2 |
  2212621.5 |
  2215154.8 |
  2217688.1 |
  2220221.5 |
  2222754.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2158450.8-2215911.0 ns)
  2158450.8 |########################################
  2161323.8 |
  2164196.8 |
  2167069.8 |#############
  2169942.8 |
  2172815.9 |#############
  2175688.9 |
  2178561.9 |
  2181434.9 |
  2184307.9 |
  2187180.9 |
  2190053.9 |
  2192926.9 |
  2195800.0 |
  2198673.0 |
  2201546.0 |
  2204419.0 |
  2207292.0 |
  2210165.0 |
  2213038.0 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2178428.3-2264726.2 ns)
  2178428.3 |########################################
  2182743.2 |####################
  2187058.1 |####################
  2191373.0 |####################
  2195687.9 |
  2200002.8 |
  2204317.7 |
  2208632.6 |
  2212947.5 |
  2217262.4 |
  2221577.3 |
  2225892.2 |
  2230207.1 |
  2234522.0 |
  2238836.9 |
  2243151.8 |
  2247466.7 |
  2251781.6 |
  2256096.5 |
  2260411.4 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 4754.2-5256.9 ns)
   4754.2 |########################################
   4779.3 |
   4804.5 |
   4829.6 |########################################
   4854.7 |
   4879.9 |########################################
   4905.0 |
   4930.1 |
   4955.3 |
   4980.4 |
   5005.5 |
   5030.7 |
   5055.8 |
   5081.0 |
   5106.1 |
   5131.2 |
   5156.4 |
   5181.5 |########################################
   5206.6 |
   5231.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=2535.3% of algo (FFI overhead may distort results)

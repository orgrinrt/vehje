# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 85866% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (2.53 us) leads abi_cross_scalar_scatter_inproc_direct (2.17 ms) by 85866%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 862.9x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.18 ms) is 862.9x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_scatter_inproc_fnptr shows alternating (throttle bounce) (autocorr -0.58)

abi_cross_scalar_scatter_inproc_fnptr's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (85866% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 85866% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 862.9x the fastest

Fastest abi_cross_scalar_scatter_null_entry (2.53 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.18 ms): 862.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 2525.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 862.90x (fastest 2525.8 ns, slowest 2179560.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2180751ns | 2179714ns | 2173198ns | 2178089ns | 2188520ns | +0.16% |
| abi_cross_scalar_scatter_inproc_direct | 2177322ns | 2174908ns | 2158693ns | 2173884ns | 2191793ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2187238ns | 2182684ns | 2176679ns | 2180963ns | 2201931ns | +0.46% |
| abi_cross_scalar_scatter_null_entry | 4885ns | 4838ns | 4725ns | 4805ns | 5086ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2177112ns | 2169914ns | 2184790ns | +0.15% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2173843ns | 2155584ns | 2188152ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2183880ns | 2173358ns | 2198319ns | +0.46% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 2548ns | 2478ns | 2630ns | -99.88% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 71981.2 | 2176599.5 | 2177112.4 | n/a |
| abi_cross_scalar_scatter_inproc_direct | 8099.8 | 2170932.5 | 2173842.5 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 8102.9 | 2194655.5 | 2183879.9 | n/a |
| abi_cross_scalar_scatter_null_entry | 27637.2 | 2831.6 | 2547.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_scatter_null_entry | 0.025 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2180751ns | 2180751ns | +0.16% |
| abi_cross_scalar_scatter_inproc_direct | 2177322ns | 2177322ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2187238ns | 2187238ns | +0.46% |
| abi_cross_scalar_scatter_null_entry | 4885ns | 4885ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2171369ns | base | --- | [2162006, 2188152] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2175971ns | no significant difference | [-12780, +22736]ns | [2170576, 2184790] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2179560ns | +7371.4ns (+0.3%) | [+2439, +20301]ns | [2173760, 2198319] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_scatter_null_entry | 2526ns | -2168844.6ns (-99.9%) | [-2185536, -2159504]ns | [2488, 2630] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2168524ns | +0.9% | +0.3% | -99.9% |
| 2 | 2183807ns | -0.2% | +0.6% | -99.9% |
| 3 | 2168428ns | +0.2% | +0.3% | -99.9% |
| 4 | 2155584ns | +1.2% | +1.3% | -99.9% |
| 5 | 2192498ns | -1.0% | +0.3% | -99.9% |
| 6 | 2174215ns | -0.2% | -0.0% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.031 | ok |
| abi_cross_scalar_scatter_inproc_direct | -0.408 | moderate- |
| abi_cross_scalar_scatter_inproc_fnptr | -0.576 | HIGH- (thermal bounce) |
| abi_cross_scalar_scatter_null_entry | 0.001 | ok |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 0/6, lost 5/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6609308.2ns | 2177112.4ns | 303.6% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6524026.4ns | 2173842.5ns | 300.1% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6578264.6ns | 2183879.9ns | 301.2% | HIGH |
| abi_cross_scalar_scatter_null_entry | 113989.2ns | 2547.8ns | 4473.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2169913.8-2184790.5 ns)
  2169913.8 |########################################
  2170657.6 |########################################
  2171401.5 |
  2172145.3 |########################################
  2172889.1 |
  2173633.0 |
  2174376.8 |
  2175120.6 |
  2175864.5 |
  2176608.3 |
  2177352.1 |
  2178096.0 |
  2178839.8 |
  2179583.6 |########################################
  2180327.5 |
  2181071.3 |########################################
  2181815.1 |
  2182559.0 |
  2183302.8 |
  2184046.6 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2155583.8-2188152.3 ns)
  2155583.8 |####################
  2157212.2 |
  2158840.6 |
  2160469.1 |
  2162097.5 |
  2163725.9 |
  2165354.3 |
  2166982.8 |########################################
  2168611.2 |
  2170239.6 |
  2171868.0 |
  2173496.5 |####################
  2175124.9 |
  2176753.3 |
  2178381.8 |
  2180010.2 |
  2181638.6 |
  2183267.0 |####################
  2184895.4 |
  2186523.9 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2173358.3-2198319.0 ns)
  2173358.3 |########################################
  2174606.3 |####################
  2175854.4 |
  2177102.4 |
  2178350.4 |
  2179598.5 |
  2180846.5 |
  2182094.5 |
  2183342.6 |####################
  2184590.6 |
  2185838.6 |
  2187086.7 |
  2188334.7 |
  2189582.7 |
  2190830.8 |
  2192078.8 |
  2193326.8 |
  2194574.9 |
  2195822.9 |####################
  2197070.9 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 2477.5-2630.0 ns)
   2477.5 |########################################
   2485.1 |
   2492.8 |########################################
   2500.4 |########################################
   2508.0 |
   2515.6 |
   2523.2 |
   2530.9 |
   2538.5 |########################################
   2546.1 |
   2553.8 |
   2561.4 |
   2569.0 |########################################
   2576.6 |
   2584.2 |
   2591.9 |
   2599.5 |
   2607.1 |
   2614.8 |
   2622.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=4501.8% of algo (FFI overhead may distort results)

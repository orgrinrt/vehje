# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 69232% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (3.13 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 69232%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 701.6x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.19 ms) is 701.6x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (69232% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 69232% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 701.6x the fastest

Fastest abi_cross_scalar_real_null_entry (3.13 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.19 ms): 701.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 3127.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 701.62x (fastest 3127.9 ns, slowest 2194590.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2198321ns | 2191954ns | 2165096ns | 2187732ns | 2230816ns | +0.72% |
| abi_cross_scalar_real_inproc_direct | 2182538ns | 2171982ns | 2164517ns | 2171799ns | 2207656ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2303465ns | 2198323ns | 2179647ns | 2195576ns | 2527207ns | +5.54% |
| abi_cross_scalar_real_null_entry | 5467ns | 5355ns | 5260ns | 5352ns | 5742ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2194776ns | 2162138ns | 2226904ns | +0.72% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2179124ns | 2161529ns | 2204034ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2299740ns | 2176541ns | 2523037ns | +5.54% | 0.000 |
| abi_cross_scalar_real_null_entry | 3180ns | 3082ns | 3314ns | -99.85% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 75286.2 | 2202807.6 | 2194775.6 | n/a |
| abi_cross_scalar_real_inproc_direct | 10603.9 | 2180984.2 | 2179124.3 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 12408.6 | 2272609.6 | 2299740.0 | n/a |
| abi_cross_scalar_real_null_entry | 27752.5 | 3170.3 | 3179.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.082 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2198321ns | 2198321ns | +0.72% |
| abi_cross_scalar_real_inproc_direct | 2182538ns | 2182538ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2303465ns | 2303465ns | +5.54% |
| abi_cross_scalar_real_null_entry | 5467ns | 5467ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2168624ns | base | --- | [2164714, 2204034] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2188443ns | no significant difference | [-29800, +58977]ns | [2168980, 2226904] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2194590ns | no significant difference | [-18837, +354412]ns | [2181593, 2523037] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_null_entry | 3128ns | -2165495.5ns (-99.9%) | [-2200910, -2161429]ns | [3097, 3314] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2167955ns | +3.9% | +10.5% | -99.8% |
| 2 | 2169294ns | +1.1% | +22.2% | -99.9% |
| 3 | 2167900ns | +1.6% | +0.9% | -99.9% |
| 4 | 2236030ns | -2.7% | -1.9% | -99.9% |
| 5 | 2161529ns | +0.0% | +1.6% | -99.9% |
| 6 | 2172038ns | +0.5% | +0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.156 | ok |
| abi_cross_scalar_real_inproc_direct | -0.328 | moderate- |
| abi_cross_scalar_real_inproc_fnptr | 0.163 | ok |
| abi_cross_scalar_real_null_entry | -0.145 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 1/6, lost 4/6
- **abi_cross_scalar_real_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6670702.9ns | 2194775.6ns | 303.9% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6552289.9ns | 2179124.3ns | 300.7% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6848115.0ns | 2299740.0ns | 297.8% | HIGH |
| abi_cross_scalar_real_null_entry | 121680.1ns | 3179.6ns | 3826.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2162138.3-2226904.0 ns)
  2162138.3 |########################################
  2165376.6 |
  2168614.9 |
  2171853.1 |
  2175091.4 |########################################
  2178329.7 |
  2181568.0 |########################################
  2184806.3 |
  2188044.6 |
  2191282.8 |########################################
  2194521.1 |
  2197759.4 |
  2200997.7 |########################################
  2204236.0 |
  2207474.3 |
  2210712.5 |
  2213950.8 |
  2217189.1 |
  2220427.4 |
  2223665.7 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2161529.2-2204034.1 ns)
  2161529.2 |####################
  2163654.4 |
  2165779.7 |####################
  2167904.9 |########################################
  2170030.2 |####################
  2172155.4 |
  2174280.7 |
  2176405.9 |
  2178531.2 |
  2180656.4 |
  2182781.7 |
  2184906.9 |
  2187032.2 |
  2189157.4 |
  2191282.7 |
  2193407.9 |
  2195533.2 |
  2197658.4 |
  2199783.7 |
  2201908.9 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2176540.8-2523036.9 ns)
  2176540.8 |########################################
  2193865.6 |#############
  2211190.4 |
  2228515.2 |
  2245840.0 |
  2263164.8 |
  2280489.6 |
  2297814.4 |
  2315139.2 |
  2332464.0 |
  2349788.8 |
  2367113.6 |
  2384438.4 |#############
  2401763.2 |
  2419088.0 |
  2436412.8 |
  2453737.6 |
  2471062.4 |
  2488387.2 |
  2505712.0 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 3082.1-3314.1 ns)
   3082.1 |####################
   3093.7 |
   3105.3 |####################
   3116.9 |####################
   3128.5 |########################################
   3140.1 |
   3151.7 |
   3163.3 |
   3174.9 |
   3186.5 |
   3198.1 |
   3209.7 |
   3221.3 |
   3232.9 |
   3244.5 |
   3256.1 |
   3267.7 |
   3279.3 |
   3290.9 |
   3302.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3822.0% of algo (FFI overhead may distort results)

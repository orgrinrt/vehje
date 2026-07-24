# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 63737% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (3.40 us) leads abi_cross_scalar_scatter_inproc_direct (2.17 ms) by 63737%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 645.7x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.20 ms) is 645.7x the fastest (3.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (63737% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 63737% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 645.7x the fastest

Fastest abi_cross_scalar_scatter_null_entry (3.40 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.20 ms): 645.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 3400.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 645.69x (fastest 3400.0 ns, slowest 2195359.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2195106ns | 2191571ns | 2175304ns | 2188537ns | 2214862ns | -2.36% |
| abi_cross_scalar_scatter_inproc_direct | 2248124ns | 2173639ns | 2154574ns | 2172016ns | 2409060ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2225183ns | 2198767ns | 2182818ns | 2195606ns | 2290729ns | -1.02% |
| abi_cross_scalar_scatter_null_entry | 5702ns | 5661ns | 5561ns | 5647ns | 5855ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2191539ns | 2172042ns | 2211070ns | -2.37% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2244828ns | 2151751ns | 2405313ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2221791ns | 2179905ns | 2286942ns | -1.03% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 3424ns | 3372ns | 3497ns | -99.85% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 70265.2 | 2191467.7 | 2191538.5 | 1 |
| abi_cross_scalar_scatter_inproc_direct | 7899.0 | 2211877.5 | 2244828.4 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 8018.4 | 2288746.5 | 2221790.8 | 0 |
| abi_cross_scalar_scatter_null_entry | 27359.4 | 3563.3 | 3424.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_scatter_null_entry | 0.001 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2195106ns | 2195106ns | -2.36% |
| abi_cross_scalar_scatter_inproc_direct | 2248124ns | 2248124ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2225183ns | 2225183ns | -1.02% |
| abi_cross_scalar_scatter_null_entry | 5702ns | 5702ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2170456ns | base | --- | [2158716, 2405313] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2188049ns | no significant difference | [-201207, +30199]ns | [2175496, 2211070] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2195359ns | no significant difference | [-132189, +44169]ns | [2183071, 2286942] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_scatter_null_entry | 3400ns | -2167063.8ns (-99.8%) | [-2401888, -2155261]ns | [3375, 3497] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2182768ns | +0.5% | +0.7% | -99.8% |
| 2 | 2151751ns | +1.4% | +1.3% | -99.8% |
| 3 | 2173678ns | -0.1% | +0.9% | -99.8% |
| 4 | 2165681ns | +1.4% | +2.8% | -99.8% |
| 5 | 2167234ns | +0.5% | +0.9% | -99.8% |
| 6 | 2627858ns | -15.3% | -10.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | -0.214 | moderate- |
| abi_cross_scalar_scatter_inproc_direct | -0.031 | ok |
| abi_cross_scalar_scatter_inproc_fnptr | -0.125 | ok |
| abi_cross_scalar_scatter_null_entry | -0.324 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 1/6, lost 4/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6649611.9ns | 2191538.5ns | 303.4% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6619741.3ns | 2244828.4ns | 294.9% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6864024.9ns | 2221790.8ns | 308.9% | HIGH |
| abi_cross_scalar_scatter_null_entry | 120319.5ns | 3424.1ns | 3513.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2172042.1-2211070.4 ns)
  2172042.1 |########################################
  2173993.5 |
  2175944.9 |
  2177896.3 |########################################
  2179847.8 |
  2181799.2 |########################################
  2183750.6 |
  2185702.0 |
  2187653.4 |
  2189604.8 |
  2191556.2 |########################################
  2193507.7 |########################################
  2195459.1 |
  2197410.5 |
  2199361.9 |
  2201313.3 |
  2203264.7 |
  2205216.2 |
  2207167.6 |
  2209119.0 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2151750.8-2405313.3 ns)
  2151750.8 |#############
  2164428.9 |########################################
  2177107.0 |#############
  2189785.2 |
  2202463.3 |
  2215141.4 |
  2227819.5 |
  2240497.7 |
  2253175.8 |
  2265853.9 |
  2278532.0 |
  2291210.2 |
  2303888.3 |
  2316566.4 |
  2329244.5 |
  2341922.7 |
  2354600.8 |
  2367278.9 |
  2379957.0 |
  2392635.2 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2179905.4-2286941.9 ns)
  2179905.4 |########################################
  2185257.2 |########################################
  2190609.0 |########################################
  2195960.9 |########################################
  2201312.7 |
  2206664.5 |
  2212016.4 |
  2217368.2 |
  2222720.0 |########################################
  2228071.8 |
  2233423.6 |
  2238775.5 |
  2244127.3 |
  2249479.1 |
  2254830.9 |
  2260182.8 |
  2265534.6 |
  2270886.4 |
  2276238.2 |
  2281590.1 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 3372.5-3496.8 ns)
   3372.5 |########################################
   3378.7 |
   3384.9 |####################
   3391.2 |
   3397.4 |
   3403.6 |
   3409.8 |####################
   3416.0 |
   3422.2 |
   3428.5 |
   3434.7 |
   3440.9 |
   3447.1 |
   3453.3 |
   3459.5 |####################
   3465.8 |
   3472.0 |
   3478.2 |
   3484.4 |
   3490.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=3546.2% of algo (FFI overhead may distort results)

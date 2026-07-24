# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 85315% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.53 us) leads abi_cross_scalar_real_inproc_direct (2.16 ms) by 85315%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 866.4x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.19 ms) is 866.4x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (85315% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 85315% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 866.4x the fastest

Fastest abi_cross_scalar_real_null_entry (2.53 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.19 ms): 866.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2531.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 866.40x (fastest 2531.7 ns, slowest 2193414.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2216029ns | 2196952ns | 2183366ns | 2193693ns | 2265864ns | +2.30% |
| abi_cross_scalar_real_inproc_direct | 2166105ns | 2165457ns | 2158308ns | 2164310ns | 2172696ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2211858ns | 2190872ns | 2186559ns | 2189809ns | 2257582ns | +2.11% |
| abi_cross_scalar_real_null_entry | 4831ns | 4844ns | 4641ns | 4800ns | 4972ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2212378ns | 2179922ns | 2261923ns | +2.29% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2162889ns | 2154870ns | 2169382ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2208326ns | 2183433ns | 2253740ns | +2.10% | 0.000 |
| abi_cross_scalar_real_null_entry | 2538ns | 2468ns | 2595ns | -99.88% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 74241.0 | 2211826.6 | 2212378.0 | n/a |
| abi_cross_scalar_real_inproc_direct | 9872.3 | 2163428.1 | 2162889.1 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10537.8 | 2345965.2 | 2208326.4 | n/a |
| abi_cross_scalar_real_null_entry | 28200.4 | 2626.0 | 2537.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.006 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2216029ns | 2216029ns | +2.30% |
| abi_cross_scalar_real_inproc_direct | 2166105ns | 2166105ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2211858ns | 2211858ns | +2.11% |
| abi_cross_scalar_real_null_entry | 4831ns | 4831ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2162410ns | base | --- | [2156875, 2169382] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2193415ns | +35685.0ns (+1.7%) | [+14678, +98104]ns | [2181796, 2261923] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2187451ns | +30193.2ns (+1.4%) | [+19346, +86773]ns | [2183788, 2253740] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 2532ns | -2159860.8ns (-99.9%) | [-2166825, -2154368]ns | [2486, 2595] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2168758ns | +4.6% | +6.5% | -99.9% |
| 2 | 2165175ns | +0.9% | +1.5% | -99.9% |
| 3 | 2158880ns | +4.5% | +1.1% | -99.9% |
| 4 | 2154870ns | +1.3% | +1.4% | -99.9% |
| 5 | 2170005ns | +0.5% | +0.7% | -99.9% |
| 6 | 2159645ns | +2.0% | +1.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.352 | moderate- |
| abi_cross_scalar_real_inproc_direct | -0.241 | moderate- |
| abi_cross_scalar_real_inproc_fnptr | 0.059 | ok |
| abi_cross_scalar_real_null_entry | -0.152 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6715017.8ns | 2212378.0ns | 303.5% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6505674.0ns | 2162889.1ns | 300.8% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6794541.5ns | 2208326.4ns | 307.7% | HIGH |
| abi_cross_scalar_real_null_entry | 118764.7ns | 2537.6ns | 4680.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2179922.5-2261923.3 ns)
  2179922.5 |########################################
  2184022.5 |####################
  2188122.6 |
  2192222.6 |
  2196322.7 |
  2200422.7 |####################
  2204522.8 |
  2208622.8 |
  2212722.8 |
  2216822.9 |
  2220922.9 |
  2225023.0 |
  2229123.0 |
  2233223.1 |
  2237323.1 |
  2241423.1 |
  2245523.2 |
  2249623.2 |
  2253723.3 |####################
  2257823.3 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2154870.4-2169381.6 ns)
  2154870.4 |########################################
  2155596.0 |
  2156321.5 |
  2157047.1 |
  2157772.6 |
  2158498.2 |########################################
  2159223.8 |########################################
  2159949.3 |
  2160674.9 |
  2161400.5 |
  2162126.0 |
  2162851.6 |
  2163577.1 |
  2164302.7 |
  2165028.3 |########################################
  2165753.8 |
  2166479.4 |
  2167205.0 |
  2167930.5 |
  2168656.1 |########################################
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2183432.9-2253740.0 ns)
  2183432.9 |########################################
  2186948.3 |#############
  2190463.6 |
  2193979.0 |#############
  2197494.3 |
  2201009.7 |
  2204525.0 |
  2208040.4 |
  2211555.7 |
  2215071.1 |
  2218586.5 |
  2222101.8 |
  2225617.2 |
  2229132.5 |
  2232647.9 |
  2236163.2 |
  2239678.6 |
  2243193.9 |
  2246709.3 |
  2250224.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2467.5-2595.4 ns)
   2467.5 |########################################
   2473.9 |
   2480.3 |
   2486.7 |
   2493.1 |
   2499.5 |########################################
   2505.9 |########################################
   2512.3 |
   2518.7 |
   2525.1 |
   2531.4 |
   2537.8 |
   2544.2 |
   2550.6 |########################################
   2557.0 |########################################
   2563.4 |
   2569.8 |
   2576.2 |
   2582.6 |
   2589.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=301.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=4686.7% of algo (FFI overhead may distort results)

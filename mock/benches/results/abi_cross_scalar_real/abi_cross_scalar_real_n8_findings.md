# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 70813% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (3.05 us) leads abi_cross_scalar_real_inproc_direct (2.16 ms) by 70813%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 714.4x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.18 ms) is 714.4x the fastest (3.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (70813% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 70813% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 714.4x the fastest

Fastest abi_cross_scalar_real_null_entry (3.05 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.18 ms): 714.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 3048.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 714.43x (fastest 3048.1 ns, slowest 2177650.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2179443ns | 2174508ns | 2168484ns | 2173600ns | 2193686ns | +0.72% |
| abi_cross_scalar_real_inproc_direct | 2163877ns | 2164689ns | 2157159ns | 2164438ns | 2166393ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2183877ns | 2180749ns | 2176718ns | 2179809ns | 2193558ns | +0.92% |
| abi_cross_scalar_real_null_entry | 5337ns | 5323ns | 5208ns | 5294ns | 5466ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2176134ns | 2165115ns | 2190183ns | +0.72% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2160645ns | 2153877ns | 2163127ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2180698ns | 2173675ns | 2190228ns | +0.93% | 0.000 |
| abi_cross_scalar_real_null_entry | 3035ns | 2930ns | 3087ns | -99.86% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 65604.4 | 2175035.6 | 2176134.0 | n/a |
| abi_cross_scalar_real_inproc_direct | 10161.0 | 2161707.4 | 2160645.3 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 9979.0 | 2184156.2 | 2180698.3 | 0 |
| abi_cross_scalar_real_null_entry | 28826.5 | 3137.5 | 3034.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.003 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2179443ns | 2179443ns | +0.72% |
| abi_cross_scalar_real_inproc_direct | 2163877ns | 2163877ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2183877ns | 2183877ns | +0.92% |
| abi_cross_scalar_real_null_entry | 5337ns | 5337ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2161512ns | base | --- | [2157297, 2163127] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2171300ns | +9912.7ns (+0.5%) | [+4038, +32515]ns | [2166919, 2190183] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2177650ns | +16536.2ns (+0.8%) | [+11362, +32261]ns | [2174216, 2190228] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 3048ns | -2158428.1ns (-99.9%) | [-2160131, -2154273]ns | [2969, 3087] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2161458ns | +2.0% | +0.6% | -99.9% |
| 2 | 2153877ns | +1.0% | +2.1% | -99.9% |
| 3 | 2162058ns | +0.5% | +0.9% | -99.9% |
| 4 | 2161566ns | +0.2% | +0.6% | -99.9% |
| 5 | 2164196ns | +0.2% | +0.4% | -99.9% |
| 6 | 2160717ns | +0.4% | +0.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.168 | ok |
| abi_cross_scalar_real_inproc_direct | -0.165 | ok |
| abi_cross_scalar_real_inproc_fnptr | -0.104 | ok |
| abi_cross_scalar_real_null_entry | -0.039 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6596190.6ns | 2176134.0ns | 303.1% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6499976.3ns | 2160645.3ns | 300.8% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6563216.7ns | 2180698.3ns | 301.0% | HIGH |
| abi_cross_scalar_real_null_entry | 120327.6ns | 3034.7ns | 3965.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2165114.6-2190182.9 ns)
  2165114.6 |########################################
  2166368.0 |
  2167621.4 |########################################
  2168874.8 |
  2170128.3 |########################################
  2171381.7 |########################################
  2172635.1 |
  2173888.5 |
  2175141.9 |########################################
  2176395.3 |
  2177648.8 |
  2178902.2 |
  2180155.6 |
  2181409.0 |
  2182662.4 |
  2183915.8 |
  2185169.2 |
  2186422.7 |
  2187676.1 |
  2188929.5 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2153876.7-2163127.0 ns)
  2153876.7 |####################
  2154339.2 |
  2154801.7 |
  2155264.3 |
  2155726.8 |
  2156189.3 |
  2156651.8 |
  2157114.3 |
  2157576.8 |
  2158039.4 |
  2158501.9 |
  2158964.4 |
  2159426.9 |
  2159889.4 |
  2160351.9 |####################
  2160814.5 |
  2161277.0 |########################################
  2161739.5 |####################
  2162202.0 |
  2162664.5 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2173674.6-2190228.4 ns)
  2173674.6 |####################
  2174502.3 |########################################
  2175330.0 |
  2176157.7 |
  2176985.4 |
  2177813.0 |
  2178640.7 |
  2179468.4 |
  2180296.1 |####################
  2181123.8 |
  2181951.5 |####################
  2182779.2 |
  2183606.9 |
  2184434.5 |
  2185262.2 |
  2186089.9 |
  2186917.6 |
  2187745.3 |
  2188573.0 |
  2189400.7 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2929.6-3087.3 ns)
   2929.6 |####################
   2937.5 |
   2945.4 |
   2953.3 |
   2961.1 |
   2969.0 |
   2976.9 |
   2984.8 |
   2992.7 |
   3000.6 |####################
   3008.4 |
   3016.3 |
   3024.2 |
   3032.1 |
   3040.0 |####################
   3047.9 |
   3055.8 |########################################
   3063.6 |
   3071.5 |
   3079.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3928.4% of algo (FFI overhead may distort results)

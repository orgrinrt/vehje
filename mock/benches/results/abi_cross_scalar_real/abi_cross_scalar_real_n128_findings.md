# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 79494% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.72 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 79494%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 801.3x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.18 ms) is 801.3x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (79494% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 79494% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 801.3x the fastest

Fastest abi_cross_scalar_real_null_entry (2.72 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.18 ms): 801.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2722.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 801.34x (fastest 2722.9 ns, slowest 2182008.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2184643ns | 2185252ns | 2180279ns | 2183992ns | 2187801ns | +0.65% |
| abi_cross_scalar_real_inproc_direct | 2170530ns | 2170474ns | 2162458ns | 2169425ns | 2176224ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2188603ns | 2185151ns | 2177598ns | 2184183ns | 2200737ns | +0.83% |
| abi_cross_scalar_real_null_entry | 5016ns | 4973ns | 4880ns | 4946ns | 5190ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2181194ns | 2176800ns | 2184436ns | +0.64% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2167244ns | 2159260ns | 2172742ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2185334ns | 2174582ns | 2197193ns | +0.83% | 0.000 |
| abi_cross_scalar_real_null_entry | 2736ns | 2647ns | 2825ns | -99.87% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 66809.1 | 2180317.4 | 2181193.7 | n/a |
| abi_cross_scalar_real_inproc_direct | 10140.9 | 2164665.6 | 2167244.0 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10496.9 | 2184699.3 | 2185333.9 | n/a |
| abi_cross_scalar_real_null_entry | 27794.7 | 2786.4 | 2735.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.047 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2184643ns | 2184643ns | +0.65% |
| abi_cross_scalar_real_inproc_direct | 2170530ns | 2170530ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2188603ns | 2188603ns | +0.83% |
| abi_cross_scalar_real_null_entry | 5016ns | 5016ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2167307ns | base | --- | [2161682, 2172742] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2181740ns | +10600.7ns (+0.5%) | [+8779, +22469]ns | [2177405, 2184436] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2182009ns | +15389.0ns (+0.7%) | [+8058, +30823]ns | [2176800, 2197193] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 2723ns | -2164546.0ns (-99.9%) | [-2170019, -2158960]ns | [2660, 2825] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2165441ns | +0.5% | +0.6% | -99.9% |
| 2 | 2171798ns | +0.5% | +0.5% | -99.9% |
| 3 | 2169173ns | +0.4% | +0.2% | -99.9% |
| 4 | 2164105ns | +0.8% | +0.8% | -99.9% |
| 5 | 2159260ns | +1.3% | +2.0% | -99.9% |
| 6 | 2173688ns | +0.4% | +0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.074 | ok |
| abi_cross_scalar_real_inproc_direct | -0.223 | moderate- |
| abi_cross_scalar_real_inproc_fnptr | 0.224 | moderate+ |
| abi_cross_scalar_real_null_entry | 0.133 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6608978.3ns | 2181193.7ns | 303.0% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6508458.5ns | 2167244.0ns | 300.3% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6566271.3ns | 2185333.9ns | 300.5% | HIGH |
| abi_cross_scalar_real_null_entry | 120123.5ns | 2735.8ns | 4390.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2176800.0-2184436.5 ns)
  2176800.0 |########################################
  2177181.8 |
  2177563.6 |
  2177945.5 |########################################
  2178327.3 |
  2178709.1 |
  2179090.9 |
  2179472.8 |
  2179854.6 |
  2180236.4 |
  2180618.2 |
  2181000.0 |
  2181381.9 |########################################
  2181763.7 |########################################
  2182145.5 |########################################
  2182527.3 |
  2182909.2 |
  2183291.0 |
  2183672.8 |
  2184054.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2159259.6-2172742.5 ns)
  2159259.6 |########################################
  2159933.7 |
  2160607.9 |
  2161282.0 |
  2161956.2 |
  2162630.3 |
  2163304.5 |
  2163978.6 |########################################
  2164652.8 |
  2165326.9 |########################################
  2166001.0 |
  2166675.2 |
  2167349.3 |
  2168023.5 |
  2168697.6 |########################################
  2169371.8 |
  2170045.9 |
  2170720.1 |
  2171394.2 |########################################
  2172068.4 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2174582.5-2197193.4 ns)
  2174582.5 |########################################
  2175713.0 |
  2176843.6 |
  2177974.1 |########################################
  2179104.7 |
  2180235.2 |
  2181365.8 |########################################
  2182496.3 |########################################
  2183626.8 |
  2184757.4 |
  2185887.9 |
  2187018.5 |
  2188149.0 |
  2189279.6 |
  2190410.1 |########################################
  2191540.6 |
  2192671.2 |
  2193801.7 |
  2194932.3 |
  2196062.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2647.1-2824.8 ns)
   2647.1 |########################################
   2656.0 |
   2664.9 |########################################
   2673.8 |
   2682.6 |
   2691.5 |
   2700.4 |
   2709.3 |
   2718.2 |########################################
   2727.1 |########################################
   2735.9 |
   2744.8 |
   2753.7 |
   2762.6 |
   2771.5 |
   2780.4 |
   2789.3 |
   2798.1 |########################################
   2807.0 |
   2815.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=4414.5% of algo (FFI overhead may distort results)

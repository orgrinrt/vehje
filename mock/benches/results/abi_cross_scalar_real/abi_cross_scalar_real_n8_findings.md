# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 70895% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (3.06 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 70895%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 716.2x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.19 ms) is 716.2x the fastest (3.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (70895% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 70895% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 716.2x the fastest

Fastest abi_cross_scalar_real_null_entry (3.06 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.19 ms): 716.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 3057.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 716.20x (fastest 3057.1 ns, slowest 2189500.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2197723ns | 2193241ns | 2176836ns | 2190727ns | 2218661ns | +1.16% |
| abi_cross_scalar_real_inproc_direct | 2172541ns | 2173713ns | 2165298ns | 2172824ns | 2175738ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2218760ns | 2193090ns | 2183328ns | 2190529ns | 2278824ns | +2.13% |
| abi_cross_scalar_real_null_entry | 5314ns | 5296ns | 5185ns | 5270ns | 5445ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2194133ns | 2173912ns | 2214823ns | +1.15% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2169218ns | 2162158ns | 2172445ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2215113ns | 2180092ns | 2274665ns | +2.12% | 0.000 |
| abi_cross_scalar_real_null_entry | 3065ns | 2990ns | 3137ns | -99.86% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 76755.4 | 2193213.5 | 2194133.3 | n/a |
| abi_cross_scalar_real_inproc_direct | 10548.7 | 2170145.0 | 2169217.8 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 11060.4 | 2216440.6 | 2215112.9 | n/a |
| abi_cross_scalar_real_null_entry | 27528.2 | 3119.6 | 3065.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.003 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2197723ns | 2197723ns | +1.16% |
| abi_cross_scalar_real_inproc_direct | 2172541ns | 2172541ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2218760ns | 2218760ns | +2.13% |
| abi_cross_scalar_real_null_entry | 5314ns | 5314ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2170373ns | base | --- | [2164835, 2172445] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2189500ns | +22174.1ns (+1.0%) | [+10194, +42378]ns | [2178077, 2214823] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2189461ns | +20625.6ns (+1.0%) | [+14512, +102548]ns | [2181212, 2274665] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 3057ns | -2167338.4ns (-99.9%) | [-2169308, -2161812]ns | [3001, 3137] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2172429ns | +2.5% | +1.1% | -99.9% |
| 2 | 2167512ns | +1.1% | +1.1% | -99.9% |
| 3 | 2171742ns | +0.7% | +0.7% | -99.9% |
| 4 | 2172461ns | +1.4% | +8.3% | -99.9% |
| 5 | 2162158ns | +0.9% | +0.8% | -99.9% |
| 6 | 2169005ns | +0.2% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.000 | ok |
| abi_cross_scalar_real_inproc_direct | -0.287 | moderate- |
| abi_cross_scalar_real_inproc_fnptr | -0.277 | moderate- |
| abi_cross_scalar_real_null_entry | -0.352 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6661154.2ns | 2194133.3ns | 303.6% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6526990.3ns | 2169217.8ns | 300.9% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6653426.6ns | 2215112.9ns | 300.4% | HIGH |
| abi_cross_scalar_real_null_entry | 119772.1ns | 3065.1ns | 3907.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2173912.5-2214822.9 ns)
  2173912.5 |########################################
  2175958.0 |
  2178003.5 |
  2180049.1 |
  2182094.6 |########################################
  2184140.1 |
  2186185.6 |########################################
  2188231.1 |
  2190276.7 |########################################
  2192322.2 |
  2194367.7 |
  2196413.2 |
  2198458.7 |
  2200504.3 |
  2202549.8 |########################################
  2204595.3 |
  2206640.8 |
  2208686.3 |
  2210731.9 |
  2212777.4 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2162157.9-2172445.0 ns)
  2162157.9 |########################################
  2162672.3 |
  2163186.6 |
  2163701.0 |
  2164215.3 |
  2164729.7 |
  2165244.0 |
  2165758.4 |
  2166272.7 |
  2166787.1 |
  2167301.5 |########################################
  2167815.8 |
  2168330.2 |
  2168844.5 |########################################
  2169358.9 |
  2169873.2 |
  2170387.6 |
  2170901.9 |
  2171416.3 |########################################
  2171930.6 |########################################
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2180091.7-2274665.0 ns)
  2180091.7 |########################################
  2184820.4 |####################
  2189549.0 |####################
  2194277.7 |####################
  2199006.4 |
  2203735.0 |
  2208463.7 |
  2213192.4 |
  2217921.0 |
  2222649.7 |
  2227378.4 |
  2232107.0 |
  2236835.7 |
  2241564.3 |
  2246293.0 |
  2251021.7 |
  2255750.3 |
  2260479.0 |
  2265207.7 |
  2269936.3 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2989.6-3137.1 ns)
   2989.6 |####################
   2997.0 |
   3004.3 |
   3011.7 |####################
   3019.1 |
   3026.5 |
   3033.8 |
   3041.2 |
   3048.6 |
   3056.0 |########################################
   3063.3 |
   3070.7 |
   3078.1 |####################
   3085.5 |
   3092.8 |
   3100.2 |
   3107.6 |
   3115.0 |
   3122.3 |
   3129.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3912.2% of algo (FFI overhead may distort results)

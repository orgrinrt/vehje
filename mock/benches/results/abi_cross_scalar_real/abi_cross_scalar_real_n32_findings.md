# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 94706% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.28 us) leads abi_cross_scalar_real_inproc_direct (2.16 ms) by 94706%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 954.3x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.17 ms) is 954.3x the fastest (2.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry shows alternating (throttle bounce) (autocorr -0.56)

abi_cross_scalar_real_null_entry's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (94706% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 94706% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 954.3x the fastest

Fastest abi_cross_scalar_real_null_entry (2.28 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.17 ms): 954.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2277.5 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 954.34x (fastest 2277.5 ns, slowest 2173502.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2173382ns | 2173941ns | 2168065ns | 2172007ns | 2178104ns | +0.39% |
| abi_cross_scalar_real_inproc_direct | 2164844ns | 2162279ns | 2152938ns | 2161765ns | 2175415ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2175718ns | 2176495ns | 2167986ns | 2174799ns | 2180962ns | +0.50% |
| abi_cross_scalar_real_null_entry | 4606ns | 4541ns | 4460ns | 4524ns | 4802ns | -99.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2170060ns | 2164665ns | 2174589ns | +0.38% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2161766ns | 2150099ns | 2172211ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2172660ns | 2164924ns | 2177785ns | +0.50% | 0.000 |
| abi_cross_scalar_real_null_entry | 2307ns | 2221ns | 2415ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 65194.7 | 2170098.2 | 2170060.5 | n/a |
| abi_cross_scalar_real_inproc_direct | 9754.5 | 2163552.7 | 2161766.1 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 9602.1 | 2173601.8 | 2172660.3 | 0 |
| abi_cross_scalar_real_null_entry | 27573.1 | 2445.0 | 2307.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.014 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2173382ns | 2173382ns | +0.39% |
| abi_cross_scalar_real_inproc_direct | 2164844ns | 2164844ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2175718ns | 2175718ns | +0.50% |
| abi_cross_scalar_real_null_entry | 4606ns | 4606ns | -99.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2159212ns | base | --- | [2153875, 2172211] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2170799ns | no significant difference | [-3142, +19035]ns | [2164794, 2174589] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2173502ns | no significant difference | [-490, +23911]ns | [2166693, 2177785] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_null_entry | 2278ns | -2156857.7ns (-99.9%) | [-2169873, -2151645]ns | [2230, 2415] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2150099ns | +1.2% | +1.3% | -99.9% |
| 2 | 2160452ns | +0.6% | +0.4% | -99.9% |
| 3 | 2157972ns | +0.5% | +0.3% | -99.9% |
| 4 | 2157651ns | +0.3% | +0.9% | -99.9% |
| 5 | 2178052ns | -0.6% | -0.4% | -99.9% |
| 6 | 2166371ns | +0.3% | +0.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.262 | moderate+ |
| abi_cross_scalar_real_inproc_direct | 0.096 | ok |
| abi_cross_scalar_real_inproc_fnptr | -0.314 | moderate- |
| abi_cross_scalar_real_null_entry | -0.558 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_real_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6576593.9ns | 2170060.5ns | 303.1% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6503321.2ns | 2161766.1ns | 300.8% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6534569.5ns | 2172660.3ns | 300.8% | HIGH |
| abi_cross_scalar_real_null_entry | 116031.0ns | 2307.4ns | 5028.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2164665.4-2174589.0 ns)
  2164665.4 |########################################
  2165161.6 |
  2165657.8 |
  2166153.9 |
  2166650.1 |
  2167146.3 |
  2167642.5 |
  2168138.7 |
  2168634.8 |####################
  2169131.0 |
  2169627.2 |
  2170123.4 |
  2170619.6 |
  2171115.7 |
  2171611.9 |
  2172108.1 |
  2172604.3 |####################
  2173100.5 |####################
  2173596.6 |
  2174092.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2150098.8-2172211.5 ns)
  2150098.8 |########################################
  2151204.4 |
  2152310.1 |
  2153415.7 |
  2154521.3 |
  2155627.0 |
  2156732.6 |########################################
  2157838.2 |########################################
  2158943.9 |
  2160049.5 |########################################
  2161155.1 |
  2162260.8 |
  2163366.4 |
  2164472.0 |
  2165577.7 |########################################
  2166683.3 |
  2167788.9 |
  2168894.6 |
  2170000.2 |
  2171105.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2164924.2-2177785.4 ns)
  2164924.2 |########################################
  2165567.3 |
  2166210.3 |
  2166853.4 |
  2167496.4 |
  2168139.5 |########################################
  2168782.6 |
  2169425.6 |
  2170068.7 |########################################
  2170711.7 |
  2171354.8 |
  2171997.9 |
  2172640.9 |
  2173284.0 |
  2173927.0 |
  2174570.1 |
  2175213.2 |
  2175856.2 |
  2176499.3 |########################################
  2177142.3 |########################################
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2221.2-2415.0 ns)
   2221.2 |########################################
   2230.9 |########################################
   2240.6 |
   2250.3 |
   2260.0 |
   2269.6 |########################################
   2279.3 |########################################
   2289.0 |
   2298.7 |
   2308.4 |
   2318.1 |
   2327.8 |
   2337.5 |
   2347.2 |
   2356.9 |
   2366.6 |
   2376.2 |
   2385.9 |########################################
   2395.6 |
   2405.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=301.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=5110.6% of algo (FFI overhead may distort results)

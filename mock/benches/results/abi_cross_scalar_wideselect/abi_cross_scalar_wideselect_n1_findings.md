# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 43880% faster than the next best (abi_cross_scalar_wideselect_inproc_fnptr)

abi_cross_scalar_wideselect_null_entry (5.20 us) leads abi_cross_scalar_wideselect_inproc_fnptr (2.29 ms) by 43880%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.33 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_ffi_batched_scalar is an outlier: 450.3x slower than the field

abi_cross_scalar_wideselect_ffi_batched_scalar (2.34 ms) is 450.3x the fastest (5.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar} (43880% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar} with a 43880% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 450.3x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (5.20 us) to slowest abi_cross_scalar_wideselect_ffi_batched_scalar (2.34 ms): 450.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 5196.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 450.32x (fastest 5196.2 ns, slowest 2339968.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2328640ns | 2343968ns | 2167034ns | 2327112ns | 2411735ns | -2.31% |
| abi_cross_scalar_wideselect_inproc_direct | 2383769ns | 2334234ns | 2126145ns | 2312152ns | 2620006ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2363076ns | 2288701ns | 2139482ns | 2249433ns | 2645337ns | -0.87% |
| abi_cross_scalar_wideselect_null_entry | 7641ns | 7608ns | 7221ns | 7501ns | 8061ns | -99.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2324641ns | 2163437ns | 2407504ns | -2.31% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2379638ns | 2122801ns | 2614859ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2359159ns | 2135991ns | 2640501ns | -0.86% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 5214ns | 4940ns | 5502ns | -99.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 81902.3 | 2334941.3 | 2324641.1 | 1 |
| abi_cross_scalar_wideselect_inproc_direct | 12983.5 | 2382065.6 | 2379638.0 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 11994.9 | 2367131.5 | 2359158.7 | n/a |
| abi_cross_scalar_wideselect_null_entry | 32476.6 | 5270.1 | 5213.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_null_entry | 0.000 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2328640ns | 2328640ns | -2.31% |
| abi_cross_scalar_wideselect_inproc_direct | 2383769ns | 2383769ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2363076ns | 2363076ns | -0.87% |
| abi_cross_scalar_wideselect_null_entry | 7641ns | 7641ns | -99.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2330404ns | base | --- | [2193652, 2614859] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2339968ns | no significant difference | [-210255, +32799]ns | [2226451, 2407504] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2285290ns | no significant difference | [-370399, +282272]ns | [2151685, 2640501] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_null_entry | 5196ns | -2325189.6ns (-99.8%) | [-2609628, -2188456]ns | [4943, 5502] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2122801ns | +1.9% | +2.1% | -99.8% |
| 2 | 2827097ns | -15.1% | -24.4% | -99.8% |
| 3 | 2402620ns | +0.5% | +9.4% | -99.8% |
| 4 | 2313838ns | +0.5% | +14.6% | -99.8% |
| 5 | 2346970ns | +0.3% | -2.1% | -99.8% |
| 6 | 2264502ns | +1.1% | +0.4% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.147 | ok |
| abi_cross_scalar_wideselect_inproc_direct | -0.351 | moderate- |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.190 | ok |
| abi_cross_scalar_wideselect_null_entry | -0.346 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 7078718.4ns | 2324641.1ns | 304.5% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 7181468.0ns | 2379638.0ns | 301.8% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 7105939.7ns | 2359158.7ns | 301.2% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 130328.9ns | 5213.5ns | 2499.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2163437.1-2407504.3 ns)
  2163437.1 |########################################
  2175640.5 |
  2187843.8 |
  2200047.2 |
  2212250.5 |
  2224453.9 |
  2236657.3 |
  2248860.6 |
  2261064.0 |
  2273267.4 |
  2285470.7 |########################################
  2297674.1 |
  2309877.4 |
  2322080.8 |########################################
  2334284.2 |
  2346487.5 |########################################
  2358690.9 |
  2370894.3 |
  2383097.6 |
  2395301.0 |########################################
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2122801.2-2614858.5 ns)
  2122801.2 |########################################
  2147404.1 |
  2172006.9 |
  2196609.8 |
  2221212.7 |
  2245815.5 |########################################
  2270418.4 |
  2295021.3 |########################################
  2319624.1 |
  2344227.0 |########################################
  2368829.9 |
  2393432.7 |########################################
  2418035.6 |
  2442638.5 |
  2467241.3 |
  2491844.2 |
  2516447.1 |
  2541049.9 |
  2565652.8 |
  2590255.7 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2135991.2-2640500.6 ns)
  2135991.2 |########################################
  2161216.7 |########################################
  2186442.1 |
  2211667.6 |
  2236893.1 |
  2262118.5 |########################################
  2287344.0 |########################################
  2312569.5 |
  2337795.0 |
  2363020.4 |
  2388245.9 |
  2413471.4 |
  2438696.8 |
  2463922.3 |
  2489147.8 |
  2514373.2 |
  2539598.7 |
  2564824.2 |
  2590049.7 |
  2615275.1 |########################################
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 4940.0-5501.6 ns)
   4940.0 |########################################
   4968.1 |
   4996.2 |####################
   5024.2 |
   5052.3 |
   5080.4 |
   5108.5 |
   5136.6 |
   5164.7 |
   5192.7 |
   5220.8 |
   5248.9 |
   5277.0 |
   5305.1 |
   5333.2 |
   5361.2 |####################
   5389.3 |
   5417.4 |
   5445.5 |
   5473.6 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=305.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=2506.3% of algo (FFI overhead may distort results)

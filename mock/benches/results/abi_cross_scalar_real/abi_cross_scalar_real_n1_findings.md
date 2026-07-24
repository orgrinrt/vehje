# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 43733% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (5.22 us) leads abi_cross_scalar_real_inproc_direct (2.29 ms) by 43733%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.28 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 457.7x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.39 ms) is 457.7x the fastest (5.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_inproc_fnptr shows alternating (throttle bounce) (autocorr -0.55)

abi_cross_scalar_real_inproc_fnptr's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (43733% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 43733% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 457.7x the fastest

Fastest abi_cross_scalar_real_null_entry (5.22 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.39 ms): 457.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 5217.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 457.66x (fastest 5217.7 ns, slowest 2387942.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2451902ns | 2383418ns | 2310827ns | 2380145ns | 2630075ns | +5.12% |
| abi_cross_scalar_real_inproc_direct | 2332449ns | 2291544ns | 2199801ns | 2272603ns | 2488542ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2414660ns | 2393137ns | 2319006ns | 2381684ns | 2511952ns | +3.52% |
| abi_cross_scalar_real_null_entry | 7643ns | 7690ns | 6995ns | 7615ns | 8009ns | -99.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2446703ns | 2306055ns | 2624455ns | +5.10% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2327935ns | 2195541ns | 2483698ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2409491ns | 2314182ns | 2506422ns | +3.50% | 0.000 |
| abi_cross_scalar_real_null_entry | 5198ns | 4783ns | 5438ns | -99.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 112234.0 | 2430073.2 | 2446702.6 | n/a |
| abi_cross_scalar_real_inproc_direct | 14053.6 | 2336646.2 | 2327934.6 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 14312.1 | 2407583.0 | 2409490.8 | n/a |
| abi_cross_scalar_real_null_entry | 32211.4 | 5283.5 | 5198.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_real_null_entry | 0.000 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2451902ns | 2451902ns | +5.12% |
| abi_cross_scalar_real_inproc_direct | 2332449ns | 2332449ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2414660ns | 2414660ns | +3.52% |
| abi_cross_scalar_real_null_entry | 7643ns | 7643ns | -99.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2287075ns | base | --- | [2213031, 2483698] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2378211ns | +113454.8ns (+5.0%) | [+28288, +214561]ns | [2337443, 2624455] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2387943ns | no significant difference | [-29141, +185746]ns | [2334108, 2506422] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_null_entry | 5218ns | -2281857.8ns (-99.8%) | [-2478644, -2207707]ns | [4939, 5438] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2264579ns | +1.8% | +8.5% | -99.8% |
| 2 | 2353730ns | +0.6% | +0.0% | -99.8% |
| 3 | 2613665ns | +5.8% | -2.2% | -99.8% |
| 4 | 2309572ns | +3.3% | +0.2% | -99.8% |
| 5 | 2195541ns | +8.0% | +7.8% | -99.8% |
| 6 | 2230521ns | +11.4% | +8.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.228 | moderate- |
| abi_cross_scalar_real_inproc_direct | 0.139 | ok |
| abi_cross_scalar_real_inproc_fnptr | -0.548 | HIGH- (thermal bounce) |
| abi_cross_scalar_real_null_entry | -0.404 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 1/6, lost 4/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 7429390.7ns | 2446702.6ns | 303.6% | HIGH |
| abi_cross_scalar_real_inproc_direct | 7008299.3ns | 2327934.6ns | 301.1% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 7251983.6ns | 2409490.8ns | 301.0% | HIGH |
| abi_cross_scalar_real_null_entry | 129953.2ns | 5198.4ns | 2499.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2306055.0-2624454.6 ns)
  2306055.0 |####################
  2321975.0 |
  2337895.0 |
  2353814.9 |####################
  2369734.9 |########################################
  2385654.9 |
  2401574.9 |
  2417494.9 |
  2433414.8 |
  2449334.8 |
  2465254.8 |
  2481174.8 |####################
  2497094.8 |
  2513014.7 |
  2528934.7 |
  2544854.7 |
  2560774.7 |
  2576694.7 |
  2592614.6 |
  2608534.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2195540.8-2483697.5 ns)
  2195540.8 |########################################
  2209948.6 |
  2224356.5 |########################################
  2238764.3 |
  2253172.1 |########################################
  2267580.0 |
  2281987.8 |
  2296395.6 |########################################
  2310803.5 |
  2325211.3 |
  2339619.1 |########################################
  2354027.0 |
  2368434.8 |
  2382842.7 |
  2397250.5 |
  2411658.3 |
  2426066.2 |
  2440474.0 |
  2454881.8 |
  2469289.7 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2314182.1-2506422.1 ns)
  2314182.1 |########################################
  2323794.1 |
  2333406.1 |
  2343018.1 |
  2352630.1 |########################################
  2362242.1 |########################################
  2371854.1 |
  2381466.1 |
  2391078.1 |
  2400690.1 |########################################
  2410302.1 |
  2419914.1 |
  2429526.1 |
  2439138.1 |
  2448750.1 |########################################
  2458362.1 |
  2467974.1 |
  2477586.1 |
  2487198.1 |
  2496810.1 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 4783.3-5438.1 ns)
   4783.3 |########################################
   4816.0 |
   4848.8 |
   4881.5 |
   4914.3 |
   4947.0 |
   4979.8 |
   5012.5 |
   5045.2 |
   5078.0 |########################################
   5110.7 |
   5143.5 |########################################
   5176.2 |
   5209.0 |
   5241.7 |
   5274.4 |########################################
   5307.2 |########################################
   5339.9 |
   5372.7 |
   5405.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=299.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=2490.1% of algo (FFI overhead may distort results)

# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 68878% faster than the next best (abi_cross_scalar_real_ffi_batched_scalar)

abi_cross_scalar_real_null_entry (3.44 us) leads abi_cross_scalar_real_ffi_batched_scalar (2.37 ms) by 68878%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.39 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 702.7x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.42 ms) is 702.7x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr} (68878% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr} with a 68878% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 702.7x the fastest

Fastest abi_cross_scalar_real_null_entry (3.44 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.42 ms): 702.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 3441.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 702.69x (fastest 3441.8 ns, slowest 2418544.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2397350ns | 2379483ns | 2324045ns | 2369007ns | 2476516ns | -3.59% |
| abi_cross_scalar_real_inproc_direct | 2486738ns | 2393914ns | 2253810ns | 2376883ns | 2767985ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2415230ns | 2423790ns | 2306022ns | 2414200ns | 2471380ns | -2.88% |
| abi_cross_scalar_real_null_entry | 6032ns | 5875ns | 5642ns | 5836ns | 6523ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2392430ns | 2319958ns | 2471590ns | -3.59% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2481498ns | 2249859ns | 2761592ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2410519ns | 2302442ns | 2466583ns | -2.86% | 0.000 |
| abi_cross_scalar_real_null_entry | 3489ns | 3241ns | 3732ns | -99.86% | 0.073 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 111826.3 | 2397371.8 | 2392430.0 | 1 |
| abi_cross_scalar_real_inproc_direct | 15256.6 | 2484273.2 | 2481497.7 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 14130.9 | 2404832.6 | 2410519.1 | n/a |
| abi_cross_scalar_real_null_entry | 35927.6 | 3501.7 | 3489.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.079 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.074 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2397350ns | 2397350ns | -3.59% |
| abi_cross_scalar_real_inproc_direct | 2486738ns | 2486738ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2415230ns | 2415230ns | -2.88% |
| abi_cross_scalar_real_null_entry | 6032ns | 6032ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2388856ns | base | --- | [2294044, 2761592] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2374133ns | no significant difference | [-351074, +67277]ns | [2331567, 2471590] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2418544ns | no significant difference | [-335156, +74764]ns | [2346430, 2466583] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_null_entry | 3442ns | -2385432.8ns (-99.9%) | [-2757860, -2290733]ns | [3294, 3732] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2249859ns | +3.1% | +2.3% | -99.8% |
| 2 | 2338230ns | +0.2% | +3.6% | -99.9% |
| 3 | 2348088ns | +1.2% | +1.8% | -99.9% |
| 4 | 2914980ns | -18.6% | -17.2% | -99.9% |
| 5 | 2429625ns | +2.7% | +2.6% | -99.9% |
| 6 | 2608205ns | -6.1% | -6.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.382 | moderate+ |
| abi_cross_scalar_real_inproc_direct | -0.116 | ok |
| abi_cross_scalar_real_inproc_fnptr | 0.046 | ok |
| abi_cross_scalar_real_null_entry | -0.148 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_real_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 7327816.1ns | 2392430.0ns | 306.3% | HIGH |
| abi_cross_scalar_real_inproc_direct | 7521616.0ns | 2481497.7ns | 303.1% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 7253653.7ns | 2410519.1ns | 300.9% | HIGH |
| abi_cross_scalar_real_null_entry | 130575.6ns | 3489.3ns | 3742.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2319957.9-2471590.2 ns)
  2319957.9 |########################################
  2327539.5 |
  2335121.1 |
  2342702.7 |########################################
  2350284.4 |
  2357866.0 |
  2365447.6 |########################################
  2373029.2 |########################################
  2380610.8 |
  2388192.4 |
  2395774.0 |
  2403355.7 |
  2410937.3 |
  2418518.9 |
  2426100.5 |
  2433682.1 |
  2441263.7 |
  2448845.4 |########################################
  2456427.0 |
  2464008.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2249858.8-2761592.3 ns)
  2249858.8 |####################
  2275445.5 |
  2301032.1 |
  2326618.8 |########################################
  2352205.5 |
  2377792.2 |
  2403378.8 |
  2428965.5 |####################
  2454552.2 |
  2480138.9 |
  2505725.5 |
  2531312.2 |
  2556898.9 |
  2582485.6 |
  2608072.2 |####################
  2633658.9 |
  2659245.6 |
  2684832.3 |
  2710418.9 |
  2736005.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2302442.5-2466583.3 ns)
  2302442.5 |########################################
  2310649.5 |
  2318856.6 |
  2327063.6 |
  2335270.7 |
  2343477.7 |
  2351684.7 |
  2359891.8 |
  2368098.8 |
  2376305.9 |
  2384512.9 |########################################
  2392719.9 |
  2400927.0 |
  2409134.0 |########################################
  2417341.1 |########################################
  2425548.1 |
  2433755.1 |########################################
  2441962.2 |
  2450169.2 |
  2458376.3 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 3241.2-3732.5 ns)
   3241.2 |########################################
   3265.8 |
   3290.3 |
   3314.9 |
   3339.5 |########################################
   3364.0 |########################################
   3388.6 |
   3413.2 |
   3437.7 |
   3462.3 |
   3486.8 |########################################
   3511.4 |
   3536.0 |########################################
   3560.5 |
   3585.1 |
   3609.7 |
   3634.2 |
   3658.8 |
   3683.4 |
   3707.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=307.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=299.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3670.8% of algo (FFI overhead may distort results)

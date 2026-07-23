# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 82600% faster than the next best (abi_cross_scalar_real_inproc_fnptr)

abi_cross_scalar_real_null_entry (2.79 us) leads abi_cross_scalar_real_inproc_fnptr (2.31 ms) by 82600%, a clear separation rather than a photo finish. CV 20.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.31 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 839.1x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.34 ms) is 839.1x the fastest (2.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry is fastest but the noisiest (CV 20.4%)

abi_cross_scalar_real_null_entry wins on median (2.79 us) yet has the highest variance (CV 20.4%), while abi_cross_scalar_real_inproc_direct is the steadiest (CV 5.8%, 2.32 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_cross_scalar_real_inproc_direct shows alternating (throttle bounce) (autocorr -0.63)

abi_cross_scalar_real_inproc_direct's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar} (82600% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar} with a 82600% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 839.1x the fastest

Fastest abi_cross_scalar_real_null_entry (2.79 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.34 ms): 839.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2790.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 839.06x (fastest 2790.4 ns, slowest 2341314.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2385017ns | 2346268ns | 2233904ns | 2329199ns | 2544300ns | +1.15% |
| abi_cross_scalar_real_inproc_direct | 2357851ns | 2322023ns | 2236802ns | 2298602ns | 2507249ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2369525ns | 2312559ns | 2238185ns | 2298677ns | 2541468ns | +0.50% |
| abi_cross_scalar_real_null_entry | 5949ns | 5732ns | 4836ns | 5465ns | 7232ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2379697ns | 2229527ns | 2538008ns | +1.13% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2353069ns | 2232407ns | 2501738ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2363279ns | 2234039ns | 2532179ns | +0.43% | 0.000 |
| abi_cross_scalar_real_null_entry | 2990ns | 2497ns | 3631ns | -99.87% | 0.021 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 115879.9 | 2376398.8 | 2379696.9 | n/a |
| abi_cross_scalar_real_inproc_direct | 14204.4 | 2359874.9 | 2353068.8 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 14477.1 | 2358103.1 | 2363278.6 | n/a |
| abi_cross_scalar_real_null_entry | 43141.0 | 3203.4 | 2989.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.023 | 89.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2385017ns | 2385017ns | +1.15% |
| abi_cross_scalar_real_inproc_direct | 2357851ns | 2357851ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2369525ns | 2369525ns | +0.50% |
| abi_cross_scalar_real_null_entry | 5949ns | 5949ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2317332ns | base | --- | [2240137, 2501738] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2341315ns | no significant difference | [-117216, +173118]ns | [2259768, 2538008] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2307671ns | no significant difference | [-226192, +286124]ns | [2249986, 2532179] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_null_entry | 2790ns | -2314495.1ns (-99.9%) | [-2498307, -2237434]ns | [2548, 3631] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2247867ns | -0.8% | -0.6% | -99.9% |
| 2 | 2621404ns | -8.2% | -13.6% | -99.9% |
| 3 | 2232407ns | +2.6% | +20.0% | -99.9% |
| 4 | 2374961ns | +0.6% | -4.1% | -99.9% |
| 5 | 2259703ns | +1.5% | +5.6% | -99.9% |
| 6 | 2382071ns | +12.1% | -1.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.263 | moderate- |
| abi_cross_scalar_real_inproc_direct | -0.632 | HIGH- (thermal bounce) |
| abi_cross_scalar_real_inproc_fnptr | -0.355 | moderate- |
| abi_cross_scalar_real_null_entry | -0.167 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_real_inproc_fnptr**: won 4/6, lost 2/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 7231966.1ns | 2379696.9ns | 303.9% | HIGH |
| abi_cross_scalar_real_inproc_direct | 7102227.2ns | 2353068.8ns | 301.8% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 7079094.0ns | 2363278.6ns | 299.5% | HIGH |
| abi_cross_scalar_real_null_entry | 133725.7ns | 2989.9ns | 4472.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2229526.7-2538008.5 ns)
  2229526.7 |########################################
  2244950.8 |
  2260374.9 |
  2275799.0 |########################################
  2291223.1 |########################################
  2306647.2 |
  2322071.2 |
  2337495.3 |
  2352919.4 |
  2368343.5 |
  2383767.6 |########################################
  2399191.7 |########################################
  2414615.8 |
  2430039.9 |
  2445464.0 |
  2460888.0 |
  2476312.1 |
  2491736.2 |
  2507160.3 |
  2522584.4 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2232407.1-2501737.5 ns)
  2232407.1 |########################################
  2245873.6 |########################################
  2259340.1 |########################################
  2272806.7 |
  2286273.2 |
  2299739.7 |
  2313206.2 |
  2326672.7 |
  2340139.3 |
  2353605.8 |
  2367072.3 |########################################
  2380538.8 |########################################
  2394005.3 |
  2407471.9 |
  2420938.4 |
  2434404.9 |
  2447871.4 |
  2461337.9 |
  2474804.5 |
  2488271.0 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2234039.2-2532178.5 ns)
  2234039.2 |####################
  2248946.2 |
  2263853.1 |########################################
  2278760.1 |
  2293667.1 |
  2308574.0 |
  2323481.0 |####################
  2338388.0 |
  2353294.9 |
  2368201.9 |
  2383108.9 |####################
  2398015.8 |
  2412922.8 |
  2427829.8 |
  2442736.7 |
  2457643.7 |
  2472550.7 |
  2487457.6 |
  2502364.6 |
  2517271.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2497.1-3631.0 ns)
   2497.1 |########################################
   2553.8 |########################################
   2610.5 |
   2667.2 |########################################
   2723.9 |
   2780.6 |
   2837.3 |
   2894.0 |########################################
   2950.7 |
   3007.4 |
   3064.1 |########################################
   3120.8 |
   3177.5 |
   3234.2 |
   3290.9 |
   3347.6 |
   3404.3 |
   3461.0 |
   3517.7 |
   3574.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=306.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=299.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=4569.9% of algo (FFI overhead may distort results)

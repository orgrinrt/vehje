# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_tight_null_entry dominates: 84355% faster than the next best (abi_cross_scalar_tight_inproc_direct)

abi_cross_scalar_tight_null_entry (2.70 us) leads abi_cross_scalar_tight_inproc_direct (2.28 ms) by 84355%, a clear separation rather than a photo finish. CV 7.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.28 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_fnptr is an outlier: 894.5x slower than the field

abi_cross_scalar_tight_inproc_fnptr (2.41 ms) is 894.5x the fastest (2.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr} (84355% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr} with a 84355% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 894.5x the fastest

Fastest abi_cross_scalar_tight_null_entry (2.70 us) to slowest abi_cross_scalar_tight_inproc_fnptr (2.41 ms): 894.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 2697.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 894.51x (fastest 2697.9 ns, slowest 2413334.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2379319ns | 2376085ns | 2127435ns | 2339646ns | 2564771ns | +4.91% |
| abi_cross_scalar_tight_inproc_direct | 2267994ns | 2283185ns | 2050448ns | 2229895ns | 2433917ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2503380ns | 2418775ns | 2076068ns | 2347140ns | 2951396ns | +10.38% |
| abi_cross_scalar_tight_null_entry | 5380ns | 5179ns | 4871ns | 5129ns | 6012ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2374016ns | 2122996ns | 2558758ns | +4.88% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2263534ns | 2046611ns | 2428893ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2498228ns | 2072479ns | 2945348ns | +10.37% | 0.000 |
| abi_cross_scalar_tight_null_entry | 2753ns | 2536ns | 3019ns | -99.88% | 0.023 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 109066.7 | 2407509.6 | 2374016.0 | n/a |
| abi_cross_scalar_tight_inproc_direct | 10643.5 | 2279628.1 | 2263534.4 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 11739.7 | 2520898.8 | 2498227.6 | n/a |
| abi_cross_scalar_tight_null_entry | 33998.0 | 3201.5 | 2752.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.025 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_tight_null_entry | 0.024 | 94.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2379319ns | 2379319ns | +4.91% |
| abi_cross_scalar_tight_inproc_direct | 2267994ns | 2267994ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2503380ns | 2503380ns | +10.38% |
| abi_cross_scalar_tight_null_entry | 5380ns | 5380ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2278562ns | base | --- | [2083148, 2428893] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2370720ns | no significant difference | [-110776, +394275]ns | [2192570, 2558758] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2413334ns | no significant difference | [-65019, +678539]ns | [2136000, 2945348] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_tight_null_entry | 2698ns | -2275723.5ns (-99.9%) | [-2426066, -2080555]ns | [2541, 3019] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2119685ns | +0.2% | -2.2% | -99.9% |
| 2 | 2282354ns | +13.5% | -3.6% | -99.9% |
| 3 | 2046611ns | +23.4% | +52.3% | -99.9% |
| 4 | 2487007ns | -9.0% | +11.5% | -99.9% |
| 5 | 2274770ns | +2.3% | +5.2% | -99.9% |
| 6 | 2370779ns | +1.8% | +2.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | -0.235 | moderate- |
| abi_cross_scalar_tight_inproc_direct | -0.398 | moderate- |
| abi_cross_scalar_tight_inproc_fnptr | 0.122 | ok |
| abi_cross_scalar_tight_null_entry | 0.283 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 7694261.0ns | 2374016.0ns | 324.1% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6812811.9ns | 2263534.4ns | 301.0% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 7639611.9ns | 2498227.6ns | 305.8% | HIGH |
| abi_cross_scalar_tight_null_entry | 121890.0ns | 2752.6ns | 4428.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2122996.2-2558757.8 ns)
  2122996.2 |########################################
  2144784.3 |
  2166572.4 |
  2188360.4 |
  2210148.5 |
  2231936.6 |
  2253724.7 |########################################
  2275512.7 |
  2297300.8 |
  2319088.9 |########################################
  2340877.0 |
  2362665.1 |
  2384453.1 |
  2406241.2 |########################################
  2428029.3 |
  2449817.4 |
  2471605.4 |
  2493393.5 |
  2515181.6 |########################################
  2536969.7 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2046610.8-2428893.0 ns)
  2046610.8 |########################################
  2065724.9 |
  2084839.0 |
  2103953.1 |########################################
  2123067.2 |
  2142181.3 |
  2161295.4 |
  2180409.6 |
  2199523.7 |
  2218637.8 |
  2237751.9 |
  2256866.0 |########################################
  2275980.1 |########################################
  2295094.2 |
  2314208.3 |
  2333322.4 |
  2352436.5 |########################################
  2371550.6 |
  2390664.7 |
  2409778.8 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2072479.2-2945348.2 ns)
  2072479.2 |########################################
  2116122.6 |
  2159766.1 |########################################
  2203409.5 |
  2247053.0 |
  2290696.4 |
  2334339.9 |
  2377983.3 |########################################
  2421626.8 |########################################
  2465270.2 |
  2508913.7 |
  2552557.1 |
  2596200.6 |
  2639844.0 |
  2683487.5 |
  2727130.9 |
  2770774.4 |########################################
  2814417.8 |
  2858061.3 |
  2901704.7 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 2536.2-3019.4 ns)
   2536.2 |########################################
   2560.4 |
   2584.5 |
   2608.7 |
   2632.8 |####################
   2657.0 |
   2681.1 |
   2705.3 |
   2729.5 |####################
   2753.6 |
   2777.8 |
   2801.9 |
   2826.1 |
   2850.2 |
   2874.4 |
   2898.6 |
   2922.7 |####################
   2946.9 |
   2971.0 |
   2995.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=305.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=4472.2% of algo (FFI overhead may distort results)

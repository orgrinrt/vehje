# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_tight_null_entry dominates: 67744% faster than the next best (abi_cross_scalar_tight_inproc_direct)

abi_cross_scalar_tight_null_entry (3.19 us) leads abi_cross_scalar_tight_inproc_direct (2.16 ms) by 67744%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_ffi_batched_scalar is an outlier: 685.4x slower than the field

abi_cross_scalar_tight_ffi_batched_scalar (2.19 ms) is 685.4x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_ffi_batched_scalar} (67744% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_ffi_batched_scalar} with a 67744% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 685.4x the fastest

Fastest abi_cross_scalar_tight_null_entry (3.19 us) to slowest abi_cross_scalar_tight_ffi_batched_scalar (2.19 ms): 685.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 3188.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 685.38x (fastest 3188.9 ns, slowest 2185643.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2255142ns | 2190626ns | 2084208ns | 2159435ns | 2484170ns | -1.45% |
| abi_cross_scalar_tight_inproc_direct | 2288325ns | 2167767ns | 2053089ns | 2143439ns | 2623273ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2190616ns | 2177260ns | 2056969ns | 2145146ns | 2325643ns | -4.27% |
| abi_cross_scalar_tight_null_entry | 5737ns | 5620ns | 5350ns | 5562ns | 6194ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2250375ns | 2080098ns | 2478981ns | -1.47% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2283940ns | 2049516ns | 2618197ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2186115ns | 2053025ns | 2320305ns | -4.28% | 0.000 |
| abi_cross_scalar_tight_null_entry | 3248ns | 3083ns | 3459ns | -99.86% | 0.002 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 96637.6 | 2245233.4 | 2250375.1 | 3 |
| abi_cross_scalar_tight_inproc_direct | 10220.3 | 2234958.7 | 2283940.0 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 9991.7 | 2193953.6 | 2186115.5 | n/a |
| abi_cross_scalar_tight_null_entry | 31479.4 | 3317.4 | 3247.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_tight_null_entry | 0.003 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2255142ns | 2255142ns | -1.45% |
| abi_cross_scalar_tight_inproc_direct | 2288325ns | 2288325ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2190616ns | 2190616ns | -4.27% |
| abi_cross_scalar_tight_null_entry | 5737ns | 5737ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2163511ns | base | --- | [2070112, 2618197] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2185643ns | no significant difference | [-192393, +82217]ns | [2086501, 2478981] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2172973ns | no significant difference | [-340238, +41229]ns | [2065069, 2320305] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_null_entry | 3189ns | -2160380.5ns (-99.9%) | [-2614738, -2066959]ns | [3095, 3459] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2187057ns | -2.0% | +1.6% | -99.9% |
| 2 | 2139965ns | +5.7% | -0.8% | -99.9% |
| 3 | 3037473ns | -11.2% | -21.2% | -99.9% |
| 4 | 2049516ns | +2.1% | +1.3% | -99.8% |
| 5 | 2090709ns | -0.5% | -1.8% | -99.8% |
| 6 | 2198921ns | +1.3% | +2.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | -0.136 | ok |
| abi_cross_scalar_tight_inproc_direct | -0.300 | moderate- |
| abi_cross_scalar_tight_inproc_fnptr | -0.385 | moderate- |
| abi_cross_scalar_tight_null_entry | -0.408 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 6920302.8ns | 2250375.1ns | 307.5% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6744078.0ns | 2283940.0ns | 295.3% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 6591097.6ns | 2186115.5ns | 301.5% | HIGH |
| abi_cross_scalar_tight_null_entry | 124390.2ns | 3247.6ns | 3830.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2080097.5-2478980.8 ns)
  2080097.5 |########################################
  2100041.7 |
  2119985.8 |
  2139930.0 |####################
  2159874.2 |
  2179818.3 |
  2199762.5 |
  2219706.7 |####################
  2239650.8 |
  2259595.0 |####################
  2279539.1 |
  2299483.3 |
  2319427.5 |
  2339371.6 |
  2359315.8 |
  2379260.0 |
  2399204.1 |
  2419148.3 |
  2439092.5 |
  2459036.6 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2049516.2-2618196.8 ns)
  2049516.2 |########################################
  2077950.2 |########################################
  2106384.3 |
  2134818.3 |########################################
  2163252.3 |########################################
  2191686.4 |########################################
  2220120.4 |
  2248554.4 |
  2276988.5 |
  2305422.5 |
  2333856.5 |
  2362290.6 |
  2390724.6 |
  2419158.6 |
  2447592.7 |
  2476026.7 |
  2504460.7 |
  2532894.8 |
  2561328.8 |
  2589762.8 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2053025.0-2320305.0 ns)
  2053025.0 |########################################
  2066389.0 |########################################
  2079753.0 |
  2093117.0 |
  2106481.0 |
  2119845.0 |########################################
  2133209.0 |
  2146573.0 |
  2159937.0 |
  2173301.0 |
  2186665.0 |
  2200029.0 |
  2213393.0 |########################################
  2226757.0 |
  2240121.0 |########################################
  2253485.0 |
  2266849.0 |
  2280213.0 |
  2293577.0 |
  2306941.0 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 3082.9-3458.6 ns)
   3082.9 |########################################
   3101.7 |########################################
   3120.5 |
   3139.2 |########################################
   3158.0 |
   3176.8 |
   3195.6 |
   3214.4 |########################################
   3233.2 |
   3251.9 |
   3270.7 |########################################
   3289.5 |
   3308.3 |
   3327.1 |
   3345.9 |
   3364.6 |
   3383.4 |
   3402.2 |
   3421.0 |
   3439.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=299.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=3914.1% of algo (FFI overhead may distort results)

# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 87028% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.48 us) leads abi_cross_scalar_real_inproc_direct (2.16 ms) by 87028%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 881.4x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.19 ms) is 881.4x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_inproc_fnptr shows alternating (throttle bounce) (autocorr -0.52)

abi_cross_scalar_real_inproc_fnptr's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (87028% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 87028% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 881.4x the fastest

Fastest abi_cross_scalar_real_null_entry (2.48 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.19 ms): 881.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2482.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 881.40x (fastest 2482.1 ns, slowest 2187713.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2314553ns | 2191445ns | 2181975ns | 2190124ns | 2567486ns | +6.01% |
| abi_cross_scalar_real_inproc_direct | 2183326ns | 2165640ns | 2164134ns | 2165338ns | 2219905ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2217208ns | 2187240ns | 2178319ns | 2184550ns | 2285641ns | +1.55% |
| abi_cross_scalar_real_null_entry | 4796ns | 4792ns | 4709ns | 4769ns | 4879ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2310716ns | 2178460ns | 2563154ns | +5.99% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2180043ns | 2161056ns | 2216291ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2213886ns | 2175405ns | 2281846ns | +1.55% | 0.000 |
| abi_cross_scalar_real_null_entry | 2497ns | 2455ns | 2546ns | -99.89% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 79507.8 | 2309000.3 | 2310716.4 | n/a |
| abi_cross_scalar_real_inproc_direct | 11087.9 | 2174931.0 | 2180042.9 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10749.1 | 2219077.8 | 2213885.7 | 0 |
| abi_cross_scalar_real_null_entry | 29039.7 | 2754.4 | 2496.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.026 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2314553ns | 2314553ns | +6.01% |
| abi_cross_scalar_real_inproc_direct | 2183326ns | 2183326ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2217208ns | 2217208ns | +1.55% |
| abi_cross_scalar_real_null_entry | 4796ns | 4796ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2162608ns | base | --- | [2161229, 2216291] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2187714ns | +23845.0ns (+1.1%) | [+15676, +352500]ns | [2181281, 2563154] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2184004ns | no significant difference | [-32288, +119238]ns | [2175807, 2281846] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_null_entry | 2482ns | -2160081.5ns (-99.9%) | [-2213791, -2158765]ns | [2463, 2546] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2161056ns | +1.1% | +0.7% | -99.9% |
| 2 | 2172934ns | +0.7% | +0.6% | -99.9% |
| 3 | 2161660ns | +8.6% | +4.1% | -99.9% |
| 4 | 2161402ns | +0.8% | +0.6% | -99.9% |
| 5 | 2163557ns | +1.1% | +6.9% | -99.9% |
| 6 | 2259649ns | +23.0% | -3.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.123 | ok |
| abi_cross_scalar_real_inproc_direct | -0.051 | ok |
| abi_cross_scalar_real_inproc_fnptr | -0.525 | HIGH- (thermal bounce) |
| abi_cross_scalar_real_null_entry | -0.027 | ok |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 7048419.8ns | 2310716.4ns | 305.0% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6535053.9ns | 2180042.9ns | 299.8% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6684198.2ns | 2213885.7ns | 301.9% | HIGH |
| abi_cross_scalar_real_null_entry | 114025.9ns | 2496.8ns | 4566.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2178460.4-2563154.0 ns)
  2178460.4 |########################################
  2197695.1 |
  2216929.8 |
  2236164.4 |
  2255399.1 |
  2274633.8 |
  2293868.5 |
  2313103.1 |
  2332337.8 |##########
  2351572.5 |
  2370807.2 |
  2390041.9 |
  2409276.5 |
  2428511.2 |
  2447745.9 |
  2466980.6 |
  2486215.2 |
  2505449.9 |
  2524684.6 |
  2543919.3 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2161055.8-2216291.3 ns)
  2161055.8 |########################################
  2163817.6 |
  2166579.3 |
  2169341.1 |
  2172102.9 |##########
  2174864.7 |
  2177626.4 |
  2180388.2 |
  2183150.0 |
  2185911.8 |
  2188673.5 |
  2191435.3 |
  2194197.1 |
  2196958.9 |
  2199720.6 |
  2202482.4 |
  2205244.2 |
  2208006.0 |
  2210767.8 |
  2213529.5 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2175404.6-2281846.2 ns)
  2175404.6 |########################################
  2180726.7 |########################################
  2186048.8 |
  2191370.8 |
  2196692.9 |
  2202015.0 |
  2207337.1 |
  2212659.2 |
  2217981.3 |
  2223303.3 |
  2228625.4 |
  2233947.5 |
  2239269.6 |
  2244591.7 |
  2249913.8 |####################
  2255235.8 |
  2260557.9 |
  2265880.0 |
  2271202.1 |
  2276524.2 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2455.4-2545.6 ns)
   2455.4 |####################
   2459.9 |
   2464.4 |
   2468.9 |########################################
   2473.4 |
   2477.9 |
   2482.5 |
   2487.0 |
   2491.5 |####################
   2496.0 |
   2500.5 |
   2505.0 |####################
   2509.5 |
   2514.0 |
   2518.5 |
   2523.1 |
   2527.6 |
   2532.1 |
   2536.6 |
   2541.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=4581.2% of algo (FFI overhead may distort results)

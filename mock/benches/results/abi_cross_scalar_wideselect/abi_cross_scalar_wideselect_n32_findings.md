# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 92064% faster than the next best (abi_cross_scalar_wideselect_inproc_direct)

abi_cross_scalar_wideselect_null_entry (2.32 us) leads abi_cross_scalar_wideselect_inproc_direct (2.14 ms) by 92064%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.14 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_ffi_batched_scalar is an outlier: 925.9x slower than the field

abi_cross_scalar_wideselect_ffi_batched_scalar (2.15 ms) is 925.9x the fastest (2.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_ffi_batched_scalar} (92064% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_ffi_batched_scalar} with a 92064% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 925.9x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (2.32 us) to slowest abi_cross_scalar_wideselect_ffi_batched_scalar (2.15 ms): 925.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 2324.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 925.86x (fastest 2324.8 ns, slowest 2152429.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2197681ns | 2156640ns | 2144522ns | 2153061ns | 2291191ns | +0.59% |
| abi_cross_scalar_wideselect_inproc_direct | 2184866ns | 2146411ns | 2128332ns | 2141701ns | 2277880ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2284782ns | 2148366ns | 2138820ns | 2146627ns | 2564995ns | +4.57% |
| abi_cross_scalar_wideselect_null_entry | 4673ns | 4654ns | 4467ns | 4638ns | 4827ns | -99.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2193450ns | 2140497ns | 2286630ns | +0.57% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2180913ns | 2124503ns | 2273449ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2280608ns | 2135328ns | 2559893ns | +4.57% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 2339ns | 2256ns | 2419ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 93868.5 | 2220743.6 | 2193449.5 | n/a |
| abi_cross_scalar_wideselect_inproc_direct | 12193.9 | 2185681.4 | 2180912.8 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 12109.1 | 2208241.7 | 2280608.3 | 0 |
| abi_cross_scalar_wideselect_null_entry | 31101.6 | 2457.7 | 2339.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_null_entry | 0.014 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2197681ns | 2197681ns | +0.59% |
| abi_cross_scalar_wideselect_inproc_direct | 2184866ns | 2184866ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2284782ns | 2284782ns | +4.57% |
| abi_cross_scalar_wideselect_null_entry | 4673ns | 4673ns | -99.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2142635ns | base | --- | [2126655, 2273449] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2152430ns | no significant difference | [-28982, +49368]ns | [2141289, 2286630] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2144595ns | no significant difference | [-32099, +318425]ns | [2137338, 2559893] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_null_entry | 2325ns | -2140336.6ns (-99.9%) | [-2271043, -2124340]ns | [2274, 2419] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2206229ns | -2.1% | -2.2% | -99.9% |
| 2 | 2133856ns | +1.3% | +0.7% | -99.9% |
| 3 | 2128806ns | +0.6% | +0.5% | -99.9% |
| 4 | 2124503ns | +1.0% | +0.7% | -99.9% |
| 5 | 2151413ns | -0.5% | -0.7% | -99.9% |
| 6 | 2340669ns | +3.0% | +26.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.067 | ok |
| abi_cross_scalar_wideselect_inproc_direct | 0.033 | ok |
| abi_cross_scalar_wideselect_inproc_fnptr | -0.043 | ok |
| abi_cross_scalar_wideselect_null_entry | -0.228 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6741946.2ns | 2193449.5ns | 307.4% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6577836.9ns | 2180912.8ns | 301.6% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6820788.8ns | 2280608.3ns | 299.1% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 120963.5ns | 2339.4ns | 5170.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2140497.1-2286630.0 ns)
  2140497.1 |########################################
  2147803.7 |
  2155110.4 |##########################
  2162417.0 |
  2169723.7 |
  2177030.3 |
  2184337.0 |
  2191643.6 |
  2198950.3 |
  2206256.9 |
  2213563.5 |
  2220870.2 |
  2228176.8 |
  2235483.5 |
  2242790.1 |
  2250096.8 |
  2257403.4 |
  2264710.1 |
  2272016.7 |
  2279323.4 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2124503.3-2273449.0 ns)
  2124503.3 |########################################
  2131950.6 |####################
  2139397.9 |
  2146845.2 |####################
  2154292.4 |
  2161739.7 |
  2169187.0 |
  2176634.3 |
  2184081.6 |
  2191528.9 |
  2198976.1 |####################
  2206423.4 |
  2213870.7 |
  2221318.0 |
  2228765.3 |
  2236212.6 |
  2243659.9 |
  2251107.1 |
  2258554.4 |
  2266001.7 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2135327.9-2559892.9 ns)
  2135327.9 |########################################
  2156556.1 |##########
  2177784.4 |
  2199012.6 |
  2220240.9 |
  2241469.1 |
  2262697.4 |
  2283925.6 |
  2305153.9 |
  2326382.1 |
  2347610.4 |
  2368838.6 |
  2390066.9 |
  2411295.1 |
  2432523.4 |
  2453751.6 |
  2474979.9 |
  2496208.1 |
  2517436.4 |
  2538664.6 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 2256.2-2419.4 ns)
   2256.2 |########################################
   2264.4 |
   2272.5 |
   2280.7 |
   2288.8 |########################################
   2297.0 |########################################
   2305.1 |
   2313.3 |
   2321.5 |
   2329.6 |
   2337.8 |########################################
   2345.9 |
   2354.1 |
   2362.2 |
   2370.4 |########################################
   2378.6 |
   2386.7 |
   2394.9 |
   2403.0 |
   2411.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=5213.0% of algo (FFI overhead may distort results)

# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_wideselect_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_wideselect_inproc_direct has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_wideselect_null_entry at 2.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_wideselect_null_entry dominates: 85437% faster than the next best (abi_cross_scalar_wideselect_ffi_batched_scalar)

abi_cross_scalar_wideselect_null_entry (2.52 us) leads abi_cross_scalar_wideselect_ffi_batched_scalar (2.15 ms) by 85437%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.19 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_inproc_direct is an outlier: 870.7x slower than the field

abi_cross_scalar_wideselect_inproc_direct (2.19 ms) is 870.7x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_inproc_direct} (85437% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_inproc_direct} with a 85437% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 870.7x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (2.52 us) to slowest abi_cross_scalar_wideselect_inproc_direct (2.19 ms): 870.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 2517.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 870.67x (fastest 2517.1 ns, slowest 2191563.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2181639ns | 2156660ns | 2089138ns | 2137461ns | 2294158ns | -0.18% |
| abi_cross_scalar_wideselect_inproc_direct | 2185630ns | 2195137ns | 2082448ns | 2162536ns | 2271864ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2167580ns | 2179743ns | 2079135ns | 2147455ns | 2241991ns | -0.83% |
| abi_cross_scalar_wideselect_null_entry | 4842ns | 4860ns | 4680ns | 4828ns | 4942ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2177818ns | 2085882ns | 2289690ns | -0.19% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2182047ns | 2079355ns | 2267902ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2164053ns | 2075947ns | 2238027ns | -0.82% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 2528ns | 2451ns | 2602ns | -99.88% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 77135.6 | 2179362.7 | 2177817.5 | 15 |
| abi_cross_scalar_wideselect_inproc_direct | 11611.2 | 2192861.7 | 2182047.4 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 10649.3 | 2164188.0 | 2164053.1 | n/a |
| abi_cross_scalar_wideselect_null_entry | 29884.9 | 2736.2 | 2527.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_null_entry | 0.025 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2181639ns | 2181639ns | -0.18% |
| abi_cross_scalar_wideselect_inproc_direct | 2185630ns | 2185630ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2167580ns | 2167580ns | -0.83% |
| abi_cross_scalar_wideselect_null_entry | 4842ns | 4842ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2191563ns | base | --- | [2086677, 2267902] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2153040ns | no significant difference | [-38523, +32894]ns | [2090722, 2289690] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2176274ns | no significant difference | [-55483, +10319]ns | [2077858, 2238027] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_null_entry | 2517ns | -2189001.0ns (-99.9%) | [-2265344, -2084213]ns | [2464, 2602] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2093999ns | +0.1% | -0.7% | -99.9% |
| 2 | 2079355ns | +0.3% | -0.2% | -99.9% |
| 3 | 2308821ns | +2.6% | -3.2% | -99.9% |
| 4 | 2226982ns | -0.7% | +0.6% | -99.9% |
| 5 | 2186107ns | -1.5% | +0.3% | -99.9% |
| 6 | 2197020ns | -2.1% | -1.7% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.069 | ok |
| abi_cross_scalar_wideselect_inproc_direct | 0.054 | ok |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.325 | moderate+ |
| abi_cross_scalar_wideselect_null_entry | -0.052 | ok |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 3/6, lost 2/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 4/6, lost 2/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6628543.6ns | 2177817.5ns | 304.4% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6563736.8ns | 2182047.4ns | 300.8% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6501342.9ns | 2164053.1ns | 300.4% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 116315.1ns | 2527.8ns | 4601.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2085882.5-2289690.4 ns)
  2085882.5 |########################################
  2096072.9 |
  2106263.3 |
  2116453.7 |
  2126644.1 |
  2136834.5 |
  2147024.9 |########################################
  2157215.3 |
  2167405.7 |
  2177596.1 |
  2187786.5 |
  2197976.8 |
  2208167.2 |####################
  2218357.6 |
  2228548.0 |
  2238738.4 |
  2248928.8 |
  2259119.2 |
  2269309.6 |
  2279500.0 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2079355.4-2267901.7 ns)
  2079355.4 |########################################
  2088782.7 |########################################
  2098210.0 |
  2107637.3 |
  2117064.6 |
  2126492.0 |
  2135919.3 |
  2145346.6 |
  2154773.9 |
  2164201.2 |
  2173628.5 |
  2183055.8 |########################################
  2192483.2 |########################################
  2201910.5 |
  2211337.8 |
  2220765.1 |########################################
  2230192.4 |
  2239619.7 |
  2249047.0 |
  2258474.3 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2075946.7-2238026.7 ns)
  2075946.7 |########################################
  2084050.7 |
  2092154.7 |
  2100258.7 |
  2108362.7 |
  2116466.7 |
  2124570.7 |
  2132674.7 |
  2140778.7 |
  2148882.7 |
  2156986.7 |####################
  2165090.7 |
  2173194.7 |
  2181298.7 |
  2189402.7 |####################
  2197506.7 |
  2205610.7 |
  2213714.7 |
  2221818.7 |
  2229922.7 |####################
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 2451.2-2602.5 ns)
   2451.2 |########################################
   2458.8 |
   2466.3 |
   2473.9 |########################################
   2481.5 |
   2489.0 |
   2496.6 |
   2504.2 |
   2511.7 |########################################
   2519.3 |########################################
   2526.8 |
   2534.4 |
   2542.0 |
   2549.5 |
   2557.1 |
   2564.7 |
   2572.2 |
   2579.8 |
   2587.4 |
   2594.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=305.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=298.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=300.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=4621.5% of algo (FFI overhead may distort results)

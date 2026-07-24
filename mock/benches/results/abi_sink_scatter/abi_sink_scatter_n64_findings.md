# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8.71 us) is smaller than the fastest variant's own run-to-run std-dev (26.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.14 ms and 2.15 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink_decode** at 2136324.6 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 2136324.6 ns, slowest 2145031.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2266475ns | 2139634ns | 2135621ns | 2138743ns | 2523499ns | -5.95% |
| abi_sink_scatter_batched_sink_decode | 2150060ns | 2138970ns | 2133786ns | 2138709ns | 2175225ns | -10.78% |
| abi_sink_scatter_null_sink | 2409839ns | 2141183ns | 2137653ns | 2140173ns | 2950432ns | base |
| abi_sink_scatter_per_record_sink | 2377029ns | 2147831ns | 2142194ns | 2145965ns | 2841041ns | -1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2263665ns | 2132949ns | 2520582ns | -5.95% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2147254ns | 2131124ns | 2172096ns | -10.79% | 0.000 |
| abi_sink_scatter_null_sink | 2406871ns | 2134988ns | 2947052ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2373713ns | 2139496ns | 2836575ns | -1.38% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 45562.5 | 2148145.9 | 2263664.8 | n/a |
| abi_sink_scatter_batched_sink_decode | 45287.5 | 2146581.0 | 2147254.2 | n/a |
| abi_sink_scatter_null_sink | 50497.1 | 2162387.9 | 2406871.2 | n/a |
| abi_sink_scatter_per_record_sink | 57837.8 | 2298810.8 | 2373712.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink_decode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.7% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.8% |
| abi_sink_scatter_null_sink | 0.000 | 99.7% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2266475ns | 2266475ns | -5.95% |
| abi_sink_scatter_batched_sink_decode | 2150060ns | 2150060ns | -10.78% |
| abi_sink_scatter_null_sink | 2409839ns | 2409839ns | base |
| abi_sink_scatter_per_record_sink | 2377029ns | 2377029ns | -1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2138360ns | base | --- | [2135201, 2947052] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2136850ns | no significant difference | [-428832, +3433]ns | [2133562, 2520582] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_scatter_batched_sink_decode | 2136325ns | no significant difference | [-803591, +24679]ns | [2133342, 2172096] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_scatter_per_record_sink | 2145032ns | no significant difference | [-465498, +360925]ns | [2139531, 2836575] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2159846ns | -1.1% | +2.1% | +32.8% |
| 2 | 2134988ns | +0.1% | +0.2% | +0.6% |
| 3 | 3734259ns | -22.3% | -42.8% | -24.9% |
| 4 | 2140148ns | -0.3% | -0.4% | -0.0% |
| 5 | 2136571ns | +0.2% | -0.0% | +0.3% |
| 6 | 2135415ns | -0.1% | +0.0% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.235 | moderate- |
| abi_sink_scatter_batched_sink_decode | 0.013 | ok |
| abi_sink_scatter_null_sink | -0.238 | moderate- |
| abi_sink_scatter_per_record_sink | -0.313 | moderate- |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 4/6, lost 2/6
- **abi_sink_scatter_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_scatter_per_record_sink**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6501653.7ns | 2263664.8ns | 287.2% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6492463.5ns | 2147254.2ns | 302.4% | HIGH |
| abi_sink_scatter_null_sink | 6540409.3ns | 2406871.2ns | 271.7% | HIGH |
| abi_sink_scatter_per_record_sink | 7100314.1ns | 2373712.5ns | 299.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2132948.7-2520582.2 ns)
  2132948.7 |########################################
  2152330.4 |
  2171712.1 |
  2191093.7 |
  2210475.4 |
  2229857.1 |
  2249238.8 |
  2268620.4 |
  2288002.1 |
  2307383.8 |
  2326765.5 |
  2346147.2 |
  2365528.8 |
  2384910.5 |
  2404292.2 |
  2423673.9 |
  2443055.5 |
  2462437.2 |
  2481818.9 |
  2501200.6 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2131123.8-2172096.0 ns)
  2131123.8 |#############
  2133172.4 |
  2135221.0 |########################################
  2137269.6 |#############
  2139318.2 |
  2141366.9 |
  2143415.5 |
  2145464.1 |
  2147512.7 |
  2149561.3 |
  2151609.9 |
  2153658.5 |
  2155707.1 |
  2157755.8 |
  2159804.4 |
  2161853.0 |
  2163901.6 |
  2165950.2 |
  2167998.8 |
  2170047.4 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2134987.9-2947052.5 ns)
  2134987.9 |########################################
  2175591.1 |
  2216194.4 |
  2256797.6 |
  2297400.8 |
  2338004.0 |
  2378607.3 |
  2419210.5 |
  2459813.7 |
  2500417.0 |
  2541020.2 |
  2581623.4 |
  2622226.7 |
  2662829.9 |
  2703433.1 |
  2744036.4 |
  2784639.6 |
  2825242.8 |
  2865846.0 |
  2906449.3 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2139495.8-2836575.2 ns)
  2139495.8 |########################################
  2174349.8 |
  2209203.7 |
  2244057.7 |
  2278911.7 |
  2313765.6 |
  2348619.6 |
  2383473.6 |
  2418327.6 |
  2453181.5 |
  2488035.5 |
  2522889.5 |
  2557743.4 |
  2592597.4 |
  2627451.4 |
  2662305.4 |
  2697159.3 |
  2732013.3 |
  2766867.3 |
  2801721.2 |##########
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: CV=24.7% (high variance, measurements may be unstable)
- **abi_sink_scatter_null_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=301.9% of algo (FFI overhead may distort results)

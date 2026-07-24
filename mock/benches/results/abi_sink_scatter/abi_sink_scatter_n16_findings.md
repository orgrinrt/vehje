# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_sink_scatter_batched_sink is fastest but the noisiest (CV 9.0%)

abi_sink_scatter_batched_sink wins on median (2.13 ms) yet has the highest variance (CV 9.0%), while abi_sink_scatter_batched_sink_decode is the steadiest (CV 0.1%, 2.13 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (12.13 us) is smaller than the fastest variant's own run-to-run std-dev (191.44 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_scatter_batched_sink vs stability leader abi_sink_scatter_batched_sink_decode (+0% speed for 70.3x steadier)

abi_sink_scatter_batched_sink is fastest (2.13 ms, CV 9.0%); abi_sink_scatter_batched_sink_decode gives up 0.1% median for 70.3x lower variance (CV 0.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink** at 2130915.8 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 2130915.8 ns, slowest 2143043.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2217789ns | 2133635ns | 2124215ns | 2132634ns | 2392308ns | +3.17% |
| abi_sink_scatter_batched_sink_decode | 2136179ns | 2136025ns | 2132963ns | 2135190ns | 2139269ns | -0.63% |
| abi_sink_scatter_null_sink | 2149635ns | 2136643ns | 2129827ns | 2135559ns | 2180652ns | base |
| abi_sink_scatter_per_record_sink | 2212903ns | 2145730ns | 2139482ns | 2145307ns | 2351007ns | +2.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2214827ns | 2121509ns | 2388852ns | +3.17% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2133504ns | 2130270ns | 2136560ns | -0.62% | 0.000 |
| abi_sink_scatter_null_sink | 2146848ns | 2126942ns | 2177760ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2210104ns | 2136910ns | 2348000ns | +2.95% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 54900.4 | 2230457.2 | 2214826.9 | n/a |
| abi_sink_scatter_batched_sink_decode | 42748.0 | 2132296.0 | 2133503.6 | n/a |
| abi_sink_scatter_null_sink | 46179.0 | 2147249.6 | 2146848.1 | n/a |
| abi_sink_scatter_per_record_sink | 45259.5 | 2186421.9 | 2210103.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.6% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.4% |
| abi_sink_scatter_null_sink | 0.000 | 99.4% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2217789ns | 2217789ns | +3.17% |
| abi_sink_scatter_batched_sink_decode | 2136179ns | 2136179ns | -0.63% |
| abi_sink_scatter_null_sink | 2149635ns | 2149635ns | base |
| abi_sink_scatter_per_record_sink | 2212903ns | 2212903ns | +2.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2133885ns | base | --- | [2128900, 2177760] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2130916ns | no significant difference | [-8686, +215574]ns | [2124713, 2388852] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_scatter_batched_sink_decode | 2133338ns | no significant difference | [-44947, +3934]ns | [2130613, 2136560] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_scatter_per_record_sink | 2143043ns | +10659.6ns (+0.5%) | [+2427, +176679]ns | [2139267, 2348000] | YES (adj: no) | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2139819ns | -0.6% | -0.4% | -0.1% |
| 2 | 2126942ns | -0.3% | +0.2% | +1.0% |
| 3 | 2215700ns | +19.3% | -3.7% | +15.0% |
| 4 | 2133860ns | -0.0% | +0.0% | +0.4% |
| 5 | 2130857ns | +0.2% | +0.1% | +0.5% |
| 6 | 2133910ns | -0.3% | +0.2% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.242 | moderate- |
| abi_sink_scatter_batched_sink_decode | -0.019 | ok |
| abi_sink_scatter_null_sink | -0.296 | moderate- |
| abi_sink_scatter_per_record_sink | -0.223 | moderate- |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 3/6, lost 2/6
- **abi_sink_scatter_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_scatter_per_record_sink**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6807814.1ns | 2214826.9ns | 307.4% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6445101.5ns | 2133503.6ns | 302.1% | HIGH |
| abi_sink_scatter_null_sink | 6482537.0ns | 2146848.1ns | 302.0% | HIGH |
| abi_sink_scatter_per_record_sink | 6579141.5ns | 2210103.6ns | 297.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2121509.2-2388851.9 ns)
  2121509.2 |########################################
  2134876.3 |##########
  2148243.5 |
  2161610.6 |
  2174977.7 |
  2188344.9 |
  2201712.0 |
  2215079.1 |
  2228446.3 |
  2241813.4 |
  2255180.6 |
  2268547.7 |
  2281914.8 |
  2295282.0 |
  2308649.1 |
  2322016.2 |
  2335383.4 |
  2348750.5 |
  2362117.6 |
  2375484.8 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2130269.6-2136560.2 ns)
  2130269.6 |########################################
  2130584.1 |
  2130898.7 |########################################
  2131213.2 |
  2131527.7 |
  2131842.2 |
  2132156.8 |########################################
  2132471.3 |
  2132785.8 |
  2133100.4 |
  2133414.9 |
  2133729.4 |
  2134044.0 |########################################
  2134358.5 |########################################
  2134673.0 |
  2134987.6 |
  2135302.1 |
  2135616.6 |
  2135931.1 |
  2136245.7 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2126942.5-2177759.6 ns)
  2126942.5 |####################
  2129483.4 |####################
  2132024.2 |########################################
  2134565.1 |
  2137105.9 |
  2139646.8 |####################
  2142187.6 |
  2144728.5 |
  2147269.3 |
  2149810.2 |
  2152351.0 |
  2154891.9 |
  2157432.8 |
  2159973.6 |
  2162514.5 |
  2165055.3 |
  2167596.2 |
  2170137.0 |
  2172677.9 |
  2175218.7 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2136910.4-2348000.5 ns)
  2136910.4 |########################################
  2147464.9 |##########
  2158019.4 |
  2168573.9 |
  2179128.4 |
  2189682.9 |
  2200237.4 |
  2210791.9 |
  2221346.4 |
  2231900.9 |
  2242455.4 |
  2253009.9 |
  2263564.4 |
  2274118.9 |
  2284673.4 |
  2295227.9 |
  2305782.4 |
  2316336.9 |
  2326891.4 |
  2337445.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.0% of algo (FFI overhead may distort results)

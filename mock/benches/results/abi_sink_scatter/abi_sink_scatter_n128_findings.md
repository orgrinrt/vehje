# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7.35 us) is smaller than the fastest variant's own run-to-run std-dev (34.86 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink** at 2131622.7 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 2131622.7 ns, slowest 2138969.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2148937ns | 2134293ns | 2131242ns | 2133352ns | 2181162ns | -0.47% |
| abi_sink_scatter_batched_sink_decode | 2143539ns | 2141613ns | 2138726ns | 2140881ns | 2149934ns | -0.72% |
| abi_sink_scatter_null_sink | 2159183ns | 2139659ns | 2136578ns | 2138905ns | 2200903ns | base |
| abi_sink_scatter_per_record_sink | 2240891ns | 2139852ns | 2138654ns | 2139497ns | 2444100ns | +3.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2146273ns | 2128675ns | 2178339ns | -0.47% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2140799ns | 2136166ns | 2147004ns | -0.73% | 0.000 |
| abi_sink_scatter_null_sink | 2156440ns | 2134018ns | 2197995ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2238115ns | 2136208ns | 2440896ns | +3.79% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 42282.3 | 2150329.5 | 2146272.7 | n/a |
| abi_sink_scatter_batched_sink_decode | 44557.7 | 2141151.9 | 2140799.2 | 0 |
| abi_sink_scatter_null_sink | 43966.5 | 2161007.1 | 2156440.4 | n/a |
| abi_sink_scatter_per_record_sink | 46490.1 | 2192303.5 | 2238115.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.9% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_scatter_null_sink | 0.000 | 99.6% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2148937ns | 2148937ns | -0.47% |
| abi_sink_scatter_batched_sink_decode | 2143539ns | 2143539ns | -0.72% |
| abi_sink_scatter_null_sink | 2159183ns | 2159183ns | base |
| abi_sink_scatter_per_record_sink | 2240891ns | 2240891ns | +3.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2136932ns | base | --- | [2134394, 2197995] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2131623ns | -7012.7ns (-0.3%) | [-19655, -3835]ns | [2128856, 2178339] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_scatter_batched_sink_decode | 2138969ns | no significant difference | [-59948, +12610]ns | [2136425, 2147004] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_scatter_per_record_sink | 2137212ns | no significant difference | [-7336, +251061]ns | [2136237, 2440896] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2152578ns | -0.9% | -0.8% | -0.6% |
| 2 | 2134018ns | -0.1% | +0.8% | +0.1% |
| 3 | 2243412ns | -0.9% | -4.6% | +22.2% |
| 4 | 2136125ns | -0.3% | +0.1% | +0.1% |
| 5 | 2134770ns | -0.3% | +0.4% | +0.1% |
| 6 | 2137740ns | -0.3% | -0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.227 | moderate- |
| abi_sink_scatter_batched_sink_decode | -0.420 | moderate- |
| abi_sink_scatter_null_sink | -0.299 | moderate- |
| abi_sink_scatter_per_record_sink | -0.233 | moderate- |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 5/6, lost 0/6
- **abi_sink_scatter_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_scatter_per_record_sink**: won 1/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6543669.7ns | 2146272.7ns | 304.9% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6469618.0ns | 2140799.2ns | 302.2% | HIGH |
| abi_sink_scatter_null_sink | 6575688.6ns | 2156440.4ns | 304.9% | HIGH |
| abi_sink_scatter_per_record_sink | 6682843.4ns | 2238115.0ns | 298.6% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2128675.0-2178339.3 ns)
  2128675.0 |########################################
  2131158.2 |##########################
  2133641.4 |
  2136124.7 |
  2138607.9 |
  2141091.1 |
  2143574.3 |
  2146057.5 |
  2148540.7 |
  2151024.0 |
  2153507.2 |
  2155990.4 |
  2158473.6 |
  2160956.8 |
  2163440.0 |
  2165923.3 |
  2168406.5 |
  2170889.7 |
  2173372.9 |
  2175856.1 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2136165.8-2147004.1 ns)
  2136165.8 |########################################
  2136707.7 |
  2137249.6 |
  2137791.6 |####################
  2138333.5 |
  2138875.4 |
  2139417.3 |####################
  2139959.2 |
  2140501.1 |
  2141043.1 |
  2141585.0 |
  2142126.9 |####################
  2142668.8 |
  2143210.7 |
  2143752.6 |
  2144294.6 |
  2144836.5 |
  2145378.4 |
  2145920.3 |
  2146462.2 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2134017.5-2197994.8 ns)
  2134017.5 |########################################
  2137216.4 |#############
  2140415.2 |
  2143614.1 |
  2146813.0 |
  2150011.8 |#############
  2153210.7 |
  2156409.6 |
  2159608.4 |
  2162807.3 |
  2166006.1 |
  2169205.0 |
  2172403.9 |
  2175602.7 |
  2178801.6 |
  2182000.5 |
  2185199.3 |
  2188398.2 |
  2191597.1 |
  2194795.9 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2136207.9-2440896.0 ns)
  2136207.9 |########################################
  2151442.3 |
  2166676.7 |
  2181911.1 |
  2197145.5 |
  2212379.9 |
  2227614.3 |
  2242848.8 |
  2258083.2 |
  2273317.6 |
  2288552.0 |
  2303786.4 |
  2319020.8 |
  2334255.2 |
  2349489.6 |
  2364724.0 |
  2379958.4 |
  2395192.8 |
  2410427.2 |
  2425661.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.2% of algo (FFI overhead may distort results)

# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.68 us) is smaller than the fastest variant's own run-to-run std-dev (33.53 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2133818.0 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 2133818.0 ns, slowest 2139495.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2149819ns | 2136402ns | 2127126ns | 2135211ns | 2183077ns | -1.54% |
| abi_sink_real_batched_sink_decode | 2137472ns | 2136953ns | 2129368ns | 2136430ns | 2143087ns | -2.10% |
| abi_sink_real_null_sink | 2183427ns | 2136989ns | 2119940ns | 2133409ns | 2290198ns | base |
| abi_sink_real_per_record_sink | 2164432ns | 2142118ns | 2132253ns | 2139682ns | 2217645ns | -0.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2147087ns | 2124608ns | 2180028ns | -1.54% | 0.000 |
| abi_sink_real_batched_sink_decode | 2134939ns | 2126988ns | 2140554ns | -2.10% | 0.000 |
| abi_sink_real_null_sink | 2180703ns | 2117527ns | 2287162ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2161756ns | 2129789ns | 2214714ns | -0.87% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 45082.8 | 2175898.0 | 2147086.7 | 0 |
| abi_sink_real_batched_sink_decode | 40007.6 | 2135371.4 | 2134938.7 | n/a |
| abi_sink_real_null_sink | 44351.7 | 2214143.9 | 2180703.0 | n/a |
| abi_sink_real_per_record_sink | 43179.6 | 2166875.3 | 2161756.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_null_sink; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.2% |
| abi_sink_real_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_real_null_sink | 0.000 | 99.2% |
| abi_sink_real_per_record_sink | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2149819ns | 2149819ns | -1.54% |
| abi_sink_real_batched_sink_decode | 2137472ns | 2137472ns | -2.10% |
| abi_sink_real_null_sink | 2183427ns | 2183427ns | base |
| abi_sink_real_per_record_sink | 2164432ns | 2164432ns | -0.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2134427ns | base | --- | [2120520, 2287162] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2133818ns | no significant difference | [-110514, +15829]ns | [2127414, 2180028] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_real_batched_sink_decode | 2134417ns | no significant difference | [-152311, +14850]ns | [2129845, 2140554] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_real_per_record_sink | 2139496ns | no significant difference | [-78600, +20624]ns | [2131059, 2214714] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2134245ns | -0.2% | -0.3% | -0.1% |
| 2 | 2432486ns | -8.7% | -12.2% | -6.0% |
| 3 | 2134608ns | -0.5% | -0.1% | +0.2% |
| 4 | 2117527ns | +0.8% | +0.9% | +1.1% |
| 5 | 2123513ns | +0.7% | +0.5% | +0.9% |
| 6 | 2141838ns | -0.4% | +0.1% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.357 | moderate- |
| abi_sink_real_batched_sink_decode | -0.129 | ok |
| abi_sink_real_null_sink | -0.190 | ok |
| abi_sink_real_per_record_sink | -0.264 | moderate- |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 4/6, lost 2/6
- **abi_sink_real_batched_sink_decode**: won 2/6, lost 3/6
- **abi_sink_real_per_record_sink**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6737958.5ns | 2147086.7ns | 313.8% | HIGH |
| abi_sink_real_batched_sink_decode | 6448263.0ns | 2134938.7ns | 302.0% | HIGH |
| abi_sink_real_null_sink | 6739957.1ns | 2180703.0ns | 309.1% | HIGH |
| abi_sink_real_per_record_sink | 6554210.5ns | 2161756.2ns | 303.2% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2124607.9-2180028.2 ns)
  2124607.9 |####################
  2127378.9 |
  2130149.9 |####################
  2132920.9 |########################################
  2135692.0 |
  2138463.0 |####################
  2141234.0 |
  2144005.0 |
  2146776.0 |
  2149547.0 |
  2152318.0 |
  2155089.0 |
  2157860.1 |
  2160631.1 |
  2163402.1 |
  2166173.1 |
  2168944.1 |
  2171715.1 |
  2174486.1 |
  2177257.1 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2126988.3-2140554.2 ns)
  2126988.3 |########################################
  2127666.6 |
  2128344.9 |
  2129023.2 |
  2129701.5 |
  2130379.8 |
  2131058.1 |
  2131736.3 |
  2132414.6 |########################################
  2133092.9 |########################################
  2133771.2 |
  2134449.5 |########################################
  2135127.8 |
  2135806.1 |
  2136484.4 |########################################
  2137162.7 |
  2137841.0 |
  2138519.3 |
  2139197.6 |
  2139875.9 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2117527.1-2287162.2 ns)
  2117527.1 |########################################
  2126008.9 |####################
  2134490.6 |########################################
  2142972.4 |
  2151454.1 |
  2159935.9 |
  2168417.6 |
  2176899.4 |
  2185381.2 |
  2193862.9 |
  2202344.7 |
  2210826.4 |
  2219308.2 |
  2227789.9 |
  2236271.7 |
  2244753.5 |
  2253235.2 |
  2261717.0 |
  2270198.7 |
  2278680.5 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2129789.2-2214714.0 ns)
  2129789.2 |##########################
  2134035.4 |
  2138281.7 |########################################
  2142527.9 |
  2146774.2 |
  2151020.4 |
  2155266.6 |
  2159512.9 |
  2163759.1 |
  2168005.3 |
  2172251.6 |
  2176497.8 |
  2180744.1 |
  2184990.3 |
  2189236.5 |
  2193482.8 |
  2197729.0 |
  2201975.2 |
  2206221.5 |
  2210467.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=302.3% of algo (FFI overhead may distort results)

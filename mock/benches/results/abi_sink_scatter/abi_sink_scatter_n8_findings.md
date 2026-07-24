# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10.25 us) is smaller than the fastest variant's own run-to-run std-dev (44.43 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_scatter_null_sink)

The baseline abi_sink_scatter_null_sink is the fastest (2.13 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.5% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_scatter_null_sink) is the fastest** at 2134423.0 ns median
- Spread: 1.00x (fastest 2134423.0 ns, slowest 2144671.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2231479ns | 2139253ns | 2131695ns | 2137938ns | 2421684ns | +3.49% |
| abi_sink_scatter_batched_sink_decode | 2141983ns | 2140911ns | 2136386ns | 2140113ns | 2147588ns | -0.66% |
| abi_sink_scatter_null_sink | 2156169ns | 2137027ns | 2133616ns | 2136041ns | 2197636ns | base |
| abi_sink_scatter_per_record_sink | 2149121ns | 2147354ns | 2140054ns | 2146541ns | 2157525ns | -0.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2228547ns | 2129156ns | 2418288ns | +3.49% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2139244ns | 2133692ns | 2144756ns | -0.66% | 0.000 |
| abi_sink_scatter_null_sink | 2153468ns | 2130962ns | 2194754ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2146354ns | 2137522ns | 2154548ns | -0.33% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 50388.7 | 2223123.9 | 2228547.1 | n/a |
| abi_sink_scatter_batched_sink_decode | 44725.6 | 2137054.1 | 2139243.9 | n/a |
| abi_sink_scatter_null_sink | 44769.3 | 2162081.7 | 2153468.0 | n/a |
| abi_sink_scatter_per_record_sink | 46237.6 | 2145735.4 | 2146353.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.7% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.6% |
| abi_sink_scatter_null_sink | 0.000 | 99.8% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2231479ns | 2231479ns | +3.49% |
| abi_sink_scatter_batched_sink_decode | 2141983ns | 2141983ns | -0.66% |
| abi_sink_scatter_null_sink | 2156169ns | 2156169ns | base |
| abi_sink_scatter_per_record_sink | 2149121ns | 2149121ns | -0.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2134423ns | base | --- | [2131227, 2194754] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2136499ns | no significant difference | [-3538, +226448]ns | [2130854, 2418288] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_scatter_batched_sink_decode | 2138270ns | no significant difference | [-57482, +9659]ns | [2134706, 2144756] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_scatter_per_record_sink | 2144672ns | no significant difference | [-54913, +22101]ns | [2139841, 2154548] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2133402ns | +0.2% | +0.4% | +1.2% |
| 2 | 2136791ns | -0.4% | +0.5% | +0.0% |
| 3 | 2252718ns | +19.4% | -5.2% | -4.9% |
| 4 | 2130962ns | +0.7% | +0.4% | +0.7% |
| 5 | 2135444ns | +0.0% | +0.1% | +0.4% |
| 6 | 2131491ns | +0.0% | +0.1% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.228 | moderate- |
| abi_sink_scatter_batched_sink_decode | 0.034 | ok |
| abi_sink_scatter_null_sink | -0.232 | moderate- |
| abi_sink_scatter_per_record_sink | -0.296 | moderate- |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 1/6, lost 3/6
- **abi_sink_scatter_batched_sink_decode**: won 1/6, lost 4/6
- **abi_sink_scatter_per_record_sink**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6749894.4ns | 2228547.1ns | 302.9% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6458448.8ns | 2139243.9ns | 301.9% | HIGH |
| abi_sink_scatter_null_sink | 6493193.0ns | 2153468.0ns | 301.5% | HIGH |
| abi_sink_scatter_per_record_sink | 6485776.6ns | 2146353.5ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2129156.2-2418288.3 ns)
  2129156.2 |########################################
  2143612.8 |##########
  2158069.4 |
  2172526.0 |
  2186982.6 |
  2201439.2 |
  2215895.8 |
  2230352.4 |
  2244809.0 |
  2259265.6 |
  2273722.2 |
  2288178.9 |
  2302635.5 |
  2317092.1 |
  2331548.7 |
  2346005.3 |
  2360461.9 |
  2374918.5 |
  2389375.1 |
  2403831.7 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2133692.5-2144756.0 ns)
  2133692.5 |########################################
  2134245.7 |
  2134798.9 |
  2135352.0 |########################################
  2135905.2 |
  2136458.4 |
  2137011.5 |########################################
  2137564.7 |
  2138117.9 |
  2138671.1 |########################################
  2139224.2 |
  2139777.4 |
  2140330.6 |
  2140883.8 |
  2141437.0 |########################################
  2141990.1 |
  2142543.3 |
  2143096.5 |
  2143649.6 |
  2144202.8 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2130962.5-2194754.4 ns)
  2130962.5 |########################################
  2134152.1 |##########################
  2137341.7 |
  2140531.3 |
  2143720.9 |
  2146910.5 |
  2150100.1 |
  2153289.6 |
  2156479.2 |
  2159668.8 |
  2162858.4 |
  2166048.0 |
  2169237.6 |
  2172427.2 |
  2175616.8 |
  2178806.4 |
  2181996.0 |
  2185185.6 |
  2188375.2 |
  2191564.8 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2137522.5-2154547.5 ns)
  2137522.5 |########################################
  2138373.8 |
  2139225.0 |
  2140076.2 |
  2140927.5 |
  2141778.8 |########################################
  2142630.0 |########################################
  2143481.2 |
  2144332.5 |
  2145183.8 |########################################
  2146035.0 |
  2146886.2 |
  2147737.5 |
  2148588.8 |########################################
  2149440.0 |
  2150291.2 |
  2151142.5 |
  2151993.8 |
  2152845.0 |
  2153696.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.2% of algo (FFI overhead may distort results)

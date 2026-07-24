# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.5% of the fastest

All 4 variants sit between 2.06 ms and 2.09 ms - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_batched_sink** at 2063590.0 ns median (-0.1% vs baseline)
- Spread: 1.01x (fastest 2063590.0 ns, slowest 2094530.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2076310ns | 2066765ns | 2053282ns | 2065479ns | 2104071ns | -2.65% |
| abi_sink_wideselect_batched_sink_decode | 2124724ns | 2097900ns | 2057702ns | 2089035ns | 2211768ns | -0.38% |
| abi_sink_wideselect_null_sink | 2132842ns | 2067531ns | 2058646ns | 2067038ns | 2268645ns | base |
| abi_sink_wideselect_per_record_sink | 2087702ns | 2079699ns | 2070213ns | 2079121ns | 2109318ns | -2.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2073302ns | 2050766ns | 2100807ns | -2.65% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2121261ns | 2055132ns | 2207609ns | -0.39% | 0.000 |
| abi_sink_wideselect_null_sink | 2129673ns | 2056118ns | 2264629ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2084382ns | 2067140ns | 2105759ns | -2.13% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 57089.2 | 2070962.8 | 2073301.7 | n/a |
| abi_sink_wideselect_batched_sink_decode | 68047.2 | 2132430.1 | 2121260.8 | 1 |
| abi_sink_wideselect_null_sink | 58617.5 | 2153823.5 | 2129672.9 | n/a |
| abi_sink_wideselect_per_record_sink | 67854.7 | 2088881.7 | 2084381.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.4% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 97.9% |
| abi_sink_wideselect_null_sink | 0.000 | 99.3% |
| abi_sink_wideselect_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2076310ns | 2076310ns | -2.65% |
| abi_sink_wideselect_batched_sink_decode | 2124724ns | 2124724ns | -0.38% |
| abi_sink_wideselect_null_sink | 2132842ns | 2132842ns | base |
| abi_sink_wideselect_per_record_sink | 2087702ns | 2087702ns | -2.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2064740ns | base | --- | [2059650, 2264629] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2063590ns | no significant difference | [-178732, +14996]ns | [2055508, 2100807] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2094531ns | no significant difference | [-97171, +73997]ns | [2061643, 2207609] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_wideselect_per_record_sink | 2076510ns | no significant difference | [-177073, +32315]ns | [2070876, 2105759] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2063181ns | +0.1% | +0.2% | +0.7% |
| 2 | 2443211ns | -13.6% | -7.5% | -14.0% |
| 3 | 2065251ns | +1.3% | +2.4% | +2.2% |
| 4 | 2086048ns | -1.2% | -0.6% | -0.5% |
| 5 | 2064228ns | -0.7% | -0.4% | +0.1% |
| 6 | 2056118ns | +0.2% | +4.8% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.286 | moderate+ |
| abi_sink_wideselect_batched_sink_decode | -0.238 | moderate- |
| abi_sink_wideselect_null_sink | -0.258 | moderate- |
| abi_sink_wideselect_per_record_sink | 0.267 | moderate+ |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 3/6, lost 3/6
- **abi_sink_wideselect_batched_sink_decode**: won 3/6, lost 3/6
- **abi_sink_wideselect_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6280765.1ns | 2073301.7ns | 302.9% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6458383.9ns | 2121260.8ns | 304.5% | HIGH |
| abi_sink_wideselect_null_sink | 6499688.1ns | 2129672.9ns | 305.2% | HIGH |
| abi_sink_wideselect_per_record_sink | 6350428.5ns | 2084381.6ns | 304.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2050765.8-2100806.6 ns)
  2050765.8 |########################################
  2053267.8 |
  2055769.9 |
  2058271.9 |########################################
  2060774.0 |########################################
  2063276.0 |
  2065778.1 |########################################
  2068280.1 |
  2070782.1 |
  2073284.2 |
  2075786.2 |
  2078288.3 |
  2080790.3 |
  2083292.4 |
  2085794.4 |
  2088296.4 |
  2090798.5 |########################################
  2093300.5 |
  2095802.6 |
  2098304.6 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2055131.7-2207609.0 ns)
  2055131.7 |########################################
  2062755.6 |########################################
  2070379.4 |########################################
  2078003.3 |
  2085627.1 |
  2093251.0 |
  2100874.9 |
  2108498.7 |########################################
  2116122.6 |
  2123746.5 |
  2131370.3 |
  2138994.2 |
  2146618.1 |
  2154241.9 |########################################
  2161865.8 |
  2169489.6 |
  2177113.5 |
  2184737.4 |
  2192361.2 |
  2199985.1 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2056118.3-2264629.3 ns)
  2056118.3 |########################################
  2066543.9 |
  2076969.4 |##########
  2087395.0 |
  2097820.5 |
  2108246.1 |
  2118671.6 |
  2129097.2 |
  2139522.7 |
  2149948.3 |
  2160373.8 |
  2170799.4 |
  2181224.9 |
  2191650.5 |
  2202076.0 |
  2212501.6 |
  2222927.1 |
  2233352.7 |
  2243778.2 |
  2254203.8 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2067140.0-2105758.5 ns)
  2067140.0 |########################################
  2069070.9 |
  2071001.9 |
  2072932.8 |########################################
  2074863.7 |########################################
  2076794.6 |########################################
  2078725.6 |
  2080656.5 |
  2082587.4 |
  2084518.3 |
  2086449.3 |
  2088380.2 |
  2090311.1 |
  2092242.1 |
  2094173.0 |
  2096103.9 |
  2098034.8 |
  2099965.8 |########################################
  2101896.7 |
  2103827.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=303.8% of algo (FFI overhead may distort results)

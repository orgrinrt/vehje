# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_wideselect_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_wideselect_null_sink has the worst median (2.17 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_wideselect_per_record_sink at 2.08 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_sink_wideselect_per_record_sink is fastest but the noisiest (CV 17.5%)

abi_sink_wideselect_per_record_sink wins on median (2.08 ms) yet has the highest variance (CV 17.5%), while abi_sink_wideselect_batched_sink is the steadiest (CV 8.9%, 2.11 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (95.61 us) is smaller than the fastest variant's own run-to-run std-dev (364.19 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_wideselect_per_record_sink vs stability leader abi_sink_wideselect_batched_sink (+2% speed for 2.0x steadier)

abi_sink_wideselect_per_record_sink is fastest (2.08 ms, CV 17.5%); abi_sink_wideselect_batched_sink gives up 1.5% median for 2.0x lower variance (CV 8.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.6% of the fastest

All 4 variants sit between 2.08 ms and 2.17 ms - a 4.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_per_record_sink** at 2076577.7 ns median (-4.4% vs baseline)
- Spread: 1.05x (fastest 2076577.7 ns, slowest 2172187.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2219517ns | 2111507ns | 2061809ns | 2099093ns | 2479006ns | -4.02% |
| abi_sink_wideselect_batched_sink_decode | 2271423ns | 2138585ns | 2070590ns | 2116994ns | 2603484ns | -1.78% |
| abi_sink_wideselect_null_sink | 2312534ns | 2175782ns | 2066071ns | 2139644ns | 2695101ns | base |
| abi_sink_wideselect_per_record_sink | 2253808ns | 2079777ns | 2070282ns | 2077113ns | 2610613ns | -2.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2215672ns | 2058980ns | 2474222ns | -3.80% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2267790ns | 2067452ns | 2599295ns | -1.53% | 0.000 |
| abi_sink_wideselect_null_sink | 2303124ns | 2062957ns | 2673539ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2248890ns | 2067360ns | 2602034ns | -2.35% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 73183.7 | 2256727.5 | 2215672.2 | n/a |
| abi_sink_wideselect_batched_sink_decode | 73454.2 | 2191010.9 | 2267790.4 | n/a |
| abi_sink_wideselect_null_sink | 76954.3 | 2269151.6 | 2303123.9 | n/a |
| abi_sink_wideselect_per_record_sink | 71141.3 | 2259256.3 | 2248890.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 97.7% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 96.4% |
| abi_sink_wideselect_null_sink | 0.000 | 94.8% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2219517ns | 2219517ns | -4.02% |
| abi_sink_wideselect_batched_sink_decode | 2271423ns | 2271423ns | -1.78% |
| abi_sink_wideselect_null_sink | 2312534ns | 2312534ns | base |
| abi_sink_wideselect_per_record_sink | 2253808ns | 2253808ns | -2.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2172188ns | base | --- | [2063645, 2673539] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2107807ns | no significant difference | [-565733, +307279]ns | [2064988, 2474222] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2134888ns | no significant difference | [-547719, +432353]ns | [2069189, 2599295] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_wideselect_per_record_sink | 2076578ns | no significant difference | [-597109, +435091]ns | [2068059, 2602034] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2074824ns | -0.8% | +0.5% | -0.4% |
| 2 | 2269552ns | +6.6% | +23.9% | +34.9% |
| 3 | 2064333ns | +22.5% | +15.6% | +3.8% |
| 4 | 2062957ns | +0.4% | +0.4% | +0.3% |
| 5 | 2517712ns | -15.5% | -13.2% | -17.8% |
| 6 | 2829366ns | -26.2% | -26.9% | -26.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.051 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.057 | ok |
| abi_sink_wideselect_null_sink | 0.274 | moderate+ |
| abi_sink_wideselect_per_record_sink | -0.191 | ok |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 3/6, lost 3/6
- **abi_sink_wideselect_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_wideselect_per_record_sink**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6801969.0ns | 2215672.2ns | 307.0% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6640597.8ns | 2267790.4ns | 292.8% | HIGH |
| abi_sink_wideselect_null_sink | 7058433.8ns | 2303123.9ns | 306.5% | HIGH |
| abi_sink_wideselect_per_record_sink | 6850246.3ns | 2248890.2ns | 304.6% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2058979.6-2474221.9 ns)
  2058979.6 |########################################
  2079741.7 |####################
  2100503.8 |
  2121265.9 |####################
  2142028.1 |
  2162790.2 |
  2183552.3 |
  2204314.4 |
  2225076.5 |
  2245838.6 |
  2266600.7 |
  2287362.8 |
  2308125.0 |
  2328887.1 |
  2349649.2 |
  2370411.3 |
  2391173.4 |
  2411935.5 |####################
  2432697.6 |
  2453459.7 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2067451.7-2599295.2 ns)
  2067451.7 |########################################
  2094043.9 |
  2120636.0 |
  2147228.2 |
  2173820.4 |#############
  2200412.6 |
  2227004.8 |
  2253596.9 |
  2280189.1 |
  2306781.3 |
  2333373.5 |
  2359965.6 |
  2386557.8 |#############
  2413150.0 |
  2439742.1 |
  2466334.3 |
  2492926.5 |
  2519518.7 |
  2546110.9 |
  2572703.0 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2062956.7-2673539.2 ns)
  2062956.7 |########################################
  2093485.8 |
  2124014.9 |
  2154544.1 |
  2185073.2 |
  2215602.3 |
  2246131.4 |#############
  2276660.6 |
  2307189.7 |
  2337718.8 |
  2368247.9 |
  2398777.0 |
  2429306.2 |
  2459835.3 |
  2490364.4 |#############
  2520893.5 |
  2551422.7 |
  2581951.8 |
  2612480.9 |
  2643010.0 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2067360.0-2602033.8 ns)
  2067360.0 |########################################
  2094093.7 |
  2120827.4 |##########
  2147561.1 |
  2174294.8 |
  2201028.4 |
  2227762.1 |
  2254495.8 |
  2281229.5 |
  2307963.2 |
  2334696.9 |
  2361430.6 |
  2388164.2 |
  2414897.9 |
  2441631.6 |
  2468365.3 |
  2495099.0 |
  2521832.7 |
  2548566.4 |
  2575300.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=321.0% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=303.8% of algo (FFI overhead may distort results)

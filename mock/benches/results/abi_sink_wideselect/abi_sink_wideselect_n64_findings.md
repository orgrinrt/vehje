# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (13.25 us) is smaller than the fastest variant's own run-to-run std-dev (42.77 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.07 ms and 2.08 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_batched_sink** at 2065738.8 ns median (-0.3% vs baseline)
- Spread: 1.01x (fastest 2065738.8 ns, slowest 2078993.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2089717ns | 2068867ns | 2061830ns | 2067802ns | 2136532ns | +0.34% |
| abi_sink_wideselect_batched_sink_decode | 2157449ns | 2072725ns | 2066143ns | 2071153ns | 2332546ns | +3.59% |
| abi_sink_wideselect_null_sink | 2082602ns | 2075535ns | 2066219ns | 2074739ns | 2102589ns | base |
| abi_sink_wideselect_per_record_sink | 2325060ns | 2082443ns | 2066995ns | 2077855ns | 2824901ns | +11.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2086225ns | 2058834ns | 2132220ns | +0.32% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2154291ns | 2063364ns | 2328878ns | +3.59% | 0.000 |
| abi_sink_wideselect_null_sink | 2079598ns | 2063407ns | 2099406ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2321814ns | 2064106ns | 2821439ns | +11.65% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 62928.5 | 2086191.4 | 2086225.1 | n/a |
| abi_sink_wideselect_batched_sink_decode | 65436.5 | 2137490.1 | 2154291.4 | n/a |
| abi_sink_wideselect_null_sink | 58323.0 | 2075950.6 | 2079597.7 | n/a |
| abi_sink_wideselect_per_record_sink | 63412.5 | 2087412.5 | 2321814.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.7% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_wideselect_null_sink | 0.000 | 99.3% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2089717ns | 2089717ns | +0.34% |
| abi_sink_wideselect_batched_sink_decode | 2157449ns | 2157449ns | +3.59% |
| abi_sink_wideselect_null_sink | 2082602ns | 2082602ns | base |
| abi_sink_wideselect_per_record_sink | 2325060ns | 2325060ns | +11.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2072580ns | base | --- | [2066807, 2099406] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2065739ns | no significant difference | [-35895, +64891]ns | [2060717, 2132220] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2069765ns | no significant difference | [-9919, +233112]ns | [2064232, 2328878] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_wideselect_per_record_sink | 2078994ns | no significant difference | [-4306, +724344]ns | [2065010, 2821439] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2063407ns | +1.0% | +0.4% | +0.1% |
| 2 | 2120281ns | -2.7% | +20.8% | +67.5% |
| 3 | 2070206ns | -0.4% | -0.3% | -0.3% |
| 4 | 2071251ns | +5.3% | +1.2% | -0.1% |
| 5 | 2078532ns | -0.5% | -0.5% | +0.5% |
| 6 | 2073909ns | -0.7% | -0.4% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | -0.260 | moderate- |
| abi_sink_wideselect_batched_sink_decode | -0.262 | moderate- |
| abi_sink_wideselect_null_sink | -0.449 | moderate- |
| abi_sink_wideselect_per_record_sink | -0.250 | moderate- |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 4/6, lost 2/6
- **abi_sink_wideselect_batched_sink_decode**: won 3/6, lost 3/6
- **abi_sink_wideselect_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6324549.9ns | 2086225.1ns | 303.2% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6465947.3ns | 2154291.4ns | 300.1% | HIGH |
| abi_sink_wideselect_null_sink | 6398250.3ns | 2079597.7ns | 307.7% | HIGH |
| abi_sink_wideselect_per_record_sink | 6329684.4ns | 2321814.4ns | 272.6% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2058833.8-2132219.8 ns)
  2058833.8 |####################
  2062503.1 |########################################
  2066172.4 |####################
  2069841.7 |
  2073511.0 |
  2077180.3 |
  2080849.6 |####################
  2084518.9 |
  2088188.2 |
  2091857.5 |
  2095526.8 |
  2099196.1 |
  2102865.4 |
  2106534.7 |
  2110204.0 |
  2113873.3 |
  2117542.6 |
  2121211.9 |
  2124881.2 |
  2128550.5 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2063364.2-2328877.5 ns)
  2063364.2 |########################################
  2076639.9 |
  2089915.5 |##########
  2103191.2 |
  2116466.9 |
  2129742.5 |
  2143018.2 |
  2156293.9 |
  2169569.5 |
  2182845.2 |
  2196120.9 |
  2209396.5 |
  2222672.2 |
  2235947.8 |
  2249223.5 |
  2262499.2 |
  2275774.8 |
  2289050.5 |
  2302326.2 |
  2315601.8 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2063407.1-2099406.5 ns)
  2063407.1 |########################################
  2065207.1 |
  2067007.0 |
  2068807.0 |########################################
  2070607.0 |########################################
  2072406.9 |########################################
  2074206.9 |
  2076006.9 |
  2077806.8 |########################################
  2079606.8 |
  2081406.8 |
  2083206.7 |
  2085006.7 |
  2086806.7 |
  2088606.6 |
  2090406.6 |
  2092206.6 |
  2094006.5 |
  2095806.5 |
  2097606.5 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2064105.8-2821439.4 ns)
  2064105.8 |########################################
  2101972.5 |
  2139839.2 |
  2177705.8 |
  2215572.5 |
  2253439.2 |
  2291305.9 |
  2329172.6 |
  2367039.2 |
  2404905.9 |
  2442772.6 |
  2480639.3 |
  2518506.0 |
  2556372.6 |
  2594239.3 |
  2632106.0 |
  2669972.7 |
  2707839.4 |
  2745706.0 |
  2783572.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: CV=23.7% (high variance, measurements may be unstable)
- **abi_sink_wideselect_per_record_sink**: bridge=302.1% of algo (FFI overhead may distort results)

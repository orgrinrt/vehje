# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (abi_sink_real_null_sink)

The baseline abi_sink_real_null_sink is the fastest (2.13 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.5% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_real_null_sink) is the fastest** at 2125542.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 2125542.1 ns, slowest 2135311.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2126987ns | 2129863ns | 2101998ns | 2127089ns | 2139329ns | -0.01% |
| abi_sink_real_batched_sink_decode | 2133184ns | 2132362ns | 2120064ns | 2131364ns | 2142474ns | +0.29% |
| abi_sink_real_null_sink | 2127099ns | 2128024ns | 2112400ns | 2126370ns | 2135542ns | base |
| abi_sink_real_per_record_sink | 2139126ns | 2137814ns | 2127525ns | 2134438ns | 2151958ns | +0.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2124506ns | 2099573ns | 2136744ns | -0.00% | 0.000 |
| abi_sink_real_batched_sink_decode | 2130721ns | 2117554ns | 2140014ns | +0.29% | 0.000 |
| abi_sink_real_null_sink | 2124604ns | 2110010ns | 2132962ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2136539ns | 2124905ns | 2149238ns | +0.56% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 38330.0 | 2126536.2 | 2124506.0 | 4 |
| abi_sink_real_batched_sink_decode | 39349.2 | 2132331.5 | 2130720.8 | n/a |
| abi_sink_real_null_sink | 37953.0 | 2124739.1 | 2124603.6 | n/a |
| abi_sink_real_per_record_sink | 39641.8 | 2136827.8 | 2136538.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 98.7% |
| abi_sink_real_batched_sink_decode | 0.000 | 98.6% |
| abi_sink_real_null_sink | 0.000 | 98.8% |
| abi_sink_real_per_record_sink | 0.000 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2126987ns | 2126987ns | -0.01% |
| abi_sink_real_batched_sink_decode | 2133184ns | 2133184ns | +0.29% |
| abi_sink_real_null_sink | 2127099ns | 2127099ns | base |
| abi_sink_real_per_record_sink | 2139126ns | 2139126ns | +0.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2125542ns | base | --- | [2115307, 2132962] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2127431ns | no significant difference | [-10750, +13903]ns | [2109343, 2136744] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_real_batched_sink_decode | 2129873ns | no significant difference | [-2073, +12387]ns | [2122276, 2140014] | no | 0.3281 | 0.2188 | 0 |
| abi_sink_real_per_record_sink | 2135311ns | +8492.9ns (+0.4%) | [+1761, +25551]ns | [2125067, 2149238] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2120604ns | +1.1% | +0.4% | +0.2% |
| 2 | 2110010ns | -0.5% | +0.4% | +1.2% |
| 3 | 2130177ns | -0.5% | +0.5% | +0.3% |
| 4 | 2135747ns | -0.4% | -0.2% | +1.2% |
| 5 | 2125077ns | +0.2% | +0.6% | +0.5% |
| 6 | 2126007ns | +0.0% | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.300 | moderate- |
| abi_sink_real_batched_sink_decode | -0.413 | moderate- |
| abi_sink_real_null_sink | 0.117 | ok |
| abi_sink_real_per_record_sink | 0.008 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 3/6, lost 2/6
- **abi_sink_real_batched_sink_decode**: won 1/6, lost 4/6
- **abi_sink_real_per_record_sink**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6418462.3ns | 2124506.0ns | 302.1% | HIGH |
| abi_sink_real_batched_sink_decode | 6434787.1ns | 2130720.8ns | 302.0% | HIGH |
| abi_sink_real_null_sink | 6418172.2ns | 2124603.6ns | 302.1% | HIGH |
| abi_sink_real_per_record_sink | 6450840.6ns | 2136538.7ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2099572.9-2136743.8 ns)
  2099572.9 |########################################
  2101431.4 |
  2103290.0 |
  2105148.5 |
  2107007.1 |
  2108865.6 |
  2110724.2 |
  2112582.7 |
  2114441.2 |
  2116299.8 |
  2118158.3 |########################################
  2120016.9 |
  2121875.4 |
  2123734.0 |
  2125592.5 |########################################
  2127451.0 |########################################
  2129309.6 |########################################
  2131168.1 |
  2133026.7 |
  2134885.2 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2117553.8-2140013.8 ns)
  2117553.8 |########################################
  2118676.8 |
  2119799.8 |
  2120922.8 |
  2122045.8 |
  2123168.8 |
  2124291.8 |
  2125414.8 |
  2126537.8 |########################################
  2127660.8 |
  2128783.8 |########################################
  2129906.8 |########################################
  2131029.8 |
  2132152.8 |
  2133275.8 |
  2134398.8 |
  2135521.8 |
  2136644.8 |
  2137767.8 |########################################
  2138890.8 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2110009.6-2132961.9 ns)
  2110009.6 |####################
  2111157.2 |
  2112304.8 |
  2113452.4 |
  2114600.1 |
  2115747.7 |
  2116895.3 |
  2118042.9 |
  2119190.5 |
  2120338.1 |####################
  2121485.8 |
  2122633.4 |
  2123781.0 |
  2124928.6 |########################################
  2126076.2 |
  2127223.8 |
  2128371.4 |
  2129519.1 |####################
  2130666.7 |
  2131814.3 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2124904.6-2149238.1 ns)
  2124904.6 |########################################
  2126121.3 |
  2127338.0 |
  2128554.6 |
  2129771.3 |
  2130988.0 |
  2132204.6 |
  2133421.3 |
  2134638.0 |########################################
  2135854.7 |
  2137071.3 |####################
  2138288.0 |
  2139504.7 |
  2140721.4 |
  2141938.0 |
  2143154.7 |
  2144371.4 |
  2145588.1 |
  2146804.7 |
  2148021.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=301.9% of algo (FFI overhead may distort results)

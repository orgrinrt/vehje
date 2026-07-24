# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (9.91 us) is smaller than the fastest variant's own run-to-run std-dev (41.22 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_real_null_sink)

The baseline abi_sink_real_null_sink is the fastest (2.13 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.5% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_real_null_sink) is the fastest** at 2130365.8 ns median
- Spread: 1.00x (fastest 2130365.8 ns, slowest 2140279.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2169260ns | 2133143ns | 2129104ns | 2131995ns | 2245236ns | +0.99% |
| abi_sink_real_batched_sink_decode | 2138426ns | 2139728ns | 2120706ns | 2138922ns | 2146542ns | -0.45% |
| abi_sink_real_null_sink | 2148038ns | 2132916ns | 2115532ns | 2129921ns | 2191466ns | base |
| abi_sink_real_per_record_sink | 2148289ns | 2142722ns | 2131492ns | 2140927ns | 2167730ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2166606ns | 2126659ns | 2242222ns | +0.99% | 0.000 |
| abi_sink_real_batched_sink_decode | 2135875ns | 2118147ns | 2144001ns | -0.44% | 0.000 |
| abi_sink_real_null_sink | 2145320ns | 2113087ns | 2188345ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2145794ns | 2129031ns | 2165134ns | +0.02% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 42897.5 | 2159647.9 | 2166605.9 | 0 |
| abi_sink_real_batched_sink_decode | 40369.4 | 2136776.5 | 2135874.5 | n/a |
| abi_sink_real_null_sink | 43898.6 | 2146831.2 | 2145320.3 | n/a |
| abi_sink_real_per_record_sink | 39107.9 | 2245888.6 | 2145794.1 | 10 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_null_sink; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.2% |
| abi_sink_real_batched_sink_decode | 0.000 | 98.9% |
| abi_sink_real_null_sink | 0.000 | 99.2% |
| abi_sink_real_per_record_sink | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2169260ns | 2169260ns | +0.99% |
| abi_sink_real_batched_sink_decode | 2138426ns | 2138426ns | -0.45% |
| abi_sink_real_null_sink | 2148038ns | 2148038ns | base |
| abi_sink_real_per_record_sink | 2148289ns | 2148289ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2130366ns | base | --- | [2117250, 2188345] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2130650ns | no significant difference | [-7840, +67967]ns | [2126946, 2242222] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_real_batched_sink_decode | 2137234ns | no significant difference | [-58830, +19984]ns | [2126389, 2144001] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_real_per_record_sink | 2140279ns | no significant difference | [-25980, +20334]ns | [2131969, 2165134] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2131518ns | -0.1% | +0.7% | +0.7% |
| 2 | 2235422ns | +5.0% | -5.2% | -2.3% |
| 3 | 2113087ns | +1.2% | +1.1% | +1.0% |
| 4 | 2121413ns | +0.2% | +0.8% | +0.9% |
| 5 | 2141268ns | -0.7% | -0.0% | -0.0% |
| 6 | 2129214ns | +0.1% | +0.3% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.196 | ok |
| abi_sink_real_batched_sink_decode | -0.400 | moderate- |
| abi_sink_real_null_sink | -0.315 | moderate- |
| abi_sink_real_per_record_sink | -0.121 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 1/6, lost 4/6
- **abi_sink_real_batched_sink_decode**: won 1/6, lost 4/6
- **abi_sink_real_per_record_sink**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6525895.4ns | 2166605.9ns | 301.2% | HIGH |
| abi_sink_real_batched_sink_decode | 6452095.5ns | 2135874.5ns | 302.1% | HIGH |
| abi_sink_real_null_sink | 6529541.2ns | 2145320.3ns | 304.4% | HIGH |
| abi_sink_real_per_record_sink | 6577831.0ns | 2145794.1ns | 306.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2126659.2-2242221.9 ns)
  2126659.2 |########################################
  2132437.3 |
  2138215.5 |##########
  2143993.6 |
  2149771.7 |
  2155549.9 |
  2161328.0 |
  2167106.1 |
  2172884.3 |
  2178662.4 |
  2184440.6 |
  2190218.7 |
  2195996.8 |
  2201775.0 |
  2207553.1 |
  2213331.2 |
  2219109.4 |
  2224887.5 |
  2230665.6 |
  2236443.8 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2118147.1-2144001.0 ns)
  2118147.1 |########################################
  2119439.8 |
  2120732.5 |
  2122025.2 |
  2123317.9 |
  2124610.6 |
  2125903.3 |
  2127196.0 |
  2128488.7 |
  2129781.4 |
  2131074.1 |
  2132366.8 |
  2133659.5 |########################################
  2134952.2 |########################################
  2136244.9 |
  2137537.6 |########################################
  2138830.3 |
  2140123.0 |########################################
  2141415.7 |
  2142708.4 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2113086.7-2188345.2 ns)
  2113086.7 |####################
  2116849.6 |
  2120612.6 |####################
  2124375.5 |
  2128138.4 |########################################
  2131901.3 |
  2135664.2 |
  2139427.2 |####################
  2143190.1 |
  2146953.0 |
  2150716.0 |
  2154478.9 |
  2158241.8 |
  2162004.7 |
  2165767.7 |
  2169530.6 |
  2173293.5 |
  2177056.4 |
  2180819.4 |
  2184582.3 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2129030.8-2165134.0 ns)
  2129030.8 |####################
  2130836.0 |
  2132641.1 |
  2134446.3 |####################
  2136251.4 |
  2138056.6 |
  2139861.7 |########################################
  2141666.9 |
  2143472.1 |
  2145277.2 |####################
  2147082.4 |
  2148887.5 |
  2150692.7 |
  2152497.8 |
  2154303.0 |
  2156108.2 |
  2157913.3 |
  2159718.5 |
  2161523.6 |
  2163328.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=301.6% of algo (FFI overhead may distort results)

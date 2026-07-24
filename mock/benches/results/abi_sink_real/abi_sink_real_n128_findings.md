# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.14 ms and 2.15 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2139906.2 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2139906.2 ns, slowest 2151705.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2141982ns | 2142717ns | 2120079ns | 2141127ns | 2154215ns | -0.72% |
| abi_sink_real_batched_sink_decode | 2151620ns | 2154697ns | 2128064ns | 2151313ns | 2163859ns | -0.27% |
| abi_sink_real_null_sink | 2157441ns | 2153125ns | 2144571ns | 2152948ns | 2170616ns | base |
| abi_sink_real_per_record_sink | 2172452ns | 2146666ns | 2140050ns | 2146186ns | 2228053ns | +0.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2139141ns | 2117664ns | 2151242ns | -0.71% | 0.000 |
| abi_sink_real_batched_sink_decode | 2148671ns | 2125600ns | 2160748ns | -0.27% | 0.000 |
| abi_sink_real_null_sink | 2154452ns | 2141797ns | 2167526ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2169429ns | 2137351ns | 2224595ns | +0.70% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 52126.5 | 2138696.3 | 2139140.7 | n/a |
| abi_sink_real_batched_sink_decode | 55654.6 | 2150132.7 | 2148671.0 | 0 |
| abi_sink_real_null_sink | 54370.6 | 2154584.8 | 2154451.7 | n/a |
| abi_sink_real_per_record_sink | 54718.3 | 2170802.9 | 2169428.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.0% |
| abi_sink_real_batched_sink_decode | 0.000 | 98.4% |
| abi_sink_real_null_sink | 0.000 | 98.5% |
| abi_sink_real_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2141982ns | 2141982ns | -0.72% |
| abi_sink_real_batched_sink_decode | 2151620ns | 2151620ns | -0.27% |
| abi_sink_real_null_sink | 2157441ns | 2157441ns | base |
| abi_sink_real_per_record_sink | 2172452ns | 2172452ns | +0.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2150142ns | base | --- | [2145687, 2167526] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2139906ns | -8889.4ns (-0.4%) | [-35130, -1914]ns | [2126274, 2151242] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_real_batched_sink_decode | 2151706ns | no significant difference | [-28155, +9725]ns | [2133560, 2160748] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_real_per_record_sink | 2143810ns | no significant difference | [-13054, +62880]ns | [2139882, 2224595] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2172976ns | -2.5% | -2.2% | +5.9% |
| 2 | 2162076ns | -0.4% | +0.2% | -0.9% |
| 3 | 2149578ns | -0.4% | +0.3% | -0.2% |
| 4 | 2150452ns | -0.0% | -0.4% | -0.2% |
| 5 | 2141797ns | -0.1% | +0.6% | -0.2% |
| 6 | 2149832ns | -0.7% | -0.1% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.318 | moderate- |
| abi_sink_real_batched_sink_decode | -0.395 | moderate- |
| abi_sink_real_null_sink | 0.374 | moderate+ |
| abi_sink_real_per_record_sink | -0.037 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 5/6, lost 0/6
- **abi_sink_real_batched_sink_decode**: won 2/6, lost 3/6
- **abi_sink_real_per_record_sink**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6471053.6ns | 2139140.7ns | 302.5% | HIGH |
| abi_sink_real_batched_sink_decode | 6507631.1ns | 2148671.0ns | 302.9% | HIGH |
| abi_sink_real_null_sink | 6519870.4ns | 2154451.7ns | 302.6% | HIGH |
| abi_sink_real_per_record_sink | 6571162.2ns | 2169428.9ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2117664.2-2151242.1 ns)
  2117664.2 |########################################
  2119343.1 |
  2121022.0 |
  2122700.9 |
  2124379.8 |
  2126058.7 |
  2127737.6 |
  2129416.5 |
  2131095.4 |
  2132774.3 |
  2134453.2 |########################################
  2136132.0 |
  2137810.9 |########################################
  2139489.8 |########################################
  2141168.7 |
  2142847.6 |
  2144526.5 |
  2146205.4 |
  2147884.3 |
  2149563.2 |########################################
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2125599.6-2160747.5 ns)
  2125599.6 |####################
  2127357.0 |
  2129114.4 |
  2130871.8 |
  2132629.2 |
  2134386.6 |
  2136144.0 |
  2137901.4 |
  2139658.8 |
  2141416.2 |####################
  2143173.5 |
  2144930.9 |
  2146688.3 |####################
  2148445.7 |
  2150203.1 |
  2151960.5 |
  2153717.9 |########################################
  2155475.3 |
  2157232.7 |
  2158990.1 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2141796.7-2167526.0 ns)
  2141796.7 |#############
  2143083.2 |
  2144369.6 |
  2145656.1 |
  2146942.6 |
  2148229.0 |
  2149515.5 |########################################
  2150802.0 |
  2152088.4 |
  2153374.9 |
  2154661.4 |
  2155947.8 |
  2157234.3 |
  2158520.7 |
  2159807.2 |
  2161093.7 |#############
  2162380.1 |
  2163666.6 |
  2164953.1 |
  2166239.5 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2137351.2-2224594.5 ns)
  2137351.2 |#############
  2141713.4 |########################################
  2146075.5 |#############
  2150437.7 |
  2154799.9 |
  2159162.0 |
  2163524.2 |
  2167886.4 |
  2172248.5 |
  2176610.7 |
  2180972.9 |
  2185335.0 |
  2189697.2 |
  2194059.4 |
  2198421.5 |
  2202783.7 |
  2207145.9 |
  2211508.0 |
  2215870.2 |
  2220232.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=302.7% of algo (FFI overhead may distort results)

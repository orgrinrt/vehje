# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.13 ms and 2.15 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2133602.9 ns median (-0.4% vs baseline)
- Spread: 1.01x (fastest 2133602.9 ns, slowest 2147173.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2137505ns | 2136145ns | 2130994ns | 2135414ns | 2143895ns | -0.38% |
| abi_sink_real_batched_sink_decode | 2190224ns | 2141970ns | 2127128ns | 2140603ns | 2296203ns | +2.08% |
| abi_sink_real_null_sink | 2145562ns | 2144226ns | 2138235ns | 2143139ns | 2152861ns | base |
| abi_sink_real_per_record_sink | 2157436ns | 2149709ns | 2133705ns | 2148863ns | 2182162ns | +0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2134988ns | 2128524ns | 2141364ns | -0.37% | 0.000 |
| abi_sink_real_batched_sink_decode | 2187659ns | 2124577ns | 2293583ns | +2.09% | 0.000 |
| abi_sink_real_null_sink | 2142972ns | 2135829ns | 2150154ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2154682ns | 2130608ns | 2179212ns | +0.55% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 39430.8 | 2134830.8 | 2134988.4 | n/a |
| abi_sink_real_batched_sink_decode | 57181.4 | 2159347.9 | 2187658.9 | n/a |
| abi_sink_real_null_sink | 40345.6 | 2146791.8 | 2142971.5 | n/a |
| abi_sink_real_per_record_sink | 44096.5 | 2155243.5 | 2154681.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink_decode; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.6% |
| abi_sink_real_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_real_null_sink | 0.000 | 99.2% |
| abi_sink_real_per_record_sink | 0.000 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2137505ns | 2137505ns | -0.38% |
| abi_sink_real_batched_sink_decode | 2190224ns | 2190224ns | +2.08% |
| abi_sink_real_null_sink | 2145562ns | 2145562ns | base |
| abi_sink_real_per_record_sink | 2157436ns | 2157436ns | +0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2141645ns | base | --- | [2137116, 2150154] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2133603ns | no significant difference | [-20156, +264]ns | [2129998, 2141364] | no | 0.6563 | 0.2188 | 0 |
| abi_sink_real_batched_sink_decode | 2139422ns | no significant difference | [-11343, +148085]ns | [2129971, 2293583] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_real_per_record_sink | 2147173ns | no significant difference | [-6831, +35002]ns | [2137660, 2179212] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2152592ns | -1.0% | +13.5% | +2.7% |
| 2 | 2138403ns | -0.1% | +0.3% | +0.4% |
| 3 | 2140134ns | +0.2% | -0.7% | +0.2% |
| 4 | 2143156ns | -0.2% | -0.3% | -0.6% |
| 5 | 2135829ns | -0.2% | -0.0% | +0.6% |
| 6 | 2147715ns | -0.9% | -0.2% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | 0.280 | moderate+ |
| abi_sink_real_batched_sink_decode | 0.000 | ok |
| abi_sink_real_null_sink | -0.342 | moderate- |
| abi_sink_real_per_record_sink | 0.035 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 5/6, lost 1/6
- **abi_sink_real_batched_sink_decode**: won 3/6, lost 2/6
- **abi_sink_real_per_record_sink**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6445786.1ns | 2134988.4ns | 301.9% | HIGH |
| abi_sink_real_batched_sink_decode | 6499274.0ns | 2187658.9ns | 297.1% | HIGH |
| abi_sink_real_null_sink | 6485837.0ns | 2142971.5ns | 302.7% | HIGH |
| abi_sink_real_per_record_sink | 6512115.5ns | 2154681.7ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2128523.8-2141364.0 ns)
  2128523.8 |########################################
  2129165.8 |
  2129807.8 |
  2130449.8 |
  2131091.8 |########################################
  2131733.8 |########################################
  2132375.9 |
  2133017.9 |
  2133659.9 |
  2134301.9 |
  2134943.9 |########################################
  2135585.9 |
  2136227.9 |
  2136869.9 |
  2137511.9 |
  2138154.0 |
  2138796.0 |########################################
  2139438.0 |
  2140080.0 |
  2140722.0 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2124577.1-2293582.9 ns)
  2124577.1 |####################
  2133027.4 |########################################
  2141477.7 |########################################
  2149928.0 |
  2158378.3 |
  2166828.5 |
  2175278.8 |
  2183729.1 |
  2192179.4 |
  2200629.7 |
  2209080.0 |
  2217530.3 |
  2225980.6 |
  2234430.9 |
  2242881.2 |
  2251331.4 |
  2259781.7 |
  2268232.0 |
  2276682.3 |
  2285132.6 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2135828.8-2150154.0 ns)
  2135828.8 |########################################
  2136545.1 |
  2137261.3 |
  2137977.6 |########################################
  2138693.8 |
  2139410.1 |
  2140126.3 |########################################
  2140842.6 |
  2141558.9 |
  2142275.1 |
  2142991.4 |########################################
  2143707.6 |
  2144423.9 |
  2145140.1 |
  2145856.4 |
  2146572.7 |
  2147288.9 |########################################
  2148005.2 |
  2148721.4 |
  2149437.7 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2130607.5-2179212.3 ns)
  2130607.5 |####################
  2133037.7 |
  2135468.0 |
  2137898.2 |
  2140328.5 |
  2142758.7 |####################
  2145188.9 |####################
  2147619.2 |########################################
  2150049.4 |
  2152479.7 |
  2154909.9 |
  2157340.1 |
  2159770.4 |
  2162200.6 |
  2164630.9 |
  2167061.1 |
  2169491.3 |
  2171921.6 |
  2174351.8 |
  2176782.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=302.1% of algo (FFI overhead may distort results)

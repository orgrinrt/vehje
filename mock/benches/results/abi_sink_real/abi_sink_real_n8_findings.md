# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_sink_real_batched_sink shows alternating (throttle bounce) (autocorr -0.56)

abi_sink_real_batched_sink's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.7% of the fastest

All 4 variants sit between 2.13 ms and 2.15 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2130688.5 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2130688.5 ns, slowest 2146383.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2133538ns | 2133263ns | 2126959ns | 2132289ns | 2138702ns | -0.75% |
| abi_sink_real_batched_sink_decode | 2153021ns | 2136546ns | 2128954ns | 2135515ns | 2191315ns | +0.16% |
| abi_sink_real_null_sink | 2149678ns | 2144677ns | 2138946ns | 2143334ns | 2164560ns | base |
| abi_sink_real_per_record_sink | 2148678ns | 2148998ns | 2140852ns | 2146921ns | 2155227ns | -0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2131020ns | 2124534ns | 2136198ns | -0.75% | 0.000 |
| abi_sink_real_batched_sink_decode | 2150376ns | 2126302ns | 2188631ns | +0.16% | 0.000 |
| abi_sink_real_null_sink | 2147023ns | 2136279ns | 2161847ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2146038ns | 2138115ns | 2152574ns | -0.05% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 39803.4 | 2129808.8 | 2131020.1 | n/a |
| abi_sink_real_batched_sink_decode | 42494.8 | 2144487.0 | 2150375.6 | 0 |
| abi_sink_real_null_sink | 43445.3 | 2207375.9 | 2147022.6 | n/a |
| abi_sink_real_per_record_sink | 41836.9 | 2147444.2 | 2146038.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.7% |
| abi_sink_real_batched_sink_decode | 0.000 | 99.6% |
| abi_sink_real_null_sink | 0.000 | 99.2% |
| abi_sink_real_per_record_sink | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2133538ns | 2133538ns | -0.75% |
| abi_sink_real_batched_sink_decode | 2153021ns | 2153021ns | +0.16% |
| abi_sink_real_null_sink | 2149678ns | 2149678ns | base |
| abi_sink_real_per_record_sink | 2148678ns | 2148678ns | -0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2142004ns | base | --- | [2137217, 2161847] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2130689ns | -13256.5ns (-0.6%) | [-28432, -6319]ns | [2126174, 2136198] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_real_batched_sink_decode | 2133963ns | no significant difference | [-16443, +31970]ns | [2128532, 2188631] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_real_per_record_sink | 2146383ns | no significant difference | [-15680, +10490]ns | [2139156, 2152574] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2138155ns | -0.5% | -0.3% | +0.1% |
| 2 | 2177045ns | -1.8% | +2.9% | -1.2% |
| 3 | 2146649ns | -0.8% | -0.7% | +0.1% |
| 4 | 2143301ns | -0.4% | -0.8% | -0.2% |
| 5 | 2140707ns | -0.8% | -0.2% | +0.1% |
| 6 | 2136279ns | -0.2% | +0.1% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.561 | HIGH- (thermal bounce) |
| abi_sink_real_batched_sink_decode | -0.253 | moderate- |
| abi_sink_real_null_sink | -0.161 | ok |
| abi_sink_real_per_record_sink | -0.189 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 6/6, lost 0/6
- **abi_sink_real_batched_sink_decode**: won 4/6, lost 1/6
- **abi_sink_real_per_record_sink**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6434221.4ns | 2131020.1ns | 301.9% | HIGH |
| abi_sink_real_batched_sink_decode | 6480340.7ns | 2150375.6ns | 301.4% | HIGH |
| abi_sink_real_null_sink | 6563274.1ns | 2147022.6ns | 305.7% | HIGH |
| abi_sink_real_per_record_sink | 6482133.8ns | 2146038.0ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2124533.8-2136197.5 ns)
  2124533.8 |########################################
  2125117.0 |
  2125700.2 |
  2126283.4 |
  2126866.5 |
  2127449.7 |########################################
  2128032.9 |
  2128616.1 |
  2129199.3 |
  2129782.5 |########################################
  2130365.6 |
  2130948.8 |########################################
  2131532.0 |
  2132115.2 |
  2132698.4 |
  2133281.6 |
  2133864.8 |
  2134447.9 |
  2135031.1 |########################################
  2135614.3 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2126302.5-2188631.5 ns)
  2126302.5 |####################
  2129418.9 |########################################
  2132535.4 |
  2135651.8 |########################################
  2138768.3 |
  2141884.7 |
  2145001.2 |
  2148117.6 |
  2151234.1 |
  2154350.5 |
  2157467.0 |
  2160583.4 |
  2163699.9 |
  2166816.3 |
  2169932.8 |
  2173049.2 |
  2176165.7 |
  2179282.1 |
  2182398.6 |
  2185515.0 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2136279.2-2161846.7 ns)
  2136279.2 |########################################
  2137557.6 |########################################
  2138836.0 |
  2140114.3 |########################################
  2141392.7 |
  2142671.1 |########################################
  2143949.5 |
  2145227.8 |
  2146506.2 |########################################
  2147784.6 |
  2149063.0 |
  2150341.3 |
  2151619.7 |
  2152898.1 |
  2154176.5 |
  2155454.8 |
  2156733.2 |
  2158011.6 |
  2159290.0 |
  2160568.3 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2138114.6-2152574.3 ns)
  2138114.6 |########################################
  2138837.6 |
  2139560.6 |########################################
  2140283.6 |
  2141006.5 |
  2141729.5 |
  2142452.5 |
  2143175.5 |########################################
  2143898.5 |
  2144621.5 |
  2145344.5 |
  2146067.5 |
  2146790.4 |
  2147513.4 |
  2148236.4 |
  2148959.4 |########################################
  2149682.4 |
  2150405.4 |########################################
  2151128.4 |
  2151851.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=301.8% of algo (FFI overhead may distort results)

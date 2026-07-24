# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5.37 us) is smaller than the fastest variant's own run-to-run std-dev (6.10 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 4 variants sit between 2.15 ms and 2.15 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink_decode** at 2146565.6 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 2146565.6 ns, slowest 2151933.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2168015ns | 2149475ns | 2144652ns | 2148521ns | 2208939ns | +0.58% |
| abi_sink_scatter_batched_sink_decode | 2149912ns | 2149221ns | 2139592ns | 2148563ns | 2157094ns | -0.26% |
| abi_sink_scatter_null_sink | 2155571ns | 2152327ns | 2143942ns | 2151545ns | 2167423ns | base |
| abi_sink_scatter_per_record_sink | 2159567ns | 2154859ns | 2148030ns | 2154172ns | 2173429ns | +0.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2165214ns | 2141956ns | 2205861ns | +0.58% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2147237ns | 2136997ns | 2154327ns | -0.26% | 0.000 |
| abi_sink_scatter_null_sink | 2152804ns | 2141244ns | 2164631ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2156683ns | 2145508ns | 2170294ns | +0.18% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 46428.6 | 2155170.1 | 2165213.6 | 0 |
| abi_sink_scatter_batched_sink_decode | 41946.0 | 2148236.5 | 2147237.4 | n/a |
| abi_sink_scatter_null_sink | 46843.5 | 2152851.4 | 2152804.1 | n/a |
| abi_sink_scatter_per_record_sink | 47322.7 | 2156394.2 | 2156682.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink_decode; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.5% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.6% |
| abi_sink_scatter_null_sink | 0.000 | 99.4% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2168015ns | 2168015ns | +0.58% |
| abi_sink_scatter_batched_sink_decode | 2149912ns | 2149912ns | -0.26% |
| abi_sink_scatter_null_sink | 2155571ns | 2155571ns | base |
| abi_sink_scatter_per_record_sink | 2159567ns | 2159567ns | +0.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2149540ns | base | --- | [2144241, 2164631] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2146841ns | no significant difference | [-9995, +49164]ns | [2142939, 2205861] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_scatter_batched_sink_decode | 2146566ns | no significant difference | [-22020, +10086]ns | [2140820, 2154327] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_scatter_per_record_sink | 2151933ns | no significant difference | [-16810, +21539]ns | [2147821, 2170294] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2157114ns | -0.6% | -0.9% | -0.3% |
| 2 | 2141244ns | +1.2% | +0.5% | +0.4% |
| 3 | 2172149ns | +3.4% | -1.1% | -1.2% |
| 4 | 2150271ns | -0.3% | -0.3% | +1.1% |
| 5 | 2147238ns | +0.1% | +0.4% | +0.9% |
| 6 | 2148809ns | -0.3% | -0.2% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.120 | ok |
| abi_sink_scatter_batched_sink_decode | -0.433 | moderate- |
| abi_sink_scatter_null_sink | -0.493 | moderate- |
| abi_sink_scatter_per_record_sink | 0.103 | ok |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 3/6, lost 3/6
- **abi_sink_scatter_batched_sink_decode**: won 4/6, lost 2/6
- **abi_sink_scatter_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6524785.9ns | 2165213.6ns | 301.3% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6488005.6ns | 2147237.4ns | 302.2% | HIGH |
| abi_sink_scatter_null_sink | 6509822.0ns | 2152804.1ns | 302.4% | HIGH |
| abi_sink_scatter_per_record_sink | 6519520.1ns | 2156682.7ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2141955.8-2205860.6 ns)
  2141955.8 |########################################
  2145151.0 |
  2148346.3 |#############
  2151541.5 |
  2154736.8 |
  2157932.0 |
  2161127.2 |
  2164322.5 |#############
  2167517.7 |
  2170713.0 |
  2173908.2 |
  2177103.4 |
  2180298.7 |
  2183493.9 |
  2186689.2 |
  2189884.4 |
  2193079.6 |
  2196274.9 |
  2199470.1 |
  2202665.4 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2136997.1-2154327.0 ns)
  2136997.1 |########################################
  2137863.6 |
  2138730.1 |
  2139596.6 |
  2140463.1 |
  2141329.6 |
  2142196.1 |
  2143062.6 |
  2143929.1 |########################################
  2144795.6 |########################################
  2145662.1 |
  2146528.6 |
  2147395.1 |########################################
  2148261.6 |
  2149128.1 |
  2149994.6 |
  2150861.1 |
  2151727.6 |
  2152594.1 |########################################
  2153460.6 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2141244.2-2164631.3 ns)
  2141244.2 |########################################
  2142413.6 |
  2143582.9 |
  2144752.3 |
  2145921.6 |
  2147091.0 |########################################
  2148260.3 |########################################
  2149429.7 |########################################
  2150599.0 |
  2151768.4 |
  2152937.8 |
  2154107.1 |
  2155276.5 |
  2156445.8 |########################################
  2157615.2 |
  2158784.5 |
  2159953.9 |
  2161123.2 |
  2162292.6 |
  2163461.9 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2145508.3-2170293.5 ns)
  2145508.3 |####################
  2146747.6 |
  2147986.8 |
  2149226.1 |########################################
  2150465.3 |
  2151704.6 |
  2152943.9 |####################
  2154183.1 |
  2155422.4 |
  2156661.7 |
  2157900.9 |
  2159140.2 |
  2160379.4 |
  2161618.7 |
  2162858.0 |
  2164097.2 |
  2165336.5 |
  2166575.8 |####################
  2167815.0 |
  2169054.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.6% of algo (FFI overhead may distort results)

# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_real_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_real_null_sink has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_real_batched_sink at 2.14 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_sink_real_batched_sink shows alternating (throttle bounce) (autocorr -0.77)

abi_sink_real_batched_sink's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.14 ms and 2.15 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2143728.5 ns median (-0.4% vs baseline)
- Spread: 1.00x (fastest 2143728.5 ns, slowest 2152385.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2146121ns | 2146345ns | 2136268ns | 2145398ns | 2152132ns | -0.73% |
| abi_sink_real_batched_sink_decode | 2169990ns | 2150956ns | 2144689ns | 2149311ns | 2213659ns | +0.37% |
| abi_sink_real_null_sink | 2161928ns | 2154930ns | 2140097ns | 2150721ns | 2189653ns | base |
| abi_sink_real_per_record_sink | 2163089ns | 2147787ns | 2140550ns | 2146151ns | 2199765ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2143603ns | 2133795ns | 2149668ns | -0.73% | 0.000 |
| abi_sink_real_batched_sink_decode | 2167374ns | 2142259ns | 2210791ns | +0.37% | 0.000 |
| abi_sink_real_null_sink | 2159302ns | 2137549ns | 2186769ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2160527ns | 2138184ns | 2196952ns | +0.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 39400.2 | 2142842.8 | 2143603.4 | n/a |
| abi_sink_real_batched_sink_decode | 42933.0 | 2161254.7 | 2167374.4 | 0 |
| abi_sink_real_null_sink | 44311.2 | 2156680.6 | 2159302.4 | n/a |
| abi_sink_real_per_record_sink | 41239.5 | 2159329.0 | 2160526.8 | 3 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.5% |
| abi_sink_real_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_real_null_sink | 0.000 | 99.1% |
| abi_sink_real_per_record_sink | 0.000 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2146121ns | 2146121ns | -0.73% |
| abi_sink_real_batched_sink_decode | 2169990ns | 2169990ns | +0.37% |
| abi_sink_real_null_sink | 2161928ns | 2161928ns | base |
| abi_sink_real_per_record_sink | 2163089ns | 2163089ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2152386ns | base | --- | [2138753, 2186769] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2143728ns | no significant difference | [-45773, +10915]ns | [2137414, 2149668] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_real_batched_sink_decode | 2148500ns | no significant difference | [-6452, +29998]ns | [2142832, 2210791] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_real_per_record_sink | 2145335ns | no significant difference | [-13351, +19672]ns | [2139293, 2196952] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2214605ns | -3.2% | +2.5% | +1.1% |
| 2 | 2158932ns | -0.7% | -0.5% | -0.9% |
| 3 | 2150971ns | -0.5% | -0.1% | -0.4% |
| 4 | 2137549ns | +0.7% | +0.2% | +0.0% |
| 5 | 2153800ns | -0.9% | -0.1% | -0.3% |
| 6 | 2139957ns | +0.4% | +0.2% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.766 | HIGH- (thermal bounce) |
| abi_sink_real_batched_sink_decode | -0.027 | ok |
| abi_sink_real_null_sink | 0.097 | ok |
| abi_sink_real_per_record_sink | -0.063 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 4/6, lost 2/6
- **abi_sink_real_batched_sink_decode**: won 2/6, lost 3/6
- **abi_sink_real_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6469203.7ns | 2143603.4ns | 301.8% | HIGH |
| abi_sink_real_batched_sink_decode | 6638457.8ns | 2167374.4ns | 306.3% | HIGH |
| abi_sink_real_null_sink | 6563802.4ns | 2159302.4ns | 304.0% | HIGH |
| abi_sink_real_per_record_sink | 6614716.9ns | 2160526.8ns | 306.2% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2133795.4-2149668.2 ns)
  2133795.4 |########################################
  2134589.0 |
  2135382.7 |
  2136176.3 |
  2136970.0 |
  2137763.6 |
  2138557.2 |
  2139350.9 |
  2140144.5 |
  2140938.1 |########################################
  2141731.8 |
  2142525.4 |########################################
  2143319.1 |
  2144112.7 |########################################
  2144906.3 |
  2145700.0 |
  2146493.6 |
  2147287.2 |########################################
  2148080.9 |
  2148874.5 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2142259.2-2210791.2 ns)
  2142259.2 |########################################
  2145685.8 |########################################
  2149112.4 |####################
  2152539.0 |
  2155965.6 |
  2159392.2 |
  2162818.8 |
  2166245.4 |
  2169672.0 |
  2173098.6 |
  2176525.2 |
  2179951.8 |
  2183378.4 |
  2186805.0 |
  2190231.6 |
  2193658.2 |
  2197084.8 |
  2200511.4 |
  2203938.0 |
  2207364.6 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2137549.2-2186768.8 ns)
  2137549.2 |########################################
  2140010.2 |
  2142471.2 |
  2144932.1 |
  2147393.1 |
  2149854.1 |####################
  2152315.1 |####################
  2154776.0 |
  2157237.0 |####################
  2159698.0 |
  2162159.0 |
  2164620.0 |
  2167080.9 |
  2169541.9 |
  2172002.9 |
  2174463.9 |
  2176924.8 |
  2179385.8 |
  2181846.8 |
  2184307.8 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2138183.8-2196952.5 ns)
  2138183.8 |########################################
  2141122.2 |####################
  2144060.7 |
  2146999.1 |####################
  2149937.5 |
  2152876.0 |####################
  2155814.4 |
  2158752.8 |
  2161691.3 |
  2164629.7 |
  2167568.1 |
  2170506.6 |
  2173445.0 |
  2176383.5 |
  2179321.9 |
  2182260.3 |
  2185198.8 |
  2188137.2 |
  2191075.6 |
  2194014.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=302.0% of algo (FFI overhead may distort results)

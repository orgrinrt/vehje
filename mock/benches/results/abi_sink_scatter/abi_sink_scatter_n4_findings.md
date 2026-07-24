# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_scatter_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_scatter_null_sink has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_scatter_batched_sink_decode at 2.14 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.14 ms and 2.15 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink_decode** at 2141943.2 ns median (-0.4% vs baseline)
- Spread: 1.00x (fastest 2141943.2 ns, slowest 2150286.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2206724ns | 2146131ns | 2134861ns | 2143925ns | 2336854ns | +2.23% |
| abi_sink_scatter_batched_sink_decode | 2145588ns | 2144631ns | 2136342ns | 2143553ns | 2153263ns | -0.60% |
| abi_sink_scatter_null_sink | 2158562ns | 2153053ns | 2139957ns | 2149793ns | 2181018ns | base |
| abi_sink_scatter_per_record_sink | 2214379ns | 2146591ns | 2141147ns | 2145497ns | 2354318ns | +2.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2203711ns | 2132247ns | 2333501ns | +2.23% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2142907ns | 2133691ns | 2150528ns | -0.59% | 0.000 |
| abi_sink_scatter_null_sink | 2155714ns | 2137296ns | 2177954ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2211494ns | 2138780ns | 2350952ns | +2.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 49363.4 | 2155409.5 | 2203711.4 | n/a |
| abi_sink_scatter_batched_sink_decode | 45112.6 | 2141006.6 | 2142907.2 | n/a |
| abi_sink_scatter_null_sink | 47084.8 | 2153013.5 | 2155713.5 | n/a |
| abi_sink_scatter_per_record_sink | 51264.6 | 2241505.1 | 2211493.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.5% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_scatter_null_sink | 0.000 | 99.2% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2206724ns | 2206724ns | +2.23% |
| abi_sink_scatter_batched_sink_decode | 2145588ns | 2145588ns | -0.60% |
| abi_sink_scatter_null_sink | 2158562ns | 2158562ns | base |
| abi_sink_scatter_per_record_sink | 2214379ns | 2214379ns | +2.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2150286ns | base | --- | [2138901, 2177954] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2143299ns | no significant difference | [-17556, +162120]ns | [2134334, 2333501] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_scatter_batched_sink_decode | 2141943ns | no significant difference | [-41704, +6642]ns | [2136250, 2150528] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_scatter_per_record_sink | 2143772ns | no significant difference | [-13086, +184095]ns | [2139756, 2350952] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2146342ns | +0.3% | -0.3% | -0.4% |
| 2 | 2159488ns | -0.8% | -1.2% | -0.6% |
| 3 | 2196419ns | +14.5% | -2.6% | +16.2% |
| 4 | 2140506ns | +0.2% | +0.2% | +0.0% |
| 5 | 2154230ns | -0.8% | +0.0% | -0.6% |
| 6 | 2137296ns | -0.2% | +0.4% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.220 | moderate- |
| abi_sink_scatter_batched_sink_decode | 0.468 | moderate+ |
| abi_sink_scatter_null_sink | -0.193 | ok |
| abi_sink_scatter_per_record_sink | -0.232 | moderate- |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 3/6, lost 3/6
- **abi_sink_scatter_batched_sink_decode**: won 3/6, lost 2/6
- **abi_sink_scatter_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6604704.1ns | 2203711.4ns | 299.7% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6474035.4ns | 2142907.2ns | 302.1% | HIGH |
| abi_sink_scatter_null_sink | 6514049.2ns | 2155713.5ns | 302.2% | HIGH |
| abi_sink_scatter_per_record_sink | 6701067.6ns | 2211493.6ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2132247.1-2333501.2 ns)
  2132247.1 |########################################
  2142309.8 |##########################
  2152372.5 |
  2162435.2 |
  2172497.9 |
  2182560.6 |
  2192623.3 |
  2202686.1 |
  2212748.8 |
  2222811.5 |
  2232874.2 |
  2242936.9 |
  2252999.6 |
  2263062.3 |
  2273125.0 |
  2283187.7 |
  2293250.4 |
  2303313.1 |
  2313375.8 |
  2323438.5 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2133690.8-2150528.4 ns)
  2133690.8 |####################
  2134532.7 |
  2135374.6 |
  2136216.4 |
  2137058.3 |
  2137900.2 |
  2138742.1 |########################################
  2139583.9 |
  2140425.8 |
  2141267.7 |
  2142109.6 |
  2142951.5 |
  2143793.3 |
  2144635.2 |####################
  2145477.1 |####################
  2146319.0 |
  2147160.8 |
  2148002.7 |
  2148844.6 |
  2149686.5 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2137296.2-2177953.5 ns)
  2137296.2 |########################################
  2139329.1 |########################################
  2141361.9 |
  2143394.8 |
  2145427.7 |########################################
  2147460.5 |
  2149493.4 |
  2151526.3 |
  2153559.1 |########################################
  2155592.0 |
  2157624.9 |########################################
  2159657.7 |
  2161690.6 |
  2163723.5 |
  2165756.3 |
  2167789.2 |
  2169822.1 |
  2171854.9 |
  2173887.8 |
  2175920.7 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2138779.6-2350952.5 ns)
  2138779.6 |########################################
  2149388.2 |
  2159996.9 |
  2170605.5 |
  2181214.2 |
  2191822.8 |
  2202431.5 |
  2213040.1 |
  2223648.8 |
  2234257.4 |
  2244866.0 |
  2255474.7 |
  2266083.3 |
  2276692.0 |
  2287300.6 |
  2297909.3 |
  2308517.9 |
  2319126.6 |
  2329735.2 |
  2340343.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.3% of algo (FFI overhead may distort results)

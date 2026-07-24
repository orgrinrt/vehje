# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_scatter_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_scatter_null_sink has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_scatter_batched_sink at 2.13 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (16.11 us) is smaller than the fastest variant's own run-to-run std-dev (260.09 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_scatter_batched_sink vs stability leader abi_sink_scatter_per_record_sink (+0% speed for 6.3x steadier)

abi_sink_scatter_batched_sink is fastest (2.13 ms, CV 12.2%); abi_sink_scatter_per_record_sink gives up 0.5% median for 6.3x lower variance (CV 1.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### abi_sink_scatter_batched_sink_decode is inconsistent: worst-20% is 1.5x its best-20%

abi_sink_scatter_batched_sink_decode's best 20% of batches run at 2.12 ms but its worst 20% at 3.26 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.8% of the fastest

All 4 variants sit between 2.13 ms and 2.15 ms - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink** at 2132163.0 ns median (-0.7% vs baseline)
- Spread: 1.01x (fastest 2132163.0 ns, slowest 2148270.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2264305ns | 2134999ns | 2123749ns | 2132782ns | 2531868ns | +0.62% |
| abi_sink_scatter_batched_sink_decode | 2509607ns | 2139949ns | 2124297ns | 2137015ns | 3261150ns | +11.52% |
| abi_sink_scatter_null_sink | 2250360ns | 2151216ns | 2122152ns | 2147117ns | 2469329ns | base |
| abi_sink_scatter_per_record_sink | 2165975ns | 2144911ns | 2131294ns | 2141669ns | 2219773ns | -3.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2260688ns | 2121320ns | 2526407ns | +0.61% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2506020ns | 2121808ns | 3255732ns | +11.52% | 0.000 |
| abi_sink_scatter_null_sink | 2247070ns | 2119468ns | 2465054ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2162984ns | 2128680ns | 2216131ns | -3.74% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 61871.1 | 2297990.6 | 2260687.9 | n/a |
| abi_sink_scatter_batched_sink_decode | 72621.6 | 2577485.3 | 2506020.0 | n/a |
| abi_sink_scatter_null_sink | 60277.8 | 2218033.9 | 2247070.1 | n/a |
| abi_sink_scatter_per_record_sink | 53885.1 | 2161678.3 | 2162984.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_null_sink; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.4% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_scatter_null_sink | 0.000 | 98.7% |
| abi_sink_scatter_per_record_sink | 0.000 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2264305ns | 2264305ns | +0.62% |
| abi_sink_scatter_batched_sink_decode | 2509607ns | 2509607ns | +11.52% |
| abi_sink_scatter_null_sink | 2250360ns | 2250360ns | base |
| abi_sink_scatter_per_record_sink | 2165975ns | 2165975ns | -3.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2148270ns | base | --- | [2127886, 2465054] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2132163ns | no significant difference | [-31502, +83547]ns | [2123494, 2526407] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_scatter_batched_sink_decode | 2137182ns | no significant difference | [-13135, +790679]ns | [2125146, 3255732] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_scatter_per_record_sink | 2142155ns | no significant difference | [-248923, +4353]ns | [2130666, 2216131] | no | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2140198ns | -0.5% | -0.2% | -0.2% |
| 2 | 2136304ns | -0.5% | -0.4% | -0.4% |
| 3 | 2119468ns | +0.1% | +0.1% | +0.6% |
| 4 | 2156342ns | -0.9% | -0.9% | -0.4% |
| 5 | 2672585ns | +6.2% | +59.0% | -18.1% |
| 6 | 2257523ns | -1.9% | +0.2% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.109 | ok |
| abi_sink_scatter_batched_sink_decode | -0.175 | ok |
| abi_sink_scatter_null_sink | 0.015 | ok |
| abi_sink_scatter_per_record_sink | 0.394 | moderate+ |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 4/6, lost 1/6
- **abi_sink_scatter_batched_sink_decode**: won 3/6, lost 3/6
- **abi_sink_scatter_per_record_sink**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6876342.2ns | 2260687.9ns | 304.2% | HIGH |
| abi_sink_scatter_batched_sink_decode | 7757358.4ns | 2506020.0ns | 309.5% | HIGH |
| abi_sink_scatter_null_sink | 6772367.1ns | 2247070.1ns | 301.4% | HIGH |
| abi_sink_scatter_per_record_sink | 6562335.6ns | 2162984.1ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2121320.4-2526407.0 ns)
  2121320.4 |########################################
  2141574.7 |
  2161829.1 |
  2182083.4 |
  2202337.7 |##########
  2222592.1 |
  2242846.4 |
  2263100.7 |
  2283355.1 |
  2303609.4 |
  2323863.7 |
  2344118.1 |
  2364372.4 |
  2384626.7 |
  2404881.1 |
  2425135.4 |
  2445389.7 |
  2465644.1 |
  2485898.4 |
  2506152.7 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2121807.9-3255732.5 ns)
  2121807.9 |########################################
  2178504.1 |
  2235200.4 |##########
  2291896.6 |
  2348592.8 |
  2405289.0 |
  2461985.3 |
  2518681.5 |
  2575377.7 |
  2632074.0 |
  2688770.2 |
  2745466.4 |
  2802162.7 |
  2858858.9 |
  2915555.1 |
  2972251.4 |
  3028947.6 |
  3085643.8 |
  3142340.0 |
  3199036.3 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2119467.5-2465054.0 ns)
  2119467.5 |########################################
  2136746.8 |####################
  2154026.1 |####################
  2171305.5 |
  2188584.8 |
  2205864.1 |
  2223143.4 |
  2240422.8 |####################
  2257702.1 |
  2274981.4 |
  2292260.7 |
  2309540.0 |
  2326819.4 |
  2344098.7 |
  2361378.0 |
  2378657.3 |
  2395936.7 |
  2413216.0 |
  2430495.3 |
  2447774.6 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2128680.4-2216131.2 ns)
  2128680.4 |########################################
  2133052.9 |####################
  2137425.5 |
  2141798.0 |
  2146170.6 |####################
  2150543.1 |
  2154915.7 |
  2159288.2 |
  2163660.7 |
  2168033.3 |
  2172405.8 |
  2176778.4 |
  2181150.9 |
  2185523.5 |####################
  2189896.0 |
  2194268.5 |
  2198641.1 |
  2203013.6 |
  2207386.2 |
  2211758.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: CV=31.2% (high variance, measurements may be unstable)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.2% of algo (FFI overhead may distort results)

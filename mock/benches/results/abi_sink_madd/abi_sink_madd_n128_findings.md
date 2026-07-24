# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_batched_sink at 2.65 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (39.50 us) is smaller than the fastest variant's own run-to-run std-dev (58.91 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.5% of the fastest

All 4 variants sit between 2.65 ms and 2.69 ms - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink** at 2653495.8 ns median (-1.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 2653495.8 ns, slowest 2692995.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2679295ns | 2656016ns | 2644465ns | 2653075ns | 2736040ns | -1.88% |
| abi_sink_madd_batched_sink_decode | 2664528ns | 2662943ns | 2657770ns | 2661368ns | 2672649ns | -2.42% |
| abi_sink_madd_null_sink | 2730601ns | 2695523ns | 2680841ns | 2694412ns | 2809765ns | base |
| abi_sink_madd_per_record_sink | 2699047ns | 2678022ns | 2664931ns | 2675141ns | 2751965ns | -1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2676658ns | 2641986ns | 2733186ns | -1.88% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2662026ns | 2655409ns | 2670082ns | -2.42% | 0.000 |
| abi_sink_madd_null_sink | 2727969ns | 2678382ns | 2806980ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2696250ns | 2662459ns | 2748807ns | -1.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 43230.9 | 2671442.6 | 2676658.2 | 0 |
| abi_sink_madd_batched_sink_decode | 40198.5 | 2660192.9 | 2662025.6 | n/a |
| abi_sink_madd_null_sink | 43180.2 | 2698409.1 | 2727968.6 | n/a |
| abi_sink_madd_per_record_sink | 46167.5 | 2712955.5 | 2696249.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.6% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_madd_null_sink | 0.000 | 98.1% |
| abi_sink_madd_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2679295ns | 2679295ns | -1.88% |
| abi_sink_madd_batched_sink_decode | 2664528ns | 2664528ns | -2.42% |
| abi_sink_madd_null_sink | 2730601ns | 2730601ns | base |
| abi_sink_madd_per_record_sink | 2699047ns | 2699047ns | -1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2692995ns | base | --- | [2683931, 2806980] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2653496ns | -37630.0ns (-1.4%) | [-80802, -35500]ns | [2643293, 2733186] | YES | 0.0469 | 0.0313 | 0 |
| abi_sink_madd_batched_sink_decode | 2660427ns | -28655.6ns (-1.1%) | [-147396, -21778]ns | [2655568, 2670082] | YES | 0.0469 | 0.0313 | 0 |
| abi_sink_madd_per_record_sink | 2675308ns | no significant difference | [-69411, +1752]ns | [2664634, 2748807] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2695313ns | -1.4% | -1.5% | -1.0% |
| 2 | 2689480ns | -1.3% | -1.2% | -1.0% |
| 3 | 2917665ns | -3.8% | -8.7% | -3.7% |
| 4 | 2690678ns | -1.4% | -1.0% | -0.3% |
| 5 | 2696295ns | -1.9% | -0.8% | -1.1% |
| 6 | 2678382ns | -1.4% | -0.9% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.182 | ok |
| abi_sink_madd_batched_sink_decode | -0.121 | ok |
| abi_sink_madd_null_sink | -0.239 | moderate- |
| abi_sink_madd_per_record_sink | -0.235 | moderate- |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 6/6, lost 0/6
- **abi_sink_madd_batched_sink_decode**: won 6/6, lost 0/6
- **abi_sink_madd_per_record_sink**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8084230.3ns | 2676658.2ns | 302.0% | HIGH |
| abi_sink_madd_batched_sink_decode | 8025324.1ns | 2662025.6ns | 301.5% | HIGH |
| abi_sink_madd_null_sink | 8155228.0ns | 2727968.6ns | 298.9% | HIGH |
| abi_sink_madd_per_record_sink | 8160736.3ns | 2696249.7ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2641985.8-2733186.0 ns)
  2641985.8 |########################################
  2646545.8 |
  2651105.8 |########################################
  2655665.8 |####################
  2660225.8 |
  2664785.9 |
  2669345.9 |
  2673905.9 |
  2678465.9 |
  2683025.9 |
  2687585.9 |
  2692145.9 |
  2696705.9 |
  2701266.0 |
  2705826.0 |
  2710386.0 |
  2714946.0 |
  2719506.0 |
  2724066.0 |
  2728626.0 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2655409.2-2670081.9 ns)
  2655409.2 |########################################
  2656142.8 |
  2656876.5 |
  2657610.1 |
  2658343.7 |####################
  2659077.4 |
  2659811.0 |
  2660544.6 |
  2661278.3 |
  2662011.9 |####################
  2662745.6 |
  2663479.2 |
  2664212.8 |####################
  2664946.5 |
  2665680.1 |
  2666413.7 |
  2667147.4 |
  2667881.0 |
  2668614.6 |
  2669348.3 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2678381.7-2806979.8 ns)
  2678381.7 |####################
  2684811.6 |########################################
  2691241.5 |########################################
  2697671.4 |
  2704101.3 |
  2710531.2 |
  2716961.1 |
  2723391.0 |
  2729820.9 |
  2736250.8 |
  2742680.8 |
  2749110.7 |
  2755540.6 |
  2761970.5 |
  2768400.4 |
  2774830.3 |
  2781260.2 |
  2787690.1 |
  2794120.0 |
  2800549.9 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2662459.2-2748806.7 ns)
  2662459.2 |####################
  2666776.6 |########################################
  2671093.9 |
  2675411.3 |
  2679728.7 |####################
  2684046.1 |
  2688363.4 |####################
  2692680.8 |
  2696998.2 |
  2701315.6 |
  2705632.9 |
  2709950.3 |
  2714267.7 |
  2718585.0 |
  2722902.4 |
  2727219.8 |
  2731537.2 |
  2735854.5 |
  2740171.9 |
  2744489.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.4% of algo (FFI overhead may distort results)

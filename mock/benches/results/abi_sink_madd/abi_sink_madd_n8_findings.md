# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_batched_sink_decode at 2.66 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 1.0% of the fastest

All 4 variants sit between 2.66 ms and 2.69 ms - a 1.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink_decode** at 2661926.6 ns median (-1.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 2661926.6 ns, slowest 2689173.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2673981ns | 2670856ns | 2657192ns | 2666569ns | 2693494ns | -0.67% |
| abi_sink_madd_batched_sink_decode | 2662493ns | 2664639ns | 2643266ns | 2663981ns | 2669875ns | -1.09% |
| abi_sink_madd_null_sink | 2691913ns | 2691811ns | 2684198ns | 2690499ns | 2697891ns | base |
| abi_sink_madd_per_record_sink | 2675005ns | 2675031ns | 2665402ns | 2672344ns | 2683798ns | -0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2671372ns | 2654761ns | 2690978ns | -0.67% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2659873ns | 2640625ns | 2667354ns | -1.09% | 0.000 |
| abi_sink_madd_null_sink | 2689277ns | 2681433ns | 2695260ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2672398ns | 2662824ns | 2681275ns | -0.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 42036.4 | 2672855.8 | 2671371.9 | n/a |
| abi_sink_madd_batched_sink_decode | 41899.0 | 2659767.4 | 2659873.3 | n/a |
| abi_sink_madd_null_sink | 42242.1 | 2689537.2 | 2689277.4 | n/a |
| abi_sink_madd_per_record_sink | 41060.6 | 2675768.7 | 2672397.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink_decode; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.0% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_madd_null_sink | 0.000 | 98.2% |
| abi_sink_madd_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2673981ns | 2673981ns | -0.67% |
| abi_sink_madd_batched_sink_decode | 2662493ns | 2662493ns | -1.09% |
| abi_sink_madd_null_sink | 2691913ns | 2691913ns | base |
| abi_sink_madd_per_record_sink | 2675005ns | 2675005ns | -0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2689173ns | base | --- | [2683399, 2695260] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2668014ns | no significant difference | [-28792, +999]ns | [2655124, 2690978] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_madd_batched_sink_decode | 2661927ns | -24969.4ns (-0.9%) | [-43771, -19472]ns | [2650339, 2667354] | YES | 0.0469 | 0.0313 | 0 |
| abi_sink_madd_per_record_sink | 2672368ns | -17310.2ns (-0.6%) | [-27783, -5545]ns | [2663552, 2681275] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2693330ns | -1.0% | -1.2% | -0.4% |
| 2 | 2685364ns | -1.1% | -0.9% | -0.8% |
| 3 | 2690609ns | -0.8% | -0.9% | -0.4% |
| 4 | 2681433ns | -1.0% | -0.7% | -0.0% |
| 5 | 2687738ns | +0.9% | -0.7% | -0.8% |
| 6 | 2697191ns | -1.0% | -2.1% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.244 | moderate- |
| abi_sink_madd_batched_sink_decode | -0.292 | moderate- |
| abi_sink_madd_null_sink | -0.198 | ok |
| abi_sink_madd_per_record_sink | -0.226 | moderate- |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 5/6, lost 1/6
- **abi_sink_madd_batched_sink_decode**: won 6/6, lost 0/6
- **abi_sink_madd_per_record_sink**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8060652.9ns | 2671371.9ns | 301.7% | HIGH |
| abi_sink_madd_batched_sink_decode | 8026083.8ns | 2659873.3ns | 301.7% | HIGH |
| abi_sink_madd_null_sink | 8103814.9ns | 2689277.4ns | 301.3% | HIGH |
| abi_sink_madd_per_record_sink | 8063562.9ns | 2672397.9ns | 301.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2654760.8-2690978.1 ns)
  2654760.8 |########################################
  2656571.7 |
  2658382.5 |
  2660193.4 |
  2662004.3 |
  2663815.1 |
  2665626.0 |####################
  2667436.9 |####################
  2669247.7 |####################
  2671058.6 |
  2672869.5 |
  2674680.3 |
  2676491.2 |
  2678302.0 |
  2680112.9 |
  2681923.8 |
  2683734.6 |
  2685545.5 |
  2687356.4 |
  2689167.2 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2640624.6-2667354.0 ns)
  2640624.6 |########################################
  2641961.1 |
  2643297.5 |
  2644634.0 |
  2645970.5 |
  2647306.9 |
  2648643.4 |
  2649979.9 |
  2651316.3 |
  2652652.8 |
  2653989.3 |
  2655325.7 |
  2656662.2 |
  2657998.7 |
  2659335.1 |########################################
  2660671.6 |########################################
  2662008.1 |########################################
  2663344.5 |
  2664681.0 |########################################
  2666017.5 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2681432.9-2695260.4 ns)
  2681432.9 |########################################
  2682124.3 |
  2682815.6 |
  2683507.0 |
  2684198.4 |
  2684889.8 |########################################
  2685581.1 |
  2686272.5 |
  2686963.9 |
  2687655.3 |########################################
  2688346.6 |
  2689038.0 |
  2689729.4 |
  2690420.8 |########################################
  2691112.1 |
  2691803.5 |
  2692494.9 |
  2693186.3 |########################################
  2693877.6 |
  2694569.0 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2662824.2-2681274.6 ns)
  2662824.2 |########################################
  2663746.7 |########################################
  2664669.2 |########################################
  2665591.8 |
  2666514.3 |
  2667436.8 |
  2668359.3 |
  2669281.8 |
  2670204.4 |
  2671126.9 |
  2672049.4 |
  2672971.9 |
  2673894.4 |
  2674817.0 |
  2675739.5 |
  2676662.0 |
  2677584.5 |
  2678507.0 |
  2679429.6 |########################################
  2680352.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.7% of algo (FFI overhead may distort results)

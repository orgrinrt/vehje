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

- **Fastest: abi_sink_madd_batched_sink_decode** at 2660319.0 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2660319.0 ns, slowest 2688061.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2753686ns | 2667687ns | 2662709ns | 2666848ns | 2929430ns | +2.45% |
| abi_sink_madd_batched_sink_decode | 2665338ns | 2662868ns | 2649455ns | 2662071ns | 2678180ns | -0.84% |
| abi_sink_madd_null_sink | 2687895ns | 2690686ns | 2670799ns | 2689442ns | 2694123ns | base |
| abi_sink_madd_per_record_sink | 2701874ns | 2681874ns | 2670248ns | 2678035ns | 2753444ns | +0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2750818ns | 2659842ns | 2926237ns | +2.44% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2662716ns | 2646745ns | 2675496ns | -0.84% | 0.000 |
| abi_sink_madd_null_sink | 2685235ns | 2668242ns | 2691415ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2699091ns | 2667481ns | 2750472ns | +0.52% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 49552.1 | 2830947.8 | 2750818.1 | n/a |
| abi_sink_madd_batched_sink_decode | 42310.9 | 2662933.5 | 2662716.1 | 0 |
| abi_sink_madd_null_sink | 41169.5 | 2684582.8 | 2685235.0 | n/a |
| abi_sink_madd_per_record_sink | 45856.4 | 2761518.7 | 2699090.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink_decode; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.3% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_madd_null_sink | 0.000 | 98.5% |
| abi_sink_madd_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2753686ns | 2753686ns | +2.45% |
| abi_sink_madd_batched_sink_decode | 2665338ns | 2665338ns | -0.84% |
| abi_sink_madd_null_sink | 2687895ns | 2687895ns | base |
| abi_sink_madd_per_record_sink | 2701874ns | 2701874ns | +0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2688061ns | base | --- | [2676228, 2691415] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2665039ns | no significant difference | [-26913, +236319]ns | [2661179, 2926237] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_madd_batched_sink_decode | 2660319ns | -26357.9ns (-1.0%) | [-36592, -4606]ns | [2652333, 2675496] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_madd_per_record_sink | 2679214ns | no significant difference | [-19912, +68420]ns | [2667586, 2750472] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2689054ns | +9.2% | -0.0% | +4.6% |
| 2 | 2684214ns | -0.8% | -0.8% | -0.6% |
| 3 | 2690782ns | +8.4% | -1.2% | -0.9% |
| 4 | 2668242ns | -0.2% | -0.3% | +0.5% |
| 5 | 2687069ns | -0.7% | -1.5% | +0.1% |
| 6 | 2692048ns | -1.2% | -1.2% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.331 | moderate- |
| abi_sink_madd_batched_sink_decode | 0.105 | ok |
| abi_sink_madd_null_sink | -0.318 | moderate- |
| abi_sink_madd_per_record_sink | -0.102 | ok |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 4/6, lost 2/6
- **abi_sink_madd_batched_sink_decode**: won 5/6, lost 0/6
- **abi_sink_madd_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8432857.3ns | 2750818.1ns | 306.6% | HIGH |
| abi_sink_madd_batched_sink_decode | 8033493.5ns | 2662716.1ns | 301.7% | HIGH |
| abi_sink_madd_null_sink | 8095405.8ns | 2685235.0ns | 301.5% | HIGH |
| abi_sink_madd_per_record_sink | 8205847.7ns | 2699090.9ns | 304.0% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2659842.1-2926236.7 ns)
  2659842.1 |########################################
  2673161.8 |
  2686481.6 |
  2699801.3 |
  2713121.0 |
  2726440.8 |
  2739760.5 |
  2753080.2 |
  2766399.9 |
  2779719.7 |
  2793039.4 |
  2806359.1 |
  2819678.9 |
  2832998.6 |
  2846318.3 |
  2859638.1 |
  2872957.8 |
  2886277.5 |
  2899597.2 |
  2912917.0 |##########
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2646744.6-2675496.3 ns)
  2646744.6 |####################
  2648182.2 |
  2649619.8 |
  2651057.4 |
  2652494.9 |
  2653932.5 |
  2655370.1 |
  2656807.7 |####################
  2658245.3 |
  2659682.9 |########################################
  2661120.5 |
  2662558.0 |####################
  2663995.6 |
  2665433.2 |
  2666870.8 |
  2668308.4 |
  2669746.0 |
  2671183.5 |
  2672621.1 |
  2674058.7 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2668242.5-2691415.2 ns)
  2668242.5 |########################################
  2669401.1 |
  2670559.8 |
  2671718.4 |
  2672877.0 |
  2674035.7 |
  2675194.3 |
  2676352.9 |
  2677511.6 |
  2678670.2 |
  2679828.9 |
  2680987.5 |
  2682146.1 |
  2683304.8 |########################################
  2684463.4 |
  2685622.0 |
  2686780.7 |########################################
  2687939.3 |########################################
  2689097.9 |
  2690256.6 |########################################
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2667481.2-2750472.3 ns)
  2667481.2 |########################################
  2671630.8 |
  2675780.3 |####################
  2679929.9 |####################
  2684079.4 |
  2688229.0 |####################
  2692378.5 |
  2696528.1 |
  2700677.6 |
  2704827.2 |
  2708976.8 |
  2713126.3 |
  2717275.9 |
  2721425.4 |
  2725575.0 |
  2729724.5 |
  2733874.1 |
  2738023.6 |
  2742173.2 |
  2746322.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.3% of algo (FFI overhead may distort results)

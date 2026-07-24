# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_batched_sink at 2.68 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.14 us) is smaller than the fastest variant's own run-to-run std-dev (112.94 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_madd_batched_sink vs stability leader abi_sink_madd_batched_sink_decode (+0% speed for 10.9x steadier)

abi_sink_madd_batched_sink is fastest (2.68 ms, CV 4.2%); abi_sink_madd_batched_sink_decode gives up 0.1% median for 10.9x lower variance (CV 0.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.2% of the fastest

All 4 variants sit between 2.68 ms and 2.69 ms - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink** at 2681060.2 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 2681060.2 ns, slowest 2687203.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2745778ns | 2683599ns | 2667680ns | 2680723ns | 2882409ns | +1.24% |
| abi_sink_madd_batched_sink_decode | 2688608ns | 2687086ns | 2676650ns | 2685183ns | 2699724ns | -0.87% |
| abi_sink_madd_null_sink | 2712147ns | 2689800ns | 2682317ns | 2688166ns | 2763034ns | base |
| abi_sink_madd_per_record_sink | 2690941ns | 2689053ns | 2667357ns | 2688019ns | 2707117ns | -0.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2742909ns | 2665162ns | 2878933ns | +1.23% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2686038ns | 2674082ns | 2697168ns | -0.87% | 0.000 |
| abi_sink_madd_null_sink | 2709501ns | 2679713ns | 2760311ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2688359ns | 2664946ns | 2704385ns | -0.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 50219.6 | 2731986.9 | 2742908.6 | n/a |
| abi_sink_madd_batched_sink_decode | 41371.0 | 2686415.3 | 2686037.7 | n/a |
| abi_sink_madd_null_sink | 41935.7 | 2706452.2 | 2709501.4 | n/a |
| abi_sink_madd_per_record_sink | 41055.4 | 2688662.4 | 2688358.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_per_record_sink; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.4% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_madd_null_sink | 0.000 | 99.2% |
| abi_sink_madd_per_record_sink | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2745778ns | 2745778ns | +1.24% |
| abi_sink_madd_batched_sink_decode | 2688608ns | 2688608ns | -0.87% |
| abi_sink_madd_null_sink | 2712147ns | 2712147ns | base |
| abi_sink_madd_per_record_sink | 2690941ns | 2690941ns | -0.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2687203ns | base | --- | [2680990, 2760311] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2681060ns | no significant difference | [-25510, +133956]ns | [2668733, 2878933] | no | 0.6563 | 0.2188 | 0 |
| abi_sink_madd_batched_sink_decode | 2684499ns | no significant difference | [-74642, +6321]ns | [2676446, 2697168] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_madd_per_record_sink | 2686460ns | no significant difference | [-71918, +11565]ns | [2674230, 2704385] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2707210ns | +10.1% | -0.0% | +0.1% |
| 2 | 2691501ns | -0.2% | -0.4% | -0.3% |
| 3 | 2679713ns | -0.5% | +0.3% | +0.7% |
| 4 | 2682905ns | -0.2% | +0.2% | +0.1% |
| 5 | 2682266ns | -0.4% | -0.1% | -0.6% |
| 6 | 2813412ns | -1.3% | -5.0% | -4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.022 | ok |
| abi_sink_madd_batched_sink_decode | -0.033 | ok |
| abi_sink_madd_null_sink | -0.055 | ok |
| abi_sink_madd_per_record_sink | -0.082 | ok |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 5/6, lost 1/6
- **abi_sink_madd_batched_sink_decode**: won 3/6, lost 2/6
- **abi_sink_madd_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8255350.1ns | 2742908.6ns | 301.0% | HIGH |
| abi_sink_madd_batched_sink_decode | 8102970.6ns | 2686037.7ns | 301.7% | HIGH |
| abi_sink_madd_null_sink | 8141672.1ns | 2709501.4ns | 300.5% | HIGH |
| abi_sink_madd_per_record_sink | 8107338.3ns | 2688358.7ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2665162.5-2878932.7 ns)
  2665162.5 |########################################
  2675851.0 |########################################
  2686539.5 |
  2697228.0 |
  2707916.5 |
  2718605.0 |
  2729293.6 |
  2739982.1 |
  2750670.6 |
  2761359.1 |
  2772047.6 |####################
  2782736.1 |
  2793424.6 |
  2804113.1 |
  2814801.6 |
  2825490.2 |
  2836178.7 |
  2846867.2 |
  2857555.7 |
  2868244.2 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2674082.5-2697168.3 ns)
  2674082.5 |####################
  2675236.8 |
  2676391.1 |
  2677545.4 |
  2678699.7 |####################
  2679854.0 |
  2681008.2 |####################
  2682162.5 |
  2683316.8 |
  2684471.1 |
  2685625.4 |
  2686779.7 |########################################
  2687934.0 |
  2689088.3 |
  2690242.6 |
  2691396.8 |
  2692551.1 |
  2693705.4 |
  2694859.7 |
  2696014.0 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2679713.3-2760311.5 ns)
  2679713.3 |########################################
  2683743.2 |
  2687773.1 |#############
  2691803.0 |
  2695832.9 |
  2699862.8 |
  2703892.7 |#############
  2707922.7 |
  2711952.6 |
  2715982.5 |
  2720012.4 |
  2724042.3 |
  2728072.2 |
  2732102.1 |
  2736132.0 |
  2740161.9 |
  2744191.8 |
  2748221.7 |
  2752251.6 |
  2756281.5 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2664945.8-2704385.4 ns)
  2664945.8 |########################################
  2666917.8 |
  2668889.8 |
  2670861.7 |
  2672833.7 |
  2674805.7 |
  2676777.7 |
  2678749.7 |
  2680721.6 |
  2682693.6 |########################################
  2684665.6 |########################################
  2686637.6 |########################################
  2688609.6 |
  2690581.5 |
  2692553.5 |
  2694525.5 |
  2696497.5 |
  2698469.5 |########################################
  2700441.4 |
  2702413.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.4% of algo (FFI overhead may distort results)

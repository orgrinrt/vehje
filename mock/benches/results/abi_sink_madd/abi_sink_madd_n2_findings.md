# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.72 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_batched_sink at 2.71 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (12.54 us) is smaller than the fastest variant's own run-to-run std-dev (30.77 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.5% of the fastest

All 4 variants sit between 2.71 ms and 2.72 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink** at 2706659.6 ns median (-0.5% vs baseline)
- Spread: 1.00x (fastest 2706659.6 ns, slowest 2719203.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2719633ns | 2709233ns | 2697931ns | 2706430ns | 2750289ns | -0.23% |
| abi_sink_madd_batched_sink_decode | 2810151ns | 2712113ns | 2699798ns | 2709302ns | 3016600ns | +3.09% |
| abi_sink_madd_null_sink | 2725953ns | 2721957ns | 2710205ns | 2718913ns | 2744387ns | base |
| abi_sink_madd_per_record_sink | 2715531ns | 2718667ns | 2700783ns | 2714033ns | 2725152ns | -0.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2717004ns | 2695324ns | 2747578ns | -0.23% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2807276ns | 2697178ns | 3013160ns | +3.08% | 0.000 |
| abi_sink_madd_null_sink | 2723319ns | 2707678ns | 2741710ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2712964ns | 2698272ns | 2722539ns | -0.38% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 44331.0 | 2723130.3 | 2717003.5 | 0 |
| abi_sink_madd_batched_sink_decode | 50469.4 | 2800843.9 | 2807275.8 | n/a |
| abi_sink_madd_null_sink | 42715.7 | 2722238.0 | 2723318.9 | n/a |
| abi_sink_madd_per_record_sink | 41644.2 | 2713003.3 | 2712964.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.6% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_madd_null_sink | 0.000 | 99.1% |
| abi_sink_madd_per_record_sink | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2719633ns | 2719633ns | -0.23% |
| abi_sink_madd_batched_sink_decode | 2810151ns | 2810151ns | +3.09% |
| abi_sink_madd_null_sink | 2725953ns | 2725953ns | base |
| abi_sink_madd_per_record_sink | 2715531ns | 2715531ns | -0.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2719203ns | base | --- | [2709043, 2741710] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2706660ns | no significant difference | [-20458, +14680]ns | [2696773, 2747578] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_madd_batched_sink_decode | 2709482ns | no significant difference | [-19088, +280262]ns | [2699185, 3013160] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_madd_per_record_sink | 2716090ns | no significant difference | [-41447, +12701]ns | [2700264, 2722539] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2710409ns | -0.4% | -0.3% | +0.6% |
| 2 | 2707678ns | +0.1% | +0.8% | +0.3% |
| 3 | 2725302ns | -0.6% | -0.6% | -0.8% |
| 4 | 2719316ns | -0.9% | -0.8% | -0.1% |
| 5 | 2758118ns | +1.0% | +19.5% | -2.2% |
| 6 | 2719090ns | -0.5% | -0.3% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.338 | moderate- |
| abi_sink_madd_batched_sink_decode | -0.261 | moderate- |
| abi_sink_madd_null_sink | -0.074 | ok |
| abi_sink_madd_per_record_sink | -0.239 | moderate- |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 4/6, lost 1/6
- **abi_sink_madd_batched_sink_decode**: won 4/6, lost 2/6
- **abi_sink_madd_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8296275.3ns | 2717003.5ns | 305.3% | HIGH |
| abi_sink_madd_batched_sink_decode | 8410346.5ns | 2807275.8ns | 299.6% | HIGH |
| abi_sink_madd_null_sink | 8208215.5ns | 2723318.9ns | 301.4% | HIGH |
| abi_sink_madd_per_record_sink | 8179241.7ns | 2712964.3ns | 301.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2695323.7-2747578.2 ns)
  2695323.7 |########################################
  2697936.4 |########################################
  2700549.1 |
  2703161.9 |########################################
  2705774.6 |########################################
  2708387.3 |########################################
  2711000.0 |
  2713612.8 |
  2716225.5 |
  2718838.2 |
  2721450.9 |
  2724063.6 |
  2726676.4 |
  2729289.1 |
  2731901.8 |
  2734514.5 |
  2737127.3 |
  2739740.0 |
  2742352.7 |
  2744965.4 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2697177.9-3013160.4 ns)
  2697177.9 |########################################
  2712977.0 |
  2728776.1 |##########
  2744575.3 |
  2760374.4 |
  2776173.5 |
  2791972.6 |
  2807771.8 |
  2823570.9 |
  2839370.0 |
  2855169.1 |
  2870968.3 |
  2886767.4 |
  2902566.5 |
  2918365.6 |
  2934164.8 |
  2949963.9 |
  2965763.0 |
  2981562.1 |
  2997361.3 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2707677.5-2741710.4 ns)
  2707677.5 |####################
  2709379.1 |####################
  2711080.8 |
  2712782.4 |
  2714484.1 |
  2716185.7 |
  2717887.4 |########################################
  2719589.0 |
  2721290.7 |
  2722992.3 |
  2724694.0 |####################
  2726395.6 |
  2728097.2 |
  2729798.9 |
  2731500.5 |
  2733202.2 |
  2734903.8 |
  2736605.5 |
  2738307.1 |
  2740008.8 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2698272.1-2722539.0 ns)
  2698272.1 |####################
  2699485.4 |
  2700698.8 |
  2701912.1 |####################
  2703125.5 |
  2704338.8 |
  2705552.2 |
  2706765.5 |
  2707978.9 |
  2709192.2 |
  2710405.5 |
  2711618.9 |
  2712832.2 |
  2714045.6 |
  2715258.9 |########################################
  2716472.3 |####################
  2717685.6 |
  2718899.0 |
  2720112.3 |
  2721325.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.3% of algo (FFI overhead may distort results)

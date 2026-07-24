# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.68 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_per_record_sink at 2.66 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.7% of the fastest

All 4 variants sit between 2.66 ms and 2.68 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_per_record_sink** at 2659024.0 ns median (-0.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 2659024.0 ns, slowest 2678781.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2676603ns | 2663602ns | 2655068ns | 2660911ns | 2710910ns | -0.20% |
| abi_sink_madd_batched_sink_decode | 2666770ns | 2666547ns | 2658165ns | 2664406ns | 2674620ns | -0.56% |
| abi_sink_madd_null_sink | 2681893ns | 2681308ns | 2673485ns | 2679864ns | 2689140ns | base |
| abi_sink_madd_per_record_sink | 2664821ns | 2661688ns | 2651418ns | 2660476ns | 2678041ns | -0.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2673981ns | 2652466ns | 2708205ns | -0.20% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2664145ns | 2655586ns | 2671954ns | -0.57% | 0.000 |
| abi_sink_madd_null_sink | 2679337ns | 2670775ns | 2686621ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2662167ns | 2648831ns | 2675351ns | -0.64% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 41616.5 | 2670908.0 | 2673981.3 | 0 |
| abi_sink_madd_batched_sink_decode | 41378.0 | 2665423.4 | 2664145.1 | 0 |
| abi_sink_madd_null_sink | 40174.9 | 2679255.9 | 2679337.3 | n/a |
| abi_sink_madd_per_record_sink | 41007.6 | 2663813.9 | 2662166.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_per_record_sink; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.5% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.4% |
| abi_sink_madd_null_sink | 0.000 | 98.9% |
| abi_sink_madd_per_record_sink | 0.000 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2676603ns | 2676603ns | -0.20% |
| abi_sink_madd_batched_sink_decode | 2666770ns | 2666770ns | -0.56% |
| abi_sink_madd_null_sink | 2681893ns | 2681893ns | base |
| abi_sink_madd_per_record_sink | 2664821ns | 2664821ns | -0.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2678782ns | base | --- | [2672609, 2686621] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2661074ns | no significant difference | [-29526, +33513]ns | [2652665, 2708205] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_madd_batched_sink_decode | 2663969ns | -13692.7ns (-0.5%) | [-27139, -4745]ns | [2656512, 2671954] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_madd_per_record_sink | 2659024ns | -18345.2ns (-0.7%) | [-30808, -2358]ns | [2652126, 2675351] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2685984ns | -0.7% | -1.1% | -0.7% |
| 2 | 2678954ns | -1.0% | -0.3% | +0.2% |
| 3 | 2687258ns | -1.2% | -0.9% | -1.4% |
| 4 | 2670775ns | +2.3% | -0.5% | -0.4% |
| 5 | 2678609ns | +0.2% | -0.5% | -0.9% |
| 6 | 2674444ns | -0.8% | -0.0% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.034 | ok |
| abi_sink_madd_batched_sink_decode | -0.199 | ok |
| abi_sink_madd_null_sink | -0.311 | moderate- |
| abi_sink_madd_per_record_sink | -0.190 | ok |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 4/6, lost 2/6
- **abi_sink_madd_batched_sink_decode**: won 5/6, lost 0/6
- **abi_sink_madd_per_record_sink**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8048903.2ns | 2673981.3ns | 301.0% | HIGH |
| abi_sink_madd_batched_sink_decode | 8037070.6ns | 2664145.1ns | 301.7% | HIGH |
| abi_sink_madd_null_sink | 8073659.2ns | 2679337.3ns | 301.3% | HIGH |
| abi_sink_madd_per_record_sink | 8033315.3ns | 2662166.9ns | 301.8% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2652465.8-2708205.4 ns)
  2652465.8 |########################################
  2655252.8 |
  2658039.8 |
  2660826.7 |
  2663613.7 |
  2666400.7 |#############
  2669187.7 |
  2671974.7 |
  2674761.6 |
  2677548.6 |
  2680335.6 |
  2683122.6 |#############
  2685909.6 |
  2688696.5 |
  2691483.5 |
  2694270.5 |
  2697057.5 |
  2699844.5 |
  2702631.4 |
  2705418.4 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2655585.8-2671953.8 ns)
  2655585.8 |########################################
  2656404.2 |
  2657222.6 |########################################
  2658041.0 |
  2658859.4 |
  2659677.8 |
  2660496.2 |
  2661314.6 |
  2662133.0 |
  2662951.4 |########################################
  2663769.8 |########################################
  2664588.2 |
  2665406.6 |
  2666225.0 |
  2667043.4 |
  2667861.8 |
  2668680.2 |
  2669498.6 |########################################
  2670317.0 |
  2671135.4 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2670774.6-2686621.2 ns)
  2670774.6 |########################################
  2671566.9 |
  2672359.3 |
  2673151.6 |
  2673943.9 |########################################
  2674736.3 |
  2675528.6 |
  2676320.9 |
  2677113.3 |
  2677905.6 |########################################
  2678697.9 |########################################
  2679490.3 |
  2680282.6 |
  2681074.9 |
  2681867.3 |
  2682659.6 |
  2683451.9 |
  2684244.3 |
  2685036.6 |
  2685828.9 |########################################
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2648831.2-2675350.6 ns)
  2648831.2 |########################################
  2650157.2 |
  2651483.1 |
  2652809.1 |
  2654135.1 |########################################
  2655461.0 |
  2656787.0 |########################################
  2658113.0 |
  2659439.0 |########################################
  2660764.9 |
  2662090.9 |
  2663416.9 |
  2664742.8 |########################################
  2666068.8 |
  2667394.8 |
  2668720.8 |
  2670046.7 |
  2671372.7 |
  2672698.7 |
  2674024.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.6% of algo (FFI overhead may distort results)

# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_batched_sink_decode at 2.67 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_sink_madd_per_record_sink shows alternating (throttle bounce) (autocorr -0.65)

abi_sink_madd_per_record_sink's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.67 ms and 2.69 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink_decode** at 2672542.3 ns median (-0.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.01x (fastest 2672542.3 ns, slowest 2689679.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2677117ns | 2682696ns | 2656652ns | 2675050ns | 2690450ns | -1.61% |
| abi_sink_madd_batched_sink_decode | 2675004ns | 2675087ns | 2666186ns | 2674705ns | 2679861ns | -1.69% |
| abi_sink_madd_null_sink | 2720922ns | 2692149ns | 2687259ns | 2690868ns | 2782835ns | base |
| abi_sink_madd_per_record_sink | 2681209ns | 2680255ns | 2668625ns | 2678482ns | 2691593ns | -1.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2674499ns | 2654142ns | 2687726ns | -1.61% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2672410ns | 2663671ns | 2677204ns | -1.69% | 0.000 |
| abi_sink_madd_null_sink | 2718288ns | 2684720ns | 2779919ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2678620ns | 2666010ns | 2688946ns | -1.46% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 41108.8 | 2674832.4 | 2674498.5 | n/a |
| abi_sink_madd_batched_sink_decode | 41094.8 | 2672549.3 | 2672410.2 | n/a |
| abi_sink_madd_null_sink | 48956.2 | 2711578.1 | 2718288.2 | n/a |
| abi_sink_madd_per_record_sink | 39798.1 | 2680536.2 | 2678619.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.0% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_madd_null_sink | 0.000 | 98.7% |
| abi_sink_madd_per_record_sink | 0.000 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2677117ns | 2677117ns | -1.61% |
| abi_sink_madd_batched_sink_decode | 2675004ns | 2675004ns | -1.69% |
| abi_sink_madd_null_sink | 2720922ns | 2720922ns | base |
| abi_sink_madd_per_record_sink | 2681209ns | 2681209ns | -1.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2689680ns | base | --- | [2685266, 2779919] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2680030ns | -24841.4ns (-0.9%) | [-101604, -4923]ns | [2655740, 2687726] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_sink_madd_batched_sink_decode | 2672542ns | -21668.1ns (-0.8%) | [-102714, -13252]ns | [2667484, 2677204] | YES | 0.0469 | 0.0313 | 0 |
| abi_sink_madd_per_record_sink | 2677723ns | -14733.4ns (-0.5%) | [-95269, -9004]ns | [2669190, 2688946] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2688733ns | -1.2% | -0.9% | -0.3% |
| 2 | 2684720ns | -1.1% | -0.5% | -0.3% |
| 3 | 2703707ns | -0.7% | -1.1% | -1.2% |
| 4 | 2856130ns | -6.0% | -6.1% | -5.6% |
| 5 | 2685812ns | +0.2% | -0.5% | -0.7% |
| 6 | 2690627ns | -0.6% | -0.7% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | 0.360 | moderate+ |
| abi_sink_madd_batched_sink_decode | 0.131 | ok |
| abi_sink_madd_null_sink | -0.178 | ok |
| abi_sink_madd_per_record_sink | -0.653 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 5/6, lost 1/6
- **abi_sink_madd_batched_sink_decode**: won 6/6, lost 0/6
- **abi_sink_madd_per_record_sink**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8067553.5ns | 2674498.5ns | 301.6% | HIGH |
| abi_sink_madd_batched_sink_decode | 8062150.4ns | 2672410.2ns | 301.7% | HIGH |
| abi_sink_madd_null_sink | 8270021.5ns | 2718288.2ns | 304.2% | HIGH |
| abi_sink_madd_per_record_sink | 8085248.5ns | 2678619.5ns | 301.8% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2654142.1-2687725.7 ns)
  2654142.1 |########################################
  2655821.3 |########################################
  2657500.5 |
  2659179.6 |
  2660858.8 |
  2662538.0 |
  2664217.2 |
  2665896.3 |
  2667575.5 |
  2669254.7 |
  2670933.9 |
  2672613.1 |
  2674292.2 |########################################
  2675971.4 |
  2677650.6 |
  2679329.8 |
  2681008.9 |
  2682688.1 |########################################
  2684367.3 |########################################
  2686046.5 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2663671.2-2677204.4 ns)
  2663671.2 |########################################
  2664347.9 |
  2665024.5 |
  2665701.2 |
  2666377.8 |
  2667054.5 |
  2667731.1 |
  2668407.8 |
  2669084.5 |
  2669761.1 |
  2670437.8 |
  2671114.4 |########################################
  2671791.1 |########################################
  2672467.7 |########################################
  2673144.4 |########################################
  2673821.1 |
  2674497.7 |
  2675174.4 |
  2675851.0 |
  2676527.7 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2684720.0-2779918.5 ns)
  2684720.0 |########################################
  2689479.9 |#############
  2694239.9 |
  2698999.8 |#############
  2703759.7 |
  2708519.6 |
  2713279.6 |
  2718039.5 |
  2722799.4 |
  2727559.3 |
  2732319.3 |
  2737079.2 |
  2741839.1 |
  2746599.1 |
  2751359.0 |
  2756118.9 |
  2760878.8 |
  2765638.8 |
  2770398.7 |
  2775158.6 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2666010.4-2688945.7 ns)
  2666010.4 |########################################
  2667157.2 |
  2668303.9 |
  2669450.7 |
  2670597.5 |
  2671744.2 |########################################
  2672891.0 |
  2674037.7 |
  2675184.5 |########################################
  2676331.3 |
  2677478.0 |
  2678624.8 |
  2679771.6 |########################################
  2680918.3 |########################################
  2682065.1 |
  2683211.8 |
  2684358.6 |
  2685505.4 |
  2686652.1 |
  2687798.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.6% of algo (FFI overhead may distort results)

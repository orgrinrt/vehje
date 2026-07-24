# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_madd_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_madd_null_sink has the worst median (2.69 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_madd_batched_sink at 2.66 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_sink_madd_batched_sink_decode shows alternating (throttle bounce) (autocorr -0.77)

abi_sink_madd_batched_sink_decode's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (25.02 us) is smaller than the fastest variant's own run-to-run std-dev (40.48 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.9% of the fastest

All 4 variants sit between 2.66 ms and 2.69 ms - a 0.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink** at 2660162.1 ns median (-0.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2660162.1 ns, slowest 2685180.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2679299ns | 2662791ns | 2655420ns | 2661986ns | 2717209ns | -0.34% |
| abi_sink_madd_batched_sink_decode | 2671498ns | 2670641ns | 2652716ns | 2667037ns | 2687580ns | -0.63% |
| abi_sink_madd_null_sink | 2688444ns | 2687851ns | 2674092ns | 2684774ns | 2701126ns | base |
| abi_sink_madd_per_record_sink | 2725408ns | 2666208ns | 2655075ns | 2664721ns | 2851606ns | +1.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2676625ns | 2652861ns | 2714440ns | -0.34% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2668740ns | 2650004ns | 2684744ns | -0.64% | 0.000 |
| abi_sink_madd_null_sink | 2685795ns | 2671395ns | 2698442ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2722569ns | 2652630ns | 2848358ns | +1.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 44364.3 | 2670216.7 | 2676624.9 | 0 |
| abi_sink_madd_batched_sink_decode | 42747.7 | 2668346.2 | 2668739.9 | n/a |
| abi_sink_madd_null_sink | 42979.0 | 2684715.0 | 2685795.0 | n/a |
| abi_sink_madd_per_record_sink | 50142.6 | 2712679.6 | 2722568.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink_decode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.6% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_madd_null_sink | 0.000 | 98.7% |
| abi_sink_madd_per_record_sink | 0.000 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2679299ns | 2679299ns | -0.34% |
| abi_sink_madd_batched_sink_decode | 2671498ns | 2671498ns | -0.63% |
| abi_sink_madd_null_sink | 2688444ns | 2688444ns | base |
| abi_sink_madd_per_record_sink | 2725408ns | 2725408ns | +1.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2685181ns | base | --- | [2673762, 2698442] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2660162ns | no significant difference | [-38912, +40678]ns | [2655273, 2714440] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_madd_batched_sink_decode | 2667855ns | -15324.4ns (-0.6%) | [-29919, -5922]ns | [2653620, 2684744] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_madd_per_record_sink | 2663412ns | no significant difference | [-34109, +166747]ns | [2655936, 2848358] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2680925ns | -1.0% | -0.1% | -0.6% |
| 2 | 2671395ns | +3.6% | -0.8% | +13.1% |
| 3 | 2701202ns | -1.5% | -0.4% | -1.4% |
| 4 | 2695682ns | -1.4% | -1.4% | -0.8% |
| 5 | 2676129ns | -0.5% | -0.4% | -0.9% |
| 6 | 2689437ns | -1.1% | -0.7% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.274 | moderate- |
| abi_sink_madd_batched_sink_decode | -0.771 | HIGH- (thermal bounce) |
| abi_sink_madd_null_sink | -0.193 | ok |
| abi_sink_madd_per_record_sink | -0.230 | moderate- |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 5/6, lost 1/6
- **abi_sink_madd_batched_sink_decode**: won 5/6, lost 0/6
- **abi_sink_madd_per_record_sink**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8052586.4ns | 2676624.9ns | 300.8% | HIGH |
| abi_sink_madd_batched_sink_decode | 8045406.0ns | 2668739.9ns | 301.5% | HIGH |
| abi_sink_madd_null_sink | 8094345.0ns | 2685795.0ns | 301.4% | HIGH |
| abi_sink_madd_per_record_sink | 8226084.2ns | 2722568.6ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2652861.2-2714439.8 ns)
  2652861.2 |####################
  2655940.1 |########################################
  2659019.1 |########################################
  2662098.0 |
  2665176.9 |
  2668255.9 |
  2671334.8 |
  2674413.7 |
  2677492.6 |
  2680571.6 |
  2683650.5 |
  2686729.4 |
  2689808.4 |
  2692887.3 |
  2695966.2 |
  2699045.1 |
  2702124.1 |
  2705203.0 |
  2708281.9 |
  2711360.9 |
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2650003.7-2684744.0 ns)
  2650003.7 |########################################
  2651740.7 |
  2653477.7 |
  2655214.7 |
  2656951.8 |########################################
  2658688.8 |
  2660425.8 |
  2662162.8 |
  2663899.8 |
  2665636.8 |########################################
  2667373.8 |
  2669110.8 |########################################
  2670847.9 |
  2672584.9 |
  2674321.9 |
  2676058.9 |
  2677795.9 |########################################
  2679532.9 |
  2681269.9 |
  2683006.9 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2671395.4-2698442.3 ns)
  2671395.4 |########################################
  2672747.7 |
  2674100.1 |
  2675452.4 |########################################
  2676804.8 |
  2678157.1 |
  2679509.5 |
  2680861.8 |########################################
  2682214.2 |
  2683566.5 |
  2684918.8 |
  2686271.2 |
  2687623.5 |
  2688975.9 |########################################
  2690328.2 |
  2691680.6 |
  2693032.9 |
  2694385.3 |########################################
  2695737.6 |
  2697090.0 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2652629.6-2848357.5 ns)
  2652629.6 |########################################
  2662416.0 |########################################
  2672202.4 |####################
  2681988.8 |
  2691775.2 |
  2701561.6 |
  2711348.0 |
  2721134.4 |
  2730920.8 |
  2740707.2 |
  2750493.5 |
  2760279.9 |
  2770066.3 |
  2779852.7 |
  2789639.1 |
  2799425.5 |
  2809211.9 |
  2818998.3 |
  2828784.7 |
  2838571.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=302.4% of algo (FFI overhead may distort results)

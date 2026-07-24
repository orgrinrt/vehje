# abi_sink (madd)

4 variants, 6 samples per variant.
Baseline: **abi_sink_madd_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_madd_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_sink_madd_batched_sink shows alternating (throttle bounce) (autocorr -0.72)

abi_sink_madd_batched_sink's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10.79 us) is smaller than the fastest variant's own run-to-run std-dev (13.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.66 ms and 2.67 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_madd_batched_sink_decode** at 2663829.5 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 2663829.5 ns, slowest 2674615.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2668553ns | 2667141ns | 2655316ns | 2663233ns | 2683150ns | -0.22% |
| abi_sink_madd_batched_sink_decode | 2665697ns | 2666411ns | 2644785ns | 2662648ns | 2680727ns | -0.32% |
| abi_sink_madd_null_sink | 2674326ns | 2674369ns | 2668645ns | 2673034ns | 2679106ns | base |
| abi_sink_madd_per_record_sink | 2695757ns | 2677094ns | 2664277ns | 2675412ns | 2742014ns | +0.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_madd_batched_sink | 2665825ns | 2652721ns | 2680234ns | -0.22% | 0.000 |
| abi_sink_madd_batched_sink_decode | 2663129ns | 2642341ns | 2678065ns | -0.32% | 0.000 |
| abi_sink_madd_null_sink | 2671742ns | 2666175ns | 2676508ns | base | 0.000 |
| abi_sink_madd_per_record_sink | 2693083ns | 2661627ns | 2739170ns | +0.80% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 42762.0 | 2666374.3 | 2665825.3 | 0 |
| abi_sink_madd_batched_sink_decode | 39453.8 | 2663539.9 | 2663129.3 | n/a |
| abi_sink_madd_null_sink | 40806.6 | 2673099.2 | 2671741.9 | n/a |
| abi_sink_madd_per_record_sink | 43475.5 | 2691196.1 | 2693082.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_madd_batched_sink_decode; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_madd_batched_sink | 0.000 | 99.2% |
| abi_sink_madd_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_madd_null_sink | 0.000 | 98.9% |
| abi_sink_madd_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_madd_batched_sink | 2668553ns | 2668553ns | -0.22% |
| abi_sink_madd_batched_sink_decode | 2665697ns | 2665697ns | -0.32% |
| abi_sink_madd_null_sink | 2674326ns | 2674326ns | base |
| abi_sink_madd_per_record_sink | 2695757ns | 2695757ns | +0.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_madd_null_sink | 2671709ns | base | --- | [2667008, 2676508] | --- | --- | --- | --- |
| abi_sink_madd_batched_sink | 2664446ns | no significant difference | [-21215, +13226]ns | [2652796, 2680234] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_madd_batched_sink_decode | 2663830ns | no significant difference | [-22844, +6366]ns | [2647494, 2678065] | no | 0.6563 | 0.2188 | 0 |
| abi_sink_madd_per_record_sink | 2674615ns | no significant difference | [-11045, +68294]ns | [2665464, 2739170] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_madd_null_sink | abi_sink_madd_batched_sink | abi_sink_madd_batched_sink_decode | abi_sink_madd_per_record_sink |
|---|---|---|---|---|
| 1 | 2678516ns | -1.0% | -0.2% | -0.3% |
| 2 | 2674501ns | -0.1% | -1.2% | -0.5% |
| 3 | 2673911ns | -0.6% | -0.5% | +0.3% |
| 4 | 2666175ns | +0.6% | -0.5% | +0.3% |
| 5 | 2669507ns | -0.6% | -0.1% | +0.2% |
| 6 | 2667841ns | +0.4% | +0.6% | +4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_madd_batched_sink | -0.716 | HIGH- (thermal bounce) |
| abi_sink_madd_batched_sink_decode | -0.090 | ok |
| abi_sink_madd_null_sink | 0.309 | moderate+ |
| abi_sink_madd_per_record_sink | -0.016 | ok |

**Consistency summary:**

- **abi_sink_madd_batched_sink**: won 4/6, lost 2/6
- **abi_sink_madd_batched_sink_decode**: won 5/6, lost 1/6
- **abi_sink_madd_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_madd_batched_sink | 8045907.6ns | 2665825.3ns | 301.8% | HIGH |
| abi_sink_madd_batched_sink_decode | 8027151.2ns | 2663129.3ns | 301.4% | HIGH |
| abi_sink_madd_null_sink | 8057637.7ns | 2671741.9ns | 301.6% | HIGH |
| abi_sink_madd_per_record_sink | 8120215.1ns | 2693082.9ns | 301.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_madd_batched_sink (n=6, range 2652721.2-2680233.8 ns)
  2652721.2 |########################################
  2654096.8 |
  2655472.5 |
  2656848.1 |####################
  2658223.7 |
  2659599.3 |
  2660975.0 |
  2662350.6 |
  2663726.2 |
  2665101.8 |
  2666477.5 |
  2667853.1 |
  2669228.7 |
  2670604.4 |####################
  2671980.0 |
  2673355.6 |
  2674731.2 |
  2676106.9 |
  2677482.5 |
  2678858.1 |####################
  (0 below, 1 above range)

abi_sink_madd_batched_sink_decode (n=6, range 2642341.2-2678065.0 ns)
  2642341.2 |########################################
  2644127.4 |
  2645913.6 |
  2647699.8 |
  2649486.0 |
  2651272.1 |########################################
  2653058.3 |
  2654844.5 |
  2656630.7 |
  2658416.9 |
  2660203.1 |########################################
  2661989.3 |
  2663775.5 |
  2665561.6 |########################################
  2667347.8 |
  2669134.0 |
  2670920.2 |
  2672706.4 |########################################
  2674492.6 |
  2676278.8 |
  (0 below, 1 above range)

abi_sink_madd_null_sink (n=6, range 2666174.6-2676508.5 ns)
  2666174.6 |########################################
  2666691.3 |
  2667208.0 |
  2667724.7 |########################################
  2668241.4 |
  2668758.1 |
  2669274.8 |########################################
  2669791.5 |
  2670308.2 |
  2670824.9 |
  2671341.5 |
  2671858.2 |
  2672374.9 |
  2672891.6 |
  2673408.3 |########################################
  2673925.0 |
  2674441.7 |########################################
  2674958.4 |
  2675475.1 |
  2675991.8 |
  (0 below, 1 above range)

abi_sink_madd_per_record_sink (n=6, range 2661626.7-2739170.0 ns)
  2661626.7 |####################
  2665503.9 |####################
  2669381.0 |
  2673258.2 |########################################
  2677135.4 |
  2681012.5 |####################
  2684889.7 |
  2688766.9 |
  2692644.0 |
  2696521.2 |
  2700398.4 |
  2704275.5 |
  2708152.7 |
  2712029.8 |
  2715907.0 |
  2719784.2 |
  2723661.3 |
  2727538.5 |
  2731415.7 |
  2735292.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_madd_batched_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_madd_batched_sink_decode**: bridge=301.1% of algo (FFI overhead may distort results)
- **abi_sink_madd_null_sink**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_sink_madd_per_record_sink**: bridge=301.4% of algo (FFI overhead may distort results)

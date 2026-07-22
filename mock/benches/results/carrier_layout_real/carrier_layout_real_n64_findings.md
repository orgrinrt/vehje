# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_real_rec16 shows alternating (throttle bounce) (autocorr -0.65)

carrier_lay_real_rec16's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_lay_real_rec24)

The baseline carrier_lay_real_rec24 is the fastest (2.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 2.1% of the fastest

All 5 variants sit between 2.46 us and 2.51 us - a 2.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_real_rec12's edge over baseline is significant but tiny (14 ns, 0.56%)

carrier_lay_real_rec12 differs from baseline carrier_lay_real_rec24 by 14 ns (0.56%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (carrier_lay_real_rec24) is the fastest** at 2463.9 ns median
- Spread: 1.02x (fastest 2463.9 ns, slowest 2514.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 4958ns | 5021ns | 4495ns | 5000ns | 5128ns | -2.05% |
| carrier_lay_real_rec16 | 5089ns | 5088ns | 5056ns | 5079ns | 5121ns | +0.53% |
| carrier_lay_real_rec20 | 5102ns | 5107ns | 4961ns | 5069ns | 5223ns | +0.79% |
| carrier_lay_real_rec24 | 5062ns | 5049ns | 4920ns | 5046ns | 5157ns | base |
| carrier_lay_real_rec32 | 5079ns | 5081ns | 4967ns | 5047ns | 5181ns | +0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 2462ns | 2188ns | 2529ns | -0.63% | 0.026 |
| carrier_lay_real_rec16 | 2487ns | 2442ns | 2523ns | +0.36% | 0.026 |
| carrier_lay_real_rec20 | 2492ns | 2459ns | 2519ns | +0.57% | 0.026 |
| carrier_lay_real_rec24 | 2478ns | 2447ns | 2520ns | base | 0.026 |
| carrier_lay_real_rec32 | 2484ns | 2457ns | 2506ns | +0.26% | 0.026 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_lay_real_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.025 | 87.0% |
| carrier_lay_real_rec16 | 0.026 | 88.0% |
| carrier_lay_real_rec20 | 0.026 | 87.9% |
| carrier_lay_real_rec24 | 0.026 | 88.8% |
| carrier_lay_real_rec32 | 0.026 | 88.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 4958ns | 4958ns | -2.05% |
| carrier_lay_real_rec16 | 5089ns | 5089ns | +0.53% |
| carrier_lay_real_rec20 | 5102ns | 5102ns | +0.79% |
| carrier_lay_real_rec24 | 5062ns | 5062ns | base |
| carrier_lay_real_rec32 | 5079ns | 5079ns | +0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 2464ns | base | --- | [2449, 2520] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 2515ns | no significant difference | [-140, +79]ns | [2343, 2529] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_real_rec16 | 2486ns | no significant difference | [-42, +56]ns | [2450, 2523] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_real_rec20 | 2488ns | no significant difference | [-41, +67]ns | [2468, 2519] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_real_rec32 | 2484ns | no significant difference | [-27, +45]ns | [2462, 2506] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 2456ns | -10.9% | +0.1% | +2.4% | +2.6% |
| 2 | 2452ns | +3.0% | +2.1% | +1.1% | +0.7% |
| 3 | 2447ns | +3.5% | +2.5% | +3.1% | +1.1% |
| 4 | 2523ns | -0.4% | -3.2% | -1.0% | -1.2% |
| 5 | 2516ns | +0.1% | +0.9% | -2.3% | -1.0% |
| 6 | 2472ns | +1.1% | -0.1% | +0.2% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.053 | ok |
| carrier_lay_real_rec16 | -0.653 | HIGH- (thermal bounce) |
| carrier_lay_real_rec20 | -0.103 | ok |
| carrier_lay_real_rec24 | 0.263 | moderate+ |
| carrier_lay_real_rec32 | -0.259 | moderate- |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 2/6, lost 3/6
- **carrier_lay_real_rec16**: won 2/6, lost 3/6
- **carrier_lay_real_rec20**: won 2/6, lost 4/6
- **carrier_lay_real_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 86343.5ns | 2462.1ns | 3506.9% | HIGH |
| carrier_lay_real_rec16 | 86756.0ns | 2486.5ns | 3489.0% | HIGH |
| carrier_lay_real_rec20 | 86558.6ns | 2491.8ns | 3473.8% | HIGH |
| carrier_lay_real_rec24 | 86707.3ns | 2477.6ns | 3499.6% | HIGH |
| carrier_lay_real_rec32 | 86477.8ns | 2484.2ns | 3481.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 2187.5-2528.8 ns)
   2187.5 |#############
   2204.6 |
   2221.6 |
   2238.7 |
   2255.8 |
   2272.8 |
   2289.9 |
   2306.9 |
   2324.0 |
   2341.1 |
   2358.1 |
   2375.2 |
   2392.2 |
   2409.3 |
   2426.4 |
   2443.4 |
   2460.5 |
   2477.6 |
   2494.6 |#############
   2511.7 |########################################
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 2442.1-2523.4 ns)
   2442.1 |########################################
   2446.2 |
   2450.2 |
   2454.3 |########################################
   2458.3 |
   2462.4 |
   2466.5 |########################################
   2470.5 |
   2474.6 |
   2478.7 |
   2482.7 |
   2486.8 |
   2490.9 |
   2494.9 |
   2499.0 |
   2503.0 |########################################
   2507.1 |########################################
   2511.2 |
   2515.2 |
   2519.3 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 2458.8-2518.9 ns)
   2458.8 |####################
   2461.8 |
   2464.8 |
   2467.8 |
   2470.8 |
   2473.8 |
   2476.8 |########################################
   2479.9 |
   2482.9 |
   2485.9 |
   2488.9 |
   2491.9 |
   2494.9 |
   2497.9 |####################
   2500.9 |
   2503.9 |
   2506.9 |
   2509.9 |
   2512.9 |####################
   2515.9 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 2447.1-2519.6 ns)
   2447.1 |########################################
   2450.7 |########################################
   2454.3 |########################################
   2458.0 |
   2461.6 |
   2465.2 |
   2468.8 |########################################
   2472.5 |
   2476.1 |
   2479.7 |
   2483.3 |
   2486.9 |
   2490.6 |
   2494.2 |
   2497.8 |
   2501.4 |
   2505.1 |
   2508.7 |
   2512.3 |
   2515.9 |########################################
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 2456.7-2506.5 ns)
   2456.7 |####################
   2459.2 |
   2461.7 |
   2464.2 |
   2466.7 |####################
   2469.1 |
   2471.6 |
   2474.1 |####################
   2476.6 |
   2479.1 |
   2481.6 |
   2484.1 |
   2486.6 |
   2489.1 |
   2491.6 |########################################
   2494.1 |
   2496.5 |
   2499.0 |
   2501.5 |
   2504.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=3427.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=3494.9% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=3484.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=3514.9% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=3490.0% of algo (FFI overhead may distort results)

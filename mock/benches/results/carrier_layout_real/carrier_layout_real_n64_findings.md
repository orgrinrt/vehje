# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.8% of the fastest

All 5 variants sit between 2.44 us and 2.48 us - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_real_rec12's edge over baseline is significant but tiny (29 ns, 1.18%)

carrier_lay_real_rec12 differs from baseline carrier_lay_real_rec24 by 29 ns (1.18%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_real_rec16** at 2440.2 ns median (-1.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.02x (fastest 2440.2 ns, slowest 2485.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 4913ns | 4935ns | 4314ns | 4918ns | 5205ns | -1.15% |
| carrier_lay_real_rec16 | 4935ns | 4922ns | 4872ns | 4920ns | 4987ns | -0.72% |
| carrier_lay_real_rec20 | 4952ns | 4914ns | 4868ns | 4901ns | 5072ns | -0.36% |
| carrier_lay_real_rec24 | 4970ns | 4981ns | 4900ns | 4967ns | 5010ns | base |
| carrier_lay_real_rec32 | 4985ns | 4946ns | 4860ns | 4919ns | 5146ns | +0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 2449ns | 2168ns | 2542ns | -1.05% | 0.026 |
| carrier_lay_real_rec16 | 2439ns | 2418ns | 2454ns | -1.45% | 0.026 |
| carrier_lay_real_rec20 | 2455ns | 2398ns | 2489ns | -0.78% | 0.026 |
| carrier_lay_real_rec24 | 2475ns | 2436ns | 2497ns | base | 0.026 |
| carrier_lay_real_rec32 | 2466ns | 2441ns | 2498ns | -0.36% | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 256714 | 1291419 | 0.199 | 1.02× |
| carrier_lay_real_rec16 | 253742 | 1273608 | 0.199 | 1.01× |
| carrier_lay_real_rec20 | 253240 | 1271212 | 0.199 | 1.01× |
| carrier_lay_real_rec24 | 250816 | 1259118 | 0.199 | 1.00× |
| carrier_lay_real_rec32 | 250393 | 1257727 | 0.199 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_lay_real_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.026 | 87.2% |
| carrier_lay_real_rec16 | 0.026 | 88.8% |
| carrier_lay_real_rec20 | 0.026 | 88.2% |
| carrier_lay_real_rec24 | 0.026 | 87.5% |
| carrier_lay_real_rec32 | 0.026 | 88.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 4913ns | 4913ns | -1.15% |
| carrier_lay_real_rec16 | 4935ns | 4935ns | -0.72% |
| carrier_lay_real_rec20 | 4952ns | 4952ns | -0.36% |
| carrier_lay_real_rec24 | 4970ns | 4970ns | base |
| carrier_lay_real_rec32 | 4985ns | 4985ns | +0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 2477ns | base | --- | [2450, 2497] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 2485ns | no significant difference | [-165, +58]ns | [2319, 2542] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec16 | 2440ns | -38.0ns (-1.5%) | [-61, -8]ns | [2423, 2454] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| carrier_lay_real_rec20 | 2458ns | no significant difference | [-52, +13]ns | [2418, 2489] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_real_rec32 | 2457ns | no significant difference | [-30, +17]ns | [2442, 2498] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 2489ns | -12.9% | -1.6% | -0.3% | -1.1% |
| 2 | 2475ns | +0.9% | -2.3% | -1.2% | -1.3% |
| 3 | 2478ns | -0.4% | -1.5% | -0.3% | -1.0% |
| 4 | 2464ns | +2.8% | -1.5% | +1.4% | -0.2% |
| 5 | 2436ns | +1.5% | +0.8% | -1.6% | +0.2% |
| 6 | 2505ns | +1.9% | -2.6% | -2.6% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.061 | ok |
| carrier_lay_real_rec16 | -0.500 | moderate- |
| carrier_lay_real_rec20 | -0.189 | ok |
| carrier_lay_real_rec24 | -0.288 | moderate- |
| carrier_lay_real_rec32 | -0.193 | ok |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 2/6, lost 4/6
- **carrier_lay_real_rec16**: won 5/6, lost 1/6
- **carrier_lay_real_rec20**: won 5/6, lost 1/6
- **carrier_lay_real_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 85670.7ns | 2448.6ns | 3498.7% | HIGH |
| carrier_lay_real_rec16 | 85995.1ns | 2439.0ns | 3525.9% | HIGH |
| carrier_lay_real_rec20 | 86305.9ns | 2455.3ns | 3515.0% | HIGH |
| carrier_lay_real_rec24 | 86166.8ns | 2474.7ns | 3481.9% | HIGH |
| carrier_lay_real_rec32 | 85741.3ns | 2465.8ns | 3477.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 2167.5-2542.3 ns)
   2167.5 |####################
   2186.2 |
   2205.0 |
   2223.7 |
   2242.5 |
   2261.2 |
   2279.9 |
   2298.7 |
   2317.4 |
   2336.2 |
   2354.9 |
   2373.6 |
   2392.4 |
   2411.1 |
   2429.9 |
   2448.6 |
   2467.3 |########################################
   2486.1 |####################
   2504.8 |
   2523.6 |####################
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 2418.3-2453.6 ns)
   2418.3 |########################################
   2420.1 |
   2421.8 |
   2423.6 |
   2425.4 |
   2427.1 |########################################
   2428.9 |
   2430.6 |
   2432.4 |
   2434.2 |
   2435.9 |
   2437.7 |########################################
   2439.5 |########################################
   2441.2 |
   2443.0 |
   2444.7 |
   2446.5 |
   2448.3 |
   2450.0 |########################################
   2451.8 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 2397.5-2489.3 ns)
   2397.5 |########################################
   2402.1 |
   2406.7 |
   2411.3 |
   2415.9 |
   2420.5 |
   2425.1 |
   2429.6 |
   2434.2 |########################################
   2438.8 |
   2443.4 |########################################
   2448.0 |
   2452.6 |
   2457.2 |
   2461.8 |
   2466.4 |########################################
   2471.0 |
   2475.6 |
   2480.2 |########################################
   2484.8 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 2436.2-2497.1 ns)
   2436.2 |########################################
   2439.2 |
   2442.3 |
   2445.3 |
   2448.4 |
   2451.4 |
   2454.5 |
   2457.5 |
   2460.6 |
   2463.6 |########################################
   2466.6 |
   2469.7 |
   2472.7 |########################################
   2475.8 |########################################
   2478.8 |
   2481.9 |
   2484.9 |
   2488.0 |########################################
   2491.0 |
   2494.1 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 2440.8-2498.3 ns)
   2440.8 |########################################
   2443.7 |
   2446.6 |
   2449.4 |
   2452.3 |####################
   2455.2 |
   2458.1 |####################
   2460.9 |####################
   2463.8 |
   2466.7 |
   2469.6 |
   2472.5 |
   2475.3 |
   2478.2 |
   2481.1 |
   2484.0 |
   2486.8 |
   2489.7 |
   2492.6 |
   2495.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=3450.4% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=3527.0% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=3502.7% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=3478.3% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=3491.7% of algo (FFI overhead may distort results)

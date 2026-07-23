# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_lay_scatter_rec16, carrier_lay_scatter_rec20) are a dead heat (<1%)

carrier_lay_scatter_rec16 (2.40 us) and carrier_lay_scatter_rec20 (2.42 us) differ by 0.85%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole field within 3.4% of the fastest

All 5 variants sit between 2.40 us and 2.48 us - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_scatter_rec12's edge over baseline is significant but tiny (29 ns, 1.18%)

carrier_lay_scatter_rec12 differs from baseline carrier_lay_scatter_rec24 by 29 ns (1.18%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_scatter_rec16** at 2396.7 ns median (-2.1% vs baseline)
- Spread: 1.03x (fastest 2396.7 ns, slowest 2477.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 4792ns | 4864ns | 4427ns | 4833ns | 4912ns | -0.01% |
| carrier_lay_scatter_rec16 | 4710ns | 4736ns | 4562ns | 4688ns | 4815ns | -1.73% |
| carrier_lay_scatter_rec20 | 4990ns | 4773ns | 4640ns | 4748ns | 5528ns | +4.12% |
| carrier_lay_scatter_rec24 | 4793ns | 4832ns | 4542ns | 4821ns | 4875ns | base |
| carrier_lay_scatter_rec32 | 4862ns | 4848ns | 4757ns | 4836ns | 4953ns | +1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2439ns | 2251ns | 2501ns | +0.65% | 0.026 |
| carrier_lay_scatter_rec16 | 2383ns | 2302ns | 2440ns | -1.65% | 0.027 |
| carrier_lay_scatter_rec20 | 2632ns | 2344ns | 3115ns | +8.60% | 0.024 |
| carrier_lay_scatter_rec24 | 2423ns | 2290ns | 2464ns | base | 0.026 |
| carrier_lay_scatter_rec32 | 2471ns | 2412ns | 2550ns | +1.96% | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 263746 | 1340836 | 0.197 | 1.01× |
| carrier_lay_scatter_rec16 | 264793 | 1342424 | 0.197 | 1.02× |
| carrier_lay_scatter_rec20 | 259982 | 1316991 | 0.197 | 1.00× |
| carrier_lay_scatter_rec24 | 260410 | 1317094 | 0.198 | 1.00× |
| carrier_lay_scatter_rec32 | 260055 | 1316319 | 0.198 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_lay_scatter_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.026 | 90.9% |
| carrier_lay_scatter_rec16 | 0.027 | 93.9% |
| carrier_lay_scatter_rec20 | 0.026 | 93.1% |
| carrier_lay_scatter_rec24 | 0.026 | 92.0% |
| carrier_lay_scatter_rec32 | 0.026 | 92.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 4792ns | 4792ns | -0.01% |
| carrier_lay_scatter_rec16 | 4710ns | 4710ns | -1.73% |
| carrier_lay_scatter_rec20 | 4990ns | 4990ns | +4.12% |
| carrier_lay_scatter_rec24 | 4793ns | 4793ns | base |
| carrier_lay_scatter_rec32 | 4862ns | 4862ns | +1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 2447ns | base | --- | [2358, 2464] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 2477ns | no significant difference | [-109, +127]ns | [2339, 2501] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 2397ns | no significant difference | [-118, +50]ns | [2313, 2440] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_scatter_rec20 | 2417ns | no significant difference | [-85, +711]ns | [2363, 3115] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec32 | 2444ns | no significant difference | [-30, +170]ns | [2417, 2550] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 2460ns | -8.5% | -2.8% | +53.6% | -1.9% |
| 2 | 2290ns | +9.3% | +4.9% | +4.6% | +6.1% |
| 3 | 2435ns | -0.3% | -4.6% | +0.2% | -0.5% |
| 4 | 2427ns | +1.7% | -5.1% | -3.4% | +8.2% |
| 5 | 2459ns | +1.7% | -0.6% | -0.2% | +0.6% |
| 6 | 2469ns | +0.7% | -1.3% | -3.5% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.180 | ok |
| carrier_lay_scatter_rec16 | 0.123 | ok |
| carrier_lay_scatter_rec20 | -0.048 | ok |
| carrier_lay_scatter_rec24 | -0.203 | moderate- |
| carrier_lay_scatter_rec32 | -0.085 | ok |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec16**: won 5/6, lost 1/6
- **carrier_lay_scatter_rec20**: won 3/6, lost 3/6
- **carrier_lay_scatter_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 86249.8ns | 2439.0ns | 3536.3% | HIGH |
| carrier_lay_scatter_rec16 | 86145.9ns | 2383.3ns | 3614.5% | HIGH |
| carrier_lay_scatter_rec20 | 93944.2ns | 2631.7ns | 3569.7% | HIGH |
| carrier_lay_scatter_rec24 | 85992.9ns | 2423.2ns | 3548.7% | HIGH |
| carrier_lay_scatter_rec32 | 87530.3ns | 2470.7ns | 3542.8% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 2250.8-2501.2 ns)
   2250.8 |########################################
   2263.3 |
   2275.8 |
   2288.4 |
   2300.9 |
   2313.4 |
   2325.9 |
   2338.5 |
   2351.0 |
   2363.5 |
   2376.0 |
   2388.5 |
   2401.1 |
   2413.6 |
   2426.1 |########################################
   2438.6 |
   2451.2 |
   2463.7 |########################################
   2476.2 |########################################
   2488.7 |########################################
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 2302.5-2440.4 ns)
   2302.5 |########################################
   2309.4 |
   2316.3 |
   2323.2 |########################################
   2330.1 |
   2337.0 |
   2343.9 |
   2350.8 |
   2357.7 |
   2364.6 |
   2371.4 |
   2378.3 |
   2385.2 |########################################
   2392.1 |
   2399.0 |########################################
   2405.9 |
   2412.8 |
   2419.7 |
   2426.6 |
   2433.5 |########################################
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 2343.8-3115.0 ns)
   2343.8 |########################################
   2382.4 |####################
   2420.9 |########################################
   2459.5 |
   2498.0 |
   2536.6 |
   2575.2 |
   2613.7 |
   2652.3 |
   2690.8 |
   2729.4 |
   2768.0 |
   2806.5 |
   2845.1 |
   2883.6 |
   2922.2 |
   2960.8 |
   2999.3 |
   3037.9 |
   3076.4 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 2289.6-2464.4 ns)
   2289.6 |####################
   2298.3 |
   2307.1 |
   2315.8 |
   2324.6 |
   2333.3 |
   2342.0 |
   2350.8 |
   2359.5 |
   2368.3 |
   2377.0 |
   2385.7 |
   2394.5 |
   2403.2 |
   2412.0 |
   2420.7 |####################
   2429.4 |####################
   2438.2 |
   2446.9 |
   2455.7 |########################################
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 2412.5-2550.4 ns)
   2412.5 |########################################
   2419.4 |########################################
   2426.3 |########################################
   2433.2 |
   2440.1 |
   2447.0 |
   2453.9 |########################################
   2460.8 |
   2467.7 |
   2474.6 |########################################
   2481.4 |
   2488.3 |
   2495.2 |
   2502.1 |
   2509.0 |
   2515.9 |
   2522.8 |
   2529.7 |
   2536.6 |
   2543.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=3482.6% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=3596.2% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=3569.0% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=3524.7% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=3537.1% of algo (FFI overhead may distort results)

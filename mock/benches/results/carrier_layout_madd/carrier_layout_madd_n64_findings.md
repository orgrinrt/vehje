# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_lay_madd_rec16, carrier_lay_madd_rec32) are a dead heat (<1%)

carrier_lay_madd_rec16 (2.49 us) and carrier_lay_madd_rec32 (2.50 us) differ by 0.37%, inside the noise, even though the wider field spreads 7.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader carrier_lay_madd_rec16 vs stability leader carrier_lay_madd_rec20 (+7% speed for 1.2x steadier)

carrier_lay_madd_rec16 is fastest (2.49 us, CV 4.7%); carrier_lay_madd_rec20 gives up 6.9% median for 1.2x lower variance (CV 3.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_lay_madd_rec12's edge over baseline is significant but tiny (31 ns, 1.25%)

carrier_lay_madd_rec12 differs from baseline carrier_lay_madd_rec24 by 31 ns (1.25%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_madd_rec16** at 2490.8 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 2490.8 ns, slowest 2667.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 5030ns | 5085ns | 4599ns | 5071ns | 5182ns | +3.46% |
| carrier_lay_madd_rec16 | 4832ns | 4779ns | 4540ns | 4706ns | 5166ns | -0.62% |
| carrier_lay_madd_rec20 | 5085ns | 5118ns | 4615ns | 5114ns | 5276ns | +4.59% |
| carrier_lay_madd_rec24 | 4861ns | 4837ns | 4573ns | 4769ns | 5145ns | base |
| carrier_lay_madd_rec32 | 4846ns | 4801ns | 4498ns | 4720ns | 5210ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 2625ns | 2376ns | 2703ns | +3.63% | 0.024 |
| carrier_lay_madd_rec16 | 2503ns | 2375ns | 2638ns | -1.19% | 0.026 |
| carrier_lay_madd_rec20 | 2629ns | 2405ns | 2693ns | +3.79% | 0.024 |
| carrier_lay_madd_rec24 | 2533ns | 2375ns | 2696ns | base | 0.025 |
| carrier_lay_madd_rec32 | 2509ns | 2337ns | 2669ns | -0.96% | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 262208 | 1261684 | 0.208 | 0.98× |
| carrier_lay_madd_rec16 | 270902 | 1308463 | 0.207 | 1.01× |
| carrier_lay_madd_rec20 | 259648 | 1254817 | 0.207 | 0.97× |
| carrier_lay_madd_rec24 | 268495 | 1297566 | 0.207 | 1.00× |
| carrier_lay_madd_rec32 | 270836 | 1307970 | 0.207 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_lay_madd_rec32; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.024 | 87.6% |
| carrier_lay_madd_rec16 | 0.026 | 93.8% |
| carrier_lay_madd_rec20 | 0.024 | 87.8% |
| carrier_lay_madd_rec24 | 0.025 | 92.6% |
| carrier_lay_madd_rec32 | 0.026 | 93.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 5030ns | 5030ns | +3.46% |
| carrier_lay_madd_rec16 | 4832ns | 4832ns | -0.62% |
| carrier_lay_madd_rec20 | 5085ns | 5085ns | +4.59% |
| carrier_lay_madd_rec24 | 4861ns | 4861ns | base |
| carrier_lay_madd_rec32 | 4846ns | 4846ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 2523ns | base | --- | [2380, 2696] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 2667ns | no significant difference | [-15, +260]ns | [2505, 2703] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec16 | 2491ns | no significant difference | [-184, +123]ns | [2380, 2638] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 2663ns | no significant difference | [-24, +278]ns | [2531, 2693] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_madd_rec32 | 2500ns | -27.0ns (-1.1%) | [-43, -3]ns | [2357, 2669] | YES (adj: no) | 0.8750 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 2384ns | -0.3% | -0.4% | +11.5% | -2.0% |
| 2 | 2704ns | +0.9% | -3.0% | -1.3% | -1.3% |
| 3 | 2416ns | +9.0% | +9.8% | -0.4% | -1.6% |
| 4 | 2375ns | +12.7% | +0.4% | +11.9% | +0.1% |
| 5 | 2631ns | +1.4% | -2.0% | +3.0% | -0.3% |
| 6 | 2689ns | -0.8% | -10.6% | -0.5% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.259 | moderate- |
| carrier_lay_madd_rec16 | -0.383 | moderate- |
| carrier_lay_madd_rec20 | -0.124 | ok |
| carrier_lay_madd_rec24 | -0.219 | moderate- |
| carrier_lay_madd_rec32 | -0.214 | moderate- |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 2/6, lost 4/6
- **carrier_lay_madd_rec16**: won 4/6, lost 2/6
- **carrier_lay_madd_rec20**: won 3/6, lost 3/6
- **carrier_lay_madd_rec32**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 86892.0ns | 2625.1ns | 3310.0% | HIGH |
| carrier_lay_madd_rec16 | 86067.0ns | 2502.8ns | 3438.8% | HIGH |
| carrier_lay_madd_rec20 | 86634.0ns | 2629.1ns | 3295.2% | HIGH |
| carrier_lay_madd_rec24 | 86294.2ns | 2533.1ns | 3406.7% | HIGH |
| carrier_lay_madd_rec32 | 86128.5ns | 2508.7ns | 3433.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 2376.2-2702.9 ns)
   2376.2 |####################
   2392.5 |
   2408.9 |
   2425.2 |
   2441.5 |
   2457.9 |
   2474.2 |
   2490.6 |
   2506.9 |
   2523.2 |
   2539.6 |
   2555.9 |
   2572.2 |
   2588.6 |
   2604.9 |
   2621.3 |####################
   2637.6 |
   2653.9 |########################################
   2670.3 |####################
   2686.6 |
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 2375.4-2637.9 ns)
   2375.4 |########################################
   2388.5 |
   2401.7 |####################
   2414.8 |
   2427.9 |
   2441.0 |
   2454.2 |
   2467.3 |
   2480.4 |
   2493.5 |
   2506.7 |
   2519.8 |
   2532.9 |
   2546.0 |
   2559.2 |
   2572.3 |####################
   2585.4 |
   2598.5 |
   2611.7 |####################
   2624.8 |
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 2405.4-2693.1 ns)
   2405.4 |####################
   2419.8 |
   2434.2 |
   2448.6 |
   2462.9 |
   2477.3 |
   2491.7 |
   2506.1 |
   2520.5 |
   2534.9 |
   2549.2 |
   2563.6 |
   2578.0 |
   2592.4 |
   2606.8 |
   2621.2 |
   2635.6 |
   2649.9 |########################################
   2664.3 |########################################
   2678.7 |
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 2375.0-2696.3 ns)
   2375.0 |########################################
   2391.1 |
   2407.1 |####################
   2423.2 |
   2439.3 |
   2455.3 |
   2471.4 |
   2487.5 |
   2503.5 |
   2519.6 |
   2535.7 |
   2551.7 |
   2567.8 |
   2583.8 |
   2599.9 |
   2616.0 |####################
   2632.0 |
   2648.1 |
   2664.2 |
   2680.2 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 2337.1-2669.4 ns)
   2337.1 |####################
   2353.7 |
   2370.3 |########################################
   2386.9 |
   2403.6 |
   2420.2 |
   2436.8 |
   2453.4 |
   2470.0 |
   2486.6 |
   2503.2 |
   2519.8 |
   2536.5 |
   2553.1 |
   2569.7 |
   2586.3 |
   2602.9 |
   2619.5 |####################
   2636.1 |
   2652.7 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=3255.1% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=3453.6% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=3253.8% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=3421.9% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=3440.9% of algo (FFI overhead may distort results)

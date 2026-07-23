# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_nullfloor dominates: 27% faster than the next best (carrier_disp_madd_ifchainlin)

carrier_disp_madd_nullfloor (1.83 us) leads carrier_disp_madd_ifchainlin (2.33 us) by 27%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_madd_nullfloor beats baseline by 27% (significant)

carrier_disp_madd_nullfloor is -693 ns (27%) faster than baseline carrier_disp_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_madd_threaded shows alternating (throttle bounce) (autocorr -0.84)

carrier_disp_madd_threaded's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_madd_nullfloor} vs {carrier_disp_madd_ifchainlin, carrier_disp_madd_bittree, carrier_disp_madd_ifchainasc, carrier_disp_madd_switch, carrier_disp_madd_ifchain, carrier_disp_madd_threaded, carrier_disp_madd_fntable} (27% apart)

The field splits into a fast tier {carrier_disp_madd_nullfloor} and a slow tier {carrier_disp_madd_ifchainlin, carrier_disp_madd_bittree, carrier_disp_madd_ifchainasc, carrier_disp_madd_switch, carrier_disp_madd_ifchain, carrier_disp_madd_threaded, carrier_disp_madd_fntable} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_madd_ifchain's edge over baseline is significant but tiny (20 ns, 0.78%)

carrier_disp_madd_ifchain differs from baseline carrier_disp_madd_switch by 20 ns (0.78%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 1831.2 ns median (-29.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.58x (fastest 1831.2 ns, slowest 2887.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 4736ns | 4706ns | 4409ns | 4610ns | 5087ns | -7.00% |
| carrier_disp_madd_fntable | 5190ns | 5351ns | 4740ns | 5177ns | 5435ns | +1.93% |
| carrier_disp_madd_ifchain | 4973ns | 5123ns | 4598ns | 4956ns | 5187ns | -2.33% |
| carrier_disp_madd_ifchainasc | 4972ns | 5025ns | 4596ns | 4892ns | 5280ns | -2.35% |
| carrier_disp_madd_ifchainlin | 4750ns | 4727ns | 4390ns | 4641ns | 5094ns | -6.71% |
| carrier_disp_madd_nullfloor | 4161ns | 4186ns | 3864ns | 4105ns | 4393ns | -18.28% |
| carrier_disp_madd_switch | 5092ns | 5132ns | 4522ns | 5098ns | 5367ns | base |
| carrier_disp_madd_threaded | 5154ns | 5184ns | 4835ns | 5072ns | 5438ns | +1.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 2421ns | 2262ns | 2590ns | -6.29% | 0.026 |
| carrier_disp_madd_fntable | 2796ns | 2570ns | 2914ns | +8.23% | 0.023 |
| carrier_disp_madd_ifchain | 2586ns | 2382ns | 2718ns | +0.07% | 0.025 |
| carrier_disp_madd_ifchainasc | 2558ns | 2370ns | 2682ns | -0.99% | 0.025 |
| carrier_disp_madd_ifchainlin | 2337ns | 2169ns | 2485ns | -9.57% | 0.027 |
| carrier_disp_madd_nullfloor | 1838ns | 1718ns | 1954ns | -28.86% | 0.035 |
| carrier_disp_madd_switch | 2584ns | 2377ns | 2665ns | base | 0.025 |
| carrier_disp_madd_threaded | 2806ns | 2625ns | 3003ns | +8.59% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 267756 | 1394622 | 0.192 | 1.01× |
| carrier_disp_madd_fntable | 257190 | 1551781 | 0.166 | 0.97× |
| carrier_disp_madd_ifchain | 262922 | 1312355 | 0.200 | 0.99× |
| carrier_disp_madd_ifchainasc | 265146 | 1316002 | 0.201 | 1.00× |
| carrier_disp_madd_ifchainlin | 270244 | 1590277 | 0.170 | 1.02× |
| carrier_disp_madd_nullfloor | 265144 | 1543036 | 0.172 | 1.00× |
| carrier_disp_madd_switch | 264993 | 1277242 | 0.207 | 1.00× |
| carrier_disp_madd_threaded | 275671 | 1667532 | 0.165 | 1.04× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.027 | 71.3% |
| carrier_disp_madd_fntable | 0.022 | 59.5% |
| carrier_disp_madd_ifchain | 0.024 | 65.0% |
| carrier_disp_madd_ifchainasc | 0.025 | 65.9% |
| carrier_disp_madd_ifchainlin | 0.027 | 73.6% |
| carrier_disp_madd_nullfloor | 0.035 | 93.8% |
| carrier_disp_madd_switch | 0.025 | 65.9% |
| carrier_disp_madd_threaded | 0.023 | 61.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 4736ns | 4736ns | -7.00% |
| carrier_disp_madd_fntable | 5190ns | 5190ns | +1.93% |
| carrier_disp_madd_ifchain | 4973ns | 4973ns | -2.33% |
| carrier_disp_madd_ifchainasc | 4972ns | 4972ns | -2.35% |
| carrier_disp_madd_ifchainlin | 4750ns | 4750ns | -6.71% |
| carrier_disp_madd_nullfloor | 4161ns | 4161ns | -18.28% |
| carrier_disp_madd_switch | 5092ns | 5092ns | base |
| carrier_disp_madd_threaded | 5154ns | 5154ns | +1.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 2606ns | base | --- | [2480, 2665] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 2409ns | -85.2ns (-3.3%) | [-378, -24]ns | [2265, 2590] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 2888ns | no significant difference | [-55, +411]ns | [2588, 2914] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_madd_ifchain | 2642ns | no significant difference | [-83, +68]ns | [2397, 2718] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_madd_ifchainasc | 2606ns | no significant difference | [-160, +76]ns | [2386, 2682] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_madd_ifchainlin | 2335ns | -159.0ns (-6.1%) | [-453, -130]ns | [2190, 2485] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 1831ns | -693.1ns (-26.6%) | [-914, -630]ns | [1729, 1954] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 2788ns | no significant difference | [-17, +394]ns | [2626, 3003] | no | 0.2552 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2377ns | -4.3% | +21.7% | +0.2% | -0.3% | -6.7% | -26.6% | +10.9% |
| 2 | 2627ns | -1.5% | +10.7% | +1.2% | -0.7% | -5.3% | -27.0% | +12.2% |
| 3 | 2583ns | -12.4% | +0.9% | -6.6% | +0.8% | -14.4% | -33.5% | +1.6% |
| 4 | 2611ns | -2.6% | +11.8% | +0.6% | +4.2% | -6.1% | -25.9% | +17.2% |
| 5 | 2703ns | -16.1% | -4.9% | +0.9% | -11.1% | -19.8% | -35.6% | -2.8% |
| 6 | 2602ns | -0.3% | +10.8% | +4.1% | +1.6% | -4.6% | -24.1% | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.808 | HIGH- (thermal bounce) |
| carrier_disp_madd_fntable | -0.617 | HIGH- (thermal bounce) |
| carrier_disp_madd_ifchain | -0.098 | ok |
| carrier_disp_madd_ifchainasc | -0.389 | moderate- |
| carrier_disp_madd_ifchainlin | -0.825 | HIGH- (thermal bounce) |
| carrier_disp_madd_nullfloor | -0.768 | HIGH- (thermal bounce) |
| carrier_disp_madd_switch | -0.059 | ok |
| carrier_disp_madd_threaded | -0.841 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 1/6, lost 5/6
- **carrier_disp_madd_ifchain**: won 1/6, lost 5/6
- **carrier_disp_madd_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 85795.0ns | 2421.3ns | 3543.4% | HIGH |
| carrier_disp_madd_fntable | 83603.0ns | 2796.4ns | 2989.7% | HIGH |
| carrier_disp_madd_ifchain | 86582.1ns | 2585.6ns | 3348.6% | HIGH |
| carrier_disp_madd_ifchainasc | 86127.9ns | 2558.1ns | 3366.8% | HIGH |
| carrier_disp_madd_ifchainlin | 86518.4ns | 2336.6ns | 3702.7% | HIGH |
| carrier_disp_madd_nullfloor | 85735.9ns | 1838.0ns | 4664.5% | HIGH |
| carrier_disp_madd_switch | 86621.0ns | 2583.8ns | 3352.5% | HIGH |
| carrier_disp_madd_threaded | 88193.6ns | 2805.7ns | 3143.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 2262.5-2590.4 ns)
   2262.5 |########################################
   2278.9 |
   2295.3 |
   2311.7 |
   2328.1 |
   2344.5 |
   2360.9 |
   2377.3 |
   2393.7 |
   2410.1 |
   2426.4 |
   2442.8 |
   2459.2 |
   2475.6 |
   2492.0 |
   2508.4 |
   2524.8 |
   2541.2 |#############
   2557.6 |
   2574.0 |#############
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 2569.6-2913.5 ns)
   2569.6 |####################
   2586.8 |
   2604.0 |####################
   2621.2 |
   2638.4 |
   2655.6 |
   2672.8 |
   2690.0 |
   2707.2 |
   2724.4 |
   2741.6 |
   2758.7 |
   2775.9 |
   2793.1 |
   2810.3 |
   2827.5 |
   2844.7 |
   2861.9 |
   2879.1 |########################################
   2896.3 |####################
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 2382.5-2717.9 ns)
   2382.5 |########################################
   2399.3 |########################################
   2416.0 |
   2432.8 |
   2449.6 |
   2466.3 |
   2483.1 |
   2499.9 |
   2516.7 |
   2533.4 |
   2550.2 |
   2567.0 |
   2583.7 |
   2600.5 |
   2617.3 |########################################
   2634.1 |
   2650.8 |########################################
   2667.6 |
   2684.4 |
   2701.1 |########################################
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 2369.6-2682.1 ns)
   2369.6 |####################
   2385.2 |
   2400.8 |####################
   2416.5 |
   2432.1 |
   2447.7 |
   2463.3 |
   2479.0 |
   2494.6 |
   2510.2 |
   2525.8 |
   2541.5 |
   2557.1 |
   2572.7 |
   2588.3 |
   2604.0 |########################################
   2619.6 |
   2635.2 |####################
   2650.8 |
   2666.5 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 2169.2-2484.6 ns)
   2169.2 |########################################
   2185.0 |
   2200.7 |########################################
   2216.5 |########################################
   2232.3 |
   2248.0 |
   2263.8 |
   2279.6 |
   2295.3 |
   2311.1 |
   2326.9 |
   2342.6 |
   2358.4 |
   2374.2 |
   2389.9 |
   2405.7 |
   2421.5 |
   2437.2 |########################################
   2453.0 |
   2468.8 |########################################
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 1718.3-1953.7 ns)
   1718.3 |########################################
   1730.1 |########################################
   1741.8 |########################################
   1753.6 |
   1765.4 |
   1777.2 |
   1788.9 |
   1800.7 |
   1812.5 |
   1824.2 |
   1836.0 |
   1847.8 |
   1859.5 |
   1871.3 |
   1883.1 |
   1894.8 |
   1906.6 |########################################
   1918.4 |
   1930.2 |########################################
   1941.9 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 2376.7-2665.0 ns)
   2376.7 |########################################
   2391.1 |
   2405.5 |
   2419.9 |
   2434.4 |
   2448.8 |
   2463.2 |
   2477.6 |
   2492.0 |
   2506.4 |
   2520.8 |
   2535.3 |
   2549.7 |
   2564.1 |
   2578.5 |########################################
   2592.9 |########################################
   2607.3 |########################################
   2621.8 |########################################
   2636.2 |
   2650.6 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 2624.6-3003.3 ns)
   2624.6 |########################################
   2643.5 |
   2662.5 |
   2681.4 |
   2700.3 |
   2719.3 |
   2738.2 |
   2757.1 |
   2776.1 |
   2795.0 |
   2813.9 |
   2832.9 |
   2851.8 |
   2870.8 |
   2889.7 |
   2908.6 |
   2927.6 |##########################
   2946.5 |
   2965.4 |
   2984.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=3563.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=2895.9% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=3282.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=3301.8% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=3703.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=4687.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=3324.8% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=3159.9% of algo (FFI overhead may distort results)

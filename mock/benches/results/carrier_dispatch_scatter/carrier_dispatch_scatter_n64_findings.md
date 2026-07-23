# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 37% faster than the next best (carrier_disp_scatter_switch)

carrier_disp_scatter_nullfloor (1.78 us) leads carrier_disp_scatter_switch (2.44 us) by 37%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 27% (significant)

carrier_disp_scatter_nullfloor is -664 ns (27%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 3.2x slower than the field

carrier_disp_scatter_ifchainlin (5.75 us) is 3.2x the fastest (1.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_threaded shows alternating (throttle bounce) (autocorr -0.61)

carrier_disp_scatter_threaded's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor, carrier_disp_scatter_switch, carrier_disp_scatter_ifchain, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} vs {carrier_disp_scatter_ifchainlin} (81% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor, carrier_disp_scatter_switch, carrier_disp_scatter_ifchain, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} and a slow tier {carrier_disp_scatter_ifchainlin} with a 81% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest carrier_disp_scatter_nullfloor (1.78 us) to slowest carrier_disp_scatter_ifchainlin (5.75 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_scatter_ifchain's edge over baseline is significant but tiny (24 ns, 0.99%)

carrier_disp_scatter_ifchain differs from baseline carrier_disp_scatter_switch by 24 ns (0.99%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 1781.7 ns median (-27.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.23x (fastest 1781.7 ns, slowest 5751.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 5278ns | 5451ns | 4793ns | 5248ns | 5566ns | +10.54% |
| carrier_disp_scatter_fntable | 5679ns | 5680ns | 5622ns | 5680ns | 5706ns | +18.93% |
| carrier_disp_scatter_ifchain | 4851ns | 5010ns | 4440ns | 4822ns | 5101ns | +1.60% |
| carrier_disp_scatter_ifchainasc | 4841ns | 5028ns | 4415ns | 4834ns | 5066ns | +1.39% |
| carrier_disp_scatter_ifchainlin | 7974ns | 8155ns | 7349ns | 7929ns | 8353ns | +66.99% |
| carrier_disp_scatter_nullfloor | 4211ns | 4272ns | 3813ns | 4253ns | 4349ns | -11.80% |
| carrier_disp_scatter_switch | 4775ns | 4949ns | 4360ns | 4762ns | 5002ns | base |
| carrier_disp_scatter_threaded | 5172ns | 5303ns | 4693ns | 5146ns | 5452ns | +8.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 2878ns | 2631ns | 3001ns | +20.97% | 0.022 |
| carrier_disp_scatter_fntable | 3192ns | 3162ns | 3219ns | +34.18% | 0.020 |
| carrier_disp_scatter_ifchain | 2423ns | 2218ns | 2534ns | +1.83% | 0.026 |
| carrier_disp_scatter_ifchainasc | 2429ns | 2188ns | 2549ns | +2.12% | 0.026 |
| carrier_disp_scatter_ifchainlin | 5586ns | 5167ns | 5814ns | +134.80% | 0.011 |
| carrier_disp_scatter_nullfloor | 1740ns | 1557ns | 1792ns | -26.87% | 0.037 |
| carrier_disp_scatter_switch | 2379ns | 2210ns | 2485ns | base | 0.027 |
| carrier_disp_scatter_threaded | 2763ns | 2544ns | 2885ns | +16.14% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 261573 | 1165840 | 0.224 | 0.99× |
| carrier_disp_scatter_fntable | 253918 | 1382320 | 0.184 | 0.96× |
| carrier_disp_scatter_ifchain | 263563 | 1360824 | 0.194 | 1.00× |
| carrier_disp_scatter_ifchainasc | 261874 | 1354151 | 0.193 | 0.99× |
| carrier_disp_scatter_ifchainlin | 276300 | 1718226 | 0.161 | 1.04× |
| carrier_disp_scatter_nullfloor | 254040 | 1585789 | 0.160 | 0.96× |
| carrier_disp_scatter_switch | 264633 | 1342360 | 0.197 | 1.00× |
| carrier_disp_scatter_threaded | 266882 | 1643762 | 0.162 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.021 | 52.1% |
| carrier_disp_scatter_fntable | 0.020 | 48.9% |
| carrier_disp_scatter_ifchain | 0.026 | 62.5% |
| carrier_disp_scatter_ifchainasc | 0.025 | 61.8% |
| carrier_disp_scatter_ifchainlin | 0.011 | 27.1% |
| carrier_disp_scatter_nullfloor | 0.036 | 87.4% |
| carrier_disp_scatter_switch | 0.026 | 63.8% |
| carrier_disp_scatter_threaded | 0.022 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 5278ns | 5278ns | +10.54% |
| carrier_disp_scatter_fntable | 5679ns | 5679ns | +18.93% |
| carrier_disp_scatter_ifchain | 4851ns | 4851ns | +1.60% |
| carrier_disp_scatter_ifchainasc | 4841ns | 4841ns | +1.39% |
| carrier_disp_scatter_ifchainlin | 7974ns | 7974ns | +66.99% |
| carrier_disp_scatter_nullfloor | 4211ns | 4211ns | -11.80% |
| carrier_disp_scatter_switch | 4775ns | 4775ns | base |
| carrier_disp_scatter_threaded | 5172ns | 5172ns | +8.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 2442ns | base | --- | [2210, 2485] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 2987ns | +511.5ns (+20.9%) | [+436, +549]ns | [2646, 3001] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 3185ns | +743.1ns (+30.4%) | [+689, +1008]ns | [3172, 3219] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 2492ns | no significant difference | [-90, +196]ns | [2241, 2534] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainasc | 2518ns | no significant difference | [-220, +339]ns | [2221, 2549] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainlin | 5751ns | +3266.4ns (+133.8%) | [+2877, +3477]ns | [5192, 5814] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 1782ns | -663.9ns (-27.2%) | [-729, -525]ns | [1646, 1792] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 2860ns | +380.0ns (+15.6%) | [+230, +542]ns | [2544, 2885] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2465ns | +21.8% | +29.3% | +2.3% | -11.3% | +131.4% | -27.4% | +17.8% |
| 2 | 2211ns | +20.4% | +47.0% | +15.2% | +15.9% | +133.7% | -19.3% | +15.1% |
| 3 | 2505ns | +19.6% | +27.2% | -1.0% | +0.7% | +131.5% | -29.0% | +14.3% |
| 4 | 2418ns | +23.2% | +31.6% | -6.3% | -6.7% | +115.7% | -25.8% | +5.2% |
| 5 | 2465ns | +21.6% | +28.3% | +1.6% | +2.0% | +135.6% | -29.6% | +16.3% |
| 6 | 2210ns | +19.1% | +44.1% | +0.4% | +14.8% | +163.4% | -29.5% | +29.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.360 | moderate- |
| carrier_disp_scatter_fntable | -0.005 | ok |
| carrier_disp_scatter_ifchain | -0.191 | ok |
| carrier_disp_scatter_ifchainasc | -0.312 | moderate- |
| carrier_disp_scatter_ifchainlin | -0.519 | HIGH- (thermal bounce) |
| carrier_disp_scatter_nullfloor | 0.159 | ok |
| carrier_disp_scatter_switch | -0.470 | moderate- |
| carrier_disp_scatter_threaded | -0.606 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 0/6, lost 6/6
- **carrier_disp_scatter_fntable**: won 0/6, lost 6/6
- **carrier_disp_scatter_ifchain**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 85425.3ns | 2877.9ns | 2968.3% | HIGH |
| carrier_disp_scatter_fntable | 85905.8ns | 3192.2ns | 2691.1% | HIGH |
| carrier_disp_scatter_ifchain | 85999.3ns | 2422.7ns | 3549.8% | HIGH |
| carrier_disp_scatter_ifchainasc | 85771.8ns | 2429.5ns | 3530.5% | HIGH |
| carrier_disp_scatter_ifchainlin | 87047.0ns | 5585.9ns | 1558.3% | HIGH |
| carrier_disp_scatter_nullfloor | 85765.8ns | 1739.8ns | 4929.6% | HIGH |
| carrier_disp_scatter_switch | 85931.0ns | 2379.0ns | 3612.0% | HIGH |
| carrier_disp_scatter_threaded | 87212.0ns | 2763.0ns | 3156.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 2631.2-3000.6 ns)
   2631.2 |####################
   2649.7 |####################
   2668.1 |
   2686.6 |
   2705.1 |
   2723.6 |
   2742.0 |
   2760.5 |
   2779.0 |
   2797.4 |
   2815.9 |
   2834.4 |
   2852.8 |
   2871.3 |
   2889.8 |
   2908.2 |
   2926.7 |
   2945.2 |
   2963.7 |####################
   2982.1 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 3162.5-3219.4 ns)
   3162.5 |####################
   3165.3 |
   3168.2 |
   3171.0 |
   3173.9 |
   3176.7 |
   3179.6 |####################
   3182.4 |####################
   3185.2 |########################################
   3188.1 |
   3190.9 |
   3193.8 |
   3196.6 |
   3199.5 |
   3202.3 |
   3205.1 |
   3208.0 |
   3210.8 |
   3213.7 |
   3216.5 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 2217.9-2534.4 ns)
   2217.9 |########################################
   2233.7 |
   2249.6 |########################################
   2265.4 |
   2281.2 |
   2297.0 |
   2312.8 |
   2328.7 |
   2344.5 |
   2360.3 |
   2376.1 |
   2392.0 |
   2407.8 |
   2423.6 |
   2439.4 |
   2455.3 |
   2471.1 |########################################
   2486.9 |
   2502.7 |########################################
   2518.6 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 2187.5-2548.8 ns)
   2187.5 |####################
   2205.6 |
   2223.6 |
   2241.7 |####################
   2259.8 |
   2277.8 |
   2295.9 |
   2313.9 |
   2332.0 |
   2350.1 |
   2368.1 |
   2386.2 |
   2404.2 |
   2422.3 |
   2440.4 |
   2458.4 |
   2476.5 |
   2494.6 |
   2512.6 |########################################
   2530.7 |####################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 5166.7-5814.4 ns)
   5166.7 |####################
   5199.1 |####################
   5231.5 |
   5263.8 |
   5296.2 |
   5328.6 |
   5361.0 |
   5393.4 |
   5425.8 |
   5458.1 |
   5490.5 |
   5522.9 |
   5555.3 |
   5587.7 |
   5620.1 |
   5652.4 |
   5684.8 |####################
   5717.2 |
   5749.6 |
   5782.0 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 1556.7-1792.1 ns)
   1556.7 |####################
   1568.5 |
   1580.2 |
   1592.0 |
   1603.8 |
   1615.5 |
   1627.3 |
   1639.1 |
   1650.9 |
   1662.6 |
   1674.4 |
   1686.2 |
   1697.9 |
   1709.7 |
   1721.5 |
   1733.2 |####################
   1745.0 |
   1756.8 |
   1768.6 |####################
   1780.3 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 2209.6-2485.2 ns)
   2209.6 |########################################
   2223.4 |
   2237.2 |
   2250.9 |
   2264.7 |
   2278.5 |
   2292.3 |
   2306.1 |
   2319.8 |
   2333.6 |
   2347.4 |
   2361.2 |
   2375.0 |
   2388.7 |
   2402.5 |
   2416.3 |####################
   2430.1 |
   2443.9 |
   2457.6 |########################################
   2471.4 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 2543.7-2884.8 ns)
   2543.7 |##########################
   2560.8 |
   2577.8 |
   2594.9 |
   2611.9 |
   2629.0 |
   2646.0 |
   2663.1 |
   2680.1 |
   2697.2 |
   2714.2 |
   2731.3 |
   2748.4 |
   2765.4 |
   2782.5 |
   2799.5 |
   2816.6 |
   2833.6 |
   2850.7 |########################################
   2867.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=2858.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=2697.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=3453.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=3411.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=1510.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=4813.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=3516.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=3079.4% of algo (FFI overhead may distort results)

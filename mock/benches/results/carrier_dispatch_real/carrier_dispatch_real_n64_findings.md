# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 46% faster than the next best (carrier_disp_real_ifchainasc)

carrier_disp_real_nullfloor (1.58 us) leads carrier_disp_real_ifchainasc (2.30 us) by 46%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 31% (significant)

carrier_disp_real_nullfloor is -726 ns (31%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 3.3x slower than the field

carrier_disp_real_ifchainlin (5.16 us) is 3.3x the fastest (1.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_fntable shows alternating (throttle bounce) (autocorr -0.85)

carrier_disp_real_fntable's per-pass series has lag-1 autocorrelation -0.85, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_ifchainasc, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} vs {carrier_disp_real_ifchainlin} (74% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_ifchainasc, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} and a slow tier {carrier_disp_real_ifchainlin} with a 74% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest carrier_disp_real_nullfloor (1.58 us) to slowest carrier_disp_real_ifchainlin (5.16 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_real_ifchainasc's edge over baseline is significant but tiny (-19 ns, 0.80%)

carrier_disp_real_ifchainasc differs from baseline carrier_disp_real_switch by -19 ns (0.80%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 1576.8 ns median (-33.2% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.27x (fastest 1576.8 ns, slowest 5162.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 5351ns | 5430ns | 4820ns | 5364ns | 5598ns | +10.88% |
| carrier_disp_real_fntable | 5441ns | 5415ns | 4976ns | 5276ns | 5920ns | +12.73% |
| carrier_disp_real_ifchain | 4805ns | 4838ns | 4450ns | 4796ns | 4997ns | -0.44% |
| carrier_disp_real_ifchainasc | 4697ns | 4719ns | 4356ns | 4650ns | 4937ns | -2.69% |
| carrier_disp_real_ifchainlin | 7533ns | 7646ns | 7037ns | 7463ns | 7885ns | +56.08% |
| carrier_disp_real_nullfloor | 4082ns | 3995ns | 3680ns | 3958ns | 4467ns | -15.43% |
| carrier_disp_real_switch | 4826ns | 4988ns | 4360ns | 4794ns | 5108ns | base |
| carrier_disp_real_threaded | 5369ns | 5438ns | 4821ns | 5407ns | 5585ns | +11.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 2847ns | 2608ns | 2943ns | +22.73% | 0.022 |
| carrier_disp_real_fntable | 2966ns | 2774ns | 3140ns | +27.87% | 0.022 |
| carrier_disp_real_ifchain | 2274ns | 2113ns | 2360ns | -1.99% | 0.028 |
| carrier_disp_real_ifchainasc | 2273ns | 2103ns | 2404ns | -2.03% | 0.028 |
| carrier_disp_real_ifchainlin | 5054ns | 4708ns | 5244ns | +117.87% | 0.013 |
| carrier_disp_real_nullfloor | 1592ns | 1476ns | 1696ns | -31.39% | 0.040 |
| carrier_disp_real_switch | 2320ns | 2125ns | 2468ns | base | 0.028 |
| carrier_disp_real_threaded | 2775ns | 2532ns | 2850ns | +19.65% | 0.023 |

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.022 | 50.8% |
| carrier_disp_real_fntable | 0.022 | 49.7% |
| carrier_disp_real_ifchain | 0.028 | 63.8% |
| carrier_disp_real_ifchainasc | 0.028 | 64.2% |
| carrier_disp_real_ifchainlin | 0.012 | 28.6% |
| carrier_disp_real_nullfloor | 0.041 | 93.6% |
| carrier_disp_real_switch | 0.027 | 62.5% |
| carrier_disp_real_threaded | 0.023 | 52.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 5351ns | 5351ns | +10.88% |
| carrier_disp_real_fntable | 5441ns | 5441ns | +12.73% |
| carrier_disp_real_ifchain | 4805ns | 4805ns | -0.44% |
| carrier_disp_real_ifchainasc | 4697ns | 4697ns | -2.69% |
| carrier_disp_real_ifchainlin | 7533ns | 7533ns | +56.08% |
| carrier_disp_real_nullfloor | 4082ns | 4082ns | -15.43% |
| carrier_disp_real_switch | 4826ns | 4826ns | base |
| carrier_disp_real_threaded | 5369ns | 5369ns | +11.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 2361ns | base | --- | [2130, 2468] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 2905ns | +462.7ns (+19.6%) | [+432, +687]ns | [2692, 2943] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_fntable | 2969ns | +681.0ns (+28.8%) | [+482, +776]ns | [2789, 3140] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 2315ns | no significant difference | [-200, +114]ns | [2146, 2360] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_real_ifchainasc | 2298ns | no significant difference | [-290, +168]ns | [2116, 2404] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainlin | 5163ns | +2695.0ns (+114.1%) | [+2624, +2883]ns | [4754, 5244] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 1577ns | -726.5ns (-30.8%) | [-905, -553]ns | [1501, 1696] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_threaded | 2815ns | +406.5ns (+17.2%) | [+380, +581]ns | [2661, 2850] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2334ns | +19.0% | +33.9% | +0.5% | -9.9% | +123.1% | -36.8% | +20.3% |
| 2 | 2478ns | +18.3% | +11.9% | -12.1% | -14.1% | +108.9% | -38.4% | +14.4% |
| 3 | 2388ns | +23.5% | +31.8% | -4.1% | +0.7% | +121.1% | -28.1% | +16.9% |
| 4 | 2135ns | +22.1% | +31.3% | -1.0% | +11.7% | +120.5% | -27.3% | +32.3% |
| 5 | 2458ns | +17.1% | +27.4% | -3.4% | -2.3% | +109.5% | -31.8% | +16.5% |
| 6 | 2125ns | +38.2% | +32.4% | +10.1% | +4.1% | +126.0% | -24.7% | +19.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.298 | moderate- |
| carrier_disp_real_fntable | -0.854 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchain | -0.371 | moderate- |
| carrier_disp_real_ifchainasc | 0.266 | moderate+ |
| carrier_disp_real_ifchainlin | -0.313 | moderate- |
| carrier_disp_real_nullfloor | -0.190 | ok |
| carrier_disp_real_switch | -0.428 | moderate- |
| carrier_disp_real_threaded | -0.182 | ok |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 0/6, lost 6/6
- **carrier_disp_real_fntable**: won 0/6, lost 6/6
- **carrier_disp_real_ifchain**: won 4/6, lost 2/6
- **carrier_disp_real_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 86821.7ns | 2846.8ns | 3049.8% | HIGH |
| carrier_disp_real_fntable | 86795.6ns | 2966.0ns | 2926.3% | HIGH |
| carrier_disp_real_ifchain | 86021.7ns | 2273.5ns | 3783.6% | HIGH |
| carrier_disp_real_ifchainasc | 86562.3ns | 2272.5ns | 3809.0% | HIGH |
| carrier_disp_real_ifchainlin | 87553.8ns | 5053.7ns | 1732.5% | HIGH |
| carrier_disp_real_nullfloor | 86152.2ns | 1591.5ns | 5413.2% | HIGH |
| carrier_disp_real_switch | 86592.2ns | 2319.6ns | 3733.1% | HIGH |
| carrier_disp_real_threaded | 88791.5ns | 2775.5ns | 3199.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 2607.5-2943.3 ns)
   2607.5 |####################
   2624.3 |
   2641.1 |
   2657.9 |
   2674.7 |
   2691.5 |
   2708.3 |
   2725.0 |
   2741.8 |
   2758.6 |
   2775.4 |####################
   2792.2 |
   2809.0 |
   2825.8 |
   2842.6 |
   2859.4 |
   2876.2 |####################
   2893.0 |
   2909.8 |
   2926.6 |########################################
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 2773.7-3139.8 ns)
   2773.7 |####################
   2792.0 |####################
   2810.3 |####################
   2828.6 |
   2846.9 |
   2865.2 |
   2883.5 |
   2901.8 |
   2920.1 |
   2938.4 |
   2956.7 |
   2975.0 |
   2993.3 |
   3011.6 |
   3029.9 |
   3048.2 |
   3066.5 |
   3084.8 |
   3103.1 |
   3121.4 |########################################
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 2112.9-2360.4 ns)
   2112.9 |####################
   2125.3 |
   2137.7 |
   2150.0 |
   2162.4 |
   2174.8 |####################
   2187.2 |
   2199.5 |
   2211.9 |
   2224.3 |
   2236.7 |
   2249.0 |
   2261.4 |
   2273.8 |
   2286.2 |####################
   2298.5 |
   2310.9 |
   2323.3 |
   2335.7 |########################################
   2348.0 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 2103.3-2403.9 ns)
   2103.3 |########################################
   2118.3 |########################################
   2133.4 |
   2148.4 |
   2163.4 |
   2178.5 |
   2193.5 |
   2208.5 |########################################
   2223.6 |
   2238.6 |
   2253.6 |
   2268.7 |
   2283.7 |
   2298.7 |
   2313.8 |
   2328.8 |
   2343.8 |
   2358.9 |
   2373.9 |########################################
   2388.9 |########################################
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 4707.5-5244.0 ns)
   4707.5 |########################################
   4734.3 |
   4761.1 |
   4788.0 |########################################
   4814.8 |
   4841.6 |
   4868.4 |
   4895.3 |
   4922.1 |
   4948.9 |
   4975.7 |
   5002.5 |
   5029.4 |
   5056.2 |
   5083.0 |
   5109.8 |
   5136.7 |########################################
   5163.5 |########################################
   5190.3 |########################################
   5217.1 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 1475.8-1696.5 ns)
   1475.8 |########################################
   1486.8 |
   1497.9 |
   1508.9 |
   1519.9 |########################################
   1531.0 |
   1542.0 |########################################
   1553.0 |
   1564.1 |
   1575.1 |
   1586.1 |
   1597.2 |########################################
   1608.2 |
   1619.2 |
   1630.3 |
   1641.3 |
   1652.3 |
   1663.4 |
   1674.4 |########################################
   1685.4 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 2124.6-2467.9 ns)
   2124.6 |########################################
   2141.8 |
   2158.9 |
   2176.1 |
   2193.3 |
   2210.4 |
   2227.6 |
   2244.8 |
   2261.9 |
   2279.1 |
   2296.2 |
   2313.4 |
   2330.6 |####################
   2347.7 |
   2364.9 |
   2382.1 |####################
   2399.2 |
   2416.4 |
   2433.6 |
   2450.7 |####################
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 2531.7-2849.8 ns)
   2531.7 |########################################
   2547.6 |
   2563.5 |
   2579.4 |
   2595.3 |
   2611.2 |
   2627.1 |
   2643.0 |
   2658.9 |
   2674.8 |
   2690.7 |
   2706.6 |
   2722.5 |
   2738.4 |
   2754.3 |
   2770.2 |
   2786.1 |########################################
   2802.0 |########################################
   2817.9 |########################################
   2833.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=2987.9% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=2924.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=3716.9% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=3769.9% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=1691.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=5462.5% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=3679.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=3138.1% of algo (FFI overhead may distort results)

# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 32% faster than the next best (carrier_disp_real_ifchain)

carrier_disp_real_nullfloor (1.70 us) leads carrier_disp_real_ifchain (2.25 us) by 32%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 30% (significant)

carrier_disp_real_nullfloor is -710 ns (30%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 2.9x slower than the field

carrier_disp_real_ifchainlin (4.89 us) is 2.9x the fastest (1.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_ifchainlin shows alternating (throttle bounce) (autocorr -0.82)

carrier_disp_real_ifchainlin's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_bittree, carrier_disp_real_threaded, carrier_disp_real_fntable} vs {carrier_disp_real_ifchainlin} (60% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_bittree, carrier_disp_real_threaded, carrier_disp_real_fntable} and a slow tier {carrier_disp_real_ifchainlin} with a 60% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_real_ifchain's edge over baseline is significant but tiny (-24 ns, 0.98%)

carrier_disp_real_ifchain differs from baseline carrier_disp_real_switch by -24 ns (0.98%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 1700.8 ns median (-29.3% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.87x (fastest 1700.8 ns, slowest 4886.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 5081ns | 5051ns | 4756ns | 4957ns | 5429ns | +8.53% |
| carrier_disp_real_fntable | 5537ns | 5539ns | 5443ns | 5525ns | 5602ns | +18.27% |
| carrier_disp_real_ifchain | 4600ns | 4601ns | 4291ns | 4505ns | 4898ns | -1.74% |
| carrier_disp_real_ifchainasc | 4722ns | 4879ns | 4249ns | 4684ns | 5015ns | +0.86% |
| carrier_disp_real_ifchainlin | 7276ns | 7208ns | 6825ns | 7092ns | 7776ns | +55.41% |
| carrier_disp_real_nullfloor | 4072ns | 4184ns | 3652ns | 4108ns | 4229ns | -13.02% |
| carrier_disp_real_switch | 4682ns | 4842ns | 4289ns | 4671ns | 4894ns | base |
| carrier_disp_real_threaded | 5091ns | 5265ns | 4681ns | 5079ns | 5314ns | +8.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 2744ns | 2585ns | 2919ns | +18.05% | 0.023 |
| carrier_disp_real_fntable | 3059ns | 3012ns | 3087ns | +31.59% | 0.021 |
| carrier_disp_real_ifchain | 2256ns | 2124ns | 2394ns | -2.94% | 0.028 |
| carrier_disp_real_ifchainasc | 2318ns | 2110ns | 2435ns | -0.30% | 0.028 |
| carrier_disp_real_ifchainlin | 4927ns | 4648ns | 5243ns | +111.93% | 0.013 |
| carrier_disp_real_nullfloor | 1655ns | 1502ns | 1708ns | -28.81% | 0.039 |
| carrier_disp_real_switch | 2325ns | 2142ns | 2428ns | base | 0.028 |
| carrier_disp_real_threaded | 2731ns | 2515ns | 2858ns | +17.46% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_real_bittree | 268889 | 1204121 | 0.223 | 1.03× |
| carrier_disp_real_fntable | 255420 | 1378337 | 0.185 | 0.98× |
| carrier_disp_real_ifchain | 266749 | 1410077 | 0.189 | 1.02× |
| carrier_disp_real_ifchainasc | 260811 | 1372854 | 0.190 | 1.00× |
| carrier_disp_real_ifchainlin | 280495 | 1693111 | 0.166 | 1.07× |
| carrier_disp_real_nullfloor | 255865 | 1603233 | 0.160 | 0.98× |
| carrier_disp_real_switch | 261407 | 1325991 | 0.197 | 1.00× |
| carrier_disp_real_threaded | 268986 | 1613651 | 0.167 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.023 | 55.1% |
| carrier_disp_real_fntable | 0.021 | 49.1% |
| carrier_disp_real_ifchain | 0.028 | 66.8% |
| carrier_disp_real_ifchainasc | 0.027 | 62.6% |
| carrier_disp_real_ifchainlin | 0.013 | 30.7% |
| carrier_disp_real_nullfloor | 0.038 | 88.3% |
| carrier_disp_real_switch | 0.027 | 62.5% |
| carrier_disp_real_threaded | 0.023 | 53.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 5081ns | 5081ns | +8.53% |
| carrier_disp_real_fntable | 5537ns | 5537ns | +18.27% |
| carrier_disp_real_ifchain | 4600ns | 4600ns | -1.74% |
| carrier_disp_real_ifchainasc | 4722ns | 4722ns | +0.86% |
| carrier_disp_real_ifchainlin | 7276ns | 7276ns | +55.41% |
| carrier_disp_real_nullfloor | 4072ns | 4072ns | -13.02% |
| carrier_disp_real_switch | 4682ns | 4682ns | base |
| carrier_disp_real_threaded | 5091ns | 5091ns | +8.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 2405ns | base | --- | [2142, 2428] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 2728ns | +443.5ns (+18.4%) | [+301, +515]ns | [2586, 2919] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_fntable | 3061ns | +666.1ns (+27.7%) | [+607, +930]ns | [3029, 3087] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 2250ns | -23.5ns (-1.0%) | [-178, -4]ns | [2125, 2394] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_real_ifchainasc | 2401ns | no significant difference | [-297, +277]ns | [2118, 2435] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainlin | 4887ns | +2615.6ns (+108.8%) | [+2371, +2821]ns | [4652, 5243] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 1701ns | -709.5ns (-29.5%) | [-764, -536]ns | [1556, 1708] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_threaded | 2814ns | +407.9ns (+17.0%) | [+238, +571]ns | [2520, 2858] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2410ns | +21.6% | +26.4% | +0.3% | -12.4% | +114.0% | -33.2% | +17.9% |
| 2 | 2142ns | +20.7% | +44.6% | -0.7% | +14.4% | +117.0% | -20.2% | +17.4% |
| 3 | 2434ns | +16.7% | +23.8% | -2.7% | -0.6% | +118.9% | -29.9% | +17.6% |
| 4 | 2421ns | +8.1% | +27.0% | -12.0% | -12.2% | +92.3% | -29.8% | +4.3% |
| 5 | 2400ns | +21.2% | +28.2% | -1.2% | +0.6% | +112.6% | -29.1% | +16.1% |
| 6 | 2142ns | +20.7% | +42.2% | -0.9% | +11.5% | +118.1% | -29.9% | +33.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.757 | HIGH- (thermal bounce) |
| carrier_disp_real_fntable | -0.655 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchain | -0.778 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchainasc | -0.371 | moderate- |
| carrier_disp_real_ifchainlin | -0.817 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | -0.068 | ok |
| carrier_disp_real_switch | -0.312 | moderate- |
| carrier_disp_real_threaded | -0.619 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 0/6, lost 6/6
- **carrier_disp_real_fntable**: won 0/6, lost 6/6
- **carrier_disp_real_ifchain**: won 5/6, lost 1/6
- **carrier_disp_real_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 85794.9ns | 2744.5ns | 3126.1% | HIGH |
| carrier_disp_real_fntable | 85644.5ns | 3059.3ns | 2799.5% | HIGH |
| carrier_disp_real_ifchain | 85864.4ns | 2256.4ns | 3805.4% | HIGH |
| carrier_disp_real_ifchainasc | 85737.7ns | 2317.9ns | 3698.9% | HIGH |
| carrier_disp_real_ifchainlin | 87030.6ns | 4927.1ns | 1766.4% | HIGH |
| carrier_disp_real_nullfloor | 85306.0ns | 1655.0ns | 5154.4% | HIGH |
| carrier_disp_real_switch | 85619.6ns | 2324.9ns | 3682.8% | HIGH |
| carrier_disp_real_threaded | 87952.9ns | 2730.7ns | 3220.9% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 2585.4-2919.3 ns)
   2585.4 |########################################
   2602.1 |####################
   2618.8 |
   2635.5 |
   2652.2 |
   2668.9 |
   2685.6 |
   2702.3 |
   2719.0 |
   2735.7 |
   2752.4 |
   2769.1 |
   2785.8 |
   2802.5 |
   2819.2 |
   2835.9 |####################
   2852.6 |
   2869.3 |
   2886.0 |
   2902.7 |####################
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 3012.5-3087.3 ns)
   3012.5 |####################
   3016.2 |
   3020.0 |
   3023.7 |
   3027.5 |
   3031.2 |
   3034.9 |
   3038.7 |
   3042.4 |
   3046.2 |########################################
   3049.9 |
   3053.6 |
   3057.4 |
   3061.1 |
   3064.9 |
   3068.6 |
   3072.3 |####################
   3076.1 |####################
   3079.8 |
   3083.6 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 2123.8-2393.9 ns)
   2123.8 |########################################
   2137.3 |
   2150.8 |
   2164.3 |
   2177.8 |
   2191.3 |
   2204.8 |
   2218.4 |
   2231.9 |
   2245.4 |
   2258.9 |
   2272.4 |
   2285.9 |
   2299.4 |
   2312.9 |
   2326.4 |
   2339.9 |
   2353.4 |
   2366.9 |##########################
   2380.4 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 2110.4-2434.6 ns)
   2110.4 |########################################
   2126.6 |
   2142.8 |
   2159.0 |
   2175.2 |
   2191.4 |
   2207.7 |
   2223.9 |
   2240.1 |
   2256.3 |
   2272.5 |
   2288.7 |
   2304.9 |
   2321.1 |
   2337.3 |
   2353.6 |
   2369.8 |
   2386.0 |####################
   2402.2 |####################
   2418.4 |####################
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 4647.9-5242.7 ns)
   4647.9 |########################################
   4677.6 |
   4707.4 |
   4737.1 |
   4766.9 |
   4796.6 |
   4826.3 |
   4856.1 |
   4885.8 |
   4915.6 |
   4945.3 |
   4975.0 |
   5004.8 |
   5034.5 |
   5064.3 |
   5094.0 |#############
   5123.7 |
   5153.5 |#############
   5183.2 |
   5213.0 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 1502.5-1707.9 ns)
   1502.5 |#############
   1512.8 |
   1523.0 |
   1533.3 |
   1543.6 |
   1553.8 |
   1564.1 |
   1574.4 |
   1584.7 |
   1594.9 |
   1605.2 |#############
   1615.5 |
   1625.7 |
   1636.0 |
   1646.3 |
   1656.6 |
   1666.8 |
   1677.1 |
   1687.4 |
   1697.6 |########################################
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 2141.7-2427.7 ns)
   2141.7 |########################################
   2156.0 |
   2170.3 |
   2184.6 |
   2198.9 |
   2213.2 |
   2227.5 |
   2241.8 |
   2256.1 |
   2270.4 |
   2284.7 |
   2299.0 |
   2313.3 |
   2327.6 |
   2341.9 |
   2356.2 |
   2370.5 |
   2384.8 |
   2399.1 |########################################
   2413.4 |####################
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 2515.0-2858.2 ns)
   2515.0 |########################################
   2532.2 |
   2549.3 |
   2566.5 |
   2583.6 |
   2600.8 |
   2617.9 |
   2635.1 |
   2652.3 |
   2669.4 |
   2686.6 |
   2703.7 |
   2720.9 |
   2738.0 |
   2755.2 |
   2772.4 |####################
   2789.5 |
   2806.7 |
   2823.8 |
   2841.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=3154.6% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=2797.7% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=3802.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=3570.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=1780.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=5015.7% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=3561.7% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=3139.2% of algo (FFI overhead may distort results)

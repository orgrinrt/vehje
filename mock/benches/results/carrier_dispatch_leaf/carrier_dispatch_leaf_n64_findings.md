# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 27% faster than the next best (carrier_disp_leaf_ifchainasc)

carrier_disp_leaf_nullfloor (1.66 us) leads carrier_disp_leaf_ifchainasc (2.11 us) by 27%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 23% (significant)

carrier_disp_leaf_nullfloor is -488 ns (23%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_threaded shows alternating (throttle bounce) (autocorr -0.52)

carrier_disp_leaf_threaded's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_switch, carrier_disp_leaf_bittree, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded, carrier_disp_leaf_ifchainlin} (27% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_switch, carrier_disp_leaf_bittree, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded, carrier_disp_leaf_ifchainlin} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_leaf_ifchain's edge over baseline is significant but tiny (6 ns, 0.28%)

carrier_disp_leaf_ifchain differs from baseline carrier_disp_leaf_switch by 6 ns (0.28%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 1658.0 ns median (-22.8% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.83x (fastest 1658.0 ns, slowest 3026.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 4998ns | 4943ns | 4865ns | 4923ns | 5176ns | +7.76% |
| carrier_disp_leaf_fntable | 5200ns | 5193ns | 5146ns | 5181ns | 5256ns | +12.13% |
| carrier_disp_leaf_ifchain | 4698ns | 4662ns | 4575ns | 4646ns | 4837ns | +1.31% |
| carrier_disp_leaf_ifchainasc | 4590ns | 4652ns | 4114ns | 4620ns | 4783ns | -1.03% |
| carrier_disp_leaf_ifchainlin | 5603ns | 5571ns | 5508ns | 5555ns | 5724ns | +20.83% |
| carrier_disp_leaf_nullfloor | 4162ns | 4188ns | 3938ns | 4159ns | 4278ns | -10.25% |
| carrier_disp_leaf_switch | 4638ns | 4638ns | 4612ns | 4632ns | 4658ns | base |
| carrier_disp_leaf_threaded | 5337ns | 5299ns | 5262ns | 5293ns | 5442ns | +15.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 2424ns | 2392ns | 2468ns | +13.08% | 0.026 |
| carrier_disp_leaf_fntable | 2677ns | 2660ns | 2697ns | +24.91% | 0.024 |
| carrier_disp_leaf_ifchain | 2137ns | 2112ns | 2156ns | -0.30% | 0.030 |
| carrier_disp_leaf_ifchainasc | 2093ns | 1878ns | 2176ns | -2.34% | 0.031 |
| carrier_disp_leaf_ifchainlin | 3047ns | 2982ns | 3113ns | +42.16% | 0.021 |
| carrier_disp_leaf_nullfloor | 1637ns | 1500ns | 1682ns | -23.61% | 0.039 |
| carrier_disp_leaf_switch | 2143ns | 2127ns | 2152ns | base | 0.030 |
| carrier_disp_leaf_threaded | 2804ns | 2761ns | 2836ns | +30.84% | 0.023 |

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.027 | 62.3% |
| carrier_disp_leaf_fntable | 0.024 | 56.1% |
| carrier_disp_leaf_ifchain | 0.030 | 70.0% |
| carrier_disp_leaf_ifchainasc | 0.030 | 71.0% |
| carrier_disp_leaf_ifchainlin | 0.021 | 49.6% |
| carrier_disp_leaf_nullfloor | 0.039 | 90.5% |
| carrier_disp_leaf_switch | 0.030 | 69.9% |
| carrier_disp_leaf_threaded | 0.023 | 53.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 4998ns | 4998ns | +7.76% |
| carrier_disp_leaf_fntable | 5200ns | 5200ns | +12.13% |
| carrier_disp_leaf_ifchain | 4698ns | 4698ns | +1.31% |
| carrier_disp_leaf_ifchainasc | 4590ns | 4590ns | -1.03% |
| carrier_disp_leaf_ifchainlin | 5603ns | 5603ns | +20.83% |
| carrier_disp_leaf_nullfloor | 4162ns | 4162ns | -10.25% |
| carrier_disp_leaf_switch | 4638ns | 4638ns | base |
| carrier_disp_leaf_threaded | 5337ns | 5337ns | +15.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 2147ns | base | --- | [2130, 2152] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 2408ns | +263.3ns (+12.3%) | [+246, +332]ns | [2395, 2468] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 2675ns | +533.1ns (+24.8%) | [+508, +561]ns | [2660, 2697] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 2142ns | no significant difference | [-36, +11]ns | [2114, 2156] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainasc | 2113ns | no significant difference | [-160, +37]ns | [1990, 2176] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainlin | 3027ns | +885.8ns (+41.3%) | [+853, +973]ns | [3001, 3113] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 1658ns | -488.1ns (-22.7%) | [-575, -455]ns | [1572, 1682] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 2802ns | +668.9ns (+31.2%) | [+626, +688]ns | [2775, 2836] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2151ns | +12.1% | +23.7% | -1.6% | -12.7% | +38.7% | -22.6% | +28.4% |
| 2 | 2150ns | +12.3% | +25.0% | -1.8% | -2.2% | +40.7% | -30.2% | +32.1% |
| 3 | 2154ns | +11.4% | +23.5% | +0.2% | -1.9% | +41.6% | -23.2% | +29.8% |
| 4 | 2127ns | +13.0% | +27.1% | +0.7% | -0.6% | +49.3% | -20.1% | +32.0% |
| 5 | 2145ns | +11.5% | +25.4% | +0.4% | +1.6% | +41.3% | -22.5% | +32.0% |
| 6 | 2134ns | +18.1% | +24.8% | +0.4% | +1.8% | +41.6% | -23.0% | +30.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | -0.145 | ok |
| carrier_disp_leaf_fntable | -0.371 | moderate- |
| carrier_disp_leaf_ifchain | 0.123 | ok |
| carrier_disp_leaf_ifchainasc | 0.117 | ok |
| carrier_disp_leaf_ifchainlin | 0.006 | ok |
| carrier_disp_leaf_nullfloor | -0.138 | ok |
| carrier_disp_leaf_switch | -0.188 | ok |
| carrier_disp_leaf_threaded | -0.519 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 0/6, lost 6/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 2/6, lost 4/6
- **carrier_disp_leaf_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_leaf_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 86216.9ns | 2423.8ns | 3557.1% | HIGH |
| carrier_disp_leaf_fntable | 83790.8ns | 2677.3ns | 3129.7% | HIGH |
| carrier_disp_leaf_ifchain | 86221.2ns | 2137.0ns | 4034.7% | HIGH |
| carrier_disp_leaf_ifchainasc | 86021.9ns | 2093.2ns | 4109.5% | HIGH |
| carrier_disp_leaf_ifchainlin | 86934.2ns | 3047.1ns | 2853.0% | HIGH |
| carrier_disp_leaf_nullfloor | 86625.1ns | 1637.2ns | 5290.9% | HIGH |
| carrier_disp_leaf_switch | 86431.2ns | 2143.4ns | 4032.5% | HIGH |
| carrier_disp_leaf_threaded | 88082.3ns | 2804.4ns | 3140.9% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 2391.7-2467.7 ns)
   2391.7 |########################################
   2395.5 |########################################
   2399.3 |
   2403.1 |########################################
   2406.9 |
   2410.7 |########################################
   2414.5 |########################################
   2418.3 |
   2422.1 |
   2425.9 |
   2429.7 |
   2433.5 |
   2437.3 |
   2441.1 |
   2444.9 |
   2448.7 |
   2452.5 |
   2456.3 |
   2460.1 |
   2463.9 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 2660.0-2696.9 ns)
   2660.0 |########################################
   2661.8 |####################
   2663.7 |
   2665.5 |
   2667.4 |
   2669.2 |
   2671.1 |
   2672.9 |
   2674.7 |
   2676.6 |
   2678.4 |
   2680.3 |
   2682.1 |
   2684.0 |
   2685.8 |####################
   2687.6 |
   2689.5 |####################
   2691.3 |
   2693.2 |
   2695.0 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 2111.7-2155.6 ns)
   2111.7 |########################################
   2113.9 |########################################
   2116.1 |
   2118.3 |
   2120.5 |
   2122.7 |
   2124.9 |
   2127.1 |
   2129.3 |
   2131.5 |
   2133.7 |
   2135.8 |
   2138.0 |
   2140.2 |########################################
   2142.4 |########################################
   2144.6 |
   2146.8 |
   2149.0 |
   2151.2 |########################################
   2153.4 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 1877.5-2176.4 ns)
   1877.5 |#############
   1892.4 |
   1907.4 |
   1922.3 |
   1937.3 |
   1952.2 |
   1967.2 |
   1982.1 |
   1997.1 |
   2012.0 |
   2027.0 |
   2041.9 |
   2056.9 |
   2071.8 |
   2086.8 |
   2101.7 |########################################
   2116.7 |
   2131.6 |
   2146.6 |
   2161.5 |#############
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 2982.1-3113.1 ns)
   2982.1 |########################################
   2988.7 |
   2995.2 |
   3001.8 |
   3008.3 |
   3014.8 |########################################
   3021.4 |########################################
   3028.0 |########################################
   3034.5 |
   3041.1 |
   3047.6 |########################################
   3054.2 |
   3060.7 |
   3067.2 |
   3073.8 |
   3080.4 |
   3086.9 |
   3093.5 |
   3100.0 |
   3106.6 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 1500.0-1681.9 ns)
   1500.0 |########################################
   1509.1 |
   1518.2 |
   1527.3 |
   1536.4 |
   1545.5 |
   1554.6 |
   1563.7 |
   1572.8 |
   1581.9 |
   1591.0 |
   1600.0 |
   1609.1 |
   1618.2 |
   1627.3 |
   1636.4 |########################################
   1645.5 |########################################
   1654.6 |########################################
   1663.7 |########################################
   1672.8 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 2126.7-2152.5 ns)
   2126.7 |########################################
   2128.0 |
   2129.3 |
   2130.6 |
   2131.9 |
   2133.1 |########################################
   2134.4 |
   2135.7 |
   2137.0 |
   2138.3 |
   2139.6 |
   2140.9 |
   2142.2 |
   2143.5 |
   2144.8 |########################################
   2146.1 |
   2147.3 |
   2148.6 |########################################
   2149.9 |########################################
   2151.2 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 2760.8-2835.6 ns)
   2760.8 |########################################
   2764.5 |
   2768.3 |
   2772.0 |
   2775.8 |
   2779.5 |
   2783.2 |
   2787.0 |########################################
   2790.7 |
   2794.5 |########################################
   2798.2 |
   2801.9 |
   2805.7 |########################################
   2809.4 |
   2813.2 |
   2816.9 |
   2820.6 |
   2824.4 |
   2828.1 |########################################
   2831.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=3580.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=3131.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=4019.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=4074.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=2871.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=5228.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=4025.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=3138.2% of algo (FFI overhead may distort results)

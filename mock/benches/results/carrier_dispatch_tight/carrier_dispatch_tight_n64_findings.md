# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 24% faster than the next best (carrier_disp_tight_ifchainasc)

carrier_disp_tight_nullfloor (1.71 us) leads carrier_disp_tight_ifchainasc (2.13 us) by 24%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_ifchainlin is an outlier: 2.1x slower than the field

carrier_disp_tight_ifchainlin (3.64 us) is 2.1x the fastest (1.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_tight_fntable shows alternating (throttle bounce) (autocorr -0.86)

carrier_disp_tight_fntable's per-pass series has lag-1 autocorrelation -0.86, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable} vs {carrier_disp_tight_ifchainlin} (25% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable} and a slow tier {carrier_disp_tight_ifchainlin} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_tight_ifchain's edge over baseline is significant but tiny (-3 ns, 0.14%)

carrier_disp_tight_ifchain differs from baseline carrier_disp_tight_switch by -3 ns (0.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 1712.1 ns median (-19.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.13x (fastest 1712.1 ns, slowest 3641.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 4713ns | 4712ns | 4415ns | 4630ns | 4986ns | +4.54% |
| carrier_disp_tight_fntable | 5223ns | 5230ns | 4865ns | 5118ns | 5561ns | +15.86% |
| carrier_disp_tight_ifchain | 4504ns | 4594ns | 4152ns | 4448ns | 4765ns | -0.09% |
| carrier_disp_tight_ifchainasc | 4450ns | 4448ns | 4151ns | 4352ns | 4746ns | -1.30% |
| carrier_disp_tight_ifchainlin | 6029ns | 5966ns | 5578ns | 5853ns | 6519ns | +33.73% |
| carrier_disp_tight_nullfloor | 4077ns | 4032ns | 3772ns | 3950ns | 4419ns | -9.57% |
| carrier_disp_tight_switch | 4508ns | 4500ns | 4178ns | 4423ns | 4802ns | base |
| carrier_disp_tight_threaded | 5121ns | 5111ns | 4757ns | 5006ns | 5475ns | +13.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 2389ns | 2251ns | 2528ns | +11.17% | 0.027 |
| carrier_disp_tight_fntable | 2900ns | 2713ns | 3078ns | +34.96% | 0.022 |
| carrier_disp_tight_ifchain | 2139ns | 2002ns | 2276ns | -0.46% | 0.030 |
| carrier_disp_tight_ifchainasc | 2136ns | 2001ns | 2275ns | -0.61% | 0.030 |
| carrier_disp_tight_ifchainlin | 3679ns | 3421ns | 3974ns | +71.20% | 0.017 |
| carrier_disp_tight_nullfloor | 1721ns | 1600ns | 1846ns | -19.92% | 0.037 |
| carrier_disp_tight_switch | 2149ns | 2003ns | 2304ns | base | 0.030 |
| carrier_disp_tight_threaded | 2773ns | 2598ns | 2939ns | +29.02% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 270485 | 1463487 | 0.185 | 1.02× |
| carrier_disp_tight_fntable | 263420 | 1507938 | 0.175 | 0.99× |
| carrier_disp_tight_ifchain | 268546 | 1568070 | 0.171 | 1.01× |
| carrier_disp_tight_ifchainasc | 268040 | 1567679 | 0.171 | 1.01× |
| carrier_disp_tight_ifchainlin | 270759 | 1468580 | 0.184 | 1.02× |
| carrier_disp_tight_nullfloor | 266081 | 1658191 | 0.160 | 1.00× |
| carrier_disp_tight_switch | 265486 | 1513790 | 0.175 | 1.00× |
| carrier_disp_tight_threaded | 275809 | 1683438 | 0.164 | 1.04× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.027 | 67.0% |
| carrier_disp_tight_fntable | 0.022 | 55.1% |
| carrier_disp_tight_ifchain | 0.030 | 74.8% |
| carrier_disp_tight_ifchainasc | 0.030 | 75.1% |
| carrier_disp_tight_ifchainlin | 0.018 | 44.0% |
| carrier_disp_tight_nullfloor | 0.037 | 93.5% |
| carrier_disp_tight_switch | 0.030 | 74.9% |
| carrier_disp_tight_threaded | 0.023 | 57.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 4713ns | 4713ns | +4.54% |
| carrier_disp_tight_fntable | 5223ns | 5223ns | +15.86% |
| carrier_disp_tight_ifchain | 4504ns | 4504ns | -0.09% |
| carrier_disp_tight_ifchainasc | 4450ns | 4450ns | -1.30% |
| carrier_disp_tight_ifchainlin | 6029ns | 6029ns | +33.73% |
| carrier_disp_tight_nullfloor | 4077ns | 4077ns | -9.57% |
| carrier_disp_tight_switch | 4508ns | 4508ns | base |
| carrier_disp_tight_threaded | 5121ns | 5121ns | +13.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 2137ns | base | --- | [2006, 2304] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 2388ns | +244.8ns (+11.5%) | [+201, +275]ns | [2251, 2528] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 2905ns | +746.0ns (+34.9%) | [+690, +818]ns | [2718, 3078] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 2138ns | no significant difference | [-52, +25]ns | [2003, 2276] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_tight_ifchainasc | 2131ns | no significant difference | [-49, +22]ns | [2002, 2275] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_tight_ifchainlin | 3641ns | +1472.9ns (+68.9%) | [+1395, +1722]ns | [3423, 3974] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 1712ns | -420.2ns (-19.7%) | [-482, -382]ns | [1604, 1846] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 2777ns | +610.6ns (+28.6%) | [+578, +683]ns | [2602, 2939] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2003ns | +12.4% | +37.1% | +0.0% | +0.1% | +71.0% | -20.1% | +29.7% |
| 2 | 2275ns | +11.3% | +35.1% | -0.0% | -0.8% | +72.5% | -20.4% | +29.2% |
| 3 | 2046ns | +10.2% | +32.6% | -2.1% | -2.1% | +67.2% | -21.4% | +27.4% |
| 4 | 2332ns | +8.2% | +32.2% | -2.6% | -2.3% | +65.3% | -21.4% | +25.9% |
| 5 | 2010ns | +12.0% | +35.4% | -0.3% | -0.4% | +70.5% | -19.7% | +30.7% |
| 6 | 2228ns | +13.1% | +37.6% | +2.2% | +2.0% | +80.5% | -16.5% | +31.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.838 | HIGH- (thermal bounce) |
| carrier_disp_tight_fntable | -0.856 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchain | -0.830 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainasc | -0.831 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainlin | -0.751 | HIGH- (thermal bounce) |
| carrier_disp_tight_nullfloor | -0.771 | HIGH- (thermal bounce) |
| carrier_disp_tight_switch | -0.810 | HIGH- (thermal bounce) |
| carrier_disp_tight_threaded | -0.826 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 0/6, lost 6/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 3/6, lost 1/6
- **carrier_disp_tight_ifchainasc**: won 4/6, lost 1/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 86749.7ns | 2389.0ns | 3631.2% | HIGH |
| carrier_disp_tight_fntable | 84180.8ns | 2900.3ns | 2902.5% | HIGH |
| carrier_disp_tight_ifchain | 86386.7ns | 2139.0ns | 4038.6% | HIGH |
| carrier_disp_tight_ifchainasc | 86220.1ns | 2135.9ns | 4036.7% | HIGH |
| carrier_disp_tight_ifchainlin | 86184.9ns | 3679.1ns | 2342.6% | HIGH |
| carrier_disp_tight_nullfloor | 86303.6ns | 1720.9ns | 5015.1% | HIGH |
| carrier_disp_tight_switch | 86034.4ns | 2149.0ns | 4003.4% | HIGH |
| carrier_disp_tight_threaded | 88542.6ns | 2772.8ns | 3193.3% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 2250.8-2528.3 ns)
   2250.8 |########################################
   2264.7 |
   2278.6 |
   2292.4 |
   2306.3 |
   2320.2 |
   2334.1 |
   2347.9 |
   2361.8 |
   2375.7 |
   2389.6 |
   2403.5 |
   2417.3 |
   2431.2 |
   2445.1 |
   2459.0 |
   2472.8 |
   2486.7 |
   2500.6 |
   2514.5 |##########################
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 2713.3-3078.1 ns)
   2713.3 |########################################
   2731.5 |####################
   2749.8 |
   2768.0 |
   2786.3 |
   2804.5 |
   2822.7 |
   2841.0 |
   2859.2 |
   2877.5 |
   2895.7 |
   2913.9 |
   2932.2 |
   2950.4 |
   2968.7 |
   2986.9 |
   3005.1 |
   3023.4 |
   3041.6 |
   3059.9 |########################################
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 2002.5-2276.1 ns)
   2002.5 |########################################
   2016.2 |
   2029.9 |
   2043.5 |
   2057.2 |
   2070.9 |
   2084.6 |
   2098.2 |
   2111.9 |
   2125.6 |
   2139.3 |
   2153.0 |
   2166.6 |
   2180.3 |
   2194.0 |
   2207.7 |
   2221.3 |
   2235.0 |
   2248.7 |
   2262.4 |##########################
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 2001.2-2274.6 ns)
   2001.2 |########################################
   2014.9 |
   2028.5 |
   2042.2 |
   2055.9 |
   2069.5 |
   2083.2 |
   2096.9 |
   2110.5 |
   2124.2 |
   2137.9 |
   2151.5 |
   2165.2 |
   2178.9 |
   2192.5 |
   2206.2 |
   2219.9 |
   2233.5 |
   2247.2 |#############
   2260.9 |#############
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 3420.8-3973.5 ns)
   3420.8 |########################################
   3448.4 |
   3476.1 |
   3503.7 |
   3531.3 |
   3559.0 |
   3586.6 |
   3614.2 |
   3641.9 |
   3669.5 |
   3697.2 |
   3724.8 |
   3752.4 |
   3780.1 |
   3807.7 |
   3835.3 |#############
   3863.0 |
   3890.6 |
   3918.2 |#############
   3945.9 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 1600.4-1846.2 ns)
   1600.4 |########################################
   1612.7 |####################
   1625.0 |
   1637.3 |
   1649.6 |
   1661.9 |
   1674.2 |
   1686.4 |
   1698.7 |
   1711.0 |
   1723.3 |
   1735.6 |
   1747.9 |
   1760.2 |
   1772.5 |
   1784.8 |
   1797.1 |
   1809.4 |####################
   1821.7 |####################
   1834.0 |
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 2002.9-2303.9 ns)
   2002.9 |########################################
   2018.0 |
   2033.0 |####################
   2048.1 |
   2063.1 |
   2078.2 |
   2093.2 |
   2108.3 |
   2123.3 |
   2138.4 |
   2153.4 |
   2168.5 |
   2183.5 |
   2198.6 |
   2213.6 |####################
   2228.7 |
   2243.7 |
   2258.8 |
   2273.8 |####################
   2288.9 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 2597.5-2938.9 ns)
   2597.5 |########################################
   2614.6 |####################
   2631.6 |
   2648.7 |
   2665.8 |
   2682.9 |
   2699.9 |
   2717.0 |
   2734.1 |
   2751.2 |
   2768.2 |
   2785.3 |
   2802.4 |
   2819.4 |
   2836.5 |
   2853.6 |
   2870.7 |
   2887.7 |
   2904.8 |
   2921.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=3627.6% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=2902.7% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=4042.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=4044.9% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=2368.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=5039.6% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=4026.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=3187.0% of algo (FFI overhead may distort results)

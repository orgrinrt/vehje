# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 229% faster than the next best (carrier_disp_wideselect_bittree)

carrier_disp_wideselect_nullfloor (509.73 us) leads carrier_disp_wideselect_bittree (1.68 ms) by 229%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 76% (significant)

carrier_disp_wideselect_nullfloor is -1.63 ms (76%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 5.4x slower than the field

carrier_disp_wideselect_ifchainlin (2.74 ms) is 5.4x the fastest (509.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_nullfloor is fastest but the noisiest (CV 5.8%)

carrier_disp_wideselect_nullfloor wins on median (509.73 us) yet has the highest variance (CV 5.8%), while carrier_disp_wideselect_ifchainasc is the steadiest (CV 0.4%, 2.15 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_disp_wideselect_fntable shows alternating (throttle bounce) (autocorr -0.73)

carrier_disp_wideselect_fntable's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor} vs {carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_ifchainlin} (229% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor} and a slow tier {carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_ifchainlin} with a 229% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.4x the fastest

Fastest carrier_disp_wideselect_nullfloor (509.73 us) to slowest carrier_disp_wideselect_ifchainlin (2.74 ms): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 509725.0 ns median (-76.2% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.38x (fastest 509725.0 ns, slowest 2742465.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1691056ns | 1680933ns | 1637423ns | 1675301ns | 1741504ns | -21.08% |
| carrier_disp_wideselect_fntable | 2035999ns | 2034305ns | 2014605ns | 2032512ns | 2051924ns | -4.98% |
| carrier_disp_wideselect_ifchain | 2192602ns | 2192576ns | 2170216ns | 2190854ns | 2206417ns | +2.33% |
| carrier_disp_wideselect_ifchainasc | 2151913ns | 2153339ns | 2139833ns | 2150498ns | 2160075ns | +0.43% |
| carrier_disp_wideselect_ifchainlin | 2743011ns | 2745619ns | 2717892ns | 2744928ns | 2752695ns | +28.02% |
| carrier_disp_wideselect_nullfloor | 516020ns | 512861ns | 482548ns | 504135ns | 550583ns | -75.92% |
| carrier_disp_wideselect_switch | 2142659ns | 2145036ns | 2112342ns | 2143397ns | 2156710ns | base |
| carrier_disp_wideselect_threaded | 2367264ns | 2365455ns | 2313793ns | 2356119ns | 2410717ns | +10.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1688073ns | 1634834ns | 1738289ns | -21.10% | 0.010 |
| carrier_disp_wideselect_fntable | 2032821ns | 2011721ns | 2048797ns | -4.98% | 0.008 |
| carrier_disp_wideselect_ifchain | 2188978ns | 2167408ns | 2202657ns | +2.32% | 0.007 |
| carrier_disp_wideselect_ifchainasc | 2148256ns | 2136313ns | 2156513ns | +0.41% | 0.008 |
| carrier_disp_wideselect_ifchainlin | 2739829ns | 2714554ns | 2749585ns | +28.06% | 0.006 |
| carrier_disp_wideselect_nullfloor | 512959ns | 479203ns | 547503ns | -76.02% | 0.032 |
| carrier_disp_wideselect_switch | 2139431ns | 2109238ns | 2153306ns | base | 0.008 |
| carrier_disp_wideselect_threaded | 2363780ns | 2309758ns | 2407679ns | +10.49% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 10511142 | 20216030 | 0.520 | 0.79× |
| carrier_disp_wideselect_fntable | 12725125 | 27189282 | 0.468 | 0.95× |
| carrier_disp_wideselect_ifchain | 13722846 | 19776212 | 0.694 | 1.03× |
| carrier_disp_wideselect_ifchainasc | 13462210 | 19777196 | 0.681 | 1.01× |
| carrier_disp_wideselect_ifchainlin | 17055660 | 52874338 | 0.323 | 1.28× |
| carrier_disp_wideselect_nullfloor | 3211151 | 16700256 | 0.192 | 0.24× |
| carrier_disp_wideselect_switch | 13363780 | 19251122 | 0.694 | 1.00× |
| carrier_disp_wideselect_threaded | 14696602 | 26367298 | 0.557 | 1.10× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.010 | 28.6% |
| carrier_disp_wideselect_fntable | 0.008 | 23.6% |
| carrier_disp_wideselect_ifchain | 0.007 | 21.9% |
| carrier_disp_wideselect_ifchainasc | 0.008 | 22.3% |
| carrier_disp_wideselect_ifchainlin | 0.006 | 17.5% |
| carrier_disp_wideselect_nullfloor | 0.032 | 94.0% |
| carrier_disp_wideselect_switch | 0.008 | 22.4% |
| carrier_disp_wideselect_threaded | 0.007 | 20.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 1691056ns | 1691056ns | -21.08% |
| carrier_disp_wideselect_fntable | 2035999ns | 2035999ns | -4.98% |
| carrier_disp_wideselect_ifchain | 2192602ns | 2192602ns | +2.33% |
| carrier_disp_wideselect_ifchainasc | 2151913ns | 2151913ns | +0.43% |
| carrier_disp_wideselect_ifchainlin | 2743011ns | 2743011ns | +28.02% |
| carrier_disp_wideselect_nullfloor | 516020ns | 516020ns | -75.92% |
| carrier_disp_wideselect_switch | 2142659ns | 2142659ns | base |
| carrier_disp_wideselect_threaded | 2367264ns | 2367264ns | +10.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 2141734ns | base | --- | [2123253, 2153306] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 1677922ns | -445330.8ns (-20.8%) | [-493724, -415018]ns | [1648009, 1738289] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 2031250ns | -104103.3ns (-4.9%) | [-131477, -84250]ns | [2018415, 2048797] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 2188856ns | +49350.4ns (+2.3%) | [+42601, +56690]ns | [2175421, 2202657] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 2149329ns | no significant difference | [-5386, +27127]ns | [2138926, 2156513] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainlin | 2742465ns | +599406.4ns (+28.0%) | [+576505, +625283]ns | [2727437, 2749585] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 509725ns | -1626978.1ns (-76.0%) | [-1648837, -1603599]ns | [481650, 547503] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 2361729ns | +219995.0ns (+10.3%) | [+198679, +254373]ns | [2321931, 2407679] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2137268ns | -21.2% | -4.8% | +2.6% | +0.8% | +28.3% | -75.8% | +9.2% |
| 2 | 2155420ns | -18.2% | -6.0% | +2.2% | +0.2% | +25.9% | -73.7% | +10.5% |
| 3 | 2109238ns | -20.7% | -4.0% | +2.8% | +1.8% | +30.6% | -77.3% | +9.5% |
| 4 | 2137431ns | -22.3% | -3.9% | +2.2% | -0.1% | +28.3% | -77.4% | +10.4% |
| 5 | 2146037ns | -23.8% | -6.3% | +1.8% | +0.3% | +27.7% | -76.6% | +10.2% |
| 6 | 2151192ns | -20.3% | -5.0% | +2.4% | -0.4% | +27.6% | -75.5% | +13.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.102 | ok |
| carrier_disp_wideselect_fntable | -0.731 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_ifchain | -0.178 | ok |
| carrier_disp_wideselect_ifchainasc | -0.018 | ok |
| carrier_disp_wideselect_ifchainlin | -0.406 | moderate- |
| carrier_disp_wideselect_nullfloor | -0.084 | ok |
| carrier_disp_wideselect_switch | -0.289 | moderate- |
| carrier_disp_wideselect_threaded | -0.147 | ok |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 6/6, lost 0/6
- **carrier_disp_wideselect_fntable**: won 6/6, lost 0/6
- **carrier_disp_wideselect_ifchain**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchainasc**: won 1/6, lost 4/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1690122.2ns | 1688073.3ns | 100.1% | HIGH |
| carrier_disp_wideselect_fntable | 2054483.1ns | 2032820.8ns | 101.1% | HIGH |
| carrier_disp_wideselect_ifchain | 2190459.7ns | 2188978.1ns | 100.1% | HIGH |
| carrier_disp_wideselect_ifchainasc | 2152634.6ns | 2148255.6ns | 100.2% | HIGH |
| carrier_disp_wideselect_ifchainlin | 2735686.7ns | 2739829.2ns | 99.8% | HIGH |
| carrier_disp_wideselect_nullfloor | 514753.6ns | 512959.4ns | 100.3% | HIGH |
| carrier_disp_wideselect_switch | 2141550.8ns | 2139431.0ns | 100.1% | HIGH |
| carrier_disp_wideselect_threaded | 2364856.7ns | 2363779.7ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 1634833.8-1738288.5 ns)
  1634833.8 |########################################
  1640006.5 |
  1645179.3 |
  1650352.0 |
  1655524.8 |
  1660697.5 |########################################
  1665870.2 |
  1671043.0 |########################################
  1676215.7 |
  1681388.4 |########################################
  1686561.2 |
  1691733.9 |
  1696906.6 |
  1702079.4 |
  1707252.1 |
  1712424.9 |########################################
  1717597.6 |
  1722770.3 |
  1727943.1 |
  1733115.8 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 2011721.2-2048797.1 ns)
  2011721.2 |########################################
  2013575.0 |
  2015428.8 |
  2017282.6 |
  2019136.4 |
  2020990.2 |
  2022844.0 |
  2024697.8 |########################################
  2026551.6 |########################################
  2028405.4 |
  2030259.1 |
  2032112.9 |
  2033966.7 |########################################
  2035820.5 |
  2037674.3 |
  2039528.1 |
  2041381.9 |
  2043235.7 |########################################
  2045089.5 |
  2046943.3 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 2167407.5-2202656.9 ns)
  2167407.5 |########################################
  2169170.0 |
  2170932.4 |
  2172694.9 |
  2174457.4 |
  2176219.9 |
  2177982.3 |
  2179744.8 |
  2181507.3 |
  2183269.7 |########################################
  2185032.2 |########################################
  2186794.7 |
  2188557.1 |
  2190319.6 |
  2192082.1 |########################################
  2193844.5 |
  2195607.0 |
  2197369.5 |
  2199132.0 |
  2200894.4 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 2136312.9-2156512.7 ns)
  2136312.9 |########################################
  2137322.9 |
  2138332.9 |
  2139342.9 |
  2140352.9 |
  2141362.9 |########################################
  2142372.8 |
  2143382.8 |
  2144392.8 |
  2145402.8 |
  2146412.8 |########################################
  2147422.8 |
  2148432.8 |
  2149442.8 |
  2150452.8 |
  2151462.8 |########################################
  2152472.7 |
  2153482.7 |########################################
  2154492.7 |
  2155502.7 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 2714553.8-2749585.4 ns)
  2714553.8 |########################################
  2716305.4 |
  2718057.0 |
  2719808.5 |
  2721560.1 |
  2723311.7 |
  2725063.3 |
  2726814.9 |
  2728566.4 |
  2730318.0 |
  2732069.6 |
  2733821.2 |
  2735572.8 |
  2737324.3 |
  2739075.9 |########################################
  2740827.5 |########################################
  2742579.1 |########################################
  2744330.7 |########################################
  2746082.2 |
  2747833.8 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 479203.3-547503.1 ns)
  479203.3 |########################################
  482618.3 |########################################
  486033.3 |
  489448.3 |
  492863.3 |
  496278.2 |
  499693.2 |########################################
  503108.2 |
  506523.2 |
  509938.2 |
  513353.2 |
  516768.2 |########################################
  520183.2 |
  523598.2 |
  527013.2 |########################################
  530428.2 |
  533843.1 |
  537258.1 |
  540673.1 |
  544088.1 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 2109237.5-2153306.5 ns)
  2109237.5 |####################
  2111440.9 |
  2113644.4 |
  2115847.8 |
  2118051.3 |
  2120254.7 |
  2122458.2 |
  2124661.6 |
  2126865.1 |
  2129068.5 |
  2131272.0 |
  2133475.4 |
  2135678.9 |########################################
  2137882.3 |
  2140085.8 |
  2142289.2 |
  2144492.7 |####################
  2146696.1 |
  2148899.6 |
  2151103.0 |####################
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 2309757.5-2407679.1 ns)
  2309757.5 |########################################
  2314653.6 |
  2319549.7 |
  2324445.7 |
  2329341.8 |########################################
  2334237.9 |
  2339134.0 |
  2344030.1 |
  2348926.2 |
  2353822.2 |
  2358718.3 |########################################
  2363614.4 |########################################
  2368510.5 |
  2373406.6 |
  2378302.7 |########################################
  2383198.7 |
  2388094.8 |
  2392990.9 |
  2397887.0 |
  2402783.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=101.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=100.0% of algo (FFI overhead may distort results)

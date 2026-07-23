# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 31% faster than the next best (carrier_disp_wideselect_ifchainasc)

carrier_disp_wideselect_nullfloor (1.79 us) leads carrier_disp_wideselect_ifchainasc (2.34 us) by 31%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 24% (significant)

carrier_disp_wideselect_nullfloor is -566 ns (24%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 2.9x slower than the field

carrier_disp_wideselect_ifchainlin (5.24 us) is 2.9x the fastest (1.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_bittree shows alternating (throttle bounce) (autocorr -0.80)

carrier_disp_wideselect_bittree's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} vs {carrier_disp_wideselect_ifchainlin} (79% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} and a slow tier {carrier_disp_wideselect_ifchainlin} with a 79% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_wideselect_ifchainasc's edge over baseline is significant but tiny (-28 ns, 1.18%)

carrier_disp_wideselect_ifchainasc differs from baseline carrier_disp_wideselect_switch by -28 ns (1.18%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 1791.8 ns median (-24.7% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.93x (fastest 1791.8 ns, slowest 5242.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 5315ns | 5279ns | 5153ns | 5247ns | 5497ns | +11.87% |
| carrier_disp_wideselect_fntable | 5277ns | 5281ns | 4917ns | 5166ns | 5624ns | +11.08% |
| carrier_disp_wideselect_ifchain | 4809ns | 4994ns | 4362ns | 4785ns | 5069ns | +1.23% |
| carrier_disp_wideselect_ifchainasc | 4664ns | 4764ns | 4306ns | 4612ns | 4920ns | -1.83% |
| carrier_disp_wideselect_ifchainlin | 7632ns | 7643ns | 6944ns | 7606ns | 8015ns | +60.65% |
| carrier_disp_wideselect_nullfloor | 4278ns | 4326ns | 4012ns | 4318ns | 4352ns | -9.94% |
| carrier_disp_wideselect_switch | 4751ns | 4890ns | 4340ns | 4737ns | 4976ns | base |
| carrier_disp_wideselect_threaded | 5158ns | 5127ns | 4742ns | 5039ns | 5544ns | +8.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 2786ns | 2752ns | 2820ns | +20.24% | 0.023 |
| carrier_disp_wideselect_fntable | 2936ns | 2722ns | 3148ns | +26.70% | 0.022 |
| carrier_disp_wideselect_ifchain | 2418ns | 2212ns | 2528ns | +4.36% | 0.026 |
| carrier_disp_wideselect_ifchainasc | 2282ns | 2089ns | 2396ns | -1.53% | 0.028 |
| carrier_disp_wideselect_ifchainlin | 5207ns | 4758ns | 5432ns | +124.73% | 0.012 |
| carrier_disp_wideselect_nullfloor | 1778ns | 1662ns | 1828ns | -23.26% | 0.036 |
| carrier_disp_wideselect_switch | 2317ns | 2147ns | 2401ns | base | 0.028 |
| carrier_disp_wideselect_threaded | 2766ns | 2572ns | 2944ns | +19.38% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 254969 | 1253353 | 0.203 | 0.99× |
| carrier_disp_wideselect_fntable | 272714 | 1552170 | 0.176 | 1.06× |
| carrier_disp_wideselect_ifchain | 261942 | 1395244 | 0.188 | 1.01× |
| carrier_disp_wideselect_ifchainasc | 263604 | 1474170 | 0.179 | 1.02× |
| carrier_disp_wideselect_ifchainlin | 269837 | 1660841 | 0.162 | 1.04× |
| carrier_disp_wideselect_nullfloor | 257971 | 1603142 | 0.161 | 1.00× |
| carrier_disp_wideselect_switch | 258374 | 1407267 | 0.184 | 1.00× |
| carrier_disp_wideselect_threaded | 272257 | 1678255 | 0.162 | 1.05× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.023 | 59.7% |
| carrier_disp_wideselect_fntable | 0.022 | 56.9% |
| carrier_disp_wideselect_ifchain | 0.025 | 66.2% |
| carrier_disp_wideselect_ifchainasc | 0.027 | 71.0% |
| carrier_disp_wideselect_ifchainlin | 0.012 | 31.7% |
| carrier_disp_wideselect_nullfloor | 0.036 | 92.8% |
| carrier_disp_wideselect_switch | 0.027 | 69.9% |
| carrier_disp_wideselect_threaded | 0.023 | 59.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 5315ns | 5315ns | +11.87% |
| carrier_disp_wideselect_fntable | 5277ns | 5277ns | +11.08% |
| carrier_disp_wideselect_ifchain | 4809ns | 4809ns | +1.23% |
| carrier_disp_wideselect_ifchainasc | 4664ns | 4664ns | -1.83% |
| carrier_disp_wideselect_ifchainlin | 7632ns | 7632ns | +60.65% |
| carrier_disp_wideselect_nullfloor | 4278ns | 4278ns | -9.94% |
| carrier_disp_wideselect_switch | 4751ns | 4751ns | base |
| carrier_disp_wideselect_threaded | 5158ns | 5158ns | +8.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 2379ns | base | --- | [2171, 2401] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 2783ns | +418.6ns (+17.6%) | [+404, +584]ns | [2755, 2820] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 2922ns | +551.7ns (+23.2%) | [+342, +963]ns | [2737, 3148] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 2511ns | no significant difference | [-63, +254]ns | [2216, 2528] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_wideselect_ifchainasc | 2341ns | no significant difference | [-178, +100]ns | [2108, 2396] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainlin | 5242ns | +2906.8ns (+122.2%) | [+2727, +3036]ns | [4947, 5432] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 1792ns | -565.8ns (-23.8%) | [-621, -430]ns | [1715, 1828] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 2776ns | +452.8ns (+19.0%) | [+232, +662]ns | [2578, 2944] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 2361ns | +17.0% | +16.6% | -6.0% | -11.5% | +120.4% | -24.1% | +9.4% |
| 2 | 2407ns | +17.3% | +29.6% | +4.6% | -3.5% | +119.4% | -26.5% | +23.1% |
| 3 | 2147ns | +28.5% | +48.0% | +18.2% | -0.9% | +121.6% | -15.2% | +35.8% |
| 4 | 2396ns | +17.0% | +13.6% | +4.7% | +0.8% | +127.6% | -25.2% | +22.1% |
| 5 | 2195ns | +25.4% | +40.8% | +0.8% | +8.2% | +133.9% | -24.3% | +17.2% |
| 6 | 2396ns | +17.5% | +14.9% | +4.9% | -1.5% | +125.8% | -23.4% | +10.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.796 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_fntable | -0.449 | moderate- |
| carrier_disp_wideselect_ifchain | -0.287 | moderate- |
| carrier_disp_wideselect_ifchainasc | -0.158 | ok |
| carrier_disp_wideselect_ifchainlin | -0.560 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_nullfloor | -0.431 | moderate- |
| carrier_disp_wideselect_switch | -0.664 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_threaded | 0.065 | ok |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 0/6, lost 6/6
- **carrier_disp_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchain**: won 1/6, lost 5/6
- **carrier_disp_wideselect_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 86458.9ns | 2786.0ns | 3103.4% | HIGH |
| carrier_disp_wideselect_fntable | 86776.1ns | 2935.7ns | 2955.9% | HIGH |
| carrier_disp_wideselect_ifchain | 85920.9ns | 2418.2ns | 3553.1% | HIGH |
| carrier_disp_wideselect_ifchainasc | 85843.5ns | 2281.5ns | 3762.5% | HIGH |
| carrier_disp_wideselect_ifchainlin | 87279.5ns | 5207.1ns | 1676.2% | HIGH |
| carrier_disp_wideselect_nullfloor | 86796.4ns | 1778.2ns | 4881.2% | HIGH |
| carrier_disp_wideselect_switch | 85550.4ns | 2317.1ns | 3692.2% | HIGH |
| carrier_disp_wideselect_threaded | 87763.6ns | 2766.0ns | 3172.9% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 2752.1-2820.0 ns)
   2752.1 |########################################
   2755.5 |########################################
   2758.9 |########################################
   2762.3 |
   2765.7 |
   2769.1 |
   2772.5 |
   2775.9 |
   2779.3 |
   2782.7 |
   2786.1 |
   2789.4 |
   2792.8 |
   2796.2 |
   2799.6 |
   2803.0 |########################################
   2806.4 |
   2809.8 |
   2813.2 |########################################
   2816.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 2721.7-3147.5 ns)
   2721.7 |####################
   2743.0 |########################################
   2764.3 |
   2785.6 |
   2806.9 |
   2828.1 |
   2849.4 |
   2870.7 |
   2892.0 |
   2913.3 |
   2934.6 |
   2955.9 |
   2977.2 |
   2998.5 |
   3019.8 |
   3041.1 |
   3062.3 |
   3083.6 |####################
   3104.9 |####################
   3126.2 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 2212.5-2527.7 ns)
   2212.5 |########################################
   2228.3 |
   2244.0 |
   2259.8 |
   2275.5 |
   2291.3 |
   2307.1 |
   2322.8 |
   2338.6 |
   2354.3 |
   2370.1 |
   2385.9 |
   2401.6 |
   2417.4 |
   2433.1 |
   2448.9 |
   2464.7 |
   2480.4 |
   2496.2 |####################
   2511.9 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 2088.8-2395.6 ns)
   2088.8 |########################################
   2104.1 |
   2119.5 |########################################
   2134.8 |
   2150.2 |
   2165.5 |
   2180.8 |
   2196.2 |
   2211.5 |
   2226.9 |
   2242.2 |
   2257.5 |
   2272.9 |
   2288.2 |
   2303.6 |
   2318.9 |########################################
   2334.2 |
   2349.6 |########################################
   2364.9 |########################################
   2380.3 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 4757.9-5432.2 ns)
   4757.9 |########################################
   4791.6 |
   4825.3 |
   4859.1 |
   4892.8 |
   4926.5 |
   4960.2 |
   4993.9 |
   5027.6 |
   5061.4 |
   5095.1 |
   5128.8 |########################################
   5162.5 |
   5196.2 |########################################
   5229.9 |
   5263.7 |########################################
   5297.4 |
   5331.1 |
   5364.8 |
   5398.5 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 1662.1-1827.5 ns)
   1662.1 |####################
   1670.4 |
   1678.6 |
   1686.9 |
   1695.2 |
   1703.4 |
   1711.7 |
   1720.0 |
   1728.3 |
   1736.5 |
   1744.8 |
   1753.1 |
   1761.3 |####################
   1769.6 |
   1777.9 |
   1786.2 |########################################
   1794.4 |
   1802.7 |
   1811.0 |
   1819.2 |####################
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 2146.7-2401.4 ns)
   2146.7 |####################
   2159.4 |
   2172.2 |
   2184.9 |####################
   2197.6 |
   2210.4 |
   2223.1 |
   2235.9 |
   2248.6 |
   2261.3 |
   2274.1 |
   2286.8 |
   2299.5 |
   2312.3 |
   2325.0 |
   2337.8 |
   2350.5 |####################
   2363.2 |
   2376.0 |
   2388.7 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 2572.5-2943.8 ns)
   2572.5 |########################################
   2591.1 |
   2609.6 |
   2628.2 |####################
   2646.8 |
   2665.3 |
   2683.9 |
   2702.4 |
   2721.0 |
   2739.6 |
   2758.1 |
   2776.7 |
   2795.2 |
   2813.8 |
   2832.4 |
   2850.9 |
   2869.5 |
   2888.1 |
   2906.6 |########################################
   2925.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=3107.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=2970.8% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=3418.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=3665.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=1672.8% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=4844.9% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=3598.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=3162.4% of algo (FFI overhead may distort results)

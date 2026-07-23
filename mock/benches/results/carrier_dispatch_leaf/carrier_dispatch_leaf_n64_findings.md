# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 26% faster than the next best (carrier_disp_leaf_switch)

carrier_disp_leaf_nullfloor (1.54 us) leads carrier_disp_leaf_switch (1.94 us) by 26%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 22% (significant)

carrier_disp_leaf_nullfloor is -431 ns (22%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_ifchain shows alternating (throttle bounce) (autocorr -0.84)

carrier_disp_leaf_ifchain's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_bittree, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded, carrier_disp_leaf_ifchainlin} (26% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_bittree, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded, carrier_disp_leaf_ifchainlin} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_leaf_ifchainasc's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_disp_leaf_ifchainasc are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### carrier_disp_leaf_ifchain's edge over baseline is significant but tiny (11 ns, 0.59%)

carrier_disp_leaf_ifchain differs from baseline carrier_disp_leaf_switch by 11 ns (0.59%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 1538.3 ns median (-20.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.96x (fastest 1538.3 ns, slowest 3021.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 4570ns | 4514ns | 4288ns | 4481ns | 4845ns | +5.71% |
| carrier_disp_leaf_fntable | 4682ns | 4545ns | 4483ns | 4534ns | 5002ns | +8.29% |
| carrier_disp_leaf_ifchain | 4366ns | 4344ns | 4051ns | 4253ns | 4692ns | +0.98% |
| carrier_disp_leaf_ifchainasc | 4329ns | 4335ns | 4025ns | 4239ns | 4617ns | +0.14% |
| carrier_disp_leaf_ifchainlin | 5336ns | 5569ns | 4815ns | 5320ns | 5621ns | +23.43% |
| carrier_disp_leaf_nullfloor | 3893ns | 3922ns | 3622ns | 3847ns | 4098ns | -9.95% |
| carrier_disp_leaf_switch | 4323ns | 4242ns | 4023ns | 4179ns | 4689ns | base |
| carrier_disp_leaf_threaded | 5042ns | 5042ns | 4805ns | 4998ns | 5227ns | +16.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 2221ns | 2086ns | 2352ns | +11.87% | 0.029 |
| carrier_disp_leaf_fntable | 2408ns | 2321ns | 2559ns | +21.29% | 0.027 |
| carrier_disp_leaf_ifchain | 2003ns | 1887ns | 2125ns | +0.88% | 0.032 |
| carrier_disp_leaf_ifchainasc | 1991ns | 1876ns | 2106ns | +0.28% | 0.032 |
| carrier_disp_leaf_ifchainlin | 2919ns | 2642ns | 3094ns | +47.04% | 0.022 |
| carrier_disp_leaf_nullfloor | 1544ns | 1460ns | 1632ns | -22.24% | 0.041 |
| carrier_disp_leaf_switch | 1985ns | 1877ns | 2134ns | base | 0.032 |
| carrier_disp_leaf_threaded | 2644ns | 2527ns | 2754ns | +33.19% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 270996 | 1396394 | 0.194 | 1.00× |
| carrier_disp_leaf_fntable | 268214 | 1732807 | 0.155 | 0.99× |
| carrier_disp_leaf_ifchain | 271012 | 1586325 | 0.171 | 1.00× |
| carrier_disp_leaf_ifchainasc | 269045 | 1579855 | 0.170 | 0.99× |
| carrier_disp_leaf_ifchainlin | 267673 | 1466897 | 0.182 | 0.99× |
| carrier_disp_leaf_nullfloor | 268742 | 1641360 | 0.164 | 0.99× |
| carrier_disp_leaf_switch | 270906 | 1544728 | 0.175 | 1.00× |
| carrier_disp_leaf_threaded | 274859 | 1686544 | 0.163 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.044 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.029 | 66.3% |
| carrier_disp_leaf_fntable | 0.027 | 62.5% |
| carrier_disp_leaf_ifchain | 0.032 | 73.3% |
| carrier_disp_leaf_ifchainasc | 0.032 | 73.4% |
| carrier_disp_leaf_ifchainlin | 0.021 | 48.3% |
| carrier_disp_leaf_nullfloor | 0.042 | 94.9% |
| carrier_disp_leaf_switch | 0.033 | 75.1% |
| carrier_disp_leaf_threaded | 0.024 | 55.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 4570ns | 4570ns | +5.71% |
| carrier_disp_leaf_fntable | 4682ns | 4682ns | +8.29% |
| carrier_disp_leaf_ifchain | 4366ns | 4366ns | +0.98% |
| carrier_disp_leaf_ifchainasc | 4329ns | 4329ns | +0.14% |
| carrier_disp_leaf_ifchainlin | 5336ns | 5336ns | +23.43% |
| carrier_disp_leaf_nullfloor | 3893ns | 3893ns | -9.95% |
| carrier_disp_leaf_switch | 4323ns | 4323ns | base |
| carrier_disp_leaf_threaded | 5042ns | 5042ns | +16.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 1944ns | base | --- | [1878, 2134] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 2202ns | +228.1ns (+11.7%) | [+151, +328]ns | [2108, 2352] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 2337ns | +450.2ns (+23.2%) | [+360, +458]ns | [2328, 2559] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 1992ns | no significant difference | [-107, +148]ns | [1891, 2125] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainasc | 1990ns | no significant difference | [-31, +51]ns | [1877, 2106] | no | 0.4375 | 0.3750 | **1** (17%, HIGH) |
| carrier_disp_leaf_ifchainlin | 3022ns | +887.5ns (+45.7%) | [+762, +1152]ns | [2642, 3094] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 1538ns | -431.3ns (-22.2%) | [-563, -330]ns | [1461, 1632] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 2651ns | +647.1ns (+33.3%) | [+517, +813]ns | [2527, 2754] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 1882ns | +10.8% | +24.2% | +0.2% | -0.3% | +40.4% | -22.5% | +34.3% |
| 2 | 2152ns | +8.8% | +18.8% | -1.3% | -2.2% | +38.0% | -24.1% | +19.2% |
| 3 | 1877ns | +15.9% | +24.4% | +1.0% | -0.0% | +65.4% | -13.0% | +46.3% |
| 4 | 2005ns | +17.8% | +15.7% | +6.0% | +5.1% | +53.7% | -21.9% | +37.8% |
| 5 | 2117ns | +5.3% | +21.0% | -8.8% | -0.7% | +45.2% | -28.6% | +29.3% |
| 6 | 1878ns | +13.4% | +24.3% | +9.3% | +0.0% | +40.7% | -22.2% | +34.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | -0.440 | moderate- |
| carrier_disp_leaf_fntable | -0.575 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchain | -0.844 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchainasc | -0.501 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchainlin | 0.034 | ok |
| carrier_disp_leaf_nullfloor | 0.139 | ok |
| carrier_disp_leaf_switch | -0.615 | HIGH- (thermal bounce) |
| carrier_disp_leaf_threaded | 0.200 | moderate+ |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 0/6, lost 6/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 2/6, lost 4/6
- **carrier_disp_leaf_ifchainasc**: won 3/6, lost 1/6
- **carrier_disp_leaf_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 85948.1ns | 2220.9ns | 3869.9% | HIGH |
| carrier_disp_leaf_fntable | 83480.8ns | 2408.0ns | 3466.9% | HIGH |
| carrier_disp_leaf_ifchain | 86861.0ns | 2002.8ns | 4337.0% | HIGH |
| carrier_disp_leaf_ifchainasc | 86319.3ns | 1990.8ns | 4335.8% | HIGH |
| carrier_disp_leaf_ifchainlin | 86582.2ns | 2919.2ns | 2966.0% | HIGH |
| carrier_disp_leaf_nullfloor | 86609.1ns | 1543.8ns | 5610.1% | HIGH |
| carrier_disp_leaf_switch | 86404.9ns | 1985.3ns | 4352.3% | HIGH |
| carrier_disp_leaf_threaded | 88454.1ns | 2644.2ns | 3345.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 2086.2-2352.1 ns)
   2086.2 |########################################
   2099.5 |
   2112.8 |
   2126.1 |########################################
   2139.4 |
   2152.7 |
   2166.0 |########################################
   2179.2 |
   2192.5 |
   2205.8 |
   2219.1 |########################################
   2232.4 |
   2245.7 |
   2259.0 |
   2272.3 |
   2285.6 |
   2298.9 |
   2312.2 |
   2325.5 |
   2338.8 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 2321.2-2559.3 ns)
   2321.2 |#############
   2333.1 |########################################
   2345.0 |
   2356.9 |
   2368.8 |
   2380.7 |
   2392.6 |
   2404.6 |
   2416.5 |
   2428.4 |
   2440.3 |
   2452.2 |
   2464.1 |
   2476.0 |
   2487.9 |
   2499.8 |
   2511.7 |
   2523.6 |
   2535.5 |
   2547.4 |#############
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 1887.1-2125.2 ns)
   1887.1 |########################################
   1899.0 |
   1910.9 |
   1922.8 |####################
   1934.7 |
   1946.6 |
   1958.5 |
   1970.4 |
   1982.3 |
   1994.2 |
   2006.1 |
   2018.1 |
   2030.0 |
   2041.9 |####################
   2053.8 |
   2065.7 |
   2077.6 |
   2089.5 |
   2101.4 |
   2113.3 |####################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 1876.2-2105.8 ns)
   1876.2 |########################################
   1887.7 |
   1899.2 |
   1910.6 |
   1922.1 |
   1933.6 |
   1945.1 |
   1956.6 |
   1968.1 |
   1979.5 |
   1991.0 |
   2002.5 |
   2014.0 |
   2025.5 |
   2037.0 |
   2048.4 |
   2059.9 |
   2071.4 |
   2082.9 |
   2094.4 |##########################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 2641.7-3093.8 ns)
   2641.7 |########################################
   2664.3 |
   2686.9 |
   2709.5 |
   2732.1 |
   2754.7 |
   2777.3 |
   2799.9 |
   2822.5 |
   2845.1 |
   2867.7 |
   2890.3 |
   2912.9 |
   2935.5 |
   2958.1 |####################
   2980.7 |
   3003.3 |
   3025.9 |
   3048.5 |
   3071.1 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 1459.6-1632.5 ns)
   1459.6 |########################################
   1468.2 |
   1476.9 |
   1485.5 |
   1494.2 |
   1502.8 |####################
   1511.5 |
   1520.1 |
   1528.8 |
   1537.4 |
   1546.0 |
   1554.7 |
   1563.3 |####################
   1572.0 |
   1580.6 |
   1589.3 |
   1597.9 |
   1606.6 |
   1615.2 |
   1623.9 |
  (0 below, 2 above range)

carrier_disp_leaf_switch (n=6, range 1877.1-2134.4 ns)
   1877.1 |########################################
   1890.0 |
   1902.8 |
   1915.7 |
   1928.6 |
   1941.4 |
   1954.3 |
   1967.2 |
   1980.0 |
   1992.9 |#############
   2005.7 |
   2018.6 |
   2031.5 |
   2044.3 |
   2057.2 |
   2070.1 |
   2082.9 |
   2095.8 |
   2108.7 |#############
   2121.5 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 2527.1-2754.2 ns)
   2527.1 |########################################
   2538.5 |
   2549.8 |
   2561.2 |####################
   2572.5 |
   2583.9 |
   2595.2 |
   2606.6 |
   2617.9 |
   2629.3 |
   2640.6 |
   2652.0 |
   2663.3 |
   2674.7 |
   2686.0 |
   2697.4 |
   2708.7 |
   2720.1 |
   2731.4 |####################
   2742.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=3894.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=3574.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=4359.5% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=4332.5% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=2878.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=5621.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=4446.5% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=3341.1% of algo (FFI overhead may distort results)

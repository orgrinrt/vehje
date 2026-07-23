# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader carrier_lay_wideselect_rec16 vs stability leader carrier_lay_wideselect_rec24 (+4% speed for 1.1x steadier)

carrier_lay_wideselect_rec16 is fastest (2.24 us, CV 4.3%); carrier_lay_wideselect_rec24 gives up 3.8% median for 1.1x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_lay_wideselect_rec16's edge over baseline is significant but tiny (-23 ns, 0.99%)

carrier_lay_wideselect_rec16 differs from baseline carrier_lay_wideselect_rec24 by -23 ns (0.99%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_wideselect_rec16** at 2240.7 ns median (-3.6% vs baseline)
- Spread: 1.06x (fastest 2240.7 ns, slowest 2375.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 4749ns | 4767ns | 4295ns | 4674ns | 5088ns | +1.85% |
| carrier_lay_wideselect_rec16 | 4589ns | 4606ns | 4217ns | 4550ns | 4832ns | -1.60% |
| carrier_lay_wideselect_rec20 | 4712ns | 4778ns | 4348ns | 4653ns | 4982ns | +1.04% |
| carrier_lay_wideselect_rec24 | 4663ns | 4762ns | 4355ns | 4627ns | 4871ns | base |
| carrier_lay_wideselect_rec32 | 4638ns | 4736ns | 4260ns | 4589ns | 4898ns | -0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2345ns | 2169ns | 2466ns | +3.32% | 0.027 |
| carrier_lay_wideselect_rec16 | 2236ns | 2085ns | 2341ns | -1.49% | 0.029 |
| carrier_lay_wideselect_rec20 | 2267ns | 2118ns | 2349ns | -0.12% | 0.028 |
| carrier_lay_wideselect_rec24 | 2270ns | 2125ns | 2344ns | base | 0.028 |
| carrier_lay_wideselect_rec32 | 2258ns | 2114ns | 2358ns | -0.51% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 269990 | 1501056 | 0.180 | 1.02× |
| carrier_lay_wideselect_rec16 | 267407 | 1459048 | 0.183 | 1.01× |
| carrier_lay_wideselect_rec20 | 263174 | 1436175 | 0.183 | 0.99× |
| carrier_lay_wideselect_rec24 | 264554 | 1440951 | 0.184 | 1.00× |
| carrier_lay_wideselect_rec32 | 265808 | 1450076 | 0.183 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_lay_wideselect_rec16; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.027 | 87.8% |
| carrier_lay_wideselect_rec16 | 0.029 | 93.1% |
| carrier_lay_wideselect_rec20 | 0.028 | 90.3% |
| carrier_lay_wideselect_rec24 | 0.028 | 89.7% |
| carrier_lay_wideselect_rec32 | 0.028 | 90.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 4749ns | 4749ns | +1.85% |
| carrier_lay_wideselect_rec16 | 4589ns | 4589ns | -1.60% |
| carrier_lay_wideselect_rec20 | 4712ns | 4712ns | +1.04% |
| carrier_lay_wideselect_rec24 | 4663ns | 4663ns | base |
| carrier_lay_wideselect_rec32 | 4638ns | 4638ns | -0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 2325ns | base | --- | [2141, 2344] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 2376ns | no significant difference | [-27, +139]ns | [2193, 2466] | no | 0.6875 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 2241ns | no significant difference | [-109, +30]ns | [2127, 2341] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec20 | 2311ns | no significant difference | [-30, +16]ns | [2141, 2349] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec32 | 2296ns | no significant difference | [-34, +19]ns | [2120, 2358] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 2315ns | -6.3% | -6.4% | +1.0% | -1.3% |
| 2 | 2335ns | +5.6% | +0.3% | -2.2% | +0.2% |
| 3 | 2125ns | +4.3% | +2.5% | -0.4% | -0.5% |
| 4 | 2156ns | +6.8% | -3.3% | +0.4% | -1.3% |
| 5 | 2345ns | +4.4% | -0.2% | +0.3% | -1.6% |
| 6 | 2343ns | +5.3% | -1.7% | +0.2% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | -0.255 | moderate- |
| carrier_lay_wideselect_rec16 | -0.235 | moderate- |
| carrier_lay_wideselect_rec20 | 0.235 | moderate+ |
| carrier_lay_wideselect_rec24 | 0.136 | ok |
| carrier_lay_wideselect_rec32 | 0.144 | ok |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec16**: won 4/6, lost 2/6
- **carrier_lay_wideselect_rec20**: won 2/6, lost 4/6
- **carrier_lay_wideselect_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 86319.2ns | 2345.0ns | 3681.0% | HIGH |
| carrier_lay_wideselect_rec16 | 85695.6ns | 2236.0ns | 3832.6% | HIGH |
| carrier_lay_wideselect_rec20 | 85644.6ns | 2267.0ns | 3777.8% | HIGH |
| carrier_lay_wideselect_rec24 | 85863.5ns | 2269.7ns | 3783.0% | HIGH |
| carrier_lay_wideselect_rec32 | 85898.3ns | 2258.1ns | 3804.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 2169.2-2466.4 ns)
   2169.2 |########################################
   2184.1 |
   2198.9 |
   2213.8 |########################################
   2228.6 |
   2243.5 |
   2258.4 |
   2273.2 |
   2288.1 |########################################
   2303.0 |
   2317.8 |
   2332.7 |
   2347.5 |
   2362.4 |
   2377.3 |
   2392.1 |
   2407.0 |
   2421.9 |
   2436.7 |########################################
   2451.6 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 2085.4-2340.6 ns)
   2085.4 |########################################
   2098.2 |
   2110.9 |
   2123.7 |
   2136.4 |
   2149.2 |
   2162.0 |########################################
   2174.7 |########################################
   2187.5 |
   2200.3 |
   2213.0 |
   2225.8 |
   2238.5 |
   2251.3 |
   2264.1 |
   2276.8 |
   2289.6 |
   2302.4 |########################################
   2315.1 |
   2327.9 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 2117.9-2349.4 ns)
   2117.9 |####################
   2129.5 |
   2141.1 |
   2152.6 |####################
   2164.2 |
   2175.8 |
   2187.3 |
   2198.9 |
   2210.5 |
   2222.1 |
   2233.6 |
   2245.2 |
   2256.8 |
   2268.4 |
   2279.9 |####################
   2291.5 |
   2303.1 |
   2314.7 |
   2326.2 |
   2337.8 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 2125.4-2343.8 ns)
   2125.4 |####################
   2136.3 |
   2147.2 |####################
   2158.2 |
   2169.1 |
   2180.0 |
   2190.9 |
   2201.8 |
   2212.7 |
   2223.7 |
   2234.6 |
   2245.5 |
   2256.4 |
   2267.3 |
   2278.2 |
   2289.2 |
   2300.1 |
   2311.0 |####################
   2321.9 |
   2332.8 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 2114.2-2357.8 ns)
   2114.2 |########################################
   2126.4 |########################################
   2138.6 |
   2150.7 |
   2162.9 |
   2175.1 |
   2187.3 |
   2199.4 |
   2211.6 |
   2223.8 |
   2236.0 |
   2248.2 |
   2260.3 |
   2272.5 |
   2284.7 |########################################
   2296.9 |########################################
   2309.0 |
   2321.2 |
   2333.4 |########################################
   2345.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=3635.4% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=3819.0% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=3709.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=3690.7% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=3732.3% of algo (FFI overhead may distort results)

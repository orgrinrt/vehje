# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_opt_real_all, carrier_opt_real_cse) are a dead heat (<1%)

carrier_opt_real_all (2.14 ms) and carrier_opt_real_cse (2.16 ms) differ by 0.74%, inside the noise, even though the wider field spreads 16.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_real_all shows alternating (throttle bounce) (autocorr -0.71)

carrier_opt_real_all's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_real_all** at 2139682.3 ns median (-13.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.17x (fastest 2139682.3 ns, slowest 2502214.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 2146260ns | 2143414ns | 2125547ns | 2140479ns | 2165289ns | -13.86% |
| carrier_opt_real_canon | 2163843ns | 2165687ns | 2146596ns | 2161154ns | 2176502ns | -13.15% |
| carrier_opt_real_cse | 2159992ns | 2158723ns | 2137114ns | 2153807ns | 2180710ns | -13.31% |
| carrier_opt_real_dce | 2502614ns | 2501934ns | 2492288ns | 2500736ns | 2510595ns | +0.44% |
| carrier_opt_real_fold | 2506332ns | 2505697ns | 2501362ns | 2504760ns | 2511175ns | +0.59% |
| carrier_opt_real_none | 2491581ns | 2486782ns | 2461859ns | 2483599ns | 2518415ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 2142755ns | 2121787ns | 2162189ns | -13.87% | 0.008 |
| carrier_opt_real_canon | 2160555ns | 2142940ns | 2173266ns | -13.16% | 0.008 |
| carrier_opt_real_cse | 2156798ns | 2134138ns | 2177633ns | -13.31% | 0.008 |
| carrier_opt_real_dce | 2499148ns | 2488770ns | 2507200ns | +0.45% | 0.007 |
| carrier_opt_real_fold | 2502949ns | 2498068ns | 2507868ns | +0.60% | 0.007 |
| carrier_opt_real_none | 2487900ns | 2457970ns | 2514767ns | base | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_real_all | 13398155 | 14995670 | 0.893 | 0.86× |
| carrier_opt_real_canon | 13444191 | 15038051 | 0.894 | 0.87× |
| carrier_opt_real_cse | 13417529 | 15045390 | 0.892 | 0.86× |
| carrier_opt_real_dce | 15590180 | 16609480 | 0.939 | 1.00× |
| carrier_opt_real_fold | 15534370 | 16620118 | 0.935 | 1.00× |
| carrier_opt_real_none | 15520484 | 16609278 | 0.934 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_opt_real_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.008 | 99.2% |
| carrier_opt_real_canon | 0.008 | 98.1% |
| carrier_opt_real_cse | 0.008 | 98.4% |
| carrier_opt_real_dce | 0.007 | 84.9% |
| carrier_opt_real_fold | 0.007 | 84.8% |
| carrier_opt_real_none | 0.007 | 85.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 2146260ns | 2146260ns | -13.86% |
| carrier_opt_real_canon | 2163843ns | 2163843ns | -13.15% |
| carrier_opt_real_cse | 2159992ns | 2159992ns | -13.31% |
| carrier_opt_real_dce | 2502614ns | 2502614ns | +0.44% |
| carrier_opt_real_fold | 2506332ns | 2506332ns | +0.59% |
| carrier_opt_real_none | 2491581ns | 2491581ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 2483071ns | base | --- | [2465862, 2514767] | --- | --- | --- | --- |
| carrier_opt_real_all | 2139682ns | -338624.6ns (-13.6%) | [-378027, -318785]ns | [2126392, 2162189] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_canon | 2162346ns | -328126.2ns (-13.2%) | [-346747, -307162]ns | [2146053, 2173266] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_cse | 2155506ns | -339017.5ns (-13.7%) | [-363962, -290326]ns | [2137255, 2177633] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_dce | 2498494ns | no significant difference | [-18122, +39240]ns | [2491751, 2507200] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_real_fold | 2502214ns | no significant difference | [-16004, +37975]ns | [2498764, 2507868] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_canon | carrier_opt_real_cse | carrier_opt_real_dce | carrier_opt_real_fold |
|---|---|---|---|---|---|---|
| 1 | 2473755ns | -12.9% | -13.4% | -13.7% | +0.6% | +1.3% |
| 2 | 2457970ns | -13.7% | -12.6% | -11.2% | +1.9% | +1.8% |
| 3 | 2488193ns | -12.8% | -13.1% | -13.6% | +0.4% | +0.9% |
| 4 | 2519562ns | -15.4% | -13.7% | -15.0% | -0.8% | -0.8% |
| 5 | 2477949ns | -13.8% | -12.3% | -12.4% | +1.3% | +1.0% |
| 6 | 2509972ns | -14.6% | -13.9% | -13.9% | -0.6% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.710 | HIGH- (thermal bounce) |
| carrier_opt_real_canon | 0.496 | moderate+ |
| carrier_opt_real_cse | -0.480 | moderate- |
| carrier_opt_real_dce | -0.450 | moderate- |
| carrier_opt_real_fold | -0.257 | moderate- |
| carrier_opt_real_none | -0.041 | ok |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_canon**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 2/6, lost 4/6
- **carrier_opt_real_fold**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 2147868.0ns | 2142754.5ns | 100.2% | HIGH |
| carrier_opt_real_canon | 2162201.5ns | 2160555.1ns | 100.1% | HIGH |
| carrier_opt_real_cse | 2159304.6ns | 2156798.3ns | 100.1% | HIGH |
| carrier_opt_real_dce | 2502465.0ns | 2499148.4ns | 100.1% | HIGH |
| carrier_opt_real_fold | 2503996.5ns | 2502948.8ns | 100.0% | HIGH |
| carrier_opt_real_none | 2490011.9ns | 2487900.1ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 2121787.1-2162188.8 ns)
  2121787.1 |########################################
  2123807.2 |
  2125827.3 |
  2127847.3 |
  2129867.4 |########################################
  2131887.5 |
  2133907.6 |
  2135927.7 |########################################
  2137947.8 |
  2139967.8 |
  2141987.9 |########################################
  2144008.0 |
  2146028.1 |
  2148048.2 |
  2150068.3 |
  2152088.3 |########################################
  2154108.4 |
  2156128.5 |
  2158148.6 |
  2160168.7 |
  (0 below, 1 above range)

carrier_opt_real_canon (n=6, range 2142940.4-2173265.9 ns)
  2142940.4 |########################################
  2144456.7 |
  2145972.9 |
  2147489.2 |
  2149005.5 |########################################
  2150521.8 |
  2152038.0 |
  2153554.3 |
  2155070.6 |
  2156586.9 |
  2158103.1 |
  2159619.4 |
  2161135.7 |########################################
  2162651.9 |########################################
  2164168.2 |
  2165684.5 |
  2167200.8 |
  2168717.0 |
  2170233.3 |
  2171749.6 |########################################
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 2134137.5-2177633.2 ns)
  2134137.5 |########################################
  2136312.3 |
  2138487.1 |########################################
  2140661.8 |
  2142836.6 |
  2145011.4 |
  2147186.2 |
  2149361.0 |########################################
  2151535.8 |
  2153710.5 |
  2155885.3 |
  2158060.1 |
  2160234.9 |########################################
  2162409.7 |
  2164584.5 |
  2166759.2 |
  2168934.0 |
  2171108.8 |########################################
  2173283.6 |
  2175458.4 |
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 2488769.6-2507200.0 ns)
  2488769.6 |####################
  2489691.1 |
  2490612.6 |
  2491534.2 |
  2492455.7 |
  2493377.2 |
  2494298.7 |####################
  2495220.2 |
  2496141.8 |
  2497063.3 |
  2497984.8 |########################################
  2498906.3 |
  2499827.8 |
  2500749.4 |
  2501670.9 |
  2502592.4 |
  2503513.9 |
  2504435.4 |
  2505357.0 |####################
  2506278.5 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 2498067.5-2507868.3 ns)
  2498067.5 |####################
  2498557.5 |
  2499047.6 |####################
  2499537.6 |
  2500027.7 |
  2500517.7 |
  2501007.8 |
  2501497.8 |
  2501987.8 |########################################
  2502477.9 |
  2502967.9 |
  2503458.0 |
  2503948.0 |
  2504438.1 |
  2504928.1 |####################
  2505418.1 |
  2505908.2 |
  2506398.2 |
  2506888.3 |
  2507378.3 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 2457970.0-2514767.1 ns)
  2457970.0 |########################################
  2460809.9 |
  2463649.7 |
  2466489.6 |
  2469329.4 |
  2472169.3 |########################################
  2475009.1 |
  2477849.0 |########################################
  2480688.8 |
  2483528.7 |
  2486368.5 |########################################
  2489208.4 |
  2492048.3 |
  2494888.1 |
  2497728.0 |
  2500567.8 |
  2503407.7 |
  2506247.5 |
  2509087.4 |########################################
  2511927.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_canon**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=100.1% of algo (FFI overhead may distort results)

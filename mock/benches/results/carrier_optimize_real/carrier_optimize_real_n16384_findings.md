# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_opt_real_all, carrier_opt_real_cseeqsat) are a dead heat (<1%)

carrier_opt_real_all (2.13 ms) and carrier_opt_real_cseeqsat (2.13 ms) differ by 0.08%, inside the noise, even though the wider field spreads 16.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_real_all shows alternating (throttle bounce) (autocorr -0.69)

carrier_opt_real_all's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_real_all** at 2128786.0 ns median (-14.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.16x (fastest 2128786.0 ns, slowest 2476962.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 2132075ns | 2132511ns | 2109541ns | 2129874ns | 2146644ns | -13.95% |
| carrier_opt_real_cse | 2149633ns | 2148676ns | 2146020ns | 2147846ns | 2154120ns | -13.24% |
| carrier_opt_real_cseeqsat | 2132059ns | 2134531ns | 2117871ns | 2132935ns | 2137840ns | -13.95% |
| carrier_opt_real_dce | 2478561ns | 2480774ns | 2467508ns | 2478402ns | 2484325ns | +0.03% |
| carrier_opt_real_eqsat | 2145248ns | 2146182ns | 2133327ns | 2143019ns | 2154553ns | -13.42% |
| carrier_opt_real_fold | 2468116ns | 2470114ns | 2452005ns | 2467197ns | 2477549ns | -0.39% |
| carrier_opt_real_none | 2477714ns | 2477946ns | 2459028ns | 2476450ns | 2488953ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 2128522ns | 2105930ns | 2143235ns | -13.96% | 0.008 |
| carrier_opt_real_cse | 2145876ns | 2142134ns | 2150461ns | -13.26% | 0.008 |
| carrier_opt_real_cseeqsat | 2127998ns | 2114170ns | 2133710ns | -13.98% | 0.008 |
| carrier_opt_real_dce | 2474694ns | 2463853ns | 2480592ns | +0.03% | 0.007 |
| carrier_opt_real_eqsat | 2141311ns | 2129237ns | 2150476ns | -13.45% | 0.008 |
| carrier_opt_real_fold | 2464244ns | 2447746ns | 2473800ns | -0.39% | 0.007 |
| carrier_opt_real_none | 2473965ns | 2456128ns | 2485046ns | base | 0.007 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_opt_real_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.008 | 98.9% |
| carrier_opt_real_cse | 0.008 | 98.2% |
| carrier_opt_real_cseeqsat | 0.008 | 98.8% |
| carrier_opt_real_dce | 0.007 | 85.0% |
| carrier_opt_real_eqsat | 0.008 | 98.3% |
| carrier_opt_real_fold | 0.007 | 85.4% |
| carrier_opt_real_none | 0.007 | 85.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 2132075ns | 2132075ns | -13.95% |
| carrier_opt_real_cse | 2149633ns | 2149633ns | -13.24% |
| carrier_opt_real_cseeqsat | 2132059ns | 2132059ns | -13.95% |
| carrier_opt_real_dce | 2478561ns | 2478561ns | +0.03% |
| carrier_opt_real_eqsat | 2145248ns | 2145248ns | -13.42% |
| carrier_opt_real_fold | 2468116ns | 2468116ns | -0.39% |
| carrier_opt_real_none | 2477714ns | 2477714ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 2473982ns | base | --- | [2462866, 2485046] | --- | --- | --- | --- |
| carrier_opt_real_all | 2128786ns | -350923.0ns (-14.2%) | [-364468, -320939]ns | [2113545, 2143235] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_cse | 2144714ns | -328826.7ns (-13.3%) | [-339455, -315984]ns | [2142454, 2150461] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_cseeqsat | 2130590ns | -342796.5ns (-13.9%) | [-362829, -332276]ns | [2119694, 2133710] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_dce | 2476963ns | no significant difference | [-18520, +17726]ns | [2466527, 2480592] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_real_eqsat | 2142569ns | -331131.2ns (-13.4%) | [-349374, -317456]ns | [2130887, 2150476] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_real_fold | 2466131ns | no significant difference | [-27629, +10934]ns | [2452802, 2473800] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_cse | carrier_opt_real_cseeqsat | carrier_opt_real_dce | carrier_opt_real_eqsat | carrier_opt_real_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2480794ns | -14.5% | -13.3% | -14.0% | -0.7% | -13.7% | -0.9% |
| 2 | 2456128ns | -12.6% | -12.6% | -13.3% | +1.0% | -12.7% | +0.6% |
| 3 | 2469605ns | -14.1% | -13.2% | -13.7% | +0.5% | -13.6% | +0.3% |
| 4 | 2472219ns | -13.4% | -13.0% | -13.7% | +0.1% | -13.0% | -0.1% |
| 5 | 2475745ns | -14.9% | -13.4% | -14.6% | +0.1% | -13.1% | -1.1% |
| 6 | 2489299ns | -14.2% | -13.9% | -14.6% | -0.8% | -14.5% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.694 | HIGH- (thermal bounce) |
| carrier_opt_real_cse | -0.267 | moderate- |
| carrier_opt_real_cseeqsat | 0.021 | ok |
| carrier_opt_real_dce | -0.207 | moderate- |
| carrier_opt_real_eqsat | -0.307 | moderate- |
| carrier_opt_real_fold | 0.096 | ok |
| carrier_opt_real_none | -0.020 | ok |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 2/6, lost 4/6
- **carrier_opt_real_eqsat**: won 6/6, lost 0/6
- **carrier_opt_real_fold**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 2133263.8ns | 2128521.8ns | 100.2% | HIGH |
| carrier_opt_real_cse | 2148817.4ns | 2145876.4ns | 100.1% | HIGH |
| carrier_opt_real_cseeqsat | 2129865.3ns | 2127998.0ns | 100.1% | HIGH |
| carrier_opt_real_dce | 2477001.7ns | 2474694.0ns | 100.1% | HIGH |
| carrier_opt_real_eqsat | 2144338.9ns | 2141310.8ns | 100.1% | HIGH |
| carrier_opt_real_fold | 2468763.8ns | 2464244.4ns | 100.2% | HIGH |
| carrier_opt_real_none | 2475265.0ns | 2473964.9ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 2105930.0-2143234.5 ns)
  2105930.0 |####################
  2107795.2 |
  2109660.5 |
  2111525.7 |
  2113390.9 |
  2115256.1 |
  2117121.4 |
  2118986.6 |
  2120851.8 |########################################
  2122717.0 |
  2124582.3 |
  2126447.5 |
  2128312.7 |
  2130178.0 |
  2132043.2 |
  2133908.4 |
  2135773.6 |####################
  2137638.9 |
  2139504.1 |####################
  2141369.3 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 2142133.8-2150461.0 ns)
  2142133.8 |########################################
  2142550.2 |########################################
  2142966.5 |
  2143382.9 |
  2143799.2 |########################################
  2144215.6 |
  2144632.0 |
  2145048.3 |########################################
  2145464.7 |
  2145881.0 |
  2146297.4 |
  2146713.8 |
  2147130.1 |
  2147546.5 |
  2147962.8 |
  2148379.2 |
  2148795.6 |
  2149211.9 |
  2149628.3 |########################################
  2150044.6 |
  (0 below, 1 above range)

carrier_opt_real_cseeqsat (n=6, range 2114169.6-2133710.0 ns)
  2114169.6 |########################################
  2115146.6 |
  2116123.6 |
  2117100.7 |
  2118077.7 |
  2119054.7 |
  2120031.7 |
  2121008.7 |
  2121985.8 |
  2122962.8 |
  2123939.8 |
  2124916.8 |########################################
  2125893.8 |
  2126870.9 |
  2127847.9 |
  2128824.9 |
  2129801.9 |########################################
  2130778.9 |########################################
  2131756.0 |
  2132733.0 |########################################
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 2463853.3-2480592.5 ns)
  2463853.3 |########################################
  2464690.3 |
  2465527.2 |
  2466364.2 |
  2467201.1 |
  2468038.1 |
  2468875.1 |########################################
  2469712.0 |
  2470549.0 |
  2471385.9 |
  2472222.9 |
  2473059.9 |
  2473896.8 |
  2474733.8 |########################################
  2475570.7 |
  2476407.7 |
  2477244.7 |
  2478081.6 |
  2478918.6 |########################################
  2479755.5 |########################################
  (0 below, 1 above range)

carrier_opt_real_eqsat (n=6, range 2129237.1-2150475.9 ns)
  2129237.1 |####################
  2130299.0 |
  2131361.0 |
  2132422.9 |####################
  2133484.9 |
  2134546.8 |
  2135608.7 |
  2136670.7 |
  2137732.6 |
  2138794.5 |
  2139856.5 |
  2140918.4 |
  2141980.4 |########################################
  2143042.3 |
  2144104.2 |
  2145166.2 |
  2146228.1 |
  2147290.0 |
  2148352.0 |
  2149413.9 |####################
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 2447746.2-2473800.0 ns)
  2447746.2 |####################
  2449048.9 |
  2450351.6 |
  2451654.3 |
  2452957.0 |
  2454259.7 |
  2455562.3 |
  2456865.0 |####################
  2458167.7 |
  2459470.4 |
  2460773.1 |####################
  2462075.8 |
  2463378.5 |
  2464681.2 |
  2465983.9 |
  2467286.5 |
  2468589.2 |
  2469891.9 |########################################
  2471194.6 |
  2472497.3 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 2456127.5-2485046.5 ns)
  2456127.5 |########################################
  2457573.5 |
  2459019.4 |
  2460465.4 |
  2461911.3 |
  2463357.2 |
  2464803.2 |
  2466249.1 |
  2467695.1 |
  2469141.0 |########################################
  2470587.0 |
  2472033.0 |########################################
  2473478.9 |
  2474924.9 |########################################
  2476370.8 |
  2477816.8 |
  2479262.7 |
  2480708.6 |########################################
  2482154.6 |
  2483600.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_cseeqsat**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_eqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=100.0% of algo (FFI overhead may distort results)

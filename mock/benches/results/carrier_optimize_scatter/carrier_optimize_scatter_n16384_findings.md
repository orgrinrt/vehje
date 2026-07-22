# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_scatter_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_scatter_none has the worst median (2.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_scatter_eqsat at 2.26 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (carrier_opt_scatter_eqsat, carrier_opt_scatter_all) are a dead heat (<1%)

carrier_opt_scatter_eqsat (2.26 ms) and carrier_opt_scatter_all (2.26 ms) differ by 0.12%, inside the noise, even though the wider field spreads 10.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_scatter_dce shows alternating (throttle bounce) (autocorr -0.66)

carrier_opt_scatter_dce's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_scatter_eqsat** at 2257987.7 ns median (-9.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.11x (fastest 2257987.7 ns, slowest 2504643.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 2265690ns | 2264264ns | 2255598ns | 2263513ns | 2274001ns | -11.71% |
| carrier_opt_scatter_cse | 2296397ns | 2295406ns | 2286349ns | 2292489ns | 2307283ns | -10.52% |
| carrier_opt_scatter_cseeqsat | 2261629ns | 2264264ns | 2224728ns | 2260437ns | 2281868ns | -11.87% |
| carrier_opt_scatter_dce | 2499520ns | 2496633ns | 2484375ns | 2494285ns | 2514944ns | -2.60% |
| carrier_opt_scatter_eqsat | 2265015ns | 2261445ns | 2253930ns | 2260037ns | 2278025ns | -11.74% |
| carrier_opt_scatter_fold | 2499754ns | 2505883ns | 2458396ns | 2501466ns | 2517865ns | -2.59% |
| carrier_opt_scatter_none | 2566333ns | 2508647ns | 2499339ns | 2505640ns | 2690869ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 2261848ns | 2251604ns | 2269808ns | -11.74% | 0.007 |
| carrier_opt_scatter_cse | 2292890ns | 2283389ns | 2303264ns | -10.52% | 0.007 |
| carrier_opt_scatter_cseeqsat | 2257988ns | 2220644ns | 2277984ns | -11.89% | 0.007 |
| carrier_opt_scatter_dce | 2495810ns | 2479968ns | 2511131ns | -2.61% | 0.007 |
| carrier_opt_scatter_eqsat | 2261534ns | 2251107ns | 2274331ns | -11.75% | 0.007 |
| carrier_opt_scatter_fold | 2496080ns | 2455952ns | 2513830ns | -2.59% | 0.007 |
| carrier_opt_scatter_none | 2562571ns | 2495847ns | 2687030ns | base | 0.006 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_opt_scatter_cseeqsat; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.007 | 98.2% |
| carrier_opt_scatter_cse | 0.007 | 96.9% |
| carrier_opt_scatter_cseeqsat | 0.007 | 98.2% |
| carrier_opt_scatter_dce | 0.007 | 89.1% |
| carrier_opt_scatter_eqsat | 0.007 | 98.3% |
| carrier_opt_scatter_fold | 0.007 | 88.7% |
| carrier_opt_scatter_none | 0.007 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 2265690ns | 2265690ns | -11.71% |
| carrier_opt_scatter_cse | 2296397ns | 2296397ns | -10.52% |
| carrier_opt_scatter_cseeqsat | 2261629ns | 2261629ns | -11.87% |
| carrier_opt_scatter_dce | 2499520ns | 2499520ns | -2.60% |
| carrier_opt_scatter_eqsat | 2265015ns | 2265015ns | -11.74% |
| carrier_opt_scatter_fold | 2499754ns | 2499754ns | -2.59% |
| carrier_opt_scatter_none | 2566333ns | 2566333ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 2504643ns | base | --- | [2496039, 2687030] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 2260760ns | -242529.4ns (-9.7%) | [-426649, -232990]ns | [2254976, 2269808] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 2291941ns | -209688.1ns (-8.4%) | [-402241, -197113]ns | [2283465, 2303264] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_cseeqsat | 2260801ns | -241387.4ns (-9.6%) | [-446792, -225568]ns | [2235179, 2277984] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 2493167ns | no significant difference | [-195751, +6961]ns | [2483133, 2511131] | no | 0.2625 | 0.2188 | 0 |
| carrier_opt_scatter_eqsat | 2257988ns | -244685.8ns (-9.8%) | [-420366, -238057]ns | [2252284, 2274331] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_fold | 2502244ns | no significant difference | [-197218, +6754]ns | [2472165, 2513830] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_cse | carrier_opt_scatter_cseeqsat | carrier_opt_scatter_dce | carrier_opt_scatter_eqsat | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2867402ns | -21.2% | -20.4% | -21.2% | -12.7% | -20.5% | -12.4% |
| 2 | 2504908ns | -9.2% | -8.1% | -8.7% | -0.7% | -10.1% | -0.7% |
| 3 | 2504378ns | -9.7% | -8.2% | -9.4% | +0.6% | -9.9% | +0.2% |
| 4 | 2506658ns | -9.7% | -8.8% | -11.4% | -1.1% | -9.5% | +0.3% |
| 5 | 2495847ns | -9.8% | -7.7% | -9.4% | -0.1% | -9.7% | -0.1% |
| 6 | 2496231ns | -9.5% | -8.5% | -9.9% | -0.2% | -9.5% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.158 | ok |
| carrier_opt_scatter_cse | -0.540 | HIGH- (thermal bounce) |
| carrier_opt_scatter_cseeqsat | -0.078 | ok |
| carrier_opt_scatter_dce | -0.660 | HIGH- (thermal bounce) |
| carrier_opt_scatter_eqsat | -0.351 | moderate- |
| carrier_opt_scatter_fold | 0.022 | ok |
| carrier_opt_scatter_none | -0.025 | ok |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 4/6, lost 1/6
- **carrier_opt_scatter_eqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_fold**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 2263746.0ns | 2261847.8ns | 100.1% | HIGH |
| carrier_opt_scatter_cse | 2296883.5ns | 2292889.9ns | 100.2% | HIGH |
| carrier_opt_scatter_cseeqsat | 2263011.7ns | 2257988.1ns | 100.2% | HIGH |
| carrier_opt_scatter_dce | 2497433.7ns | 2495810.3ns | 100.1% | HIGH |
| carrier_opt_scatter_eqsat | 2263240.1ns | 2261534.4ns | 100.1% | HIGH |
| carrier_opt_scatter_fold | 2500001.1ns | 2496079.7ns | 100.2% | HIGH |
| carrier_opt_scatter_none | 2679779.8ns | 2562570.6ns | 104.6% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 2251604.2-2269807.9 ns)
  2251604.2 |########################################
  2252514.4 |
  2253424.6 |
  2254334.8 |
  2255244.9 |
  2256155.1 |
  2257065.3 |
  2257975.5 |########################################
  2258885.7 |########################################
  2259795.9 |
  2260706.1 |
  2261616.2 |########################################
  2262526.4 |
  2263436.6 |########################################
  2264346.8 |
  2265257.0 |
  2266167.2 |
  2267077.3 |
  2267987.5 |
  2268897.7 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 2283388.8-2303264.3 ns)
  2283388.8 |########################################
  2284382.6 |
  2285376.4 |####################
  2286370.1 |
  2287363.9 |
  2288357.7 |
  2289351.5 |
  2290345.2 |
  2291339.0 |
  2292332.8 |
  2293326.6 |
  2294320.4 |
  2295314.1 |
  2296307.9 |
  2297301.7 |####################
  2298295.5 |
  2299289.2 |
  2300283.0 |
  2301276.8 |
  2302270.6 |####################
  (0 below, 1 above range)

carrier_opt_scatter_cseeqsat (n=6, range 2220643.7-2277984.2 ns)
  2220643.7 |########################################
  2223510.7 |
  2226377.8 |
  2229244.8 |
  2232111.8 |
  2234978.8 |
  2237845.9 |
  2240712.9 |
  2243579.9 |
  2246446.9 |
  2249314.0 |########################################
  2252181.0 |
  2255048.0 |
  2257915.0 |########################################
  2260782.1 |########################################
  2263649.1 |
  2266516.1 |########################################
  2269383.1 |
  2272250.2 |
  2275117.2 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 2479967.9-2511130.9 ns)
  2479967.9 |########################################
  2481526.0 |
  2483084.2 |
  2484642.3 |
  2486200.5 |########################################
  2487758.6 |
  2489316.8 |
  2490874.9 |########################################
  2492433.1 |
  2493991.2 |########################################
  2495549.4 |
  2497107.5 |
  2498665.7 |
  2500223.8 |
  2501782.0 |########################################
  2503340.1 |
  2504898.3 |
  2506456.4 |
  2508014.6 |
  2509572.7 |
  (0 below, 1 above range)

carrier_opt_scatter_eqsat (n=6, range 2251107.1-2274331.0 ns)
  2251107.1 |########################################
  2252268.3 |
  2253429.5 |########################################
  2254590.7 |
  2255751.9 |
  2256913.1 |########################################
  2258074.3 |########################################
  2259235.5 |
  2260396.7 |
  2261557.9 |
  2262719.1 |
  2263880.3 |
  2265041.5 |
  2266202.7 |
  2267363.9 |########################################
  2268525.1 |
  2269686.3 |
  2270847.5 |
  2272008.7 |
  2273169.9 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 2455951.7-2513829.8 ns)
  2455951.7 |########################################
  2458845.6 |
  2461739.5 |
  2464633.4 |
  2467527.3 |
  2470421.2 |
  2473315.1 |
  2476209.0 |
  2479102.9 |
  2481996.8 |
  2484890.8 |
  2487784.7 |########################################
  2490678.6 |
  2493572.5 |########################################
  2496466.4 |
  2499360.3 |
  2502254.2 |
  2505148.1 |
  2508042.0 |########################################
  2510935.9 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 2495846.7-2687029.8 ns)
  2495846.7 |########################################
  2505405.9 |##########
  2514965.0 |
  2524524.2 |
  2534083.3 |
  2543642.5 |
  2553201.6 |
  2562760.8 |
  2572319.9 |
  2581879.1 |
  2591438.2 |
  2600997.4 |
  2610556.6 |
  2620115.7 |
  2629674.9 |
  2639234.0 |
  2648793.2 |
  2658352.3 |
  2667911.5 |
  2677470.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cseeqsat**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_eqsat**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=100.3% of algo (FFI overhead may distort results)

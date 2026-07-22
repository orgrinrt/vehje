# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_opt_scatter_all, carrier_opt_scatter_fold) are a dead heat (<1%)

carrier_opt_scatter_all (2.18 us) and carrier_opt_scatter_fold (2.19 us) differ by 0.61%, inside the noise, even though the wider field spreads 6.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_scatter_dce's edge over baseline is significant but tiny (-1 ns, 0.05%)

carrier_opt_scatter_dce differs from baseline carrier_opt_scatter_none by -1 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_scatter_all** at 2179.2 ns median (-5.5% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 1.06x (fastest 2179.2 ns, slowest 2313.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 4814ns | 4760ns | 4735ns | 4753ns | 4943ns | -1.80% |
| carrier_opt_scatter_cse | 4760ns | 4838ns | 4210ns | 4802ns | 4973ns | -2.89% |
| carrier_opt_scatter_cseeqsat | 4813ns | 4816ns | 4700ns | 4797ns | 4895ns | -1.80% |
| carrier_opt_scatter_dce | 4928ns | 4917ns | 4827ns | 4907ns | 5010ns | +0.54% |
| carrier_opt_scatter_eqsat | 4933ns | 4812ns | 4768ns | 4806ns | 5205ns | +0.63% |
| carrier_opt_scatter_fold | 4842ns | 4802ns | 4745ns | 4788ns | 4971ns | -1.22% |
| carrier_opt_scatter_none | 4902ns | 4899ns | 4840ns | 4885ns | 4958ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 2184ns | 2156ns | 2216ns | -5.56% | 0.029 |
| carrier_opt_scatter_cse | 2204ns | 1919ns | 2277ns | -4.70% | 0.029 |
| carrier_opt_scatter_cseeqsat | 2222ns | 2196ns | 2250ns | -3.92% | 0.029 |
| carrier_opt_scatter_dce | 2314ns | 2288ns | 2333ns | +0.07% | 0.028 |
| carrier_opt_scatter_eqsat | 2223ns | 2196ns | 2271ns | -3.87% | 0.029 |
| carrier_opt_scatter_fold | 2203ns | 2156ns | 2254ns | -4.77% | 0.029 |
| carrier_opt_scatter_none | 2313ns | 2280ns | 2352ns | base | 0.028 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_opt_scatter_cse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.029 | 88.0% |
| carrier_opt_scatter_cse | 0.028 | 84.8% |
| carrier_opt_scatter_cseeqsat | 0.029 | 86.6% |
| carrier_opt_scatter_dce | 0.028 | 82.9% |
| carrier_opt_scatter_eqsat | 0.029 | 87.1% |
| carrier_opt_scatter_fold | 0.029 | 87.5% |
| carrier_opt_scatter_none | 0.028 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 4814ns | 4814ns | -1.80% |
| carrier_opt_scatter_cse | 4760ns | 4760ns | -2.89% |
| carrier_opt_scatter_cseeqsat | 4813ns | 4813ns | -1.80% |
| carrier_opt_scatter_dce | 4928ns | 4928ns | +0.54% |
| carrier_opt_scatter_eqsat | 4933ns | 4933ns | +0.63% |
| carrier_opt_scatter_fold | 4842ns | 4842ns | -1.22% |
| carrier_opt_scatter_none | 4902ns | 4902ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 2305ns | base | --- | [2281, 2352] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 2179ns | -133.5ns (-5.8%) | [-171, -82]ns | [2158, 2216] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 2263ns | -70.6ns (-3.1%) | [-234, -22]ns | [2072, 2277] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_cseeqsat | 2216ns | -90.6ns (-3.9%) | [-135, -46]ns | [2201, 2250] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 2314ns | no significant difference | [-38, +44]ns | [2296, 2333] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_scatter_eqsat | 2202ns | -87.3ns (-3.8%) | [-155, -26]ns | [2197, 2271] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| carrier_opt_scatter_fold | 2192ns | -106.3ns (-4.6%) | [-147, -77]ns | [2161, 2254] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_cse | carrier_opt_scatter_cseeqsat | carrier_opt_scatter_dce | carrier_opt_scatter_eqsat | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2282ns | -4.4% | -15.9% | -3.3% | +1.0% | -3.2% | -5.5% |
| 2 | 2358ns | -8.6% | -4.5% | -6.9% | -1.8% | -6.9% | -3.2% |
| 3 | 2312ns | -5.9% | -1.7% | -4.5% | -1.1% | +0.9% | -3.7% |
| 4 | 2346ns | -5.6% | -2.9% | -4.6% | -1.5% | -6.3% | -6.9% |
| 5 | 2280ns | -2.8% | -0.2% | -2.4% | +1.6% | -3.3% | -3.5% |
| 6 | 2298ns | -6.1% | -3.2% | -1.6% | +2.2% | -4.3% | -5.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | 0.067 | ok |
| carrier_opt_scatter_cse | 0.013 | ok |
| carrier_opt_scatter_cseeqsat | 0.259 | moderate+ |
| carrier_opt_scatter_dce | 0.036 | ok |
| carrier_opt_scatter_eqsat | -0.301 | moderate- |
| carrier_opt_scatter_fold | -0.212 | moderate- |
| carrier_opt_scatter_none | -0.379 | moderate- |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 3/6, lost 3/6
- **carrier_opt_scatter_eqsat**: won 5/6, lost 1/6
- **carrier_opt_scatter_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 87214.7ns | 2184.2ns | 3993.0% | HIGH |
| carrier_opt_scatter_cse | 86059.9ns | 2204.1ns | 3904.6% | HIGH |
| carrier_opt_scatter_cseeqsat | 86189.6ns | 2222.3ns | 3878.4% | HIGH |
| carrier_opt_scatter_dce | 86474.0ns | 2314.4ns | 3736.4% | HIGH |
| carrier_opt_scatter_eqsat | 87194.3ns | 2223.4ns | 3921.6% | HIGH |
| carrier_opt_scatter_fold | 86410.9ns | 2202.6ns | 3923.2% | HIGH |
| carrier_opt_scatter_none | 86405.1ns | 2312.8ns | 3735.9% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 2155.8-2215.8 ns)
   2155.8 |########################################
   2158.8 |########################################
   2161.8 |
   2164.8 |
   2167.8 |
   2170.8 |
   2173.8 |########################################
   2176.8 |
   2179.8 |########################################
   2182.8 |
   2185.8 |
   2188.8 |
   2191.8 |
   2194.8 |
   2197.8 |
   2200.8 |
   2203.8 |
   2206.8 |
   2209.8 |
   2212.8 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 1918.7-2276.6 ns)
   1918.7 |####################
   1936.6 |
   1954.5 |
   1972.4 |
   1990.3 |
   2008.2 |
   2026.1 |
   2044.0 |
   2061.9 |
   2079.8 |
   2097.7 |
   2115.6 |
   2133.5 |
   2151.4 |
   2169.3 |
   2187.2 |
   2205.1 |
   2223.0 |####################
   2240.9 |####################
   2258.8 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_cseeqsat (n=6, range 2195.8-2249.8 ns)
   2195.8 |########################################
   2198.5 |
   2201.2 |
   2203.9 |########################################
   2206.6 |########################################
   2209.3 |
   2212.0 |
   2214.7 |
   2217.4 |
   2220.1 |
   2222.8 |########################################
   2225.5 |
   2228.2 |
   2230.9 |
   2233.6 |
   2236.3 |########################################
   2239.0 |
   2241.7 |
   2244.4 |
   2247.1 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 2287.9-2333.1 ns)
   2287.9 |####################
   2290.2 |
   2292.4 |
   2294.7 |
   2296.9 |
   2299.2 |
   2301.5 |
   2303.7 |####################
   2306.0 |
   2308.3 |
   2310.5 |####################
   2312.8 |
   2315.0 |########################################
   2317.3 |
   2319.6 |
   2321.8 |
   2324.1 |
   2326.4 |
   2328.6 |
   2330.9 |
  (0 below, 1 above range)

carrier_opt_scatter_eqsat (n=6, range 2195.8-2270.9 ns)
   2195.8 |########################################
   2199.6 |####################
   2203.3 |####################
   2207.1 |####################
   2210.8 |
   2214.6 |
   2218.3 |
   2222.1 |
   2225.8 |
   2229.6 |
   2233.3 |
   2237.1 |
   2240.8 |
   2244.6 |
   2248.3 |
   2252.1 |
   2255.8 |
   2259.6 |
   2263.3 |
   2267.1 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 2155.8-2253.9 ns)
   2155.8 |########################################
   2160.7 |
   2165.6 |########################################
   2170.5 |
   2175.4 |
   2180.3 |########################################
   2185.2 |
   2190.2 |
   2195.1 |
   2200.0 |########################################
   2204.9 |
   2209.8 |
   2214.7 |
   2219.6 |
   2224.5 |########################################
   2229.4 |
   2234.3 |
   2239.2 |
   2244.1 |
   2249.0 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 2280.4-2352.1 ns)
   2280.4 |########################################
   2284.0 |
   2287.6 |
   2291.1 |
   2294.7 |####################
   2298.3 |
   2301.9 |
   2305.5 |
   2309.1 |####################
   2312.6 |
   2316.2 |
   2319.8 |
   2323.4 |
   2327.0 |
   2330.6 |
   2334.1 |
   2337.7 |
   2341.3 |
   2344.9 |####################
   2348.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=3981.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=3800.5% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cseeqsat**: bridge=3886.8% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=3733.7% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_eqsat**: bridge=3923.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=3939.3% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=3749.6% of algo (FFI overhead may distort results)

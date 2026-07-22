# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_real_dce's edge over baseline is significant but tiny (18 ns, 0.82%)

carrier_opt_real_dce differs from baseline carrier_opt_real_none by 18 ns (0.82%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_real_all** at 1908.8 ns median (-13.6% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 1908.8 ns, slowest 2232.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 4482ns | 4482ns | 4387ns | 4462ns | 4560ns | -6.33% |
| carrier_opt_real_cse | 4570ns | 4646ns | 4135ns | 4617ns | 4716ns | -4.50% |
| carrier_opt_real_cseeqsat | 4726ns | 4745ns | 4592ns | 4705ns | 4822ns | -1.24% |
| carrier_opt_real_dce | 4815ns | 4824ns | 4670ns | 4793ns | 4921ns | +0.64% |
| carrier_opt_real_eqsat | 4700ns | 4708ns | 4564ns | 4686ns | 4790ns | -1.77% |
| carrier_opt_real_fold | 4797ns | 4829ns | 4661ns | 4778ns | 4893ns | +0.24% |
| carrier_opt_real_none | 4785ns | 4757ns | 4710ns | 4747ns | 4879ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 1912ns | 1897ns | 1931ns | -13.35% | 0.033 |
| carrier_opt_real_cse | 2013ns | 1793ns | 2078ns | -8.80% | 0.032 |
| carrier_opt_real_cseeqsat | 2106ns | 2080ns | 2137ns | -4.55% | 0.030 |
| carrier_opt_real_dce | 2242ns | 2201ns | 2292ns | +1.61% | 0.029 |
| carrier_opt_real_eqsat | 2104ns | 2068ns | 2139ns | -4.66% | 0.030 |
| carrier_opt_real_fold | 2221ns | 2190ns | 2245ns | +0.63% | 0.029 |
| carrier_opt_real_none | 2207ns | 2173ns | 2226ns | base | 0.029 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_opt_real_cse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.034 | 94.0% |
| carrier_opt_real_cse | 0.031 | 87.5% |
| carrier_opt_real_cseeqsat | 0.030 | 85.4% |
| carrier_opt_real_dce | 0.029 | 80.3% |
| carrier_opt_real_eqsat | 0.030 | 85.2% |
| carrier_opt_real_fold | 0.029 | 80.7% |
| carrier_opt_real_none | 0.029 | 81.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 4482ns | 4482ns | -6.33% |
| carrier_opt_real_cse | 4570ns | 4570ns | -4.50% |
| carrier_opt_real_cseeqsat | 4726ns | 4726ns | -1.24% |
| carrier_opt_real_dce | 4815ns | 4815ns | +0.64% |
| carrier_opt_real_eqsat | 4700ns | 4700ns | -1.77% |
| carrier_opt_real_fold | 4797ns | 4797ns | +0.24% |
| carrier_opt_real_none | 4785ns | 4785ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 2210ns | base | --- | [2184, 2226] | --- | --- | --- | --- |
| carrier_opt_real_all | 1909ns | -298.9ns (-13.5%) | [-324, -261]ns | [1897, 1931] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_real_cse | 2049ns | -154.6ns (-7.0%) | [-297, -130]ns | [1911, 2078] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_real_cseeqsat | 2099ns | -97.7ns (-4.4%) | [-134, -70]ns | [2083, 2137] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_real_dce | 2232ns | +18.1ns (+0.8%) | [+9, +80]ns | [2203, 2292] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_real_eqsat | 2104ns | -100.0ns (-4.5%) | [-133, -76]ns | [2070, 2139] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_real_fold | 2221ns | no significant difference | [-12, +40]ns | [2196, 2245] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_cse | carrier_opt_real_cseeqsat | carrier_opt_real_dce | carrier_opt_real_eqsat | carrier_opt_real_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2210ns | -13.2% | -18.9% | -3.0% | +6.0% | -3.1% | +1.4% |
| 2 | 2173ns | -10.6% | -6.7% | -3.4% | +1.3% | -3.8% | +2.3% |
| 3 | 2210ns | -14.2% | -7.4% | -3.6% | +1.1% | -6.3% | +0.4% |
| 4 | 2231ns | -15.0% | -8.0% | -6.5% | +0.5% | -5.1% | +0.8% |
| 5 | 2195ns | -13.5% | -6.2% | -5.3% | +0.4% | -5.8% | -0.3% |
| 6 | 2222ns | -13.6% | -5.6% | -5.5% | +0.3% | -3.9% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | 0.037 | ok |
| carrier_opt_real_cse | 0.070 | ok |
| carrier_opt_real_cseeqsat | -0.041 | ok |
| carrier_opt_real_dce | -0.250 | moderate- |
| carrier_opt_real_eqsat | -0.413 | moderate- |
| carrier_opt_real_fold | -0.136 | ok |
| carrier_opt_real_none | -0.280 | moderate- |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 0/6, lost 6/6
- **carrier_opt_real_eqsat**: won 6/6, lost 0/6
- **carrier_opt_real_fold**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 86042.3ns | 1912.3ns | 4499.4% | HIGH |
| carrier_opt_real_cse | 85955.5ns | 2012.8ns | 4270.4% | HIGH |
| carrier_opt_real_cseeqsat | 85833.9ns | 2106.5ns | 4074.8% | HIGH |
| carrier_opt_real_dce | 86213.6ns | 2242.4ns | 3844.6% | HIGH |
| carrier_opt_real_eqsat | 85705.6ns | 2104.0ns | 4073.4% | HIGH |
| carrier_opt_real_fold | 86406.0ns | 2220.9ns | 3890.6% | HIGH |
| carrier_opt_real_none | 86376.2ns | 2206.9ns | 3913.9% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 1896.7-1931.2 ns)
   1896.7 |########################################
   1898.4 |####################
   1900.2 |
   1901.9 |
   1903.6 |
   1905.3 |
   1907.1 |
   1908.8 |
   1910.5 |
   1912.2 |
   1914.0 |
   1915.7 |
   1917.4 |####################
   1919.2 |####################
   1920.9 |
   1922.6 |
   1924.3 |
   1926.1 |
   1927.8 |
   1929.5 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 1793.3-2078.3 ns)
   1793.3 |####################
   1807.6 |
   1821.8 |
   1836.1 |
   1850.3 |
   1864.6 |
   1878.8 |
   1893.1 |
   1907.3 |
   1921.6 |
   1935.8 |
   1950.1 |
   1964.3 |
   1978.6 |
   1992.8 |
   2007.1 |
   2021.3 |####################
   2035.6 |####################
   2049.8 |########################################
   2064.1 |
  (0 below, 1 above range)

carrier_opt_real_cseeqsat (n=6, range 2079.6-2136.9 ns)
   2079.6 |########################################
   2082.5 |
   2085.3 |########################################
   2088.2 |
   2091.1 |
   2093.9 |
   2096.8 |########################################
   2099.6 |########################################
   2102.5 |
   2105.4 |
   2108.2 |
   2111.1 |
   2114.0 |
   2116.8 |
   2119.7 |
   2122.5 |
   2125.4 |
   2128.3 |########################################
   2131.1 |
   2134.0 |
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 2201.2-2291.7 ns)
   2201.2 |########################################
   2205.7 |
   2210.2 |
   2214.8 |
   2219.3 |
   2223.8 |
   2228.3 |####################
   2232.9 |####################
   2237.4 |####################
   2241.9 |
   2246.4 |
   2251.0 |
   2255.5 |
   2260.0 |
   2264.5 |
   2269.1 |
   2273.6 |
   2278.1 |
   2282.6 |
   2287.2 |
  (0 below, 1 above range)

carrier_opt_real_eqsat (n=6, range 2067.9-2138.6 ns)
   2067.9 |########################################
   2071.4 |
   2075.0 |
   2078.5 |
   2082.0 |
   2085.6 |
   2089.1 |####################
   2092.6 |
   2096.2 |
   2099.7 |
   2103.2 |
   2106.8 |
   2110.3 |
   2113.8 |
   2117.4 |####################
   2120.9 |
   2124.4 |
   2128.0 |
   2131.5 |
   2135.0 |####################
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 2189.6-2245.2 ns)
   2189.6 |########################################
   2192.4 |
   2195.2 |
   2197.9 |
   2200.7 |########################################
   2203.5 |
   2206.3 |
   2209.1 |
   2211.8 |
   2214.6 |
   2217.4 |########################################
   2220.2 |########################################
   2223.0 |
   2225.7 |
   2228.5 |
   2231.3 |
   2234.1 |
   2236.9 |
   2239.6 |########################################
   2242.4 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 2173.3-2226.4 ns)
   2173.3 |####################
   2176.0 |
   2178.6 |
   2181.3 |
   2183.9 |
   2186.6 |
   2189.2 |
   2191.9 |
   2194.6 |####################
   2197.2 |
   2199.9 |
   2202.5 |
   2205.2 |
   2207.8 |########################################
   2210.5 |
   2213.2 |
   2215.8 |
   2218.5 |
   2221.1 |####################
   2223.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=4504.5% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=4193.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_cseeqsat**: bridge=4082.4% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=3860.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_eqsat**: bridge=4072.4% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=3893.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=3906.9% of algo (FFI overhead may distort results)

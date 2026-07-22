# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_all dominates: 10% faster than the next best (carrier_opt_wideselect_fold)

carrier_opt_wideselect_all (1.86 us) leads carrier_opt_wideselect_fold (2.06 us) by 10%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_wideselect_eqsat shows alternating (throttle bounce) (autocorr -0.51)

carrier_opt_wideselect_eqsat's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_opt_wideselect_cseeqsat's edge over baseline is significant but tiny (2 ns, 0.08%)

carrier_opt_wideselect_cseeqsat differs from baseline carrier_opt_wideselect_none by 2 ns (0.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_wideselect_all** at 1862.5 ns median (-12.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.15x (fastest 1862.5 ns, slowest 2139.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 4367ns | 4363ns | 4318ns | 4351ns | 4415ns | -5.45% |
| carrier_opt_wideselect_cse | 4410ns | 4556ns | 3997ns | 4434ns | 4582ns | -4.51% |
| carrier_opt_wideselect_cseeqsat | 4702ns | 4640ns | 4610ns | 4639ns | 4844ns | +1.81% |
| carrier_opt_wideselect_dce | 4696ns | 4646ns | 4589ns | 4637ns | 4837ns | +1.67% |
| carrier_opt_wideselect_eqsat | 4643ns | 4624ns | 4582ns | 4610ns | 4724ns | +0.54% |
| carrier_opt_wideselect_fold | 4576ns | 4597ns | 4499ns | 4570ns | 4624ns | -0.92% |
| carrier_opt_wideselect_none | 4618ns | 4615ns | 4572ns | 4604ns | 4664ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 1871ns | 1858ns | 1891ns | -12.45% | 0.034 |
| carrier_opt_wideselect_cse | 2016ns | 1834ns | 2099ns | -5.66% | 0.032 |
| carrier_opt_wideselect_cseeqsat | 2189ns | 2116ns | 2302ns | +2.45% | 0.029 |
| carrier_opt_wideselect_dce | 2133ns | 2118ns | 2153ns | -0.15% | 0.030 |
| carrier_opt_wideselect_eqsat | 2131ns | 2115ns | 2148ns | -0.25% | 0.030 |
| carrier_opt_wideselect_fold | 2056ns | 2043ns | 2069ns | -3.78% | 0.031 |
| carrier_opt_wideselect_none | 2137ns | 2109ns | 2159ns | base | 0.030 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_opt_wideselect_cse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.034 | 98.5% |
| carrier_opt_wideselect_cse | 0.031 | 88.5% |
| carrier_opt_wideselect_cseeqsat | 0.030 | 85.7% |
| carrier_opt_wideselect_dce | 0.030 | 86.3% |
| carrier_opt_wideselect_eqsat | 0.030 | 86.2% |
| carrier_opt_wideselect_fold | 0.031 | 89.2% |
| carrier_opt_wideselect_none | 0.030 | 85.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 4367ns | 4367ns | -5.45% |
| carrier_opt_wideselect_cse | 4410ns | 4410ns | -4.51% |
| carrier_opt_wideselect_cseeqsat | 4702ns | 4702ns | +1.81% |
| carrier_opt_wideselect_dce | 4696ns | 4696ns | +1.67% |
| carrier_opt_wideselect_eqsat | 4643ns | 4643ns | +0.54% |
| carrier_opt_wideselect_fold | 4576ns | 4576ns | -0.92% |
| carrier_opt_wideselect_none | 4618ns | 4618ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 2138ns | base | --- | [2113, 2159] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 1862ns | -267.1ns (-12.5%) | [-298, -233]ns | [1859, 1891] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 2073ns | -78.2ns (-3.7%) | [-238, -47]ns | [1875, 2099] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_cseeqsat | 2139ns | no significant difference | [-18, +173]ns | [2126, 2302] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_dce | 2126ns | no significant difference | [-28, +30]ns | [2120, 2153] | no | 0.8250 | 0.6875 | 0 |
| carrier_opt_wideselect_eqsat | 2129ns | no significant difference | [-37, +33]ns | [2117, 2148] | no | 0.8250 | 0.6875 | 0 |
| carrier_opt_wideselect_fold | 2056ns | -88.5ns (-4.1%) | [-109, -45]ns | [2043, 2069] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_cse | carrier_opt_wideselect_cseeqsat | carrier_opt_wideselect_dce | carrier_opt_wideselect_eqsat | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|---|
| 1 | 2117ns | -12.2% | -13.3% | -0.0% | +0.3% | +1.0% | -2.5% |
| 2 | 2138ns | -12.9% | -2.9% | -0.1% | -0.4% | -0.9% | -4.4% |
| 3 | 2138ns | -12.0% | -1.4% | +0.2% | -0.7% | -1.1% | -4.4% |
| 4 | 2109ns | -9.9% | -9.2% | +1.8% | +2.5% | +2.2% | -1.8% |
| 5 | 2170ns | -14.3% | -3.7% | -1.6% | -1.1% | -2.3% | -5.6% |
| 6 | 2148ns | -13.2% | -3.6% | +14.4% | -1.4% | -0.3% | -3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | 0.020 | ok |
| carrier_opt_wideselect_cse | -0.280 | moderate- |
| carrier_opt_wideselect_cseeqsat | -0.044 | ok |
| carrier_opt_wideselect_dce | -0.012 | ok |
| carrier_opt_wideselect_eqsat | -0.512 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_fold | -0.441 | moderate- |
| carrier_opt_wideselect_none | -0.245 | moderate- |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cseeqsat**: won 1/6, lost 3/6
- **carrier_opt_wideselect_dce**: won 4/6, lost 2/6
- **carrier_opt_wideselect_eqsat**: won 4/6, lost 2/6
- **carrier_opt_wideselect_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 86192.4ns | 1870.6ns | 4607.7% | HIGH |
| carrier_opt_wideselect_cse | 86191.1ns | 2015.6ns | 4276.2% | HIGH |
| carrier_opt_wideselect_cseeqsat | 88550.9ns | 2189.0ns | 4045.2% | HIGH |
| carrier_opt_wideselect_dce | 86494.0ns | 2133.3ns | 4054.4% | HIGH |
| carrier_opt_wideselect_eqsat | 86459.1ns | 2131.3ns | 4056.7% | HIGH |
| carrier_opt_wideselect_fold | 85983.8ns | 2055.9ns | 4182.3% | HIGH |
| carrier_opt_wideselect_none | 86315.6ns | 2136.6ns | 4039.9% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 1858.3-1890.6 ns)
   1858.3 |########################################
   1859.9 |####################
   1861.5 |
   1863.1 |####################
   1864.8 |
   1866.4 |
   1868.0 |
   1869.6 |
   1871.2 |
   1872.8 |
   1874.4 |
   1876.1 |
   1877.7 |
   1879.3 |####################
   1880.9 |
   1882.5 |
   1884.1 |
   1885.8 |
   1887.4 |
   1889.0 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 1834.2-2098.9 ns)
   1834.2 |########################################
   1847.4 |
   1860.7 |
   1873.9 |
   1887.2 |
   1900.4 |
   1913.6 |########################################
   1926.9 |
   1940.1 |
   1953.3 |
   1966.6 |
   1979.8 |
   1993.0 |
   2006.3 |
   2019.5 |
   2032.8 |
   2046.0 |
   2059.2 |########################################
   2072.5 |########################################
   2085.7 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_cseeqsat (n=6, range 2116.2-2302.1 ns)
   2116.2 |#############
   2125.5 |
   2134.8 |########################################
   2144.1 |#############
   2153.4 |
   2162.7 |
   2172.0 |
   2181.3 |
   2190.6 |
   2199.9 |
   2209.1 |
   2218.4 |
   2227.7 |
   2237.0 |
   2246.3 |
   2255.6 |
   2264.9 |
   2274.2 |
   2283.5 |
   2292.8 |
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 2117.5-2153.3 ns)
   2117.5 |####################
   2119.3 |
   2121.1 |
   2122.9 |########################################
   2124.7 |
   2126.5 |
   2128.3 |####################
   2130.0 |
   2131.8 |
   2133.6 |
   2135.4 |
   2137.2 |
   2139.0 |
   2140.8 |
   2142.6 |
   2144.4 |####################
   2146.2 |
   2148.0 |
   2149.8 |
   2151.6 |
  (0 below, 1 above range)

carrier_opt_wideselect_eqsat (n=6, range 2114.6-2148.3 ns)
   2114.6 |########################################
   2116.3 |
   2118.0 |########################################
   2119.7 |########################################
   2121.3 |
   2123.0 |
   2124.7 |
   2126.4 |
   2128.1 |
   2129.8 |
   2131.5 |
   2133.2 |
   2134.8 |
   2136.5 |########################################
   2138.2 |
   2139.9 |
   2141.6 |########################################
   2143.3 |
   2145.0 |
   2146.7 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 2042.9-2068.9 ns)
   2042.9 |########################################
   2044.2 |
   2045.5 |
   2046.8 |####################
   2048.1 |
   2049.4 |
   2050.7 |
   2052.0 |
   2053.3 |
   2054.6 |
   2055.9 |
   2057.2 |
   2058.5 |
   2059.8 |
   2061.1 |
   2062.4 |
   2063.7 |####################
   2065.0 |####################
   2066.3 |
   2067.6 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 2109.2-2158.9 ns)
   2109.2 |####################
   2111.7 |
   2114.2 |
   2116.7 |####################
   2119.1 |
   2121.6 |
   2124.1 |
   2126.6 |
   2129.1 |
   2131.6 |
   2134.1 |
   2136.6 |########################################
   2139.0 |
   2141.5 |
   2144.0 |
   2146.5 |####################
   2149.0 |
   2151.5 |
   2154.0 |
   2156.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=4626.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=4153.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cseeqsat**: bridge=4053.9% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=4060.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_eqsat**: bridge=4065.6% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=4180.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=4034.4% of algo (FFI overhead may distort results)

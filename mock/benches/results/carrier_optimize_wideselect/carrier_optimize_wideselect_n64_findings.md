# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_cse shows alternating (throttle bounce) (autocorr -0.57)

carrier_opt_wideselect_cse's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_opt_wideselect_cse's edge over baseline is significant but tiny (-36 ns, 1.78%)

carrier_opt_wideselect_cse differs from baseline carrier_opt_wideselect_none by -36 ns (1.78%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_wideselect_all** at 1828.8 ns median (-9.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.14x (fastest 1828.8 ns, slowest 2088.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 4211ns | 4302ns | 3865ns | 4164ns | 4455ns | -3.94% |
| carrier_opt_wideselect_canon | 4349ns | 4391ns | 3991ns | 4268ns | 4650ns | -0.80% |
| carrier_opt_wideselect_cse | 4372ns | 4392ns | 4020ns | 4282ns | 4684ns | -0.28% |
| carrier_opt_wideselect_dce | 4410ns | 4481ns | 4047ns | 4364ns | 4662ns | +0.60% |
| carrier_opt_wideselect_fold | 4295ns | 4336ns | 3985ns | 4221ns | 4562ns | -2.03% |
| carrier_opt_wideselect_none | 4384ns | 4397ns | 4120ns | 4309ns | 4629ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 1790ns | 1671ns | 1866ns | -11.08% | 0.036 |
| carrier_opt_wideselect_canon | 1966ns | 1836ns | 2095ns | -2.35% | 0.033 |
| carrier_opt_wideselect_cse | 1968ns | 1836ns | 2101ns | -2.24% | 0.033 |
| carrier_opt_wideselect_dce | 2041ns | 1888ns | 2134ns | +1.40% | 0.031 |
| carrier_opt_wideselect_fold | 1950ns | 1835ns | 2066ns | -3.15% | 0.033 |
| carrier_opt_wideselect_none | 2013ns | 1877ns | 2144ns | base | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 262265 | 1485352 | 0.177 | 0.98× |
| carrier_opt_wideselect_canon | 264996 | 1551709 | 0.171 | 0.99× |
| carrier_opt_wideselect_cse | 265927 | 1554995 | 0.171 | 0.99× |
| carrier_opt_wideselect_dce | 265331 | 1534689 | 0.173 | 0.99× |
| carrier_opt_wideselect_fold | 265735 | 1562970 | 0.170 | 0.99× |
| carrier_opt_wideselect_none | 268337 | 1557281 | 0.172 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_opt_wideselect_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.035 | 91.4% |
| carrier_opt_wideselect_canon | 0.033 | 84.9% |
| carrier_opt_wideselect_cse | 0.033 | 84.9% |
| carrier_opt_wideselect_dce | 0.031 | 80.0% |
| carrier_opt_wideselect_fold | 0.033 | 85.8% |
| carrier_opt_wideselect_none | 0.032 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 4211ns | 4211ns | -3.94% |
| carrier_opt_wideselect_canon | 4349ns | 4349ns | -0.80% |
| carrier_opt_wideselect_cse | 4372ns | 4372ns | -0.28% |
| carrier_opt_wideselect_dce | 4410ns | 4410ns | +0.60% |
| carrier_opt_wideselect_fold | 4295ns | 4295ns | -2.03% |
| carrier_opt_wideselect_none | 4384ns | 4384ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 2016ns | base | --- | [1880, 2144] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 1829ns | -236.5ns (-11.7%) | [-301, -132]ns | [1675, 1866] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_opt_wideselect_canon | 1967ns | no significant difference | [-109, +15]ns | [1836, 2095] | no | 0.2734 | 0.2188 | 0 |
| carrier_opt_wideselect_cse | 1967ns | -35.9ns (-1.8%) | [-99, -0]ns | [1837, 2101] | YES (adj: no) | 0.2734 | 0.2188 | 0 |
| carrier_opt_wideselect_dce | 2088ns | no significant difference | [-20, +94]ns | [1902, 2134] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_wideselect_fold | 1948ns | -63.8ns (-3.2%) | [-84, -43]ns | [1836, 2066] | YES (adj: no) | 0.0781 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_canon | carrier_opt_wideselect_cse | carrier_opt_wideselect_dce | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|
| 1 | 1877ns | -3.2% | +2.6% | -2.1% | +2.0% | -2.2% |
| 2 | 2130ns | -13.5% | -2.3% | -1.2% | -0.2% | -3.0% |
| 3 | 1903ns | -12.2% | -3.5% | -3.5% | +7.9% | -3.6% |
| 4 | 2158ns | -14.5% | -7.0% | -6.1% | -1.6% | -4.7% |
| 5 | 2129ns | -11.3% | -0.9% | -1.6% | +0.7% | -3.0% |
| 6 | 1883ns | -10.8% | -2.5% | +1.3% | +0.3% | -2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.397 | moderate- |
| carrier_opt_wideselect_canon | -0.530 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_cse | -0.574 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_dce | -0.250 | moderate- |
| carrier_opt_wideselect_fold | -0.511 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_none | -0.450 | moderate- |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_canon**: won 5/6, lost 1/6
- **carrier_opt_wideselect_cse**: won 5/6, lost 1/6
- **carrier_opt_wideselect_dce**: won 2/6, lost 4/6
- **carrier_opt_wideselect_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 85841.5ns | 1790.1ns | 4795.3% | HIGH |
| carrier_opt_wideselect_canon | 85586.6ns | 1965.9ns | 4353.6% | HIGH |
| carrier_opt_wideselect_cse | 85865.1ns | 1968.1ns | 4362.8% | HIGH |
| carrier_opt_wideselect_dce | 86249.9ns | 2041.4ns | 4225.1% | HIGH |
| carrier_opt_wideselect_fold | 85625.6ns | 1949.8ns | 4391.6% | HIGH |
| carrier_opt_wideselect_none | 86177.4ns | 2013.3ns | 4280.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 1670.8-1866.5 ns)
   1670.8 |########################################
   1680.6 |
   1690.4 |
   1700.1 |
   1709.9 |
   1719.7 |
   1729.5 |
   1739.3 |
   1749.1 |
   1758.8 |
   1768.6 |
   1778.4 |
   1788.2 |
   1798.0 |
   1807.8 |####################
   1817.5 |
   1827.3 |
   1837.1 |########################################
   1846.9 |
   1856.7 |
  (0 below, 1 above range)

carrier_opt_wideselect_canon (n=6, range 1835.8-2094.8 ns)
   1835.8 |########################################
   1848.8 |
   1861.7 |
   1874.7 |
   1887.6 |
   1900.5 |
   1913.5 |####################
   1926.5 |
   1939.4 |
   1952.4 |
   1965.3 |
   1978.2 |
   1991.2 |
   2004.2 |####################
   2017.1 |
   2030.1 |
   2043.0 |
   2056.0 |
   2068.9 |####################
   2081.9 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 1835.8-2100.6 ns)
   1835.8 |########################################
   1849.0 |
   1862.3 |
   1875.5 |
   1888.8 |
   1902.0 |####################
   1915.2 |
   1928.5 |
   1941.7 |
   1955.0 |
   1968.2 |
   1981.4 |
   1994.7 |
   2007.9 |
   2021.2 |####################
   2034.4 |
   2047.6 |
   2060.9 |
   2074.1 |
   2087.4 |####################
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 1888.3-2134.4 ns)
   1888.3 |####################
   1900.6 |
   1912.9 |####################
   1925.2 |
   1937.5 |
   1949.8 |
   1962.1 |
   1974.4 |
   1986.7 |
   1999.0 |
   2011.3 |
   2023.6 |
   2035.9 |
   2048.2 |####################
   2060.5 |
   2072.8 |
   2085.1 |
   2097.4 |
   2109.7 |
   2122.0 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 1835.0-2065.6 ns)
   1835.0 |########################################
   1846.5 |
   1858.1 |
   1869.6 |
   1881.1 |
   1892.7 |
   1904.2 |
   1915.7 |
   1927.2 |
   1938.8 |
   1950.3 |
   1961.8 |
   1973.4 |
   1984.9 |
   1996.4 |
   2007.9 |
   2019.5 |
   2031.0 |
   2042.5 |
   2054.1 |##########################
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 1876.7-2144.2 ns)
   1876.7 |########################################
   1890.1 |####################
   1903.4 |
   1916.8 |
   1930.2 |
   1943.6 |
   1956.9 |
   1970.3 |
   1983.7 |
   1997.1 |
   2010.4 |
   2023.8 |
   2037.2 |
   2050.5 |
   2063.9 |
   2077.3 |
   2090.7 |
   2104.0 |
   2117.4 |########################################
   2130.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=4685.7% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_canon**: bridge=4344.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=4358.0% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=4133.0% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=4394.6% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=4270.9% of algo (FFI overhead may distort results)

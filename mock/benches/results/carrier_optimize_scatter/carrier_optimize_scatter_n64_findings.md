# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_scatter_cse is fastest but the noisiest (CV 6.4%)

carrier_opt_scatter_cse wins on median (1.99 us) yet has the highest variance (CV 6.4%), while carrier_opt_scatter_none is the steadiest (CV 4.5%, 2.24 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_opt_scatter_all shows alternating (throttle bounce) (autocorr -0.60)

carrier_opt_scatter_all's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_opt_scatter_dce's edge over baseline is significant but tiny (-10 ns, 0.44%)

carrier_opt_scatter_dce differs from baseline carrier_opt_scatter_none by -10 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_scatter_cse** at 1988.8 ns median (-11.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.13x (fastest 1988.8 ns, slowest 2254.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 4504ns | 4683ns | 4087ns | 4485ns | 4742ns | -3.15% |
| carrier_opt_scatter_canon | 4570ns | 4717ns | 4122ns | 4572ns | 4791ns | -1.74% |
| carrier_opt_scatter_cse | 4482ns | 4494ns | 4096ns | 4371ns | 4843ns | -3.61% |
| carrier_opt_scatter_dce | 4608ns | 4698ns | 4247ns | 4573ns | 4840ns | -0.92% |
| carrier_opt_scatter_fold | 4524ns | 4669ns | 4149ns | 4506ns | 4739ns | -2.72% |
| carrier_opt_scatter_none | 4650ns | 4776ns | 4270ns | 4625ns | 4879ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 2079ns | 1918ns | 2171ns | -5.54% | 0.031 |
| carrier_opt_scatter_canon | 2148ns | 1961ns | 2249ns | -2.44% | 0.030 |
| carrier_opt_scatter_cse | 2063ns | 1953ns | 2241ns | -6.28% | 0.031 |
| carrier_opt_scatter_dce | 2199ns | 2041ns | 2301ns | -0.10% | 0.029 |
| carrier_opt_scatter_fold | 2086ns | 1945ns | 2165ns | -5.26% | 0.031 |
| carrier_opt_scatter_none | 2201ns | 2045ns | 2297ns | base | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_scatter_all | 261789 | 1432648 | 0.183 | 1.00× |
| carrier_opt_scatter_canon | 260648 | 1411454 | 0.185 | 0.99× |
| carrier_opt_scatter_cse | 271662 | 1466360 | 0.185 | 1.04× |
| carrier_opt_scatter_dce | 261187 | 1399716 | 0.187 | 1.00× |
| carrier_opt_scatter_fold | 261952 | 1461340 | 0.179 | 1.00× |
| carrier_opt_scatter_none | 261963 | 1402192 | 0.187 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_opt_scatter_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.030 | 89.3% |
| carrier_opt_scatter_canon | 0.029 | 86.0% |
| carrier_opt_scatter_cse | 0.032 | 96.4% |
| carrier_opt_scatter_dce | 0.028 | 85.1% |
| carrier_opt_scatter_fold | 0.030 | 89.6% |
| carrier_opt_scatter_none | 0.029 | 85.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 4504ns | 4504ns | -3.15% |
| carrier_opt_scatter_canon | 4570ns | 4570ns | -1.74% |
| carrier_opt_scatter_cse | 4482ns | 4482ns | -3.61% |
| carrier_opt_scatter_dce | 4608ns | 4608ns | -0.92% |
| carrier_opt_scatter_fold | 4524ns | 4524ns | -2.72% |
| carrier_opt_scatter_none | 4650ns | 4650ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 2241ns | base | --- | [2066, 2297] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 2147ns | -119.1ns (-5.3%) | [-176, -71]ns | [1920, 2171] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_canon | 2231ns | no significant difference | [-103, +8]ns | [1963, 2249] | no | 0.2734 | 0.2188 | 0 |
| carrier_opt_scatter_cse | 1989ns | -85.9ns (-3.8%) | [-281, -48]ns | [1960, 2241] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 2255ns | no significant difference | [-34, +37]ns | [2041, 2301] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_scatter_fold | 2141ns | -111.4ns (-5.0%) | [-160, -76]ns | [1951, 2165] | YES (adj: no) | 0.0521 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_canon | carrier_opt_scatter_cse | carrier_opt_scatter_dce | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|
| 1 | 2260ns | -3.5% | -0.4% | -13.6% | -1.0% | -3.5% |
| 2 | 2045ns | -6.0% | -4.1% | -3.9% | -0.1% | -4.4% |
| 3 | 2316ns | -7.9% | -3.3% | -4.0% | -0.7% | -7.7% |
| 4 | 2222ns | -2.8% | +1.1% | -11.5% | +2.2% | -3.3% |
| 5 | 2087ns | -8.1% | -5.8% | -3.7% | -2.2% | -6.8% |
| 6 | 2278ns | -5.0% | -2.4% | -0.8% | +1.1% | -5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.601 | HIGH- (thermal bounce) |
| carrier_opt_scatter_canon | -0.574 | HIGH- (thermal bounce) |
| carrier_opt_scatter_cse | -0.263 | moderate- |
| carrier_opt_scatter_dce | -0.550 | HIGH- (thermal bounce) |
| carrier_opt_scatter_fold | -0.591 | HIGH- (thermal bounce) |
| carrier_opt_scatter_none | -0.594 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_canon**: won 5/6, lost 1/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 4/6, lost 2/6
- **carrier_opt_scatter_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 85997.3ns | 2079.3ns | 4135.8% | HIGH |
| carrier_opt_scatter_canon | 85668.3ns | 2147.7ns | 3988.8% | HIGH |
| carrier_opt_scatter_cse | 85790.8ns | 2063.1ns | 4158.3% | HIGH |
| carrier_opt_scatter_dce | 85884.3ns | 2199.1ns | 3905.5% | HIGH |
| carrier_opt_scatter_fold | 86089.4ns | 2085.6ns | 4127.9% | HIGH |
| carrier_opt_scatter_none | 86166.6ns | 2201.4ns | 3914.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 1917.9-2171.4 ns)
   1917.9 |########################################
   1930.6 |
   1943.3 |
   1955.9 |
   1968.6 |
   1981.3 |
   1994.0 |
   2006.6 |
   2019.3 |
   2032.0 |
   2044.7 |
   2057.4 |
   2070.0 |
   2082.7 |
   2095.4 |
   2108.1 |
   2120.7 |
   2133.4 |####################
   2146.1 |
   2158.8 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_canon (n=6, range 1960.8-2248.8 ns)
   1960.8 |########################################
   1975.2 |
   1989.6 |
   2004.0 |
   2018.4 |
   2032.8 |
   2047.2 |
   2061.6 |
   2076.0 |
   2090.4 |
   2104.8 |
   2119.2 |
   2133.6 |
   2148.0 |
   2162.4 |
   2176.8 |
   2191.2 |
   2205.6 |
   2220.0 |####################
   2234.4 |########################################
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 1953.3-2241.0 ns)
   1953.3 |########################################
   1967.7 |
   1982.1 |
   1996.5 |#############
   2010.8 |
   2025.2 |
   2039.6 |
   2054.0 |
   2068.4 |
   2082.8 |
   2097.2 |
   2111.5 |
   2125.9 |
   2140.3 |
   2154.7 |
   2169.1 |
   2183.5 |
   2197.8 |
   2212.2 |#############
   2226.6 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 2040.8-2301.2 ns)
   2040.8 |########################################
   2053.8 |
   2066.8 |
   2079.9 |
   2092.9 |
   2105.9 |
   2118.9 |
   2132.0 |
   2145.0 |
   2158.0 |
   2171.0 |
   2184.0 |
   2197.1 |
   2210.1 |
   2223.1 |
   2236.1 |####################
   2249.2 |
   2262.2 |####################
   2275.2 |
   2288.2 |####################
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 1945.4-2165.4 ns)
   1945.4 |########################################
   1956.4 |
   1967.4 |
   1978.4 |
   1989.4 |
   2000.4 |
   2011.4 |
   2022.4 |
   2033.4 |
   2044.4 |
   2055.4 |
   2066.4 |
   2077.4 |
   2088.4 |
   2099.4 |
   2110.4 |
   2121.4 |
   2132.4 |####################
   2143.4 |########################################
   2154.4 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 2045.0-2296.8 ns)
   2045.0 |########################################
   2057.6 |
   2070.2 |
   2082.8 |########################################
   2095.4 |
   2108.0 |
   2120.6 |
   2133.1 |
   2145.7 |
   2158.3 |
   2170.9 |
   2183.5 |
   2196.1 |
   2208.7 |
   2221.3 |########################################
   2233.9 |
   2246.5 |
   2259.1 |########################################
   2271.7 |########################################
   2284.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=4009.9% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_canon**: bridge=3841.7% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=4311.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=3812.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=4021.7% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=3841.1% of algo (FFI overhead may distort results)

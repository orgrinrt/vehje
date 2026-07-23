# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec20 shows alternating (throttle bounce) (autocorr -0.59)

carrier_lay_leaf_rec20's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_lay_leaf_rec12's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_lay_leaf_rec12 are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader carrier_lay_leaf_rec12 vs stability leader carrier_lay_leaf_rec32 (+6% speed for 1.1x steadier)

carrier_lay_leaf_rec12 is fastest (1.92 us, CV 5.2%); carrier_lay_leaf_rec32 gives up 5.6% median for 1.1x lower variance (CV 4.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_lay_leaf_rec12's edge over baseline is significant but tiny (-37 ns, 1.78%)

carrier_lay_leaf_rec12 differs from baseline carrier_lay_leaf_rec24 by -37 ns (1.78%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_leaf_rec12** at 1916.2 ns median (-8.3% vs baseline)
- Spread: 1.11x (fastest 1916.2 ns, slowest 2128.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 4220ns | 4111ns | 4037ns | 4092ns | 4504ns | -3.22% |
| carrier_lay_leaf_rec16 | 4377ns | 4481ns | 4082ns | 4348ns | 4569ns | +0.38% |
| carrier_lay_leaf_rec20 | 4418ns | 4571ns | 4053ns | 4416ns | 4604ns | +1.33% |
| carrier_lay_leaf_rec24 | 4361ns | 4471ns | 4041ns | 4337ns | 4555ns | base |
| carrier_lay_leaf_rec32 | 4347ns | 4335ns | 4093ns | 4279ns | 4574ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1968ns | 1881ns | 2105ns | -3.27% | 0.033 |
| carrier_lay_leaf_rec16 | 2050ns | 1906ns | 2136ns | +0.72% | 0.031 |
| carrier_lay_leaf_rec20 | 2058ns | 1888ns | 2151ns | +1.14% | 0.031 |
| carrier_lay_leaf_rec24 | 2035ns | 1885ns | 2129ns | base | 0.031 |
| carrier_lay_leaf_rec32 | 2031ns | 1916ns | 2142ns | -0.18% | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 273159 | 1557143 | 0.175 | 1.03× |
| carrier_lay_leaf_rec16 | 264842 | 1506247 | 0.176 | 1.00× |
| carrier_lay_leaf_rec20 | 262835 | 1496584 | 0.176 | 0.99× |
| carrier_lay_leaf_rec24 | 265032 | 1512196 | 0.175 | 1.00× |
| carrier_lay_leaf_rec32 | 266251 | 1517261 | 0.175 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_lay_leaf_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.033 | 98.2% |
| carrier_lay_leaf_rec16 | 0.030 | 89.5% |
| carrier_lay_leaf_rec20 | 0.030 | 88.4% |
| carrier_lay_leaf_rec24 | 0.031 | 90.0% |
| carrier_lay_leaf_rec32 | 0.032 | 93.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 4220ns | 4220ns | -3.22% |
| carrier_lay_leaf_rec16 | 4377ns | 4377ns | +0.38% |
| carrier_lay_leaf_rec20 | 4418ns | 4418ns | +1.33% |
| carrier_lay_leaf_rec24 | 4361ns | 4361ns | base |
| carrier_lay_leaf_rec32 | 4347ns | 4347ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 2090ns | base | --- | [1886, 2129] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 1916ns | no significant difference | [-191, +28]ns | [1884, 2105] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| carrier_lay_leaf_rec16 | 2102ns | no significant difference | [-3, +29]ns | [1911, 2136] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_leaf_rec20 | 2128ns | no significant difference | [-2, +53]ns | [1895, 2151] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_leaf_rec32 | 2023ns | no significant difference | [-99, +55]ns | [1929, 2142] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 2073ns | -9.2% | +0.6% | +3.4% | +2.6% |
| 2 | 1886ns | +0.0% | +1.1% | +0.1% | +1.6% |
| 3 | 2138ns | -3.5% | -0.8% | +1.0% | -2.3% |
| 4 | 2120ns | +1.2% | +1.3% | -0.3% | +1.8% |
| 5 | 1885ns | +1.6% | +1.6% | +0.9% | +3.0% |
| 6 | 2107ns | -9.0% | +0.8% | +1.7% | -7.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.157 | ok |
| carrier_lay_leaf_rec16 | -0.524 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec20 | -0.592 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec24 | -0.518 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec32 | -0.276 | moderate- |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 3/6, lost 2/6
- **carrier_lay_leaf_rec16**: won 1/6, lost 5/6
- **carrier_lay_leaf_rec20**: won 1/6, lost 4/6
- **carrier_lay_leaf_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 86344.9ns | 1968.3ns | 4386.7% | HIGH |
| carrier_lay_leaf_rec16 | 86635.6ns | 2049.7ns | 4226.8% | HIGH |
| carrier_lay_leaf_rec20 | 86477.1ns | 2058.1ns | 4201.9% | HIGH |
| carrier_lay_leaf_rec24 | 86483.8ns | 2034.9ns | 4250.0% | HIGH |
| carrier_lay_leaf_rec32 | 86573.2ns | 2031.2ns | 4262.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 1881.2-2105.0 ns)
   1881.2 |########################################
   1892.4 |
   1903.6 |
   1914.8 |########################################
   1926.0 |
   1937.2 |
   1948.3 |
   1959.5 |
   1970.7 |
   1981.9 |
   1993.1 |
   2004.3 |
   2015.5 |
   2026.7 |
   2037.9 |
   2049.1 |
   2060.2 |####################
   2071.4 |
   2082.6 |
   2093.8 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 1906.2-2135.8 ns)
   1906.2 |########################################
   1917.7 |
   1929.2 |
   1940.6 |
   1952.1 |
   1963.6 |
   1975.1 |
   1986.6 |
   1998.1 |
   2009.5 |
   2021.0 |
   2032.5 |
   2044.0 |
   2055.5 |
   2067.0 |
   2078.4 |####################
   2089.9 |
   2101.4 |
   2112.9 |####################
   2124.4 |####################
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 1887.5-2151.2 ns)
   1887.5 |####################
   1900.7 |####################
   1913.9 |
   1927.1 |
   1940.2 |
   1953.4 |
   1966.6 |
   1979.8 |
   1993.0 |
   2006.2 |
   2019.4 |
   2032.6 |
   2045.8 |
   2058.9 |
   2072.1 |
   2085.3 |
   2098.5 |
   2111.7 |####################
   2124.9 |
   2138.1 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 1885.4-2128.9 ns)
   1885.4 |########################################
   1897.6 |
   1909.8 |
   1921.9 |
   1934.1 |
   1946.3 |
   1958.5 |
   1970.6 |
   1982.8 |
   1995.0 |
   2007.2 |
   2019.4 |
   2031.5 |
   2043.7 |
   2055.9 |
   2068.1 |####################
   2080.2 |
   2092.4 |
   2104.6 |####################
   2116.8 |####################
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 1915.8-2141.9 ns)
   1915.8 |########################################
   1927.1 |
   1938.4 |########################################
   1949.7 |########################################
   1961.0 |
   1972.3 |
   1983.6 |
   1994.9 |
   2006.2 |
   2017.5 |
   2028.8 |
   2040.1 |
   2051.4 |
   2062.7 |
   2074.0 |
   2085.3 |########################################
   2096.6 |
   2107.9 |
   2119.2 |########################################
   2130.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=4510.0% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=4123.9% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=4066.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=4139.1% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=4282.7% of algo (FFI overhead may distort results)

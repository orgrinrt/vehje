# Entropy x locality surface: op_correlation {0,500,900} x locality_window {4,64,unbounded}, fixed predecoded switch dispatch

9 variants, 6 samples per variant.
Baseline: **carrier_ent_c0_w64**

## Highlights

Baseline for all deltas below: **carrier_ent_c0_w64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ent_c900_wmax beats baseline by 25% (significant)

carrier_ent_c900_wmax is -536 ns (25%) faster than baseline carrier_ent_c0_w64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (carrier_ent_c900_wmax, carrier_ent_c900_w64) are a dead heat (<1%)

carrier_ent_c900_wmax (1.58 us) and carrier_ent_c900_w64 (1.59 us) differ by 0.12%, inside the noise, even though the wider field spreads 34.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_ent_c0_wmax shows alternating (throttle bounce) (autocorr -0.84)

carrier_ent_c0_wmax's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_ent_c0_wmax's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_ent_c0_wmax are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader carrier_ent_c900_wmax vs stability leader carrier_ent_c900_w64 (+0% speed for 1.3x steadier)

carrier_ent_c900_wmax is fastest (1.58 us, CV 5.2%); carrier_ent_c900_w64 gives up 0.1% median for 1.3x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_ent_c0_w4's edge over baseline is significant but tiny (14 ns, 0.68%)

carrier_ent_c0_w4 differs from baseline carrier_ent_c0_w64 by 14 ns (0.68%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_ent_c900_wmax** at 1583.7 ns median (-24.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 1.35x (fastest 1583.7 ns, slowest 2135.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 4499ns | 4571ns | 4026ns | 4539ns | 4676ns | +1.65% |
| carrier_ent_c0_w64 | 4426ns | 4521ns | 4025ns | 4455ns | 4583ns | base |
| carrier_ent_c0_wmax | 4325ns | 4301ns | 4031ns | 4212ns | 4642ns | -2.28% |
| carrier_ent_c500_w4 | 4129ns | 4106ns | 3863ns | 4027ns | 4417ns | -6.70% |
| carrier_ent_c500_w64 | 4239ns | 4379ns | 3875ns | 4212ns | 4460ns | -4.24% |
| carrier_ent_c500_wmax | 4221ns | 4382ns | 3858ns | 4209ns | 4420ns | -4.64% |
| carrier_ent_c900_w4 | 4139ns | 4132ns | 3868ns | 4045ns | 4417ns | -6.48% |
| carrier_ent_c900_w64 | 3924ns | 3992ns | 3559ns | 3987ns | 4011ns | -11.35% |
| carrier_ent_c900_wmax | 3869ns | 3987ns | 3570ns | 3851ns | 4047ns | -12.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 2101ns | 1880ns | 2184ns | +1.82% | 0.030 |
| carrier_ent_c0_w64 | 2064ns | 1876ns | 2139ns | base | 0.031 |
| carrier_ent_c0_wmax | 2014ns | 1875ns | 2166ns | -2.39% | 0.032 |
| carrier_ent_c500_w4 | 1830ns | 1692ns | 1970ns | -11.32% | 0.035 |
| carrier_ent_c500_w64 | 1882ns | 1710ns | 1980ns | -8.82% | 0.034 |
| carrier_ent_c500_wmax | 1867ns | 1710ns | 1959ns | -9.52% | 0.034 |
| carrier_ent_c900_w4 | 1830ns | 1713ns | 1946ns | -11.31% | 0.035 |
| carrier_ent_c900_w64 | 1556ns | 1417ns | 1593ns | -24.60% | 0.041 |
| carrier_ent_c900_wmax | 1533ns | 1418ns | 1598ns | -25.71% | 0.042 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 254726 | 861906 | 0.296 | 0.99× |
| carrier_ent_c0_w64 | 256825 | 875999 | 0.293 | 1.00× |
| carrier_ent_c0_wmax | 263566 | 899334 | 0.293 | 1.03× |
| carrier_ent_c500_w4 | 265350 | 1004781 | 0.264 | 1.03× |
| carrier_ent_c500_w64 | 257407 | 972653 | 0.265 | 1.00× |
| carrier_ent_c500_wmax | 256618 | 973394 | 0.264 | 1.00× |
| carrier_ent_c900_w4 | 273202 | 1017648 | 0.268 | 1.06× |
| carrier_ent_c900_w64 | 238804 | 1073699 | 0.222 | 0.93× |
| carrier_ent_c900_wmax | 242951 | 1092842 | 0.222 | 0.95× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_ent_c900_w64; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ent_c0_w4 | 0.030 | 66.4% |
| carrier_ent_c0_w64 | 0.030 | 67.2% |
| carrier_ent_c0_wmax | 0.032 | 70.8% |
| carrier_ent_c500_w4 | 0.035 | 77.9% |
| carrier_ent_c500_w64 | 0.033 | 72.8% |
| carrier_ent_c500_wmax | 0.033 | 73.3% |
| carrier_ent_c900_w4 | 0.035 | 77.4% |
| carrier_ent_c900_w64 | 0.040 | 89.4% |
| carrier_ent_c900_wmax | 0.040 | 89.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ent_c0_w4 | 4499ns | 4499ns | +1.65% |
| carrier_ent_c0_w64 | 4426ns | 4426ns | base |
| carrier_ent_c0_wmax | 4325ns | 4325ns | -2.28% |
| carrier_ent_c500_w4 | 4129ns | 4129ns | -6.70% |
| carrier_ent_c500_w64 | 4239ns | 4239ns | -4.24% |
| carrier_ent_c500_wmax | 4221ns | 4221ns | -4.64% |
| carrier_ent_c900_w4 | 4139ns | 4139ns | -6.48% |
| carrier_ent_c900_w64 | 3924ns | 3924ns | -11.35% |
| carrier_ent_c900_wmax | 3869ns | 3869ns | -12.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ent_c0_w64 | 2110ns | base | --- | [1942, 2139] | --- | --- | --- | --- |
| carrier_ent_c0_w4 | 2136ns | no significant difference | [-16, +115]ns | [1985, 2184] | no | 0.7857 | 0.6875 | 0 |
| carrier_ent_c0_wmax | 2001ns | no significant difference | [-192, +36]ns | [1876, 2166] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| carrier_ent_c500_w4 | 1820ns | -190.0ns (-9.0%) | [-367, -143]ns | [1701, 1970] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c500_w64 | 1947ns | -166.3ns (-7.9%) | [-231, -149]ns | [1718, 1980] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c500_wmax | 1932ns | -187.1ns (-8.9%) | [-245, -157]ns | [1711, 1959] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c900_w4 | 1830ns | -188.0ns (-8.9%) | [-351, -161]ns | [1715, 1946] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c900_w64 | 1586ns | -519.4ns (-24.6%) | [-562, -442]ns | [1489, 1593] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c900_wmax | 1584ns | -535.8ns (-25.4%) | [-577, -479]ns | [1418, 1598] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ent_c0_w64 | carrier_ent_c0_w4 | carrier_ent_c0_wmax | carrier_ent_c500_w4 | carrier_ent_c500_w64 | carrier_ent_c500_wmax | carrier_ent_c900_w4 | carrier_ent_c900_w64 | carrier_ent_c900_wmax |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2129ns | +3.6% | -11.8% | -20.5% | -8.5% | -9.1% | -19.2% | -25.2% | -25.7% |
| 2 | 2149ns | -0.6% | +2.4% | -9.3% | -7.7% | -9.0% | -9.7% | -27.4% | -26.2% |
| 3 | 2008ns | +7.6% | -6.6% | -14.9% | -14.1% | -14.7% | -14.7% | -21.1% | -29.4% |
| 4 | 2110ns | -1.0% | +1.0% | -5.7% | -6.3% | -7.0% | -7.9% | -24.5% | -24.8% |
| 5 | 1876ns | +0.2% | +0.0% | -8.9% | -8.9% | -8.9% | -8.5% | -24.5% | -24.4% |
| 6 | 2110ns | +1.2% | +0.7% | -8.5% | -7.8% | -8.5% | -7.7% | -24.7% | -23.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ent_c0_w4 | 0.005 | ok |
| carrier_ent_c0_w64 | -0.353 | moderate- |
| carrier_ent_c0_wmax | -0.842 | HIGH- (thermal bounce) |
| carrier_ent_c500_w4 | -0.832 | HIGH- (thermal bounce) |
| carrier_ent_c500_w64 | -0.629 | HIGH- (thermal bounce) |
| carrier_ent_c500_wmax | -0.639 | HIGH- (thermal bounce) |
| carrier_ent_c900_w4 | -0.833 | HIGH- (thermal bounce) |
| carrier_ent_c900_w64 | -0.347 | moderate- |
| carrier_ent_c900_wmax | -0.611 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_ent_c0_w4**: won 2/6, lost 4/6
- **carrier_ent_c0_wmax**: won 2/6, lost 3/6
- **carrier_ent_c500_w4**: won 6/6, lost 0/6
- **carrier_ent_c500_w64**: won 6/6, lost 0/6
- **carrier_ent_c500_wmax**: won 6/6, lost 0/6
- **carrier_ent_c900_w4**: won 6/6, lost 0/6
- **carrier_ent_c900_w64**: won 6/6, lost 0/6
- **carrier_ent_c900_wmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 85691.9ns | 2101.3ns | 4078.0% | HIGH |
| carrier_ent_c0_w64 | 85556.9ns | 2063.8ns | 4145.7% | HIGH |
| carrier_ent_c0_wmax | 85558.5ns | 2014.4ns | 4247.4% | HIGH |
| carrier_ent_c500_w4 | 85805.9ns | 1830.2ns | 4688.3% | HIGH |
| carrier_ent_c500_w64 | 84688.3ns | 1881.7ns | 4500.7% | HIGH |
| carrier_ent_c500_wmax | 84540.1ns | 1867.4ns | 4527.2% | HIGH |
| carrier_ent_c900_w4 | 88532.8ns | 1830.3ns | 4837.0% | HIGH |
| carrier_ent_c900_w64 | 79315.3ns | 1556.0ns | 5097.5% | HIGH |
| carrier_ent_c900_wmax | 79610.1ns | 1533.1ns | 5192.7% | HIGH |

## Distribution (algo ns)

```
carrier_ent_c0_w4 (n=6, range 1880.0-2183.5 ns)
   1880.0 |####################
   1895.2 |
   1910.3 |
   1925.5 |
   1940.7 |
   1955.9 |
   1971.0 |
   1986.2 |
   2001.4 |
   2016.6 |
   2031.8 |
   2046.9 |
   2062.1 |
   2077.3 |####################
   2092.4 |
   2107.6 |
   2122.8 |########################################
   2138.0 |
   2153.2 |####################
   2168.3 |
  (0 below, 1 above range)

carrier_ent_c0_w64 (n=6, range 1876.2-2139.0 ns)
   1876.2 |####################
   1889.3 |
   1902.5 |
   1915.6 |
   1928.8 |
   1941.9 |
   1955.0 |
   1968.2 |
   1981.3 |
   1994.5 |
   2007.6 |####################
   2020.7 |
   2033.9 |
   2047.0 |
   2060.2 |
   2073.3 |
   2086.4 |
   2099.6 |########################################
   2112.7 |
   2125.9 |####################
  (0 below, 1 above range)

carrier_ent_c0_wmax (n=6, range 1875.4-2166.0 ns)
   1875.4 |########################################
   1889.9 |
   1904.5 |
   1919.0 |
   1933.5 |
   1948.1 |
   1962.6 |
   1977.1 |
   1991.6 |
   2006.2 |
   2020.7 |
   2035.2 |
   2049.8 |
   2064.3 |
   2078.8 |
   2093.3 |
   2107.9 |
   2122.4 |##########################
   2136.9 |
   2151.5 |
  (0 below, 1 above range)

carrier_ent_c500_w4 (n=6, range 1692.5-1969.6 ns)
   1692.5 |####################
   1706.4 |########################################
   1720.2 |
   1734.1 |
   1747.9 |
   1761.8 |
   1775.6 |
   1789.5 |
   1803.3 |
   1817.2 |
   1831.0 |
   1844.9 |
   1858.8 |
   1872.6 |
   1886.5 |
   1900.3 |
   1914.2 |
   1928.0 |####################
   1941.9 |####################
   1955.7 |
  (0 below, 1 above range)

carrier_ent_c500_w64 (n=6, range 1710.0-1980.4 ns)
   1710.0 |####################
   1723.5 |####################
   1737.0 |
   1750.6 |
   1764.1 |
   1777.6 |
   1791.1 |
   1804.6 |
   1818.2 |
   1831.7 |
   1845.2 |
   1858.7 |
   1872.2 |
   1885.8 |
   1899.3 |
   1912.8 |
   1926.3 |
   1939.8 |########################################
   1953.4 |
   1966.9 |####################
  (0 below, 1 above range)

carrier_ent_c500_wmax (n=6, range 1710.0-1958.5 ns)
   1710.0 |########################################
   1722.4 |
   1734.9 |
   1747.3 |
   1759.7 |
   1772.1 |
   1784.6 |
   1797.0 |
   1809.4 |
   1821.8 |
   1834.3 |
   1846.7 |
   1859.1 |
   1871.6 |
   1884.0 |
   1896.4 |
   1908.8 |
   1921.3 |####################
   1933.7 |####################
   1946.1 |####################
  (0 below, 1 above range)

carrier_ent_c900_w4 (n=6, range 1713.3-1945.8 ns)
   1713.3 |########################################
   1724.9 |
   1736.5 |
   1748.2 |
   1759.8 |
   1771.4 |
   1783.0 |
   1794.7 |
   1806.3 |
   1817.9 |
   1829.6 |
   1841.2 |
   1852.8 |
   1864.4 |
   1876.1 |
   1887.7 |
   1899.3 |
   1910.9 |
   1922.6 |
   1934.2 |##########################
  (0 below, 1 above range)

carrier_ent_c900_w64 (n=6, range 1417.1-1593.3 ns)
   1417.1 |####################
   1425.9 |
   1434.7 |
   1443.5 |
   1452.3 |
   1461.2 |
   1470.0 |
   1478.8 |
   1487.6 |
   1496.4 |
   1505.2 |
   1514.0 |
   1522.8 |
   1531.7 |
   1540.5 |
   1549.3 |
   1558.1 |####################
   1566.9 |
   1575.7 |####################
   1584.5 |########################################
  (0 below, 1 above range)

carrier_ent_c900_wmax (n=6, range 1417.5-1597.7 ns)
   1417.5 |##########################
   1426.5 |
   1435.5 |
   1444.5 |
   1453.5 |
   1462.5 |
   1471.6 |
   1480.6 |
   1489.6 |
   1498.6 |
   1507.6 |
   1516.6 |
   1525.6 |
   1534.6 |
   1543.6 |
   1552.7 |
   1561.7 |
   1570.7 |
   1579.7 |########################################
   1588.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ent_c0_w4**: bridge=4016.0% of algo (FFI overhead may distort results)
- **carrier_ent_c0_w64**: bridge=4064.2% of algo (FFI overhead may distort results)
- **carrier_ent_c0_wmax**: bridge=4268.5% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w4**: bridge=4721.8% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w64**: bridge=4352.5% of algo (FFI overhead may distort results)
- **carrier_ent_c500_wmax**: bridge=4365.9% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w4**: bridge=4847.2% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w64**: bridge=4999.1% of algo (FFI overhead may distort results)
- **carrier_ent_c900_wmax**: bridge=5033.7% of algo (FFI overhead may distort results)

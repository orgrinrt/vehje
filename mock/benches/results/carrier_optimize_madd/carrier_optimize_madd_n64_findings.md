# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_madd_all dominates: 343% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (405 ns) leads carrier_opt_madd_fold (1.79 us) by 343%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 81% (significant)

carrier_opt_madd_all is -1.77 us (81%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_dce is an outlier: 5.5x slower than the field

carrier_opt_madd_dce (2.24 us) is 5.5x the fastest (405 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_madd_fold shows alternating (throttle bounce) (autocorr -0.56)

carrier_opt_madd_fold's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_none, carrier_opt_madd_dce} (343% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_none, carrier_opt_madd_dce} with a 343% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.5x the fastest

Fastest carrier_opt_madd_all (405 ns) to slowest carrier_opt_madd_dce (2.24 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_opt_madd_dce's edge over baseline is significant but tiny (25 ns, 1.14%)

carrier_opt_madd_dce differs from baseline carrier_opt_madd_none by 25 ns (1.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_madd_all** at 404.6 ns median (-81.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.55x (fastest 404.6 ns, slowest 2244.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 2778ns | 2841ns | 2593ns | 2772ns | 2878ns | -38.72% |
| carrier_opt_madd_canon | 4218ns | 4188ns | 3977ns | 4124ns | 4481ns | -6.95% |
| carrier_opt_madd_cse | 4394ns | 4422ns | 4053ns | 4340ns | 4646ns | -3.07% |
| carrier_opt_madd_dce | 4589ns | 4668ns | 4214ns | 4538ns | 4852ns | +1.23% |
| carrier_opt_madd_fold | 4175ns | 4192ns | 3886ns | 4098ns | 4434ns | -7.91% |
| carrier_opt_madd_none | 4533ns | 4515ns | 4254ns | 4453ns | 4794ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 398ns | 375ns | 416ns | -81.65% | 0.161 |
| carrier_opt_madd_canon | 1911ns | 1783ns | 2028ns | -11.99% | 0.033 |
| carrier_opt_madd_cse | 1978ns | 1791ns | 2127ns | -8.88% | 0.032 |
| carrier_opt_madd_dce | 2211ns | 2044ns | 2345ns | +1.86% | 0.029 |
| carrier_opt_madd_fold | 1794ns | 1679ns | 1898ns | -17.35% | 0.036 |
| carrier_opt_madd_none | 2171ns | 2052ns | 2279ns | base | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_madd_all | 249589 | 1577094 | 0.158 | 0.93× |
| carrier_opt_madd_canon | 267299 | 1406030 | 0.190 | 0.99× |
| carrier_opt_madd_cse | 263866 | 1436385 | 0.184 | 0.98× |
| carrier_opt_madd_dce | 265418 | 1370416 | 0.194 | 0.99× |
| carrier_opt_madd_fold | 263424 | 1539026 | 0.171 | 0.98× |
| carrier_opt_madd_none | 268945 | 1391904 | 0.193 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.171 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.158 | 92.7% |
| carrier_opt_madd_canon | 0.034 | 19.6% |
| carrier_opt_madd_cse | 0.032 | 18.6% |
| carrier_opt_madd_dce | 0.029 | 16.7% |
| carrier_opt_madd_fold | 0.036 | 20.9% |
| carrier_opt_madd_none | 0.029 | 17.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 2778ns | 2778ns | -38.72% |
| carrier_opt_madd_canon | 4218ns | 4218ns | -6.95% |
| carrier_opt_madd_cse | 4394ns | 4394ns | -3.07% |
| carrier_opt_madd_dce | 4589ns | 4589ns | +1.23% |
| carrier_opt_madd_fold | 4175ns | 4175ns | -7.91% |
| carrier_opt_madd_none | 4533ns | 4533ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 2171ns | base | --- | [2063, 2279] | --- | --- | --- | --- |
| carrier_opt_madd_all | 405ns | -1766.8ns (-81.4%) | [-1863, -1688]ns | [375, 416] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_madd_canon | 1910ns | -268.4ns (-12.4%) | [-335, -178]ns | [1795, 2028] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_madd_cse | 2011ns | -207.3ns (-9.5%) | [-266, -105]ns | [1797, 2127] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_madd_dce | 2244ns | no significant difference | [-18, +114]ns | [2045, 2345] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_madd_fold | 1791ns | -372.7ns (-17.2%) | [-445, -312]ns | [1694, 1898] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_canon | carrier_opt_madd_cse | carrier_opt_madd_dce | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|
| 1 | 2052ns | -81.7% | -12.0% | -12.2% | -0.4% | -18.2% |
| 2 | 2335ns | -82.2% | -13.3% | -8.0% | +0.2% | -19.5% |
| 3 | 2222ns | -81.3% | -8.6% | -5.3% | +2.0% | -16.7% |
| 4 | 2074ns | -81.9% | -14.0% | -13.6% | -1.3% | -17.6% |
| 5 | 2175ns | -81.1% | -7.5% | -10.5% | +8.0% | -11.9% |
| 6 | 2168ns | -81.7% | -16.6% | -4.3% | +2.5% | -20.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.448 | moderate- |
| carrier_opt_madd_canon | -0.490 | moderate- |
| carrier_opt_madd_cse | -0.240 | moderate- |
| carrier_opt_madd_dce | -0.477 | moderate- |
| carrier_opt_madd_fold | -0.559 | HIGH- (thermal bounce) |
| carrier_opt_madd_none | -0.311 | moderate- |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_canon**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_dce**: won 2/6, lost 4/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 82837.2ns | 398.5ns | 20789.0% | HIGH |
| carrier_opt_madd_canon | 85680.7ns | 1910.8ns | 4484.0% | HIGH |
| carrier_opt_madd_cse | 86139.1ns | 1978.3ns | 4354.3% | HIGH |
| carrier_opt_madd_dce | 86114.9ns | 2211.4ns | 3894.2% | HIGH |
| carrier_opt_madd_fold | 85458.5ns | 1794.3ns | 4762.8% | HIGH |
| carrier_opt_madd_none | 85978.5ns | 2171.1ns | 3960.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 375.0-415.6 ns)
    375.0 |########################################
    377.0 |
    379.1 |
    381.1 |
    383.1 |
    385.1 |
    387.2 |
    389.2 |
    391.2 |
    393.3 |
    395.3 |
    397.3 |####################
    399.4 |
    401.4 |
    403.4 |
    405.5 |
    407.5 |
    409.5 |
    411.5 |####################
    413.6 |####################
  (0 below, 1 above range)

carrier_opt_madd_canon (n=6, range 1783.3-2027.5 ns)
   1783.3 |########################################
   1795.5 |########################################
   1807.7 |########################################
   1819.9 |
   1832.1 |
   1844.3 |
   1856.6 |
   1868.8 |
   1881.0 |
   1893.2 |
   1905.4 |
   1917.6 |
   1929.8 |
   1942.0 |
   1954.2 |
   1966.5 |
   1978.7 |
   1990.9 |
   2003.1 |########################################
   2015.3 |########################################
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 1791.2-2126.9 ns)
   1791.2 |########################################
   1808.0 |
   1824.8 |
   1841.6 |
   1858.3 |
   1875.1 |
   1891.9 |
   1908.7 |
   1925.5 |
   1942.3 |####################
   1959.0 |
   1975.8 |
   1992.6 |
   2009.4 |
   2026.2 |
   2043.0 |
   2059.8 |####################
   2076.5 |
   2093.3 |####################
   2110.1 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 2044.2-2344.8 ns)
   2044.2 |########################################
   2059.2 |
   2074.3 |
   2089.3 |
   2104.3 |
   2119.3 |
   2134.4 |
   2149.4 |
   2164.4 |
   2179.5 |
   2194.5 |
   2209.5 |####################
   2224.6 |
   2239.6 |
   2254.6 |####################
   2269.7 |
   2284.7 |
   2299.7 |
   2314.7 |
   2329.8 |####################
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 1678.7-1898.3 ns)
   1678.7 |########################################
   1689.7 |
   1700.7 |########################################
   1711.6 |
   1722.6 |########################################
   1733.6 |
   1744.6 |
   1755.6 |
   1766.5 |
   1777.5 |
   1788.5 |
   1799.5 |
   1810.5 |
   1821.4 |
   1832.4 |
   1843.4 |########################################
   1854.4 |
   1865.4 |
   1876.3 |########################################
   1887.3 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 2052.5-2278.6 ns)
   2052.5 |####################
   2063.8 |####################
   2075.1 |
   2086.4 |
   2097.7 |
   2109.0 |
   2120.3 |
   2131.6 |
   2142.9 |
   2154.2 |
   2165.5 |########################################
   2176.8 |
   2188.1 |
   2199.4 |
   2210.7 |####################
   2222.0 |
   2233.3 |
   2244.6 |
   2255.9 |
   2267.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=20493.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_canon**: bridge=4491.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=4288.0% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=3839.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=4768.0% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=3958.4% of algo (FFI overhead may distort results)

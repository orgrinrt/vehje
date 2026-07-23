# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 276% faster than the next best (carrier_opt_tight_canon)

carrier_opt_tight_all (451 ns) leads carrier_opt_tight_canon (1.69 us) by 276%, a clear separation rather than a photo finish. CV 8.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 77% (significant)

carrier_opt_tight_all is -1.34 us (77%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_dce is an outlier: 4.2x slower than the field

carrier_opt_tight_dce (1.89 us) is 4.2x the fastest (451 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_all is fastest but the noisiest (CV 8.1%)

carrier_opt_tight_all wins on median (451 ns) yet has the highest variance (CV 8.1%), while carrier_opt_tight_dce is the steadiest (CV 0.7%, 1.89 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_opt_tight_dce shows alternating (throttle bounce) (autocorr -0.72)

carrier_opt_tight_dce's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_canon, carrier_opt_tight_none, carrier_opt_tight_cse, carrier_opt_tight_fold, carrier_opt_tight_dce} (276% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_canon, carrier_opt_tight_none, carrier_opt_tight_cse, carrier_opt_tight_fold, carrier_opt_tight_dce} with a 276% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest carrier_opt_tight_all (451 ns) to slowest carrier_opt_tight_dce (1.89 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_opt_tight_cse's edge over baseline is significant but tiny (-33 ns, 1.88%)

carrier_opt_tight_cse differs from baseline carrier_opt_tight_none by -33 ns (1.88%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_tight_all** at 450.8 ns median (-74.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.18x (fastest 450.8 ns, slowest 1885.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 2826ns | 2929ns | 2585ns | 2818ns | 2960ns | -31.83% |
| carrier_opt_tight_canon | 4065ns | 4079ns | 3759ns | 3992ns | 4328ns | -1.95% |
| carrier_opt_tight_cse | 4140ns | 4170ns | 3818ns | 4078ns | 4395ns | -0.14% |
| carrier_opt_tight_dce | 4380ns | 4371ns | 4324ns | 4359ns | 4439ns | +5.64% |
| carrier_opt_tight_fold | 4303ns | 4350ns | 3877ns | 4329ns | 4476ns | +3.78% |
| carrier_opt_tight_none | 4146ns | 3971ns | 3866ns | 3948ns | 4582ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 429ns | 377ns | 459ns | -76.10% | 0.149 |
| carrier_opt_tight_canon | 1701ns | 1585ns | 1821ns | -5.24% | 0.038 |
| carrier_opt_tight_cse | 1794ns | 1645ns | 1906ns | -0.11% | 0.036 |
| carrier_opt_tight_dce | 1886ns | 1866ns | 1902ns | +5.06% | 0.034 |
| carrier_opt_tight_fold | 1847ns | 1683ns | 1899ns | +2.88% | 0.035 |
| carrier_opt_tight_none | 1796ns | 1711ns | 1932ns | base | 0.036 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_tight_all | 251365 | 1587005 | 0.158 | 0.94× |
| carrier_opt_tight_canon | 266309 | 1654404 | 0.161 | 1.00× |
| carrier_opt_tight_cse | 263778 | 1644279 | 0.160 | 0.99× |
| carrier_opt_tight_dce | 253808 | 1576390 | 0.161 | 0.95× |
| carrier_opt_tight_fold | 257081 | 1498886 | 0.172 | 0.96× |
| carrier_opt_tight_none | 267450 | 1661010 | 0.161 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.170 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.142 | 83.6% |
| carrier_opt_tight_canon | 0.038 | 22.2% |
| carrier_opt_tight_cse | 0.035 | 20.7% |
| carrier_opt_tight_dce | 0.034 | 20.0% |
| carrier_opt_tight_fold | 0.034 | 20.1% |
| carrier_opt_tight_none | 0.037 | 21.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 2826ns | 2826ns | -31.83% |
| carrier_opt_tight_canon | 4065ns | 4065ns | -1.95% |
| carrier_opt_tight_cse | 4140ns | 4140ns | -0.14% |
| carrier_opt_tight_dce | 4380ns | 4380ns | +5.64% |
| carrier_opt_tight_fold | 4303ns | 4303ns | +3.78% |
| carrier_opt_tight_none | 4146ns | 4146ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 1739ns | base | --- | [1715, 1932] | --- | --- | --- | --- |
| carrier_opt_tight_all | 451ns | -1337.4ns (-76.9%) | [-1473, -1288]ns | [378, 459] | YES (adj: no) | 0.1563 | 0.0313 | 0 |
| carrier_opt_tight_canon | 1694ns | -118.5ns (-6.8%) | [-148, -16]ns | [1589, 1821] | YES (adj: no) | 0.5469 | 0.2188 | 0 |
| carrier_opt_tight_cse | 1817ns | no significant difference | [-60, +87]ns | [1658, 1906] | no | 0.8594 | 0.6875 | 0 |
| carrier_opt_tight_dce | 1886ns | no significant difference | [-40, +174]ns | [1871, 1902] | no | 0.8594 | 0.6875 | 0 |
| carrier_opt_tight_fold | 1878ns | no significant difference | [-68, +162]ns | [1765, 1899] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_canon | carrier_opt_tight_cse | carrier_opt_tight_dce | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|
| 1 | 1720ns | -78.0% | -6.3% | -4.4% | +9.4% | +9.1% |
| 2 | 1728ns | -73.4% | -8.3% | +1.0% | +9.4% | +9.7% |
| 3 | 1935ns | -76.3% | -6.2% | -2.3% | -3.0% | -2.9% |
| 4 | 1711ns | -78.0% | -6.8% | -2.4% | +10.9% | -1.6% |
| 5 | 1751ns | -74.7% | +4.4% | +8.9% | +6.6% | +8.6% |
| 6 | 1929ns | -76.2% | -7.9% | -1.3% | -1.1% | -4.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.309 | moderate- |
| carrier_opt_tight_canon | -0.279 | moderate- |
| carrier_opt_tight_cse | -0.145 | ok |
| carrier_opt_tight_dce | -0.723 | HIGH- (thermal bounce) |
| carrier_opt_tight_fold | -0.333 | moderate- |
| carrier_opt_tight_none | -0.322 | moderate- |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_canon**: won 5/6, lost 1/6
- **carrier_opt_tight_cse**: won 4/6, lost 2/6
- **carrier_opt_tight_dce**: won 2/6, lost 4/6
- **carrier_opt_tight_fold**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 83741.8ns | 429.1ns | 19513.4% | HIGH |
| carrier_opt_tight_canon | 85580.1ns | 1701.5ns | 5029.7% | HIGH |
| carrier_opt_tight_cse | 85559.5ns | 1793.6ns | 4770.2% | HIGH |
| carrier_opt_tight_dce | 85934.6ns | 1886.5ns | 4555.3% | HIGH |
| carrier_opt_tight_fold | 85601.5ns | 1847.3ns | 4633.9% | HIGH |
| carrier_opt_tight_none | 85499.4ns | 1795.6ns | 4761.7% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 376.7-458.7 ns)
    376.7 |########################################
    380.8 |
    384.9 |
    389.0 |
    393.1 |
    397.2 |
    401.3 |
    405.4 |
    409.5 |
    413.6 |
    417.7 |
    421.8 |
    425.9 |
    430.0 |
    434.1 |
    438.2 |
    442.3 |####################
    446.4 |
    450.5 |
    454.6 |####################
  (0 below, 2 above range)

carrier_opt_tight_canon (n=6, range 1584.6-1820.8 ns)
   1584.6 |########################################
   1596.4 |
   1608.2 |####################
   1620.0 |
   1631.8 |
   1643.7 |
   1655.5 |
   1667.3 |
   1679.1 |
   1690.9 |
   1702.7 |
   1714.5 |
   1726.3 |
   1738.2 |
   1750.0 |
   1761.8 |
   1773.6 |####################
   1785.4 |
   1797.2 |
   1809.0 |####################
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 1645.0-1906.2 ns)
   1645.0 |########################################
   1658.1 |########################################
   1671.1 |
   1684.2 |
   1697.2 |
   1710.3 |
   1723.4 |
   1736.4 |########################################
   1749.5 |
   1762.6 |
   1775.6 |
   1788.7 |
   1801.8 |
   1814.8 |
   1827.9 |
   1840.9 |
   1854.0 |
   1867.1 |
   1880.1 |########################################
   1893.2 |########################################
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 1866.2-1902.3 ns)
   1866.2 |########################################
   1868.0 |
   1869.8 |
   1871.6 |
   1873.4 |
   1875.2 |########################################
   1877.0 |
   1878.8 |
   1880.6 |########################################
   1882.4 |
   1884.2 |
   1886.1 |
   1887.9 |########################################
   1889.7 |
   1891.5 |
   1893.3 |
   1895.1 |
   1896.9 |########################################
   1898.7 |
   1900.5 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 1682.9-1899.0 ns)
   1682.9 |########################################
   1693.7 |
   1704.5 |
   1715.3 |
   1726.1 |
   1736.9 |
   1747.7 |
   1758.5 |
   1769.3 |
   1780.1 |
   1790.9 |
   1801.7 |
   1812.5 |
   1823.3 |
   1834.1 |
   1844.9 |########################################
   1855.7 |
   1866.5 |########################################
   1877.3 |########################################
   1888.1 |########################################
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 1710.8-1931.9 ns)
   1710.8 |########################################
   1721.9 |####################
   1732.9 |
   1744.0 |####################
   1755.0 |
   1766.1 |
   1777.1 |
   1788.2 |
   1799.2 |
   1810.3 |
   1821.3 |
   1832.4 |
   1843.5 |
   1854.5 |
   1865.6 |
   1876.6 |
   1887.7 |
   1898.7 |
   1909.8 |
   1920.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=18552.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_canon**: bridge=5055.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=4711.1% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=4558.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=4562.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=4907.7% of algo (FFI overhead may distort results)

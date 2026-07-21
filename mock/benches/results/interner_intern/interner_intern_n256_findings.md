# Interner intern hot path: FNV vs FxHash x load factor 25% vs 75%

4 variants, 6 samples per variant.
Baseline: **intern_fnv_lf25**

## Highlights

Baseline for all deltas below: **intern_fnv_lf25**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (intern_fnv_lf25)

The baseline intern_fnv_lf25 is the fastest (1.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader intern_fnv_lf25 vs stability leader intern_fx_lf25 (+7% speed for 1.2x steadier)

intern_fnv_lf25 is fastest (1.85 us, CV 6.8%); intern_fx_lf25 gives up 7.3% median for 1.2x lower variance (CV 5.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (intern_fnv_lf25) is the fastest** at 1847.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.15x (fastest 1847.5 ns, slowest 2118.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 4270ns | 4364ns | 3774ns | 4248ns | 4550ns | base |
| intern_fnv_lf75 | 4374ns | 4210ns | 3872ns | 4127ns | 4995ns | +2.43% |
| intern_fx_lf25 | 4400ns | 4505ns | 3925ns | 4377ns | 4672ns | +3.05% |
| intern_fx_lf75 | 4499ns | 4596ns | 4015ns | 4451ns | 4814ns | +5.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| intern_fnv_lf25 | 1810ns | 1586ns | 1934ns | base | 0.141 |
| intern_fnv_lf75 | 1996ns | 1691ns | 2395ns | +10.26% | 0.128 |
| intern_fx_lf25 | 1948ns | 1746ns | 2058ns | +7.61% | 0.131 |
| intern_fx_lf75 | 2063ns | 1825ns | 2190ns | +13.94% | 0.124 |

## Performance model

- Peak throughput: **0.161 Gops/s** (intern_fnv_lf25; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| intern_fnv_lf25 | 0.139 | 85.8% |
| intern_fnv_lf75 | 0.137 | 84.9% |
| intern_fx_lf25 | 0.129 | 80.0% |
| intern_fx_lf75 | 0.121 | 74.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| intern_fnv_lf25 | 4270ns | 4270ns | base |
| intern_fnv_lf75 | 4374ns | 4374ns | +2.43% |
| intern_fx_lf25 | 4400ns | 4400ns | +3.05% |
| intern_fx_lf75 | 4499ns | 4499ns | +5.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 1848ns | base | --- | [1650, 1934] | --- | --- | --- | --- |
| intern_fnv_lf75 | 1867ns | no significant difference | [-97, +523]ns | [1726, 2395] | no | 0.6875 | 0.6875 | 0 |
| intern_fx_lf25 | 1983ns | +144.1ns (+7.8%) | [+18, +251]ns | [1804, 2058] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| intern_fx_lf75 | 2119ns | +254.6ns (+13.8%) | [+229, +274]ns | [1879, 2190] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | intern_fnv_lf25 | intern_fnv_lf75 | intern_fx_lf25 | intern_fx_lf75 |
|---|---|---|---|---|
| 1 | 1812ns | -0.8% | +14.4% | +14.9% |
| 2 | 1927ns | +42.8% | +6.0% | +14.2% |
| 3 | 1882ns | +8.3% | +6.8% | +14.5% |
| 4 | 1941ns | -9.3% | -4.1% | +12.3% |
| 5 | 1586ns | +6.6% | +10.1% | +15.1% |
| 6 | 1715ns | +12.9% | +14.0% | +12.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| intern_fnv_lf25 | 0.108 | ok |
| intern_fnv_lf75 | -0.048 | ok |
| intern_fx_lf25 | 0.370 | moderate+ |
| intern_fx_lf75 | 0.253 | moderate+ |

**Consistency summary:**

- **intern_fnv_lf75**: won 2/6, lost 4/6
- **intern_fx_lf25**: won 1/6, lost 5/6
- **intern_fx_lf75**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| intern_fnv_lf25 | 2058.3ns | 1810.5ns | 113.7% | HIGH |
| intern_fnv_lf75 | 777.9ns | 1996.3ns | 39.0% | HIGH |
| intern_fx_lf25 | 2019.1ns | 1948.2ns | 103.6% | HIGH |
| intern_fx_lf75 | 804.1ns | 2062.8ns | 39.0% | HIGH |

## Distribution (algo ns)

```
intern_fnv_lf25 (n=6, range 1585.8-1933.8 ns)
   1585.8 |########################################
   1603.2 |
   1620.6 |
   1638.0 |
   1655.4 |
   1672.8 |
   1690.2 |
   1707.6 |########################################
   1725.0 |
   1742.4 |
   1759.8 |
   1777.2 |
   1794.6 |
   1812.0 |########################################
   1829.4 |
   1846.8 |
   1864.2 |
   1881.6 |########################################
   1899.0 |
   1916.4 |########################################
  (0 below, 1 above range)

intern_fnv_lf75 (n=6, range 1691.2-2395.4 ns)
   1691.2 |########################################
   1726.4 |########################################
   1761.6 |
   1796.8 |########################################
   1832.0 |
   1867.2 |
   1902.5 |########################################
   1937.7 |
   1972.9 |
   2008.1 |########################################
   2043.3 |
   2078.5 |
   2113.7 |
   2148.9 |
   2184.1 |
   2219.3 |
   2254.6 |
   2289.8 |
   2325.0 |
   2360.2 |
  (0 below, 1 above range)

intern_fx_lf25 (n=6, range 1745.8-2058.2 ns)
   1745.8 |########################################
   1761.4 |
   1777.0 |
   1792.7 |
   1808.3 |
   1823.9 |
   1839.5 |
   1855.1 |########################################
   1870.7 |
   1886.4 |
   1902.0 |
   1917.6 |
   1933.2 |
   1948.8 |########################################
   1964.4 |
   1980.1 |
   1995.7 |########################################
   2011.3 |
   2026.9 |########################################
   2042.5 |
  (0 below, 1 above range)

intern_fx_lf75 (n=6, range 1824.6-2190.4 ns)
   1824.6 |########################################
   1842.9 |
   1861.2 |
   1879.5 |
   1897.8 |
   1916.0 |########################################
   1934.3 |
   1952.6 |
   1970.9 |
   1989.2 |
   2007.5 |
   2025.8 |
   2044.1 |
   2062.4 |
   2080.7 |########################################
   2098.9 |
   2117.2 |
   2135.5 |
   2153.8 |########################################
   2172.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **intern_fnv_lf25**: bridge=113.0% of algo (FFI overhead may distort results)
- **intern_fnv_lf75**: bridge=40.6% of algo (FFI overhead may distort results)
- **intern_fx_lf25**: bridge=104.0% of algo (FFI overhead may distort results)
- **intern_fx_lf75**: bridge=38.0% of algo (FFI overhead may distort results)

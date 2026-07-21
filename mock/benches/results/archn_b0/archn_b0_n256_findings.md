# Per-branch strategy (NATIVE tier): archetype 0

5 variants, 6 samples per variant.
Baseline: **an_b0_table**

## Highlights

Baseline for all deltas below: **an_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (172 ns) is smaller than the fastest variant's own run-to-run std-dev (203 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader an_b0_seq vs stability leader an_b0_tree (+3% speed for 2.0x steadier)

an_b0_seq is fastest (2.12 us, CV 9.6%); an_b0_tree gives up 3.0% median for 2.0x lower variance (CV 4.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b0_prof's edge over baseline is significant but tiny (4 ns, 0.17%)

an_b0_prof differs from baseline an_b0_table by 4 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b0_seq** at 2117.3 ns median (-2.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 2117.3 ns, slowest 2289.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b0_pred | 4780ns | 4865ns | 4201ns | 4852ns | 4962ns | +5.35% |
| an_b0_prof | 4630ns | 4824ns | 3935ns | 4550ns | 5098ns | +2.04% |
| an_b0_seq | 4610ns | 4600ns | 4013ns | 4547ns | 5001ns | +1.59% |
| an_b0_table | 4537ns | 4758ns | 3938ns | 4550ns | 4818ns | base |
| an_b0_tree | 4631ns | 4761ns | 4160ns | 4682ns | 4792ns | +2.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b0_pred | 2252ns | 1974ns | 2348ns | +7.89% | 0.114 |
| an_b0_prof | 2115ns | 1801ns | 2326ns | +1.33% | 0.121 |
| an_b0_seq | 2146ns | 1839ns | 2379ns | +2.81% | 0.119 |
| an_b0_table | 2087ns | 1804ns | 2231ns | base | 0.123 |
| an_b0_tree | 2134ns | 1908ns | 2201ns | +2.24% | 0.120 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_b0_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b0_pred | 0.112 | 78.7% |
| an_b0_prof | 0.117 | 82.0% |
| an_b0_seq | 0.121 | 85.1% |
| an_b0_table | 0.118 | 82.7% |
| an_b0_tree | 0.117 | 82.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b0_pred | 4780ns | 4780ns | +5.35% |
| an_b0_prof | 4630ns | 4630ns | +2.04% |
| an_b0_seq | 4610ns | 4610ns | +1.59% |
| an_b0_table | 4537ns | 4537ns | base |
| an_b0_tree | 4631ns | 4631ns | +2.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b0_table | 2176ns | base | --- | [1853, 2231] | --- | --- | --- | --- |
| an_b0_pred | 2289ns | +96.7ns (+4.4%) | [+42, +355]ns | [2118, 2348] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b0_prof | 2196ns | no significant difference | [-71, +150]ns | [1822, 2326] | no | 1.0000 | 1.0000 | 0 |
| an_b0_seq | 2117ns | no significant difference | [-179, +336]ns | [1940, 2379] | no | 1.0000 | 1.0000 | 0 |
| an_b0_tree | 2181ns | no significant difference | [-66, +202]ns | [2019, 2201] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b0_table | an_b0_pred | an_b0_prof | an_b0_seq | an_b0_tree |
|---|---|---|---|---|---|
| 1 | 1804ns | +29.4% | -0.2% | +21.1% | +21.4% |
| 2 | 2197ns | +4.7% | +0.5% | +13.2% | -3.0% |
| 3 | 2182ns | +8.2% | +5.2% | -6.1% | +0.8% |
| 4 | 2171ns | +4.1% | +8.6% | +4.6% | +0.1% |
| 5 | 2265ns | +0.6% | -3.6% | -9.9% | -2.9% |
| 6 | 1902ns | +3.8% | -3.1% | -3.3% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b0_pred | 0.032 | ok |
| an_b0_prof | 0.107 | ok |
| an_b0_seq | -0.050 | ok |
| an_b0_table | -0.176 | ok |
| an_b0_tree | -0.160 | ok |

**Consistency summary:**

- **an_b0_pred**: won 0/6, lost 6/6
- **an_b0_prof**: won 3/6, lost 3/6
- **an_b0_seq**: won 3/6, lost 3/6
- **an_b0_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b0_pred | 6.3ns | 2251.6ns | 0.3% |  |
| an_b0_prof | 6.5ns | 2114.6ns | 0.3% |  |
| an_b0_seq | 5.8ns | 2145.5ns | 0.3% |  |
| an_b0_table | 5.4ns | 2086.9ns | 0.3% |  |
| an_b0_tree | 6.1ns | 2133.7ns | 0.3% |  |

## Distribution (algo ns)

```
an_b0_pred (n=6, range 1974.2-2348.1 ns)
   1974.2 |########################################
   1992.9 |
   2011.6 |
   2030.3 |
   2049.0 |
   2067.7 |
   2086.4 |
   2105.1 |
   2123.8 |
   2142.5 |
   2161.2 |
   2179.8 |
   2198.5 |
   2217.2 |
   2235.9 |
   2254.6 |########################################
   2273.3 |########################################
   2292.0 |########################################
   2310.7 |
   2329.4 |########################################
  (0 below, 1 above range)

an_b0_prof (n=6, range 1800.8-2326.4 ns)
   1800.8 |########################################
   1827.1 |########################################
   1853.4 |
   1879.6 |
   1905.9 |
   1932.2 |
   1958.5 |
   1984.8 |
   2011.1 |
   2037.3 |
   2063.6 |
   2089.9 |
   2116.2 |
   2142.5 |
   2168.8 |########################################
   2195.0 |########################################
   2221.3 |
   2247.6 |
   2273.9 |########################################
   2300.2 |
  (0 below, 1 above range)

an_b0_seq (n=6, range 1839.2-2379.2 ns)
   1839.2 |####################
   1866.2 |
   1893.2 |
   1920.2 |
   1947.2 |
   1974.2 |
   2001.2 |
   2028.2 |########################################
   2055.2 |
   2082.2 |
   2109.2 |
   2136.2 |
   2163.2 |####################
   2190.2 |
   2217.2 |
   2244.2 |####################
   2271.2 |
   2298.2 |
   2325.2 |
   2352.2 |
  (0 below, 1 above range)

an_b0_table (n=6, range 1804.2-2231.1 ns)
   1804.2 |####################
   1825.5 |
   1846.9 |
   1868.2 |
   1889.6 |####################
   1910.9 |
   1932.3 |
   1953.6 |
   1974.9 |
   1996.3 |
   2017.6 |
   2039.0 |
   2060.3 |
   2081.7 |
   2103.0 |
   2124.3 |
   2145.7 |
   2167.0 |########################################
   2188.4 |####################
   2209.7 |
  (0 below, 1 above range)

an_b0_tree (n=6, range 1907.9-2200.6 ns)
   1907.9 |####################
   1922.5 |
   1937.2 |
   1951.8 |
   1966.4 |
   1981.1 |
   1995.7 |
   2010.3 |
   2025.0 |
   2039.6 |
   2054.2 |
   2068.9 |
   2083.5 |
   2098.2 |
   2112.8 |
   2127.4 |####################
   2142.1 |
   2156.7 |
   2171.3 |####################
   2186.0 |########################################
  (0 below, 1 above range)

```

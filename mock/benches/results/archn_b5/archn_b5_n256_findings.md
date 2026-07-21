# Per-branch strategy (NATIVE tier): archetype 5

5 variants, 6 samples per variant.
Baseline: **an_b5_table**

## Highlights

Baseline for all deltas below: **an_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b5_tree, an_b5_table) are a dead heat (<1%)

an_b5_tree (2.18 us) and an_b5_table (2.18 us) differ by 0.29%, inside the noise, even though the wider field spreads 23.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b5_tree's edge over baseline is significant but tiny (-25 ns, 1.14%)

an_b5_tree differs from baseline an_b5_table by -25 ns (1.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b5_tree** at 2177.3 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 2177.3 ns, slowest 2694.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b5_pred | 5175ns | 5263ns | 4562ns | 5031ns | 5699ns | +9.95% |
| an_b5_prof | 4626ns | 4649ns | 4038ns | 4473ns | 5149ns | -1.72% |
| an_b5_seq | 4782ns | 4888ns | 4119ns | 4681ns | 5264ns | +1.59% |
| an_b5_table | 4707ns | 4754ns | 4009ns | 4748ns | 4994ns | base |
| an_b5_tree | 4629ns | 4763ns | 4150ns | 4606ns | 4903ns | -1.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b5_pred | 2651ns | 2337ns | 2920ns | +22.23% | 0.097 |
| an_b5_prof | 2198ns | 1929ns | 2440ns | +1.37% | 0.116 |
| an_b5_seq | 2267ns | 1964ns | 2475ns | +4.54% | 0.113 |
| an_b5_table | 2169ns | 1848ns | 2310ns | base | 0.118 |
| an_b5_tree | 2109ns | 1891ns | 2252ns | -2.74% | 0.121 |

## Performance model

- Peak throughput: **0.139 Gops/s** (an_b5_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b5_pred | 0.095 | 68.6% |
| an_b5_prof | 0.116 | 83.5% |
| an_b5_seq | 0.110 | 79.4% |
| an_b5_table | 0.117 | 84.6% |
| an_b5_tree | 0.118 | 84.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b5_pred | 5175ns | 5175ns | +9.95% |
| an_b5_prof | 4626ns | 4626ns | -1.72% |
| an_b5_seq | 4782ns | 4782ns | +1.59% |
| an_b5_table | 4707ns | 4707ns | base |
| an_b5_tree | 4629ns | 4629ns | -1.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b5_table | 2184ns | base | --- | [2013, 2310] | --- | --- | --- | --- |
| an_b5_pred | 2694ns | +504.4ns (+23.1%) | [+313, +629]ns | [2339, 2920] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b5_prof | 2212ns | no significant difference | [-129, +149]ns | [1944, 2440] | no | 0.9167 | 0.6875 | 0 |
| an_b5_seq | 2326ns | no significant difference | [-16, +184]ns | [2001, 2475] | no | 0.4375 | 0.2188 | 0 |
| an_b5_tree | 2177ns | no significant difference | [-245, +92]ns | [1898, 2252] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b5_table | an_b5_pred | an_b5_prof | an_b5_seq | an_b5_tree |
|---|---|---|---|---|---|
| 1 | 2179ns | +7.4% | -11.5% | -6.4% | -13.2% |
| 2 | 1848ns | +26.5% | +6.0% | +6.3% | +3.2% |
| 3 | 2187ns | +23.8% | -0.4% | +6.4% | +0.1% |
| 4 | 2218ns | +20.9% | +1.2% | +4.9% | -2.3% |
| 5 | 2180ns | +33.9% | +6.9% | +10.3% | +5.8% |
| 6 | 2401ns | +21.6% | +6.1% | +5.9% | -8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b5_pred | 0.473 | moderate+ |
| an_b5_prof | 0.443 | moderate+ |
| an_b5_seq | 0.411 | moderate+ |
| an_b5_table | -0.030 | ok |
| an_b5_tree | 0.426 | moderate+ |

**Consistency summary:**

- **an_b5_pred**: won 0/6, lost 6/6
- **an_b5_prof**: won 2/6, lost 4/6
- **an_b5_seq**: won 1/6, lost 5/6
- **an_b5_tree**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b5_pred | 6.5ns | 2650.8ns | 0.2% |  |
| an_b5_prof | 5.9ns | 2198.5ns | 0.3% |  |
| an_b5_seq | 6.3ns | 2267.2ns | 0.3% |  |
| an_b5_table | 6.2ns | 2168.8ns | 0.3% |  |
| an_b5_tree | 6.8ns | 2109.4ns | 0.3% |  |

## Distribution (algo ns)

```
an_b5_pred (n=6, range 2337.1-2919.6 ns)
   2337.1 |########################################
   2366.2 |
   2395.3 |
   2424.5 |
   2453.6 |
   2482.7 |
   2511.8 |
   2541.0 |
   2570.1 |
   2599.2 |
   2628.3 |
   2657.4 |####################
   2686.6 |####################
   2715.7 |
   2744.8 |
   2773.9 |
   2803.1 |
   2832.2 |
   2861.3 |
   2890.4 |####################
  (0 below, 1 above range)

an_b5_prof (n=6, range 1929.2-2439.6 ns)
   1929.2 |########################################
   1954.7 |########################################
   1980.2 |
   2005.8 |
   2031.3 |
   2056.8 |
   2082.3 |
   2107.8 |
   2133.4 |
   2158.9 |########################################
   2184.4 |
   2209.9 |
   2235.4 |########################################
   2261.0 |
   2286.5 |
   2312.0 |########################################
   2337.5 |
   2363.0 |
   2388.6 |
   2414.1 |
  (0 below, 1 above range)

an_b5_seq (n=6, range 1963.8-2474.6 ns)
   1963.8 |####################
   1989.3 |
   2014.9 |####################
   2040.4 |
   2065.9 |
   2091.5 |
   2117.0 |
   2142.6 |
   2168.1 |
   2193.6 |
   2219.2 |
   2244.7 |
   2270.2 |
   2295.8 |
   2321.3 |########################################
   2346.9 |
   2372.4 |
   2397.9 |####################
   2423.5 |
   2449.0 |
  (0 below, 1 above range)

an_b5_table (n=6, range 1847.5-2309.6 ns)
   1847.5 |#############
   1870.6 |
   1893.7 |
   1916.8 |
   1939.9 |
   1963.0 |
   1986.1 |
   2009.2 |
   2032.3 |
   2055.4 |
   2078.5 |
   2101.6 |
   2124.7 |
   2147.8 |
   2170.9 |########################################
   2194.0 |
   2217.1 |#############
   2240.2 |
   2263.3 |
   2286.4 |
  (0 below, 1 above range)

an_b5_tree (n=6, range 1890.8-2252.5 ns)
   1890.8 |########################################
   1908.9 |
   1927.0 |
   1945.1 |
   1963.1 |
   1981.2 |
   1999.3 |
   2017.4 |
   2035.5 |
   2053.6 |
   2071.7 |
   2089.7 |
   2107.8 |
   2125.9 |
   2144.0 |
   2162.1 |####################
   2180.2 |####################
   2198.2 |####################
   2216.3 |
   2234.4 |
  (0 below, 1 above range)

```

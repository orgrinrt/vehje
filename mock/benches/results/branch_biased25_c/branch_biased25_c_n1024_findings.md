# Branch strategies, cheap-arm, biased25: ~25% taken (b<64)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_biased25**

## Key findings

- **Baseline (br_branch_c_biased25) is the fastest** at 1499.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.54x (fastest 1499.2 ns, slowest 2306.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 3975ns | 4004ns | 3430ns | 3816ns | 4487ns | base |
| br_lut_c_biased25 | 4623ns | 4683ns | 4064ns | 4518ns | 5059ns | +16.28% |
| br_mask_c_biased25 | 4653ns | 4742ns | 4070ns | 4524ns | 5138ns | +17.05% |
| br_predicate_c_biased25 | 4805ns | 4926ns | 4223ns | 4692ns | 5266ns | +20.87% |
| br_profiled_hot_c_biased25 | 4099ns | 4035ns | 3782ns | 3962ns | 4464ns | +3.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_biased25 | 1502ns | 1259ns | 1723ns | base | 0.682 |
| br_lut_c_biased25 | 2148ns | 1886ns | 2350ns | +43.01% | 0.477 |
| br_mask_c_biased25 | 2178ns | 1900ns | 2438ns | +45.04% | 0.470 |
| br_predicate_c_biased25 | 2247ns | 1969ns | 2465ns | +49.62% | 0.456 |
| br_profiled_hot_c_biased25 | 1716ns | 1568ns | 1875ns | +14.25% | 0.597 |

## Performance model

- Peak throughput: **0.813 Gops/s** (br_branch_c_biased25; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_biased25 | 0.683 | 84.0% |
| br_lut_c_biased25 | 0.471 | 57.9% |
| br_mask_c_biased25 | 0.467 | 57.4% |
| br_predicate_c_biased25 | 0.444 | 54.6% |
| br_profiled_hot_c_biased25 | 0.607 | 74.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_biased25 | 3975ns | 3975ns | base |
| br_lut_c_biased25 | 4623ns | 4623ns | +16.28% |
| br_mask_c_biased25 | 4653ns | 4653ns | +17.05% |
| br_predicate_c_biased25 | 4805ns | 4805ns | +20.87% |
| br_profiled_hot_c_biased25 | 4099ns | 4099ns | +3.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 1499ns | base | --- | [1283, 1723] | --- | --- | --- | --- |
| br_lut_c_biased25 | 2176ns | +634.4ns (+42.3%) | [+554, +749]ns | [1918, 2350] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_biased25 | 2194ns | +630.6ns (+42.1%) | [+558, +841]ns | [1903, 2438] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_biased25 | 2307ns | +711.7ns (+47.5%) | [+678, +846]ns | [1970, 2465] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_biased25 | 1687ns | no significant difference | [-70, +414]ns | [1586, 1875] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_biased25 | br_lut_c_biased25 | br_mask_c_biased25 | br_predicate_c_biased25 | br_profiled_hot_c_biased25 |
|---|---|---|---|---|---|
| 1 | 1596ns | +31.8% | +32.8% | +46.5% | -1.7% |
| 2 | 1850ns | +32.5% | +33.2% | +37.4% | -6.0% |
| 3 | 1561ns | +44.1% | +54.4% | +45.7% | +17.1% |
| 4 | 1437ns | +56.4% | +57.9% | +66.1% | +33.7% |
| 5 | 1259ns | +49.8% | +51.4% | +56.4% | +27.3% |
| 6 | 1308ns | +49.0% | +45.3% | +50.7% | +25.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_biased25 | 0.480 | moderate+ |
| br_lut_c_biased25 | 0.242 | moderate+ |
| br_mask_c_biased25 | 0.407 | moderate+ |
| br_predicate_c_biased25 | 0.285 | moderate+ |
| br_profiled_hot_c_biased25 | 0.087 | ok |

**Consistency summary:**

- **br_lut_c_biased25**: won 0/6, lost 6/6
- **br_mask_c_biased25**: won 0/6, lost 6/6
- **br_predicate_c_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_c_biased25**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_biased25 | 3.5ns | 1501.8ns | 0.2% |  |
| br_lut_c_biased25 | 3.5ns | 2147.8ns | 0.2% |  |
| br_mask_c_biased25 | 3.4ns | 2178.3ns | 0.2% |  |
| br_predicate_c_biased25 | 3.5ns | 2247.0ns | 0.2% |  |
| br_profiled_hot_c_biased25 | 3.9ns | 1715.7ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_c_biased25 (n=6, range 1258.8-1723.1 ns)
   1258.8 |########################################
   1282.0 |
   1305.2 |########################################
   1328.4 |
   1351.7 |
   1374.9 |
   1398.1 |
   1421.3 |########################################
   1444.5 |
   1467.7 |
   1490.9 |
   1514.2 |
   1537.4 |
   1560.6 |########################################
   1583.8 |########################################
   1607.0 |
   1630.2 |
   1653.5 |
   1676.7 |
   1699.9 |
  (0 below, 1 above range)

br_lut_c_biased25 (n=6, range 1886.2-2350.2 ns)
   1886.2 |####################
   1909.4 |
   1932.6 |####################
   1955.8 |
   1979.0 |
   2002.2 |
   2025.4 |
   2048.6 |
   2071.8 |
   2095.0 |####################
   2118.2 |
   2141.4 |
   2164.6 |
   2187.8 |
   2211.0 |
   2234.2 |########################################
   2257.4 |
   2280.6 |
   2303.8 |
   2327.0 |
  (0 below, 1 above range)

br_mask_c_biased25 (n=6, range 1900.4-2437.7 ns)
   1900.4 |########################################
   1927.3 |
   1954.1 |
   1981.0 |
   2007.9 |
   2034.7 |
   2061.6 |
   2088.5 |
   2115.3 |####################
   2142.2 |
   2169.1 |
   2195.9 |
   2222.8 |
   2249.6 |####################
   2276.5 |
   2303.4 |
   2330.2 |
   2357.1 |
   2384.0 |
   2410.8 |####################
  (0 below, 1 above range)

br_predicate_c_biased25 (n=6, range 1968.8-2464.6 ns)
   1968.8 |########################################
   1993.6 |
   2018.4 |
   2043.2 |
   2068.0 |
   2092.8 |
   2117.5 |
   2142.3 |
   2167.1 |
   2191.9 |
   2216.7 |
   2241.5 |
   2266.3 |####################
   2291.1 |
   2315.9 |####################
   2340.7 |
   2365.4 |####################
   2390.2 |
   2415.0 |
   2439.8 |
  (0 below, 1 above range)

br_profiled_hot_c_biased25 (n=6, range 1568.3-1874.8 ns)
   1568.3 |########################################
   1583.6 |
   1599.0 |########################################
   1614.3 |
   1629.6 |########################################
   1644.9 |
   1660.2 |
   1675.6 |
   1690.9 |
   1706.2 |
   1721.5 |
   1736.9 |########################################
   1752.2 |
   1767.5 |
   1782.8 |
   1798.2 |
   1813.5 |########################################
   1828.8 |
   1844.1 |
   1859.5 |
  (0 below, 1 above range)

```

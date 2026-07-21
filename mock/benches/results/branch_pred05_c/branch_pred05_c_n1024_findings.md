# Branch strategies, cheap-arm, pred05: ~5% taken, predictable (b<13)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred05**

## Key findings

- **Baseline (br_branch_c_pred05) is the fastest** at 1542.3 ns median
- 4 variants significantly slower than baseline
- Spread: 1.49x (fastest 1542.3 ns, slowest 2296.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 4060ns | 4146ns | 3652ns | 4081ns | 4234ns | base |
| br_lut_c_pred05 | 5528ns | 4953ns | 4048ns | 4917ns | 7186ns | +36.15% |
| br_mask_c_pred05 | 4732ns | 4770ns | 4080ns | 4696ns | 5112ns | +16.53% |
| br_predicate_c_pred05 | 4574ns | 4702ns | 4007ns | 4569ns | 4866ns | +12.66% |
| br_profiled_hot_c_pred05 | 4166ns | 4216ns | 3548ns | 4202ns | 4421ns | +2.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred05 | 1528ns | 1389ns | 1585ns | base | 0.670 |
| br_lut_c_pred05 | 2629ns | 1887ns | 3525ns | +72.09% | 0.389 |
| br_mask_c_pred05 | 2192ns | 1903ns | 2365ns | +43.45% | 0.467 |
| br_predicate_c_pred05 | 2137ns | 1870ns | 2272ns | +39.87% | 0.479 |
| br_profiled_hot_c_pred05 | 1592ns | 1370ns | 1717ns | +4.21% | 0.643 |

## Performance model

- Peak throughput: **0.748 Gops/s** (br_profiled_hot_c_pred05; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred05 | 0.664 | 88.8% |
| br_lut_c_pred05 | 0.446 | 59.6% |
| br_mask_c_pred05 | 0.466 | 62.4% |
| br_predicate_c_pred05 | 0.467 | 62.4% |
| br_profiled_hot_c_pred05 | 0.641 | 85.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred05 | 4060ns | 4060ns | base |
| br_lut_c_pred05 | 5528ns | 5528ns | +36.15% |
| br_mask_c_pred05 | 4732ns | 4732ns | +16.53% |
| br_predicate_c_pred05 | 4574ns | 4574ns | +12.66% |
| br_profiled_hot_c_pred05 | 4166ns | 4166ns | +2.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 1542ns | base | --- | [1456, 1585] | --- | --- | --- | --- |
| br_lut_c_pred05 | 2296ns | +765.0ns (+49.6%) | [+507, +2032]ns | [2066, 3525] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred05 | 2196ns | +661.3ns (+42.9%) | [+533, +797]ns | [2013, 2365] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred05 | 2194ns | +651.9ns (+42.3%) | [+448, +728]ns | [1945, 2272] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred05 | 1598ns | +53.9ns (+3.5%) | [+5, +134]ns | [1461, 1717] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred05 | br_lut_c_pred05 | br_mask_c_pred05 | br_predicate_c_pred05 | br_profiled_hot_c_pred05 |
|---|---|---|---|---|---|
| 1 | 1389ns | +232.1% | +37.1% | +45.4% | -1.4% |
| 2 | 1573ns | +19.9% | +35.0% | +18.9% | +12.8% |
| 3 | 1523ns | +47.8% | +39.3% | +39.3% | +1.9% |
| 4 | 1597ns | +52.7% | +54.2% | +42.1% | +4.0% |
| 5 | 1545ns | +45.3% | +46.8% | +46.7% | +4.3% |
| 6 | 1540ns | +52.1% | +47.4% | +47.7% | +2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred05 | -0.205 | moderate- |
| br_lut_c_pred05 | -0.191 | ok |
| br_mask_c_pred05 | 0.181 | ok |
| br_predicate_c_pred05 | 0.494 | moderate+ |
| br_profiled_hot_c_pred05 | -0.550 | HIGH- (thermal bounce) |

**Consistency summary:**

- **br_lut_c_pred05**: won 0/6, lost 6/6
- **br_mask_c_pred05**: won 0/6, lost 6/6
- **br_predicate_c_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred05**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred05 | 4.1ns | 1527.8ns | 0.3% |  |
| br_lut_c_pred05 | 5.5ns | 2629.2ns | 0.2% |  |
| br_mask_c_pred05 | 4.1ns | 2191.5ns | 0.2% |  |
| br_predicate_c_pred05 | 3.7ns | 2136.9ns | 0.2% |  |
| br_profiled_hot_c_pred05 | 4.5ns | 1592.1ns | 0.3% |  |

## Distribution (algo ns)

```
br_branch_c_pred05 (n=6, range 1388.7-1585.0 ns)
   1388.7 |####################
   1398.5 |
   1408.3 |
   1418.1 |
   1428.0 |
   1437.8 |
   1447.6 |
   1457.4 |
   1467.2 |
   1477.0 |
   1486.8 |
   1496.7 |
   1506.5 |
   1516.3 |####################
   1526.1 |
   1535.9 |########################################
   1545.7 |
   1555.6 |
   1565.4 |####################
   1575.2 |
  (0 below, 1 above range)

br_lut_c_pred05 (n=6, range 1886.7-3525.0 ns)
   1886.7 |####################
   1968.6 |
   2050.5 |
   2132.4 |
   2214.4 |########################################
   2296.3 |####################
   2378.2 |####################
   2460.1 |
   2542.0 |
   2623.9 |
   2705.8 |
   2787.8 |
   2869.7 |
   2951.6 |
   3033.5 |
   3115.4 |
   3197.3 |
   3279.3 |
   3361.2 |
   3443.1 |
  (0 below, 1 above range)

br_mask_c_pred05 (n=6, range 1903.3-2365.4 ns)
   1903.3 |####################
   1926.4 |
   1949.5 |
   1972.6 |
   1995.7 |
   2018.8 |
   2041.9 |
   2065.1 |
   2088.2 |
   2111.3 |########################################
   2134.4 |
   2157.5 |
   2180.6 |
   2203.7 |
   2226.8 |
   2249.9 |########################################
   2273.0 |
   2296.1 |
   2319.2 |
   2342.3 |
  (0 below, 1 above range)

br_predicate_c_pred05 (n=6, range 1870.4-2272.1 ns)
   1870.4 |####################
   1890.5 |
   1910.6 |
   1930.7 |
   1950.7 |
   1970.8 |
   1990.9 |
   2011.0 |####################
   2031.1 |
   2051.2 |
   2071.2 |
   2091.3 |
   2111.4 |####################
   2131.5 |
   2151.6 |
   2171.7 |
   2191.8 |
   2211.8 |
   2231.9 |
   2252.0 |########################################
  (0 below, 1 above range)

br_profiled_hot_c_pred05 (n=6, range 1369.6-1717.3 ns)
   1369.6 |########################################
   1387.0 |
   1404.4 |
   1421.8 |
   1439.1 |
   1456.5 |
   1473.9 |
   1491.3 |
   1508.7 |
   1526.1 |
   1543.5 |########################################
   1560.8 |
   1578.2 |########################################
   1595.6 |########################################
   1613.0 |
   1630.4 |
   1647.8 |########################################
   1665.1 |
   1682.5 |
   1699.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_lut_c_pred05**: CV=34.3% (high variance, measurements may be unstable)

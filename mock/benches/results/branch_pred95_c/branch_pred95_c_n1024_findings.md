# Branch strategies, cheap-arm, pred95: ~95% taken, predictable (b<243)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred95**

## Key findings

- **Fastest: br_profiled_hot_c_pred95** at 1424.6 ns median (-7.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.66x (fastest 1424.6 ns, slowest 2366.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 4071ns | 4190ns | 3446ns | 3979ns | 4520ns | base |
| br_lut_c_pred95 | 4717ns | 4840ns | 4060ns | 4582ns | 5249ns | +15.88% |
| br_mask_c_pred95 | 4730ns | 4750ns | 4073ns | 4577ns | 5287ns | +16.19% |
| br_predicate_c_pred95 | 4854ns | 5063ns | 4073ns | 4786ns | 5346ns | +19.24% |
| br_profiled_hot_c_pred95 | 3846ns | 3846ns | 3516ns | 3750ns | 4154ns | -5.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred95 | 1525ns | 1272ns | 1704ns | base | 0.672 |
| br_lut_c_pred95 | 2194ns | 1885ns | 2444ns | +43.93% | 0.467 |
| br_mask_c_pred95 | 2208ns | 1904ns | 2464ns | +44.85% | 0.464 |
| br_predicate_c_pred95 | 2263ns | 1901ns | 2486ns | +48.41% | 0.453 |
| br_profiled_hot_c_pred95 | 1427ns | 1297ns | 1554ns | -6.37% | 0.717 |

## Performance model

- Peak throughput: **0.805 Gops/s** (br_branch_c_pred95; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred95 | 0.662 | 82.2% |
| br_lut_c_pred95 | 0.455 | 56.5% |
| br_mask_c_pred95 | 0.461 | 57.2% |
| br_predicate_c_pred95 | 0.433 | 53.7% |
| br_profiled_hot_c_pred95 | 0.719 | 89.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred95 | 4071ns | 4071ns | base |
| br_lut_c_pred95 | 4717ns | 4717ns | +15.88% |
| br_mask_c_pred95 | 4730ns | 4730ns | +16.19% |
| br_predicate_c_pred95 | 4854ns | 4854ns | +19.24% |
| br_profiled_hot_c_pred95 | 3846ns | 3846ns | -5.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 1548ns | base | --- | [1323, 1704] | --- | --- | --- | --- |
| br_lut_c_pred95 | 2252ns | +669.1ns (+43.2%) | [+526, +815]ns | [1887, 2444] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred95 | 2222ns | +673.3ns (+43.5%) | [+543, +835]ns | [1939, 2464] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred95 | 2367ns | +734.2ns (+47.4%) | [+541, +939]ns | [1935, 2486] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred95 | 1425ns | no significant difference | [-194, +10]ns | [1303, 1554] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred95 | br_lut_c_pred95 | br_mask_c_pred95 | br_predicate_c_pred95 | br_profiled_hot_c_pred95 |
|---|---|---|---|---|---|
| 1 | 1528ns | +47.5% | +42.2% | +63.9% | -15.1% |
| 2 | 1567ns | +55.9% | +57.2% | +57.5% | -1.1% |
| 3 | 1691ns | +44.6% | +45.7% | +45.6% | -9.3% |
| 4 | 1374ns | +37.6% | +38.6% | +38.4% | -4.3% |
| 5 | 1272ns | +48.2% | +55.2% | +54.8% | +3.0% |
| 6 | 1716ns | +31.2% | +32.4% | +32.3% | -9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred95 | -0.184 | ok |
| br_lut_c_pred95 | 0.241 | moderate+ |
| br_mask_c_pred95 | 0.123 | ok |
| br_predicate_c_pred95 | 0.341 | moderate+ |
| br_profiled_hot_c_pred95 | -0.197 | ok |

**Consistency summary:**

- **br_lut_c_pred95**: won 0/6, lost 6/6
- **br_mask_c_pred95**: won 0/6, lost 6/6
- **br_predicate_c_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred95**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred95 | 4.6ns | 1524.6ns | 0.3% |  |
| br_lut_c_pred95 | 3.7ns | 2194.4ns | 0.2% |  |
| br_mask_c_pred95 | 3.0ns | 2208.3ns | 0.1% |  |
| br_predicate_c_pred95 | 2.8ns | 2262.6ns | 0.1% |  |
| br_profiled_hot_c_pred95 | 3.5ns | 1427.4ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_c_pred95 (n=6, range 1271.7-1703.5 ns)
   1271.7 |########################################
   1293.3 |
   1314.9 |
   1336.5 |
   1358.1 |########################################
   1379.7 |
   1401.2 |
   1422.8 |
   1444.4 |
   1466.0 |
   1487.6 |
   1509.2 |########################################
   1530.8 |
   1552.4 |########################################
   1574.0 |
   1595.5 |
   1617.1 |
   1638.7 |
   1660.3 |
   1681.9 |########################################
  (0 below, 1 above range)

br_lut_c_pred95 (n=6, range 1885.0-2443.6 ns)
   1885.0 |########################################
   1912.9 |
   1940.9 |
   1968.8 |
   1996.7 |
   2024.6 |
   2052.6 |
   2080.5 |
   2108.4 |
   2136.3 |
   2164.3 |
   2192.2 |
   2220.1 |
   2248.1 |########################################
   2276.0 |
   2303.9 |
   2331.8 |
   2359.8 |
   2387.7 |
   2415.6 |####################
  (0 below, 1 above range)

br_mask_c_pred95 (n=6, range 1904.2-2463.8 ns)
   1904.2 |########################################
   1932.2 |
   1960.2 |########################################
   1988.1 |
   2016.1 |
   2044.1 |
   2072.1 |
   2100.0 |
   2128.0 |
   2156.0 |########################################
   2184.0 |
   2212.0 |
   2239.9 |
   2267.9 |########################################
   2295.9 |
   2323.9 |
   2351.8 |
   2379.8 |
   2407.8 |
   2435.8 |########################################
  (0 below, 1 above range)

br_predicate_c_pred95 (n=6, range 1901.2-2486.1 ns)
   1901.2 |####################
   1930.4 |
   1959.7 |####################
   1988.9 |
   2018.2 |
   2047.4 |
   2076.7 |
   2105.9 |
   2135.1 |
   2164.4 |
   2193.6 |
   2222.9 |
   2252.1 |####################
   2281.4 |
   2310.6 |
   2339.8 |
   2369.1 |
   2398.3 |
   2427.6 |
   2456.8 |########################################
  (0 below, 1 above range)

br_profiled_hot_c_pred95 (n=6, range 1297.1-1554.2 ns)
   1297.1 |########################################
   1310.0 |####################
   1322.8 |
   1335.7 |
   1348.5 |
   1361.4 |
   1374.2 |
   1387.1 |
   1399.9 |
   1412.8 |
   1425.7 |
   1438.5 |
   1451.4 |
   1464.2 |
   1477.1 |
   1489.9 |
   1502.8 |
   1515.6 |
   1528.5 |####################
   1541.3 |####################
  (0 below, 1 above range)

```

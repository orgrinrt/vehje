# Multiway branch strategies, cheap-arm, mw8_uni: 8-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_uni**

## Key findings

- **Fastest: mw_jumptable_c_mw8_uni** at 1300.6 ns median (-38.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.53x (fastest 1300.6 ns, slowest 7189.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 4542ns | 4619ns | 4005ns | 4421ns | 4993ns | base |
| mw_chain_c_mw8_uni | 3764ns | 3733ns | 3338ns | 3645ns | 4157ns | -17.13% |
| mw_chain_rev_c_mw8_uni | 3775ns | 3848ns | 3315ns | 3679ns | 4149ns | -16.89% |
| mw_jumptable_c_mw8_uni | 3820ns | 3771ns | 3336ns | 3671ns | 4287ns | -15.89% |
| mw_predicate_all_c_mw8_uni | 9977ns | 9392ns | 9306ns | 9368ns | 11227ns | +119.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 2074ns | 1829ns | 2278ns | base | 0.494 |
| mw_chain_c_mw8_uni | 1313ns | 1170ns | 1439ns | -36.71% | 0.780 |
| mw_chain_rev_c_mw8_uni | 1317ns | 1148ns | 1449ns | -36.49% | 0.777 |
| mw_jumptable_c_mw8_uni | 1329ns | 1169ns | 1498ns | -35.92% | 0.770 |
| mw_predicate_all_c_mw8_uni | 7645ns | 7137ns | 8607ns | +268.58% | 0.134 |

## Performance model

- Peak throughput: **0.892 Gops/s** (mw_chain_rev_c_mw8_uni; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.485 | 54.4% |
| mw_chain_c_mw8_uni | 0.782 | 87.6% |
| mw_chain_rev_c_mw8_uni | 0.761 | 85.3% |
| mw_jumptable_c_mw8_uni | 0.787 | 88.3% |
| mw_predicate_all_c_mw8_uni | 0.142 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_uni | 4542ns | 4542ns | base |
| mw_chain_c_mw8_uni | 3764ns | 3764ns | -17.13% |
| mw_chain_rev_c_mw8_uni | 3775ns | 3775ns | -16.89% |
| mw_jumptable_c_mw8_uni | 3820ns | 3820ns | -15.89% |
| mw_predicate_all_c_mw8_uni | 9977ns | 9977ns | +119.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 2111ns | base | --- | [1834, 2278] | --- | --- | --- | --- |
| mw_chain_c_mw8_uni | 1310ns | -810.9ns (-38.4%) | [-879, -595]ns | [1189, 1439] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_uni | 1346ns | -794.5ns (-37.6%) | [-876, -600]ns | [1157, 1449] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_uni | 1301ns | -718.3ns (-34.0%) | [-872, -645]ns | [1189, 1498] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_uni | 7189ns | +5355.6ns (+253.8%) | [+5026, +6330]ns | [7138, 8607] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_uni | mw_chain_c_mw8_uni | mw_chain_rev_c_mw8_uni | mw_jumptable_c_mw8_uni | mw_predicate_all_c_mw8_uni |
|---|---|---|---|---|---|
| 1 | 2186ns | -40.0% | -36.4% | -40.5% | +226.5% |
| 2 | 2370ns | -37.3% | -36.4% | -36.2% | +277.2% |
| 3 | 2038ns | -40.7% | -43.7% | -36.2% | +250.2% |
| 4 | 2183ns | -36.3% | -36.4% | -32.0% | +279.1% |
| 5 | 1838ns | -36.4% | -36.6% | -36.4% | +293.2% |
| 6 | 1829ns | -28.5% | -28.8% | -33.9% | +290.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.221 | moderate+ |
| mw_chain_c_mw8_uni | -0.545 | HIGH- (thermal bounce) |
| mw_chain_rev_c_mw8_uni | -0.395 | moderate- |
| mw_jumptable_c_mw8_uni | -0.208 | moderate- |
| mw_predicate_all_c_mw8_uni | -0.563 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mw_chain_c_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_uni**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 5.1ns | 2074.1ns | 0.2% |  |
| mw_chain_c_mw8_uni | 3.7ns | 1312.6ns | 0.3% |  |
| mw_chain_rev_c_mw8_uni | 2.4ns | 1317.3ns | 0.2% |  |
| mw_jumptable_c_mw8_uni | 4.6ns | 1329.2ns | 0.3% |  |
| mw_predicate_all_c_mw8_uni | 3.9ns | 7644.7ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_uni (n=6, range 1829.2-2277.9 ns)
   1829.2 |########################################
   1851.6 |
   1874.1 |
   1896.5 |
   1918.9 |
   1941.4 |
   1963.8 |
   1986.2 |
   2008.7 |
   2031.1 |####################
   2053.5 |
   2076.0 |
   2098.4 |
   2120.9 |
   2143.3 |
   2165.7 |########################################
   2188.2 |
   2210.6 |
   2233.0 |
   2255.5 |
  (0 below, 1 above range)

mw_chain_c_mw8_uni (n=6, range 1169.6-1438.7 ns)
   1169.6 |####################
   1183.1 |
   1196.5 |####################
   1210.0 |
   1223.4 |
   1236.9 |
   1250.3 |
   1263.8 |
   1277.2 |
   1290.7 |
   1304.2 |########################################
   1317.6 |
   1331.1 |
   1344.5 |
   1358.0 |
   1371.4 |
   1384.9 |####################
   1398.3 |
   1411.8 |
   1425.2 |
  (0 below, 1 above range)

mw_chain_rev_c_mw8_uni (n=6, range 1147.9-1449.2 ns)
   1147.9 |####################
   1163.0 |####################
   1178.0 |
   1193.1 |
   1208.2 |
   1223.2 |
   1238.3 |
   1253.3 |
   1268.4 |
   1283.5 |
   1298.5 |####################
   1313.6 |
   1328.7 |
   1343.7 |
   1358.8 |
   1373.8 |
   1388.9 |########################################
   1404.0 |
   1419.0 |
   1434.1 |
  (0 below, 1 above range)

mw_jumptable_c_mw8_uni (n=6, range 1168.8-1497.9 ns)
   1168.8 |########################################
   1185.3 |
   1201.7 |########################################
   1218.2 |
   1234.6 |
   1251.1 |
   1267.5 |
   1284.0 |########################################
   1300.4 |########################################
   1316.9 |
   1333.3 |
   1349.8 |
   1366.3 |
   1382.7 |
   1399.2 |
   1415.6 |
   1432.1 |
   1448.5 |
   1465.0 |
   1481.4 |########################################
  (0 below, 1 above range)

mw_predicate_all_c_mw8_uni (n=6, range 7136.7-8606.9 ns)
   7136.7 |########################################
   7210.2 |#############
   7283.7 |
   7357.2 |
   7430.7 |
   7504.2 |
   7577.8 |
   7651.3 |
   7724.8 |
   7798.3 |
   7871.8 |
   7945.3 |
   8018.8 |
   8092.3 |
   8165.8 |
   8239.4 |#############
   8312.9 |
   8386.4 |
   8459.9 |
   8533.4 |
  (0 below, 1 above range)

```

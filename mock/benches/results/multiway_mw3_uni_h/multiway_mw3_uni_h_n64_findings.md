# Multiway branch strategies, heavy-arm, mw3_uni: 3-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw3_uni** at 1367.5 ns median (-6.6% vs baseline)
- Spread: 1.07x (fastest 1367.5 ns, slowest 1464.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 4265ns | 4332ns | 3309ns | 4117ns | 4964ns | base |
| mw_chain_h_mw3_uni | 4077ns | 4112ns | 3317ns | 4055ns | 4489ns | -4.41% |
| mw_chain_rev_h_mw3_uni | 3994ns | 3994ns | 3302ns | 3898ns | 4485ns | -6.34% |
| mw_jumptable_h_mw3_uni | 4319ns | 4349ns | 3311ns | 4216ns | 4978ns | +1.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 1453ns | 1128ns | 1701ns | base | 0.044 |
| mw_chain_h_mw3_uni | 1394ns | 1133ns | 1534ns | -4.06% | 0.046 |
| mw_chain_rev_h_mw3_uni | 1368ns | 1134ns | 1539ns | -5.83% | 0.047 |
| mw_jumptable_h_mw3_uni | 1487ns | 1131ns | 1758ns | +2.30% | 0.043 |

## Performance model

- Peak throughput: **0.057 Gops/s** (mw_bintree_h_mw3_uni; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.044 | 77.1% |
| mw_chain_h_mw3_uni | 0.045 | 80.1% |
| mw_chain_rev_h_mw3_uni | 0.047 | 82.5% |
| mw_jumptable_h_mw3_uni | 0.044 | 77.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_uni | 4265ns | 4265ns | base |
| mw_chain_h_mw3_uni | 4077ns | 4077ns | -4.41% |
| mw_chain_rev_h_mw3_uni | 3994ns | 3994ns | -6.34% |
| mw_jumptable_h_mw3_uni | 4319ns | 4319ns | +1.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 1464ns | base | --- | [1195, 1701] | --- | --- | --- | --- |
| mw_chain_h_mw3_uni | 1408ns | no significant difference | [-223, +45]ns | [1240, 1534] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_h_mw3_uni | 1368ns | no significant difference | [-263, +6]ns | [1199, 1539] | no | 1.0000 | 0.6875 | 0 |
| mw_jumptable_h_mw3_uni | 1462ns | no significant difference | [-6, +105]ns | [1240, 1758] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_uni | mw_chain_h_mw3_uni | mw_chain_rev_h_mw3_uni | mw_jumptable_h_mw3_uni |
|---|---|---|---|---|
| 1 | 1793ns | -24.6% | -29.3% | +6.8% |
| 2 | 1128ns | +0.4% | +0.5% | +0.3% |
| 3 | 1261ns | +6.8% | +0.3% | +7.0% |
| 4 | 1463ns | +0.2% | +0.4% | -0.1% |
| 5 | 1465ns | -0.1% | +0.2% | -0.2% |
| 6 | 1608ns | -0.3% | -0.0% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_uni | -0.170 | ok |
| mw_chain_h_mw3_uni | 0.313 | moderate+ |
| mw_chain_rev_h_mw3_uni | 0.465 | moderate+ |
| mw_jumptable_h_mw3_uni | -0.298 | moderate- |

**Consistency summary:**

- **mw_chain_h_mw3_uni**: won 2/6, lost 3/6
- **mw_chain_rev_h_mw3_uni**: won 1/6, lost 4/6
- **mw_jumptable_h_mw3_uni**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 4.0ns | 1453.2ns | 0.3% |  |
| mw_chain_h_mw3_uni | 3.1ns | 1394.2ns | 0.2% |  |
| mw_chain_rev_h_mw3_uni | 3.5ns | 1368.5ns | 0.3% |  |
| mw_jumptable_h_mw3_uni | 4.5ns | 1486.6ns | 0.3% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_uni (n=6, range 1128.3-1700.8 ns)
   1128.3 |####################
   1156.9 |
   1185.5 |
   1214.2 |
   1242.8 |####################
   1271.4 |
   1300.0 |
   1328.7 |
   1357.3 |
   1385.9 |
   1414.5 |
   1443.2 |########################################
   1471.8 |
   1500.4 |
   1529.0 |
   1557.7 |
   1586.3 |####################
   1614.9 |
   1643.5 |
   1672.2 |
  (0 below, 1 above range)

mw_chain_h_mw3_uni (n=6, range 1132.9-1534.3 ns)
   1132.9 |####################
   1153.0 |
   1173.0 |
   1193.1 |
   1213.2 |
   1233.3 |
   1253.3 |
   1273.4 |
   1293.5 |
   1313.6 |
   1333.6 |########################################
   1353.7 |
   1373.8 |
   1393.8 |
   1413.9 |
   1434.0 |
   1454.1 |########################################
   1474.1 |
   1494.2 |
   1514.3 |
  (0 below, 1 above range)

mw_chain_rev_h_mw3_uni (n=6, range 1133.8-1538.5 ns)
   1133.8 |####################
   1154.0 |
   1174.3 |
   1194.5 |
   1214.8 |
   1235.0 |
   1255.2 |########################################
   1275.5 |
   1295.7 |
   1315.9 |
   1336.2 |
   1356.4 |
   1376.7 |
   1396.9 |
   1417.1 |
   1437.4 |
   1457.6 |########################################
   1477.8 |
   1498.1 |
   1518.3 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_uni (n=6, range 1131.2-1758.1 ns)
   1131.2 |####################
   1162.5 |
   1193.9 |
   1225.2 |
   1256.6 |
   1287.9 |
   1319.3 |####################
   1350.6 |
   1382.0 |
   1413.3 |
   1444.7 |########################################
   1476.0 |
   1507.3 |
   1538.7 |
   1570.0 |####################
   1601.4 |
   1632.7 |
   1664.1 |
   1695.4 |
   1726.8 |
  (0 below, 1 above range)

```

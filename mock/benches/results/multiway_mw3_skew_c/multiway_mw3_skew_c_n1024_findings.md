# Multiway branch strategies, cheap-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_skew**

## Key findings

- **Baseline (mw_bintree_c_mw3_skew) is the fastest** at 1586.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.39x (fastest 1586.5 ns, slowest 3797.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 4178ns | 4199ns | 3573ns | 4006ns | 4739ns | base |
| mw_chain_c_mw3_skew | 4137ns | 4333ns | 3486ns | 4095ns | 4524ns | -1.00% |
| mw_chain_rev_c_mw3_skew | 4245ns | 4348ns | 3499ns | 4292ns | 4547ns | +1.59% |
| mw_jumptable_c_mw3_skew | 4138ns | 4202ns | 3494ns | 4098ns | 4519ns | -0.97% |
| mw_predicate_all_c_mw3_skew | 6198ns | 6605ns | 5025ns | 6105ns | 6924ns | +48.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 1576ns | 1346ns | 1785ns | base | 0.650 |
| mw_chain_c_mw3_skew | 1561ns | 1316ns | 1705ns | -0.92% | 0.656 |
| mw_chain_rev_c_mw3_skew | 1601ns | 1313ns | 1721ns | +1.57% | 0.640 |
| mw_jumptable_c_mw3_skew | 1562ns | 1322ns | 1704ns | -0.89% | 0.656 |
| mw_predicate_all_c_mw3_skew | 3563ns | 2888ns | 3980ns | +126.10% | 0.287 |

## Performance model

- Peak throughput: **0.780 Gops/s** (mw_chain_rev_c_mw3_skew; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.645 | 82.8% |
| mw_chain_c_mw3_skew | 0.624 | 80.1% |
| mw_chain_rev_c_mw3_skew | 0.625 | 80.1% |
| mw_jumptable_c_mw3_skew | 0.645 | 82.7% |
| mw_predicate_all_c_mw3_skew | 0.270 | 34.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_skew | 4178ns | 4178ns | base |
| mw_chain_c_mw3_skew | 4137ns | 4137ns | -1.00% |
| mw_chain_rev_c_mw3_skew | 4245ns | 4245ns | +1.59% |
| mw_jumptable_c_mw3_skew | 4138ns | 4138ns | -0.97% |
| mw_predicate_all_c_mw3_skew | 6198ns | 6198ns | +48.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 1586ns | base | --- | [1357, 1785] | --- | --- | --- | --- |
| mw_chain_c_mw3_skew | 1640ns | no significant difference | [-145, +119]ns | [1340, 1705] | no | 0.6875 | 0.6875 | 0 |
| mw_chain_rev_c_mw3_skew | 1639ns | no significant difference | [-95, +154]ns | [1442, 1721] | no | 0.6875 | 0.6875 | 0 |
| mw_jumptable_c_mw3_skew | 1587ns | no significant difference | [-197, +167]ns | [1395, 1704] | no | 0.6875 | 0.6875 | 0 |
| mw_predicate_all_c_mw3_skew | 3798ns | +2197.6ns (+138.5%) | [+1385, +2379]ns | [2911, 3980] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_skew | mw_chain_c_mw3_skew | mw_chain_rev_c_mw3_skew | mw_jumptable_c_mw3_skew | mw_predicate_all_c_mw3_skew |
|---|---|---|---|---|---|
| 1 | 1707ns | -7.7% | +1.6% | -13.8% | +69.2% |
| 2 | 1703ns | +0.1% | +0.2% | -0.0% | +123.4% |
| 3 | 1368ns | -0.3% | +14.9% | +7.3% | +177.5% |
| 4 | 1346ns | -2.2% | -2.4% | -1.8% | +118.1% |
| 5 | 1470ns | +16.0% | +7.0% | +15.9% | +158.5% |
| 6 | 1862ns | -8.5% | -8.4% | -8.5% | +123.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.144 | ok |
| mw_chain_c_mw3_skew | 0.047 | ok |
| mw_chain_rev_c_mw3_skew | 0.197 | ok |
| mw_jumptable_c_mw3_skew | -0.127 | ok |
| mw_predicate_all_c_mw3_skew | -0.190 | ok |

**Consistency summary:**

- **mw_chain_c_mw3_skew**: won 4/6, lost 1/6
- **mw_chain_rev_c_mw3_skew**: won 2/6, lost 4/6
- **mw_jumptable_c_mw3_skew**: won 3/6, lost 2/6
- **mw_predicate_all_c_mw3_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 3.9ns | 1576.0ns | 0.3% |  |
| mw_chain_c_mw3_skew | 3.3ns | 1561.5ns | 0.2% |  |
| mw_chain_rev_c_mw3_skew | 4.2ns | 1600.7ns | 0.3% |  |
| mw_jumptable_c_mw3_skew | 3.8ns | 1561.9ns | 0.2% |  |
| mw_predicate_all_c_mw3_skew | 3.9ns | 3563.2ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_skew (n=6, range 1345.8-1784.8 ns)
   1345.8 |########################################
   1367.8 |
   1389.7 |
   1411.6 |
   1433.6 |
   1455.5 |####################
   1477.5 |
   1499.5 |
   1521.4 |
   1543.3 |
   1565.3 |
   1587.2 |
   1609.2 |
   1631.1 |
   1653.1 |
   1675.0 |
   1697.0 |########################################
   1718.9 |
   1740.9 |
   1762.8 |
  (0 below, 1 above range)

mw_chain_c_mw3_skew (n=6, range 1316.2-1705.0 ns)
   1316.2 |####################
   1335.6 |
   1355.1 |####################
   1374.5 |
   1394.0 |
   1413.4 |
   1432.8 |
   1452.3 |
   1471.7 |
   1491.2 |
   1510.6 |
   1530.0 |
   1549.5 |
   1568.9 |####################
   1588.4 |
   1607.8 |
   1627.2 |
   1646.7 |
   1666.1 |
   1685.6 |########################################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_skew (n=6, range 1312.9-1720.7 ns)
   1312.9 |####################
   1333.3 |
   1353.7 |
   1374.1 |
   1394.5 |
   1414.8 |
   1435.2 |
   1455.6 |
   1476.0 |
   1496.4 |
   1516.8 |
   1537.2 |
   1557.6 |########################################
   1577.9 |
   1598.3 |
   1618.7 |
   1639.1 |
   1659.5 |
   1679.9 |
   1700.3 |########################################
  (0 below, 1 above range)

mw_jumptable_c_mw3_skew (n=6, range 1321.7-1704.4 ns)
   1321.7 |####################
   1340.8 |
   1360.0 |
   1379.1 |
   1398.2 |
   1417.4 |
   1436.5 |
   1455.6 |########################################
   1474.8 |
   1493.9 |
   1513.1 |
   1532.2 |
   1551.3 |
   1570.5 |
   1589.6 |
   1608.7 |
   1627.9 |
   1647.0 |
   1666.1 |
   1685.3 |########################################
  (0 below, 1 above range)

mw_predicate_all_c_mw3_skew (n=6, range 2888.3-3980.2 ns)
   2888.3 |##########################
   2942.9 |
   2997.5 |
   3052.1 |
   3106.7 |
   3161.3 |
   3215.9 |
   3270.5 |
   3325.1 |
   3379.7 |
   3434.3 |
   3488.9 |
   3543.5 |
   3598.1 |
   3652.7 |
   3707.3 |
   3761.9 |########################################
   3816.5 |
   3871.1 |
   3925.7 |
  (0 below, 1 above range)

```

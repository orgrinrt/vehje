# IR-driven strategy showdown: interp vs native, all strategies, same mockup IR

12 variants, 6 samples per variant.
Baseline: **ir_bintree_int**

## Key findings

- **Fastest: ir_jumptable_nat** at 86.2 ns median (-94.6% vs baseline)
- 6 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 52.21x (fastest 86.2 ns, slowest 4503.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ir_bintree_int | 3990ns | 4114ns | 3554ns | 3971ns | 4236ns | base |
| ir_bintree_nat | 2618ns | 2674ns | 2245ns | 2559ns | 2893ns | -34.38% |
| ir_branch_int | 4086ns | 4151ns | 3664ns | 4088ns | 4295ns | +2.41% |
| ir_branch_nat | 2681ns | 2701ns | 2241ns | 2633ns | 2973ns | -32.81% |
| ir_chain_rev_int | 4196ns | 4265ns | 3579ns | 4093ns | 4660ns | +5.17% |
| ir_chain_rev_nat | 2702ns | 2734ns | 2260ns | 2715ns | 2904ns | -32.28% |
| ir_jumptable_int | 4097ns | 4214ns | 3548ns | 4038ns | 4460ns | +2.70% |
| ir_jumptable_nat | 2592ns | 2673ns | 2164ns | 2530ns | 2898ns | -35.04% |
| ir_predicate_int | 7001ns | 6919ns | 6188ns | 6746ns | 7789ns | +75.46% |
| ir_predicate_nat | 2726ns | 2722ns | 2267ns | 2657ns | 3057ns | -31.69% |
| ir_profiled_int | 4032ns | 4139ns | 3578ns | 4003ns | 4303ns | +1.06% |
| ir_profiled_nat | 2616ns | 2669ns | 2496ns | 2616ns | 2674ns | -34.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ir_bintree_int | 1556ns | 1386ns | 1647ns | base | 0.041 |
| ir_bintree_nat | 87ns | 75ns | 99ns | -94.39% | 0.733 |
| ir_branch_int | 1619ns | 1442ns | 1712ns | +4.06% | 0.040 |
| ir_branch_nat | 87ns | 72ns | 98ns | -94.38% | 0.732 |
| ir_chain_rev_int | 1654ns | 1419ns | 1857ns | +6.28% | 0.039 |
| ir_chain_rev_nat | 85ns | 67ns | 94ns | -94.54% | 0.753 |
| ir_jumptable_int | 1595ns | 1388ns | 1755ns | +2.51% | 0.040 |
| ir_jumptable_nat | 83ns | 68ns | 94ns | -94.68% | 0.773 |
| ir_predicate_int | 4571ns | 4017ns | 5126ns | +193.74% | 0.014 |
| ir_predicate_nat | 121ns | 105ns | 135ns | -92.24% | 0.530 |
| ir_profiled_int | 1598ns | 1395ns | 1720ns | +2.66% | 0.040 |
| ir_profiled_nat | 86ns | 82ns | 89ns | -94.45% | 0.741 |

## Performance model

- Peak throughput: **0.960 Gops/s** (ir_chain_rev_nat; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ir_bintree_int | 0.040 | 4.1% |
| ir_bintree_nat | 0.740 | 77.2% |
| ir_branch_int | 0.039 | 4.0% |
| ir_branch_nat | 0.728 | 75.8% |
| ir_chain_rev_int | 0.038 | 4.0% |
| ir_chain_rev_nat | 0.703 | 73.3% |
| ir_jumptable_int | 0.039 | 4.1% |
| ir_jumptable_nat | 0.742 | 77.3% |
| ir_predicate_int | 0.014 | 1.5% |
| ir_predicate_nat | 0.526 | 54.8% |
| ir_profiled_int | 0.039 | 4.1% |
| ir_profiled_nat | 0.737 | 76.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ir_bintree_int | 3990ns | 3990ns | base |
| ir_bintree_nat | 2618ns | 2618ns | -34.38% |
| ir_branch_int | 4086ns | 4086ns | +2.41% |
| ir_branch_nat | 2681ns | 2681ns | -32.81% |
| ir_chain_rev_int | 4196ns | 4196ns | +5.17% |
| ir_chain_rev_nat | 2702ns | 2702ns | -32.28% |
| ir_jumptable_int | 4097ns | 4097ns | +2.70% |
| ir_jumptable_nat | 2592ns | 2592ns | -35.04% |
| ir_predicate_int | 7001ns | 7001ns | +75.46% |
| ir_predicate_nat | 2726ns | 2726ns | -31.69% |
| ir_profiled_int | 4032ns | 4032ns | +1.06% |
| ir_profiled_nat | 2616ns | 2616ns | -34.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ir_bintree_int | 1610ns | base | --- | [1412, 1647] | --- | --- | --- | --- |
| ir_bintree_nat | 86ns | -1516.2ns (-94.2%) | [-1565, -1326]ns | [76, 99] | YES | 0.0430 | 0.0313 | 0 |
| ir_branch_int | 1649ns | +48.3ns (+3.0%) | [+19, +122]ns | [1496, 1712] | YES | 0.0430 | 0.0313 | 0 |
| ir_branch_nat | 88ns | -1519.5ns (-94.4%) | [-1562, -1324]ns | [77, 98] | YES | 0.0430 | 0.0313 | 0 |
| ir_chain_rev_int | 1677ns | no significant difference | [-90, +349]ns | [1428, 1857] | no | 0.2406 | 0.2188 | 0 |
| ir_chain_rev_nat | 91ns | -1517.6ns (-94.3%) | [-1568, -1328]ns | [70, 94] | YES | 0.0430 | 0.0313 | 0 |
| ir_jumptable_int | 1624ns | no significant difference | [-155, +247]ns | [1407, 1755] | no | 0.6875 | 0.6875 | 0 |
| ir_jumptable_nat | 86ns | -1518.7ns (-94.3%) | [-1571, -1330]ns | [68, 94] | YES | 0.0430 | 0.0313 | 0 |
| ir_predicate_int | 4503ns | +2954.0ns (+183.5%) | [+2508, +3582]ns | [4084, 5126] | YES | 0.0430 | 0.0313 | 0 |
| ir_predicate_nat | 122ns | -1478.3ns (-91.8%) | [-1522, -1306]ns | [106, 135] | YES | 0.0430 | 0.0313 | 0 |
| ir_profiled_int | 1635ns | no significant difference | [-26, +110]ns | [1437, 1720] | no | 0.2406 | 0.2188 | 0 |
| ir_profiled_nat | 87ns | -1522.7ns (-94.6%) | [-1559, -1328]ns | [84, 89] | YES | 0.0430 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ir_bintree_int | ir_bintree_nat | ir_branch_int | ir_branch_nat | ir_chain_rev_int | ir_chain_rev_nat | ir_jumptable_int | ir_jumptable_nat | ir_predicate_int | ir_predicate_nat | ir_profiled_int | ir_profiled_nat |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1437ns | -93.2% | +0.3% | -92.9% | +26.8% | -93.4% | +17.9% | -93.4% | +242.1% | -92.7% | +3.0% | -94.3% |
| 2 | 1650ns | -95.3% | +3.1% | -95.1% | +2.2% | -96.0% | -15.9% | -95.9% | +223.3% | -92.1% | -3.7% | -94.6% |
| 3 | 1642ns | -94.8% | +2.0% | -94.7% | -12.5% | -94.4% | -2.9% | -95.0% | +152.9% | -92.9% | +2.4% | -94.6% |
| 4 | 1643ns | -94.6% | +4.9% | -94.6% | +1.5% | -94.5% | +0.6% | -94.3% | +152.6% | -92.3% | +4.3% | -94.7% |
| 5 | 1386ns | -94.6% | +11.8% | -94.8% | +2.4% | -94.8% | +2.9% | -95.1% | +189.8% | -92.3% | +0.6% | -93.8% |
| 6 | 1578ns | -93.6% | +2.9% | -94.1% | +19.8% | -94.1% | +15.1% | -94.3% | +207.7% | -91.2% | +9.4% | -94.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ir_bintree_int | -0.211 | moderate- |
| ir_bintree_nat | -0.474 | moderate- |
| ir_branch_int | -0.206 | moderate- |
| ir_branch_nat | -0.384 | moderate- |
| ir_chain_rev_int | -0.337 | moderate- |
| ir_chain_rev_nat | -0.600 | HIGH- (thermal bounce) |
| ir_jumptable_int | -0.505 | HIGH- (thermal bounce) |
| ir_jumptable_nat | -0.586 | HIGH- (thermal bounce) |
| ir_predicate_int | 0.135 | ok |
| ir_predicate_nat | -0.603 | HIGH- (thermal bounce) |
| ir_profiled_int | -0.432 | moderate- |
| ir_profiled_nat | 0.012 | ok |

**Consistency summary:**

- **ir_bintree_nat**: won 6/6, lost 0/6
- **ir_branch_int**: won 0/6, lost 6/6
- **ir_branch_nat**: won 6/6, lost 0/6
- **ir_chain_rev_int**: won 1/6, lost 5/6
- **ir_chain_rev_nat**: won 6/6, lost 0/6
- **ir_jumptable_int**: won 2/6, lost 4/6
- **ir_jumptable_nat**: won 6/6, lost 0/6
- **ir_predicate_int**: won 0/6, lost 6/6
- **ir_predicate_nat**: won 6/6, lost 0/6
- **ir_profiled_int**: won 1/6, lost 5/6
- **ir_profiled_nat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ir_bintree_int | 4.0ns | 1556.1ns | 0.3% |  |
| ir_bintree_nat | 3.5ns | 87.3ns | 4.0% |  |
| ir_branch_int | 3.3ns | 1619.2ns | 0.2% |  |
| ir_branch_nat | 2.9ns | 87.4ns | 3.4% |  |
| ir_chain_rev_int | 4.3ns | 1653.8ns | 0.3% |  |
| ir_chain_rev_nat | 3.3ns | 85.0ns | 3.9% |  |
| ir_jumptable_int | 3.3ns | 1595.2ns | 0.2% |  |
| ir_jumptable_nat | 3.4ns | 82.8ns | 4.2% |  |
| ir_predicate_int | 4.4ns | 4570.9ns | 0.1% |  |
| ir_predicate_nat | 29.8ns | 120.8ns | 24.7% | HIGH |
| ir_profiled_int | 2.3ns | 1597.6ns | 0.1% |  |
| ir_profiled_nat | 2.5ns | 86.4ns | 2.9% |  |

## Distribution (algo ns)

```
ir_bintree_int (n=6, range 1386.2-1646.8 ns)
   1386.2 |####################
   1399.2 |
   1412.3 |
   1425.3 |####################
   1438.3 |
   1451.4 |
   1464.4 |
   1477.4 |
   1490.5 |
   1503.5 |
   1516.5 |
   1529.6 |
   1542.6 |
   1555.6 |
   1568.7 |####################
   1581.7 |
   1594.7 |
   1607.8 |
   1620.8 |
   1633.8 |########################################
  (0 below, 1 above range)

ir_bintree_nat (n=6, range 74.6-99.0 ns)
     74.6 |########################################
     75.8 |
     77.0 |
     78.3 |########################################
     79.5 |
     80.7 |
     81.9 |
     83.1 |
     84.3 |########################################
     85.6 |
     86.8 |
     88.0 |########################################
     89.2 |
     90.4 |
     91.6 |
     92.9 |
     94.1 |
     95.3 |
     96.5 |########################################
     97.7 |
  (0 below, 1 above range)

ir_branch_int (n=6, range 1442.1-1712.5 ns)
   1442.1 |########################################
   1455.6 |
   1469.1 |
   1482.7 |
   1496.2 |
   1509.7 |
   1523.2 |
   1536.7 |########################################
   1550.2 |
   1563.8 |
   1577.3 |
   1590.8 |
   1604.3 |
   1617.8 |########################################
   1631.3 |
   1644.9 |
   1658.4 |
   1671.9 |########################################
   1685.4 |
   1698.9 |########################################
  (0 below, 1 above range)

ir_branch_nat (n=6, range 72.5-97.5 ns)
     72.5 |########################################
     73.8 |
     75.0 |
     76.2 |
     77.5 |
     78.8 |
     80.0 |########################################
     81.2 |
     82.5 |
     83.8 |
     85.0 |
     86.2 |########################################
     87.5 |
     88.8 |########################################
     90.0 |
     91.2 |
     92.5 |########################################
     93.8 |
     95.0 |
     96.2 |
  (0 below, 1 above range)

ir_chain_rev_int (n=6, range 1419.2-1856.7 ns)
   1419.2 |########################################
   1441.1 |
   1462.9 |
   1484.8 |
   1506.7 |
   1528.6 |
   1550.4 |
   1572.3 |
   1594.2 |
   1616.1 |
   1637.9 |
   1659.8 |####################
   1681.7 |####################
   1703.5 |
   1725.4 |
   1747.3 |
   1769.2 |
   1791.0 |
   1812.9 |####################
   1834.8 |
  (0 below, 1 above range)

ir_chain_rev_nat (n=6, range 66.7-94.3 ns)
     66.7 |####################
     68.1 |
     69.5 |
     70.8 |
     72.2 |####################
     73.6 |
     75.0 |
     76.4 |
     77.8 |
     79.1 |
     80.5 |
     81.9 |
     83.3 |
     84.7 |
     86.1 |
     87.4 |
     88.8 |
     90.2 |########################################
     91.6 |
     93.0 |####################
  (0 below, 1 above range)

ir_jumptable_int (n=6, range 1387.5-1754.8 ns)
   1387.5 |########################################
   1405.9 |
   1424.2 |########################################
   1442.6 |
   1461.0 |
   1479.3 |
   1497.7 |
   1516.0 |
   1534.4 |
   1552.8 |
   1571.1 |
   1589.5 |########################################
   1607.8 |
   1626.2 |
   1644.6 |########################################
   1662.9 |
   1681.3 |########################################
   1699.7 |
   1718.0 |
   1736.4 |
  (0 below, 1 above range)

ir_jumptable_nat (n=6, range 67.5-94.4 ns)
     67.5 |########################################
     68.8 |
     70.2 |
     71.5 |
     72.9 |
     74.2 |
     75.6 |
     76.9 |
     78.3 |
     79.6 |
     81.0 |
     82.3 |####################
     83.6 |
     85.0 |
     86.3 |
     87.7 |
     89.0 |####################
     90.4 |
     91.7 |
     93.1 |####################
  (0 below, 1 above range)

ir_predicate_int (n=6, range 4016.7-5125.8 ns)
   4016.7 |####################
   4072.2 |
   4127.6 |########################################
   4183.1 |
   4238.5 |
   4294.0 |
   4349.4 |
   4404.9 |
   4460.3 |
   4515.8 |
   4571.2 |
   4626.7 |
   4682.2 |
   4737.6 |
   4793.1 |
   4848.5 |####################
   4904.0 |####################
   4959.4 |
   5014.9 |
   5070.3 |
  (0 below, 1 above range)

ir_predicate_nat (n=6, range 105.4-134.6 ns)
    105.4 |########################################
    106.9 |
    108.3 |
    109.8 |
    111.2 |
    112.7 |
    114.1 |
    115.6 |
    117.1 |####################
    118.5 |
    120.0 |
    121.4 |
    122.9 |
    124.3 |
    125.8 |####################
    127.3 |
    128.7 |
    130.2 |####################
    131.6 |
    133.1 |
  (0 below, 1 above range)

ir_profiled_int (n=6, range 1395.0-1720.2 ns)
   1395.0 |########################################
   1411.3 |
   1427.5 |
   1443.8 |
   1460.0 |
   1476.3 |########################################
   1492.6 |
   1508.8 |
   1525.1 |
   1541.3 |
   1557.6 |
   1573.9 |########################################
   1590.1 |
   1606.4 |
   1622.6 |
   1638.9 |
   1655.2 |
   1671.4 |########################################
   1687.7 |
   1703.9 |########################################
  (0 below, 1 above range)

ir_profiled_nat (n=6, range 82.1-88.8 ns)
     82.1 |########################################
     82.4 |
     82.8 |
     83.1 |
     83.4 |
     83.8 |
     84.1 |
     84.4 |
     84.8 |########################################
     85.1 |
     85.4 |
     85.8 |
     86.1 |########################################
     86.4 |
     86.8 |
     87.1 |
     87.4 |########################################
     87.8 |
     88.1 |########################################
     88.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **ir_predicate_nat**: bridge=24.3% of algo (FFI overhead may distort results)

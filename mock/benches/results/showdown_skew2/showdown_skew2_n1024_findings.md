# Strategy showdown (skew2): all strategies x tiers (native/interp) same footing

12 variants, 6 samples per variant.
Baseline: **sd_bintree_int_skew2**

## Key findings

- **Fastest: sd_chain_nat_skew2** at 1564.3 ns median (-84.9% vs baseline)
- 9 variants significantly faster than baseline
- Spread: 6.92x (fastest 1564.3 ns, slowest 10824.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 13018ns | 12944ns | 10950ns | 12912ns | 14209ns | base |
| sd_bintree_nat_skew2 | 4243ns | 4266ns | 3745ns | 4141ns | 4646ns | -67.40% |
| sd_chain_int_skew2 | 13398ns | 13125ns | 12189ns | 12971ns | 14644ns | +2.92% |
| sd_chain_nat_skew2 | 3960ns | 3974ns | 3562ns | 3841ns | 4336ns | -69.58% |
| sd_chain_rev_int_skew2 | 12479ns | 12612ns | 10972ns | 12371ns | 13396ns | -4.13% |
| sd_chain_rev_nat_skew2 | 4180ns | 4285ns | 3632ns | 4098ns | 4575ns | -67.89% |
| sd_evalall_int_skew2 | 8391ns | 8545ns | 7502ns | 8200ns | 9122ns | -35.54% |
| sd_evalall_nat_skew2 | 8565ns | 8766ns | 7457ns | 8467ns | 9265ns | -34.21% |
| sd_jumptable_int_skew2 | 4058ns | 4119ns | 3628ns | 3963ns | 4415ns | -68.83% |
| sd_jumptable_nat_skew2 | 4113ns | 4140ns | 3575ns | 4092ns | 4413ns | -68.40% |
| sd_profiled_int_skew2 | 11736ns | 12043ns | 10688ns | 11667ns | 12363ns | -9.85% |
| sd_profiled_nat_skew2 | 4030ns | 4014ns | 3664ns | 3906ns | 4399ns | -69.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 10419ns | 8766ns | 11383ns | base | 0.098 |
| sd_bintree_nat_skew2 | 1754ns | 1575ns | 1954ns | -83.16% | 0.584 |
| sd_chain_int_skew2 | 11037ns | 10058ns | 12058ns | +5.93% | 0.093 |
| sd_chain_nat_skew2 | 1566ns | 1371ns | 1756ns | -84.97% | 0.654 |
| sd_chain_rev_int_skew2 | 10017ns | 8808ns | 10705ns | -3.85% | 0.102 |
| sd_chain_rev_nat_skew2 | 1635ns | 1467ns | 1804ns | -84.31% | 0.626 |
| sd_evalall_int_skew2 | 5949ns | 5315ns | 6492ns | -42.90% | 0.172 |
| sd_evalall_nat_skew2 | 6065ns | 5276ns | 6574ns | -41.79% | 0.169 |
| sd_jumptable_int_skew2 | 1578ns | 1383ns | 1723ns | -84.86% | 0.649 |
| sd_jumptable_nat_skew2 | 1610ns | 1385ns | 1718ns | -84.54% | 0.636 |
| sd_profiled_int_skew2 | 9278ns | 8464ns | 9781ns | -10.95% | 0.110 |
| sd_profiled_nat_skew2 | 1623ns | 1408ns | 1813ns | -84.42% | 0.631 |

## Performance model

- Peak throughput: **0.747 Gops/s** (sd_chain_nat_skew2; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sd_bintree_int_skew2 | 0.099 | 13.2% |
| sd_bintree_nat_skew2 | 0.602 | 80.6% |
| sd_chain_int_skew2 | 0.095 | 12.7% |
| sd_chain_nat_skew2 | 0.655 | 87.7% |
| sd_chain_rev_int_skew2 | 0.101 | 13.6% |
| sd_chain_rev_nat_skew2 | 0.628 | 84.1% |
| sd_evalall_int_skew2 | 0.170 | 22.8% |
| sd_evalall_nat_skew2 | 0.166 | 22.2% |
| sd_jumptable_int_skew2 | 0.635 | 85.0% |
| sd_jumptable_nat_skew2 | 0.628 | 84.0% |
| sd_profiled_int_skew2 | 0.108 | 14.4% |
| sd_profiled_nat_skew2 | 0.630 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sd_bintree_int_skew2 | 13018ns | 13018ns | base |
| sd_bintree_nat_skew2 | 4243ns | 4243ns | -67.40% |
| sd_chain_int_skew2 | 13398ns | 13398ns | +2.92% |
| sd_chain_nat_skew2 | 3960ns | 3960ns | -69.58% |
| sd_chain_rev_int_skew2 | 12479ns | 12479ns | -4.13% |
| sd_chain_rev_nat_skew2 | 4180ns | 4180ns | -67.89% |
| sd_evalall_int_skew2 | 8391ns | 8391ns | -35.54% |
| sd_evalall_nat_skew2 | 8565ns | 8565ns | -34.21% |
| sd_jumptable_int_skew2 | 4058ns | 4058ns | -68.83% |
| sd_jumptable_nat_skew2 | 4113ns | 4113ns | -68.40% |
| sd_profiled_int_skew2 | 11736ns | 11736ns | -9.85% |
| sd_profiled_nat_skew2 | 4030ns | 4030ns | -69.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 10359ns | base | --- | [9514, 11383] | --- | --- | --- | --- |
| sd_bintree_nat_skew2 | 1702ns | -8489.4ns (-81.9%) | [-9776, -7728]ns | [1607, 1954] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_int_skew2 | 10824ns | no significant difference | [-1155, +1723]ns | [10229, 12058] | no | 0.7563 | 0.6875 | 0 |
| sd_chain_nat_skew2 | 1564ns | -8678.4ns (-83.8%) | [-9936, -7944]ns | [1378, 1756] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_rev_int_skew2 | 10118ns | no significant difference | [-1989, +838]ns | [9230, 10705] | no | 1.0000 | 1.0000 | 0 |
| sd_chain_rev_nat_skew2 | 1630ns | -8638.6ns (-83.4%) | [-9769, -7946]ns | [1470, 1804] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_int_skew2 | 6027ns | -4104.9ns (-39.6%) | [-5639, -3665]ns | [5329, 6492] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_nat_skew2 | 6170ns | -4112.2ns (-39.7%) | [-5487, -3463]ns | [5451, 6574] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_int_skew2 | 1614ns | -8669.8ns (-83.7%) | [-9909, -7945]ns | [1395, 1723] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_nat_skew2 | 1631ns | -8746.1ns (-84.4%) | [-9665, -8014]ns | [1481, 1718] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_int_skew2 | 9489ns | -684.4ns (-6.6%) | [-2455, -283]ns | [8564, 9781] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_nat_skew2 | 1626ns | -8564.2ns (-82.7%) | [-9903, -7920]ns | [1431, 1813] | YES | 0.0382 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sd_bintree_int_skew2 | sd_bintree_nat_skew2 | sd_chain_int_skew2 | sd_chain_nat_skew2 | sd_chain_rev_int_skew2 | sd_chain_rev_nat_skew2 | sd_evalall_int_skew2 | sd_evalall_nat_skew2 | sd_jumptable_int_skew2 | sd_jumptable_nat_skew2 | sd_profiled_int_skew2 | sd_profiled_nat_skew2 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 10492ns | -84.4% | -4.1% | -85.6% | -4.9% | -83.2% | -37.0% | -40.7% | -86.8% | -84.3% | -6.9% | -86.1% |
| 2 | 12275ns | -87.2% | -15.3% | -88.7% | -28.2% | -88.1% | -56.5% | -54.2% | -87.2% | -85.5% | -31.0% | -87.7% |
| 3 | 10262ns | -82.1% | +8.8% | -82.8% | +0.8% | -83.8% | -37.9% | -38.5% | -83.8% | -84.6% | -4.5% | -82.7% |
| 4 | 8766ns | -80.8% | +19.5% | -84.4% | +10.1% | -83.2% | -39.4% | -39.8% | -83.9% | -84.2% | -1.2% | -83.9% |
| 5 | 10447ns | -80.1% | +15.9% | -83.3% | -1.8% | -82.3% | -41.4% | -41.4% | -83.7% | -84.3% | -10.5% | -82.3% |
| 6 | 10272ns | -83.2% | +16.9% | -84.2% | +7.7% | -84.5% | -42.3% | -33.4% | -83.1% | -84.2% | -6.2% | -83.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sd_bintree_int_skew2 | 0.009 | ok |
| sd_bintree_nat_skew2 | -0.203 | moderate- |
| sd_chain_int_skew2 | 0.240 | moderate+ |
| sd_chain_nat_skew2 | -0.612 | HIGH- (thermal bounce) |
| sd_chain_rev_int_skew2 | -0.109 | ok |
| sd_chain_rev_nat_skew2 | -0.625 | HIGH- (thermal bounce) |
| sd_evalall_int_skew2 | -0.735 | HIGH- (thermal bounce) |
| sd_evalall_nat_skew2 | -0.247 | moderate- |
| sd_jumptable_int_skew2 | -0.119 | ok |
| sd_jumptable_nat_skew2 | 0.035 | ok |
| sd_profiled_int_skew2 | -0.690 | HIGH- (thermal bounce) |
| sd_profiled_nat_skew2 | -0.299 | moderate- |

**Consistency summary:**

- **sd_bintree_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_int_skew2**: won 2/6, lost 4/6
- **sd_chain_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_rev_int_skew2**: won 3/6, lost 3/6
- **sd_chain_rev_nat_skew2**: won 6/6, lost 0/6
- **sd_evalall_int_skew2**: won 6/6, lost 0/6
- **sd_evalall_nat_skew2**: won 6/6, lost 0/6
- **sd_jumptable_int_skew2**: won 6/6, lost 0/6
- **sd_jumptable_nat_skew2**: won 6/6, lost 0/6
- **sd_profiled_int_skew2**: won 6/6, lost 0/6
- **sd_profiled_nat_skew2**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sd_bintree_int_skew2 | 4.4ns | 10418.9ns | 0.0% |  |
| sd_bintree_nat_skew2 | 3.5ns | 1754.2ns | 0.2% |  |
| sd_chain_int_skew2 | 3.1ns | 11037.0ns | 0.0% |  |
| sd_chain_nat_skew2 | 2.8ns | 1566.1ns | 0.2% |  |
| sd_chain_rev_int_skew2 | 4.3ns | 10017.5ns | 0.0% |  |
| sd_chain_rev_nat_skew2 | 3.1ns | 1634.6ns | 0.2% |  |
| sd_evalall_int_skew2 | 3.1ns | 5949.3ns | 0.1% |  |
| sd_evalall_nat_skew2 | 4.6ns | 6065.1ns | 0.1% |  |
| sd_jumptable_int_skew2 | 3.2ns | 1577.5ns | 0.2% |  |
| sd_jumptable_nat_skew2 | 2.5ns | 1610.3ns | 0.2% |  |
| sd_profiled_int_skew2 | 2.9ns | 9278.1ns | 0.0% |  |
| sd_profiled_nat_skew2 | 3.0ns | 1623.1ns | 0.2% |  |

## Distribution (algo ns)

```
sd_bintree_int_skew2 (n=6, range 8766.2-11383.4 ns)
   8766.2 |####################
   8897.1 |
   9027.9 |
   9158.8 |
   9289.6 |
   9420.5 |
   9551.3 |
   9682.2 |
   9813.1 |
   9943.9 |
  10074.8 |
  10205.6 |########################################
  10336.5 |####################
  10467.3 |####################
  10598.2 |
  10729.1 |
  10859.9 |
  10990.8 |
  11121.6 |
  11252.5 |
  (0 below, 1 above range)

sd_bintree_nat_skew2 (n=6, range 1575.0-1954.1 ns)
   1575.0 |########################################
   1594.0 |
   1612.9 |
   1631.9 |########################################
   1650.8 |
   1669.8 |########################################
   1688.7 |
   1707.7 |########################################
   1726.7 |
   1745.6 |
   1764.6 |
   1783.5 |
   1802.5 |
   1821.4 |########################################
   1840.4 |
   1859.4 |
   1878.3 |
   1897.3 |
   1916.2 |
   1935.2 |
  (0 below, 1 above range)

sd_chain_int_skew2 (n=6, range 10057.5-12058.5 ns)
  10057.5 |########################################
  10157.5 |
  10257.6 |
  10357.6 |########################################
  10457.7 |########################################
  10557.8 |
  10657.8 |
  10757.9 |
  10857.9 |
  10958.0 |
  11058.0 |
  11158.0 |########################################
  11258.1 |
  11358.1 |
  11458.2 |
  11558.2 |
  11658.3 |
  11758.4 |
  11858.4 |
  11958.5 |########################################
  (0 below, 1 above range)

sd_chain_nat_skew2 (n=6, range 1371.2-1755.8 ns)
   1371.2 |########################################
   1390.4 |
   1409.7 |
   1428.9 |
   1448.1 |
   1467.4 |
   1486.6 |
   1505.8 |####################
   1525.1 |
   1544.3 |
   1563.5 |
   1582.8 |
   1602.0 |####################
   1621.2 |
   1640.5 |
   1659.7 |
   1678.9 |
   1698.2 |
   1717.4 |
   1736.6 |####################
  (0 below, 1 above range)

sd_chain_rev_int_skew2 (n=6, range 8807.9-10704.5 ns)
   8807.9 |########################################
   8902.7 |
   8997.6 |
   9092.4 |
   9187.2 |
   9282.1 |
   9376.9 |
   9471.7 |
   9566.6 |########################################
   9661.4 |
   9756.2 |
   9851.1 |
   9945.9 |########################################
  10040.7 |
  10135.6 |
  10230.4 |########################################
  10325.2 |########################################
  10420.1 |
  10514.9 |
  10609.7 |
  (0 below, 1 above range)

sd_chain_rev_nat_skew2 (n=6, range 1466.7-1804.2 ns)
   1466.7 |########################################
   1483.6 |
   1500.4 |
   1517.3 |
   1534.2 |
   1551.1 |
   1567.9 |
   1584.8 |####################
   1601.7 |
   1618.6 |
   1635.4 |
   1652.3 |####################
   1669.2 |
   1686.0 |
   1702.9 |
   1719.8 |
   1736.7 |
   1753.5 |####################
   1770.4 |
   1787.3 |
  (0 below, 1 above range)

sd_evalall_int_skew2 (n=6, range 5315.4-6492.3 ns)
   5315.4 |########################################
   5374.2 |
   5433.1 |
   5491.9 |
   5550.8 |
   5609.6 |
   5668.5 |
   5727.3 |
   5786.2 |
   5845.0 |
   5903.8 |####################
   5962.7 |
   6021.5 |
   6080.4 |####################
   6139.2 |
   6198.1 |
   6256.9 |
   6315.8 |####################
   6374.6 |
   6433.5 |
  (0 below, 1 above range)

sd_evalall_nat_skew2 (n=6, range 5275.8-6574.4 ns)
   5275.8 |########################################
   5340.7 |
   5405.7 |
   5470.6 |
   5535.5 |
   5600.4 |########################################
   5665.4 |
   5730.3 |
   5795.2 |
   5860.1 |
   5925.1 |
   5990.0 |
   6054.9 |
   6119.9 |########################################
   6184.8 |########################################
   6249.7 |########################################
   6314.6 |
   6379.6 |
   6444.5 |
   6509.4 |
  (0 below, 1 above range)

sd_jumptable_int_skew2 (n=6, range 1382.9-1723.3 ns)
   1382.9 |########################################
   1399.9 |########################################
   1416.9 |
   1434.0 |
   1451.0 |
   1468.0 |
   1485.0 |
   1502.1 |
   1519.1 |
   1536.1 |
   1553.1 |########################################
   1570.1 |
   1587.2 |
   1604.2 |
   1621.2 |
   1638.2 |
   1655.3 |########################################
   1672.3 |
   1689.3 |
   1706.3 |########################################
  (0 below, 1 above range)

sd_jumptable_nat_skew2 (n=6, range 1385.4-1717.9 ns)
   1385.4 |########################################
   1402.0 |
   1418.7 |
   1435.3 |
   1451.9 |
   1468.5 |
   1485.2 |
   1501.8 |
   1518.4 |
   1535.0 |
   1551.7 |
   1568.3 |########################################
   1584.9 |
   1601.6 |
   1618.2 |########################################
   1634.8 |########################################
   1651.4 |########################################
   1668.1 |
   1684.7 |
   1701.3 |
  (0 below, 1 above range)

sd_profiled_int_skew2 (n=6, range 8464.2-9781.0 ns)
   8464.2 |########################################
   8530.0 |
   8595.9 |
   8661.7 |########################################
   8727.6 |
   8793.4 |
   8859.2 |
   8925.1 |
   8990.9 |
   9056.8 |
   9122.6 |
   9188.4 |
   9254.3 |
   9320.1 |########################################
   9386.0 |
   9451.8 |
   9517.6 |
   9583.5 |########################################
   9649.3 |
   9715.2 |########################################
  (0 below, 1 above range)

sd_profiled_nat_skew2 (n=6, range 1407.9-1812.7 ns)
   1407.9 |########################################
   1428.1 |
   1448.4 |########################################
   1468.6 |
   1488.9 |########################################
   1509.1 |
   1529.3 |
   1549.6 |
   1569.8 |
   1590.1 |
   1610.3 |
   1630.5 |
   1650.8 |
   1671.0 |
   1691.3 |
   1711.5 |
   1731.7 |########################################
   1752.0 |
   1772.2 |########################################
   1792.5 |
  (0 below, 1 above range)

```

# Multiway branch strategies, heavy-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_skew**

## Key findings

- **Fastest: mw_chain_h_mw3_skew** at 1288.0 ns median (-3.3% vs baseline)
- Spread: 1.04x (fastest 1288.0 ns, slowest 1336.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 4139ns | 4037ns | 3244ns | 3780ns | 5125ns | base |
| mw_chain_h_mw3_skew | 4514ns | 3904ns | 3245ns | 3729ns | 6326ns | +9.07% |
| mw_chain_rev_h_mw3_skew | 4163ns | 4044ns | 3248ns | 3906ns | 5006ns | +0.58% |
| mw_jumptable_h_mw3_skew | 4511ns | 4042ns | 3241ns | 3778ns | 6247ns | +9.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 1365ns | 1072ns | 1688ns | base | 0.047 |
| mw_chain_h_mw3_skew | 1482ns | 1069ns | 2068ns | +8.58% | 0.043 |
| mw_chain_rev_h_mw3_skew | 1374ns | 1073ns | 1654ns | +0.66% | 0.047 |
| mw_jumptable_h_mw3_skew | 1488ns | 1066ns | 2061ns | +9.03% | 0.043 |

## Performance model

- Peak throughput: **0.060 Gops/s** (mw_jumptable_h_mw3_skew; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.048 | 80.0% |
| mw_chain_h_mw3_skew | 0.050 | 82.8% |
| mw_chain_rev_h_mw3_skew | 0.048 | 79.9% |
| mw_jumptable_h_mw3_skew | 0.048 | 79.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_skew | 4139ns | 4139ns | base |
| mw_chain_h_mw3_skew | 4514ns | 4514ns | +9.07% |
| mw_chain_rev_h_mw3_skew | 4163ns | 4163ns | +0.58% |
| mw_jumptable_h_mw3_skew | 4511ns | 4511ns | +9.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 1332ns | base | --- | [1074, 1688] | --- | --- | --- | --- |
| mw_chain_h_mw3_skew | 1288ns | no significant difference | [-292, +587]ns | [1089, 2068] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_h_mw3_skew | 1333ns | no significant difference | [-190, +214]ns | [1134, 1654] | no | 1.0000 | 0.6875 | 0 |
| mw_jumptable_h_mw3_skew | 1336ns | no significant difference | [-206, +580]ns | [1067, 2061] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_skew | mw_chain_h_mw3_skew | mw_chain_rev_h_mw3_skew | mw_jumptable_h_mw3_skew |
|---|---|---|---|---|
| 1 | 1275ns | +74.4% | -6.2% | +73.3% |
| 2 | 1688ns | +13.4% | +13.5% | +13.3% |
| 3 | 1689ns | -34.3% | -17.8% | -24.0% |
| 4 | 1072ns | -0.2% | +0.1% | -0.6% |
| 5 | 1077ns | +10.6% | +18.6% | -0.8% |
| 6 | 1389ns | -0.4% | +0.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.150 | ok |
| mw_chain_h_mw3_skew | 0.405 | moderate+ |
| mw_chain_rev_h_mw3_skew | -0.155 | ok |
| mw_jumptable_h_mw3_skew | 0.474 | moderate+ |

**Consistency summary:**

- **mw_chain_h_mw3_skew**: won 3/6, lost 3/6
- **mw_chain_rev_h_mw3_skew**: won 2/6, lost 4/6
- **mw_jumptable_h_mw3_skew**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 3.8ns | 1364.8ns | 0.3% |  |
| mw_chain_h_mw3_skew | 5.0ns | 1481.9ns | 0.3% |  |
| mw_chain_rev_h_mw3_skew | 3.6ns | 1373.8ns | 0.3% |  |
| mw_jumptable_h_mw3_skew | 4.6ns | 1488.1ns | 0.3% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_skew (n=6, range 1071.7-1688.2 ns)
   1071.7 |########################################
   1102.5 |
   1133.3 |
   1164.2 |
   1195.0 |
   1225.8 |
   1256.6 |####################
   1287.5 |
   1318.3 |
   1349.1 |
   1379.9 |####################
   1410.7 |
   1441.6 |
   1472.4 |
   1503.2 |
   1534.0 |
   1564.9 |
   1595.7 |
   1626.5 |
   1657.3 |####################
  (0 below, 1 above range)

mw_chain_h_mw3_skew (n=6, range 1069.2-2068.3 ns)
   1069.2 |########################################
   1119.2 |
   1169.1 |####################
   1219.1 |
   1269.0 |
   1319.0 |
   1368.9 |####################
   1418.9 |
   1468.8 |
   1518.8 |
   1568.8 |
   1618.7 |
   1668.7 |
   1718.6 |
   1768.6 |
   1818.5 |
   1868.5 |####################
   1918.4 |
   1968.4 |
   2018.3 |
  (0 below, 1 above range)

mw_chain_rev_h_mw3_skew (n=6, range 1072.9-1654.2 ns)
   1072.9 |####################
   1102.0 |
   1131.0 |
   1160.1 |
   1189.2 |####################
   1218.2 |
   1247.3 |
   1276.3 |####################
   1305.4 |
   1334.5 |
   1363.5 |########################################
   1392.6 |
   1421.7 |
   1450.7 |
   1479.8 |
   1508.8 |
   1537.9 |
   1567.0 |
   1596.0 |
   1625.1 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_skew (n=6, range 1065.8-2060.7 ns)
   1065.8 |########################################
   1115.5 |
   1165.3 |
   1215.0 |
   1264.8 |####################
   1314.5 |
   1364.3 |####################
   1414.0 |
   1463.7 |
   1513.5 |
   1563.2 |
   1613.0 |
   1662.7 |
   1712.5 |
   1762.2 |
   1811.9 |
   1861.7 |
   1911.4 |####################
   1961.2 |
   2010.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_chain_h_mw3_skew**: CV=29.4% (high variance, measurements may be unstable)
- **mw_jumptable_h_mw3_skew**: CV=28.8% (high variance, measurements may be unstable)

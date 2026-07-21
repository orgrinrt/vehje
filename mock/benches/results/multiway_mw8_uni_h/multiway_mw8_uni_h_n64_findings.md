# Multiway branch strategies, heavy-arm, mw8_uni: 8-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw8_uni** at 1225.4 ns median (-11.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.13x (fastest 1225.4 ns, slowest 1390.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 4202ns | 4092ns | 3289ns | 3926ns | 5073ns | base |
| mw_chain_h_mw8_uni | 4103ns | 3857ns | 3212ns | 3680ns | 5184ns | -2.35% |
| mw_chain_rev_h_mw8_uni | 3859ns | 3857ns | 3334ns | 3710ns | 4345ns | -8.16% |
| mw_jumptable_h_mw8_uni | 4091ns | 3934ns | 3206ns | 3692ns | 5132ns | -2.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 1417ns | 1113ns | 1724ns | base | 0.045 |
| mw_chain_h_mw8_uni | 1321ns | 1030ns | 1671ns | -6.75% | 0.048 |
| mw_chain_rev_h_mw8_uni | 1231ns | 1071ns | 1396ns | -13.11% | 0.052 |
| mw_jumptable_h_mw8_uni | 1313ns | 1030ns | 1649ns | -7.31% | 0.049 |

## Performance model

- Peak throughput: **0.062 Gops/s** (mw_chain_h_mw8_uni; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.046 | 74.1% |
| mw_chain_h_mw8_uni | 0.052 | 82.9% |
| mw_chain_rev_h_mw8_uni | 0.052 | 84.1% |
| mw_jumptable_h_mw8_uni | 0.051 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_uni | 4202ns | 4202ns | base |
| mw_chain_h_mw8_uni | 4103ns | 4103ns | -2.35% |
| mw_chain_rev_h_mw8_uni | 3859ns | 3859ns | -8.16% |
| mw_jumptable_h_mw8_uni | 4091ns | 4091ns | -2.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 1390ns | base | --- | [1136, 1724] | --- | --- | --- | --- |
| mw_chain_h_mw8_uni | 1242ns | -100.4ns (-7.2%) | [-170, -17]ns | [1050, 1671] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| mw_chain_rev_h_mw8_uni | 1225ns | -98.0ns (-7.0%) | [-443, -16]ns | [1071, 1396] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| mw_jumptable_h_mw8_uni | 1260ns | no significant difference | [-215, +10]ns | [1031, 1649] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_uni | mw_chain_h_mw8_uni | mw_chain_rev_h_mw8_uni | mw_jumptable_h_mw8_uni |
|---|---|---|---|---|
| 1 | 1998ns | -7.7% | -38.7% | -8.0% |
| 2 | 1447ns | -7.7% | +0.7% | +0.7% |
| 3 | 1160ns | -7.8% | -7.7% | -11.1% |
| 4 | 1113ns | -7.4% | -3.7% | -7.4% |
| 5 | 1333ns | -13.9% | -8.0% | -20.2% |
| 6 | 1449ns | +3.4% | -7.8% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.219 | moderate+ |
| mw_chain_h_mw8_uni | 0.201 | moderate+ |
| mw_chain_rev_h_mw8_uni | -0.103 | ok |
| mw_jumptable_h_mw8_uni | 0.275 | moderate+ |

**Consistency summary:**

- **mw_chain_h_mw8_uni**: won 5/6, lost 1/6
- **mw_chain_rev_h_mw8_uni**: won 5/6, lost 1/6
- **mw_jumptable_h_mw8_uni**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 4.6ns | 1416.6ns | 0.3% |  |
| mw_chain_h_mw8_uni | 4.1ns | 1321.0ns | 0.3% |  |
| mw_chain_rev_h_mw8_uni | 5.1ns | 1230.9ns | 0.4% |  |
| mw_jumptable_h_mw8_uni | 3.9ns | 1313.1ns | 0.3% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_uni (n=6, range 1112.9-1723.5 ns)
   1112.9 |########################################
   1143.4 |########################################
   1174.0 |
   1204.5 |
   1235.0 |
   1265.6 |
   1296.1 |
   1326.6 |########################################
   1357.2 |
   1387.7 |
   1418.2 |########################################
   1448.8 |########################################
   1479.3 |
   1509.8 |
   1540.4 |
   1570.9 |
   1601.4 |
   1632.0 |
   1662.5 |
   1693.0 |
  (0 below, 1 above range)

mw_chain_h_mw8_uni (n=6, range 1030.0-1671.4 ns)
   1030.0 |########################################
   1062.1 |########################################
   1094.1 |
   1126.2 |########################################
   1158.3 |
   1190.4 |
   1222.4 |
   1254.5 |
   1286.6 |
   1318.7 |########################################
   1350.7 |
   1382.8 |
   1414.9 |
   1446.9 |
   1479.0 |########################################
   1511.1 |
   1543.2 |
   1575.2 |
   1607.3 |
   1639.4 |
  (0 below, 1 above range)

mw_chain_rev_h_mw8_uni (n=6, range 1070.8-1396.2 ns)
   1070.8 |########################################
   1087.1 |
   1103.3 |
   1119.6 |
   1135.9 |
   1152.2 |
   1168.4 |
   1184.7 |
   1201.0 |
   1217.3 |########################################
   1233.5 |
   1249.8 |
   1266.1 |
   1282.3 |
   1298.6 |
   1314.9 |
   1331.2 |####################
   1347.4 |
   1363.7 |
   1380.0 |
  (0 below, 1 above range)

mw_jumptable_h_mw8_uni (n=6, range 1030.4-1648.8 ns)
   1030.4 |########################################
   1061.3 |####################
   1092.2 |
   1123.2 |
   1154.1 |
   1185.0 |
   1215.9 |
   1246.8 |
   1277.7 |
   1308.7 |
   1339.6 |
   1370.5 |
   1401.4 |
   1432.3 |########################################
   1463.2 |
   1494.2 |
   1525.1 |
   1556.0 |
   1586.9 |
   1617.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_bintree_h_mw8_uni**: CV=20.5% (high variance, measurements may be unstable)
- **mw_chain_h_mw8_uni**: CV=21.5% (high variance, measurements may be unstable)
- **mw_jumptable_h_mw8_uni**: CV=22.8% (high variance, measurements may be unstable)

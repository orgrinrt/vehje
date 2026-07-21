# Multiway branch strategies, heavy-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_skew**

## Key findings

- **Fastest: mw_chain_h_mw8_skew** at 1186.7 ns median (-14.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.17x (fastest 1186.7 ns, slowest 1385.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 3847ns | 3980ns | 3335ns | 3766ns | 4225ns | base |
| mw_chain_h_mw8_skew | 3698ns | 3725ns | 3145ns | 3550ns | 4195ns | -3.88% |
| mw_chain_rev_h_mw8_skew | 3911ns | 3815ns | 3308ns | 3683ns | 4555ns | +1.67% |
| mw_jumptable_h_mw8_skew | 3788ns | 3807ns | 3284ns | 3640ns | 4263ns | -1.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 1337ns | 1160ns | 1465ns | base | 0.048 |
| mw_chain_h_mw8_skew | 1175ns | 1005ns | 1325ns | -12.12% | 0.054 |
| mw_chain_rev_h_mw8_skew | 1330ns | 1128ns | 1534ns | -0.52% | 0.048 |
| mw_jumptable_h_mw8_skew | 1276ns | 1108ns | 1433ns | -4.59% | 0.050 |

## Performance model

- Peak throughput: **0.064 Gops/s** (mw_chain_h_mw8_skew; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_skew | 0.046 | 72.5% |
| mw_chain_h_mw8_skew | 0.054 | 84.7% |
| mw_chain_rev_h_mw8_skew | 0.049 | 76.9% |
| mw_jumptable_h_mw8_skew | 0.050 | 78.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_skew | 3847ns | 3847ns | base |
| mw_chain_h_mw8_skew | 3698ns | 3698ns | -3.88% |
| mw_chain_rev_h_mw8_skew | 3911ns | 3911ns | +1.67% |
| mw_jumptable_h_mw8_skew | 3788ns | 3788ns | -1.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 1385ns | base | --- | [1161, 1465] | --- | --- | --- | --- |
| mw_chain_h_mw8_skew | 1187ns | -147.3ns (-10.6%) | [-275, -64]ns | [1014, 1325] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_skew | 1306ns | no significant difference | [-137, +160]ns | [1150, 1534] | no | 0.6875 | 0.6875 | 0 |
| mw_jumptable_h_mw8_skew | 1286ns | -59.8ns (-4.3%) | [-123, -1]ns | [1109, 1433] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_skew | mw_chain_h_mw8_skew | mw_chain_rev_h_mw8_skew | mw_jumptable_h_mw8_skew |
|---|---|---|---|---|
| 1 | 1389ns | -4.8% | -15.5% | -4.8% |
| 2 | 1382ns | -4.4% | +15.8% | +3.6% |
| 3 | 1409ns | -25.3% | -4.1% | -11.3% |
| 4 | 1160ns | -11.8% | -2.8% | -4.5% |
| 5 | 1162ns | -13.5% | +8.7% | -4.5% |
| 6 | 1521ns | -12.7% | -3.5% | -5.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_skew | -0.080 | ok |
| mw_chain_h_mw8_skew | 0.165 | ok |
| mw_chain_rev_h_mw8_skew | -0.226 | moderate- |
| mw_jumptable_h_mw8_skew | 0.084 | ok |

**Consistency summary:**

- **mw_chain_h_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_skew**: won 4/6, lost 2/6
- **mw_jumptable_h_mw8_skew**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_skew | 3.8ns | 1337.2ns | 0.3% |  |
| mw_chain_h_mw8_skew | 5.2ns | 1175.1ns | 0.4% |  |
| mw_chain_rev_h_mw8_skew | 3.0ns | 1330.2ns | 0.2% |  |
| mw_jumptable_h_mw8_skew | 3.1ns | 1275.8ns | 0.2% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_skew (n=6, range 1160.4-1465.0 ns)
   1160.4 |########################################
   1175.6 |
   1190.9 |
   1206.1 |
   1221.3 |
   1236.6 |
   1251.8 |
   1267.0 |
   1282.2 |
   1297.5 |
   1312.7 |
   1327.9 |
   1343.2 |
   1358.4 |
   1373.6 |########################################
   1388.8 |
   1404.1 |####################
   1419.3 |
   1434.5 |
   1449.8 |
  (0 below, 1 above range)

mw_chain_h_mw8_skew (n=6, range 1004.6-1324.8 ns)
   1004.6 |####################
   1020.6 |####################
   1036.6 |####################
   1052.6 |
   1068.6 |
   1084.7 |
   1100.7 |
   1116.7 |
   1132.7 |
   1148.7 |
   1164.7 |
   1180.7 |
   1196.7 |
   1212.7 |
   1228.7 |
   1244.8 |
   1260.8 |
   1276.8 |
   1292.8 |
   1308.8 |########################################
  (0 below, 1 above range)

mw_chain_rev_h_mw8_skew (n=6, range 1127.5-1534.0 ns)
   1127.5 |########################################
   1147.8 |
   1168.1 |########################################
   1188.5 |
   1208.8 |
   1229.1 |
   1249.4 |########################################
   1269.8 |
   1290.1 |
   1310.4 |
   1330.7 |########################################
   1351.0 |
   1371.4 |
   1391.7 |
   1412.0 |
   1432.3 |
   1452.7 |########################################
   1473.0 |
   1493.3 |
   1513.6 |
  (0 below, 1 above range)

mw_jumptable_h_mw8_skew (n=6, range 1108.3-1433.1 ns)
   1108.3 |########################################
   1124.5 |
   1140.8 |
   1157.0 |
   1173.3 |
   1189.5 |
   1205.7 |
   1222.0 |
   1238.2 |####################
   1254.5 |
   1270.7 |
   1286.9 |
   1303.2 |
   1319.4 |####################
   1335.7 |
   1351.9 |
   1368.1 |
   1384.4 |
   1400.6 |
   1416.9 |####################
  (0 below, 1 above range)

```

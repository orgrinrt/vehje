# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct dominates: 11% faster than the next best (carrier_pre_leaf_null)

carrier_pre_leaf_direct (989 ns) leads carrier_pre_leaf_null (1.10 us) by 11%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_leaf_direct beats baseline by 28% (significant)

carrier_pre_leaf_direct is -378 ns (28%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (28% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 28% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 988.8 ns median (-27.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 988.8 ns, slowest 1812.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 3473ns | 3477ns | 3411ns | 3462ns | 3521ns | -9.92% |
| carrier_pre_leaf_fntable | 4206ns | 4297ns | 3735ns | 4273ns | 4342ns | +9.11% |
| carrier_pre_leaf_null | 3528ns | 3568ns | 3286ns | 3558ns | 3605ns | -8.49% |
| carrier_pre_leaf_regcache | 3907ns | 3889ns | 3872ns | 3885ns | 3957ns | +1.33% |
| carrier_pre_leaf_switch | 3855ns | 3849ns | 3816ns | 3843ns | 3893ns | base |
| carrier_pre_leaf_threaded | 3793ns | 3791ns | 3755ns | 3786ns | 3821ns | -1.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 990ns | 974ns | 1006ns | -27.46% | 0.065 |
| carrier_pre_leaf_fntable | 1785ns | 1590ns | 1851ns | +30.86% | 0.036 |
| carrier_pre_leaf_null | 1094ns | 1020ns | 1122ns | -19.83% | 0.059 |
| carrier_pre_leaf_regcache | 1422ns | 1402ns | 1440ns | +4.23% | 0.045 |
| carrier_pre_leaf_switch | 1364ns | 1348ns | 1378ns | base | 0.047 |
| carrier_pre_leaf_threaded | 1285ns | 1275ns | 1295ns | -5.81% | 0.050 |

## Performance model

- Peak throughput: **0.066 Gops/s** (carrier_pre_leaf_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.065 | 98.5% |
| carrier_pre_leaf_fntable | 0.035 | 53.7% |
| carrier_pre_leaf_null | 0.058 | 88.5% |
| carrier_pre_leaf_regcache | 0.045 | 68.6% |
| carrier_pre_leaf_switch | 0.047 | 71.4% |
| carrier_pre_leaf_threaded | 0.050 | 75.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 3473ns | 3473ns | -9.92% |
| carrier_pre_leaf_fntable | 4206ns | 4206ns | +9.11% |
| carrier_pre_leaf_null | 3528ns | 3528ns | -8.49% |
| carrier_pre_leaf_regcache | 3907ns | 3907ns | +1.33% |
| carrier_pre_leaf_switch | 3855ns | 3855ns | base |
| carrier_pre_leaf_threaded | 3793ns | 3793ns | -1.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 1363ns | base | --- | [1352, 1378] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 989ns | -378.3ns (-27.8%) | [-400, -346]ns | [974, 1006] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 1812ns | +454.2ns (+33.3%) | [+318, +491]ns | [1692, 1851] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_null | 1100ns | -263.3ns (-19.3%) | [-303, -245]ns | [1059, 1122] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 1420ns | +56.7ns (+4.2%) | [+31, +86]ns | [1406, 1440] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_leaf_threaded | 1283ns | -79.8ns (-5.9%) | [-97, -61]ns | [1278, 1295] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 1362ns | -26.9% | +16.7% | -19.4% | +4.3% | -4.3% |
| 2 | 1387ns | -29.2% | +29.4% | -18.9% | +1.1% | -7.2% |
| 3 | 1368ns | -28.8% | +32.4% | -19.7% | +4.0% | -6.8% |
| 4 | 1348ns | -26.0% | +34.5% | -17.0% | +5.3% | -4.6% |
| 5 | 1365ns | -28.6% | +35.3% | -19.2% | +3.4% | -6.2% |
| 6 | 1356ns | -25.1% | +36.9% | -24.8% | +7.4% | -5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.426 | moderate- |
| carrier_pre_leaf_fntable | 0.110 | ok |
| carrier_pre_leaf_null | -0.002 | ok |
| carrier_pre_leaf_regcache | -0.215 | moderate- |
| carrier_pre_leaf_switch | -0.036 | ok |
| carrier_pre_leaf_threaded | 0.040 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 0/6, lost 6/6
- **carrier_pre_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 86663.2ns | 989.6ns | 8757.4% | HIGH |
| carrier_pre_leaf_fntable | 83774.5ns | 1785.4ns | 4692.3% | HIGH |
| carrier_pre_leaf_null | 85861.7ns | 1093.8ns | 7850.2% | HIGH |
| carrier_pre_leaf_regcache | 86330.6ns | 1422.1ns | 6070.8% | HIGH |
| carrier_pre_leaf_switch | 84340.4ns | 1364.3ns | 6182.0% | HIGH |
| carrier_pre_leaf_threaded | 86486.3ns | 1285.0ns | 6730.2% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 973.8-1006.0 ns)
    973.8 |########################################
    975.4 |
    977.0 |
    978.6 |
    980.2 |####################
    981.9 |
    983.5 |
    985.1 |
    986.7 |
    988.3 |
    989.9 |
    991.5 |
    993.1 |
    994.8 |####################
    996.4 |####################
    998.0 |
    999.6 |
   1001.2 |
   1002.8 |
   1004.4 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 1590.0-1851.5 ns)
   1590.0 |########################################
   1603.1 |
   1616.1 |
   1629.2 |
   1642.3 |
   1655.4 |
   1668.4 |
   1681.5 |
   1694.6 |
   1707.7 |
   1720.7 |
   1733.8 |
   1746.9 |
   1759.9 |
   1773.0 |
   1786.1 |########################################
   1799.2 |########################################
   1812.2 |########################################
   1825.3 |
   1838.4 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 1019.6-1122.3 ns)
   1019.6 |####################
   1024.7 |
   1029.9 |
   1035.0 |
   1040.1 |
   1045.3 |
   1050.4 |
   1055.5 |
   1060.7 |
   1065.8 |
   1071.0 |
   1076.1 |
   1081.2 |
   1086.4 |
   1091.5 |
   1096.6 |########################################
   1101.8 |####################
   1106.9 |
   1112.0 |
   1117.2 |####################
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 1402.5-1439.8 ns)
   1402.5 |####################
   1404.4 |
   1406.2 |
   1408.1 |
   1410.0 |####################
   1411.8 |
   1413.7 |
   1415.5 |
   1417.4 |
   1419.3 |########################################
   1421.1 |
   1423.0 |####################
   1424.8 |
   1426.7 |
   1428.6 |
   1430.4 |
   1432.3 |
   1434.2 |
   1436.0 |
   1437.9 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 1348.3-1377.5 ns)
   1348.3 |########################################
   1349.8 |
   1351.2 |
   1352.7 |
   1354.1 |
   1355.6 |########################################
   1357.1 |
   1358.5 |
   1360.0 |
   1361.4 |########################################
   1362.9 |
   1364.4 |########################################
   1365.8 |
   1367.3 |########################################
   1368.7 |
   1370.2 |
   1371.7 |
   1373.1 |
   1374.6 |
   1376.0 |
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 1274.6-1294.6 ns)
   1274.6 |####################
   1275.6 |
   1276.6 |
   1277.6 |
   1278.6 |
   1279.6 |########################################
   1280.6 |
   1281.6 |
   1282.6 |
   1283.6 |
   1284.6 |
   1285.6 |########################################
   1286.6 |
   1287.6 |
   1288.6 |
   1289.6 |
   1290.6 |
   1291.6 |
   1292.6 |
   1293.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=8763.6% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=4775.6% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=7791.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=6090.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=6187.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=6736.2% of algo (FFI overhead may distort results)

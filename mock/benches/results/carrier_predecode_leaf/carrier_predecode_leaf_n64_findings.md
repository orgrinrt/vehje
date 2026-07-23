# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct dominates: 13% faster than the next best (carrier_pre_leaf_null)

carrier_pre_leaf_direct (976 ns) leads carrier_pre_leaf_null (1.10 us) by 13%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_leaf_direct beats baseline by 28% (significant)

carrier_pre_leaf_direct is -384 ns (28%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_switch, carrier_pre_leaf_threaded, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (30% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_switch, carrier_pre_leaf_threaded, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 30% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_pre_leaf_threaded's edge over baseline is significant but tiny (21 ns, 1.55%)

carrier_pre_leaf_threaded differs from baseline carrier_pre_leaf_switch by 21 ns (1.55%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 976.2 ns median (-28.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.86x (fastest 976.2 ns, slowest 1818.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 3410ns | 3409ns | 3377ns | 3401ns | 3439ns | -10.36% |
| carrier_pre_leaf_fntable | 4178ns | 4247ns | 3704ns | 4232ns | 4335ns | +9.85% |
| carrier_pre_leaf_null | 3526ns | 3529ns | 3491ns | 3526ns | 3544ns | -7.30% |
| carrier_pre_leaf_regcache | 3833ns | 3828ns | 3755ns | 3823ns | 3887ns | +0.78% |
| carrier_pre_leaf_switch | 3804ns | 3793ns | 3788ns | 3792ns | 3830ns | base |
| carrier_pre_leaf_threaded | 3844ns | 3845ns | 3788ns | 3838ns | 3880ns | +1.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 978ns | 970ns | 988ns | -27.76% | 0.065 |
| carrier_pre_leaf_fntable | 1788ns | 1590ns | 1855ns | +32.11% | 0.036 |
| carrier_pre_leaf_null | 1102ns | 1094ns | 1108ns | -18.59% | 0.058 |
| carrier_pre_leaf_regcache | 1406ns | 1378ns | 1427ns | +3.84% | 0.046 |
| carrier_pre_leaf_switch | 1354ns | 1324ns | 1367ns | base | 0.047 |
| carrier_pre_leaf_threaded | 1373ns | 1354ns | 1387ns | +1.39% | 0.047 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 249478 | 1246503 | 0.200 | 1.03× |
| carrier_pre_leaf_fntable | 249296 | 1153285 | 0.216 | 1.03× |
| carrier_pre_leaf_null | 248305 | 1312168 | 0.189 | 1.03× |
| carrier_pre_leaf_regcache | 251356 | 1452473 | 0.173 | 1.04× |
| carrier_pre_leaf_switch | 241814 | 1209367 | 0.200 | 1.00× |
| carrier_pre_leaf_threaded | 244911 | 1275014 | 0.192 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.066 Gops/s** (carrier_pre_leaf_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.066 | 99.3% |
| carrier_pre_leaf_fntable | 0.035 | 53.3% |
| carrier_pre_leaf_null | 0.058 | 88.0% |
| carrier_pre_leaf_regcache | 0.046 | 69.1% |
| carrier_pre_leaf_switch | 0.047 | 71.5% |
| carrier_pre_leaf_threaded | 0.047 | 70.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 3410ns | 3410ns | -10.36% |
| carrier_pre_leaf_fntable | 4178ns | 4178ns | +9.85% |
| carrier_pre_leaf_null | 3526ns | 3526ns | -7.30% |
| carrier_pre_leaf_regcache | 3833ns | 3833ns | +0.78% |
| carrier_pre_leaf_switch | 3804ns | 3804ns | base |
| carrier_pre_leaf_threaded | 3844ns | 3844ns | +1.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 1356ns | base | --- | [1338, 1367] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 976ns | -383.9ns (-28.3%) | [-391, -353]ns | [970, 988] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 1819ns | +462.3ns (+34.1%) | [+333, +509]ns | [1692, 1855] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 1101ns | -255.8ns (-18.9%) | [-264, -235]ns | [1097, 1108] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 1403ns | +54.0ns (+4.0%) | [+21, +81]ns | [1388, 1427] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_threaded | 1371ns | +21.1ns (+1.6%) | [+1, +34]ns | [1360, 1387] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 1366ns | -28.5% | +16.4% | -19.5% | +2.3% | +0.3% |
| 2 | 1368ns | -28.7% | +35.7% | -19.0% | +0.7% | -0.1% |
| 3 | 1324ns | -25.9% | +40.0% | -16.7% | +6.1% | +2.3% |
| 4 | 1357ns | -26.7% | +34.2% | -18.4% | +3.2% | +1.9% |
| 5 | 1356ns | -28.4% | +34.0% | -19.3% | +4.7% | +1.2% |
| 6 | 1352ns | -28.3% | +32.6% | -18.6% | +6.0% | +2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.045 | ok |
| carrier_pre_leaf_fntable | -0.120 | ok |
| carrier_pre_leaf_null | -0.239 | moderate- |
| carrier_pre_leaf_regcache | 0.306 | moderate+ |
| carrier_pre_leaf_switch | -0.265 | moderate- |
| carrier_pre_leaf_threaded | -0.111 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 0/6, lost 6/6
- **carrier_pre_leaf_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 86515.9ns | 978.0ns | 8846.4% | HIGH |
| carrier_pre_leaf_fntable | 83864.8ns | 1788.5ns | 4689.2% | HIGH |
| carrier_pre_leaf_null | 85999.9ns | 1102.1ns | 7803.3% | HIGH |
| carrier_pre_leaf_regcache | 86940.9ns | 1405.8ns | 6184.7% | HIGH |
| carrier_pre_leaf_switch | 83749.8ns | 1353.8ns | 6186.2% | HIGH |
| carrier_pre_leaf_threaded | 86055.7ns | 1372.6ns | 6269.3% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 969.6-987.7 ns)
    969.6 |########################################
    970.5 |
    971.4 |
    972.3 |
    973.2 |
    974.1 |
    975.0 |####################
    975.9 |####################
    976.8 |
    977.7 |
    978.7 |
    979.6 |
    980.5 |####################
    981.4 |
    982.3 |
    983.2 |
    984.1 |
    985.0 |
    985.9 |
    986.8 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 1590.4-1855.0 ns)
   1590.4 |####################
   1603.6 |
   1616.9 |
   1630.1 |
   1643.3 |
   1656.6 |
   1669.8 |
   1683.0 |
   1696.2 |
   1709.5 |
   1722.7 |
   1735.9 |
   1749.2 |
   1762.4 |
   1775.6 |
   1788.8 |####################
   1802.1 |
   1815.3 |########################################
   1828.5 |
   1841.8 |####################
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 1094.2-1108.2 ns)
   1094.2 |########################################
   1094.9 |
   1095.6 |
   1096.3 |
   1097.0 |
   1097.7 |
   1098.4 |
   1099.1 |########################################
   1099.8 |########################################
   1100.5 |
   1101.2 |
   1101.9 |
   1102.6 |########################################
   1103.3 |
   1104.0 |
   1104.7 |
   1105.4 |
   1106.1 |
   1106.8 |
   1107.5 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 1377.9-1426.7 ns)
   1377.9 |########################################
   1380.3 |
   1382.8 |
   1385.2 |
   1387.7 |
   1390.1 |
   1392.5 |
   1395.0 |
   1397.4 |########################################
   1399.8 |########################################
   1402.3 |
   1404.7 |########################################
   1407.2 |
   1409.6 |
   1412.0 |
   1414.5 |
   1416.9 |
   1419.3 |########################################
   1421.8 |
   1424.2 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 1323.8-1367.2 ns)
   1323.8 |########################################
   1326.0 |
   1328.1 |
   1330.3 |
   1332.5 |
   1334.7 |
   1336.8 |
   1339.0 |
   1341.2 |
   1343.4 |
   1345.5 |
   1347.7 |
   1349.9 |
   1352.0 |########################################
   1354.2 |########################################
   1356.4 |########################################
   1358.6 |
   1360.7 |
   1362.9 |
   1365.1 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 1353.8-1386.8 ns)
   1353.8 |########################################
   1355.5 |
   1357.1 |
   1358.8 |
   1360.4 |
   1362.1 |
   1363.7 |
   1365.4 |########################################
   1367.0 |
   1368.7 |########################################
   1370.3 |########################################
   1372.0 |
   1373.6 |
   1375.3 |
   1376.9 |
   1378.6 |
   1380.2 |
   1381.9 |########################################
   1383.5 |
   1385.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=8864.2% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=4752.7% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=7789.3% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=6200.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=6182.1% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=6273.1% of algo (FFI overhead may distort results)

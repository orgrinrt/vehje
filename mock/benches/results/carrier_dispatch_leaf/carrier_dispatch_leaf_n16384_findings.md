# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 44% faster than the next best (carrier_disp_leaf_bittree)

carrier_disp_leaf_nullfloor (548.06 us) leads carrier_disp_leaf_bittree (786.62 us) by 44%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 48% (significant)

carrier_disp_leaf_nullfloor is -517.48 us (48%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_threaded is an outlier: 2.3x slower than the field

carrier_disp_leaf_threaded (1.24 ms) is 2.3x the fastest (548.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded} (44% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded} with a 44% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 548062.3 ns median (-48.9% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.26x (fastest 548062.3 ns, slowest 1240599.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 785428ns | 789855ns | 768153ns | 784575ns | 795345ns | -27.00% |
| carrier_disp_leaf_fntable | 1155981ns | 1156012ns | 1145864ns | 1154662ns | 1163017ns | +7.44% |
| carrier_disp_leaf_ifchain | 1090608ns | 1088472ns | 1085743ns | 1087743ns | 1097336ns | +1.37% |
| carrier_disp_leaf_ifchainasc | 1077144ns | 1078100ns | 1067542ns | 1076462ns | 1082968ns | +0.12% |
| carrier_disp_leaf_ifchainlin | 980700ns | 985227ns | 963997ns | 980342ns | 989590ns | -8.85% |
| carrier_disp_leaf_nullfloor | 556991ns | 550694ns | 538658ns | 547019ns | 581115ns | -48.23% |
| carrier_disp_leaf_switch | 1075884ns | 1075913ns | 1062698ns | 1073886ns | 1085474ns | base |
| carrier_disp_leaf_threaded | 1249419ns | 1244191ns | 1239241ns | 1242821ns | 1264406ns | +16.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 782211ns | 765720ns | 791875ns | -27.08% | 0.021 |
| carrier_disp_leaf_fntable | 1153017ns | 1141950ns | 1160470ns | +7.49% | 0.014 |
| carrier_disp_leaf_ifchain | 1087263ns | 1082373ns | 1094244ns | +1.36% | 0.015 |
| carrier_disp_leaf_ifchainasc | 1073686ns | 1064115ns | 1079955ns | +0.10% | 0.015 |
| carrier_disp_leaf_ifchainlin | 977400ns | 960753ns | 986324ns | -8.88% | 0.017 |
| carrier_disp_leaf_nullfloor | 554063ns | 536146ns | 577568ns | -48.35% | 0.030 |
| carrier_disp_leaf_switch | 1072650ns | 1059182ns | 1082697ns | base | 0.015 |
| carrier_disp_leaf_threaded | 1246002ns | 1235447ns | 1261544ns | +16.16% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 4899987 | 18066768 | 0.271 | 0.73× |
| carrier_disp_leaf_fntable | 7209276 | 26192850 | 0.275 | 1.08× |
| carrier_disp_leaf_ifchain | 6812284 | 18433521 | 0.370 | 1.02× |
| carrier_disp_leaf_ifchainasc | 6722739 | 18434343 | 0.365 | 1.01× |
| carrier_disp_leaf_ifchainlin | 6126657 | 24105798 | 0.254 | 0.92× |
| carrier_disp_leaf_nullfloor | 3455828 | 14971214 | 0.231 | 0.52× |
| carrier_disp_leaf_switch | 6683260 | 17911359 | 0.373 | 1.00× |
| carrier_disp_leaf_threaded | 7806888 | 25249964 | 0.309 | 1.17× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.021 | 68.2% |
| carrier_disp_leaf_fntable | 0.014 | 46.5% |
| carrier_disp_leaf_ifchain | 0.015 | 49.4% |
| carrier_disp_leaf_ifchainasc | 0.015 | 49.9% |
| carrier_disp_leaf_ifchainlin | 0.017 | 54.6% |
| carrier_disp_leaf_nullfloor | 0.030 | 97.8% |
| carrier_disp_leaf_switch | 0.015 | 50.0% |
| carrier_disp_leaf_threaded | 0.013 | 43.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 785428ns | 785428ns | -27.00% |
| carrier_disp_leaf_fntable | 1155981ns | 1155981ns | +7.44% |
| carrier_disp_leaf_ifchain | 1090608ns | 1090608ns | +1.37% |
| carrier_disp_leaf_ifchainasc | 1077144ns | 1077144ns | +0.12% |
| carrier_disp_leaf_ifchainlin | 980700ns | 980700ns | -8.85% |
| carrier_disp_leaf_nullfloor | 556991ns | 556991ns | -48.23% |
| carrier_disp_leaf_switch | 1075884ns | 1075884ns | base |
| carrier_disp_leaf_threaded | 1249419ns | 1249419ns | +16.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 1072625ns | base | --- | [1062628, 1082697] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 786615ns | -295674.4ns (-27.6%) | [-302788, -272855]ns | [768141, 791875] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 1153110ns | +82842.1ns (+7.7%) | [+73749, +84509]ns | [1145470, 1160470] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 1084740ns | +16300.5ns (+1.5%) | [+1424, +26114]ns | [1082805, 1094244] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_leaf_ifchainasc | 1074363ns | no significant difference | [-9820, +12199]ns | [1066739, 1079955] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_leaf_ifchainlin | 981542ns | -96586.6ns (-9.0%) | [-107356, -81806]ns | [964335, 986324] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 548062ns | -517480.2ns (-48.2%) | [-533151, -505129]ns | [536559, 577568] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 1240599ns | +170616.6ns (+15.9%) | [+154218, +195221]ns | [1235862, 1261544] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 1059182ns | -25.4% | +7.8% | +2.9% | +1.4% | -7.0% | -49.3% | +20.1% |
| 2 | 1066072ns | -28.2% | +7.8% | +1.6% | -0.2% | -9.9% | -49.7% | +16.6% |
| 3 | 1070279ns | -25.9% | +7.8% | +1.4% | +0.9% | -9.6% | -47.9% | +15.7% |
| 4 | 1077077ns | -27.0% | +7.0% | +2.0% | +0.3% | -8.3% | -47.1% | +16.1% |
| 5 | 1088317ns | -27.7% | +6.6% | -0.5% | -1.3% | -10.1% | -46.2% | +13.5% |
| 6 | 1074970ns | -28.3% | +7.9% | +0.8% | -0.5% | -8.4% | -49.9% | +15.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | -0.495 | moderate- |
| carrier_disp_leaf_fntable | 0.354 | moderate+ |
| carrier_disp_leaf_ifchain | -0.322 | moderate- |
| carrier_disp_leaf_ifchainasc | -0.113 | ok |
| carrier_disp_leaf_ifchainlin | -0.074 | ok |
| carrier_disp_leaf_nullfloor | 0.138 | ok |
| carrier_disp_leaf_switch | 0.398 | moderate+ |
| carrier_disp_leaf_threaded | -0.034 | ok |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 6/6, lost 0/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 1/6, lost 5/6
- **carrier_disp_leaf_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_leaf_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 783788.7ns | 782210.6ns | 100.2% | HIGH |
| carrier_disp_leaf_fntable | 1162551.8ns | 1153016.6ns | 100.8% | HIGH |
| carrier_disp_leaf_ifchain | 1088318.5ns | 1087262.8ns | 100.1% | HIGH |
| carrier_disp_leaf_ifchainasc | 1074808.9ns | 1073685.9ns | 100.1% | HIGH |
| carrier_disp_leaf_ifchainlin | 978020.7ns | 977400.4ns | 100.1% | HIGH |
| carrier_disp_leaf_nullfloor | 555246.2ns | 554063.1ns | 100.2% | HIGH |
| carrier_disp_leaf_switch | 1073611.8ns | 1072649.7ns | 100.1% | HIGH |
| carrier_disp_leaf_threaded | 1246434.2ns | 1246001.6ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 765720.0-791875.4 ns)
  765720.0 |########################################
  767027.8 |
  768335.5 |
  769643.3 |########################################
  770951.1 |
  772258.8 |
  773566.6 |
  774874.4 |
  776182.2 |
  777489.9 |
  778797.7 |
  780105.5 |
  781413.2 |
  782721.0 |
  784028.8 |
  785336.6 |########################################
  786644.3 |########################################
  787952.1 |
  789259.9 |
  790567.6 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 1141950.4-1160470.2 ns)
  1141950.4 |########################################
  1142876.4 |
  1143802.4 |
  1144728.4 |
  1145654.4 |
  1146580.3 |
  1147506.3 |
  1148432.3 |########################################
  1149358.3 |
  1150284.3 |
  1151210.3 |
  1152136.3 |########################################
  1153062.3 |
  1153988.3 |########################################
  1154914.3 |
  1155840.2 |
  1156766.2 |
  1157692.2 |
  1158618.2 |
  1159544.2 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 1082373.3-1094244.2 ns)
  1082373.3 |########################################
  1082966.8 |########################################
  1083560.4 |########################################
  1084153.9 |
  1084747.5 |
  1085341.0 |########################################
  1085934.6 |
  1086528.1 |
  1087121.7 |
  1087715.2 |
  1088308.8 |
  1088902.3 |
  1089495.8 |
  1090089.4 |########################################
  1090682.9 |
  1091276.5 |
  1091870.0 |
  1092463.6 |
  1093057.1 |
  1093650.7 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 1064115.4-1079955.2 ns)
  1064115.4 |########################################
  1064907.4 |
  1065699.4 |
  1066491.4 |
  1067283.4 |
  1068075.3 |
  1068867.3 |########################################
  1069659.3 |
  1070451.3 |
  1071243.3 |
  1072035.3 |
  1072827.3 |
  1073619.3 |########################################
  1074411.3 |########################################
  1075203.3 |
  1075995.2 |
  1076787.2 |
  1077579.2 |
  1078371.2 |
  1079163.2 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 960753.3-986324.2 ns)
  960753.3 |########################################
  962031.8 |
  963310.4 |
  964588.9 |
  965867.5 |
  967146.0 |########################################
  968424.6 |
  969703.1 |
  970981.6 |
  972260.2 |
  973538.7 |
  974817.3 |
  976095.8 |
  977374.4 |
  978652.9 |########################################
  979931.4 |
  981210.0 |
  982488.5 |
  983767.1 |########################################
  985045.6 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 536145.8-577568.3 ns)
  536145.8 |########################################
  538216.9 |####################
  540288.1 |
  542359.2 |
  544430.3 |
  546501.4 |
  548572.6 |
  550643.7 |
  552714.8 |
  554785.9 |
  556857.1 |####################
  558928.2 |
  560999.3 |
  563070.5 |
  565141.6 |
  567212.7 |
  569283.8 |####################
  571355.0 |
  573426.1 |
  575497.2 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 1059182.5-1082697.1 ns)
  1059182.5 |########################################
  1060358.2 |
  1061534.0 |
  1062709.7 |
  1063885.4 |
  1065061.1 |########################################
  1066236.9 |
  1067412.6 |
  1068588.3 |
  1069764.1 |########################################
  1070939.8 |
  1072115.5 |
  1073291.3 |
  1074467.0 |########################################
  1075642.7 |
  1076818.5 |########################################
  1077994.2 |
  1079169.9 |
  1080345.6 |
  1081521.4 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 1235447.1-1261543.5 ns)
  1235447.1 |########################################
  1236751.9 |####################
  1238056.7 |
  1239361.6 |
  1240666.4 |
  1241971.2 |
  1243276.0 |####################
  1244580.8 |
  1245885.7 |
  1247190.5 |
  1248495.3 |
  1249800.1 |####################
  1251104.9 |
  1252409.8 |
  1253714.6 |
  1255019.4 |
  1256324.2 |
  1257629.0 |
  1258933.9 |
  1260238.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=100.0% of algo (FFI overhead may distort results)

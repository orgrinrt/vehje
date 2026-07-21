# Per-branch strategy: archetype 4 (ifchain4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b4_table**

## Highlights

Baseline for all deltas below: **ab_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b4_tree, ab_b4_table) are a dead heat (<1%)

ab_b4_tree (1.29 ms) and ab_b4_table (1.30 ms) differ by 0.35%, inside the noise, even though the wider field spreads 17.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b4_prof shows warm-up / thermal drift (autocorr +0.60)

ab_b4_prof's per-pass series has lag-1 autocorrelation +0.60, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ab_b4_tree** at 1292369.1 ns median (-0.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.17x (fastest 1292369.1 ns, slowest 1518219.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b4_pred | 1520299ns | 1521652ns | 1492729ns | 1519772ns | 1534874ns | +17.08% |
| ab_b4_prof | 1337684ns | 1339440ns | 1324795ns | 1335759ns | 1347017ns | +3.02% |
| ab_b4_seq | 1328393ns | 1324664ns | 1312441ns | 1322140ns | 1345748ns | +2.30% |
| ab_b4_table | 1298519ns | 1299944ns | 1289622ns | 1298781ns | 1302574ns | base |
| ab_b4_tree | 1298369ns | 1295721ns | 1287417ns | 1295071ns | 1308791ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b4_pred | 1517092ns | 1489123ns | 1532281ns | +17.13% | 0.003 |
| ab_b4_prof | 1334579ns | 1321630ns | 1344562ns | +3.04% | 0.003 |
| ab_b4_seq | 1325303ns | 1309172ns | 1342932ns | +2.32% | 0.003 |
| ab_b4_table | 1295261ns | 1285934ns | 1299411ns | base | 0.003 |
| ab_b4_tree | 1295274ns | 1284066ns | 1306216ns | +0.00% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b4_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b4_pred | 0.003 | 84.6% |
| ab_b4_prof | 0.003 | 96.1% |
| ab_b4_seq | 0.003 | 97.2% |
| ab_b4_table | 0.003 | 99.0% |
| ab_b4_tree | 0.003 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b4_pred | 1520299ns | 1520299ns | +17.08% |
| ab_b4_prof | 1337684ns | 1337684ns | +3.02% |
| ab_b4_seq | 1328393ns | 1328393ns | +2.30% |
| ab_b4_table | 1298519ns | 1298519ns | base |
| ab_b4_tree | 1298369ns | 1298369ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b4_table | 1296893ns | base | --- | [1289478, 1299411] | --- | --- | --- | --- |
| ab_b4_pred | 1518219ns | +226537.5ns (+17.5%) | [+206088, +232870]ns | [1500778, 1532281] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_prof | 1335896ns | +37656.2ns (+2.9%) | [+25214, +55085]ns | [1323278, 1344562] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_seq | 1321494ns | +29571.4ns (+2.3%) | [+12333, +48222]ns | [1311482, 1342932] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_tree | 1292369ns | no significant difference | [-10517, +7977]ns | [1287236, 1306216] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b4_table | ab_b4_pred | ab_b4_prof | ab_b4_seq | ab_b4_tree |
|---|---|---|---|---|---|
| 1 | 1300745ns | +17.4% | +3.0% | +0.6% | +0.6% |
| 2 | 1285934ns | +17.6% | +4.7% | +3.3% | +0.3% |
| 3 | 1293021ns | +17.5% | +3.8% | +3.2% | +0.1% |
| 4 | 1295734ns | +14.9% | +2.8% | +1.4% | +0.6% |
| 5 | 1298052ns | +16.9% | +1.8% | +4.1% | -1.1% |
| 6 | 1298077ns | +18.4% | +2.1% | +1.3% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b4_pred | -0.098 | ok |
| ab_b4_prof | 0.595 | HIGH+ (drift/warm-up) |
| ab_b4_seq | -0.554 | HIGH- (thermal bounce) |
| ab_b4_table | -0.160 | ok |
| ab_b4_tree | -0.274 | moderate- |

**Consistency summary:**

- **ab_b4_pred**: won 0/6, lost 6/6
- **ab_b4_prof**: won 0/6, lost 6/6
- **ab_b4_seq**: won 0/6, lost 6/6
- **ab_b4_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b4_pred | 18.4ns | 1517092.5ns | 0.0% |  |
| ab_b4_prof | 17.4ns | 1334578.8ns | 0.0% |  |
| ab_b4_seq | 15.6ns | 1325302.7ns | 0.0% |  |
| ab_b4_table | 19.8ns | 1295260.6ns | 0.0% |  |
| ab_b4_tree | 11.4ns | 1295273.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b4_pred (n=6, range 1489122.9-1532280.6 ns)
  1489122.9 |########################################
  1491280.8 |
  1493438.7 |
  1495596.6 |
  1497754.4 |
  1499912.3 |
  1502070.2 |
  1504228.1 |
  1506386.0 |
  1508543.9 |
  1510701.8 |########################################
  1512859.6 |
  1515017.5 |########################################
  1517175.4 |
  1519333.3 |########################################
  1521491.2 |
  1523649.1 |
  1525806.9 |########################################
  1527964.8 |
  1530122.7 |
  (0 below, 1 above range)

ab_b4_prof (n=6, range 1321630.0-1344562.3 ns)
  1321630.0 |########################################
  1322776.6 |
  1323923.2 |########################################
  1325069.8 |
  1326216.5 |
  1327363.1 |
  1328509.7 |
  1329656.3 |
  1330802.9 |########################################
  1331949.5 |
  1333096.1 |
  1334242.8 |
  1335389.4 |
  1336536.0 |
  1337682.6 |
  1338829.2 |
  1339975.8 |########################################
  1341122.5 |
  1342269.1 |########################################
  1343415.7 |
  (0 below, 1 above range)

ab_b4_seq (n=6, range 1309172.5-1342931.9 ns)
  1309172.5 |########################################
  1310860.5 |
  1312548.4 |########################################
  1314236.4 |########################################
  1315924.4 |
  1317612.3 |
  1319300.3 |
  1320988.3 |
  1322676.2 |
  1324364.2 |
  1326052.2 |
  1327740.1 |########################################
  1329428.1 |
  1331116.1 |
  1332804.0 |########################################
  1334492.0 |
  1336180.0 |
  1337867.9 |
  1339555.9 |
  1341243.9 |
  (0 below, 1 above range)

ab_b4_table (n=6, range 1285934.2-1299410.9 ns)
  1285934.2 |########################################
  1286608.0 |
  1287281.9 |
  1287955.7 |
  1288629.5 |
  1289303.4 |
  1289977.2 |
  1290651.0 |
  1291324.9 |
  1291998.7 |
  1292672.5 |########################################
  1293346.4 |
  1294020.2 |
  1294694.0 |
  1295367.9 |########################################
  1296041.7 |
  1296715.5 |
  1297389.4 |########################################
  1298063.2 |########################################
  1298737.0 |
  (0 below, 1 above range)

ab_b4_tree (n=6, range 1284066.2-1306216.2 ns)
  1284066.2 |########################################
  1285173.7 |
  1286281.2 |
  1287388.7 |
  1288496.2 |
  1289603.7 |########################################
  1290711.2 |########################################
  1291818.7 |
  1292926.2 |########################################
  1294033.7 |
  1295141.2 |
  1296248.7 |
  1297356.2 |
  1298463.7 |
  1299571.2 |
  1300678.7 |
  1301786.2 |
  1302893.7 |########################################
  1304001.2 |
  1305108.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **ab_b4_prof**: autocorrelation=0.60 (measurement drift or warm-up artifact)

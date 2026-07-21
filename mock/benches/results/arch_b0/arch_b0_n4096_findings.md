# Per-branch strategy: archetype 0 (match4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b0_table**

## Highlights

Baseline for all deltas below: **ab_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b0_table, ab_b0_prof) are a dead heat (<1%)

ab_b0_table (1.29 ms) and ab_b0_prof (1.29 ms) differ by 0.11%, inside the noise, even though the wider field spreads 20.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b0_pred shows alternating (throttle bounce) (autocorr -0.65)

ab_b0_pred's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b0_table)

The baseline ab_b0_table is the fastest (1.29 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b0_table) is the fastest** at 1291891.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 1291891.9 ns, slowest 1557291.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b0_pred | 1562628ns | 1560741ns | 1553493ns | 1559983ns | 1571162ns | +20.48% |
| ab_b0_prof | 1298282ns | 1296232ns | 1287395ns | 1295103ns | 1308493ns | +0.10% |
| ab_b0_seq | 1305904ns | 1303647ns | 1292106ns | 1301380ns | 1319589ns | +0.68% |
| ab_b0_table | 1297045ns | 1295045ns | 1285035ns | 1294120ns | 1307440ns | base |
| ab_b0_tree | 1301032ns | 1301135ns | 1289795ns | 1299417ns | 1309074ns | +0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b0_pred | 1559413ns | 1549895ns | 1568471ns | +20.52% | 0.003 |
| ab_b0_prof | 1295137ns | 1283984ns | 1305368ns | +0.09% | 0.003 |
| ab_b0_seq | 1302880ns | 1288785ns | 1316589ns | +0.69% | 0.003 |
| ab_b0_table | 1293941ns | 1281628ns | 1304571ns | base | 0.003 |
| ab_b0_tree | 1297825ns | 1286542ns | 1306114ns | +0.30% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b0_table; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b0_pred | 0.003 | 82.3% |
| ab_b0_prof | 0.003 | 99.1% |
| ab_b0_seq | 0.003 | 98.5% |
| ab_b0_table | 0.003 | 99.2% |
| ab_b0_tree | 0.003 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b0_pred | 1562628ns | 1562628ns | +20.48% |
| ab_b0_prof | 1298282ns | 1298282ns | +0.10% |
| ab_b0_seq | 1305904ns | 1305904ns | +0.68% |
| ab_b0_table | 1297045ns | 1297045ns | base |
| ab_b0_tree | 1301032ns | 1301032ns | +0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b0_table | 1291892ns | base | --- | [1285361, 1304571] | --- | --- | --- | --- |
| ab_b0_pred | 1557291ns | +269397.7ns (+20.9%) | [+249349, +277669]ns | [1552477, 1568471] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b0_prof | 1293316ns | no significant difference | [-14845, +11358]ns | [1286729, 1305368] | no | 0.9167 | 0.6875 | 0 |
| ab_b0_seq | 1300858ns | no significant difference | [-10942, +30378]ns | [1291192, 1316589] | no | 0.9167 | 0.6875 | 0 |
| ab_b0_tree | 1297731ns | no significant difference | [-10045, +18515]ns | [1289631, 1306114] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b0_table | ab_b0_pred | ab_b0_prof | ab_b0_seq | ab_b0_tree |
|---|---|---|---|---|---|
| 1 | 1290214ns | +20.8% | +0.5% | -0.1% | -0.3% |
| 2 | 1289093ns | +21.9% | -0.4% | +0.8% | +0.7% |
| 3 | 1314053ns | +17.9% | -1.9% | -1.6% | -1.2% |
| 4 | 1295089ns | +20.9% | +0.6% | +0.5% | -0.2% |
| 5 | 1293570ns | +20.3% | +1.1% | +0.6% | +1.1% |
| 6 | 1281628ns | +21.3% | +0.7% | +3.9% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b0_pred | -0.652 | HIGH- (thermal bounce) |
| ab_b0_prof | 0.122 | ok |
| ab_b0_seq | 0.040 | ok |
| ab_b0_table | -0.088 | ok |
| ab_b0_tree | 0.065 | ok |

**Consistency summary:**

- **ab_b0_pred**: won 0/6, lost 6/6
- **ab_b0_prof**: won 2/6, lost 4/6
- **ab_b0_seq**: won 2/6, lost 4/6
- **ab_b0_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b0_pred | 26.7ns | 1559413.1ns | 0.0% |  |
| ab_b0_prof | 18.8ns | 1295137.3ns | 0.0% |  |
| ab_b0_seq | 16.3ns | 1302879.5ns | 0.0% |  |
| ab_b0_table | 9.8ns | 1293941.2ns | 0.0% |  |
| ab_b0_tree | 12.0ns | 1297825.4ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b0_pred (n=6, range 1549895.4-1568471.2 ns)
  1549895.4 |########################################
  1550824.2 |
  1551753.0 |
  1552681.8 |
  1553610.6 |
  1554539.4 |########################################
  1555468.2 |
  1556396.9 |########################################
  1557325.7 |########################################
  1558254.5 |
  1559183.3 |
  1560112.1 |
  1561040.9 |
  1561969.7 |
  1562898.5 |
  1563827.3 |
  1564756.1 |
  1565684.9 |########################################
  1566613.7 |
  1567542.5 |
  (0 below, 1 above range)

ab_b0_prof (n=6, range 1283983.8-1305367.7 ns)
  1283983.8 |########################################
  1285053.0 |
  1286122.2 |
  1287191.4 |
  1288260.6 |
  1289329.8 |########################################
  1290399.0 |########################################
  1291468.2 |
  1292537.4 |
  1293606.6 |
  1294675.8 |
  1295744.9 |########################################
  1296814.1 |
  1297883.3 |
  1298952.5 |
  1300021.7 |
  1301090.9 |
  1302160.1 |
  1303229.3 |########################################
  1304298.5 |
  (0 below, 1 above range)

ab_b0_seq (n=6, range 1288785.4-1316588.8 ns)
  1288785.4 |####################
  1290175.6 |
  1291565.7 |
  1292955.9 |####################
  1294346.1 |
  1295736.2 |
  1297126.4 |
  1298516.6 |
  1299906.7 |####################
  1301296.9 |########################################
  1302687.1 |
  1304077.2 |
  1305467.4 |
  1306857.6 |
  1308247.7 |
  1309637.9 |
  1311028.1 |
  1312418.2 |
  1313808.4 |
  1315198.6 |
  (0 below, 1 above range)

ab_b0_table (n=6, range 1281627.9-1304571.1 ns)
  1281627.9 |########################################
  1282775.1 |
  1283922.2 |
  1285069.4 |
  1286216.5 |
  1287363.7 |
  1288510.8 |########################################
  1289658.0 |########################################
  1290805.2 |
  1291952.3 |
  1293099.5 |########################################
  1294246.6 |########################################
  1295393.8 |
  1296540.9 |
  1297688.1 |
  1298835.3 |
  1299982.4 |
  1301129.6 |
  1302276.7 |
  1303423.9 |
  (0 below, 1 above range)

ab_b0_tree (n=6, range 1286542.1-1306114.1 ns)
  1286542.1 |####################
  1287520.7 |
  1288499.3 |
  1289477.9 |
  1290456.5 |
  1291435.1 |
  1292413.7 |####################
  1293392.3 |
  1294370.9 |
  1295349.5 |
  1296328.1 |
  1297306.7 |########################################
  1298285.3 |
  1299263.9 |
  1300242.5 |
  1301221.1 |
  1302199.7 |
  1303178.3 |
  1304156.9 |####################
  1305135.5 |
  (0 below, 1 above range)

```

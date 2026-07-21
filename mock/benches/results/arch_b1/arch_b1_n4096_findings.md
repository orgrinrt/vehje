# Per-branch strategy: archetype 1 (match4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b1_table**

## Highlights

Baseline for all deltas below: **ab_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b1_table, ab_b1_tree) are a dead heat (<1%)

ab_b1_table (1.28 ms) and ab_b1_tree (1.29 ms) differ by 0.76%, inside the noise, even though the wider field spreads 21.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b1_tree shows alternating (throttle bounce) (autocorr -0.53)

ab_b1_tree's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b1_table)

The baseline ab_b1_table is the fastest (1.28 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b1_table) is the fastest** at 1282651.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.21x (fastest 1282651.2 ns, slowest 1556740.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b1_pred | 1561204ns | 1559608ns | 1544935ns | 1557822ns | 1574412ns | +21.12% |
| ab_b1_prof | 1303753ns | 1300908ns | 1288180ns | 1298839ns | 1318909ns | +1.15% |
| ab_b1_seq | 1300748ns | 1299597ns | 1293669ns | 1297747ns | 1308790ns | +0.92% |
| ab_b1_table | 1288940ns | 1285908ns | 1279202ns | 1285129ns | 1299524ns | base |
| ab_b1_tree | 1299328ns | 1295619ns | 1287244ns | 1293559ns | 1314023ns | +0.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b1_pred | 1558142ns | 1541418ns | 1571545ns | +21.18% | 0.003 |
| ab_b1_prof | 1300843ns | 1285501ns | 1316320ns | +1.17% | 0.003 |
| ab_b1_seq | 1297542ns | 1290186ns | 1305674ns | +0.91% | 0.003 |
| ab_b1_table | 1285846ns | 1275887ns | 1296764ns | base | 0.003 |
| ab_b1_tree | 1296331ns | 1284196ns | 1311366ns | +0.82% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b1_table; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b1_pred | 0.003 | 82.0% |
| ab_b1_prof | 0.003 | 98.3% |
| ab_b1_seq | 0.003 | 98.4% |
| ab_b1_table | 0.003 | 99.5% |
| ab_b1_tree | 0.003 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b1_pred | 1561204ns | 1561204ns | +21.12% |
| ab_b1_prof | 1303753ns | 1303753ns | +1.15% |
| ab_b1_seq | 1300748ns | 1300748ns | +0.92% |
| ab_b1_table | 1288940ns | 1288940ns | base |
| ab_b1_tree | 1299328ns | 1299328ns | +0.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b1_table | 1282651ns | base | --- | [1278123, 1296764] | --- | --- | --- | --- |
| ab_b1_pred | 1556740ns | +275690.6ns (+21.5%) | [+251342, +289854]ns | [1546141, 1571545] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b1_prof | 1297778ns | no significant difference | [-6273, +34054]ns | [1288432, 1316320] | no | 0.2188 | 0.2188 | 0 |
| ab_b1_seq | 1296462ns | +12541.6ns (+1.0%) | [+1317, +21230]ns | [1290491, 1305674] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| ab_b1_tree | 1292431ns | no significant difference | [-9661, +32776]ns | [1285196, 1311366] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b1_table | ab_b1_pred | ab_b1_prof | ab_b1_seq | ab_b1_tree |
|---|---|---|---|---|---|
| 1 | 1280360ns | +22.2% | +3.2% | +1.4% | +0.3% |
| 2 | 1275887ns | +22.4% | +1.2% | +1.1% | +3.2% |
| 3 | 1309354ns | +18.5% | -1.3% | -0.3% | -1.8% |
| 4 | 1284010ns | +22.9% | +1.5% | +0.8% | +0.6% |
| 5 | 1284173ns | +20.8% | +2.1% | +0.5% | +0.7% |
| 6 | 1281293ns | +20.3% | +0.3% | +1.9% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b1_pred | -0.175 | ok |
| ab_b1_prof | -0.293 | moderate- |
| ab_b1_seq | -0.507 | HIGH- (thermal bounce) |
| ab_b1_table | -0.299 | moderate- |
| ab_b1_tree | -0.525 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ab_b1_pred**: won 0/6, lost 6/6
- **ab_b1_prof**: won 1/6, lost 5/6
- **ab_b1_seq**: won 1/6, lost 5/6
- **ab_b1_tree**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b1_pred | 18.9ns | 1558141.8ns | 0.0% |  |
| ab_b1_prof | 16.3ns | 1300843.2ns | 0.0% |  |
| ab_b1_seq | 14.3ns | 1297542.3ns | 0.0% |  |
| ab_b1_table | 11.2ns | 1285846.1ns | 0.0% |  |
| ab_b1_tree | 12.9ns | 1296330.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b1_pred (n=6, range 1541417.9-1571544.6 ns)
  1541417.9 |####################
  1542924.2 |
  1544430.6 |
  1545936.9 |
  1547443.2 |
  1548949.6 |
  1550455.9 |########################################
  1551962.2 |
  1553468.6 |
  1554974.9 |
  1556481.2 |
  1557987.6 |
  1559493.9 |
  1561000.2 |####################
  1562506.6 |
  1564012.9 |####################
  1565519.2 |
  1567025.6 |
  1568531.9 |
  1570038.2 |
  (0 below, 1 above range)

ab_b1_prof (n=6, range 1285500.8-1316320.0 ns)
  1285500.8 |########################################
  1287041.8 |
  1288582.7 |
  1290123.7 |########################################
  1291664.6 |########################################
  1293205.6 |
  1294746.6 |
  1296287.5 |
  1297828.5 |
  1299369.4 |
  1300910.4 |
  1302451.4 |########################################
  1303992.3 |
  1305533.3 |
  1307074.2 |
  1308615.2 |
  1310156.2 |########################################
  1311697.1 |
  1313238.1 |
  1314779.0 |
  (0 below, 1 above range)

ab_b1_seq (n=6, range 1290185.8-1305674.2 ns)
  1290185.8 |########################################
  1290960.2 |
  1291734.6 |
  1292509.1 |
  1293283.5 |
  1294057.9 |####################
  1294832.3 |
  1295606.7 |
  1296381.2 |
  1297155.6 |
  1297930.0 |####################
  1298704.4 |
  1299478.8 |
  1300253.3 |
  1301027.7 |
  1301802.1 |
  1302576.5 |
  1303350.9 |
  1304125.4 |
  1304899.8 |####################
  (0 below, 1 above range)

ab_b1_table (n=6, range 1275887.1-1296763.6 ns)
  1275887.1 |####################
  1276930.9 |
  1277974.7 |
  1279018.6 |
  1280062.4 |####################
  1281106.2 |####################
  1282150.0 |
  1283193.9 |########################################
  1284237.7 |
  1285281.5 |
  1286325.3 |
  1287369.1 |
  1288413.0 |
  1289456.8 |
  1290500.6 |
  1291544.4 |
  1292588.3 |
  1293632.1 |
  1294675.9 |
  1295719.7 |
  (0 below, 1 above range)

ab_b1_tree (n=6, range 1284195.8-1311365.9 ns)
  1284195.8 |########################################
  1285554.3 |########################################
  1286912.8 |
  1288271.3 |
  1289629.8 |
  1290988.3 |########################################
  1292346.8 |########################################
  1293705.3 |
  1295063.8 |
  1296422.3 |
  1297780.8 |
  1299139.3 |
  1300497.8 |
  1301856.3 |
  1303214.8 |
  1304573.3 |########################################
  1305931.8 |
  1307290.3 |
  1308648.8 |
  1310007.3 |
  (0 below, 1 above range)

```

# Per-branch strategy: archetype 3 (match4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b3_table**

## Highlights

Baseline for all deltas below: **ab_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b3_prof, ab_b3_tree) are a dead heat (<1%)

ab_b3_prof (1.29 ms) and ab_b3_tree (1.30 ms) differ by 0.46%, inside the noise, even though the wider field spreads 91.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ab_b3_prof, ab_b3_tree, ab_b3_seq, ab_b3_table} vs {ab_b3_pred} (89% apart)

The field splits into a fast tier {ab_b3_prof, ab_b3_tree, ab_b3_seq, ab_b3_table} and a slow tier {ab_b3_pred} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_b3_prof** at 1289346.9 ns median (-1.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 1289346.9 ns, slowest 2462022.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b3_pred | 2469981ns | 2465937ns | 2456589ns | 2465157ns | 2483913ns | +88.88% |
| ab_b3_prof | 1298980ns | 1292600ns | 1286290ns | 1291152ns | 1317068ns | -0.67% |
| ab_b3_seq | 1311833ns | 1304568ns | 1290798ns | 1301396ns | 1338005ns | +0.31% |
| ab_b3_table | 1307726ns | 1307249ns | 1290540ns | 1303010ns | 1323394ns | base |
| ab_b3_tree | 1300892ns | 1298657ns | 1282672ns | 1296713ns | 1316272ns | -0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b3_pred | 2466403ns | 2452825ns | 2480621ns | +89.00% | 0.002 |
| ab_b3_prof | 1295905ns | 1283428ns | 1314103ns | -0.69% | 0.003 |
| ab_b3_seq | 1308733ns | 1287291ns | 1335386ns | +0.29% | 0.003 |
| ab_b3_table | 1304952ns | 1287512ns | 1320851ns | base | 0.003 |
| ab_b3_tree | 1297787ns | 1279474ns | 1313557ns | -0.55% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b3_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b3_pred | 0.002 | 52.0% |
| ab_b3_prof | 0.003 | 99.2% |
| ab_b3_seq | 0.003 | 98.3% |
| ab_b3_table | 0.003 | 98.1% |
| ab_b3_tree | 0.003 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b3_pred | 2469981ns | 2469981ns | +88.88% |
| ab_b3_prof | 1298980ns | 1298980ns | -0.67% |
| ab_b3_seq | 1311833ns | 1311833ns | +0.31% |
| ab_b3_table | 1307726ns | 1307726ns | base |
| ab_b3_tree | 1300892ns | 1300892ns | -0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b3_table | 1304588ns | base | --- | [1289416, 1320851] | --- | --- | --- | --- |
| ab_b3_pred | 2462022ns | +1160809.4ns (+89.0%) | [+1142908, +1180637]ns | [2456566, 2480621] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b3_prof | 1289347ns | no significant difference | [-21112, +9212]ns | [1284264, 1314103] | no | 0.4375 | 0.2188 | 0 |
| ab_b3_seq | 1301409ns | no significant difference | [-26196, +38015]ns | [1289402, 1335386] | no | 1.0000 | 1.0000 | 0 |
| ab_b3_tree | 1295227ns | no significant difference | [-33767, +24141]ns | [1284578, 1313557] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b3_table | ab_b3_pred | ab_b3_prof | ab_b3_seq | ab_b3_tree |
|---|---|---|---|---|---|
| 1 | 1291320ns | +90.5% | +1.7% | +1.1% | +1.6% |
| 2 | 1310704ns | +88.2% | -2.0% | -1.5% | -1.6% |
| 3 | 1301945ns | +91.6% | -1.1% | -1.1% | -0.2% |
| 4 | 1287512ns | +90.5% | -0.3% | +2.4% | +2.2% |
| 5 | 1307231ns | +88.4% | -1.3% | +3.4% | -2.1% |
| 6 | 1330998ns | +84.9% | -1.2% | -2.5% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b3_pred | -0.288 | moderate- |
| ab_b3_prof | -0.040 | ok |
| ab_b3_seq | 0.062 | ok |
| ab_b3_table | -0.019 | ok |
| ab_b3_tree | -0.319 | moderate- |

**Consistency summary:**

- **ab_b3_pred**: won 0/6, lost 6/6
- **ab_b3_prof**: won 5/6, lost 1/6
- **ab_b3_seq**: won 3/6, lost 3/6
- **ab_b3_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b3_pred | 27.9ns | 2466403.0ns | 0.0% |  |
| ab_b3_prof | 15.3ns | 1295904.6ns | 0.0% |  |
| ab_b3_seq | 12.5ns | 1308732.7ns | 0.0% |  |
| ab_b3_table | 13.7ns | 1304951.7ns | 0.0% |  |
| ab_b3_tree | 17.9ns | 1297787.4ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b3_pred (n=6, range 2452824.6-2480620.9 ns)
  2452824.6 |########################################
  2454214.4 |
  2455604.2 |
  2456994.0 |
  2458383.9 |
  2459773.7 |########################################
  2461163.5 |########################################
  2462553.3 |########################################
  2463943.1 |
  2465332.9 |
  2466722.7 |########################################
  2468112.5 |
  2469502.4 |
  2470892.2 |
  2472282.0 |
  2473671.8 |
  2475061.6 |
  2476451.4 |
  2477841.2 |
  2479231.0 |
  (0 below, 1 above range)

ab_b3_prof (n=6, range 1283428.3-1314102.7 ns)
  1283428.3 |########################################
  1284962.0 |########################################
  1286495.7 |########################################
  1288029.5 |
  1289563.2 |########################################
  1291096.9 |
  1292630.6 |
  1294164.3 |
  1295698.1 |
  1297231.8 |
  1298765.5 |
  1300299.2 |
  1301832.9 |
  1303366.7 |
  1304900.4 |
  1306434.1 |
  1307967.8 |
  1309501.5 |
  1311035.3 |
  1312569.0 |########################################
  (0 below, 1 above range)

ab_b3_seq (n=6, range 1287290.8-1335386.2 ns)
  1287290.8 |########################################
  1289695.6 |########################################
  1292100.3 |
  1294505.1 |
  1296909.9 |########################################
  1299314.7 |
  1301719.4 |
  1304124.2 |########################################
  1306529.0 |
  1308933.8 |
  1311338.5 |
  1313743.3 |
  1316148.1 |
  1318552.8 |########################################
  1320957.6 |
  1323362.4 |
  1325767.2 |
  1328171.9 |
  1330576.7 |
  1332981.5 |
  (0 below, 1 above range)

ab_b3_table (n=6, range 1287511.7-1320850.9 ns)
  1287511.7 |########################################
  1289178.7 |
  1290845.6 |########################################
  1292512.6 |
  1294179.5 |
  1295846.5 |
  1297513.4 |
  1299180.4 |
  1300847.4 |########################################
  1302514.3 |
  1304181.3 |
  1305848.2 |########################################
  1307515.2 |
  1309182.1 |########################################
  1310849.1 |
  1312516.1 |
  1314183.0 |
  1315850.0 |
  1317516.9 |
  1319183.9 |
  (0 below, 1 above range)

ab_b3_tree (n=6, range 1279473.8-1313557.4 ns)
  1279473.8 |########################################
  1281178.0 |
  1282882.2 |
  1284586.3 |
  1286290.5 |
  1287994.7 |########################################
  1289698.9 |########################################
  1291403.1 |
  1293107.3 |
  1294811.4 |
  1296515.6 |
  1298219.8 |########################################
  1299924.0 |
  1301628.2 |
  1303332.4 |
  1305036.5 |
  1306740.7 |
  1308444.9 |
  1310149.1 |########################################
  1311853.3 |
  (0 below, 1 above range)

```

# Per-branch strategy: archetype 6 (ifchain4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b6_table**

## Highlights

Baseline for all deltas below: **ab_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b6_tree, ab_b6_table) are a dead heat (<1%)

ab_b6_tree (1.29 ms) and ab_b6_table (1.30 ms) differ by 0.83%, inside the noise, even though the wider field spreads 88.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b6_tree shows alternating (throttle bounce) (autocorr -0.51)

ab_b6_tree's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_b6_tree, ab_b6_table, ab_b6_prof, ab_b6_seq} vs {ab_b6_pred} (85% apart)

The field splits into a fast tier {ab_b6_tree, ab_b6_table, ab_b6_prof, ab_b6_seq} and a slow tier {ab_b6_pred} with a 85% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_b6_tree** at 1292859.4 ns median (-0.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.88x (fastest 1292859.4 ns, slowest 2433470.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b6_pred | 2444161ns | 2436812ns | 2428149ns | 2434138ns | 2467201ns | +86.71% |
| ab_b6_prof | 1322797ns | 1317558ns | 1314248ns | 1316486ns | 1336538ns | +1.05% |
| ab_b6_seq | 1334368ns | 1321062ns | 1318272ns | 1320527ns | 1363178ns | +1.93% |
| ab_b6_table | 1309086ns | 1306668ns | 1289676ns | 1302584ns | 1328542ns | base |
| ab_b6_tree | 1298103ns | 1295847ns | 1284710ns | 1293642ns | 1311490ns | -0.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b6_pred | 2440893ns | 2425002ns | 2464131ns | +86.89% | 0.002 |
| ab_b6_prof | 1319661ns | 1310900ns | 1333423ns | +1.04% | 0.003 |
| ab_b6_seq | 1331231ns | 1315160ns | 1360265ns | +1.93% | 0.003 |
| ab_b6_table | 1306026ns | 1286250ns | 1325709ns | base | 0.003 |
| ab_b6_tree | 1295041ns | 1281546ns | 1308590ns | -0.84% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b6_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b6_pred | 0.002 | 52.7% |
| ab_b6_prof | 0.003 | 97.5% |
| ab_b6_seq | 0.003 | 97.2% |
| ab_b6_table | 0.003 | 98.3% |
| ab_b6_tree | 0.003 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b6_pred | 2444161ns | 2444161ns | +86.71% |
| ab_b6_prof | 1322797ns | 1322797ns | +1.05% |
| ab_b6_seq | 1334368ns | 1334368ns | +1.93% |
| ab_b6_table | 1309086ns | 1309086ns | base |
| ab_b6_tree | 1298103ns | 1298103ns | -0.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b6_table | 1303543ns | base | --- | [1288827, 1325709] | --- | --- | --- | --- |
| ab_b6_pred | 2433470ns | +1135820.5ns (+87.1%) | [+1117499, +1151282]ns | [2425078, 2464131] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b6_prof | 1314571ns | no significant difference | [-12714, +36812]ns | [1310990, 1333423] | no | 0.6875 | 0.6875 | 0 |
| ab_b6_seq | 1317818ns | no significant difference | [-9129, +71438]ns | [1315609, 1360265] | no | 0.4375 | 0.2188 | 0 |
| ab_b6_tree | 1292859ns | no significant difference | [-38006, +11481]ns | [1283675, 1308590] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b6_table | ab_b6_pred | ab_b6_prof | ab_b6_seq | ab_b6_tree |
|---|---|---|---|---|---|
| 1 | 1286250ns | +88.5% | +1.9% | +6.8% | +1.7% |
| 2 | 1316171ns | +87.9% | +0.7% | +0.2% | -2.6% |
| 3 | 1335247ns | +83.9% | -1.6% | -1.5% | -3.1% |
| 4 | 1291403ns | +88.7% | +1.9% | +4.3% | +0.0% |
| 5 | 1314820ns | +84.8% | -0.3% | +0.1% | -0.5% |
| 6 | 1292265ns | +87.7% | +3.8% | +1.8% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b6_pred | 0.053 | ok |
| ab_b6_prof | -0.306 | moderate- |
| ab_b6_seq | -0.205 | moderate- |
| ab_b6_table | -0.318 | moderate- |
| ab_b6_tree | -0.513 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ab_b6_pred**: won 0/6, lost 6/6
- **ab_b6_prof**: won 2/6, lost 4/6
- **ab_b6_seq**: won 1/6, lost 5/6
- **ab_b6_tree**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b6_pred | 30.0ns | 2440893.3ns | 0.0% |  |
| ab_b6_prof | 11.2ns | 1319661.3ns | 0.0% |  |
| ab_b6_seq | 19.0ns | 1331230.6ns | 0.0% |  |
| ab_b6_table | 18.3ns | 1306026.2ns | 0.0% |  |
| ab_b6_tree | 16.1ns | 1295041.2ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b6_pred (n=6, range 2425002.5-2464131.5 ns)
  2425002.5 |########################################
  2426958.9 |
  2428915.4 |####################
  2430871.8 |
  2432828.3 |
  2434784.7 |
  2436741.2 |####################
  2438697.6 |
  2440654.1 |
  2442610.5 |
  2444567.0 |
  2446523.4 |
  2448479.9 |
  2450436.3 |
  2452392.8 |
  2454349.2 |####################
  2456305.7 |
  2458262.1 |
  2460218.6 |
  2462175.0 |
  (0 below, 1 above range)

ab_b6_prof (n=6, range 1310899.6-1333422.9 ns)
  1310899.6 |########################################
  1312025.8 |
  1313151.9 |####################
  1314278.1 |####################
  1315404.3 |
  1316530.4 |
  1317656.6 |
  1318782.8 |
  1319908.9 |
  1321035.1 |
  1322161.2 |
  1323287.4 |
  1324413.6 |
  1325539.7 |####################
  1326665.9 |
  1327792.1 |
  1328918.2 |
  1330044.4 |
  1331170.6 |
  1332296.7 |
  (0 below, 1 above range)

ab_b6_seq (n=6, range 1315160.0-1360265.0 ns)
  1315160.0 |########################################
  1317415.2 |#############
  1319670.5 |
  1321925.8 |
  1324181.0 |
  1326436.2 |
  1328691.5 |
  1330946.8 |
  1333202.0 |
  1335457.2 |
  1337712.5 |
  1339967.8 |
  1342223.0 |
  1344478.2 |
  1346733.5 |#############
  1348988.8 |
  1351244.0 |
  1353499.2 |
  1355754.5 |
  1358009.8 |
  (0 below, 1 above range)

ab_b6_table (n=6, range 1286250.4-1325708.9 ns)
  1286250.4 |########################################
  1288223.3 |
  1290196.3 |########################################
  1292169.2 |########################################
  1294142.1 |
  1296115.0 |
  1298088.0 |
  1300060.9 |
  1302033.8 |
  1304006.7 |
  1305979.7 |
  1307952.6 |
  1309925.5 |
  1311898.5 |
  1313871.4 |########################################
  1315844.3 |########################################
  1317817.2 |
  1319790.2 |
  1321763.1 |
  1323736.0 |
  (0 below, 1 above range)

ab_b6_tree (n=6, range 1281545.8-1308589.6 ns)
  1281545.8 |########################################
  1282898.0 |
  1284250.2 |
  1285602.4 |########################################
  1286954.6 |
  1288306.8 |
  1289658.9 |
  1291011.1 |########################################
  1292363.3 |
  1293715.5 |########################################
  1295067.7 |
  1296419.9 |
  1297772.1 |
  1299124.3 |
  1300476.5 |
  1301828.7 |
  1303180.8 |
  1304533.0 |
  1305885.2 |
  1307237.4 |########################################
  (0 below, 1 above range)

```

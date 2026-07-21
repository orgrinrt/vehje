# Whole-program single strategy (all branches), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_whole_table**

## Highlights

Baseline for all deltas below: **ab_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_whole_pred is an outlier: 4.0x slower than the field

ab_whole_pred (5.22 ms) is 4.0x the fastest (1.30 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_whole_table, ab_whole_tree) are a dead heat (<1%)

ab_whole_table (1.30 ms) and ab_whole_tree (1.31 ms) differ by 0.64%, inside the noise, even though the wider field spreads 301.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_whole_pred shows alternating (throttle bounce) (autocorr -0.52)

ab_whole_pred's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_whole_table)

The baseline ab_whole_table is the fastest (1.30 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_whole_table, ab_whole_tree, ab_whole_seq, ab_whole_prof} vs {ab_whole_pred} (289% apart)

The field splits into a fast tier {ab_whole_table, ab_whole_tree, ab_whole_seq, ab_whole_prof} and a slow tier {ab_whole_pred} with a 289% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest ab_whole_table (1.30 ms) to slowest ab_whole_pred (5.22 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ab_whole_table) is the fastest** at 1300994.4 ns median
- 1 variant significantly slower than baseline
- Spread: 4.01x (fastest 1300994.4 ns, slowest 5222918.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_whole_pred | 5228208ns | 5226758ns | 5199948ns | 5219705ns | 5255093ns | +298.21% |
| ab_whole_prof | 1356087ns | 1344650ns | 1343069ns | 1344323ns | 1380241ns | +3.29% |
| ab_whole_seq | 1343009ns | 1339966ns | 1327574ns | 1337712ns | 1358673ns | +2.29% |
| ab_whole_table | 1312937ns | 1304360ns | 1283420ns | 1300029ns | 1347057ns | base |
| ab_whole_tree | 1312226ns | 1312467ns | 1298920ns | 1308014ns | 1325197ns | -0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_whole_pred | 5224284ns | 5196130ns | 5251166ns | +298.91% | 0.001 |
| ab_whole_prof | 1352947ns | 1340214ns | 1377555ns | +3.31% | 0.003 |
| ab_whole_seq | 1339902ns | 1324209ns | 1355926ns | +2.31% | 0.003 |
| ab_whole_table | 1309653ns | 1280230ns | 1344078ns | base | 0.003 |
| ab_whole_tree | 1309349ns | 1295867ns | 1322754ns | -0.02% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_whole_table; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_whole_pred | 0.001 | 24.5% |
| ab_whole_prof | 0.003 | 95.5% |
| ab_whole_seq | 0.003 | 95.8% |
| ab_whole_table | 0.003 | 98.4% |
| ab_whole_tree | 0.003 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_whole_pred | 5228208ns | 5228208ns | +298.21% |
| ab_whole_prof | 1356087ns | 1356087ns | +3.29% |
| ab_whole_seq | 1343009ns | 1343009ns | +2.29% |
| ab_whole_table | 1312937ns | 1312937ns | base |
| ab_whole_tree | 1312226ns | 1312226ns | -0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_whole_table | 1300994ns | base | --- | [1283887, 1344078] | --- | --- | --- | --- |
| ab_whole_pred | 5222918ns | +3913727.7ns (+300.8%) | [+3884790, +3945375]ns | [5198767, 5251166] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_whole_prof | 1341068ns | no significant difference | [-3860, +76561]ns | [1340218, 1377555] | no | 0.2917 | 0.2188 | 0 |
| ab_whole_seq | 1336645ns | no significant difference | [-11194, +62095]ns | [1327135, 1355926] | no | 0.2917 | 0.2188 | 0 |
| ab_whole_tree | 1309286ns | no significant difference | [-42577, +35361]ns | [1296007, 1322754] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_whole_table | ab_whole_pred | ab_whole_prof | ab_whole_seq | ab_whole_tree |
|---|---|---|---|---|---|
| 1 | 1280230ns | +305.9% | +4.8% | +5.0% | +2.7% |
| 2 | 1376298ns | +281.6% | -2.6% | -2.9% | -5.0% |
| 3 | 1307432ns | +297.8% | +6.0% | +4.6% | -0.9% |
| 4 | 1294557ns | +303.4% | +5.8% | +3.3% | +2.8% |
| 5 | 1287543ns | +307.8% | +4.1% | +2.8% | +1.9% |
| 6 | 1311859ns | +298.2% | +2.2% | +1.4% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_whole_pred | -0.520 | HIGH- (thermal bounce) |
| ab_whole_prof | 0.117 | ok |
| ab_whole_seq | -0.006 | ok |
| ab_whole_table | -0.297 | moderate- |
| ab_whole_tree | -0.292 | moderate- |

**Consistency summary:**

- **ab_whole_pred**: won 0/6, lost 6/6
- **ab_whole_prof**: won 1/6, lost 5/6
- **ab_whole_seq**: won 1/6, lost 5/6
- **ab_whole_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_whole_pred | 76.1ns | 5224283.8ns | 0.0% |  |
| ab_whole_prof | 19.0ns | 1352947.0ns | 0.0% |  |
| ab_whole_seq | 19.0ns | 1339902.3ns | 0.0% |  |
| ab_whole_table | 16.8ns | 1309653.1ns | 0.0% |  |
| ab_whole_tree | 18.7ns | 1309349.0ns | 0.0% |  |

## Distribution (algo ns)

```
ab_whole_pred (n=6, range 5196130.0-5251166.1 ns)
  5196130.0 |####################
  5198881.8 |####################
  5201633.6 |
  5204385.4 |
  5207137.2 |
  5209889.0 |
  5212640.8 |
  5215392.6 |
  5218144.4 |
  5220896.2 |########################################
  5223648.0 |
  5226399.8 |
  5229151.6 |
  5231903.4 |
  5234655.2 |
  5237407.0 |
  5240158.8 |
  5242910.6 |
  5245662.4 |
  5248414.2 |####################
  (0 below, 1 above range)

ab_whole_prof (n=6, range 1340214.2-1377555.0 ns)
  1340214.2 |########################################
  1342081.2 |
  1343948.3 |
  1345815.3 |
  1347682.4 |
  1349549.4 |
  1351416.4 |
  1353283.5 |
  1355150.5 |
  1357017.6 |
  1358884.6 |
  1360751.6 |
  1362618.7 |
  1364485.7 |
  1366352.8 |
  1368219.8 |##########
  1370086.8 |
  1371953.9 |
  1373820.9 |
  1375688.0 |
  (0 below, 1 above range)

ab_whole_seq (n=6, range 1324209.2-1355926.4 ns)
  1324209.2 |########################################
  1325795.1 |
  1327380.9 |
  1328966.8 |########################################
  1330552.6 |
  1332138.5 |
  1333724.4 |
  1335310.2 |########################################
  1336896.1 |########################################
  1338482.0 |
  1340067.8 |
  1341653.7 |
  1343239.6 |
  1344825.4 |########################################
  1346411.3 |
  1347997.1 |
  1349583.0 |
  1351168.9 |
  1352754.7 |
  1354340.6 |
  (0 below, 1 above range)

ab_whole_table (n=6, range 1280230.0-1344078.1 ns)
  1280230.0 |########################################
  1283422.4 |
  1286614.8 |########################################
  1289807.2 |
  1292999.6 |########################################
  1296192.0 |
  1299384.4 |
  1302576.9 |
  1305769.3 |########################################
  1308961.7 |########################################
  1312154.1 |
  1315346.5 |
  1318538.9 |
  1321731.3 |
  1324923.7 |
  1328116.1 |
  1331308.5 |
  1334500.9 |
  1337693.3 |
  1340885.7 |
  (0 below, 1 above range)

ab_whole_tree (n=6, range 1295867.1-1322754.4 ns)
  1295867.1 |########################################
  1297211.5 |
  1298555.8 |
  1299900.2 |
  1301244.6 |
  1302588.9 |
  1303933.3 |
  1305277.6 |
  1306622.0 |####################
  1307966.4 |
  1309310.7 |
  1310655.1 |####################
  1311999.5 |
  1313343.8 |
  1314688.2 |####################
  1316032.5 |
  1317376.9 |
  1318721.3 |
  1320065.6 |
  1321410.0 |
  (0 below, 1 above range)

```

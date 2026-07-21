# Per-type strategy: all ifchain branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_ifchain_table**

## Highlights

Baseline for all deltas below: **ab_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_ifchain_pred is an outlier: 2.3x slower than the field

ab_ifchain_pred (2.91 ms) is 2.3x the fastest (1.29 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_ifchain_table, ab_ifchain_tree) are a dead heat (<1%)

ab_ifchain_table (1.29 ms) and ab_ifchain_tree (1.29 ms) differ by 0.04%, inside the noise, even though the wider field spreads 125.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_ifchain_pred shows alternating (throttle bounce) (autocorr -0.69)

ab_ifchain_pred's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_ifchain_table)

The baseline ab_ifchain_table is the fastest (1.29 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_ifchain_table, ab_ifchain_tree, ab_ifchain_seq, ab_ifchain_prof} vs {ab_ifchain_pred} (120% apart)

The field splits into a fast tier {ab_ifchain_table, ab_ifchain_tree, ab_ifchain_seq, ab_ifchain_prof} and a slow tier {ab_ifchain_pred} with a 120% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_ifchain_table) is the fastest** at 1289778.8 ns median
- 3 variants significantly slower than baseline
- Spread: 2.26x (fastest 1289778.8 ns, slowest 2911063.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_ifchain_pred | 2918787ns | 2914655ns | 2882195ns | 2909242ns | 2951401ns | +125.13% |
| ab_ifchain_prof | 1326622ns | 1325961ns | 1323516ns | 1325872ns | 1329299ns | +2.32% |
| ab_ifchain_seq | 1330780ns | 1317911ns | 1313454ns | 1316874ns | 1360302ns | +2.64% |
| ab_ifchain_table | 1296514ns | 1292984ns | 1286405ns | 1292072ns | 1308230ns | base |
| ab_ifchain_tree | 1295378ns | 1293416ns | 1277038ns | 1290447ns | 1311944ns | -0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_ifchain_pred | 2915433ns | 2878737ns | 2948380ns | +125.41% | 0.001 |
| ab_ifchain_prof | 1323387ns | 1320142ns | 1325779ns | +2.32% | 0.003 |
| ab_ifchain_seq | 1327700ns | 1310267ns | 1357336ns | +2.65% | 0.003 |
| ab_ifchain_table | 1293376ns | 1283055ns | 1305481ns | base | 0.003 |
| ab_ifchain_tree | 1292347ns | 1273685ns | 1309451ns | -0.08% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_ifchain_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_ifchain_pred | 0.001 | 43.8% |
| ab_ifchain_prof | 0.003 | 96.3% |
| ab_ifchain_seq | 0.003 | 96.9% |
| ab_ifchain_table | 0.003 | 98.8% |
| ab_ifchain_tree | 0.003 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_ifchain_pred | 2918787ns | 2918787ns | +125.13% |
| ab_ifchain_prof | 1326622ns | 1326622ns | +2.32% |
| ab_ifchain_seq | 1330780ns | 1330780ns | +2.64% |
| ab_ifchain_table | 1296514ns | 1296514ns | base |
| ab_ifchain_tree | 1295378ns | 1295378ns | -0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_ifchain_table | 1289779ns | base | --- | [1284869, 1305481] | --- | --- | --- | --- |
| ab_ifchain_pred | 2911064ns | +1610816.5ns (+124.9%) | [+1592328, +1663025]ns | [2886855, 2948380] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_ifchain_prof | 1322970ns | +32243.8ns (+2.5%) | [+19688, +38101]ns | [1321413, 1325779] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_ifchain_seq | 1314781ns | +27301.7ns (+2.1%) | [+10249, +65419]ns | [1310982, 1357336] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_ifchain_tree | 1290317ns | no significant difference | [-23461, +14923]ns | [1277272, 1309451] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_ifchain_table | ab_ifchain_pred | ab_ifchain_prof | ab_ifchain_seq | ab_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 1297151ns | +123.2% | +2.0% | +4.7% | +0.9% |
| 2 | 1313812ns | +122.1% | +1.1% | -0.2% | -2.5% |
| 3 | 1287654ns | +128.6% | +2.5% | +1.8% | -1.1% |
| 4 | 1291903ns | +122.8% | +2.5% | +1.9% | +1.4% |
| 5 | 1283055ns | +130.2% | +3.1% | +2.3% | -0.0% |
| 6 | 1286682ns | +125.7% | +2.8% | +5.4% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_ifchain_pred | -0.687 | HIGH- (thermal bounce) |
| ab_ifchain_prof | -0.618 | HIGH- (thermal bounce) |
| ab_ifchain_seq | -0.109 | ok |
| ab_ifchain_table | 0.086 | ok |
| ab_ifchain_tree | -0.441 | moderate- |

**Consistency summary:**

- **ab_ifchain_pred**: won 0/6, lost 6/6
- **ab_ifchain_prof**: won 0/6, lost 6/6
- **ab_ifchain_seq**: won 1/6, lost 5/6
- **ab_ifchain_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_ifchain_pred | 32.3ns | 2915432.8ns | 0.0% |  |
| ab_ifchain_prof | 15.0ns | 1323387.4ns | 0.0% |  |
| ab_ifchain_seq | 14.9ns | 1327699.6ns | 0.0% |  |
| ab_ifchain_table | 29.6ns | 1293376.3ns | 0.0% |  |
| ab_ifchain_tree | 14.0ns | 1292346.6ns | 0.0% |  |

## Distribution (algo ns)

```
ab_ifchain_pred (n=6, range 2878736.7-2948380.0 ns)
  2878736.7 |########################################
  2882218.9 |
  2885701.0 |
  2889183.2 |
  2892665.4 |########################################
  2896147.5 |
  2899629.7 |
  2903111.9 |########################################
  2906594.0 |
  2910076.2 |
  2913558.4 |
  2917040.5 |########################################
  2920522.7 |
  2924004.8 |
  2927487.0 |
  2930969.2 |
  2934451.3 |
  2937933.5 |
  2941415.7 |########################################
  2944897.8 |
  (0 below, 1 above range)

ab_ifchain_prof (n=6, range 1320142.1-1325778.9 ns)
  1320142.1 |####################
  1320423.9 |
  1320705.8 |
  1320987.6 |
  1321269.5 |
  1321551.3 |
  1321833.2 |
  1322115.0 |
  1322396.8 |
  1322678.7 |########################################
  1322960.5 |####################
  1323242.4 |
  1323524.2 |
  1323806.1 |####################
  1324087.9 |
  1324369.7 |
  1324651.6 |
  1324933.4 |
  1325215.3 |
  1325497.1 |
  (0 below, 1 above range)

ab_ifchain_seq (n=6, range 1310267.1-1357335.9 ns)
  1310267.1 |########################################
  1312620.5 |####################
  1314974.0 |####################
  1317327.4 |
  1319680.9 |
  1322034.3 |
  1324387.7 |
  1326741.2 |
  1329094.6 |
  1331448.0 |
  1333801.5 |
  1336154.9 |
  1338508.4 |
  1340861.8 |
  1343215.2 |
  1345568.7 |
  1347922.1 |
  1350275.5 |
  1352629.0 |
  1354982.4 |####################
  (0 below, 1 above range)

ab_ifchain_table (n=6, range 1283055.0-1305481.4 ns)
  1283055.0 |########################################
  1284176.3 |
  1285297.6 |
  1286419.0 |########################################
  1287540.3 |########################################
  1288661.6 |
  1289782.9 |
  1290904.3 |########################################
  1292025.6 |
  1293146.9 |
  1294268.2 |
  1295389.5 |
  1296510.9 |########################################
  1297632.2 |
  1298753.5 |
  1299874.8 |
  1300996.2 |
  1302117.5 |
  1303238.8 |
  1304360.1 |
  (0 below, 1 above range)

ab_ifchain_tree (n=6, range 1273684.6-1309450.6 ns)
  1273684.6 |####################
  1275472.9 |
  1277261.2 |
  1279049.5 |
  1280837.8 |########################################
  1282626.1 |
  1284414.4 |
  1286202.7 |
  1287991.0 |
  1289779.3 |
  1291567.6 |
  1293355.9 |
  1295144.2 |
  1296932.5 |####################
  1298720.8 |
  1300509.1 |
  1302297.4 |
  1304085.7 |
  1305874.0 |
  1307662.3 |####################
  (0 below, 1 above range)

```

# Per-branch strategy: archetype 2 (match8_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b2_table**

## Highlights

Baseline for all deltas below: **ab_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b2_table, ab_b2_seq) are a dead heat (<1%)

ab_b2_table (1.29 ms) and ab_b2_seq (1.29 ms) differ by 0.03%, inside the noise, even though the wider field spreads 41.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b2_seq shows alternating (throttle bounce) (autocorr -0.62)

ab_b2_seq's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b2_table)

The baseline ab_b2_table is the fastest (1.29 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_b2_table, ab_b2_seq, ab_b2_tree, ab_b2_prof} vs {ab_b2_pred} (41% apart)

The field splits into a fast tier {ab_b2_table, ab_b2_seq, ab_b2_tree, ab_b2_prof} and a slow tier {ab_b2_pred} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_b2_table) is the fastest** at 1292629.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.42x (fastest 1292629.8 ns, slowest 1833866.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b2_pred | 1840694ns | 1837390ns | 1815936ns | 1832700ns | 1865065ns | +41.52% |
| ab_b2_prof | 1310885ns | 1307031ns | 1297733ns | 1305809ns | 1325074ns | +0.79% |
| ab_b2_seq | 1299535ns | 1296282ns | 1288538ns | 1293701ns | 1313784ns | -0.09% |
| ab_b2_table | 1300661ns | 1295809ns | 1287255ns | 1293535ns | 1318052ns | base |
| ab_b2_tree | 1301380ns | 1300842ns | 1293828ns | 1298564ns | 1309380ns | +0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b2_pred | 1837421ns | 1812708ns | 1862010ns | +41.60% | 0.002 |
| ab_b2_prof | 1307683ns | 1294371ns | 1322184ns | +0.78% | 0.003 |
| ab_b2_seq | 1296479ns | 1285179ns | 1311132ns | -0.09% | 0.003 |
| ab_b2_table | 1297608ns | 1283980ns | 1315318ns | base | 0.003 |
| ab_b2_tree | 1298021ns | 1290392ns | 1305855ns | +0.03% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b2_table; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b2_pred | 0.002 | 70.0% |
| ab_b2_prof | 0.003 | 98.5% |
| ab_b2_seq | 0.003 | 99.3% |
| ab_b2_table | 0.003 | 99.3% |
| ab_b2_tree | 0.003 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b2_pred | 1840694ns | 1840694ns | +41.52% |
| ab_b2_prof | 1310885ns | 1310885ns | +0.79% |
| ab_b2_seq | 1299535ns | 1299535ns | -0.09% |
| ab_b2_table | 1300661ns | 1300661ns | base |
| ab_b2_tree | 1301380ns | 1301380ns | +0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b2_table | 1292630ns | base | --- | [1284875, 1315318] | --- | --- | --- | --- |
| ab_b2_pred | 1833866ns | +533813.3ns (+41.3%) | [+524515, +561111]ns | [1816386, 1862010] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b2_prof | 1303548ns | no significant difference | [-15220, +35235]ns | [1297316, 1322184] | no | 0.9167 | 0.6875 | 0 |
| ab_b2_seq | 1293047ns | no significant difference | [-13142, +6129]ns | [1285258, 1311132] | no | 0.4375 | 0.2188 | 0 |
| ab_b2_tree | 1297667ns | no significant difference | [-14649, +13705]ns | [1290540, 1305855] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b2_table | ab_b2_pred | ab_b2_prof | ab_b2_seq | ab_b2_tree |
|---|---|---|---|---|---|
| 1 | 1297131ns | +41.3% | +0.3% | +0.2% | -0.5% |
| 2 | 1312819ns | +39.8% | -1.4% | -2.1% | -0.2% |
| 3 | 1317818ns | +42.0% | -0.9% | +0.4% | -1.7% |
| 4 | 1283980ns | +44.3% | +1.3% | +0.1% | +0.5% |
| 5 | 1288129ns | +41.3% | +3.4% | +0.6% | +1.0% |
| 6 | 1285770ns | +41.0% | +2.0% | +0.4% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b2_pred | 0.263 | moderate+ |
| ab_b2_prof | 0.053 | ok |
| ab_b2_seq | -0.622 | HIGH- (thermal bounce) |
| ab_b2_table | 0.252 | moderate+ |
| ab_b2_tree | -0.418 | moderate- |

**Consistency summary:**

- **ab_b2_pred**: won 0/6, lost 6/6
- **ab_b2_prof**: won 2/6, lost 4/6
- **ab_b2_seq**: won 1/6, lost 4/6
- **ab_b2_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b2_pred | 24.6ns | 1837420.8ns | 0.0% |  |
| ab_b2_prof | 14.9ns | 1307683.0ns | 0.0% |  |
| ab_b2_seq | 11.5ns | 1296478.8ns | 0.0% |  |
| ab_b2_table | 21.0ns | 1297607.8ns | 0.0% |  |
| ab_b2_tree | 14.3ns | 1298020.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b2_pred (n=6, range 1812707.9-1862010.0 ns)
  1812707.9 |########################################
  1815173.0 |
  1817638.1 |########################################
  1820103.2 |
  1822568.3 |
  1825033.4 |
  1827498.5 |
  1829963.6 |
  1832428.7 |########################################
  1834893.8 |########################################
  1837358.9 |
  1839824.1 |
  1842289.2 |
  1844754.3 |
  1847219.4 |
  1849684.5 |
  1852149.6 |########################################
  1854614.7 |
  1857079.8 |
  1859544.9 |
  (0 below, 1 above range)

ab_b2_prof (n=6, range 1294371.2-1322184.1 ns)
  1294371.2 |####################
  1295761.8 |
  1297152.5 |
  1298543.1 |
  1299933.8 |########################################
  1301324.4 |
  1302715.1 |
  1304105.7 |
  1305496.4 |####################
  1306887.0 |
  1308277.7 |
  1309668.3 |
  1311059.0 |####################
  1312449.6 |
  1313840.3 |
  1315230.9 |
  1316621.6 |
  1318012.2 |
  1319402.9 |
  1320793.5 |
  (0 below, 1 above range)

ab_b2_seq (n=6, range 1285178.8-1311131.7 ns)
  1285178.8 |########################################
  1286476.4 |
  1287774.1 |
  1289071.7 |####################
  1290369.4 |
  1291667.0 |
  1292964.7 |
  1294262.3 |
  1295560.0 |####################
  1296857.6 |
  1298155.2 |
  1299452.9 |####################
  1300750.5 |
  1302048.2 |
  1303345.8 |
  1304643.5 |
  1305941.1 |
  1307238.8 |
  1308536.4 |
  1309834.1 |
  (0 below, 1 above range)

ab_b2_table (n=6, range 1283980.0-1315318.4 ns)
  1283980.0 |########################################
  1285546.9 |########################################
  1287113.8 |########################################
  1288680.8 |
  1290247.7 |
  1291814.6 |
  1293381.5 |
  1294948.4 |
  1296515.3 |########################################
  1298082.3 |
  1299649.2 |
  1301216.1 |
  1302783.0 |
  1304349.9 |
  1305916.8 |
  1307483.8 |
  1309050.7 |
  1310617.6 |
  1312184.5 |########################################
  1313751.4 |
  (0 below, 1 above range)

ab_b2_tree (n=6, range 1290392.5-1305855.0 ns)
  1290392.5 |########################################
  1291165.6 |
  1291938.8 |
  1292711.9 |
  1293485.0 |
  1294258.1 |####################
  1295031.2 |
  1295804.4 |
  1296577.5 |
  1297350.6 |
  1298123.8 |
  1298896.9 |
  1299670.0 |####################
  1300443.1 |####################
  1301216.2 |
  1301989.4 |
  1302762.5 |
  1303535.6 |
  1304308.8 |
  1305081.9 |
  (0 below, 1 above range)

```

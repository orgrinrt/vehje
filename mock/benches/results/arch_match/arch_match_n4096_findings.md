# Per-type strategy: all match branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_match_table**

## Highlights

Baseline for all deltas below: **ab_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_match_pred is an outlier: 2.7x slower than the field

ab_match_pred (3.57 ms) is 2.7x the fastest (1.30 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_match_table, ab_match_seq) are a dead heat (<1%)

ab_match_table (1.30 ms) and ab_match_seq (1.31 ms) differ by 0.20%, inside the noise, even though the wider field spreads 174.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_match_seq shows alternating (throttle bounce) (autocorr -0.55)

ab_match_seq's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_match_table)

The baseline ab_match_table is the fastest (1.30 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_match_table, ab_match_seq, ab_match_tree, ab_match_prof} vs {ab_match_pred} (173% apart)

The field splits into a fast tier {ab_match_table, ab_match_seq, ab_match_tree, ab_match_prof} and a slow tier {ab_match_pred} with a 173% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### ab_match_seq's edge over baseline is significant but tiny (8 ns, 0.00%)

ab_match_seq differs from baseline ab_match_table by 8 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (ab_match_table) is the fastest** at 1302879.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.74x (fastest 1302879.5 ns, slowest 3574231.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_match_pred | 3581944ns | 3577634ns | 3560440ns | 3572729ns | 3606518ns | +174.29% |
| ab_match_prof | 1316795ns | 1313923ns | 1305550ns | 1311530ns | 1330314ns | +0.83% |
| ab_match_seq | 1306837ns | 1308782ns | 1295904ns | 1304800ns | 1315359ns | +0.07% |
| ab_match_table | 1305893ns | 1306110ns | 1298582ns | 1303959ns | 1312449ns | base |
| ab_match_tree | 1311157ns | 1311745ns | 1303089ns | 1309249ns | 1318052ns | +0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_match_pred | 3578454ns | 3556427ns | 3603172ns | +174.68% | 0.001 |
| ab_match_prof | 1313657ns | 1302462ns | 1327401ns | +0.83% | 0.003 |
| ab_match_seq | 1303661ns | 1292355ns | 1312372ns | +0.07% | 0.003 |
| ab_match_table | 1302793ns | 1295265ns | 1309726ns | base | 0.003 |
| ab_match_tree | 1307990ns | 1300137ns | 1314706ns | +0.40% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_match_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_match_pred | 0.001 | 36.2% |
| ab_match_prof | 0.003 | 98.6% |
| ab_match_seq | 0.003 | 99.0% |
| ab_match_table | 0.003 | 99.2% |
| ab_match_tree | 0.003 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_match_pred | 3581944ns | 3581944ns | +174.29% |
| ab_match_prof | 1316795ns | 1316795ns | +0.83% |
| ab_match_seq | 1306837ns | 1306837ns | +0.07% |
| ab_match_table | 1305893ns | 1305893ns | base |
| ab_match_tree | 1311157ns | 1311157ns | +0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_match_table | 1302880ns | base | --- | [1295773, 1309726] | --- | --- | --- | --- |
| ab_match_pred | 3574231ns | +2276543.2ns (+174.7%) | [+2255170, +2295271]ns | [3557959, 3603172] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_match_prof | 1310691ns | no significant difference | [-4399, +31628]ns | [1302878, 1327401] | no | 0.9167 | 0.6875 | 0 |
| ab_match_seq | 1305515ns | no significant difference | [-6475, +9072]ns | [1293096, 1312372] | no | 1.0000 | 1.0000 | 0 |
| ab_match_tree | 1308667ns | no significant difference | [-6875, +16381]ns | [1300598, 1314706] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_match_table | ab_match_pred | ab_match_prof | ab_match_seq | ab_match_tree |
|---|---|---|---|---|---|
| 1 | 1305648ns | +177.0% | +0.6% | -0.4% | -0.4% |
| 2 | 1310155ns | +174.0% | -0.1% | +0.2% | +0.3% |
| 3 | 1300111ns | +175.2% | +0.2% | -0.6% | +1.2% |
| 4 | 1295265ns | +175.7% | +2.2% | +1.2% | +0.7% |
| 5 | 1309296ns | +171.9% | -0.5% | +0.2% | -0.7% |
| 6 | 1296282ns | +174.4% | +2.7% | -0.2% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_match_pred | 0.389 | moderate+ |
| ab_match_prof | -0.518 | HIGH- (thermal bounce) |
| ab_match_seq | -0.545 | HIGH- (thermal bounce) |
| ab_match_table | -0.331 | moderate- |
| ab_match_tree | -0.146 | ok |

**Consistency summary:**

- **ab_match_pred**: won 0/6, lost 6/6
- **ab_match_prof**: won 2/6, lost 4/6
- **ab_match_seq**: won 3/6, lost 3/6
- **ab_match_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_match_pred | 46.3ns | 3578454.0ns | 0.0% |  |
| ab_match_prof | 20.6ns | 1313656.9ns | 0.0% |  |
| ab_match_seq | 17.8ns | 1303661.3ns | 0.0% |  |
| ab_match_table | 12.6ns | 1302792.8ns | 0.0% |  |
| ab_match_tree | 16.5ns | 1307990.2ns | 0.0% |  |

## Distribution (algo ns)

```
ab_match_pred (n=6, range 3556426.7-3603172.1 ns)
  3556426.7 |########################################
  3558764.0 |########################################
  3561101.2 |
  3563438.5 |
  3565775.8 |
  3568113.1 |
  3570450.3 |########################################
  3572787.6 |
  3575124.9 |
  3577462.1 |########################################
  3579799.4 |
  3582136.7 |
  3584473.9 |
  3586811.2 |
  3589148.5 |########################################
  3591485.8 |
  3593823.0 |
  3596160.3 |
  3598497.6 |
  3600834.8 |
  (0 below, 1 above range)

ab_match_prof (n=6, range 1302462.1-1327401.4 ns)
  1302462.1 |########################################
  1303709.1 |
  1304956.0 |
  1306203.0 |
  1307450.0 |####################
  1308696.9 |
  1309943.9 |
  1311190.9 |
  1312437.8 |####################
  1313684.8 |
  1314931.8 |
  1316178.7 |
  1317425.7 |
  1318672.7 |
  1319919.6 |
  1321166.6 |
  1322413.6 |####################
  1323660.5 |
  1324907.5 |
  1326154.5 |
  (0 below, 1 above range)

ab_match_seq (n=6, range 1292355.4-1312372.1 ns)
  1292355.4 |########################################
  1293356.2 |########################################
  1294357.1 |
  1295357.9 |
  1296358.7 |
  1297359.6 |
  1298360.4 |
  1299361.2 |
  1300362.1 |########################################
  1301362.9 |
  1302363.8 |
  1303364.6 |
  1304365.4 |
  1305366.3 |
  1306367.1 |
  1307367.9 |
  1308368.8 |
  1309369.6 |
  1310370.4 |########################################
  1311371.3 |########################################
  (0 below, 1 above range)

ab_match_table (n=6, range 1295264.6-1309725.6 ns)
  1295264.6 |########################################
  1295987.7 |########################################
  1296710.7 |
  1297433.8 |
  1298156.8 |
  1298879.9 |
  1299602.9 |########################################
  1300326.0 |
  1301049.0 |
  1301772.1 |
  1302495.1 |
  1303218.2 |
  1303941.2 |
  1304664.2 |
  1305387.3 |########################################
  1306110.4 |
  1306833.4 |
  1307556.5 |
  1308279.5 |
  1309002.6 |########################################
  (0 below, 1 above range)

ab_match_tree (n=6, range 1300137.1-1314705.9 ns)
  1300137.1 |########################################
  1300865.5 |########################################
  1301594.0 |
  1302322.4 |
  1303050.9 |
  1303779.3 |########################################
  1304507.7 |
  1305236.2 |
  1305964.6 |
  1306693.0 |
  1307421.5 |
  1308149.9 |
  1308878.4 |
  1309606.8 |
  1310335.2 |
  1311063.7 |
  1311792.1 |
  1312520.5 |########################################
  1313249.0 |########################################
  1313977.4 |
  (0 below, 1 above range)

```

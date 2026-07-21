# Whole-program single strategy (all branches), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_whole_table**

## Highlights

Baseline for all deltas below: **ab_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_whole_pred is an outlier: 4.0x slower than the field

ab_whole_pred (1.31 ms) is 4.0x the fastest (326.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ab_whole_table)

The baseline ab_whole_table is the fastest (326.23 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_whole_table, ab_whole_tree, ab_whole_seq, ab_whole_prof} vs {ab_whole_pred} (285% apart)

The field splits into a fast tier {ab_whole_table, ab_whole_tree, ab_whole_seq, ab_whole_prof} and a slow tier {ab_whole_pred} with a 285% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest ab_whole_table (326.23 us) to slowest ab_whole_pred (1.31 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ab_whole_table) is the fastest** at 326232.5 ns median
- 3 variants significantly slower than baseline
- Spread: 4.03x (fastest 326232.5 ns, slowest 1313861.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_whole_pred | 1319008ns | 1317024ns | 1300420ns | 1314498ns | 1335066ns | +300.64% |
| ab_whole_prof | 344098ns | 343757ns | 335998ns | 343242ns | 349433ns | +4.52% |
| ab_whole_seq | 338814ns | 336336ns | 334780ns | 335836ns | 345300ns | +2.91% |
| ab_whole_table | 329221ns | 328816ns | 322797ns | 328161ns | 334024ns | base |
| ab_whole_tree | 334558ns | 334959ns | 325593ns | 333873ns | 340068ns | +1.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_whole_pred | 1315809ns | 1296911ns | 1332234ns | +302.89% | 0.001 |
| ab_whole_prof | 341523ns | 333385ns | 346808ns | +4.57% | 0.003 |
| ab_whole_seq | 336101ns | 332107ns | 342602ns | +2.91% | 0.003 |
| ab_whole_table | 326593ns | 320381ns | 331328ns | base | 0.003 |
| ab_whole_tree | 332027ns | 322958ns | 337647ns | +1.66% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_whole_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_whole_pred | 0.001 | 24.4% |
| ab_whole_prof | 0.003 | 93.9% |
| ab_whole_seq | 0.003 | 96.1% |
| ab_whole_table | 0.003 | 98.2% |
| ab_whole_tree | 0.003 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_whole_pred | 1319008ns | 1319008ns | +300.64% |
| ab_whole_prof | 344098ns | 344098ns | +4.52% |
| ab_whole_seq | 338814ns | 338814ns | +2.91% |
| ab_whole_table | 329221ns | 329221ns | base |
| ab_whole_tree | 334558ns | 334558ns | +1.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_whole_table | 326232ns | base | --- | [322218, 331328] | --- | --- | --- | --- |
| ab_whole_pred | 1313862ns | +986475.8ns (+302.4%) | [+972975, +1008197]ns | [1301330, 1332234] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_prof | 341246ns | +14751.0ns (+4.5%) | [+11343, +18695]ns | [336513, 346808] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_seq | 333452ns | +8198.5ns (+2.5%) | [+1760, +18565]ns | [332248, 342602] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_tree | 332383ns | no significant difference | [-1336, +14301]ns | [326050, 337647] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_whole_table | ab_whole_pred | ab_whole_prof | ab_whole_seq | ab_whole_tree |
|---|---|---|---|---|---|
| 1 | 320381ns | +312.2% | +4.1% | +7.3% | +3.6% |
| 2 | 324054ns | +302.9% | +4.8% | +2.7% | +5.2% |
| 3 | 327693ns | +310.1% | +5.9% | +4.2% | +2.0% |
| 4 | 324772ns | +303.9% | +5.6% | +2.3% | -0.6% |
| 5 | 330000ns | +298.7% | +2.9% | +0.6% | -0.3% |
| 6 | 332656ns | +289.9% | +4.2% | +0.4% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_whole_pred | -0.345 | moderate- |
| ab_whole_prof | 0.003 | ok |
| ab_whole_seq | -0.297 | moderate- |
| ab_whole_table | 0.260 | moderate+ |
| ab_whole_tree | 0.133 | ok |

**Consistency summary:**

- **ab_whole_pred**: won 0/6, lost 6/6
- **ab_whole_prof**: won 0/6, lost 6/6
- **ab_whole_seq**: won 0/6, lost 6/6
- **ab_whole_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_whole_pred | 22.2ns | 1315808.6ns | 0.0% |  |
| ab_whole_prof | 6.4ns | 341522.6ns | 0.0% |  |
| ab_whole_seq | 6.2ns | 336100.9ns | 0.0% |  |
| ab_whole_table | 8.3ns | 326592.8ns | 0.0% |  |
| ab_whole_tree | 5.2ns | 332026.5ns | 0.0% |  |

## Distribution (algo ns)

```
ab_whole_pred (n=6, range 1296910.8-1332233.9 ns)
  1296910.8 |########################################
  1298677.0 |
  1300443.1 |
  1302209.3 |
  1303975.4 |
  1305741.6 |########################################
  1307507.7 |
  1309273.9 |
  1311040.1 |########################################
  1312806.2 |
  1314572.4 |########################################
  1316338.5 |
  1318104.7 |
  1319870.8 |########################################
  1321637.0 |
  1323403.2 |
  1325169.3 |
  1326935.5 |
  1328701.6 |
  1330467.8 |
  (0 below, 1 above range)

ab_whole_prof (n=6, range 333385.0-346808.5 ns)
  333385.0 |####################
  334056.2 |
  334727.3 |
  335398.5 |
  336069.7 |
  336740.9 |
  337412.0 |
  338083.2 |
  338754.4 |
  339425.6 |########################################
  340096.8 |
  340767.9 |
  341439.1 |
  342110.3 |
  342781.5 |####################
  343452.6 |
  344123.8 |
  344795.0 |
  345466.2 |
  346137.3 |####################
  (0 below, 1 above range)

ab_whole_seq (n=6, range 332106.7-342602.5 ns)
  332106.7 |########################################
  332631.5 |####################
  333156.3 |
  333681.1 |####################
  334205.9 |
  334730.7 |
  335255.4 |
  335780.2 |
  336305.0 |
  336829.8 |
  337354.6 |
  337879.4 |
  338404.2 |
  338929.0 |
  339453.8 |
  339978.5 |
  340503.3 |
  341028.1 |####################
  341552.9 |
  342077.7 |
  (0 below, 1 above range)

ab_whole_table (n=6, range 320380.8-331328.3 ns)
  320380.8 |########################################
  320928.2 |
  321475.5 |
  322022.9 |
  322570.3 |
  323117.7 |
  323665.0 |########################################
  324212.4 |
  324759.8 |########################################
  325307.2 |
  325854.6 |
  326401.9 |
  326949.3 |
  327496.7 |########################################
  328044.1 |
  328591.4 |
  329138.8 |
  329686.2 |########################################
  330233.6 |
  330780.9 |
  (0 below, 1 above range)

ab_whole_tree (n=6, range 322957.9-337647.1 ns)
  322957.9 |########################################
  323692.4 |
  324426.8 |
  325161.3 |
  325895.7 |
  326630.2 |
  327364.7 |
  328099.1 |
  328833.6 |########################################
  329568.0 |
  330302.5 |
  331037.0 |
  331771.4 |########################################
  332505.9 |########################################
  333240.3 |
  333974.8 |########################################
  334709.3 |
  335443.7 |
  336178.2 |
  336912.6 |
  (0 below, 1 above range)

```

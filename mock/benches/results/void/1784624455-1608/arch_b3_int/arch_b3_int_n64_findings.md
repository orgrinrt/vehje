# Per-branch strategy: archetype 3 (match4-blocks (long sequential arms)), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b3_pred_int**

## Key findings

- **Fastest: ab_b3_tree_int** at 12709.4 ns median (-63.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.76x (fastest 12709.4 ns, slowest 35042.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b3_pred_int | 35888ns | 37649ns | 31555ns | 35741ns | 38276ns | base |
| ab_b3_prof_int | 15180ns | 15654ns | 13207ns | 14962ns | 16492ns | -57.70% |
| ab_b3_seq_int | 14990ns | 15641ns | 13111ns | 14803ns | 16209ns | -58.23% |
| ab_b3_table_int | 15064ns | 15582ns | 13315ns | 14833ns | 16285ns | -58.03% |
| ab_b3_tree_int | 14971ns | 15233ns | 13542ns | 14878ns | 15825ns | -58.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b3_pred_int | 33394ns | 29357ns | 35618ns | base | 0.002 |
| ab_b3_prof_int | 12662ns | 11025ns | 13757ns | -62.08% | 0.005 |
| ab_b3_seq_int | 12485ns | 10923ns | 13508ns | -62.61% | 0.005 |
| ab_b3_table_int | 12547ns | 11007ns | 13584ns | -62.43% | 0.005 |
| ab_b3_tree_int | 12487ns | 11296ns | 13198ns | -62.61% | 0.005 |

## Performance model

- Peak throughput: **0.006 Gops/s** (ab_b3_seq_int; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b3_pred_int | 0.002 | 31.2% |
| ab_b3_prof_int | 0.005 | 83.7% |
| ab_b3_seq_int | 0.005 | 83.9% |
| ab_b3_table_int | 0.005 | 84.0% |
| ab_b3_tree_int | 0.005 | 85.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b3_pred_int | 35888ns | 35888ns | base |
| ab_b3_prof_int | 15180ns | 15180ns | -57.70% |
| ab_b3_seq_int | 14990ns | 14990ns | -58.23% |
| ab_b3_table_int | 15064ns | 15064ns | -58.03% |
| ab_b3_tree_int | 14971ns | 14971ns | -58.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b3_pred_int | 35043ns | base | --- | [29523, 35618] | --- | --- | --- | --- |
| ab_b3_prof_int | 13048ns | -21697.9ns (-61.9%) | [-22156, -18343]ns | [11180, 13757] | YES | 0.0313 | 0.0313 | 0 |
| ab_b3_seq_int | 13022ns | -21968.5ns (-62.7%) | [-22162, -18596]ns | [10926, 13508] | YES | 0.0313 | 0.0313 | 0 |
| ab_b3_table_int | 12998ns | -21862.7ns (-62.4%) | [-22216, -18463]ns | [11060, 13584] | YES | 0.0313 | 0.0313 | 0 |
| ab_b3_tree_int | 12709ns | -22165.4ns (-63.3%) | [-22588, -17970]ns | [11553, 13198] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b3_pred_int | ab_b3_prof_int | ab_b3_seq_int | ab_b3_table_int | ab_b3_tree_int |
|---|---|---|---|---|---|
| 1 | 35912ns | -59.8% | -61.1% | -60.9% | -62.9% |
| 2 | 29357ns | -62.4% | -62.8% | -62.1% | -61.5% |
| 3 | 29689ns | -61.8% | -63.2% | -62.9% | -60.2% |
| 4 | 35322ns | -63.0% | -63.0% | -63.2% | -63.0% |
| 5 | 34980ns | -62.7% | -62.8% | -62.5% | -64.6% |
| 6 | 35105ns | -62.8% | -62.9% | -62.9% | -62.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b3_pred_int | 0.075 | ok |
| ab_b3_prof_int | -0.121 | ok |
| ab_b3_seq_int | -0.021 | ok |
| ab_b3_table_int | -0.011 | ok |
| ab_b3_tree_int | -0.217 | moderate- |

**Consistency summary:**

- **ab_b3_prof_int**: won 6/6, lost 0/6
- **ab_b3_seq_int**: won 6/6, lost 0/6
- **ab_b3_table_int**: won 6/6, lost 0/6
- **ab_b3_tree_int**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b3_pred_int | 3.8ns | 33394.4ns | 0.0% |  |
| ab_b3_prof_int | 4.2ns | 12662.0ns | 0.0% |  |
| ab_b3_seq_int | 3.5ns | 12485.3ns | 0.0% |  |
| ab_b3_table_int | 3.9ns | 12547.1ns | 0.0% |  |
| ab_b3_tree_int | 3.8ns | 12486.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b3_pred_int (n=6, range 29356.7-35617.5 ns)
  29356.7 |########################################
  29669.7 |########################################
  29982.8 |
  30295.8 |
  30608.9 |
  30921.9 |
  31234.9 |
  31548.0 |
  31861.0 |
  32174.1 |
  32487.1 |
  32800.1 |
  33113.2 |
  33426.2 |
  33739.3 |
  34052.3 |
  34365.3 |
  34678.4 |########################################
  34991.4 |########################################
  35304.5 |########################################
  (0 below, 1 above range)

ab_b3_prof_int (n=6, range 11024.6-13757.5 ns)
  11024.6 |#############
  11161.2 |
  11297.9 |#############
  11434.5 |
  11571.2 |
  11707.8 |
  11844.5 |
  11981.1 |
  12117.7 |
  12254.4 |
  12391.0 |
  12527.7 |
  12664.3 |
  12801.0 |
  12937.6 |########################################
  13074.2 |
  13210.9 |
  13347.5 |
  13484.2 |
  13620.8 |
  (0 below, 1 above range)

ab_b3_seq_int (n=6, range 10923.3-13507.7 ns)
  10923.3 |##########################
  11052.5 |
  11181.7 |
  11311.0 |
  11440.2 |
  11569.4 |
  11698.6 |
  11827.8 |
  11957.1 |
  12086.3 |
  12215.5 |
  12344.7 |
  12473.9 |
  12603.2 |
  12732.4 |
  12861.6 |
  12990.8 |########################################
  13120.0 |
  13249.3 |
  13378.5 |
  (0 below, 1 above range)

ab_b3_table_int (n=6, range 11007.1-13583.5 ns)
  11007.1 |########################################
  11135.9 |
  11264.7 |
  11393.6 |
  11522.4 |
  11651.2 |
  11780.0 |
  11908.9 |
  12037.7 |
  12166.5 |
  12295.3 |
  12424.1 |
  12553.0 |
  12681.8 |
  12810.6 |
  12939.4 |########################################
  13068.3 |####################
  13197.1 |
  13325.9 |
  13454.7 |
  (0 below, 1 above range)

ab_b3_tree_int (n=6, range 11295.8-13197.7 ns)
  11295.8 |####################
  11390.9 |
  11486.0 |
  11581.1 |
  11676.2 |
  11771.3 |####################
  11866.4 |
  11961.5 |
  12056.6 |
  12151.7 |
  12246.8 |
  12341.8 |####################
  12436.9 |
  12532.0 |
  12627.1 |
  12722.2 |
  12817.3 |
  12912.4 |
  13007.5 |########################################
  13102.6 |
  (0 below, 1 above range)

```

# Whole-program single strategy (all branches), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_whole_table**

## Highlights

Baseline for all deltas below: **ab_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_whole_pred is an outlier: 4.1x slower than the field

ab_whole_pred (89.58 us) is 4.1x the fastest (21.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (ab_whole_table)

The baseline ab_whole_table is the fastest (21.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_whole_table, ab_whole_tree, ab_whole_prof, ab_whole_seq} vs {ab_whole_pred} (263% apart)

The field splits into a fast tier {ab_whole_table, ab_whole_tree, ab_whole_prof, ab_whole_seq} and a slow tier {ab_whole_pred} with a 263% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest ab_whole_table (21.75 us) to slowest ab_whole_pred (89.58 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ab_whole_table) is the fastest** at 21754.8 ns median
- 3 variants significantly slower than baseline
- Spread: 4.12x (fastest 21754.8 ns, slowest 89578.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_whole_pred | 91060ns | 92116ns | 84060ns | 90075ns | 96038ns | +279.06% |
| ab_whole_prof | 26207ns | 27161ns | 22811ns | 26098ns | 28069ns | +9.10% |
| ab_whole_seq | 25859ns | 27159ns | 22690ns | 25782ns | 27560ns | +7.65% |
| ab_whole_table | 24022ns | 24068ns | 21963ns | 23537ns | 25780ns | base |
| ab_whole_tree | 25405ns | 26079ns | 22666ns | 25153ns | 27151ns | +5.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_whole_pred | 88629ns | 81901ns | 93479ns | +308.24% | 0.001 |
| ab_whole_prof | 23712ns | 20652ns | 25352ns | +9.22% | 0.003 |
| ab_whole_seq | 23399ns | 20564ns | 24831ns | +7.78% | 0.003 |
| ab_whole_table | 21710ns | 19852ns | 23292ns | base | 0.003 |
| ab_whole_tree | 22955ns | 20475ns | 24516ns | +5.73% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_whole_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_whole_pred | 0.001 | 22.2% |
| ab_whole_prof | 0.003 | 80.7% |
| ab_whole_seq | 0.003 | 80.5% |
| ab_whole_table | 0.003 | 91.3% |
| ab_whole_tree | 0.003 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_whole_pred | 91060ns | 91060ns | +279.06% |
| ab_whole_prof | 26207ns | 26207ns | +9.10% |
| ab_whole_seq | 25859ns | 25859ns | +7.65% |
| ab_whole_table | 24022ns | 24022ns | base |
| ab_whole_tree | 25405ns | 25405ns | +5.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_whole_table | 21755ns | base | --- | [20082, 23292] | --- | --- | --- | --- |
| ab_whole_pred | 89579ns | +67092.1ns (+308.4%) | [+62746, +70918]ns | [82828, 93479] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_prof | 24591ns | no significant difference | [-385, +4224]ns | [21194, 25352] | no | 0.2188 | 0.2188 | 0 |
| ab_whole_seq | 24653ns | +1524.5ns (+7.0%) | [+632, +2912]ns | [20715, 24831] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_tree | 23581ns | +1311.2ns (+6.0%) | [+405, +2019]ns | [20768, 24516] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_whole_table | ab_whole_pred | ab_whole_prof | ab_whole_seq | ab_whole_tree |
|---|---|---|---|---|---|
| 1 | 22490ns | +314.8% | -8.2% | +10.3% | +9.0% |
| 2 | 22504ns | +315.7% | +13.0% | +10.1% | +8.9% |
| 3 | 21020ns | +308.5% | +14.3% | +16.7% | +8.9% |
| 4 | 20313ns | +312.3% | +7.0% | +2.7% | +3.7% |
| 5 | 19852ns | +312.6% | +27.4% | +3.6% | +3.1% |
| 6 | 24080ns | +287.9% | +4.4% | +3.2% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_whole_pred | 0.161 | ok |
| ab_whole_prof | -0.296 | moderate- |
| ab_whole_seq | 0.169 | ok |
| ab_whole_table | -0.061 | ok |
| ab_whole_tree | 0.239 | moderate+ |

**Consistency summary:**

- **ab_whole_pred**: won 0/6, lost 6/6
- **ab_whole_prof**: won 1/6, lost 5/6
- **ab_whole_seq**: won 0/6, lost 6/6
- **ab_whole_tree**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_whole_pred | 4.6ns | 88628.5ns | 0.0% |  |
| ab_whole_prof | 3.7ns | 23712.4ns | 0.0% |  |
| ab_whole_seq | 3.5ns | 23399.4ns | 0.0% |  |
| ab_whole_table | 3.5ns | 21709.8ns | 0.0% |  |
| ab_whole_tree | 3.2ns | 22954.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_whole_pred (n=6, range 81901.2-93478.8 ns)
  81901.2 |####################
  82480.1 |
  83059.0 |
  83637.8 |####################
  84216.7 |
  84795.6 |
  85374.5 |####################
  85953.3 |
  86532.2 |
  87111.1 |
  87690.0 |
  88268.9 |
  88847.7 |
  89426.6 |
  90005.5 |
  90584.4 |
  91163.2 |
  91742.1 |
  92321.0 |
  92899.9 |########################################
  (0 below, 1 above range)

ab_whole_prof (n=6, range 20652.5-25352.1 ns)
  20652.5 |####################
  20887.5 |
  21122.5 |
  21357.4 |
  21592.4 |####################
  21827.4 |
  22062.4 |
  22297.4 |
  22532.3 |
  22767.3 |
  23002.3 |
  23237.3 |
  23472.3 |
  23707.2 |
  23942.2 |####################
  24177.2 |
  24412.2 |
  24647.2 |
  24882.1 |
  25117.1 |########################################
  (0 below, 1 above range)

ab_whole_seq (n=6, range 20564.2-24830.6 ns)
  20564.2 |####################
  20777.5 |####################
  20990.8 |
  21204.2 |
  21417.5 |
  21630.8 |
  21844.1 |
  22057.4 |
  22270.8 |
  22484.1 |
  22697.4 |
  22910.7 |
  23124.0 |
  23337.4 |
  23550.7 |
  23764.0 |
  23977.3 |
  24190.6 |
  24404.0 |####################
  24617.3 |########################################
  (0 below, 1 above range)

ab_whole_table (n=6, range 19851.7-23292.1 ns)
  19851.7 |####################
  20023.7 |
  20195.7 |####################
  20367.8 |
  20539.8 |
  20711.8 |
  20883.8 |####################
  21055.8 |
  21227.9 |
  21399.9 |
  21571.9 |
  21743.9 |
  21915.9 |
  22088.0 |
  22260.0 |
  22432.0 |########################################
  22604.0 |
  22776.0 |
  22948.1 |
  23120.1 |
  (0 below, 1 above range)

ab_whole_tree (n=6, range 20475.4-24515.8 ns)
  20475.4 |########################################
  20677.4 |
  20879.4 |########################################
  21081.5 |
  21283.5 |
  21485.5 |
  21687.5 |
  21889.5 |
  22091.6 |
  22293.6 |
  22495.6 |
  22697.6 |########################################
  22899.6 |
  23101.7 |
  23303.7 |
  23505.7 |
  23707.7 |
  23909.7 |
  24111.8 |########################################
  24313.8 |########################################
  (0 below, 1 above range)

```

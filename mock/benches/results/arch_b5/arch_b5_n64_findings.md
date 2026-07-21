# Per-branch strategy: archetype 5 (ifchain4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b5_table**

## Highlights

Baseline for all deltas below: **ab_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_b5_table dominates: 11% faster than the next best (ab_b5_prof)

ab_b5_table (20.87 us) leads ab_b5_prof (23.17 us) by 11%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (ab_b5_table)

The baseline ab_b5_table is the fastest (20.87 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b5_table) is the fastest** at 20865.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.33x (fastest 20865.2 ns, slowest 27646.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b5_pred | 29254ns | 30142ns | 25527ns | 28809ns | 31785ns | +21.90% |
| ab_b5_prof | 25332ns | 25592ns | 22692ns | 24769ns | 27496ns | +5.56% |
| ab_b5_seq | 25074ns | 25693ns | 22897ns | 24801ns | 26571ns | +4.48% |
| ab_b5_table | 23998ns | 23093ns | 22408ns | 22891ns | 26455ns | base |
| ab_b5_tree | 24987ns | 25888ns | 22328ns | 24749ns | 26674ns | +4.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b5_pred | 26823ns | 23379ns | 29187ns | +23.72% | 0.002 |
| ab_b5_prof | 22950ns | 20562ns | 24912ns | +5.86% | 0.003 |
| ab_b5_seq | 22705ns | 20737ns | 24080ns | +4.73% | 0.003 |
| ab_b5_table | 21680ns | 20251ns | 23900ns | base | 0.003 |
| ab_b5_tree | 22574ns | 20180ns | 24090ns | +4.12% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b5_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b5_pred | 0.002 | 73.0% |
| ab_b5_prof | 0.003 | 87.1% |
| ab_b5_seq | 0.003 | 86.8% |
| ab_b5_table | 0.003 | 96.7% |
| ab_b5_tree | 0.003 | 86.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b5_pred | 29254ns | 29254ns | +21.90% |
| ab_b5_prof | 25332ns | 25332ns | +5.56% |
| ab_b5_seq | 25074ns | 25074ns | +4.48% |
| ab_b5_table | 23998ns | 23998ns | base |
| ab_b5_tree | 24987ns | 24987ns | +4.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b5_table | 20865ns | base | --- | [20275, 23900] | --- | --- | --- | --- |
| ab_b5_pred | 27647ns | +4654.6ns (+22.3%) | [+3361, +7415]ns | [23636, 29187] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b5_prof | 23175ns | +1012.1ns (+4.9%) | [+203, +2596]ns | [20764, 24912] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_b5_seq | 23249ns | no significant difference | [-88, +2384]ns | [20787, 24080] | no | 0.2917 | 0.2188 | 0 |
| ab_b5_tree | 23391ns | no significant difference | [-69, +2526]ns | [20240, 24090] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b5_table | ab_b5_pred | ab_b5_prof | ab_b5_seq | ab_b5_tree |
|---|---|---|---|---|---|
| 1 | 20872ns | +41.7% | -1.5% | +10.7% | +15.2% |
| 2 | 20858ns | +29.4% | +11.0% | +12.2% | +9.0% |
| 3 | 20251ns | +18.0% | +3.5% | +2.4% | +0.2% |
| 4 | 20299ns | +15.2% | +14.3% | +2.6% | -0.6% |
| 5 | 23732ns | +19.3% | +5.0% | +4.3% | +1.7% |
| 6 | 24067ns | +19.6% | +3.5% | -2.8% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b5_pred | 0.230 | moderate+ |
| ab_b5_prof | 0.172 | ok |
| ab_b5_seq | 0.015 | ok |
| ab_b5_table | 0.387 | moderate+ |
| ab_b5_tree | 0.219 | moderate+ |

**Consistency summary:**

- **ab_b5_pred**: won 0/6, lost 6/6
- **ab_b5_prof**: won 1/6, lost 5/6
- **ab_b5_seq**: won 1/6, lost 5/6
- **ab_b5_tree**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b5_pred | 2.9ns | 26823.4ns | 0.0% |  |
| ab_b5_prof | 3.1ns | 22950.3ns | 0.0% |  |
| ab_b5_seq | 3.6ns | 22705.3ns | 0.0% |  |
| ab_b5_table | 3.5ns | 21679.9ns | 0.0% |  |
| ab_b5_tree | 3.8ns | 22573.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b5_pred (n=6, range 23378.8-29187.2 ns)
  23378.8 |########################################
  23669.2 |########################################
  23959.6 |
  24250.1 |
  24540.5 |
  24830.9 |
  25121.3 |
  25411.8 |
  25702.2 |
  25992.6 |
  26283.0 |
  26573.4 |
  26863.9 |########################################
  27154.3 |
  27444.7 |
  27735.1 |
  28025.6 |########################################
  28316.0 |
  28606.4 |########################################
  28896.8 |
  (0 below, 1 above range)

ab_b5_prof (n=6, range 20562.1-24911.7 ns)
  20562.1 |########################################
  20779.6 |########################################
  20997.1 |
  21214.5 |
  21432.0 |
  21649.5 |
  21867.0 |
  22084.5 |
  22301.9 |
  22519.4 |
  22736.9 |
  22954.4 |########################################
  23171.9 |########################################
  23389.3 |
  23606.8 |
  23824.3 |
  24041.8 |
  24259.3 |
  24476.7 |
  24694.2 |########################################
  (0 below, 1 above range)

ab_b5_seq (n=6, range 20737.1-24080.2 ns)
  20737.1 |########################################
  20904.3 |
  21071.4 |
  21238.6 |
  21405.7 |
  21572.9 |
  21740.0 |
  21907.2 |
  22074.4 |
  22241.5 |
  22408.7 |
  22575.8 |
  22743.0 |
  22910.1 |
  23077.3 |####################
  23244.5 |########################################
  23411.6 |
  23578.8 |
  23745.9 |
  23913.1 |
  (0 below, 1 above range)

ab_b5_table (n=6, range 20250.8-23899.6 ns)
  20250.8 |########################################
  20433.2 |
  20615.7 |
  20798.1 |########################################
  20980.6 |
  21163.0 |
  21345.4 |
  21527.9 |
  21710.3 |
  21892.8 |
  22075.2 |
  22257.6 |
  22440.1 |
  22622.5 |
  22805.0 |
  22987.4 |
  23169.8 |
  23352.3 |
  23534.7 |
  23717.2 |####################
  (0 below, 1 above range)

ab_b5_tree (n=6, range 20179.6-24090.5 ns)
  20179.6 |########################################
  20375.1 |
  20570.7 |
  20766.2 |
  20961.8 |
  21157.3 |
  21352.9 |
  21548.4 |
  21743.9 |
  21939.5 |
  22135.0 |
  22330.6 |
  22526.1 |
  22721.7 |####################
  22917.2 |
  23112.7 |
  23308.3 |
  23503.8 |
  23699.4 |
  23894.9 |########################################
  (0 below, 1 above range)

```

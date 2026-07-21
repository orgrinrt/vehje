# Per-branch strategy (NATIVE tier): archetype 6

5 variants, 6 samples per variant.
Baseline: **an_b6_table**

## Highlights

Baseline for all deltas below: **an_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b6_table dominates: 11% faster than the next best (an_b6_seq)

an_b6_table (7.70 us) leads an_b6_seq (8.56 us) by 11%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (an_b6_table)

The baseline an_b6_table is the fastest (7.70 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b6_table, an_b6_seq, an_b6_tree, an_b6_prof} vs {an_b6_pred} (71% apart)

The field splits into a fast tier {an_b6_table, an_b6_seq, an_b6_tree, an_b6_prof} and a slow tier {an_b6_pred} with a 71% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (an_b6_table) is the fastest** at 7699.4 ns median
- 3 variants significantly slower than baseline
- Spread: 1.95x (fastest 7699.4 ns, slowest 15009.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b6_pred | 17160ns | 17407ns | 15542ns | 16928ns | 18318ns | +67.95% |
| an_b6_prof | 10935ns | 11268ns | 9628ns | 10803ns | 11785ns | +7.02% |
| an_b6_seq | 10899ns | 10979ns | 9840ns | 10659ns | 11789ns | +6.68% |
| an_b6_table | 10217ns | 9976ns | 9405ns | 9911ns | 11082ns | base |
| an_b6_tree | 10797ns | 11189ns | 9425ns | 10625ns | 11743ns | +5.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b6_pred | 14759ns | 13340ns | 15741ns | +87.39% | 0.069 |
| an_b6_prof | 8528ns | 7502ns | 9192ns | +8.28% | 0.120 |
| an_b6_seq | 8517ns | 7678ns | 9218ns | +8.13% | 0.120 |
| an_b6_table | 7876ns | 7254ns | 8527ns | base | 0.130 |
| an_b6_tree | 8354ns | 7270ns | 9171ns | +6.07% | 0.123 |

## Performance model

- Peak throughput: **0.141 Gops/s** (an_b6_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b6_pred | 0.068 | 48.3% |
| an_b6_prof | 0.117 | 82.5% |
| an_b6_seq | 0.120 | 84.7% |
| an_b6_table | 0.133 | 94.2% |
| an_b6_tree | 0.119 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b6_pred | 17160ns | 17160ns | +67.95% |
| an_b6_prof | 10935ns | 10935ns | +7.02% |
| an_b6_seq | 10899ns | 10899ns | +6.68% |
| an_b6_table | 10217ns | 10217ns | base |
| an_b6_tree | 10797ns | 10797ns | +5.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b6_table | 7699ns | base | --- | [7403, 8527] | --- | --- | --- | --- |
| an_b6_pred | 15010ns | +6833.1ns (+88.7%) | [+6124, +7692]ns | [13527, 15741] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b6_prof | 8789ns | +665.9ns (+8.6%) | [+22, +1268]ns | [7604, 9192] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| an_b6_seq | 8562ns | +662.5ns (+8.6%) | [+236, +1023]ns | [7771, 9218] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b6_tree | 8615ns | no significant difference | [-274, +1472]ns | [7278, 9171] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b6_table | an_b6_pred | an_b6_prof | an_b6_seq | an_b6_tree |
|---|---|---|---|---|---|
| 1 | 7611ns | +89.1% | -1.4% | +13.3% | +25.6% |
| 2 | 8903ns | +77.4% | +4.5% | +3.9% | -3.2% |
| 3 | 7254ns | +89.0% | +17.3% | +8.4% | +0.2% |
| 4 | 7552ns | +76.6% | +2.0% | +1.7% | -3.5% |
| 5 | 7788ns | +100.7% | +16.5% | +9.2% | +12.7% |
| 6 | 8150ns | +92.5% | +11.5% | +12.7% | +5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b6_pred | -0.067 | ok |
| an_b6_prof | -0.319 | moderate- |
| an_b6_seq | 0.070 | ok |
| an_b6_table | -0.415 | moderate- |
| an_b6_tree | 0.206 | moderate+ |

**Consistency summary:**

- **an_b6_pred**: won 0/6, lost 6/6
- **an_b6_prof**: won 1/6, lost 5/6
- **an_b6_seq**: won 0/6, lost 6/6
- **an_b6_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b6_pred | 4.4ns | 14759.3ns | 0.0% |  |
| an_b6_prof | 6.2ns | 8528.4ns | 0.1% |  |
| an_b6_seq | 5.9ns | 8517.0ns | 0.1% |  |
| an_b6_table | 5.8ns | 7876.4ns | 0.1% |  |
| an_b6_tree | 6.6ns | 8354.5ns | 0.1% |  |

## Distribution (algo ns)

```
an_b6_pred (n=6, range 13340.0-15741.5 ns)
  13340.0 |####################
  13460.1 |
  13580.1 |
  13700.2 |####################
  13820.3 |
  13940.4 |
  14060.4 |
  14180.5 |
  14300.6 |####################
  14420.7 |
  14540.7 |
  14660.8 |
  14780.9 |
  14900.9 |
  15021.0 |
  15141.1 |
  15261.2 |
  15381.2 |
  15501.3 |
  15621.4 |########################################
  (0 below, 1 above range)

an_b6_prof (n=6, range 7502.5-9192.5 ns)
   7502.5 |####################
   7587.0 |
   7671.5 |####################
   7756.0 |
   7840.5 |
   7925.0 |
   8009.5 |
   8094.0 |
   8178.5 |
   8263.0 |
   8347.5 |
   8432.0 |####################
   8516.5 |
   8601.0 |
   8685.5 |
   8770.0 |
   8854.5 |
   8939.0 |
   9023.5 |########################################
   9108.0 |
  (0 below, 1 above range)

an_b6_seq (n=6, range 7677.5-9217.9 ns)
   7677.5 |########################################
   7754.5 |
   7831.5 |########################################
   7908.6 |
   7985.6 |
   8062.6 |
   8139.6 |
   8216.6 |
   8293.7 |
   8370.7 |
   8447.7 |########################################
   8524.7 |
   8601.7 |########################################
   8678.8 |
   8755.8 |
   8832.8 |
   8909.8 |
   8986.8 |
   9063.9 |
   9140.9 |########################################
  (0 below, 1 above range)

an_b6_table (n=6, range 7254.2-8526.6 ns)
   7254.2 |########################################
   7317.8 |
   7381.4 |
   7445.1 |
   7508.7 |########################################
   7572.3 |########################################
   7635.9 |
   7699.6 |
   7763.2 |########################################
   7826.8 |
   7890.4 |
   7954.0 |
   8017.7 |
   8081.3 |
   8144.9 |########################################
   8208.5 |
   8272.2 |
   8335.8 |
   8399.4 |
   8463.0 |
  (0 below, 1 above range)

an_b6_tree (n=6, range 7270.4-9170.9 ns)
   7270.4 |########################################
   7365.4 |
   7460.4 |
   7555.5 |
   7650.5 |
   7745.5 |
   7840.5 |
   7935.6 |
   8030.6 |
   8125.6 |
   8220.6 |
   8315.6 |
   8410.7 |
   8505.7 |
   8600.7 |########################################
   8695.7 |####################
   8790.8 |
   8885.8 |
   8980.8 |
   9075.8 |
  (0 below, 1 above range)

```

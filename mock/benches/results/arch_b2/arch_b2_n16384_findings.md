# Per-branch strategy: archetype 2 (match8_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b2_table**

## Highlights

Baseline for all deltas below: **ab_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b2_table, ab_b2_prof) are a dead heat (<1%)

ab_b2_table (5.20 ms) and ab_b2_prof (5.21 ms) differ by 0.11%, inside the noise, even though the wider field spreads 40.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ab_b2_table)

The baseline ab_b2_table is the fastest (5.20 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_b2_table, ab_b2_prof, ab_b2_tree, ab_b2_seq} vs {ab_b2_pred} (40% apart)

The field splits into a fast tier {ab_b2_table, ab_b2_prof, ab_b2_tree, ab_b2_seq} and a slow tier {ab_b2_pred} with a 40% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_b2_table) is the fastest** at 5200657.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 5200657.9 ns, slowest 7310530.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b2_pred | 7314654ns | 7314556ns | 7287494ns | 7311441ns | 7333055ns | +40.43% |
| ab_b2_prof | 5214769ns | 5210441ns | 5178799ns | 5202176ns | 5251643ns | +0.12% |
| ab_b2_seq | 5239600ns | 5235768ns | 5178519ns | 5230368ns | 5283989ns | +0.59% |
| ab_b2_table | 5208714ns | 5204107ns | 5136895ns | 5202781ns | 5253523ns | base |
| ab_b2_tree | 5219130ns | 5220561ns | 5186110ns | 5212612ns | 5245416ns | +0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b2_pred | 7310637ns | 7283331ns | 7329045ns | +40.45% | 0.002 |
| ab_b2_prof | 5210994ns | 5175011ns | 5247986ns | +0.12% | 0.003 |
| ab_b2_seq | 5235745ns | 5174493ns | 5280205ns | +0.59% | 0.003 |
| ab_b2_table | 5204990ns | 5133033ns | 5249834ns | base | 0.003 |
| ab_b2_tree | 5215451ns | 5182071ns | 5242002ns | +0.20% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b2_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b2_pred | 0.002 | 70.2% |
| ab_b2_prof | 0.003 | 98.6% |
| ab_b2_seq | 0.003 | 98.1% |
| ab_b2_table | 0.003 | 98.7% |
| ab_b2_tree | 0.003 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b2_pred | 7314654ns | 7314654ns | +40.43% |
| ab_b2_prof | 5214769ns | 5214769ns | +0.12% |
| ab_b2_seq | 5239600ns | 5239600ns | +0.59% |
| ab_b2_table | 5208714ns | 5208714ns | base |
| ab_b2_tree | 5219130ns | 5219130ns | +0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b2_table | 5200658ns | base | --- | [5164479, 5249834] | --- | --- | --- | --- |
| ab_b2_pred | 7310531ns | +2095866.5ns (+40.3%) | [+2059728, +2161345]ns | [7292335, 7329045] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b2_prof | 5206545ns | no significant difference | [-52508, +73067]ns | [5178452, 5247986] | no | 1.0000 | 1.0000 | 0 |
| ab_b2_seq | 5231854ns | no significant difference | [-31451, +114908]ns | [5195176, 5280205] | no | 1.0000 | 0.6875 | 0 |
| ab_b2_tree | 5216830ns | no significant difference | [-30572, +63401]ns | [5187519, 5242002] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b2_table | ab_b2_pred | ab_b2_prof | ab_b2_seq | ab_b2_tree |
|---|---|---|---|---|---|
| 1 | 5202368ns | +40.7% | +0.4% | -0.5% | -0.2% |
| 2 | 5275500ns | +38.5% | -1.6% | -0.7% | -0.9% |
| 3 | 5195925ns | +40.2% | -0.4% | +0.8% | -0.3% |
| 4 | 5198948ns | +40.4% | -0.3% | +0.3% | +0.1% |
| 5 | 5224169ns | +40.0% | +0.2% | +0.0% | +0.2% |
| 6 | 5133033ns | +42.9% | +2.5% | +3.6% | +2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b2_pred | 0.215 | moderate+ |
| ab_b2_prof | 0.360 | moderate+ |
| ab_b2_seq | -0.091 | ok |
| ab_b2_table | -0.213 | moderate- |
| ab_b2_tree | 0.024 | ok |

**Consistency summary:**

- **ab_b2_pred**: won 0/6, lost 6/6
- **ab_b2_prof**: won 3/6, lost 3/6
- **ab_b2_seq**: won 2/6, lost 3/6
- **ab_b2_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b2_pred | 86.5ns | 7310637.1ns | 0.0% |  |
| ab_b2_prof | 55.3ns | 5210994.5ns | 0.0% |  |
| ab_b2_seq | 73.0ns | 5235745.1ns | 0.0% |  |
| ab_b2_table | 67.4ns | 5204990.5ns | 0.0% |  |
| ab_b2_tree | 68.4ns | 5215450.5ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b2_pred (n=6, range 7283330.8-7329045.2 ns)
  7283330.8 |########################################
  7285616.5 |
  7287902.2 |
  7290188.0 |
  7292473.7 |
  7294759.4 |
  7297045.1 |
  7299330.8 |########################################
  7301616.6 |
  7303902.3 |
  7306188.0 |########################################
  7308473.7 |
  7310759.4 |
  7313045.2 |########################################
  7315330.9 |
  7317616.6 |
  7319902.3 |########################################
  7322188.0 |
  7324473.8 |
  7326759.5 |
  (0 below, 1 above range)

ab_b2_prof (n=6, range 5175010.8-5247985.7 ns)
  5175010.8 |########################################
  5178659.5 |########################################
  5182308.3 |
  5185957.0 |
  5189605.8 |########################################
  5193254.5 |
  5196903.3 |
  5200552.0 |
  5204200.7 |
  5207849.5 |
  5211498.2 |
  5215147.0 |
  5218795.7 |########################################
  5222444.5 |
  5226093.2 |
  5229741.9 |
  5233390.7 |########################################
  5237039.4 |
  5240688.2 |
  5244336.9 |
  (0 below, 1 above range)

ab_b2_seq (n=6, range 5174493.3-5280205.5 ns)
  5174493.3 |####################
  5179778.9 |
  5185064.5 |
  5190350.1 |
  5195635.7 |
  5200921.3 |
  5206206.9 |
  5211492.6 |####################
  5216778.2 |
  5222063.8 |####################
  5227349.4 |
  5232635.0 |
  5237920.6 |########################################
  5243206.2 |
  5248491.8 |
  5253777.4 |
  5259063.0 |
  5264348.6 |
  5269634.2 |
  5274919.8 |
  (0 below, 1 above range)

ab_b2_table (n=6, range 5133032.9-5249834.4 ns)
  5133032.9 |####################
  5138873.0 |
  5144713.1 |
  5150553.1 |
  5156393.2 |
  5162233.3 |
  5168073.4 |
  5173913.4 |
  5179753.5 |
  5185593.6 |
  5191433.7 |####################
  5197273.7 |########################################
  5203113.8 |
  5208953.9 |
  5214794.0 |
  5220634.0 |####################
  5226474.1 |
  5232314.2 |
  5238154.2 |
  5243994.3 |
  (0 below, 1 above range)

ab_b2_tree (n=6, range 5182070.8-5242002.3 ns)
  5182070.8 |########################################
  5185067.4 |
  5188064.0 |
  5191060.5 |########################################
  5194057.1 |
  5197053.7 |
  5200050.2 |
  5203046.8 |########################################
  5206043.4 |
  5209040.0 |
  5212036.6 |
  5215033.1 |
  5218029.7 |
  5221026.3 |
  5224022.9 |
  5227019.4 |########################################
  5230016.0 |
  5233012.6 |########################################
  5236009.2 |
  5239005.7 |
  (0 below, 1 above range)

```

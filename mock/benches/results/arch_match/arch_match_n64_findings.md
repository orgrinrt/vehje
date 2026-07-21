# Per-type strategy: all match branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_match_table**

## Highlights

Baseline for all deltas below: **ab_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_match_pred is an outlier: 2.7x slower than the field

ab_match_pred (60.10 us) is 2.7x the fastest (22.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_match_table, ab_match_seq) are a dead heat (<1%)

ab_match_table (22.13 us) and ab_match_seq (22.24 us) differ by 0.51%, inside the noise, even though the wider field spreads 171.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ab_match_table)

The baseline ab_match_table is the fastest (22.13 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_match_table, ab_match_seq, ab_match_tree, ab_match_prof} vs {ab_match_pred} (151% apart)

The field splits into a fast tier {ab_match_table, ab_match_seq, ab_match_tree, ab_match_prof} and a slow tier {ab_match_pred} with a 151% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_match_table) is the fastest** at 22129.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.72x (fastest 22129.5 ns, slowest 60100.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_match_pred | 62990ns | 62445ns | 58340ns | 61115ns | 68129ns | +159.17% |
| ab_match_prof | 25352ns | 26459ns | 22159ns | 25233ns | 27126ns | +4.31% |
| ab_match_seq | 24756ns | 24568ns | 22470ns | 24133ns | 26832ns | +1.86% |
| ab_match_table | 24304ns | 24419ns | 21938ns | 24087ns | 25814ns | base |
| ab_match_tree | 25509ns | 26039ns | 23092ns | 25098ns | 27333ns | +4.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_match_pred | 60614ns | 56130ns | 65593ns | +175.72% | 0.001 |
| ab_match_prof | 22910ns | 20048ns | 24544ns | +4.22% | 0.003 |
| ab_match_seq | 22408ns | 20319ns | 24304ns | +1.93% | 0.003 |
| ab_match_table | 21984ns | 19829ns | 23308ns | base | 0.003 |
| ab_match_tree | 23092ns | 20883ns | 24769ns | +5.04% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_match_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_match_pred | 0.001 | 33.0% |
| ab_match_prof | 0.003 | 82.8% |
| ab_match_seq | 0.003 | 89.2% |
| ab_match_table | 0.003 | 89.6% |
| ab_match_tree | 0.003 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_match_pred | 62990ns | 62990ns | +159.17% |
| ab_match_prof | 25352ns | 25352ns | +4.31% |
| ab_match_seq | 24756ns | 24756ns | +1.86% |
| ab_match_table | 24304ns | 24304ns | base |
| ab_match_tree | 25509ns | 25509ns | +4.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_match_table | 22130ns | base | --- | [20513, 23308] | --- | --- | --- | --- |
| ab_match_pred | 60101ns | +37596.6ns (+169.9%) | [+35352, +42942]ns | [56147, 65593] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_match_prof | 23938ns | no significant difference | [-1230, +3325]ns | [20249, 24544] | no | 0.6875 | 0.6875 | 0 |
| ab_match_seq | 22242ns | no significant difference | [-895, +1848]ns | [20677, 24304] | no | 0.6875 | 0.6875 | 0 |
| ab_match_tree | 23562ns | no significant difference | [-1190, +4256]ns | [20945, 24769] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_match_table | ab_match_pred | ab_match_prof | ab_match_seq | ab_match_tree |
|---|---|---|---|---|---|
| 1 | 21197ns | +209.1% | -5.4% | +8.7% | +15.1% |
| 2 | 22498ns | +168.5% | +8.1% | +8.2% | +1.1% |
| 3 | 22510ns | +165.6% | +4.6% | -6.6% | -7.2% |
| 4 | 21761ns | +157.9% | -6.0% | -1.4% | -3.5% |
| 5 | 19829ns | +183.2% | +24.3% | +2.5% | +26.8% |
| 6 | 24107ns | +172.4% | +1.4% | +0.6% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_match_pred | 0.003 | ok |
| ab_match_prof | -0.288 | moderate- |
| ab_match_seq | -0.134 | ok |
| ab_match_table | -0.419 | moderate- |
| ab_match_tree | 0.193 | ok |

**Consistency summary:**

- **ab_match_pred**: won 0/6, lost 6/6
- **ab_match_prof**: won 2/6, lost 4/6
- **ab_match_seq**: won 2/6, lost 4/6
- **ab_match_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_match_pred | 3.9ns | 60613.6ns | 0.0% |  |
| ab_match_prof | 3.5ns | 22910.2ns | 0.0% |  |
| ab_match_seq | 3.9ns | 22407.8ns | 0.0% |  |
| ab_match_table | 3.2ns | 21983.6ns | 0.0% |  |
| ab_match_tree | 4.0ns | 23091.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_match_pred (n=6, range 56130.0-65593.3 ns)
  56130.0 |########################################
  56603.2 |
  57076.3 |
  57549.5 |
  58022.7 |
  58495.8 |
  58969.0 |
  59442.2 |####################
  59915.3 |
  60388.5 |####################
  60861.7 |
  61334.8 |
  61808.0 |
  62281.1 |
  62754.3 |
  63227.5 |
  63700.6 |
  64173.8 |
  64647.0 |
  65120.1 |####################
  (0 below, 1 above range)

ab_match_prof (n=6, range 20048.3-24543.8 ns)
  20048.3 |####################
  20273.1 |####################
  20497.8 |
  20722.6 |
  20947.4 |
  21172.2 |
  21396.9 |
  21621.7 |
  21846.5 |
  22071.3 |
  22296.0 |
  22520.8 |
  22745.6 |
  22970.3 |
  23195.1 |
  23419.9 |####################
  23644.7 |
  23869.4 |
  24094.2 |
  24319.0 |########################################
  (0 below, 1 above range)

ab_match_seq (n=6, range 20318.7-24304.3 ns)
  20318.7 |########################################
  20518.0 |
  20717.3 |
  20916.5 |########################################
  21115.8 |
  21315.1 |########################################
  21514.4 |
  21713.7 |
  21913.0 |
  22112.2 |
  22311.5 |
  22510.8 |
  22710.1 |
  22909.4 |########################################
  23108.7 |
  23307.9 |
  23507.2 |
  23706.5 |
  23905.8 |
  24105.1 |########################################
  (0 below, 1 above range)

ab_match_table (n=6, range 19829.2-23308.2 ns)
  19829.2 |####################
  20003.1 |
  20177.1 |
  20351.0 |
  20525.0 |
  20698.9 |
  20872.9 |
  21046.8 |####################
  21220.8 |
  21394.7 |
  21568.7 |
  21742.6 |####################
  21916.6 |
  22090.5 |
  22264.5 |
  22438.4 |########################################
  22612.4 |
  22786.3 |
  22960.3 |
  23134.2 |
  (0 below, 1 above range)

ab_match_tree (n=6, range 20882.9-24768.7 ns)
  20882.9 |########################################
  21077.2 |
  21271.5 |
  21465.8 |
  21660.1 |
  21854.4 |
  22048.6 |
  22242.9 |
  22437.2 |
  22631.5 |####################
  22825.8 |
  23020.1 |
  23214.4 |
  23408.7 |
  23603.0 |
  23797.2 |
  23991.5 |
  24185.8 |####################
  24380.1 |####################
  24574.4 |
  (0 below, 1 above range)

```

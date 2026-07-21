# Per-branch strategy: archetype 3 (match4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b3_table**

## Highlights

Baseline for all deltas below: **ab_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (ab_b3_table)

The baseline ab_b3_table is the fastest (21.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_b3_table, ab_b3_prof, ab_b3_seq, ab_b3_tree} vs {ab_b3_pred} (90% apart)

The field splits into a fast tier {ab_b3_table, ab_b3_prof, ab_b3_seq, ab_b3_tree} and a slow tier {ab_b3_pred} with a 90% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader ab_b3_table vs stability leader ab_b3_seq (+5% speed for 1.9x steadier)

ab_b3_table is fastest (21.59 us, CV 5.1%); ab_b3_seq gives up 4.8% median for 1.9x lower variance (CV 2.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (ab_b3_table) is the fastest** at 21590.5 ns median
- 4 variants significantly slower than baseline
- Spread: 2.00x (fastest 21590.5 ns, slowest 43167.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b3_pred | 44444ns | 45636ns | 41026ns | 44597ns | 45924ns | +87.67% |
| ab_b3_prof | 24265ns | 24228ns | 22108ns | 23681ns | 26220ns | +2.46% |
| ab_b3_seq | 24979ns | 25031ns | 23623ns | 25016ns | 25601ns | +5.48% |
| ab_b3_table | 23682ns | 23917ns | 21936ns | 23416ns | 24955ns | base |
| ab_b3_tree | 25238ns | 25133ns | 22521ns | 25121ns | 26774ns | +6.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b3_pred | 42027ns | 38847ns | 43360ns | +96.59% | 0.002 |
| ab_b3_prof | 21934ns | 19992ns | 23704ns | +2.60% | 0.003 |
| ab_b3_seq | 22591ns | 21382ns | 23170ns | +5.67% | 0.003 |
| ab_b3_table | 21378ns | 19826ns | 22504ns | base | 0.003 |
| ab_b3_tree | 22803ns | 20370ns | 24195ns | +6.66% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b3_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b3_pred | 0.001 | 45.9% |
| ab_b3_prof | 0.003 | 90.6% |
| ab_b3_seq | 0.003 | 87.6% |
| ab_b3_table | 0.003 | 91.8% |
| ab_b3_tree | 0.003 | 87.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b3_pred | 44444ns | 44444ns | +87.67% |
| ab_b3_prof | 24265ns | 24265ns | +2.46% |
| ab_b3_seq | 24979ns | 24979ns | +5.48% |
| ab_b3_table | 23682ns | 23682ns | base |
| ab_b3_tree | 25238ns | 25238ns | +6.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b3_table | 21590ns | base | --- | [20040, 22504] | --- | --- | --- | --- |
| ab_b3_pred | 43167ns | +20719.6ns (+96.0%) | [+18947, +22280]ns | [39554, 43360] | YES | 0.0417 | 0.0313 | 0 |
| ab_b3_prof | 21894ns | +210.8ns (+1.0%) | [+39, +1418]ns | [20204, 23704] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| ab_b3_seq | 22622ns | +997.0ns (+4.6%) | [+424, +2216]ns | [21980, 23170] | YES | 0.0417 | 0.0313 | 0 |
| ab_b3_tree | 22700ns | +982.5ns (+4.6%) | [+212, +3079]ns | [21514, 24195] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b3_table | ab_b3_pred | ab_b3_prof | ab_b3_seq | ab_b3_tree |
|---|---|---|---|---|---|
| 1 | 19826ns | +117.7% | +0.8% | +13.9% | +14.5% |
| 2 | 22222ns | +95.5% | +7.5% | +3.9% | +2.1% |
| 3 | 22350ns | +93.6% | +5.2% | +1.1% | +1.4% |
| 4 | 22658ns | +90.5% | -0.4% | +2.6% | +6.6% |
| 5 | 20959ns | +92.1% | +1.2% | +8.0% | +15.6% |
| 6 | 20254ns | +91.8% | +0.8% | +5.6% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b3_pred | 0.433 | moderate+ |
| ab_b3_prof | 0.070 | ok |
| ab_b3_seq | -0.009 | ok |
| ab_b3_table | 0.097 | ok |
| ab_b3_tree | -0.176 | ok |

**Consistency summary:**

- **ab_b3_pred**: won 0/6, lost 6/6
- **ab_b3_prof**: won 1/6, lost 5/6
- **ab_b3_seq**: won 0/6, lost 6/6
- **ab_b3_tree**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b3_pred | 3.6ns | 42027.2ns | 0.0% |  |
| ab_b3_prof | 4.2ns | 21934.1ns | 0.0% |  |
| ab_b3_seq | 3.8ns | 22590.7ns | 0.0% |  |
| ab_b3_table | 3.4ns | 21378.2ns | 0.0% |  |
| ab_b3_tree | 4.1ns | 22802.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b3_pred (n=6, range 38846.7-43360.4 ns)
  38846.7 |#############
  39072.4 |
  39298.1 |
  39523.8 |
  39749.4 |
  39975.1 |
  40200.8 |#############
  40426.5 |
  40652.2 |
  40877.9 |
  41103.6 |
  41329.3 |
  41554.9 |
  41780.6 |
  42006.3 |
  42232.0 |
  42457.7 |
  42683.4 |
  42909.1 |
  43134.8 |########################################
  (0 below, 1 above range)

ab_b3_prof (n=6, range 19992.5-23704.0 ns)
  19992.5 |########################################
  20178.1 |
  20363.7 |########################################
  20549.2 |
  20734.8 |
  20920.4 |
  21106.0 |########################################
  21291.5 |
  21477.1 |
  21662.7 |
  21848.2 |
  22033.8 |
  22219.4 |
  22405.0 |########################################
  22590.5 |
  22776.1 |
  22961.7 |
  23147.3 |
  23332.8 |########################################
  23518.4 |
  (0 below, 1 above range)

ab_b3_seq (n=6, range 21382.5-23170.4 ns)
  21382.5 |####################
  21471.9 |
  21561.3 |
  21650.7 |
  21740.1 |
  21829.5 |
  21918.9 |
  22008.3 |
  22097.7 |
  22187.1 |
  22276.5 |
  22365.8 |
  22455.2 |
  22544.6 |########################################
  22634.0 |####################
  22723.4 |
  22812.8 |
  22902.2 |
  22991.6 |
  23081.0 |####################
  (0 below, 1 above range)

ab_b3_table (n=6, range 19826.2-22504.0 ns)
  19826.2 |########################################
  19960.1 |
  20094.0 |
  20227.9 |########################################
  20361.8 |
  20495.6 |
  20629.5 |
  20763.4 |
  20897.3 |########################################
  21031.2 |
  21165.1 |
  21299.0 |
  21432.9 |
  21566.7 |
  21700.6 |
  21834.5 |
  21968.4 |
  22102.3 |########################################
  22236.2 |########################################
  22370.1 |
  (0 below, 1 above range)

ab_b3_tree (n=6, range 20369.6-24194.6 ns)
  20369.6 |####################
  20560.8 |
  20752.1 |
  20943.3 |
  21134.6 |
  21325.8 |
  21517.1 |
  21708.3 |
  21899.6 |
  22090.8 |
  22282.1 |
  22473.3 |####################
  22664.6 |########################################
  22855.8 |
  23047.1 |
  23238.3 |
  23429.6 |
  23620.8 |
  23812.1 |
  24003.3 |####################
  (0 below, 1 above range)

```

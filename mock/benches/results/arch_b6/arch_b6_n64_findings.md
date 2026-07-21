# Per-branch strategy: archetype 6 (ifchain4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b6_table**

## Highlights

Baseline for all deltas below: **ab_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (ab_b6_table)

The baseline ab_b6_table is the fastest (22.04 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_b6_table, ab_b6_tree, ab_b6_seq, ab_b6_prof} vs {ab_b6_pred} (67% apart)

The field splits into a fast tier {ab_b6_table, ab_b6_tree, ab_b6_seq, ab_b6_prof} and a slow tier {ab_b6_pred} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_b6_table) is the fastest** at 22037.7 ns median
- 2 variants significantly slower than baseline
- Spread: 1.87x (fastest 22037.7 ns, slowest 41204.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b6_pred | 43843ns | 43530ns | 39838ns | 42415ns | 47989ns | +78.29% |
| ab_b6_prof | 26542ns | 27308ns | 23843ns | 26737ns | 27598ns | +7.93% |
| ab_b6_seq | 25482ns | 26072ns | 22337ns | 25347ns | 27257ns | +3.62% |
| ab_b6_table | 24591ns | 24377ns | 21946ns | 23956ns | 26866ns | base |
| ab_b6_tree | 25099ns | 25248ns | 23298ns | 24631ns | 26703ns | +2.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b6_pred | 41475ns | 37678ns | 45391ns | +86.88% | 0.002 |
| ab_b6_prof | 24045ns | 21608ns | 25027ns | +8.35% | 0.003 |
| ab_b6_seq | 23077ns | 20217ns | 24673ns | +3.98% | 0.003 |
| ab_b6_table | 22193ns | 19826ns | 24197ns | base | 0.003 |
| ab_b6_tree | 22648ns | 21062ns | 24129ns | +2.05% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b6_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b6_pred | 0.002 | 48.1% |
| ab_b6_prof | 0.003 | 80.2% |
| ab_b6_seq | 0.003 | 83.9% |
| ab_b6_table | 0.003 | 90.0% |
| ab_b6_tree | 0.003 | 87.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b6_pred | 43843ns | 43843ns | +78.29% |
| ab_b6_prof | 26542ns | 26542ns | +7.93% |
| ab_b6_seq | 25482ns | 25482ns | +3.62% |
| ab_b6_table | 24591ns | 24591ns | base |
| ab_b6_tree | 25099ns | 25099ns | +2.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b6_table | 22038ns | base | --- | [20344, 24197] | --- | --- | --- | --- |
| ab_b6_pred | 41204ns | +18905.4ns (+85.8%) | [+17018, +21922]ns | [37829, 45391] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b6_prof | 24719ns | +1512.1ns (+6.9%) | [+504, +3542]ns | [22390, 25027] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b6_seq | 23625ns | no significant difference | [-216, +2331]ns | [20932, 24673] | no | 0.2917 | 0.2188 | 0 |
| ab_b6_tree | 22722ns | no significant difference | [-175, +1610]ns | [21094, 24129] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b6_table | ab_b6_pred | ab_b6_prof | ab_b6_seq | ab_b6_tree |
|---|---|---|---|---|---|
| 1 | 21260ns | +81.0% | +1.6% | +1.8% | -0.6% |
| 2 | 19826ns | +91.6% | +24.1% | +2.0% | +6.2% |
| 3 | 22815ns | +97.4% | +8.9% | +7.6% | -0.9% |
| 4 | 24121ns | +89.6% | +4.1% | +2.8% | -0.1% |
| 5 | 20861ns | +80.6% | +11.1% | +14.1% | +9.5% |
| 6 | 24272ns | +81.0% | +2.7% | -3.4% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b6_pred | -0.170 | ok |
| ab_b6_prof | -0.185 | ok |
| ab_b6_seq | 0.246 | moderate+ |
| ab_b6_table | -0.204 | moderate- |
| ab_b6_tree | 0.324 | moderate+ |

**Consistency summary:**

- **ab_b6_pred**: won 0/6, lost 6/6
- **ab_b6_prof**: won 0/6, lost 6/6
- **ab_b6_seq**: won 1/6, lost 5/6
- **ab_b6_tree**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b6_pred | 3.7ns | 41474.6ns | 0.0% |  |
| ab_b6_prof | 3.6ns | 24045.4ns | 0.0% |  |
| ab_b6_seq | 2.9ns | 23076.7ns | 0.0% |  |
| ab_b6_table | 3.4ns | 22192.8ns | 0.0% |  |
| ab_b6_tree | 4.2ns | 22648.2ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b6_pred (n=6, range 37677.9-45390.6 ns)
  37677.9 |########################################
  38063.5 |
  38449.2 |####################
  38834.8 |
  39220.4 |
  39606.1 |
  39991.7 |
  40377.3 |
  40763.0 |
  41148.6 |
  41534.2 |
  41919.9 |
  42305.5 |
  42691.2 |
  43076.8 |
  43462.4 |
  43848.1 |####################
  44233.7 |
  44619.3 |
  45005.0 |####################
  (0 below, 1 above range)

ab_b6_prof (n=6, range 21607.9-25026.9 ns)
  21607.9 |########################################
  21778.9 |
  21949.8 |
  22120.8 |
  22291.7 |
  22462.7 |
  22633.6 |
  22804.6 |
  22975.5 |
  23146.5 |########################################
  23317.4 |
  23488.4 |
  23659.3 |
  23830.2 |
  24001.2 |
  24172.2 |
  24343.1 |
  24514.1 |########################################
  24685.0 |########################################
  24856.0 |########################################
  (0 below, 1 above range)

ab_b6_seq (n=6, range 20216.7-24673.3 ns)
  20216.7 |########################################
  20439.5 |
  20662.4 |
  20885.2 |
  21108.0 |
  21330.9 |
  21553.7 |########################################
  21776.5 |
  21999.3 |
  22222.2 |
  22445.0 |
  22667.8 |
  22890.7 |
  23113.5 |
  23336.3 |########################################
  23559.2 |
  23782.0 |########################################
  24004.8 |
  24227.6 |
  24450.5 |########################################
  (0 below, 1 above range)

ab_b6_table (n=6, range 19826.2-24196.8 ns)
  19826.2 |########################################
  20044.7 |
  20263.3 |
  20481.8 |
  20700.3 |########################################
  20918.9 |
  21137.4 |########################################
  21355.9 |
  21574.5 |
  21793.0 |
  22011.5 |
  22230.1 |
  22448.6 |
  22667.1 |########################################
  22885.7 |
  23104.2 |
  23322.7 |
  23541.3 |
  23759.8 |
  23978.3 |########################################
  (0 below, 1 above range)

ab_b6_tree (n=6, range 21062.5-24128.9 ns)
  21062.5 |########################################
  21215.8 |
  21369.1 |
  21522.5 |
  21675.8 |
  21829.1 |
  21982.4 |
  22135.8 |
  22289.1 |
  22442.4 |
  22595.7 |####################
  22749.0 |####################
  22902.4 |
  23055.7 |
  23209.0 |
  23362.3 |
  23515.7 |
  23669.0 |
  23822.3 |
  23975.6 |####################
  (0 below, 1 above range)

```

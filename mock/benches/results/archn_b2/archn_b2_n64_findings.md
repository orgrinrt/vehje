# Per-branch strategy (NATIVE tier): archetype 2

5 variants, 6 samples per variant.
Baseline: **an_b2_table**

## Highlights

Baseline for all deltas below: **an_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (an_b2_table)

The baseline an_b2_table is the fastest (573 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b2_table, an_b2_prof, an_b2_seq, an_b2_tree} vs {an_b2_pred} (29% apart)

The field splits into a fast tier {an_b2_table, an_b2_prof, an_b2_seq, an_b2_tree} and a slow tier {an_b2_pred} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b2_table vs stability leader an_b2_prof (+3% speed for 1.3x steadier)

an_b2_table is fastest (573 ns, CV 9.1%); an_b2_prof gives up 2.9% median for 1.3x lower variance (CV 7.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b2_tree is inconsistent: worst-20% is 1.7x its best-20%

an_b2_tree's best 20% of batches run at 476 ns but its worst 20% at 815 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### an_b2_prof's edge over baseline is significant but tiny (4 ns, 0.65%)

an_b2_prof differs from baseline an_b2_table by 4 ns (0.65%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b2_table) is the fastest** at 573.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.33x (fastest 573.1 ns, slowest 760.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b2_pred | 3591ns | 3444ns | 2806ns | 3394ns | 4280ns | +14.57% |
| an_b2_prof | 3204ns | 3272ns | 2718ns | 3230ns | 3408ns | +2.22% |
| an_b2_seq | 3451ns | 3264ns | 2630ns | 3083ns | 4412ns | +10.08% |
| an_b2_table | 3135ns | 3167ns | 2719ns | 3089ns | 3411ns | base |
| an_b2_tree | 3446ns | 3284ns | 2627ns | 3075ns | 4411ns | +9.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b2_pred | 798ns | 630ns | 960ns | +39.97% | 0.080 |
| an_b2_prof | 582ns | 500ns | 620ns | +2.10% | 0.110 |
| an_b2_seq | 628ns | 480ns | 809ns | +10.23% | 0.102 |
| an_b2_table | 570ns | 492ns | 625ns | base | 0.112 |
| an_b2_tree | 628ns | 476ns | 815ns | +10.25% | 0.102 |

## Performance model

- Peak throughput: **0.135 Gops/s** (an_b2_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b2_pred | 0.084 | 62.6% |
| an_b2_prof | 0.109 | 80.7% |
| an_b2_seq | 0.108 | 80.6% |
| an_b2_table | 0.112 | 83.0% |
| an_b2_tree | 0.108 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b2_pred | 3591ns | 3591ns | +14.57% |
| an_b2_prof | 3204ns | 3204ns | +2.22% |
| an_b2_seq | 3451ns | 3451ns | +10.08% |
| an_b2_table | 3135ns | 3135ns | base |
| an_b2_tree | 3446ns | 3446ns | +9.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b2_table | 573ns | base | --- | [511, 625] | --- | --- | --- | --- |
| an_b2_pred | 761ns | +187.5ns (+32.7%) | [+123, +373]ns | [673, 960] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b2_prof | 590ns | no significant difference | [-26, +59]ns | [536, 620] | no | 1.0000 | 1.0000 | 0 |
| an_b2_seq | 591ns | no significant difference | [-26, +201]ns | [485, 809] | no | 1.0000 | 1.0000 | 0 |
| an_b2_tree | 591ns | no significant difference | [-33, +205]ns | [479, 815] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b2_table | an_b2_pred | an_b2_prof | an_b2_seq | an_b2_tree |
|---|---|---|---|---|---|
| 1 | 624ns | +78.3% | -8.2% | +58.6% | +60.3% |
| 2 | 538ns | +36.3% | +6.5% | +6.7% | +6.4% |
| 3 | 530ns | +48.7% | +15.5% | -7.5% | -10.3% |
| 4 | 492ns | +28.0% | +1.6% | -2.5% | -2.2% |
| 5 | 609ns | +17.7% | -0.3% | -0.1% | +0.3% |
| 6 | 627ns | +28.7% | -0.1% | +0.3% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b2_pred | -0.040 | ok |
| an_b2_prof | -0.348 | moderate- |
| an_b2_seq | 0.064 | ok |
| an_b2_table | 0.112 | ok |
| an_b2_tree | 0.067 | ok |

**Consistency summary:**

- **an_b2_pred**: won 0/6, lost 6/6
- **an_b2_prof**: won 2/6, lost 3/6
- **an_b2_seq**: won 3/6, lost 3/6
- **an_b2_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b2_pred | 4.4ns | 797.8ns | 0.6% |  |
| an_b2_prof | 6.2ns | 581.9ns | 1.1% |  |
| an_b2_seq | 7.3ns | 628.3ns | 1.2% |  |
| an_b2_table | 7.0ns | 570.0ns | 1.2% |  |
| an_b2_tree | 7.6ns | 628.4ns | 1.2% |  |

## Distribution (algo ns)

```
an_b2_pred (n=6, range 630.4-959.6 ns)
    630.4 |########################################
    646.9 |
    663.3 |
    679.8 |
    696.2 |
    712.7 |########################################
    729.2 |########################################
    745.6 |
    762.1 |
    778.5 |########################################
    795.0 |########################################
    811.5 |
    827.9 |
    844.4 |
    860.8 |
    877.3 |
    893.8 |
    910.2 |
    926.7 |
    943.1 |
  (0 below, 1 above range)

an_b2_prof (n=6, range 500.4-619.6 ns)
    500.4 |####################
    506.4 |
    512.3 |
    518.3 |
    524.2 |
    530.2 |
    536.2 |
    542.1 |
    548.1 |
    554.0 |
    560.0 |
    566.0 |
    571.9 |########################################
    577.9 |
    583.8 |
    589.8 |
    595.8 |
    601.7 |####################
    607.7 |####################
    613.6 |
  (0 below, 1 above range)

an_b2_seq (n=6, range 480.0-809.2 ns)
    480.0 |########################################
    496.5 |
    512.9 |
    529.4 |
    545.8 |
    562.3 |####################
    578.8 |
    595.2 |####################
    611.7 |
    628.1 |####################
    644.6 |
    661.1 |
    677.5 |
    694.0 |
    710.4 |
    726.9 |
    743.4 |
    759.8 |
    776.3 |
    792.7 |
  (0 below, 1 above range)

an_b2_table (n=6, range 492.5-625.5 ns)
    492.5 |########################################
    499.1 |
    505.8 |
    512.4 |
    519.1 |
    525.7 |########################################
    532.4 |########################################
    539.0 |
    545.7 |
    552.3 |
    559.0 |
    565.6 |
    572.3 |
    578.9 |
    585.6 |
    592.2 |
    598.9 |
    605.5 |########################################
    612.2 |
    618.8 |########################################
  (0 below, 1 above range)

an_b2_tree (n=6, range 475.8-815.4 ns)
    475.8 |########################################
    492.8 |
    509.8 |
    526.7 |
    543.7 |
    560.7 |####################
    577.7 |
    594.7 |####################
    611.6 |
    628.6 |####################
    645.6 |
    662.6 |
    679.6 |
    696.5 |
    713.5 |
    730.5 |
    747.5 |
    764.5 |
    781.4 |
    798.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_b2_seq**: CV=27.2% (high variance, measurements may be unstable)
- **an_b2_tree**: CV=28.1% (high variance, measurements may be unstable)

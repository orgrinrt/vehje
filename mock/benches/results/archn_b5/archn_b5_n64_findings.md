# Per-branch strategy (NATIVE tier): archetype 5

5 variants, 6 samples per variant.
Baseline: **an_b5_table**

## Highlights

Baseline for all deltas below: **an_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b5_tree, an_b5_prof) are a dead heat (<1%)

an_b5_tree (583 ns) and an_b5_prof (585 ns) differ by 0.46%, inside the noise, even though the wider field spreads 23.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (138 ns) is smaller than the fastest variant's own run-to-run std-dev (142 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### an_b5_tree's comparison is tie-heavy (17% tied pairs)

17% of paired samples for an_b5_tree are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader an_b5_tree vs stability leader an_b5_prof (+0% speed for 2.4x steadier)

an_b5_tree is fastest (583 ns, CV 24.4%); an_b5_prof gives up 0.5% median for 2.4x lower variance (CV 10.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b5_seq is inconsistent: worst-20% is 1.8x its best-20%

an_b5_seq's best 20% of batches run at 499 ns but its worst 20% at 891 ns (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### an_b5_tree's edge over baseline is significant but tiny (-1 ns, 0.21%)

an_b5_tree differs from baseline an_b5_table by -1 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b5_tree** at 582.7 ns median (-1.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.24x (fastest 582.7 ns, slowest 720.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b5_pred | 3450ns | 3411ns | 2755ns | 3229ns | 4128ns | +10.55% |
| an_b5_prof | 3087ns | 3068ns | 2725ns | 2966ns | 3450ns | -1.08% |
| an_b5_seq | 3527ns | 3319ns | 2604ns | 3102ns | 4627ns | +13.03% |
| an_b5_table | 3121ns | 3264ns | 2629ns | 3092ns | 3409ns | base |
| an_b5_tree | 3400ns | 3221ns | 2584ns | 3056ns | 4325ns | +8.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b5_pred | 733ns | 583ns | 888ns | +29.02% | 0.087 |
| an_b5_prof | 592ns | 524ns | 668ns | +4.26% | 0.108 |
| an_b5_seq | 677ns | 499ns | 891ns | +19.08% | 0.095 |
| an_b5_table | 568ns | 481ns | 626ns | base | 0.113 |
| an_b5_tree | 608ns | 468ns | 760ns | +7.04% | 0.105 |

## Performance model

- Peak throughput: **0.137 Gops/s** (an_b5_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b5_pred | 0.089 | 65.0% |
| an_b5_prof | 0.109 | 79.9% |
| an_b5_seq | 0.101 | 74.0% |
| an_b5_table | 0.108 | 79.2% |
| an_b5_tree | 0.110 | 80.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b5_pred | 3450ns | 3450ns | +10.55% |
| an_b5_prof | 3087ns | 3087ns | -1.08% |
| an_b5_seq | 3527ns | 3527ns | +13.03% |
| an_b5_table | 3121ns | 3121ns | base |
| an_b5_tree | 3400ns | 3400ns | +8.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b5_table | 591ns | base | --- | [487, 626] | --- | --- | --- | --- |
| an_b5_pred | 720ns | +129.4ns (+21.9%) | [+104, +262]ns | [591, 888] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b5_prof | 585ns | no significant difference | [-8, +47]ns | [524, 668] | no | 0.2917 | 0.2188 | 0 |
| an_b5_seq | 633ns | +41.9ns (+7.1%) | [+19, +264]ns | [506, 891] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b5_tree | 583ns | no significant difference | [-25, +146]ns | [481, 760] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b5_table | an_b5_pred | an_b5_prof | an_b5_seq | an_b5_tree |
|---|---|---|---|---|---|
| 1 | 623ns | +50.1% | -7.6% | +69.0% | +44.7% |
| 2 | 481ns | +21.1% | +8.9% | +6.8% | +2.9% |
| 3 | 493ns | +21.4% | +6.3% | +1.2% | -5.1% |
| 4 | 619ns | +21.5% | +8.2% | +8.3% | +0.0% |
| 5 | 563ns | +22.4% | +5.7% | +5.8% | -0.4% |
| 6 | 630ns | +33.5% | +5.7% | +15.6% | -3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b5_pred | -0.192 | ok |
| an_b5_prof | 0.045 | ok |
| an_b5_seq | -0.167 | ok |
| an_b5_table | -0.118 | ok |
| an_b5_tree | -0.159 | ok |

**Consistency summary:**

- **an_b5_pred**: won 0/6, lost 6/6
- **an_b5_prof**: won 1/6, lost 5/6
- **an_b5_seq**: won 0/6, lost 6/6
- **an_b5_tree**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b5_pred | 5.7ns | 733.0ns | 0.8% |  |
| an_b5_prof | 6.2ns | 592.4ns | 1.0% |  |
| an_b5_seq | 5.8ns | 676.6ns | 0.9% |  |
| an_b5_table | 6.5ns | 568.2ns | 1.1% |  |
| an_b5_tree | 7.1ns | 608.2ns | 1.2% |  |

## Distribution (algo ns)

```
an_b5_pred (n=6, range 582.9-888.1 ns)
    582.9 |########################################
    598.2 |########################################
    613.4 |
    628.7 |
    643.9 |
    659.2 |
    674.5 |########################################
    689.7 |
    705.0 |
    720.2 |
    735.5 |
    750.8 |########################################
    766.0 |
    781.3 |
    796.5 |
    811.8 |
    827.1 |########################################
    842.3 |
    857.6 |
    872.8 |
  (0 below, 1 above range)

an_b5_prof (n=6, range 523.8-667.7 ns)
    523.8 |########################################
    531.0 |
    538.2 |
    545.4 |
    552.6 |
    559.8 |
    567.0 |
    574.2 |####################
    581.4 |
    588.6 |####################
    595.8 |
    602.9 |
    610.1 |
    617.3 |
    624.5 |
    631.7 |
    638.9 |
    646.1 |
    653.3 |
    660.5 |####################
  (0 below, 1 above range)

an_b5_seq (n=6, range 499.2-890.6 ns)
    499.2 |########################################
    518.8 |
    538.3 |
    557.9 |
    577.5 |####################
    597.0 |
    616.6 |
    636.2 |
    655.8 |####################
    675.3 |
    694.9 |
    714.5 |####################
    734.0 |
    753.6 |
    773.2 |
    792.8 |
    812.3 |
    831.9 |
    851.5 |
    871.0 |
  (0 below, 1 above range)

an_b5_table (n=6, range 481.2-626.5 ns)
    481.2 |########################################
    488.5 |########################################
    495.7 |
    503.0 |
    510.2 |
    517.5 |
    524.8 |
    532.0 |
    539.3 |
    546.6 |
    553.8 |
    561.1 |########################################
    568.4 |
    575.6 |
    582.9 |
    590.1 |
    597.4 |
    604.7 |
    611.9 |########################################
    619.2 |########################################
  (0 below, 1 above range)

an_b5_tree (n=6, range 467.9-760.5 ns)
    467.9 |########################################
    482.5 |########################################
    497.2 |
    511.8 |
    526.4 |
    541.0 |
    555.7 |########################################
    570.3 |
    584.9 |
    599.5 |########################################
    614.2 |########################################
    628.8 |
    643.4 |
    658.1 |
    672.7 |
    687.3 |
    701.9 |
    716.6 |
    731.2 |
    745.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_b5_seq**: CV=27.6% (high variance, measurements may be unstable)
- **an_b5_tree**: CV=23.4% (high variance, measurements may be unstable)

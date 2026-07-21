# Per-type strategy (NATIVE tier): all match

5 variants, 6 samples per variant.
Baseline: **an_match_table**

## Highlights

Baseline for all deltas below: **an_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Two tiers: {an_match_prof, an_match_table, an_match_tree, an_match_seq} vs {an_match_pred} (67% apart)

The field splits into a fast tier {an_match_prof, an_match_table, an_match_tree, an_match_seq} and a slow tier {an_match_pred} with a 67% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_match_prof's comparison is tie-heavy (17% tied pairs)

17% of paired samples for an_match_prof are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### an_match_prof's edge over baseline is significant but tiny (7 ns, 1.04%)

an_match_prof differs from baseline an_match_table by 7 ns (1.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_match_prof** at 628.1 ns median (-2.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.74x (fastest 628.1 ns, slowest 1091.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_match_pred | 3649ns | 3875ns | 2952ns | 3587ns | 4091ns | +5.27% |
| an_match_prof | 3468ns | 3415ns | 3143ns | 3410ns | 3718ns | +0.05% |
| an_match_seq | 3402ns | 3583ns | 2640ns | 3434ns | 3736ns | -1.85% |
| an_match_table | 3467ns | 3544ns | 2726ns | 3441ns | 3875ns | base |
| an_match_tree | 3320ns | 3559ns | 2645ns | 3263ns | 3743ns | -4.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_match_pred | 1028ns | 830ns | 1151ns | +66.14% | 0.062 |
| an_match_prof | 631ns | 575ns | 670ns | +2.06% | 0.101 |
| an_match_seq | 620ns | 482ns | 680ns | +0.29% | 0.103 |
| an_match_table | 618ns | 498ns | 680ns | base | 0.103 |
| an_match_tree | 604ns | 483ns | 676ns | -2.36% | 0.106 |

## Performance model

- Peak throughput: **0.133 Gops/s** (an_match_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_match_pred | 0.059 | 44.2% |
| an_match_prof | 0.102 | 76.8% |
| an_match_seq | 0.098 | 73.9% |
| an_match_table | 0.100 | 75.2% |
| an_match_tree | 0.099 | 74.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_match_pred | 3649ns | 3649ns | +5.27% |
| an_match_prof | 3468ns | 3468ns | +0.05% |
| an_match_seq | 3402ns | 3402ns | -1.85% |
| an_match_table | 3467ns | 3467ns | base |
| an_match_tree | 3320ns | 3320ns | -4.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_match_table | 641ns | base | --- | [534, 680] | --- | --- | --- | --- |
| an_match_pred | 1092ns | +430.5ns (+67.1%) | [+305, +491]ns | [839, 1151] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_match_prof | 628ns | no significant difference | [-35, +66]ns | [595, 670] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| an_match_seq | 652ns | no significant difference | [-14, +15]ns | [529, 680] | no | 0.9167 | 0.6875 | 0 |
| an_match_tree | 649ns | no significant difference | [-50, +11]ns | [487, 676] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_match_table | an_match_pred | an_match_prof | an_match_seq | an_match_tree |
|---|---|---|---|---|---|
| 1 | 498ns | +70.3% | +15.3% | -3.3% | -1.5% |
| 2 | 570ns | +45.6% | +9.9% | +1.1% | -15.2% |
| 3 | 670ns | +62.9% | -8.0% | +1.6% | +0.6% |
| 4 | 690ns | +73.9% | -2.2% | -1.6% | -1.8% |
| 5 | 652ns | +67.3% | +2.0% | +3.1% | +2.7% |
| 6 | 630ns | +75.1% | +0.0% | +0.2% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_match_pred | 0.433 | moderate+ |
| an_match_prof | 0.178 | ok |
| an_match_seq | 0.343 | moderate+ |
| an_match_table | 0.381 | moderate+ |
| an_match_tree | 0.393 | moderate+ |

**Consistency summary:**

- **an_match_pred**: won 0/6, lost 6/6
- **an_match_prof**: won 2/6, lost 3/6
- **an_match_seq**: won 2/6, lost 4/6
- **an_match_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_match_pred | 5.5ns | 1027.5ns | 0.5% |  |
| an_match_prof | 7.2ns | 631.2ns | 1.1% |  |
| an_match_seq | 6.7ns | 620.3ns | 1.1% |  |
| an_match_table | 7.4ns | 618.5ns | 1.2% |  |
| an_match_tree | 6.5ns | 603.9ns | 1.1% |  |

## Distribution (algo ns)

```
an_match_pred (n=6, range 830.0-1151.4 ns)
    830.0 |####################
    846.1 |####################
    862.1 |
    878.2 |
    894.3 |
    910.4 |
    926.4 |
    942.5 |
    958.6 |
    974.7 |
    990.7 |
   1006.8 |
   1022.9 |
   1038.9 |
   1055.0 |
   1071.1 |
   1087.2 |########################################
   1103.2 |####################
   1119.3 |
   1135.4 |
  (0 below, 1 above range)

an_match_prof (n=6, range 574.6-670.2 ns)
    574.6 |########################################
    579.4 |
    584.2 |
    588.9 |
    593.7 |
    598.5 |
    603.3 |
    608.1 |
    612.8 |########################################
    617.6 |
    622.4 |########################################
    627.2 |########################################
    632.0 |
    636.7 |
    641.5 |
    646.3 |
    651.1 |
    655.9 |
    660.6 |
    665.4 |########################################
  (0 below, 1 above range)

an_match_seq (n=6, range 482.1-679.5 ns)
    482.1 |####################
    492.0 |
    501.8 |
    511.7 |
    521.6 |
    531.5 |
    541.3 |
    551.2 |
    561.1 |
    571.0 |####################
    580.8 |
    590.7 |
    600.6 |
    610.4 |
    620.3 |
    630.2 |####################
    640.1 |
    649.9 |
    659.8 |
    669.7 |########################################
  (0 below, 1 above range)

an_match_table (n=6, range 498.3-680.0 ns)
    498.3 |########################################
    507.4 |
    516.5 |
    525.6 |
    534.6 |
    543.7 |
    552.8 |
    561.9 |########################################
    571.0 |
    580.1 |
    589.1 |
    598.2 |
    607.3 |
    616.4 |
    625.5 |########################################
    634.6 |
    643.7 |########################################
    652.7 |
    661.8 |########################################
    670.9 |
  (0 below, 1 above range)

an_match_tree (n=6, range 483.3-675.9 ns)
    483.3 |########################################
    492.9 |
    502.6 |
    512.2 |
    521.8 |
    531.4 |
    541.1 |
    550.7 |
    560.3 |
    569.9 |
    579.6 |
    589.2 |
    598.8 |
    608.5 |
    618.1 |####################
    627.7 |
    637.3 |
    647.0 |
    656.6 |
    666.2 |########################################
  (0 below, 1 above range)

```

# Per-branch strategy (NATIVE tier): archetype 0

5 variants, 6 samples per variant.
Baseline: **an_b0_table**

## Highlights

Baseline for all deltas below: **an_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b0_prof, an_b0_seq) are a dead heat (<1%)

an_b0_prof (565 ns) and an_b0_seq (568 ns) differ by 0.44%, inside the noise, even though the wider field spreads 5.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (31 ns) is smaller than the fastest variant's own run-to-run std-dev (41 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### an_b0_prof's edge over baseline is significant but tiny (-0 ns, 0.01%)

an_b0_prof differs from baseline an_b0_table by -0 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b0_prof** at 565.2 ns median (-0.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.05x (fastest 565.2 ns, slowest 596.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b0_pred | 3122ns | 3168ns | 2645ns | 3018ns | 3517ns | -2.49% |
| an_b0_prof | 2980ns | 3132ns | 2607ns | 2996ns | 3142ns | -6.95% |
| an_b0_seq | 3017ns | 3137ns | 2593ns | 2978ns | 3287ns | -5.78% |
| an_b0_table | 3202ns | 3138ns | 2705ns | 3125ns | 3566ns | base |
| an_b0_tree | 3050ns | 3145ns | 2637ns | 2981ns | 3360ns | -4.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b0_pred | 588ns | 500ns | 660ns | +2.81% | 0.109 |
| an_b0_prof | 543ns | 476ns | 577ns | -5.09% | 0.118 |
| an_b0_seq | 547ns | 470ns | 595ns | -4.41% | 0.117 |
| an_b0_table | 572ns | 492ns | 634ns | base | 0.112 |
| an_b0_tree | 550ns | 481ns | 597ns | -3.81% | 0.116 |

## Performance model

- Peak throughput: **0.136 Gops/s** (an_b0_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b0_pred | 0.107 | 78.8% |
| an_b0_prof | 0.113 | 83.2% |
| an_b0_seq | 0.113 | 82.8% |
| an_b0_table | 0.112 | 82.5% |
| an_b0_tree | 0.112 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b0_pred | 3122ns | 3122ns | -2.49% |
| an_b0_prof | 2980ns | 2980ns | -6.95% |
| an_b0_seq | 3017ns | 3017ns | -5.78% |
| an_b0_table | 3202ns | 3202ns | base |
| an_b0_tree | 3050ns | 3050ns | -4.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b0_table | 570ns | base | --- | [513, 634] | --- | --- | --- | --- |
| an_b0_pred | 596ns | no significant difference | [-43, +67]ns | [508, 660] | no | 0.9167 | 0.6875 | 0 |
| an_b0_prof | 565ns | no significant difference | [-90, +3]ns | [486, 577] | no | 1.0000 | 1.0000 | 0 |
| an_b0_seq | 568ns | -5.0ns (-0.9%) | [-70, -1]ns | [477, 595] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b0_tree | 573ns | no significant difference | [-84, +26]ns | [481, 597] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b0_table | an_b0_pred | an_b0_prof | an_b0_seq | an_b0_tree |
|---|---|---|---|---|---|
| 1 | 492ns | +4.8% | +1.0% | -1.4% | -2.2% |
| 2 | 534ns | -6.4% | -10.9% | -12.0% | -10.0% |
| 3 | 567ns | +4.6% | +0.1% | -0.5% | +8.7% |
| 4 | 572ns | +4.8% | +0.1% | -0.1% | +0.4% |
| 5 | 685ns | -7.6% | -17.8% | -11.0% | -16.7% |
| 6 | 582ns | +18.2% | -0.1% | -0.2% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b0_pred | 0.442 | moderate+ |
| an_b0_prof | 0.360 | moderate+ |
| an_b0_seq | 0.482 | moderate+ |
| an_b0_table | 0.210 | moderate+ |
| an_b0_tree | 0.185 | ok |

**Consistency summary:**

- **an_b0_pred**: won 2/6, lost 4/6
- **an_b0_prof**: won 2/6, lost 2/6
- **an_b0_seq**: won 6/6, lost 0/6
- **an_b0_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b0_pred | 5.8ns | 588.1ns | 1.0% |  |
| an_b0_prof | 6.2ns | 542.9ns | 1.1% |  |
| an_b0_seq | 6.5ns | 546.8ns | 1.2% |  |
| an_b0_table | 5.3ns | 572.0ns | 0.9% |  |
| an_b0_tree | 5.6ns | 550.2ns | 1.0% |  |

## Distribution (algo ns)

```
an_b0_pred (n=6, range 500.0-660.4 ns)
    500.0 |########################################
    508.0 |########################################
    516.0 |
    524.1 |
    532.1 |
    540.1 |
    548.1 |
    556.1 |
    564.2 |
    572.2 |
    580.2 |
    588.2 |########################################
    596.2 |########################################
    604.3 |
    612.3 |
    620.3 |
    628.3 |########################################
    636.3 |
    644.4 |
    652.4 |
  (0 below, 1 above range)

an_b0_prof (n=6, range 475.8-577.2 ns)
    475.8 |####################
    480.9 |
    485.9 |
    491.0 |
    496.1 |####################
    501.2 |
    506.2 |
    511.3 |
    516.4 |
    521.5 |
    526.5 |
    531.6 |
    536.7 |
    541.7 |
    546.8 |
    551.9 |
    557.0 |
    562.0 |########################################
    567.1 |
    572.2 |####################
  (0 below, 1 above range)

an_b0_seq (n=6, range 470.0-595.4 ns)
    470.0 |########################################
    476.3 |
    482.5 |########################################
    488.8 |
    495.1 |
    501.4 |
    507.6 |
    513.9 |
    520.2 |
    526.4 |
    532.7 |
    539.0 |
    545.2 |
    551.5 |
    557.8 |########################################
    564.0 |
    570.3 |########################################
    576.6 |########################################
    582.9 |
    589.1 |
  (0 below, 1 above range)

an_b0_table (n=6, range 491.7-633.5 ns)
    491.7 |########################################
    498.8 |
    505.9 |
    513.0 |
    520.1 |
    527.2 |########################################
    534.3 |
    541.3 |
    548.4 |
    555.5 |
    562.6 |########################################
    569.7 |########################################
    576.8 |########################################
    583.9 |
    591.0 |
    598.1 |
    605.2 |
    612.3 |
    619.4 |
    626.5 |
  (0 below, 1 above range)

an_b0_tree (n=6, range 480.8-597.2 ns)
    480.8 |########################################
    486.6 |
    492.4 |
    498.3 |
    504.1 |
    509.9 |
    515.7 |
    521.6 |
    527.4 |
    533.2 |
    539.0 |
    544.8 |
    550.7 |
    556.5 |
    562.3 |
    568.1 |####################
    574.0 |########################################
    579.8 |
    585.6 |
    591.4 |
  (0 below, 1 above range)

```

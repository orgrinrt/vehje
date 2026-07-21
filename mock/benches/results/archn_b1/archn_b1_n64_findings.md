# Per-branch strategy (NATIVE tier): archetype 1

5 variants, 6 samples per variant.
Baseline: **an_b1_table**

## Highlights

Baseline for all deltas below: **an_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b1_tree is fastest but the noisiest (CV 25.4%)

an_b1_tree wins on median (568 ns) yet has the highest variance (CV 25.4%), while an_b1_seq is the steadiest (CV 19.9%, 601 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader an_b1_tree vs stability leader an_b1_seq (+6% speed for 1.3x steadier)

an_b1_tree is fastest (568 ns, CV 25.4%); an_b1_seq gives up 5.9% median for 1.3x lower variance (CV 19.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b1_tree is inconsistent: worst-20% is 1.7x its best-20%

an_b1_tree's best 20% of batches run at 470 ns but its worst 20% at 810 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### an_b1_prof's edge over baseline is significant but tiny (-1 ns, 0.21%)

an_b1_prof differs from baseline an_b1_table by -1 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b1_tree** at 567.8 ns median (-3.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.28x (fastest 567.8 ns, slowest 727.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b1_pred | 3777ns | 3543ns | 3204ns | 3446ns | 4562ns | +9.20% |
| an_b1_prof | 3350ns | 3269ns | 2622ns | 3094ns | 4098ns | -3.15% |
| an_b1_seq | 3478ns | 3280ns | 2657ns | 3235ns | 4254ns | +0.55% |
| an_b1_table | 3459ns | 3275ns | 2620ns | 3236ns | 4213ns | base |
| an_b1_tree | 3422ns | 3145ns | 2622ns | 2983ns | 4480ns | -1.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b1_pred | 762ns | 561ns | 947ns | +21.72% | 0.084 |
| an_b1_prof | 607ns | 470ns | 741ns | -3.06% | 0.105 |
| an_b1_seq | 632ns | 477ns | 772ns | +0.97% | 0.101 |
| an_b1_table | 626ns | 468ns | 770ns | base | 0.102 |
| an_b1_tree | 617ns | 470ns | 810ns | -1.41% | 0.104 |

## Performance model

- Peak throughput: **0.137 Gops/s** (an_b1_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b1_pred | 0.088 | 64.4% |
| an_b1_prof | 0.106 | 77.9% |
| an_b1_seq | 0.106 | 77.9% |
| an_b1_table | 0.109 | 79.6% |
| an_b1_tree | 0.113 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b1_pred | 3777ns | 3777ns | +9.20% |
| an_b1_prof | 3350ns | 3350ns | -3.15% |
| an_b1_seq | 3478ns | 3478ns | +0.55% |
| an_b1_table | 3459ns | 3459ns | base |
| an_b1_tree | 3422ns | 3422ns | -1.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b1_table | 588ns | base | --- | [520, 770] | --- | --- | --- | --- |
| an_b1_pred | 728ns | +146.2ns (+24.9%) | [+79, +182]ns | [612, 947] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b1_prof | 601ns | no significant difference | [-82, +25]ns | [479, 741] | no | 1.0000 | 1.0000 | 0 |
| an_b1_seq | 601ns | no significant difference | [-15, +29]ns | [524, 772] | no | 0.9167 | 0.6875 | 0 |
| an_b1_tree | 568ns | no significant difference | [-65, +43]ns | [474, 810] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b1_table | an_b1_pred | an_b1_prof | an_b1_seq | an_b1_tree |
|---|---|---|---|---|---|
| 1 | 864ns | +16.5% | -0.4% | -0.3% | -0.5% |
| 2 | 675ns | +31.3% | -8.0% | +1.1% | +12.5% |
| 3 | 598ns | +11.0% | -18.3% | -4.6% | -5.2% |
| 4 | 572ns | +26.2% | +8.6% | +8.4% | -0.6% |
| 5 | 468ns | +19.8% | +0.3% | +1.9% | +0.3% |
| 6 | 579ns | +26.5% | +0.3% | +0.3% | -17.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b1_pred | 0.281 | moderate+ |
| an_b1_prof | 0.018 | ok |
| an_b1_seq | 0.220 | moderate+ |
| an_b1_table | 0.308 | moderate+ |
| an_b1_tree | 0.459 | moderate+ |

**Consistency summary:**

- **an_b1_pred**: won 0/6, lost 6/6
- **an_b1_prof**: won 3/6, lost 3/6
- **an_b1_seq**: won 2/6, lost 4/6
- **an_b1_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b1_pred | 6.9ns | 762.2ns | 0.9% |  |
| an_b1_prof | 7.5ns | 607.0ns | 1.2% |  |
| an_b1_seq | 6.4ns | 632.3ns | 1.0% |  |
| an_b1_table | 6.5ns | 626.2ns | 1.0% |  |
| an_b1_tree | 6.8ns | 617.4ns | 1.1% |  |

## Distribution (algo ns)

```
an_b1_pred (n=6, range 561.2-946.7 ns)
    561.2 |####################
    580.5 |
    599.8 |
    619.0 |
    638.3 |
    657.6 |####################
    676.9 |
    696.1 |
    715.4 |########################################
    734.7 |
    754.0 |
    773.2 |
    792.5 |
    811.8 |
    831.0 |
    850.3 |
    869.6 |####################
    888.9 |
    908.2 |
    927.4 |
  (0 below, 1 above range)

an_b1_prof (n=6, range 469.6-741.0 ns)
    469.6 |####################
    483.2 |####################
    496.7 |
    510.3 |
    523.9 |
    537.5 |
    551.0 |
    564.6 |
    578.2 |####################
    591.8 |
    605.3 |
    618.9 |########################################
    632.5 |
    646.0 |
    659.6 |
    673.2 |
    686.8 |
    700.3 |
    713.9 |
    727.5 |
  (0 below, 1 above range)

an_b1_seq (n=6, range 477.1-772.3 ns)
    477.1 |########################################
    491.9 |
    506.6 |
    521.4 |
    536.1 |
    550.9 |
    565.7 |########################################
    580.4 |########################################
    595.2 |
    609.9 |########################################
    624.7 |
    639.5 |
    654.2 |
    669.0 |########################################
    683.7 |
    698.5 |
    713.3 |
    728.0 |
    742.8 |
    757.5 |
  (0 below, 1 above range)

an_b1_table (n=6, range 468.3-769.8 ns)
    468.3 |########################################
    483.4 |
    498.4 |
    513.5 |
    528.6 |
    543.7 |
    558.8 |########################################
    573.8 |########################################
    588.9 |########################################
    604.0 |
    619.0 |
    634.1 |
    649.2 |
    664.3 |########################################
    679.3 |
    694.4 |
    709.5 |
    724.6 |
    739.6 |
    754.7 |
  (0 below, 1 above range)

an_b1_tree (n=6, range 469.6-810.0 ns)
    469.6 |########################################
    486.6 |
    503.6 |
    520.7 |
    537.7 |
    554.7 |########################################
    571.7 |
    588.7 |
    605.8 |
    622.8 |
    639.8 |
    656.8 |
    673.8 |
    690.9 |
    707.9 |
    724.9 |
    741.9 |
    758.9 |####################
    776.0 |
    793.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_b1_prof**: CV=21.1% (high variance, measurements may be unstable)
- **an_b1_tree**: CV=23.4% (high variance, measurements may be unstable)

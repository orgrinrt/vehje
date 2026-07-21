# Per-branch strategy (NATIVE tier): archetype 6

5 variants, 6 samples per variant.
Baseline: **an_b6_table**

## Highlights

Baseline for all deltas below: **an_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b6_table is fastest but the noisiest (CV 24.7%)

an_b6_table wins on median (551 ns) yet has the highest variance (CV 24.7%), while an_b6_pred is the steadiest (CV 14.9%, 1.06 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (an_b6_table)

The baseline an_b6_table is the fastest (551 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b6_table, an_b6_prof, an_b6_tree, an_b6_seq} vs {an_b6_pred} (69% apart)

The field splits into a fast tier {an_b6_table, an_b6_prof, an_b6_tree, an_b6_seq} and a slow tier {an_b6_pred} with a 69% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_b6_table is inconsistent: worst-20% is 1.6x its best-20%

an_b6_table's best 20% of batches run at 472 ns but its worst 20% at 767 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### an_b6_tree's edge over baseline is significant but tiny (2 ns, 0.30%)

an_b6_tree differs from baseline an_b6_table by 2 ns (0.30%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b6_table) is the fastest** at 550.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.92x (fastest 550.6 ns, slowest 1056.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b6_pred | 3843ns | 3741ns | 2991ns | 3684ns | 4507ns | +18.59% |
| an_b6_prof | 3060ns | 2974ns | 2642ns | 2870ns | 3555ns | -5.56% |
| an_b6_seq | 3331ns | 3300ns | 2668ns | 3115ns | 3988ns | +2.81% |
| an_b6_table | 3240ns | 3038ns | 2592ns | 2899ns | 4077ns | base |
| an_b6_tree | 3368ns | 3435ns | 2622ns | 3169ns | 4039ns | +3.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b6_pred | 1082ns | 840ns | 1268ns | +81.11% | 0.059 |
| an_b6_prof | 585ns | 490ns | 698ns | -2.03% | 0.109 |
| an_b6_seq | 628ns | 494ns | 753ns | +5.14% | 0.102 |
| an_b6_table | 598ns | 472ns | 767ns | base | 0.107 |
| an_b6_tree | 611ns | 468ns | 739ns | +2.18% | 0.105 |

## Performance model

- Peak throughput: **0.137 Gops/s** (an_b6_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b6_pred | 0.061 | 44.3% |
| an_b6_prof | 0.114 | 83.6% |
| an_b6_seq | 0.102 | 74.8% |
| an_b6_table | 0.116 | 85.0% |
| an_b6_tree | 0.104 | 76.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b6_pred | 3843ns | 3843ns | +18.59% |
| an_b6_prof | 3060ns | 3060ns | -5.56% |
| an_b6_seq | 3331ns | 3331ns | +2.81% |
| an_b6_table | 3240ns | 3240ns | base |
| an_b6_tree | 3368ns | 3368ns | +3.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b6_table | 551ns | base | --- | [475, 767] | --- | --- | --- | --- |
| an_b6_pred | 1056ns | +500.8ns (+90.9%) | [+402, +551]ns | [922, 1268] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b6_prof | 560ns | no significant difference | [-143, +79]ns | [498, 698] | no | 0.4375 | 0.2188 | 0 |
| an_b6_seq | 625ns | no significant difference | [-42, +111]ns | [507, 753] | no | 0.9167 | 0.6875 | 0 |
| an_b6_tree | 616ns | no significant difference | [-78, +115]ns | [477, 739] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b6_table | an_b6_pred | an_b6_prof | an_b6_seq | an_b6_tree |
|---|---|---|---|---|---|
| 1 | 864ns | +55.0% | -35.2% | -8.2% | -12.0% |
| 2 | 534ns | +105.5% | +4.9% | -2.6% | +34.6% |
| 3 | 568ns | +77.0% | +12.8% | +4.5% | +8.2% |
| 4 | 670ns | +78.5% | +12.9% | +6.3% | -7.8% |
| 5 | 472ns | +77.9% | +3.7% | +4.6% | -0.9% |
| 6 | 478ns | +112.6% | +6.0% | +37.5% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b6_pred | -0.121 | ok |
| an_b6_prof | -0.005 | ok |
| an_b6_seq | -0.486 | moderate- |
| an_b6_table | -0.102 | ok |
| an_b6_tree | 0.475 | moderate+ |

**Consistency summary:**

- **an_b6_pred**: won 0/6, lost 6/6
- **an_b6_prof**: won 1/6, lost 5/6
- **an_b6_seq**: won 2/6, lost 4/6
- **an_b6_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b6_pred | 6.8ns | 1082.1ns | 0.6% |  |
| an_b6_prof | 6.0ns | 585.4ns | 1.0% |  |
| an_b6_seq | 6.0ns | 628.2ns | 1.0% |  |
| an_b6_table | 6.7ns | 597.5ns | 1.1% |  |
| an_b6_tree | 6.8ns | 610.6ns | 1.1% |  |

## Distribution (algo ns)

```
an_b6_pred (n=6, range 840.0-1267.7 ns)
    840.0 |########################################
    861.4 |
    882.8 |
    904.2 |
    925.5 |
    946.9 |
    968.3 |
    989.7 |########################################
   1011.1 |########################################
   1032.5 |
   1053.8 |
   1075.2 |
   1096.6 |########################################
   1118.0 |
   1139.4 |
   1160.8 |
   1182.2 |########################################
   1203.5 |
   1224.9 |
   1246.3 |
  (0 below, 1 above range)

an_b6_prof (n=6, range 489.6-698.1 ns)
    489.6 |####################
    500.0 |####################
    510.5 |
    520.9 |
    531.3 |
    541.7 |
    552.1 |########################################
    562.6 |
    573.0 |
    583.4 |
    593.9 |
    604.3 |
    614.7 |
    625.1 |
    635.6 |####################
    646.0 |
    656.4 |
    666.8 |
    677.2 |
    687.7 |
  (0 below, 1 above range)

an_b6_seq (n=6, range 493.7-752.7 ns)
    493.7 |########################################
    506.6 |
    519.6 |########################################
    532.5 |
    545.5 |
    558.5 |
    571.4 |
    584.4 |########################################
    597.3 |
    610.2 |
    623.2 |
    636.2 |
    649.1 |########################################
    662.0 |
    675.0 |
    688.0 |
    700.9 |########################################
    713.9 |
    726.8 |
    739.8 |
  (0 below, 1 above range)

an_b6_table (n=6, range 472.1-766.9 ns)
    472.1 |########################################
    486.8 |
    501.6 |
    516.3 |
    531.1 |####################
    545.8 |
    560.5 |####################
    575.3 |
    590.0 |
    604.8 |
    619.5 |
    634.2 |
    649.0 |
    663.7 |####################
    678.5 |
    693.2 |
    707.9 |
    722.7 |
    737.4 |
    752.2 |
  (0 below, 1 above range)

an_b6_tree (n=6, range 467.9-739.1 ns)
    467.9 |########################################
    481.5 |########################################
    495.0 |
    508.6 |
    522.1 |
    535.7 |
    549.3 |
    562.8 |
    576.4 |
    590.0 |
    603.5 |########################################
    617.1 |########################################
    630.6 |
    644.2 |
    657.8 |
    671.3 |
    684.9 |
    698.5 |
    712.0 |########################################
    725.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_b6_table**: CV=22.8% (high variance, measurements may be unstable)

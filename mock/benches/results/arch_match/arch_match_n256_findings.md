# Per-type strategy: all match branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_match_table**

## Highlights

Baseline for all deltas below: **ab_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_match_pred is an outlier: 2.8x slower than the field

ab_match_pred (227.99 us) is 2.8x the fastest (82.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_match_prof, ab_match_tree) are a dead heat (<1%)

ab_match_prof (82.08 us) and ab_match_tree (82.84 us) differ by 0.92%, inside the noise, even though the wider field spreads 177.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ab_match_prof, ab_match_tree, ab_match_table, ab_match_seq} vs {ab_match_pred} (162% apart)

The field splits into a fast tier {ab_match_prof, ab_match_tree, ab_match_table, ab_match_seq} and a slow tier {ab_match_pred} with a 162% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_match_prof** at 82080.6 ns median (-1.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.78x (fastest 82080.6 ns, slowest 227992.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_match_pred | 231296ns | 230398ns | 224565ns | 229014ns | 238085ns | +167.73% |
| ab_match_prof | 85792ns | 84348ns | 83004ns | 84139ns | 89666ns | -0.70% |
| ab_match_seq | 89733ns | 89294ns | 82171ns | 87507ns | 96854ns | +3.87% |
| ab_match_table | 86393ns | 85852ns | 81890ns | 84909ns | 90869ns | base |
| ab_match_tree | 86444ns | 85065ns | 82310ns | 84652ns | 91199ns | +0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_match_pred | 228762ns | 222230ns | 235408ns | +172.30% | 0.001 |
| ab_match_prof | 83496ns | 80737ns | 87271ns | -0.61% | 0.003 |
| ab_match_seq | 87340ns | 79924ns | 94316ns | +3.96% | 0.003 |
| ab_match_table | 84011ns | 79642ns | 88312ns | base | 0.003 |
| ab_match_tree | 84133ns | 80178ns | 88682ns | +0.14% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_match_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_match_pred | 0.001 | 34.9% |
| ab_match_prof | 0.003 | 97.0% |
| ab_match_seq | 0.003 | 91.6% |
| ab_match_table | 0.003 | 95.4% |
| ab_match_tree | 0.003 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_match_pred | 231296ns | 231296ns | +167.73% |
| ab_match_prof | 85792ns | 85792ns | -0.70% |
| ab_match_seq | 89733ns | 89733ns | +3.87% |
| ab_match_table | 86393ns | 86393ns | base |
| ab_match_tree | 86444ns | 86444ns | +0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_match_table | 83469ns | base | --- | [80251, 88312] | --- | --- | --- | --- |
| ab_match_pred | 227993ns | +143521.9ns (+171.9%) | [+140680, +150052]ns | [222887, 235408] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_match_prof | 82081ns | no significant difference | [-6963, +4664]ns | [81138, 87271] | no | 1.0000 | 1.0000 | 0 |
| ab_match_seq | 86916ns | no significant difference | [-2673, +9364]ns | [80787, 94316] | no | 0.9167 | 0.6875 | 0 |
| ab_match_tree | 82839ns | no significant difference | [-3474, +3109]ns | [80877, 88682] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_match_table | ab_match_pred | ab_match_prof | ab_match_seq | ab_match_tree |
|---|---|---|---|---|---|
| 1 | 79642ns | +180.7% | +2.4% | +0.4% | +0.7% |
| 2 | 82566ns | +173.4% | -0.4% | -0.8% | +1.1% |
| 3 | 90286ns | +155.1% | -10.6% | +7.0% | +3.0% |
| 4 | 84372ns | +184.5% | +8.8% | +8.9% | -3.3% |
| 5 | 86339ns | +167.3% | -5.1% | -5.4% | -4.8% |
| 6 | 80860ns | +174.8% | +2.3% | +13.8% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_match_pred | 0.178 | ok |
| ab_match_prof | -0.334 | moderate- |
| ab_match_seq | -0.083 | ok |
| ab_match_table | -0.092 | ok |
| ab_match_tree | -0.202 | moderate- |

**Consistency summary:**

- **ab_match_pred**: won 0/6, lost 6/6
- **ab_match_prof**: won 3/6, lost 3/6
- **ab_match_seq**: won 2/6, lost 4/6
- **ab_match_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_match_pred | 6.3ns | 228762.4ns | 0.0% |  |
| ab_match_prof | 4.9ns | 83496.5ns | 0.0% |  |
| ab_match_seq | 5.3ns | 87339.5ns | 0.0% |  |
| ab_match_table | 6.9ns | 84010.8ns | 0.0% |  |
| ab_match_tree | 4.7ns | 84132.6ns | 0.0% |  |

## Distribution (algo ns)

```
ab_match_pred (n=6, range 222230.4-235408.0 ns)
  222230.4 |####################
  222889.3 |####################
  223548.2 |
  224207.0 |
  224865.9 |
  225524.8 |####################
  226183.7 |
  226842.5 |
  227501.4 |
  228160.3 |
  228819.2 |
  229478.1 |
  230136.9 |########################################
  230795.8 |
  231454.7 |
  232113.6 |
  232772.4 |
  233431.3 |
  234090.2 |
  234749.1 |
  (0 below, 1 above range)

ab_match_prof (n=6, range 80736.7-87271.1 ns)
  80736.7 |########################################
  81063.4 |
  81390.1 |########################################
  81716.9 |########################################
  82043.6 |########################################
  82370.3 |
  82697.0 |########################################
  83023.7 |
  83350.4 |
  83677.2 |
  84003.9 |
  84330.6 |
  84657.3 |
  84984.0 |
  85310.7 |
  85637.5 |
  85964.2 |
  86290.9 |
  86617.6 |
  86944.3 |
  (0 below, 1 above range)

ab_match_seq (n=6, range 79923.8-94316.1 ns)
  79923.8 |####################
  80643.4 |
  81363.0 |########################################
  82082.6 |
  82802.2 |
  83521.9 |
  84241.5 |
  84961.1 |
  85680.7 |
  86400.3 |
  87119.9 |
  87839.5 |
  88559.1 |
  89278.8 |
  89998.4 |
  90718.0 |
  91437.6 |########################################
  92157.2 |
  92876.8 |
  93596.4 |
  (0 below, 1 above range)

ab_match_table (n=6, range 79642.5-88312.3 ns)
  79642.5 |########################################
  80076.0 |
  80509.5 |########################################
  80943.0 |
  81376.5 |
  81809.9 |
  82243.4 |########################################
  82676.9 |
  83110.4 |
  83543.9 |
  83977.4 |########################################
  84410.9 |
  84844.4 |
  85277.9 |
  85711.4 |
  86144.9 |########################################
  86578.3 |
  87011.8 |
  87445.3 |
  87878.8 |
  (0 below, 1 above range)

ab_match_tree (n=6, range 80178.3-88682.1 ns)
  80178.3 |########################################
  80603.5 |
  81028.7 |
  81453.9 |########################################
  81879.1 |########################################
  82304.2 |
  82729.4 |
  83154.6 |########################################
  83579.8 |
  84005.0 |########################################
  84430.2 |
  84855.4 |
  85280.6 |
  85705.8 |
  86131.0 |
  86556.2 |
  86981.3 |
  87406.5 |
  87831.7 |
  88256.9 |
  (0 below, 1 above range)

```

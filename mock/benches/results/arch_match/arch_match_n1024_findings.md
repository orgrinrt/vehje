# Per-type strategy: all match branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_match_table**

## Highlights

Baseline for all deltas below: **ab_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_match_pred is an outlier: 2.7x slower than the field

ab_match_pred (887.06 us) is 2.7x the fastest (329.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_match_table, ab_match_seq) are a dead heat (<1%)

ab_match_table (329.59 us) and ab_match_seq (330.55 us) differ by 0.29%, inside the noise, even though the wider field spreads 169.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_match_prof shows alternating (throttle bounce) (autocorr -0.69)

ab_match_prof's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_match_table)

The baseline ab_match_table is the fastest (329.59 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_match_table, ab_match_seq, ab_match_tree, ab_match_prof} vs {ab_match_pred} (162% apart)

The field splits into a fast tier {ab_match_table, ab_match_seq, ab_match_tree, ab_match_prof} and a slow tier {ab_match_pred} with a 162% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_match_table) is the fastest** at 329594.8 ns median
- 2 variants significantly slower than baseline
- Spread: 2.69x (fastest 329594.8 ns, slowest 887061.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_match_pred | 890251ns | 890421ns | 885861ns | 889517ns | 893546ns | +166.89% |
| ab_match_prof | 339820ns | 340589ns | 334365ns | 338726ns | 344188ns | +1.87% |
| ab_match_seq | 333856ns | 333166ns | 326181ns | 333010ns | 338961ns | +0.09% |
| ab_match_table | 333568ns | 332124ns | 329655ns | 331940ns | 337965ns | base |
| ab_match_tree | 332212ns | 333811ns | 324664ns | 331074ns | 337694ns | -0.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_match_pred | 887077ns | 882868ns | 890317ns | +167.95% | 0.001 |
| ab_match_prof | 337340ns | 331813ns | 341688ns | +1.90% | 0.003 |
| ab_match_seq | 331282ns | 323624ns | 336399ns | +0.07% | 0.003 |
| ab_match_table | 331056ns | 327397ns | 335366ns | base | 0.003 |
| ab_match_tree | 329693ns | 322065ns | 335214ns | -0.41% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_match_tree; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_match_pred | 0.001 | 36.3% |
| ab_match_prof | 0.003 | 95.2% |
| ab_match_seq | 0.003 | 97.4% |
| ab_match_table | 0.003 | 97.7% |
| ab_match_tree | 0.003 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_match_pred | 890251ns | 890251ns | +166.89% |
| ab_match_prof | 339820ns | 339820ns | +1.87% |
| ab_match_seq | 333856ns | 333856ns | +0.09% |
| ab_match_table | 333568ns | 333568ns | base |
| ab_match_tree | 332212ns | 332212ns | -0.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_match_table | 329595ns | base | --- | [328207, 335366] | --- | --- | --- | --- |
| ab_match_pred | 887062ns | +557194.2ns (+169.1%) | [+550147, +560722]ns | [883852, 890317] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_match_prof | 338314ns | +4333.3ns (+1.3%) | [+2404, +12116]ns | [332020, 341688] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_match_seq | 330549ns | no significant difference | [-5484, +7563]ns | [326898, 336399] | no | 1.0000 | 1.0000 | 0 |
| ab_match_tree | 331364ns | no significant difference | [-8839, +5922]ns | [322503, 335214] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_match_table | ab_match_pred | ab_match_prof | ab_match_seq | ab_match_tree |
|---|---|---|---|---|---|
| 1 | 327397ns | +169.7% | +3.5% | +0.8% | -1.6% |
| 2 | 335224ns | +164.0% | +0.8% | -1.5% | -3.7% |
| 3 | 335509ns | +164.1% | +1.8% | -1.4% | -1.6% |
| 4 | 329622ns | +169.8% | +0.8% | -1.8% | +0.9% |
| 5 | 329017ns | +169.9% | +3.9% | +3.8% | +2.1% |
| 6 | 329567ns | +170.4% | +0.7% | +0.6% | +1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_match_pred | 0.317 | moderate+ |
| ab_match_prof | -0.687 | HIGH- (thermal bounce) |
| ab_match_seq | -0.435 | moderate- |
| ab_match_table | 0.049 | ok |
| ab_match_tree | 0.562 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **ab_match_pred**: won 0/6, lost 6/6
- **ab_match_prof**: won 0/6, lost 6/6
- **ab_match_seq**: won 3/6, lost 3/6
- **ab_match_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_match_pred | 9.0ns | 887077.2ns | 0.0% |  |
| ab_match_prof | 4.9ns | 337340.5ns | 0.0% |  |
| ab_match_seq | 7.0ns | 331282.0ns | 0.0% |  |
| ab_match_table | 7.9ns | 331056.0ns | 0.0% |  |
| ab_match_tree | 5.0ns | 329693.5ns | 0.0% |  |

## Distribution (algo ns)

```
ab_match_pred (n=6, range 882868.3-890317.2 ns)
  882868.3 |########################################
  883240.7 |
  883613.2 |
  883985.6 |
  884358.1 |
  884730.5 |########################################
  885103.0 |
  885475.4 |
  885847.9 |########################################
  886220.3 |
  886592.8 |
  886965.2 |
  887337.7 |
  887710.1 |########################################
  888082.6 |
  888455.0 |
  888827.5 |
  889199.9 |########################################
  889572.4 |
  889944.8 |
  (0 below, 1 above range)

ab_match_prof (n=6, range 331812.9-341687.5 ns)
  331812.9 |########################################
  332306.6 |
  332800.4 |
  333294.1 |
  333787.8 |
  334281.6 |
  334775.3 |
  335269.0 |
  335762.7 |
  336256.5 |
  336750.2 |
  337243.9 |
  337737.7 |####################
  338231.4 |
  338725.1 |####################
  339218.8 |
  339712.6 |
  340206.3 |
  340700.0 |
  341193.8 |####################
  (0 below, 1 above range)

ab_match_seq (n=6, range 323624.2-336399.2 ns)
  323624.2 |####################
  324263.0 |
  324901.7 |
  325540.5 |
  326179.2 |
  326818.0 |
  327456.7 |
  328095.5 |
  328734.2 |
  329373.0 |
  330011.7 |########################################
  330650.4 |####################
  331289.2 |####################
  331927.9 |
  332566.7 |
  333205.4 |
  333844.2 |
  334482.9 |
  335121.7 |
  335760.4 |
  (0 below, 1 above range)

ab_match_table (n=6, range 327397.1-335366.3 ns)
  327397.1 |####################
  327795.6 |
  328194.0 |
  328592.5 |
  328990.9 |####################
  329389.4 |########################################
  329787.9 |
  330186.3 |
  330584.8 |
  330983.2 |
  331381.7 |
  331780.2 |
  332178.6 |
  332577.1 |
  332975.5 |
  333374.0 |
  333772.5 |
  334170.9 |
  334569.4 |
  334967.8 |####################
  (0 below, 1 above range)

ab_match_tree (n=6, range 322064.6-335213.8 ns)
  322064.6 |########################################
  322722.1 |########################################
  323379.5 |
  324037.0 |
  324694.4 |
  325351.9 |
  326009.3 |
  326666.8 |
  327324.3 |
  327981.7 |
  328639.2 |
  329296.6 |
  329954.1 |########################################
  330611.5 |
  331269.0 |
  331926.5 |
  332583.9 |########################################
  333241.4 |
  333898.8 |
  334556.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **ab_match_tree**: autocorrelation=0.56 (measurement drift or warm-up artifact)

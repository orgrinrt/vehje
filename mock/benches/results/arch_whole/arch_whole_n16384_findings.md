# Whole-program single strategy (all branches), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_whole_table**

## Highlights

Baseline for all deltas below: **ab_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_whole_pred is an outlier: 4.0x slower than the field

ab_whole_pred (20.92 ms) is 4.0x the fastest (5.20 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_whole_table, ab_whole_tree) are a dead heat (<1%)

ab_whole_table (5.20 ms) and ab_whole_tree (5.24 ms) differ by 0.69%, inside the noise, even though the wider field spreads 302.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ab_whole_table)

The baseline ab_whole_table is the fastest (5.20 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_whole_table, ab_whole_tree, ab_whole_seq, ab_whole_prof} vs {ab_whole_pred} (289% apart)

The field splits into a fast tier {ab_whole_table, ab_whole_tree, ab_whole_seq, ab_whole_prof} and a slow tier {ab_whole_pred} with a 289% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest ab_whole_table (5.20 ms) to slowest ab_whole_pred (20.92 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ab_whole_table) is the fastest** at 5202258.2 ns median
- 3 variants significantly slower than baseline
- Spread: 4.02x (fastest 5202258.2 ns, slowest 20922242.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_whole_pred | 20946535ns | 20927088ns | 20892281ns | 20924843ns | 21006200ns | +302.43% |
| ab_whole_prof | 5398175ns | 5382049ns | 5352160ns | 5375548ns | 5455124ns | +3.71% |
| ab_whole_seq | 5320374ns | 5313267ns | 5284011ns | 5306281ns | 5359695ns | +2.22% |
| ab_whole_table | 5205046ns | 5205914ns | 5164624ns | 5196945ns | 5237408ns | base |
| ab_whole_tree | 5257595ns | 5241510ns | 5214639ns | 5237366ns | 5309416ns | +1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_whole_pred | 20941351ns | 20886928ns | 21001097ns | +302.61% | 0.001 |
| ab_whole_prof | 5394489ns | 5348352ns | 5451229ns | +3.71% | 0.003 |
| ab_whole_seq | 5316459ns | 5280030ns | 5356110ns | +2.21% | 0.003 |
| ab_whole_table | 5201397ns | 5160927ns | 5233726ns | base | 0.003 |
| ab_whole_tree | 5253846ns | 5210741ns | 5305335ns | +1.01% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_whole_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_whole_pred | 0.001 | 24.7% |
| ab_whole_prof | 0.003 | 96.0% |
| ab_whole_seq | 0.003 | 97.2% |
| ab_whole_table | 0.003 | 99.2% |
| ab_whole_tree | 0.003 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_whole_pred | 20946535ns | 20946535ns | +302.43% |
| ab_whole_prof | 5398175ns | 5398175ns | +3.71% |
| ab_whole_seq | 5320374ns | 5320374ns | +2.22% |
| ab_whole_table | 5205046ns | 5205046ns | base |
| ab_whole_tree | 5257595ns | 5257595ns | +1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_whole_table | 5202258ns | base | --- | [5168208, 5233726] | --- | --- | --- | --- |
| ab_whole_pred | 20922243ns | +15731769.2ns (+302.4%) | [+15667885, +15820207]ns | [20900713, 21001097] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_prof | 5378514ns | +184402.7ns (+3.5%) | [+142442, +252430]ns | [5353724, 5451229] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_seq | 5309164ns | +96197.9ns (+1.8%) | [+69242, +179745]ns | [5284102, 5356110] | YES | 0.0417 | 0.0313 | 0 |
| ab_whole_tree | 5238001ns | no significant difference | [-4536, +120165]ns | [5218201, 5305335] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_whole_table | ab_whole_pred | ab_whole_prof | ab_whole_seq | ab_whole_tree |
|---|---|---|---|---|---|
| 1 | 5238562ns | +298.7% | +3.0% | +1.5% | -0.5% |
| 2 | 5191805ns | +303.6% | +3.5% | +3.6% | +0.7% |
| 3 | 5228889ns | +300.0% | +2.5% | +1.1% | +0.9% |
| 4 | 5212712ns | +301.2% | +5.7% | +1.7% | +0.4% |
| 5 | 5160927ns | +305.5% | +3.6% | +3.4% | +1.6% |
| 6 | 5175489ns | +306.7% | +4.0% | +2.0% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_whole_pred | -0.084 | ok |
| ab_whole_prof | -0.472 | moderate- |
| ab_whole_seq | -0.338 | moderate- |
| ab_whole_table | 0.060 | ok |
| ab_whole_tree | -0.055 | ok |

**Consistency summary:**

- **ab_whole_pred**: won 0/6, lost 6/6
- **ab_whole_prof**: won 0/6, lost 6/6
- **ab_whole_seq**: won 0/6, lost 6/6
- **ab_whole_tree**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_whole_pred | 222.6ns | 20941351.0ns | 0.0% |  |
| ab_whole_prof | 79.5ns | 5394489.0ns | 0.0% |  |
| ab_whole_seq | 67.8ns | 5316458.8ns | 0.0% |  |
| ab_whole_table | 74.4ns | 5201397.2ns | 0.0% |  |
| ab_whole_tree | 80.3ns | 5253845.9ns | 0.0% |  |

## Distribution (algo ns)

```
ab_whole_pred (n=6, range 20886928.3-21001097.1 ns)
  20886928.3 |########################################
  20892636.7 |
  20898345.2 |
  20904053.6 |
  20909762.1 |########################################
  20915470.5 |########################################
  20921178.9 |
  20926887.4 |########################################
  20932595.8 |
  20938304.3 |
  20944012.7 |
  20949721.1 |########################################
  20955429.6 |
  20961138.0 |
  20966846.5 |
  20972554.9 |
  20978263.3 |
  20983971.8 |
  20989680.2 |
  20995388.7 |
  (0 below, 1 above range)

ab_whole_prof (n=6, range 5348352.5-5451229.3 ns)
  5348352.5 |########################################
  5353496.3 |
  5358640.2 |########################################
  5363784.0 |
  5368927.9 |########################################
  5374071.7 |
  5379215.6 |########################################
  5384359.4 |
  5389503.2 |########################################
  5394647.1 |
  5399790.9 |
  5404934.8 |
  5410078.6 |
  5415222.5 |
  5420366.3 |
  5425510.1 |
  5430654.0 |
  5435797.8 |
  5440941.7 |
  5446085.5 |
  (0 below, 1 above range)

ab_whole_seq (n=6, range 5280030.0-5356110.2 ns)
  5280030.0 |########################################
  5283834.0 |
  5287638.0 |########################################
  5291442.0 |
  5295246.0 |
  5299050.0 |########################################
  5302854.1 |
  5306658.1 |
  5310462.1 |
  5314266.1 |########################################
  5318070.1 |
  5321874.1 |
  5325678.1 |
  5329482.1 |
  5333286.1 |########################################
  5337090.1 |
  5340894.2 |
  5344698.2 |
  5348502.2 |
  5352306.2 |
  (0 below, 1 above range)

ab_whole_table (n=6, range 5160926.7-5233725.7 ns)
  5160926.7 |########################################
  5164566.6 |
  5168206.6 |
  5171846.5 |
  5175486.5 |########################################
  5179126.4 |
  5182766.4 |
  5186406.3 |
  5190046.3 |########################################
  5193686.2 |
  5197326.2 |
  5200966.1 |
  5204606.1 |
  5208246.0 |
  5211886.0 |########################################
  5215525.9 |
  5219165.9 |
  5222805.8 |
  5226445.8 |########################################
  5230085.7 |
  (0 below, 1 above range)

ab_whole_tree (n=6, range 5210741.2-5305335.2 ns)
  5210741.2 |########################################
  5215470.9 |
  5220200.6 |
  5224930.3 |########################################
  5229660.0 |########################################
  5234389.7 |
  5239119.4 |
  5243849.1 |########################################
  5248578.8 |
  5253308.5 |
  5258038.2 |
  5262767.9 |
  5267497.6 |
  5272227.3 |
  5276957.0 |########################################
  5281686.7 |
  5286416.4 |
  5291146.1 |
  5295875.8 |
  5300605.5 |
  (0 below, 1 above range)

```

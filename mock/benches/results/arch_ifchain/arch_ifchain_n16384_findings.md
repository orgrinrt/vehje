# Per-type strategy: all ifchain branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_ifchain_table**

## Highlights

Baseline for all deltas below: **ab_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_ifchain_pred is an outlier: 2.3x slower than the field

ab_ifchain_pred (11.64 ms) is 2.3x the fastest (5.15 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_ifchain_tree, ab_ifchain_table) are a dead heat (<1%)

ab_ifchain_tree (5.15 ms) and ab_ifchain_table (5.19 ms) differ by 0.67%, inside the noise, even though the wider field spreads 125.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_ifchain_tree shows alternating (throttle bounce) (autocorr -0.65)

ab_ifchain_tree's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_ifchain_tree, ab_ifchain_table, ab_ifchain_seq, ab_ifchain_prof} vs {ab_ifchain_pred} (119% apart)

The field splits into a fast tier {ab_ifchain_tree, ab_ifchain_table, ab_ifchain_seq, ab_ifchain_prof} and a slow tier {ab_ifchain_pred} with a 119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_ifchain_tree** at 5154660.7 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.26x (fastest 5154660.7 ns, slowest 11642312.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_ifchain_pred | 11640587ns | 11647018ns | 11570215ns | 11639341ns | 11677642ns | +124.12% |
| ab_ifchain_prof | 5327232ns | 5330573ns | 5295138ns | 5320227ns | 5353786ns | +2.56% |
| ab_ifchain_seq | 5317624ns | 5302876ns | 5277430ns | 5297584ns | 5367780ns | +2.38% |
| ab_ifchain_table | 5194009ns | 5192863ns | 5158712ns | 5188349ns | 5220148ns | base |
| ab_ifchain_tree | 5152443ns | 5158595ns | 5104382ns | 5151485ns | 5177912ns | -0.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_ifchain_pred | 11635947ns | 11565828ns | 11672844ns | +124.18% | 0.001 |
| ab_ifchain_prof | 5323504ns | 5291205ns | 5350145ns | +2.57% | 0.003 |
| ab_ifchain_seq | 5313745ns | 5273282ns | 5364020ns | +2.38% | 0.003 |
| ab_ifchain_table | 5190364ns | 5154779ns | 5216486ns | base | 0.003 |
| ab_ifchain_tree | 5148588ns | 5100175ns | 5174301ns | -0.80% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_ifchain_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_ifchain_pred | 0.001 | 43.8% |
| ab_ifchain_prof | 0.003 | 95.7% |
| ab_ifchain_seq | 0.003 | 96.2% |
| ab_ifchain_table | 0.003 | 98.3% |
| ab_ifchain_tree | 0.003 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_ifchain_pred | 11640587ns | 11640587ns | +124.12% |
| ab_ifchain_prof | 5327232ns | 5327232ns | +2.56% |
| ab_ifchain_seq | 5317624ns | 5317624ns | +2.38% |
| ab_ifchain_table | 5194009ns | 5194009ns | base |
| ab_ifchain_tree | 5152443ns | 5152443ns | -0.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_ifchain_table | 5189422ns | base | --- | [5165184, 5216486] | --- | --- | --- | --- |
| ab_ifchain_pred | 11642313ns | +6453221.7ns (+124.4%) | [+6403087, +6480440]ns | [11592683, 11672844] | YES | 0.0417 | 0.0313 | 0 |
| ab_ifchain_prof | 5326901ns | +133658.7ns (+2.6%) | [+104044, +161717]ns | [5293465, 5350145] | YES | 0.0417 | 0.0313 | 0 |
| ab_ifchain_seq | 5299019ns | +116168.6ns (+2.2%) | [+73196, +180780]ns | [5278197, 5364020] | YES | 0.0417 | 0.0313 | 0 |
| ab_ifchain_tree | 5154661ns | -44288.5ns (-0.9%) | [-80970, -68]ns | [5116803, 5174301] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_ifchain_table | ab_ifchain_pred | ab_ifchain_prof | ab_ifchain_seq | ab_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 5193254ns | +124.1% | +2.0% | +4.3% | -0.9% |
| 2 | 5224412ns | +122.4% | +2.3% | +1.1% | -1.0% |
| 3 | 5175588ns | +125.0% | +3.2% | +2.7% | -0.8% |
| 4 | 5154779ns | +124.4% | +3.1% | +2.7% | +0.1% |
| 5 | 5208560ns | +124.0% | +2.8% | +1.8% | -2.1% |
| 6 | 5185590ns | +125.2% | +2.0% | +1.7% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_ifchain_pred | -0.232 | moderate- |
| ab_ifchain_prof | -0.462 | moderate- |
| ab_ifchain_seq | -0.185 | ok |
| ab_ifchain_table | -0.204 | moderate- |
| ab_ifchain_tree | -0.649 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ab_ifchain_pred**: won 0/6, lost 6/6
- **ab_ifchain_prof**: won 0/6, lost 6/6
- **ab_ifchain_seq**: won 0/6, lost 6/6
- **ab_ifchain_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_ifchain_pred | 160.8ns | 11635946.7ns | 0.0% |  |
| ab_ifchain_prof | 72.4ns | 5323503.7ns | 0.0% |  |
| ab_ifchain_seq | 68.4ns | 5313745.1ns | 0.0% |  |
| ab_ifchain_table | 74.2ns | 5190363.8ns | 0.0% |  |
| ab_ifchain_tree | 65.0ns | 5148588.3ns | 0.0% |  |

## Distribution (algo ns)

```
ab_ifchain_pred (n=6, range 11565828.3-11672844.2 ns)
  11565828.3 |########################################
  11571179.1 |
  11576529.9 |
  11581880.7 |
  11587231.5 |
  11592582.3 |
  11597933.1 |
  11603283.8 |
  11608634.6 |
  11613985.4 |
  11619336.2 |########################################
  11624687.0 |
  11630037.8 |
  11635388.6 |########################################
  11640739.4 |########################################
  11646090.2 |
  11651441.0 |
  11656791.8 |
  11662142.6 |
  11667493.4 |########################################
  (0 below, 1 above range)

ab_ifchain_prof (n=6, range 5291204.6-5350144.8 ns)
  5291204.6 |########################################
  5294151.6 |########################################
  5297098.6 |
  5300045.6 |
  5302992.6 |
  5305939.6 |
  5308886.6 |
  5311833.7 |########################################
  5314780.7 |
  5317727.7 |
  5320674.7 |
  5323621.7 |
  5326568.7 |
  5329515.7 |
  5332462.7 |
  5335409.7 |
  5338356.7 |########################################
  5341303.7 |
  5344250.7 |########################################
  5347197.7 |
  (0 below, 1 above range)

ab_ifchain_seq (n=6, range 5273282.5-5364019.8 ns)
  5273282.5 |########################################
  5277819.4 |
  5282356.2 |########################################
  5286893.1 |
  5291430.0 |########################################
  5295966.8 |
  5300503.7 |########################################
  5305040.6 |
  5309577.4 |########################################
  5314114.3 |
  5318651.2 |
  5323188.0 |
  5327724.9 |
  5332261.7 |
  5336798.6 |
  5341335.5 |
  5345872.3 |
  5350409.2 |
  5354946.1 |
  5359482.9 |
  (0 below, 1 above range)

ab_ifchain_table (n=6, range 5154778.8-5216486.0 ns)
  5154778.8 |########################################
  5157864.2 |
  5160949.5 |
  5164034.9 |
  5167120.2 |
  5170205.6 |
  5173291.0 |########################################
  5176376.3 |
  5179461.7 |
  5182547.1 |########################################
  5185632.4 |
  5188717.8 |
  5191803.1 |########################################
  5194888.5 |
  5197973.9 |
  5201059.2 |
  5204144.6 |
  5207230.0 |########################################
  5210315.3 |
  5213400.7 |
  (0 below, 1 above range)

ab_ifchain_tree (n=6, range 5100175.0-5174301.5 ns)
  5100175.0 |########################################
  5103881.3 |
  5107587.6 |
  5111294.0 |
  5115000.3 |
  5118706.6 |
  5122412.9 |
  5126119.3 |
  5129825.6 |########################################
  5133531.9 |
  5137238.2 |
  5140944.5 |
  5144650.9 |########################################
  5148357.2 |
  5152063.5 |
  5155769.8 |
  5159476.2 |########################################
  5163182.5 |
  5166888.8 |
  5170595.1 |########################################
  (0 below, 1 above range)

```

# Per-branch strategy (NATIVE tier): archetype 6

5 variants, 6 samples per variant.
Baseline: **an_b6_table**

## Highlights

Baseline for all deltas below: **an_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b6_tree, an_b6_table) are a dead heat (<1%)

an_b6_tree (331.89 us) and an_b6_table (332.27 us) differ by 0.12%, inside the noise, even though the wider field spreads 30.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: an_b6_tree** at 331890.2 ns median (-0.1% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.30x (fastest 331890.2 ns, slowest 431381.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b6_pred | 434114ns | 433764ns | 430002ns | 433553ns | 437013ns | +30.41% |
| an_b6_prof | 348416ns | 349181ns | 338223ns | 346894ns | 355797ns | +4.66% |
| an_b6_seq | 345861ns | 345210ns | 340826ns | 344095ns | 351028ns | +3.90% |
| an_b6_table | 332889ns | 334807ns | 315346ns | 332926ns | 341604ns | base |
| an_b6_tree | 334051ns | 334505ns | 327481ns | 334327ns | 336921ns | +0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b6_pred | 431549ns | 427228ns | 434371ns | +30.62% | 0.038 |
| an_b6_prof | 345827ns | 335638ns | 353125ns | +4.67% | 0.047 |
| an_b6_seq | 343315ns | 338523ns | 348331ns | +3.91% | 0.048 |
| an_b6_table | 330387ns | 313208ns | 338848ns | base | 0.050 |
| an_b6_tree | 331360ns | 325114ns | 334082ns | +0.29% | 0.049 |

## Performance model

- Peak throughput: **0.052 Gops/s** (an_b6_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b6_pred | 0.038 | 72.6% |
| an_b6_prof | 0.047 | 90.3% |
| an_b6_seq | 0.048 | 91.4% |
| an_b6_table | 0.049 | 94.3% |
| an_b6_tree | 0.049 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b6_pred | 434114ns | 434114ns | +30.41% |
| an_b6_prof | 348416ns | 348416ns | +4.66% |
| an_b6_seq | 345861ns | 345861ns | +3.90% |
| an_b6_table | 332889ns | 332889ns | base |
| an_b6_tree | 334051ns | 334051ns | +0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b6_table | 332272ns | base | --- | [320041, 338848] | --- | --- | --- | --- |
| an_b6_pred | 431381ns | +98601.5ns (+29.7%) | [+91417, +113466]ns | [428894, 434371] | YES | 0.0417 | 0.0313 | 0 |
| an_b6_prof | 346711ns | +15682.1ns (+4.7%) | [+6070, +24566]ns | [337644, 353125] | YES | 0.0417 | 0.0313 | 0 |
| an_b6_seq | 342701ns | +13349.2ns (+4.0%) | [+4206, +21230]ns | [338914, 348331] | YES | 0.0417 | 0.0313 | 0 |
| an_b6_tree | 331890ns | no significant difference | [-6958, +11230]ns | [328108, 334082] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b6_table | an_b6_pred | an_b6_prof | an_b6_seq | an_b6_tree |
|---|---|---|---|---|---|
| 1 | 330065ns | +30.4% | +1.7% | +5.0% | +1.4% |
| 2 | 313208ns | +39.0% | +8.4% | +8.3% | +5.7% |
| 3 | 326874ns | +32.0% | +6.2% | +3.6% | -0.5% |
| 4 | 335185ns | +27.5% | +3.3% | +4.5% | -1.0% |
| 5 | 342511ns | +26.5% | +1.9% | +0.1% | -3.1% |
| 6 | 334480ns | +28.9% | +6.8% | +2.4% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b6_pred | -0.318 | moderate- |
| an_b6_prof | 0.333 | moderate+ |
| an_b6_seq | -0.302 | moderate- |
| an_b6_table | 0.317 | moderate+ |
| an_b6_tree | -0.010 | ok |

**Consistency summary:**

- **an_b6_pred**: won 0/6, lost 6/6
- **an_b6_prof**: won 0/6, lost 6/6
- **an_b6_seq**: won 0/6, lost 5/6
- **an_b6_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b6_pred | 8.4ns | 431548.6ns | 0.0% |  |
| an_b6_prof | 15.7ns | 345826.5ns | 0.0% |  |
| an_b6_seq | 15.2ns | 343315.1ns | 0.0% |  |
| an_b6_table | 11.2ns | 330387.0ns | 0.0% |  |
| an_b6_tree | 8.3ns | 331359.9ns | 0.0% |  |

## Distribution (algo ns)

```
an_b6_pred (n=6, range 427227.5-434370.7 ns)
  427227.5 |########################################
  427584.7 |
  427941.8 |
  428299.0 |
  428656.1 |
  429013.3 |
  429370.4 |
  429727.6 |
  430084.8 |
  430441.9 |########################################
  430799.1 |
  431156.2 |########################################
  431513.4 |########################################
  431870.5 |
  432227.7 |
  432584.9 |
  432942.0 |
  433299.2 |########################################
  433656.3 |
  434013.5 |
  (0 below, 1 above range)

an_b6_prof (n=6, range 335638.3-353124.6 ns)
  335638.3 |########################################
  336512.6 |
  337386.9 |
  338261.2 |
  339135.6 |########################################
  340009.9 |
  340884.2 |
  341758.5 |
  342632.8 |
  343507.1 |
  344381.4 |
  345255.8 |
  346130.1 |########################################
  347004.4 |########################################
  347878.7 |
  348753.0 |########################################
  349627.3 |
  350501.7 |
  351376.0 |
  352250.3 |
  (0 below, 1 above range)

an_b6_seq (n=6, range 338522.9-348330.8 ns)
  338522.9 |####################
  339013.3 |####################
  339503.7 |
  339994.1 |
  340484.5 |
  340974.9 |
  341465.3 |
  341955.7 |
  342446.1 |########################################
  342936.5 |
  343426.9 |
  343917.3 |
  344407.7 |
  344898.1 |
  345388.5 |
  345878.9 |
  346369.3 |####################
  346859.7 |
  347350.1 |
  347840.5 |
  (0 below, 1 above range)

an_b6_table (n=6, range 313208.3-338847.7 ns)
  313208.3 |########################################
  314490.3 |
  315772.2 |
  317054.2 |
  318336.2 |
  319618.1 |
  320900.1 |
  322182.1 |
  323464.1 |
  324746.0 |
  326028.0 |########################################
  327310.0 |
  328591.9 |
  329873.9 |########################################
  331155.9 |
  332437.8 |
  333719.8 |########################################
  335001.8 |########################################
  336283.8 |
  337565.7 |
  (0 below, 1 above range)

an_b6_tree (n=6, range 325114.2-334081.7 ns)
  325114.2 |########################################
  325562.6 |
  326010.9 |
  326459.3 |
  326907.7 |
  327356.1 |
  327804.4 |
  328252.8 |
  328701.2 |
  329149.6 |
  329597.9 |
  330046.3 |
  330494.7 |
  330943.0 |########################################
  331391.4 |########################################
  331839.8 |########################################
  332288.2 |
  332736.5 |
  333184.9 |########################################
  333633.3 |
  (0 below, 1 above range)

```

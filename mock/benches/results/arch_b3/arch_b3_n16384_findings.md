# Per-branch strategy: archetype 3 (match4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b3_table**

## Highlights

Baseline for all deltas below: **ab_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b3_tree, ab_b3_table) are a dead heat (<1%)

ab_b3_tree (5.19 ms) and ab_b3_table (5.20 ms) differ by 0.24%, inside the noise, even though the wider field spreads 90.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ab_b3_tree, ab_b3_table, ab_b3_prof, ab_b3_seq} vs {ab_b3_pred} (89% apart)

The field splits into a fast tier {ab_b3_tree, ab_b3_table, ab_b3_prof, ab_b3_seq} and a slow tier {ab_b3_pred} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_b3_tree** at 5190702.1 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 5190702.1 ns, slowest 9896335.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b3_pred | 9895311ns | 9900289ns | 9852519ns | 9898299ns | 9912226ns | +89.97% |
| ab_b3_prof | 5217529ns | 5215483ns | 5183848ns | 5205769ns | 5252011ns | +0.17% |
| ab_b3_seq | 5233132ns | 5238907ns | 5202610ns | 5229075ns | 5254480ns | +0.47% |
| ab_b3_table | 5208762ns | 5206963ns | 5157309ns | 5197833ns | 5250882ns | base |
| ab_b3_tree | 5195130ns | 5194960ns | 5182725ns | 5192414ns | 5205405ns | -0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b3_pred | 9891270ns | 9848419ns | 9908142ns | +90.03% | 0.002 |
| ab_b3_prof | 5213953ns | 5180592ns | 5248320ns | +0.17% | 0.003 |
| ab_b3_seq | 5229400ns | 5198986ns | 5251005ns | +0.47% | 0.003 |
| ab_b3_table | 5205080ns | 5153860ns | 5247099ns | base | 0.003 |
| ab_b3_tree | 5191094ns | 5179110ns | 5201311ns | -0.27% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b3_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b3_pred | 0.002 | 52.1% |
| ab_b3_prof | 0.003 | 98.9% |
| ab_b3_seq | 0.003 | 98.4% |
| ab_b3_table | 0.003 | 99.1% |
| ab_b3_tree | 0.003 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b3_pred | 9895311ns | 9895311ns | +89.97% |
| ab_b3_prof | 5217529ns | 5217529ns | +0.17% |
| ab_b3_seq | 5233132ns | 5233132ns | +0.47% |
| ab_b3_table | 5208762ns | 5208762ns | base |
| ab_b3_tree | 5195130ns | 5195130ns | -0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b3_table | 5202954ns | base | --- | [5165185, 5247099] | --- | --- | --- | --- |
| ab_b3_pred | 9896335ns | +4692399.6ns (+90.2%) | [+4625212, +4740960]ns | [9869333, 9908142] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b3_prof | 5211884ns | no significant difference | [-37584, +62251]ns | [5181655, 5248320] | no | 1.0000 | 1.0000 | 0 |
| ab_b3_seq | 5235132ns | no significant difference | [-30843, +79270]ns | [5202062, 5251005] | no | 1.0000 | 0.6875 | 0 |
| ab_b3_tree | 5190702ns | no significant difference | [-56611, +23152]ns | [5181268, 5201311] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b3_table | ab_b3_pred | ab_b3_prof | ab_b3_seq | ab_b3_tree |
|---|---|---|---|---|---|
| 1 | 5180401ns | +91.4% | +0.7% | +1.2% | +0.3% |
| 2 | 5153860ns | +92.1% | +0.6% | +1.9% | +0.6% |
| 3 | 5231463ns | +89.1% | -0.5% | -0.1% | -0.5% |
| 4 | 5262735ns | +87.9% | -0.6% | -1.1% | -1.3% |
| 5 | 5176511ns | +91.3% | +1.7% | +0.4% | +0.2% |
| 6 | 5225507ns | +88.5% | -0.9% | +0.5% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b3_pred | -0.088 | ok |
| ab_b3_prof | -0.150 | ok |
| ab_b3_seq | 0.126 | ok |
| ab_b3_table | -0.094 | ok |
| ab_b3_tree | -0.094 | ok |

**Consistency summary:**

- **ab_b3_pred**: won 0/6, lost 6/6
- **ab_b3_prof**: won 3/6, lost 3/6
- **ab_b3_seq**: won 1/6, lost 4/6
- **ab_b3_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b3_pred | 122.3ns | 9891270.0ns | 0.0% |  |
| ab_b3_prof | 71.2ns | 5213953.2ns | 0.0% |  |
| ab_b3_seq | 81.3ns | 5229399.8ns | 0.0% |  |
| ab_b3_table | 72.0ns | 5205079.5ns | 0.0% |  |
| ab_b3_tree | 69.0ns | 5191093.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b3_pred (n=6, range 9848418.7-9908142.4 ns)
  9848418.7 |####################
  9851404.9 |
  9854391.1 |
  9857377.3 |
  9860363.4 |
  9863349.6 |
  9866335.8 |
  9869322.0 |
  9872308.2 |
  9875294.4 |
  9878280.6 |
  9881266.8 |
  9884252.9 |
  9887239.1 |
  9890225.3 |########################################
  9893211.5 |
  9896197.7 |
  9899183.9 |########################################
  9902170.1 |
  9905156.3 |
  (0 below, 1 above range)

ab_b3_prof (n=6, range 5180591.7-5248320.4 ns)
  5180591.7 |########################################
  5183978.1 |
  5187364.6 |
  5190751.0 |
  5194137.4 |
  5197523.9 |
  5200910.3 |
  5204296.7 |####################
  5207683.2 |
  5211069.6 |
  5214456.1 |####################
  5217842.5 |
  5221228.9 |
  5224615.4 |
  5228001.8 |
  5231388.2 |####################
  5234774.7 |
  5238161.1 |
  5241547.5 |
  5244934.0 |
  (0 below, 1 above range)

ab_b3_seq (n=6, range 5198985.8-5251005.2 ns)
  5198985.8 |########################################
  5201586.8 |
  5204187.7 |########################################
  5206788.7 |
  5209389.7 |
  5211990.7 |
  5214591.6 |
  5217192.6 |
  5219793.6 |
  5222394.5 |
  5224995.5 |########################################
  5227596.5 |
  5230197.4 |
  5232798.4 |
  5235399.4 |
  5238000.3 |
  5240601.3 |########################################
  5243202.3 |
  5245803.3 |
  5248404.2 |########################################
  (0 below, 1 above range)

ab_b3_table (n=6, range 5153860.0-5247099.3 ns)
  5153860.0 |########################################
  5158522.0 |
  5163183.9 |
  5167845.9 |
  5172507.9 |########################################
  5177169.8 |########################################
  5181831.8 |
  5186493.8 |
  5191155.7 |
  5195817.7 |
  5200479.7 |
  5205141.6 |
  5209803.6 |
  5214465.6 |
  5219127.5 |
  5223789.5 |########################################
  5228451.5 |########################################
  5233113.4 |
  5237775.4 |
  5242437.4 |
  (0 below, 1 above range)

ab_b3_tree (n=6, range 5179110.4-5201310.8 ns)
  5179110.4 |########################################
  5180220.4 |
  5181330.4 |
  5182440.5 |########################################
  5183550.5 |
  5184660.5 |########################################
  5185770.5 |
  5186880.6 |
  5187990.6 |
  5189100.6 |
  5190210.6 |
  5191320.6 |
  5192430.7 |
  5193540.7 |
  5194650.7 |
  5195760.7 |########################################
  5196870.8 |########################################
  5197980.8 |
  5199090.8 |
  5200200.8 |
  (0 below, 1 above range)

```

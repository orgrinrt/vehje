# Per-branch strategy: archetype 1 (match4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b1_table**

## Highlights

Baseline for all deltas below: **ab_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b1_table, ab_b1_tree) are a dead heat (<1%)

ab_b1_table (5.18 ms) and ab_b1_tree (5.20 ms) differ by 0.47%, inside the noise, even though the wider field spreads 19.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b1_table shows alternating (throttle bounce) (autocorr -0.75)

ab_b1_table's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b1_table)

The baseline ab_b1_table is the fastest (5.18 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b1_table) is the fastest** at 5179306.3 ns median
- 2 variants significantly slower than baseline
- Spread: 1.20x (fastest 5179306.3 ns, slowest 6201329.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b1_pred | 6216772ns | 6205196ns | 6194589ns | 6204487ns | 6246290ns | +19.94% |
| ab_b1_prof | 5210985ns | 5215018ns | 5178820ns | 5203261ns | 5238655ns | +0.53% |
| ab_b1_seq | 5228116ns | 5219393ns | 5187120ns | 5218131ns | 5263591ns | +0.86% |
| ab_b1_table | 5183424ns | 5183105ns | 5163637ns | 5178176ns | 5201190ns | base |
| ab_b1_tree | 5237594ns | 5207345ns | 5160532ns | 5202646ns | 5328546ns | +1.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b1_pred | 6212979ns | 6190615ns | 6242642ns | +19.95% | 0.003 |
| ab_b1_prof | 5207379ns | 5175099ns | 5235349ns | +0.54% | 0.003 |
| ab_b1_seq | 5224313ns | 5182999ns | 5259659ns | +0.86% | 0.003 |
| ab_b1_table | 5179644ns | 5159670ns | 5197385ns | base | 0.003 |
| ab_b1_tree | 5233813ns | 5156958ns | 5324695ns | +1.05% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b1_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b1_pred | 0.003 | 83.2% |
| ab_b1_prof | 0.003 | 99.0% |
| ab_b1_seq | 0.003 | 98.9% |
| ab_b1_table | 0.003 | 99.6% |
| ab_b1_tree | 0.003 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b1_pred | 6216772ns | 6216772ns | +19.94% |
| ab_b1_prof | 5210985ns | 5210985ns | +0.53% |
| ab_b1_seq | 5228116ns | 5228116ns | +0.86% |
| ab_b1_table | 5183424ns | 5183424ns | base |
| ab_b1_tree | 5237594ns | 5237594ns | +1.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b1_table | 5179306ns | base | --- | [5162241, 5197385] | --- | --- | --- | --- |
| ab_b1_pred | 6201330ns | +1016871.6ns (+19.6%) | [+1002732, +1080401]ns | [6194965, 6242642] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b1_prof | 5211022ns | no significant difference | [-6708, +63118]ns | [5175765, 5235349] | no | 0.6875 | 0.6875 | 0 |
| ab_b1_seq | 5215650ns | +43103.5ns (+0.8%) | [+3392, +87512]ns | [5197631, 5259659] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| ab_b1_tree | 5203598ns | no significant difference | [-20165, +147149]ns | [5173147, 5324695] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b1_table | ab_b1_pred | ab_b1_prof | ab_b1_seq | ab_b1_tree |
|---|---|---|---|---|---|
| 1 | 5190279ns | +19.4% | +0.7% | -0.1% | +0.8% |
| 2 | 5173989ns | +19.6% | +0.9% | +0.7% | +0.8% |
| 3 | 5184624ns | +19.6% | -0.2% | +1.8% | -0.5% |
| 4 | 5159670ns | +21.0% | +0.3% | +1.6% | +0.6% |
| 5 | 5204490ns | +19.1% | -0.1% | +0.3% | -0.2% |
| 6 | 5164812ns | +20.8% | +1.5% | +0.9% | +4.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b1_pred | -0.191 | ok |
| ab_b1_prof | 0.208 | moderate+ |
| ab_b1_seq | 0.154 | ok |
| ab_b1_table | -0.749 | HIGH- (thermal bounce) |
| ab_b1_tree | -0.022 | ok |

**Consistency summary:**

- **ab_b1_pred**: won 0/6, lost 6/6
- **ab_b1_prof**: won 2/6, lost 4/6
- **ab_b1_seq**: won 1/6, lost 5/6
- **ab_b1_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b1_pred | 78.2ns | 6212979.1ns | 0.0% |  |
| ab_b1_prof | 55.8ns | 5207378.8ns | 0.0% |  |
| ab_b1_seq | 61.6ns | 5224313.0ns | 0.0% |  |
| ab_b1_table | 65.3ns | 5179644.0ns | 0.0% |  |
| ab_b1_tree | 65.9ns | 5233813.1ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b1_pred (n=6, range 6190615.0-6242642.5 ns)
  6190615.0 |####################
  6193216.4 |
  6195817.8 |
  6198419.1 |########################################
  6201020.5 |####################
  6203621.9 |
  6206223.2 |
  6208824.6 |
  6211426.0 |
  6214027.4 |
  6216628.8 |
  6219230.1 |
  6221831.5 |
  6224432.9 |
  6227034.2 |
  6229635.6 |
  6232237.0 |
  6234838.4 |
  6237439.8 |####################
  6240041.1 |
  (0 below, 1 above range)

ab_b1_prof (n=6, range 5175098.7-5235349.0 ns)
  5175098.7 |########################################
  5178111.2 |
  5181123.7 |
  5184136.2 |
  5187148.8 |
  5190161.3 |
  5193173.8 |
  5196186.3 |
  5199198.8 |####################
  5202211.3 |
  5205223.8 |
  5208236.4 |
  5211248.9 |
  5214261.4 |
  5217273.9 |
  5220286.4 |####################
  5223298.9 |
  5226311.5 |####################
  5229324.0 |
  5232336.5 |
  (0 below, 1 above range)

ab_b1_seq (n=6, range 5182998.8-5259658.6 ns)
  5182998.8 |####################
  5186831.8 |
  5190664.8 |
  5194497.8 |
  5198330.8 |
  5202163.7 |
  5205996.7 |
  5209829.7 |########################################
  5213662.7 |
  5217495.7 |####################
  5221328.7 |
  5225161.7 |
  5228994.7 |
  5232827.6 |
  5236660.6 |
  5240493.6 |####################
  5244326.6 |
  5248159.6 |
  5251992.6 |
  5255825.6 |
  (0 below, 1 above range)

ab_b1_table (n=6, range 5159669.6-5197384.6 ns)
  5159669.6 |########################################
  5161555.3 |
  5163441.1 |########################################
  5165326.8 |
  5167212.6 |
  5169098.3 |
  5170984.1 |
  5172869.8 |########################################
  5174755.6 |
  5176641.3 |
  5178527.1 |
  5180412.8 |
  5182298.6 |
  5184184.3 |########################################
  5186070.1 |
  5187955.8 |
  5189841.6 |########################################
  5191727.3 |
  5193613.1 |
  5195498.8 |
  (0 below, 1 above range)

ab_b1_tree (n=6, range 5156957.5-5324694.8 ns)
  5156957.5 |########################################
  5165344.4 |
  5173731.2 |
  5182118.1 |########################################
  5190505.0 |########################################
  5198891.8 |
  5207278.7 |########################################
  5215665.6 |
  5224052.4 |
  5232439.3 |########################################
  5240826.2 |
  5249213.0 |
  5257599.9 |
  5265986.7 |
  5274373.6 |
  5282760.5 |
  5291147.3 |
  5299534.2 |
  5307921.1 |
  5316307.9 |
  (0 below, 1 above range)

```

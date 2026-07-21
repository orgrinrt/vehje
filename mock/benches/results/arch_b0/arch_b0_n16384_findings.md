# Per-branch strategy: archetype 0 (match4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b0_table**

## Highlights

Baseline for all deltas below: **ab_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b0_table, ab_b0_tree) are a dead heat (<1%)

ab_b0_table (5.16 ms) and ab_b0_tree (5.18 ms) differ by 0.32%, inside the noise, even though the wider field spreads 20.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ab_b0_table)

The baseline ab_b0_table is the fastest (5.16 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b0_table) is the fastest** at 5160409.5 ns median
- 3 variants significantly slower than baseline
- Spread: 1.20x (fastest 5160409.5 ns, slowest 6212000.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b0_pred | 6213054ns | 6215765ns | 6152775ns | 6211534ns | 6245475ns | +20.29% |
| ab_b0_prof | 5214753ns | 5221258ns | 5163588ns | 5203240ns | 5257606ns | +0.96% |
| ab_b0_seq | 5213077ns | 5212609ns | 5151897ns | 5210517ns | 5247506ns | +0.93% |
| ab_b0_table | 5165240ns | 5164043ns | 5147165ns | 5160164ns | 5181891ns | base |
| ab_b0_tree | 5190402ns | 5180477ns | 5161443ns | 5176107ns | 5226324ns | +0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b0_pred | 6209372ns | 6149116ns | 6241771ns | +20.30% | 0.003 |
| ab_b0_prof | 5211170ns | 5159872ns | 5254063ns | +0.96% | 0.003 |
| ab_b0_seq | 5209473ns | 5148013ns | 5243858ns | +0.93% | 0.003 |
| ab_b0_table | 5161467ns | 5143396ns | 5178171ns | base | 0.003 |
| ab_b0_tree | 5186774ns | 5157573ns | 5222800ns | +0.49% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b0_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b0_pred | 0.003 | 82.8% |
| ab_b0_prof | 0.003 | 98.6% |
| ab_b0_seq | 0.003 | 98.7% |
| ab_b0_table | 0.003 | 99.7% |
| ab_b0_tree | 0.003 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b0_pred | 6213054ns | 6213054ns | +20.29% |
| ab_b0_prof | 5214753ns | 5214753ns | +0.96% |
| ab_b0_seq | 5213077ns | 5213077ns | +0.93% |
| ab_b0_table | 5165240ns | 5165240ns | base |
| ab_b0_tree | 5190402ns | 5190402ns | +0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b0_table | 5160410ns | base | --- | [5145822, 5178171] | --- | --- | --- | --- |
| ab_b0_pred | 6212000ns | +1058082.3ns (+20.5%) | [+1000929, +1084702]ns | [6174344, 6241771] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b0_prof | 5217685ns | +52299.2ns (+1.0%) | [+15939, +80869]ns | [5161760, 5254063] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b0_seq | 5209091ns | +59173.6ns (+1.1%) | [+2510, +82333]ns | [5175470, 5243858] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_b0_tree | 5176848ns | no significant difference | [-17498, +62391]ns | [5160673, 5222800] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b0_table | ab_b0_pred | ab_b0_prof | ab_b0_seq | ab_b0_tree |
|---|---|---|---|---|---|
| 1 | 5157481ns | +20.6% | +1.0% | -0.2% | +1.5% |
| 2 | 5163338ns | +20.2% | +1.3% | +1.8% | +0.9% |
| 3 | 5190450ns | +18.5% | +1.1% | +0.3% | -0.6% |
| 4 | 5148247ns | +21.4% | +0.2% | +1.1% | +0.8% |
| 5 | 5143396ns | +20.5% | +0.4% | +1.4% | +0.4% |
| 6 | 5165892ns | +20.6% | +1.9% | +1.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b0_pred | -0.493 | moderate- |
| ab_b0_prof | -0.138 | ok |
| ab_b0_seq | -0.473 | moderate- |
| ab_b0_table | -0.129 | ok |
| ab_b0_tree | 0.188 | ok |

**Consistency summary:**

- **ab_b0_pred**: won 0/6, lost 6/6
- **ab_b0_prof**: won 0/6, lost 6/6
- **ab_b0_seq**: won 1/6, lost 5/6
- **ab_b0_tree**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b0_pred | 63.6ns | 6209371.7ns | 0.0% |  |
| ab_b0_prof | 46.2ns | 5211169.7ns | 0.0% |  |
| ab_b0_seq | 46.9ns | 5209472.8ns | 0.0% |  |
| ab_b0_table | 55.1ns | 5161467.4ns | 0.0% |  |
| ab_b0_tree | 53.4ns | 5186773.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b0_pred (n=6, range 6149116.2-6241771.5 ns)
  6149116.2 |########################################
  6153749.0 |
  6158381.7 |
  6163014.5 |
  6167647.2 |
  6172280.0 |
  6176912.8 |
  6181545.5 |
  6186178.3 |
  6190811.1 |
  6195443.8 |########################################
  6200076.6 |
  6204709.4 |########################################
  6209342.1 |
  6213974.9 |########################################
  6218607.6 |
  6223240.4 |
  6227873.2 |
  6232505.9 |########################################
  6237138.7 |
  (0 below, 1 above range)

ab_b0_prof (n=6, range 5159872.5-5254063.3 ns)
  5159872.5 |########################################
  5164582.0 |
  5169291.6 |
  5174001.1 |
  5178710.7 |
  5183420.2 |
  5188129.8 |
  5192839.3 |
  5197548.8 |
  5202258.4 |
  5206967.9 |####################
  5211677.5 |
  5216387.0 |
  5221096.6 |
  5225806.1 |####################
  5230515.6 |
  5235225.2 |
  5239934.7 |
  5244644.3 |####################
  5249353.8 |
  (0 below, 1 above range)

ab_b0_seq (n=6, range 5148012.9-5243857.7 ns)
  5148012.9 |####################
  5152805.1 |
  5157597.4 |
  5162389.6 |
  5167181.9 |
  5171974.1 |
  5176766.3 |
  5181558.6 |
  5186350.8 |
  5191143.1 |
  5195935.3 |
  5200727.5 |########################################
  5205519.8 |
  5210312.0 |####################
  5215104.3 |
  5219896.5 |
  5224688.7 |
  5229481.0 |####################
  5234273.2 |
  5239065.5 |
  (0 below, 1 above range)

ab_b0_table (n=6, range 5143396.2-5178171.0 ns)
  5143396.2 |########################################
  5145134.9 |
  5146873.7 |########################################
  5148612.4 |
  5150351.2 |
  5152089.9 |
  5153828.7 |
  5155567.4 |
  5157306.1 |########################################
  5159044.9 |
  5160783.6 |
  5162522.4 |########################################
  5164261.1 |########################################
  5165999.9 |
  5167738.6 |
  5169477.3 |
  5171216.1 |
  5172954.8 |
  5174693.6 |
  5176432.3 |
  (0 below, 1 above range)

ab_b0_tree (n=6, range 5157572.9-5222800.2 ns)
  5157572.9 |########################################
  5160834.3 |########################################
  5164095.6 |########################################
  5167357.0 |
  5170618.4 |
  5173879.7 |
  5177141.1 |
  5180402.5 |
  5183663.8 |
  5186925.2 |########################################
  5190186.6 |
  5193447.9 |
  5196709.3 |
  5199970.6 |
  5203232.0 |
  5206493.4 |
  5209754.7 |########################################
  5213016.1 |
  5216277.5 |
  5219538.8 |
  (0 below, 1 above range)

```

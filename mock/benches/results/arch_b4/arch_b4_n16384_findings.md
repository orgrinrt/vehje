# Per-branch strategy: archetype 4 (ifchain4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b4_table**

## Highlights

Baseline for all deltas below: **ab_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b4_table, ab_b4_tree) are a dead heat (<1%)

ab_b4_table (5.18 ms) and ab_b4_tree (5.19 ms) differ by 0.11%, inside the noise, even though the wider field spreads 17.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b4_tree shows alternating (throttle bounce) (autocorr -0.51)

ab_b4_tree's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b4_table)

The baseline ab_b4_table is the fastest (5.18 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b4_table) is the fastest** at 5182580.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.18x (fastest 5182580.8 ns, slowest 6098157.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b4_pred | 6103133ns | 6102075ns | 6081860ns | 6098286ns | 6121039ns | +17.52% |
| ab_b4_prof | 5342698ns | 5333282ns | 5317992ns | 5329954ns | 5374166ns | +2.88% |
| ab_b4_seq | 5294306ns | 5295195ns | 5278877ns | 5291761ns | 5305839ns | +1.94% |
| ab_b4_table | 5193304ns | 5186625ns | 5178562ns | 5185283ns | 5212705ns | base |
| ab_b4_tree | 5198384ns | 5192339ns | 5166358ns | 5189259ns | 5228084ns | +0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b4_pred | 6099147ns | 6077816ns | 6116993ns | +17.53% | 0.003 |
| ab_b4_prof | 5338696ns | 5313945ns | 5370084ns | +2.88% | 0.003 |
| ab_b4_seq | 5290269ns | 5275107ns | 5301792ns | +1.94% | 0.003 |
| ab_b4_table | 5189340ns | 5175137ns | 5208579ns | base | 0.003 |
| ab_b4_tree | 5194620ns | 5162950ns | 5224432ns | +0.10% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b4_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b4_pred | 0.003 | 84.7% |
| ab_b4_prof | 0.003 | 96.9% |
| ab_b4_seq | 0.003 | 97.6% |
| ab_b4_table | 0.003 | 99.6% |
| ab_b4_tree | 0.003 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b4_pred | 6103133ns | 6103133ns | +17.52% |
| ab_b4_prof | 5342698ns | 5342698ns | +2.88% |
| ab_b4_seq | 5294306ns | 5294306ns | +1.94% |
| ab_b4_table | 5193304ns | 5193304ns | base |
| ab_b4_tree | 5198384ns | 5198384ns | +0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b4_table | 5182581ns | base | --- | [5176862, 5208579] | --- | --- | --- | --- |
| ab_b4_pred | 6098158ns | +906986.6ns (+17.5%) | [+891089, +931344]ns | [6082291, 6116993] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_prof | 5329536ns | +134683.1ns (+2.6%) | [+120161, +193223]ns | [5316468, 5370084] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_seq | 5291031ns | +106985.0ns (+2.1%) | [+73893, +121908]ns | [5277985, 5301792] | YES | 0.0417 | 0.0313 | 0 |
| ab_b4_tree | 5188416ns | no significant difference | [-28160, +36192]ns | [5171012, 5224432] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b4_table | ab_b4_pred | ab_b4_prof | ab_b4_seq | ab_b4_tree |
|---|---|---|---|---|---|
| 1 | 5221395ns | +17.1% | +2.1% | +1.1% | -0.7% |
| 2 | 5178586ns | +17.6% | +3.6% | +2.4% | +0.0% |
| 3 | 5195762ns | +17.1% | +2.6% | +1.7% | +0.3% |
| 4 | 5175137ns | +17.4% | +3.9% | +1.9% | +0.3% |
| 5 | 5180719ns | +18.2% | +2.6% | +2.3% | +1.1% |
| 6 | 5184443ns | +17.8% | +2.6% | +2.2% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b4_pred | -0.041 | ok |
| ab_b4_prof | -0.377 | moderate- |
| ab_b4_seq | -0.273 | moderate- |
| ab_b4_table | -0.229 | moderate- |
| ab_b4_tree | -0.513 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ab_b4_pred**: won 0/6, lost 6/6
- **ab_b4_prof**: won 0/6, lost 6/6
- **ab_b4_seq**: won 0/6, lost 6/6
- **ab_b4_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b4_pred | 109.5ns | 6099147.1ns | 0.0% |  |
| ab_b4_prof | 82.4ns | 5338695.8ns | 0.0% |  |
| ab_b4_seq | 64.2ns | 5290269.2ns | 0.0% |  |
| ab_b4_table | 62.5ns | 5189340.4ns | 0.0% |  |
| ab_b4_tree | 67.2ns | 5194620.0ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b4_pred (n=6, range 6077815.8-6116993.1 ns)
  6077815.8 |########################################
  6079774.7 |
  6081733.5 |
  6083692.4 |
  6085651.3 |########################################
  6087610.1 |
  6089569.0 |########################################
  6091527.9 |
  6093486.7 |
  6095445.6 |
  6097404.4 |
  6099363.3 |
  6101322.2 |
  6103281.0 |
  6105239.9 |########################################
  6107198.8 |
  6109157.6 |
  6111116.5 |########################################
  6113075.4 |
  6115034.2 |
  (0 below, 1 above range)

ab_b4_prof (n=6, range 5313945.0-5370084.2 ns)
  5313945.0 |####################
  5316752.0 |####################
  5319558.9 |
  5322365.9 |
  5325172.8 |
  5327979.8 |########################################
  5330786.7 |
  5333593.7 |
  5336400.7 |
  5339207.6 |
  5342014.6 |
  5344821.5 |
  5347628.5 |
  5350435.4 |
  5353242.4 |
  5356049.4 |
  5358856.3 |
  5361663.3 |####################
  5364470.2 |
  5367277.2 |
  (0 below, 1 above range)

ab_b4_seq (n=6, range 5275107.1-5301792.1 ns)
  5275107.1 |####################
  5276441.3 |
  5277775.6 |
  5279109.8 |
  5280444.1 |####################
  5281778.3 |
  5283112.6 |####################
  5284446.8 |
  5285781.1 |
  5287115.3 |
  5288449.6 |
  5289783.8 |
  5291118.1 |
  5292452.3 |
  5293786.6 |
  5295120.8 |
  5296455.1 |
  5297789.3 |########################################
  5299123.6 |
  5300457.8 |
  (0 below, 1 above range)

ab_b4_table (n=6, range 5175137.1-5208578.8 ns)
  5175137.1 |########################################
  5176809.2 |
  5178481.3 |########################################
  5180153.3 |########################################
  5181825.4 |
  5183497.5 |########################################
  5185169.6 |
  5186841.7 |
  5188513.8 |
  5190185.8 |
  5191857.9 |
  5193530.0 |
  5195202.1 |########################################
  5196874.2 |
  5198546.3 |
  5200218.3 |
  5201890.4 |
  5203562.5 |
  5205234.6 |
  5206906.7 |
  (0 below, 1 above range)

ab_b4_tree (n=6, range 5162949.6-5224432.3 ns)
  5162949.6 |########################################
  5166023.7 |
  5169097.9 |
  5172172.0 |
  5175246.1 |
  5178320.3 |########################################
  5181394.4 |
  5184468.5 |########################################
  5187542.7 |########################################
  5190616.8 |
  5193690.9 |
  5196765.1 |
  5199839.2 |
  5202913.4 |
  5205987.5 |
  5209061.6 |
  5212135.8 |########################################
  5215209.9 |
  5218284.0 |
  5221358.2 |
  (0 below, 1 above range)

```

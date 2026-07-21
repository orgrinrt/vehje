# Per-branch strategy: archetype 5 (ifchain4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b5_table**

## Highlights

Baseline for all deltas below: **ab_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b5_tree, ab_b5_table) are a dead heat (<1%)

ab_b5_tree (5.18 ms) and ab_b5_table (5.19 ms) differ by 0.24%, inside the noise, even though the wider field spreads 17.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: ab_b5_tree** at 5178272.9 ns median (-0.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.17x (fastest 5178272.9 ns, slowest 6072637.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b5_pred | 6066505ns | 6076236ns | 6012498ns | 6062125ns | 6100077ns | +16.88% |
| ab_b5_prof | 5356807ns | 5352937ns | 5343197ns | 5352524ns | 5370035ns | +3.20% |
| ab_b5_seq | 5319477ns | 5320374ns | 5287285ns | 5315384ns | 5341713ns | +2.48% |
| ab_b5_table | 5190579ns | 5194455ns | 5140534ns | 5190624ns | 5215534ns | base |
| ab_b5_tree | 5181360ns | 5181912ns | 5130968ns | 5169962ns | 5223651ns | -0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b5_pred | 6062792ns | 6008803ns | 6096231ns | +16.88% | 0.003 |
| ab_b5_prof | 5352863ns | 5339231ns | 5366040ns | +3.20% | 0.003 |
| ab_b5_seq | 5315690ns | 5283566ns | 5338146ns | +2.48% | 0.003 |
| ab_b5_table | 5187001ns | 5136616ns | 5211974ns | base | 0.003 |
| ab_b5_tree | 5177791ns | 5127140ns | 5220121ns | -0.18% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b5_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b5_pred | 0.003 | 84.4% |
| ab_b5_prof | 0.003 | 95.9% |
| ab_b5_seq | 0.003 | 96.4% |
| ab_b5_table | 0.003 | 98.8% |
| ab_b5_tree | 0.003 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b5_pred | 6066505ns | 6066505ns | +16.88% |
| ab_b5_prof | 5356807ns | 5356807ns | +3.20% |
| ab_b5_seq | 5319477ns | 5319477ns | +2.48% |
| ab_b5_table | 5190579ns | 5190579ns | base |
| ab_b5_tree | 5181360ns | 5181360ns | -0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b5_table | 5190746ns | base | --- | [5158284, 5211974] | --- | --- | --- | --- |
| ab_b5_pred | 6072637ns | +880073.4ns (+17.0%) | [+839647, +907651]ns | [6019506, 6096231] | YES | 0.0417 | 0.0313 | 0 |
| ab_b5_prof | 5348784ns | +158038.0ns (+3.0%) | [+143106, +196441]ns | [5343765, 5366040] | YES | 0.0417 | 0.0313 | 0 |
| ab_b5_seq | 5316287ns | +126927.5ns (+2.4%) | [+80663, +178476]ns | [5292637, 5338146] | YES | 0.0417 | 0.0313 | 0 |
| ab_b5_tree | 5178273ns | no significant difference | [-76995, +41205]ns | [5134980, 5220121] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b5_table | ab_b5_pred | ab_b5_prof | ab_b5_seq | ab_b5_tree |
|---|---|---|---|---|---|
| 1 | 5136616ns | +17.0% | +3.9% | +4.0% | +0.7% |
| 2 | 5219548ns | +15.5% | +2.7% | +1.2% | -1.8% |
| 3 | 5198768ns | +17.1% | +2.9% | +2.2% | +0.9% |
| 4 | 5179951ns | +17.2% | +3.7% | +2.7% | +0.3% |
| 5 | 5204401ns | +16.7% | +2.8% | +1.9% | -1.2% |
| 6 | 5182723ns | +17.8% | +3.2% | +3.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b5_pred | 0.264 | moderate+ |
| ab_b5_prof | -0.451 | moderate- |
| ab_b5_seq | -0.469 | moderate- |
| ab_b5_table | -0.374 | moderate- |
| ab_b5_tree | -0.324 | moderate- |

**Consistency summary:**

- **ab_b5_pred**: won 0/6, lost 6/6
- **ab_b5_prof**: won 0/6, lost 6/6
- **ab_b5_seq**: won 0/6, lost 6/6
- **ab_b5_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b5_pred | 71.8ns | 6062791.5ns | 0.0% |  |
| ab_b5_prof | 66.9ns | 5352863.0ns | 0.0% |  |
| ab_b5_seq | 94.7ns | 5315690.3ns | 0.0% |  |
| ab_b5_table | 64.3ns | 5187001.3ns | 0.0% |  |
| ab_b5_tree | 56.5ns | 5177791.3ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b5_pred (n=6, range 6008802.9-6096231.0 ns)
  6008802.9 |####################
  6013174.3 |
  6017545.7 |
  6021917.1 |
  6026288.5 |####################
  6030659.9 |
  6035031.3 |
  6039402.7 |
  6043774.1 |
  6048145.5 |
  6052517.0 |
  6056888.4 |
  6061259.8 |
  6065631.2 |
  6070002.6 |########################################
  6074374.0 |
  6078745.4 |
  6083116.8 |####################
  6087488.2 |
  6091859.6 |
  (0 below, 1 above range)

ab_b5_prof (n=6, range 5339230.8-5366040.2 ns)
  5339230.8 |####################
  5340571.3 |
  5341911.7 |
  5343252.2 |
  5344592.7 |
  5345933.1 |
  5347273.6 |########################################
  5348614.1 |####################
  5349954.6 |
  5351295.0 |
  5352635.5 |
  5353976.0 |
  5355316.4 |
  5356656.9 |
  5357997.4 |
  5359337.8 |
  5360678.3 |####################
  5362018.8 |
  5363359.3 |
  5364699.7 |
  (0 below, 1 above range)

ab_b5_seq (n=6, range 5283565.8-5338146.2 ns)
  5283565.8 |########################################
  5286294.8 |
  5289023.8 |
  5291752.9 |
  5294481.9 |
  5297210.9 |
  5299939.9 |########################################
  5302669.0 |
  5305398.0 |
  5308127.0 |
  5310856.0 |
  5313585.0 |########################################
  5316314.1 |########################################
  5319043.1 |
  5321772.1 |
  5324501.1 |
  5327230.2 |
  5329959.2 |
  5332688.2 |
  5335417.2 |########################################
  (0 below, 1 above range)

ab_b5_table (n=6, range 5136616.2-5211974.3 ns)
  5136616.2 |########################################
  5140384.1 |
  5144152.0 |
  5147919.9 |
  5151687.8 |
  5155455.7 |
  5159223.6 |
  5162991.6 |
  5166759.5 |
  5170527.4 |
  5174295.3 |
  5178063.2 |########################################
  5181831.1 |########################################
  5185599.0 |
  5189366.9 |
  5193134.8 |
  5196902.7 |########################################
  5200670.6 |########################################
  5204438.5 |
  5208206.4 |
  (0 below, 1 above range)

ab_b5_tree (n=6, range 5127139.6-5220121.2 ns)
  5127139.6 |########################################
  5131788.7 |
  5136437.8 |
  5141086.8 |########################################
  5145735.9 |
  5150385.0 |
  5155034.1 |
  5159683.2 |
  5164332.3 |
  5168981.3 |########################################
  5173630.4 |
  5178279.5 |
  5182928.6 |########################################
  5187577.7 |
  5192226.8 |########################################
  5196875.8 |
  5201524.9 |
  5206174.0 |
  5210823.1 |
  5215472.2 |
  (0 below, 1 above range)

```

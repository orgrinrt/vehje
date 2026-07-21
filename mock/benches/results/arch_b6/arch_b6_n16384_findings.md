# Per-branch strategy: archetype 6 (ifchain4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b6_table**

## Highlights

Baseline for all deltas below: **ab_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b6_tree, ab_b6_table) are a dead heat (<1%)

ab_b6_tree (5.16 ms) and ab_b6_table (5.22 ms) differ by 0.99%, inside the noise, even though the wider field spreads 88.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b6_prof shows alternating (throttle bounce) (autocorr -0.57)

ab_b6_prof's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_b6_tree, ab_b6_table, ab_b6_prof, ab_b6_seq} vs {ab_b6_pred} (83% apart)

The field splits into a fast tier {ab_b6_tree, ab_b6_table, ab_b6_prof, ab_b6_seq} and a slow tier {ab_b6_pred} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: ab_b6_tree** at 5164082.5 ns median (-1.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.89x (fastest 5164082.5 ns, slowest 9745204.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b6_pred | 9752701ns | 9749573ns | 9729620ns | 9745970ns | 9774338ns | +87.34% |
| ab_b6_prof | 5312884ns | 5308164ns | 5285859ns | 5304095ns | 5339581ns | +2.05% |
| ab_b6_seq | 5319958ns | 5325928ns | 5295435ns | 5316499ns | 5337408ns | +2.19% |
| ab_b6_table | 5205970ns | 5219002ns | 5155865ns | 5199809ns | 5240262ns | base |
| ab_b6_tree | 5167191ns | 5168084ns | 5151700ns | 5162635ns | 5181771ns | -0.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b6_pred | 9748340ns | 9725380ns | 9769858ns | +87.39% | 0.002 |
| ab_b6_prof | 5309122ns | 5281890ns | 5335817ns | +2.06% | 0.003 |
| ab_b6_seq | 5316289ns | 5291733ns | 5333656ns | +2.19% | 0.003 |
| ab_b6_table | 5202207ns | 5152021ns | 5236519ns | base | 0.003 |
| ab_b6_tree | 5163203ns | 5147378ns | 5178099ns | -0.75% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b6_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b6_pred | 0.002 | 52.8% |
| ab_b6_prof | 0.003 | 97.0% |
| ab_b6_seq | 0.003 | 96.7% |
| ab_b6_table | 0.003 | 98.7% |
| ab_b6_tree | 0.003 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b6_pred | 9752701ns | 9752701ns | +87.34% |
| ab_b6_prof | 5312884ns | 5312884ns | +2.05% |
| ab_b6_seq | 5319958ns | 5319958ns | +2.19% |
| ab_b6_table | 5205970ns | 5205970ns | base |
| ab_b6_tree | 5167191ns | 5167191ns | -0.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b6_table | 5215359ns | base | --- | [5154744, 5236519] | --- | --- | --- | --- |
| ab_b6_pred | 9745205ns | +4540800.6ns (+87.1%) | [+4499713, +4597884]ns | [9729957, 9769858] | YES | 0.0417 | 0.0313 | 0 |
| ab_b6_prof | 5304616ns | +95673.1ns (+1.8%) | [+63745, +161326]ns | [5286934, 5335817] | YES | 0.0417 | 0.0313 | 0 |
| ab_b6_seq | 5322311ns | +95339.1ns (+1.8%) | [+86271, +160636]ns | [5292901, 5333656] | YES | 0.0417 | 0.0313 | 0 |
| ab_b6_tree | 5164082ns | no significant difference | [-81376, +9355]ns | [5147428, 5178099] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b6_table | ab_b6_pred | ab_b6_prof | ab_b6_seq | ab_b6_tree |
|---|---|---|---|---|---|
| 1 | 5152021ns | +88.8% | +3.4% | +2.8% | +0.6% |
| 2 | 5157466ns | +89.6% | +2.9% | +3.5% | -0.2% |
| 3 | 5223163ns | +86.7% | +1.5% | +1.8% | -1.4% |
| 4 | 5207555ns | +87.4% | +1.6% | +1.6% | -0.6% |
| 5 | 5239984ns | +85.8% | +2.0% | +1.7% | -1.7% |
| 6 | 5233055ns | +86.0% | +0.9% | +1.9% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b6_pred | -0.265 | moderate- |
| ab_b6_prof | -0.567 | HIGH- (thermal bounce) |
| ab_b6_seq | -0.309 | moderate- |
| ab_b6_table | 0.378 | moderate+ |
| ab_b6_tree | -0.390 | moderate- |

**Consistency summary:**

- **ab_b6_pred**: won 0/6, lost 6/6
- **ab_b6_prof**: won 0/6, lost 6/6
- **ab_b6_seq**: won 0/6, lost 6/6
- **ab_b6_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b6_pred | 125.6ns | 9748339.6ns | 0.0% |  |
| ab_b6_prof | 83.4ns | 5309122.1ns | 0.0% |  |
| ab_b6_seq | 72.2ns | 5316289.4ns | 0.0% |  |
| ab_b6_table | 79.5ns | 5202207.3ns | 0.0% |  |
| ab_b6_tree | 72.8ns | 5163203.2ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b6_pred (n=6, range 9725380.0-9769857.5 ns)
  9725380.0 |########################################
  9727603.9 |
  9729827.8 |
  9732051.6 |
  9734275.5 |########################################
  9736499.4 |########################################
  9738723.2 |
  9740947.1 |
  9743171.0 |
  9745394.9 |
  9747618.8 |
  9749842.6 |
  9752066.5 |########################################
  9754290.4 |
  9756514.2 |
  9758738.1 |########################################
  9760962.0 |
  9763185.9 |
  9765409.8 |
  9767633.6 |
  (0 below, 1 above range)

ab_b6_prof (n=6, range 5281889.6-5335816.8 ns)
  5281889.6 |########################################
  5284586.0 |
  5287282.3 |
  5289978.7 |########################################
  5292675.0 |
  5295371.4 |
  5298067.8 |
  5300764.1 |########################################
  5303460.5 |
  5306156.9 |########################################
  5308853.2 |
  5311549.6 |
  5314245.9 |
  5316942.3 |
  5319638.7 |
  5322335.0 |########################################
  5325031.4 |
  5327727.8 |
  5330424.1 |
  5333120.5 |
  (0 below, 1 above range)

ab_b6_seq (n=6, range 5291732.9-5333656.4 ns)
  5291732.9 |########################################
  5293829.1 |########################################
  5295925.3 |
  5298021.4 |
  5300117.6 |
  5302213.8 |
  5304310.0 |
  5306406.1 |
  5308502.3 |
  5310598.5 |
  5312694.7 |
  5314790.9 |########################################
  5316887.0 |
  5318983.2 |
  5321079.4 |
  5323175.6 |
  5325271.7 |
  5327367.9 |########################################
  5329464.1 |########################################
  5331560.3 |
  (0 below, 1 above range)

ab_b6_table (n=6, range 5152021.2-5236519.4 ns)
  5152021.2 |########################################
  5156246.1 |########################################
  5160471.0 |
  5164695.9 |
  5168920.8 |
  5173145.8 |
  5177370.7 |
  5181595.6 |
  5185820.5 |
  5190045.4 |
  5194270.3 |
  5198495.2 |
  5202720.1 |
  5206945.0 |########################################
  5211169.9 |
  5215394.9 |
  5219619.8 |########################################
  5223844.7 |
  5228069.6 |
  5232294.5 |########################################
  (0 below, 1 above range)

ab_b6_tree (n=6, range 5147377.5-5178099.3 ns)
  5147377.5 |########################################
  5148913.6 |
  5150449.7 |
  5151985.8 |####################
  5153521.9 |
  5155058.0 |
  5156594.1 |
  5158130.1 |
  5159666.2 |
  5161202.3 |
  5162738.4 |
  5164274.5 |
  5165810.6 |
  5167346.7 |
  5168882.8 |
  5170418.9 |
  5171955.0 |
  5173491.1 |
  5175027.2 |########################################
  5176563.3 |
  (0 below, 1 above range)

```

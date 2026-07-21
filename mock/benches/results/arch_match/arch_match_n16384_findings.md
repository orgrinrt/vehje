# Per-type strategy: all match branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_match_table**

## Highlights

Baseline for all deltas below: **ab_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_match_pred is an outlier: 2.8x slower than the field

ab_match_pred (14.28 ms) is 2.8x the fastest (5.19 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ab_match_table, ab_match_prof) are a dead heat (<1%)

ab_match_table (5.19 ms) and ab_match_prof (5.23 ms) differ by 0.89%, inside the noise, even though the wider field spreads 175.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_match_seq shows alternating (throttle bounce) (autocorr -0.51)

ab_match_seq's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_match_table)

The baseline ab_match_table is the fastest (5.19 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_match_table, ab_match_prof, ab_match_seq, ab_match_tree} vs {ab_match_pred} (172% apart)

The field splits into a fast tier {ab_match_table, ab_match_prof, ab_match_seq, ab_match_tree} and a slow tier {ab_match_pred} with a 172% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_match_table) is the fastest** at 5188117.7 ns median
- 2 variants significantly slower than baseline
- Spread: 2.75x (fastest 5188117.7 ns, slowest 14280157.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_match_pred | 14286598ns | 14284819ns | 14250680ns | 14278931ns | 14316056ns | +174.71% |
| ab_match_prof | 5246325ns | 5238334ns | 5215620ns | 5233111ns | 5281500ns | +0.88% |
| ab_match_seq | 5246874ns | 5247584ns | 5225451ns | 5243861ns | 5262104ns | +0.89% |
| ab_match_table | 5200581ns | 5191957ns | 5166052ns | 5184868ns | 5241416ns | base |
| ab_match_tree | 5255518ns | 5259999ns | 5208402ns | 5246688ns | 5292322ns | +1.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_match_pred | 14281812ns | 14246004ns | 14311192ns | +174.82% | 0.001 |
| ab_match_prof | 5242289ns | 5211312ns | 5277730ns | +0.87% | 0.003 |
| ab_match_seq | 5243185ns | 5221965ns | 5258175ns | +0.89% | 0.003 |
| ab_match_table | 5196874ns | 5162157ns | 5237865ns | base | 0.003 |
| ab_match_tree | 5251725ns | 5204380ns | 5288645ns | +1.06% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_match_table; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_match_pred | 0.001 | 36.1% |
| ab_match_prof | 0.003 | 98.6% |
| ab_match_seq | 0.003 | 98.4% |
| ab_match_table | 0.003 | 99.5% |
| ab_match_tree | 0.003 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_match_pred | 14286598ns | 14286598ns | +174.71% |
| ab_match_prof | 5246325ns | 5246325ns | +0.88% |
| ab_match_seq | 5246874ns | 5246874ns | +0.89% |
| ab_match_table | 5200581ns | 5200581ns | base |
| ab_match_tree | 5255518ns | 5255518ns | +1.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_match_table | 5188118ns | base | --- | [5164641, 5237865] | --- | --- | --- | --- |
| ab_match_pred | 14280158ns | +9082315.8ns (+175.1%) | [+9044512, +9127985]ns | [14254086, 14311192] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_match_prof | 5234256ns | no significant difference | [-15819, +94491]ns | [5214879, 5277730] | no | 0.6875 | 0.6875 | 0 |
| ab_match_seq | 5244005ns | +44137.1ns (+0.9%) | [+12339, +82457]ns | [5227376, 5258175] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_match_tree | 5256111ns | no significant difference | [-8286, +112051]ns | [5210419, 5288645] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_match_table | ab_match_pred | ab_match_prof | ab_match_seq | ab_match_tree |
|---|---|---|---|---|---|
| 1 | 5162157ns | +176.6% | +2.2% | +1.2% | +2.7% |
| 2 | 5242640ns | +173.0% | -0.6% | +0.4% | +0.7% |
| 3 | 5233089ns | +172.9% | -0.0% | +0.1% | -0.3% |
| 4 | 5171915ns | +176.7% | +1.2% | +1.5% | +1.7% |
| 5 | 5167125ns | +176.0% | +1.0% | +1.7% | +1.7% |
| 6 | 5204321ns | +173.7% | +1.4% | +0.5% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_match_pred | 0.002 | ok |
| ab_match_prof | -0.342 | moderate- |
| ab_match_seq | -0.511 | HIGH- (thermal bounce) |
| ab_match_table | -0.051 | ok |
| ab_match_tree | -0.000 | ok |

**Consistency summary:**

- **ab_match_pred**: won 0/6, lost 6/6
- **ab_match_prof**: won 1/6, lost 4/6
- **ab_match_seq**: won 0/6, lost 6/6
- **ab_match_tree**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_match_pred | 173.9ns | 14281811.9ns | 0.0% |  |
| ab_match_prof | 80.8ns | 5242288.7ns | 0.0% |  |
| ab_match_seq | 65.0ns | 5243185.3ns | 0.0% |  |
| ab_match_table | 66.5ns | 5196874.3ns | 0.0% |  |
| ab_match_tree | 98.4ns | 5251725.0ns | 0.0% |  |

## Distribution (algo ns)

```
ab_match_pred (n=6, range 14246003.8-14311192.1 ns)
  14246003.8 |####################
  14249263.2 |
  14252522.6 |
  14255782.0 |
  14259041.5 |####################
  14262300.9 |
  14265560.3 |
  14268819.7 |
  14272079.1 |
  14275338.5 |
  14278597.9 |########################################
  14281857.3 |
  14285116.8 |
  14288376.2 |
  14291635.6 |
  14294895.0 |
  14298154.4 |
  14301413.8 |
  14304673.2 |
  14307932.6 |####################
  (0 below, 1 above range)

ab_match_prof (n=6, range 5211312.1-5277730.2 ns)
  5211312.1 |########################################
  5214633.0 |
  5217953.9 |########################################
  5221274.8 |
  5224595.7 |
  5227916.6 |
  5231237.5 |########################################
  5234558.4 |########################################
  5237879.3 |
  5241200.2 |
  5244521.2 |
  5247842.1 |
  5251163.0 |
  5254483.9 |
  5257804.8 |
  5261125.7 |
  5264446.6 |
  5267767.5 |
  5271088.4 |
  5274409.3 |########################################
  (0 below, 1 above range)

ab_match_seq (n=6, range 5221965.4-5258175.5 ns)
  5221965.4 |########################################
  5223775.9 |
  5225586.4 |
  5227396.9 |
  5229207.4 |
  5231017.9 |########################################
  5232828.4 |
  5234638.9 |
  5236449.4 |
  5238259.9 |########################################
  5240070.4 |
  5241880.9 |
  5243691.4 |
  5245501.9 |
  5247312.4 |
  5249122.9 |########################################
  5250933.4 |
  5252743.9 |########################################
  5254554.4 |
  5256364.9 |
  (0 below, 1 above range)

ab_match_table (n=6, range 5162156.7-5237864.6 ns)
  5162156.7 |########################################
  5165942.1 |########################################
  5169727.5 |########################################
  5173512.9 |
  5177298.3 |
  5181083.7 |
  5184869.1 |
  5188654.5 |
  5192439.9 |
  5196225.3 |
  5200010.7 |
  5203796.0 |########################################
  5207581.4 |
  5211366.8 |
  5215152.2 |
  5218937.6 |
  5222723.0 |
  5226508.4 |
  5230293.8 |########################################
  5234079.2 |
  (0 below, 1 above range)

ab_match_tree (n=6, range 5204379.6-5288644.6 ns)
  5204379.6 |########################################
  5208592.8 |
  5212806.1 |########################################
  5217019.3 |
  5221232.6 |
  5225445.8 |
  5229659.1 |
  5233872.3 |
  5238085.6 |
  5242298.8 |
  5246512.1 |
  5250725.3 |########################################
  5254938.6 |########################################
  5259151.8 |
  5263365.1 |
  5267578.3 |
  5271791.6 |
  5276004.8 |########################################
  5280218.1 |
  5284431.3 |
  (0 below, 1 above range)

```

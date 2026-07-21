# Per-branch strategy: archetype 2 (match8_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b2_table**

## Highlights

Baseline for all deltas below: **ab_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_b2_table is fastest but the noisiest (CV 6.3%)

ab_b2_table wins on median (85.42 us) yet has the highest variance (CV 6.3%), while ab_b2_pred is the steadiest (CV 5.3%, 115.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ab_b2_table, ab_b2_seq) are a dead heat (<1%)

ab_b2_table (85.42 us) and ab_b2_seq (86.09 us) differ by 0.78%, inside the noise, even though the wider field spreads 35.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ab_b2_table)

The baseline ab_b2_table is the fastest (85.42 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_b2_table, ab_b2_seq, ab_b2_prof, ab_b2_tree} vs {ab_b2_pred} (26% apart)

The field splits into a fast tier {ab_b2_table, ab_b2_seq, ab_b2_prof, ab_b2_tree} and a slow tier {ab_b2_pred} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_b2_table) is the fastest** at 85425.0 ns median
- 3 variants significantly slower than baseline
- Spread: 1.36x (fastest 85425.0 ns, slowest 115901.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b2_pred | 121555ns | 118094ns | 116015ns | 117729ns | 130064ns | +38.26% |
| ab_b2_prof | 91654ns | 94316ns | 82418ns | 91130ns | 97058ns | +4.25% |
| ab_b2_seq | 89625ns | 88431ns | 82710ns | 87607ns | 96110ns | +1.94% |
| ab_b2_table | 87920ns | 87786ns | 81638ns | 85988ns | 93959ns | base |
| ab_b2_tree | 92034ns | 94804ns | 83061ns | 91546ns | 97254ns | +4.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b2_pred | 119190ns | 113645ns | 127506ns | +39.36% | 0.002 |
| ab_b2_prof | 89181ns | 80105ns | 94462ns | +4.27% | 0.003 |
| ab_b2_seq | 87227ns | 80454ns | 93476ns | +1.98% | 0.003 |
| ab_b2_table | 85529ns | 79326ns | 91453ns | base | 0.003 |
| ab_b2_tree | 89504ns | 80703ns | 94513ns | +4.65% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b2_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b2_pred | 0.002 | 68.4% |
| ab_b2_prof | 0.003 | 86.5% |
| ab_b2_seq | 0.003 | 92.1% |
| ab_b2_table | 0.003 | 92.9% |
| ab_b2_tree | 0.003 | 86.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b2_pred | 121555ns | 121555ns | +38.26% |
| ab_b2_prof | 91654ns | 91654ns | +4.25% |
| ab_b2_seq | 89625ns | 89625ns | +1.94% |
| ab_b2_table | 87920ns | 87920ns | base |
| ab_b2_tree | 92034ns | 92034ns | +4.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b2_table | 85425ns | base | --- | [79709, 91453] | --- | --- | --- | --- |
| ab_b2_pred | 115902ns | +34792.2ns (+40.7%) | [+29343, +36848]ns | [114163, 127506] | YES | 0.0417 | 0.0313 | 0 |
| ab_b2_prof | 91743ns | +2577.9ns (+3.0%) | [+58, +8321]ns | [81339, 94462] | YES | 0.0417 | 0.0313 | 0 |
| ab_b2_seq | 86090ns | no significant difference | [-1840, +5442]ns | [82114, 93476] | no | 0.6875 | 0.6875 | 0 |
| ab_b2_tree | 92230ns | +2140.0ns (+2.5%) | [+437, +9349]ns | [81771, 94513] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b2_table | ab_b2_pred | ab_b2_prof | ab_b2_seq | ab_b2_tree |
|---|---|---|---|---|---|
| 1 | 80092ns | +41.9% | +0.0% | +0.5% | +0.8% |
| 2 | 91905ns | +41.2% | +0.1% | -0.1% | +0.3% |
| 3 | 81155ns | +44.1% | +1.7% | +3.2% | +2.1% |
| 4 | 79326ns | +44.6% | +15.3% | +6.9% | +18.0% |
| 5 | 89695ns | +28.0% | +4.2% | +6.0% | +2.9% |
| 6 | 91002ns | +37.6% | +4.9% | -4.0% | +4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b2_pred | -0.353 | moderate- |
| ab_b2_prof | -0.116 | ok |
| ab_b2_seq | -0.384 | moderate- |
| ab_b2_table | -0.220 | moderate- |
| ab_b2_tree | -0.216 | moderate- |

**Consistency summary:**

- **ab_b2_pred**: won 0/6, lost 6/6
- **ab_b2_prof**: won 0/6, lost 5/6
- **ab_b2_seq**: won 1/6, lost 4/6
- **ab_b2_tree**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b2_pred | 5.3ns | 119190.1ns | 0.0% |  |
| ab_b2_prof | 5.9ns | 89181.4ns | 0.0% |  |
| ab_b2_seq | 4.2ns | 87226.7ns | 0.0% |  |
| ab_b2_table | 5.0ns | 85529.1ns | 0.0% |  |
| ab_b2_tree | 5.7ns | 89504.4ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b2_pred (n=6, range 113644.6-127505.8 ns)
  113644.6 |####################
  114337.7 |########################################
  115030.7 |
  115723.8 |
  116416.8 |####################
  117109.9 |
  117803.0 |
  118496.0 |
  119189.1 |
  119882.1 |
  120575.2 |
  121268.3 |
  121961.3 |
  122654.4 |
  123347.4 |
  124040.5 |
  124733.6 |####################
  125426.6 |
  126119.7 |
  126812.7 |
  (0 below, 1 above range)

ab_b2_prof (n=6, range 80105.4-94462.2 ns)
  80105.4 |########################################
  80823.2 |
  81541.1 |
  82258.9 |########################################
  82976.8 |
  83694.6 |
  84412.5 |
  85130.3 |
  85848.1 |
  86566.0 |
  87283.8 |
  88001.7 |
  88719.5 |
  89437.4 |
  90155.2 |
  90873.0 |########################################
  91590.9 |########################################
  92308.7 |
  93026.6 |########################################
  93744.4 |
  (0 below, 1 above range)

ab_b2_seq (n=6, range 80454.2-93475.9 ns)
  80454.2 |########################################
  81105.3 |
  81756.4 |
  82407.4 |
  83058.5 |
  83709.6 |########################################
  84360.7 |########################################
  85011.8 |
  85662.9 |
  86313.9 |
  86965.0 |########################################
  87616.1 |
  88267.2 |
  88918.3 |
  89569.4 |
  90220.4 |
  90871.5 |
  91522.6 |########################################
  92173.7 |
  92824.8 |
  (0 below, 1 above range)

ab_b2_table (n=6, range 79325.8-91453.4 ns)
  79325.8 |########################################
  79932.2 |########################################
  80538.6 |
  81144.9 |########################################
  81751.3 |
  82357.7 |
  82964.1 |
  83570.4 |
  84176.8 |
  84783.2 |
  85389.6 |
  85996.0 |
  86602.3 |
  87208.7 |
  87815.1 |
  88421.5 |
  89027.8 |
  89634.2 |########################################
  90240.6 |
  90847.0 |########################################
  (0 below, 1 above range)

ab_b2_tree (n=6, range 80702.9-94512.7 ns)
  80702.9 |####################
  81393.4 |
  82083.9 |
  82774.4 |####################
  83464.9 |
  84155.4 |
  84845.8 |
  85536.3 |
  86226.8 |
  86917.3 |
  87607.8 |
  88298.3 |
  88988.8 |
  89679.3 |
  90369.8 |
  91060.2 |
  91750.7 |########################################
  92441.2 |
  93131.7 |####################
  93822.2 |
  (0 below, 1 above range)

```

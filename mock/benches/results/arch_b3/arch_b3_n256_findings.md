# Per-branch strategy: archetype 3 (match4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b3_table**

## Highlights

Baseline for all deltas below: **ab_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b3_table, ab_b3_seq) are a dead heat (<1%)

ab_b3_table (81.19 us) and ab_b3_seq (81.31 us) differ by 0.15%, inside the noise, even though the wider field spreads 91.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b3_table shows alternating (throttle bounce) (autocorr -0.58)

ab_b3_table's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b3_table)

The baseline ab_b3_table is the fastest (81.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_b3_table, ab_b3_seq, ab_b3_tree, ab_b3_prof} vs {ab_b3_pred} (89% apart)

The field splits into a fast tier {ab_b3_table, ab_b3_seq, ab_b3_tree, ab_b3_prof} and a slow tier {ab_b3_pred} with a 89% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_b3_table) is the fastest** at 81190.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 81190.6 ns, slowest 155178.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b3_pred | 158890ns | 157493ns | 156335ns | 157292ns | 162566ns | +89.84% |
| ab_b3_prof | 84212ns | 84399ns | 82742ns | 84254ns | 84884ns | +0.61% |
| ab_b3_seq | 83593ns | 83538ns | 82478ns | 83415ns | 84417ns | -0.13% |
| ab_b3_table | 83699ns | 83418ns | 81459ns | 83325ns | 85380ns | base |
| ab_b3_tree | 84158ns | 83840ns | 82687ns | 83636ns | 85676ns | +0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b3_pred | 156482ns | 153997ns | 159989ns | +92.11% | 0.002 |
| ab_b3_prof | 81908ns | 80462ns | 82581ns | +0.56% | 0.003 |
| ab_b3_seq | 81322ns | 80190ns | 82147ns | -0.16% | 0.003 |
| ab_b3_table | 81456ns | 79258ns | 83107ns | base | 0.003 |
| ab_b3_tree | 81939ns | 80502ns | 83377ns | +0.59% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b3_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b3_pred | 0.002 | 51.1% |
| ab_b3_prof | 0.003 | 96.5% |
| ab_b3_seq | 0.003 | 97.5% |
| ab_b3_table | 0.003 | 97.6% |
| ab_b3_tree | 0.003 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b3_pred | 158890ns | 158890ns | +89.84% |
| ab_b3_prof | 84212ns | 84212ns | +0.61% |
| ab_b3_seq | 83593ns | 83593ns | -0.13% |
| ab_b3_table | 83699ns | 83699ns | base |
| ab_b3_tree | 84158ns | 84158ns | +0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b3_table | 81191ns | base | --- | [80069, 83107] | --- | --- | --- | --- |
| ab_b3_pred | 155178ns | +74450.8ns (+91.7%) | [+73459, +77170]ns | [154280, 159989] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b3_prof | 82101ns | no significant difference | [-1096, +1757]ns | [81042, 82581] | no | 0.9167 | 0.6875 | 0 |
| ab_b3_seq | 81309ns | no significant difference | [-1798, +1060]ns | [80511, 82147] | no | 1.0000 | 1.0000 | 0 |
| ab_b3_tree | 81658ns | no significant difference | [-388, +1493]ns | [80780, 83377] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b3_table | ab_b3_pred | ab_b3_prof | ab_b3_seq | ab_b3_tree |
|---|---|---|---|---|---|
| 1 | 79258ns | +94.3% | +3.0% | +1.2% | +2.6% |
| 2 | 84269ns | +88.6% | -2.1% | -3.4% | -0.5% |
| 3 | 80943ns | +91.0% | +0.9% | -0.1% | +0.1% |
| 4 | 81945ns | +89.4% | +0.8% | -0.9% | +1.1% |
| 5 | 81438ns | +97.7% | +1.4% | +1.0% | +0.7% |
| 6 | 80880ns | +91.8% | -0.5% | +1.5% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b3_pred | -0.507 | HIGH- (thermal bounce) |
| ab_b3_prof | -0.274 | moderate- |
| ab_b3_seq | 0.192 | ok |
| ab_b3_table | -0.580 | HIGH- (thermal bounce) |
| ab_b3_tree | -0.482 | moderate- |

**Consistency summary:**

- **ab_b3_pred**: won 0/6, lost 6/6
- **ab_b3_prof**: won 2/6, lost 4/6
- **ab_b3_seq**: won 3/6, lost 3/6
- **ab_b3_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b3_pred | 7.3ns | 156482.3ns | 0.0% |  |
| ab_b3_prof | 3.2ns | 81908.2ns | 0.0% |  |
| ab_b3_seq | 5.3ns | 81322.3ns | 0.0% |  |
| ab_b3_table | 4.8ns | 81455.6ns | 0.0% |  |
| ab_b3_tree | 4.5ns | 81938.6ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b3_pred (n=6, range 153997.1-159988.5 ns)
  153997.1 |########################################
  154296.7 |########################################
  154596.2 |
  154895.8 |########################################
  155195.4 |########################################
  155495.0 |
  155794.5 |
  156094.1 |
  156393.7 |
  156693.3 |
  156992.8 |
  157292.4 |
  157592.0 |
  157891.5 |
  158191.1 |
  158490.7 |
  158790.3 |########################################
  159089.8 |
  159389.4 |
  159689.0 |
  (0 below, 1 above range)

ab_b3_prof (n=6, range 80461.7-82581.0 ns)
  80461.7 |####################
  80567.7 |
  80673.6 |
  80779.6 |
  80885.6 |
  80991.5 |
  81097.5 |
  81203.5 |
  81309.4 |
  81415.4 |
  81521.4 |####################
  81627.3 |####################
  81733.3 |
  81839.3 |
  81945.2 |
  82051.2 |
  82157.2 |
  82263.1 |
  82369.1 |
  82475.1 |########################################
  (0 below, 1 above range)

ab_b3_seq (n=6, range 80190.0-82147.2 ns)
  80190.0 |########################################
  80287.9 |
  80385.7 |
  80483.6 |
  80581.4 |
  80679.3 |
  80777.2 |########################################
  80875.0 |
  80972.9 |
  81070.8 |
  81168.6 |########################################
  81266.5 |
  81364.4 |########################################
  81462.2 |
  81560.1 |
  81657.9 |
  81755.8 |
  81853.7 |
  81951.5 |
  82049.4 |########################################
  (0 below, 1 above range)

ab_b3_table (n=6, range 79257.9-83106.9 ns)
  79257.9 |####################
  79450.3 |
  79642.8 |
  79835.2 |
  80027.7 |
  80220.1 |
  80412.6 |
  80605.0 |
  80797.5 |########################################
  80989.9 |
  81182.4 |
  81374.8 |####################
  81567.3 |
  81759.8 |####################
  81952.2 |
  82144.6 |
  82337.1 |
  82529.5 |
  82722.0 |
  82914.4 |
  (0 below, 1 above range)

ab_b3_tree (n=6, range 80502.1-83377.2 ns)
  80502.1 |########################################
  80645.9 |
  80789.6 |
  80933.4 |########################################
  81077.1 |
  81220.9 |########################################
  81364.6 |
  81508.4 |
  81652.2 |
  81795.9 |
  81939.7 |########################################
  82083.4 |
  82227.2 |
  82370.9 |
  82514.7 |
  82658.5 |
  82802.2 |########################################
  82946.0 |
  83089.7 |
  83233.5 |
  (0 below, 1 above range)

```

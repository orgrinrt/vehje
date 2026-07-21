# Per-branch strategy (NATIVE tier): archetype 3

5 variants, 6 samples per variant.
Baseline: **an_b3_table**

## Highlights

Baseline for all deltas below: **an_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b3_table is fastest but the noisiest (CV 8.4%)

an_b3_table wins on median (51.46 us) yet has the highest variance (CV 8.4%), while an_b3_tree is the steadiest (CV 6.0%, 57.06 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### an_b3_table shows alternating (throttle bounce) (autocorr -0.50)

an_b3_table's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (an_b3_table)

The baseline an_b3_table is the fastest (51.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b3_table, an_b3_seq, an_b3_tree, an_b3_prof} vs {an_b3_pred} (29% apart)

The field splits into a fast tier {an_b3_table, an_b3_seq, an_b3_tree, an_b3_prof} and a slow tier {an_b3_pred} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (an_b3_table) is the fastest** at 51461.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.48x (fastest 51461.2 ns, slowest 76030.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b3_pred | 79638ns | 78434ns | 71559ns | 78142ns | 85921ns | +48.15% |
| an_b3_prof | 59881ns | 61142ns | 54630ns | 59081ns | 63705ns | +11.39% |
| an_b3_seq | 58300ns | 57876ns | 53820ns | 57136ns | 62285ns | +8.45% |
| an_b3_table | 53757ns | 53720ns | 47668ns | 52644ns | 58471ns | base |
| an_b3_tree | 58950ns | 59413ns | 52375ns | 58940ns | 62251ns | +9.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b3_pred | 77191ns | 69354ns | 83267ns | +49.98% | 0.053 |
| an_b3_prof | 57476ns | 52489ns | 61043ns | +11.67% | 0.071 |
| an_b3_seq | 55945ns | 51561ns | 59902ns | +8.70% | 0.073 |
| an_b3_table | 51468ns | 45266ns | 56202ns | base | 0.080 |
| an_b3_tree | 56602ns | 50125ns | 59818ns | +9.97% | 0.072 |

## Performance model

- Peak throughput: **0.090 Gops/s** (an_b3_table; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b3_pred | 0.054 | 59.5% |
| an_b3_prof | 0.070 | 77.0% |
| an_b3_seq | 0.074 | 81.5% |
| an_b3_table | 0.080 | 88.0% |
| an_b3_tree | 0.072 | 79.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b3_pred | 79638ns | 79638ns | +48.15% |
| an_b3_prof | 59881ns | 59881ns | +11.39% |
| an_b3_seq | 58300ns | 58300ns | +8.45% |
| an_b3_table | 53757ns | 53757ns | base |
| an_b3_tree | 58950ns | 58950ns | +9.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b3_table | 51461ns | base | --- | [46742, 56202] | --- | --- | --- | --- |
| an_b3_pred | 76030ns | +28209.4ns (+54.8%) | [+16073, +32885]ns | [72275, 83267] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b3_prof | 58752ns | no significant difference | [-3275, +14301]ns | [52632, 61043] | no | 0.6875 | 0.6875 | 0 |
| an_b3_seq | 55519ns | no significant difference | [-683, +9120]ns | [52414, 59902] | no | 0.2917 | 0.2188 | 0 |
| an_b3_tree | 57056ns | no significant difference | [-2976, +13075]ns | [52931, 59818] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b3_table | an_b3_pred | an_b3_prof | an_b3_seq | an_b3_tree |
|---|---|---|---|---|---|
| 1 | 54288ns | +53.4% | -3.3% | +12.2% | +2.7% |
| 2 | 54878ns | +37.0% | +9.1% | +4.9% | +6.0% |
| 3 | 45266ns | +68.8% | +36.4% | +17.7% | +29.4% |
| 4 | 57526ns | +20.6% | -8.3% | -7.0% | -12.9% |
| 5 | 48634ns | +71.2% | +18.5% | +21.1% | +15.0% |
| 6 | 48218ns | +56.9% | +25.1% | +6.9% | +26.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b3_pred | -0.431 | moderate- |
| an_b3_prof | -0.279 | moderate- |
| an_b3_seq | -0.142 | ok |
| an_b3_table | -0.503 | HIGH- (thermal bounce) |
| an_b3_tree | -0.138 | ok |

**Consistency summary:**

- **an_b3_pred**: won 0/6, lost 6/6
- **an_b3_prof**: won 2/6, lost 4/6
- **an_b3_seq**: won 1/6, lost 5/6
- **an_b3_tree**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b3_pred | 11.7ns | 77190.8ns | 0.0% |  |
| an_b3_prof | 5.5ns | 57475.8ns | 0.0% |  |
| an_b3_seq | 10.3ns | 55945.3ns | 0.0% |  |
| an_b3_table | 6.2ns | 51468.4ns | 0.0% |  |
| an_b3_tree | 10.1ns | 56601.5ns | 0.0% |  |

## Distribution (algo ns)

```
an_b3_pred (n=6, range 69353.8-83267.3 ns)
  69353.8 |########################################
  70049.5 |
  70745.2 |
  71440.8 |
  72136.5 |
  72832.2 |
  73527.9 |
  74223.5 |
  74919.2 |########################################
  75614.9 |########################################
  76310.6 |########################################
  77006.2 |
  77701.9 |
  78397.6 |
  79093.2 |
  79788.9 |
  80484.6 |
  81180.3 |
  81875.9 |
  82571.6 |########################################
  (0 below, 1 above range)

an_b3_prof (n=6, range 52489.2-61043.1 ns)
  52489.2 |########################################
  52916.9 |
  53344.6 |
  53772.3 |
  54200.0 |
  54627.7 |
  55055.4 |
  55483.1 |
  55910.8 |
  56338.5 |
  56766.1 |
  57193.8 |
  57621.5 |####################
  58049.2 |
  58476.9 |
  58904.6 |
  59332.3 |
  59760.0 |####################
  60187.7 |####################
  60615.4 |
  (0 below, 1 above range)

an_b3_seq (n=6, range 51560.8-59902.5 ns)
  51560.8 |####################
  51977.9 |
  52395.0 |
  52812.1 |
  53229.1 |########################################
  53646.2 |
  54063.3 |
  54480.4 |
  54897.5 |
  55314.6 |
  55731.7 |
  56148.7 |
  56565.8 |
  56982.9 |
  57400.0 |####################
  57817.1 |
  58234.2 |
  58651.2 |####################
  59068.3 |
  59485.4 |
  (0 below, 1 above range)

an_b3_table (n=6, range 45265.8-56202.1 ns)
  45265.8 |########################################
  45812.6 |
  46359.4 |
  46906.2 |
  47453.1 |
  47999.9 |########################################
  48546.7 |########################################
  49093.5 |
  49640.3 |
  50187.1 |
  50733.9 |
  51280.7 |
  51827.6 |
  52374.4 |
  52921.2 |
  53468.0 |
  54014.8 |########################################
  54561.6 |########################################
  55108.4 |
  55655.2 |
  (0 below, 1 above range)

an_b3_tree (n=6, range 50125.4-59817.5 ns)
  50125.4 |####################
  50610.0 |
  51094.6 |
  51579.2 |
  52063.8 |
  52548.4 |
  53033.0 |
  53517.6 |
  54002.2 |
  54486.8 |
  54971.4 |
  55456.1 |########################################
  55940.7 |
  56425.3 |
  56909.9 |
  57394.5 |
  57879.1 |####################
  58363.7 |####################
  58848.3 |
  59332.9 |
  (0 below, 1 above range)

```

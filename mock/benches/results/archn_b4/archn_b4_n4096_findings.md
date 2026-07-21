# Per-branch strategy (NATIVE tier): archetype 4

5 variants, 6 samples per variant.
Baseline: **an_b4_table**

## Highlights

Baseline for all deltas below: **an_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b4_table, an_b4_tree) are a dead heat (<1%)

an_b4_table (54.92 us) and an_b4_tree (55.27 us) differ by 0.63%, inside the noise, even though the wider field spreads 11.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b4_pred shows alternating (throttle bounce) (autocorr -0.51)

an_b4_pred's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.38 us) is smaller than the fastest variant's own run-to-run std-dev (7.36 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (an_b4_table)

The baseline an_b4_table is the fastest (54.92 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (an_b4_table) is the fastest** at 54922.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.12x (fastest 54922.7 ns, slowest 61298.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b4_pred | 59142ns | 61118ns | 47009ns | 57042ns | 68357ns | +1.63% |
| an_b4_prof | 64771ns | 63476ns | 62415ns | 63186ns | 68326ns | +11.31% |
| an_b4_seq | 63260ns | 63522ns | 55388ns | 61568ns | 69735ns | +8.71% |
| an_b4_table | 58191ns | 57235ns | 48545ns | 55142ns | 67587ns | base |
| an_b4_tree | 57263ns | 57557ns | 50736ns | 57544ns | 60104ns | -1.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b4_pred | 56842ns | 44785ns | 65922ns | +1.82% | 0.072 |
| an_b4_prof | 62455ns | 60160ns | 65913ns | +11.88% | 0.066 |
| an_b4_seq | 60896ns | 53077ns | 67340ns | +9.09% | 0.067 |
| an_b4_table | 55824ns | 46372ns | 65021ns | base | 0.073 |
| an_b4_tree | 54976ns | 48563ns | 57752ns | -1.52% | 0.075 |

## Performance model

- Peak throughput: **0.091 Gops/s** (an_b4_pred; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b4_pred | 0.070 | 76.0% |
| an_b4_prof | 0.067 | 73.2% |
| an_b4_seq | 0.067 | 73.1% |
| an_b4_table | 0.075 | 81.5% |
| an_b4_tree | 0.074 | 81.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b4_pred | 59142ns | 59142ns | +1.63% |
| an_b4_prof | 64771ns | 64771ns | +11.31% |
| an_b4_seq | 63260ns | 63260ns | +8.71% |
| an_b4_table | 58191ns | 58191ns | base |
| an_b4_tree | 57263ns | 57263ns | -1.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b4_table | 54923ns | base | --- | [47527, 65021] | --- | --- | --- | --- |
| an_b4_pred | 58926ns | no significant difference | [-12484, +13218]ns | [45679, 65922] | no | 0.9167 | 0.6875 | 0 |
| an_b4_prof | 61151ns | +5931.1ns (+10.8%) | [+892, +13072]ns | [60302, 65913] | YES (adj: no) | 0.8750 | 0.2188 | 0 |
| an_b4_seq | 61298ns | no significant difference | [-5190, +18105]ns | [54050, 67340] | no | 0.9167 | 0.6875 | 0 |
| an_b4_tree | 55269ns | no significant difference | [-9874, +7742]ns | [51906, 57752] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b4_table | an_b4_pred | an_b4_prof | an_b4_seq | an_b4_tree |
|---|---|---|---|---|---|
| 1 | 48683ns | +31.6% | +24.8% | +43.9% | +13.5% |
| 2 | 57051ns | +5.9% | +5.4% | +7.6% | -14.9% |
| 3 | 46372ns | +23.9% | +30.3% | +32.0% | +19.2% |
| 4 | 63532ns | -26.7% | +3.6% | -13.4% | -7.7% |
| 5 | 66510ns | +1.9% | -0.7% | -2.8% | -16.9% |
| 6 | 52794ns | -15.2% | +16.6% | +0.5% | +7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b4_pred | -0.511 | HIGH- (thermal bounce) |
| an_b4_prof | 0.284 | moderate+ |
| an_b4_seq | -0.252 | moderate- |
| an_b4_table | -0.133 | ok |
| an_b4_tree | -0.020 | ok |

**Consistency summary:**

- **an_b4_pred**: won 2/6, lost 4/6
- **an_b4_prof**: won 1/6, lost 5/6
- **an_b4_seq**: won 2/6, lost 4/6
- **an_b4_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b4_pred | 4.5ns | 56842.2ns | 0.0% |  |
| an_b4_prof | 5.9ns | 62455.5ns | 0.0% |  |
| an_b4_seq | 6.4ns | 60896.0ns | 0.0% |  |
| an_b4_table | 7.1ns | 55823.8ns | 0.0% |  |
| an_b4_tree | 4.8ns | 54976.0ns | 0.0% |  |

## Distribution (algo ns)

```
an_b4_pred (n=6, range 44785.4-65921.6 ns)
  44785.4 |########################################
  45842.2 |########################################
  46899.0 |
  47955.8 |
  49012.7 |
  50069.5 |
  51126.3 |
  52183.1 |
  53239.9 |
  54296.7 |
  55353.5 |
  56410.3 |########################################
  57467.1 |
  58524.0 |
  59580.8 |########################################
  60637.6 |
  61694.4 |
  62751.2 |
  63808.0 |########################################
  64864.8 |
  (0 below, 1 above range)

an_b4_prof (n=6, range 60160.0-65913.1 ns)
  60160.0 |########################################
  60447.7 |
  60735.3 |####################
  61023.0 |
  61310.6 |####################
  61598.3 |
  61885.9 |
  62173.6 |
  62461.2 |
  62748.9 |
  63036.6 |
  63324.2 |
  63611.9 |
  63899.5 |
  64187.2 |
  64474.8 |
  64762.5 |
  65050.1 |
  65337.8 |
  65625.4 |####################
  (0 below, 1 above range)

an_b4_seq (n=6, range 53077.1-67340.0 ns)
  53077.1 |####################
  53790.2 |
  54503.4 |####################
  55216.5 |
  55929.7 |
  56642.8 |
  57356.0 |
  58069.1 |
  58782.3 |
  59495.4 |
  60208.6 |
  60921.7 |########################################
  61634.8 |
  62348.0 |
  63061.1 |
  63774.3 |
  64487.4 |####################
  65200.6 |
  65913.7 |
  66626.9 |
  (0 below, 1 above range)

an_b4_table (n=6, range 46371.7-65021.4 ns)
  46371.7 |########################################
  47304.2 |
  48236.7 |########################################
  49169.2 |
  50101.6 |
  51034.1 |
  51966.6 |########################################
  52899.1 |
  53831.6 |
  54764.1 |
  55696.6 |
  56629.1 |########################################
  57561.5 |
  58494.0 |
  59426.5 |
  60359.0 |
  61291.5 |
  62224.0 |
  63156.5 |########################################
  64089.0 |
  (0 below, 1 above range)

an_b4_tree (n=6, range 48563.3-57752.3 ns)
  48563.3 |#############
  49022.8 |
  49482.2 |
  49941.7 |
  50401.1 |
  50860.6 |
  51320.0 |
  51779.5 |
  52238.9 |
  52698.4 |
  53157.8 |
  53617.2 |
  54076.7 |
  54536.2 |
  54995.6 |########################################
  55455.1 |
  55914.5 |
  56374.0 |
  56833.4 |#############
  57292.9 |
  (0 below, 1 above range)

```

# Per-branch strategy: archetype 0 (match4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b0_table**

## Highlights

Baseline for all deltas below: **ab_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b0_table, ab_b0_prof) are a dead heat (<1%)

ab_b0_table (324.95 us) and ab_b0_prof (325.95 us) differ by 0.31%, inside the noise, even though the wider field spreads 20.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b0_table shows alternating (throttle bounce) (autocorr -0.69)

ab_b0_table's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b0_table)

The baseline ab_b0_table is the fastest (324.95 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b0_table) is the fastest** at 324953.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 324953.6 ns, slowest 392276.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b0_pred | 394529ns | 394995ns | 383076ns | 393449ns | 401875ns | +20.30% |
| ab_b0_prof | 330847ns | 328230ns | 324115ns | 327125ns | 339795ns | +0.88% |
| ab_b0_seq | 334101ns | 332893ns | 324415ns | 332640ns | 341135ns | +1.88% |
| ab_b0_table | 327945ns | 327369ns | 321841ns | 326103ns | 333761ns | base |
| ab_b0_tree | 330009ns | 330253ns | 324482ns | 329501ns | 333535ns | +0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b0_pred | 391842ns | 380219ns | 399340ns | +20.39% | 0.003 |
| ab_b0_prof | 328418ns | 321644ns | 337280ns | +0.90% | 0.003 |
| ab_b0_seq | 331644ns | 321838ns | 338680ns | +1.89% | 0.003 |
| ab_b0_table | 325490ns | 319130ns | 331481ns | base | 0.003 |
| ab_b0_tree | 327616ns | 322305ns | 331021ns | +0.65% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b0_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b0_pred | 0.003 | 81.4% |
| ab_b0_prof | 0.003 | 97.9% |
| ab_b0_seq | 0.003 | 96.6% |
| ab_b0_table | 0.003 | 98.2% |
| ab_b0_tree | 0.003 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b0_pred | 394529ns | 394529ns | +20.30% |
| ab_b0_prof | 330847ns | 330847ns | +0.88% |
| ab_b0_seq | 334101ns | 334101ns | +1.88% |
| ab_b0_table | 327945ns | 327945ns | base |
| ab_b0_tree | 330009ns | 330009ns | +0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b0_table | 324954ns | base | --- | [320035, 331481] | --- | --- | --- | --- |
| ab_b0_pred | 392276ns | +65904.5ns (+20.3%) | [+62431, +70720]ns | [383909, 399340] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b0_prof | 325950ns | no significant difference | [-5970, +14153]ns | [322025, 337280] | no | 1.0000 | 1.0000 | 0 |
| ab_b0_seq | 330389ns | no significant difference | [-3353, +17342]ns | [325864, 338680] | no | 1.0000 | 1.0000 | 0 |
| ab_b0_tree | 327927ns | no significant difference | [-2212, +6781]ns | [323899, 331021] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b0_table | ab_b0_pred | ab_b0_prof | ab_b0_seq | ab_b0_tree |
|---|---|---|---|---|---|
| 1 | 320941ns | +20.8% | +0.5% | +3.0% | +1.4% |
| 2 | 330517ns | +21.8% | -0.2% | -0.2% | -0.4% |
| 3 | 323545ns | +21.5% | -0.4% | +4.3% | +2.8% |
| 4 | 332444ns | +19.2% | -3.2% | -0.7% | -0.9% |
| 5 | 319130ns | +19.1% | +8.0% | +6.5% | +1.0% |
| 6 | 326362ns | +20.0% | +0.9% | -1.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b0_pred | -0.259 | moderate- |
| ab_b0_prof | -0.195 | ok |
| ab_b0_seq | -0.530 | HIGH- (thermal bounce) |
| ab_b0_table | -0.689 | HIGH- (thermal bounce) |
| ab_b0_tree | 0.121 | ok |

**Consistency summary:**

- **ab_b0_pred**: won 0/6, lost 6/6
- **ab_b0_prof**: won 3/6, lost 3/6
- **ab_b0_seq**: won 3/6, lost 3/6
- **ab_b0_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b0_pred | 5.0ns | 391841.6ns | 0.0% |  |
| ab_b0_prof | 4.4ns | 328418.2ns | 0.0% |  |
| ab_b0_seq | 4.9ns | 331644.1ns | 0.0% |  |
| ab_b0_table | 4.9ns | 325489.9ns | 0.0% |  |
| ab_b0_tree | 4.9ns | 327615.6ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b0_pred (n=6, range 380218.8-399340.0 ns)
  380218.8 |########################################
  381174.9 |
  382130.9 |
  383087.0 |
  384043.0 |
  384999.1 |
  385955.2 |
  386911.2 |########################################
  387867.3 |
  388823.3 |
  389779.4 |
  390735.5 |########################################
  391691.5 |
  392647.6 |########################################
  393603.6 |
  394559.7 |
  395515.8 |########################################
  396471.8 |
  397427.9 |
  398383.9 |
  (0 below, 1 above range)

ab_b0_prof (n=6, range 321644.2-337279.8 ns)
  321644.2 |########################################
  322426.0 |####################
  323207.8 |
  323989.5 |
  324771.3 |
  325553.1 |
  326334.9 |
  327116.7 |
  327898.4 |
  328680.2 |####################
  329462.0 |####################
  330243.8 |
  331025.6 |
  331807.3 |
  332589.1 |
  333370.9 |
  334152.7 |
  334934.5 |
  335716.2 |
  336498.0 |
  (0 below, 1 above range)

ab_b0_seq (n=6, range 321838.3-338679.8 ns)
  321838.3 |####################
  322680.4 |
  323522.5 |
  324364.5 |
  325206.6 |
  326048.7 |
  326890.8 |
  327732.8 |
  328574.9 |
  329417.0 |####################
  330259.0 |########################################
  331101.1 |
  331943.2 |
  332785.3 |
  333627.3 |
  334469.4 |
  335311.5 |
  336153.6 |
  336995.6 |####################
  337837.7 |
  (0 below, 1 above range)

ab_b0_table (n=6, range 319130.0-331480.7 ns)
  319130.0 |########################################
  319747.5 |
  320365.1 |########################################
  320982.6 |
  321600.1 |
  322217.7 |
  322835.2 |
  323452.7 |########################################
  324070.3 |
  324687.8 |
  325305.3 |
  325922.9 |########################################
  326540.4 |
  327157.9 |
  327775.5 |
  328393.0 |
  329010.5 |
  329628.1 |
  330245.6 |########################################
  330863.1 |
  (0 below, 1 above range)

ab_b0_tree (n=6, range 322305.0-331021.0 ns)
  322305.0 |########################################
  322740.8 |
  323176.6 |
  323612.4 |
  324048.2 |
  324484.0 |
  324919.8 |
  325355.6 |########################################
  325791.4 |
  326227.2 |
  326663.0 |########################################
  327098.8 |
  327534.6 |
  327970.4 |
  328406.2 |
  328842.0 |########################################
  329277.8 |########################################
  329713.6 |
  330149.4 |
  330585.2 |
  (0 below, 1 above range)

```

# Per-type strategy (NATIVE tier): all match

5 variants, 6 samples per variant.
Baseline: **an_match_table**

## Highlights

Baseline for all deltas below: **an_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: an_match_pred** at 319667.3 ns median (-2.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 319667.3 ns, slowest 340995.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_match_pred | 321485ns | 322297ns | 312963ns | 321644ns | 325508ns | -3.89% |
| an_match_prof | 344522ns | 343765ns | 320718ns | 338302ns | 365755ns | +3.00% |
| an_match_seq | 334832ns | 336046ns | 326479ns | 333593ns | 340868ns | +0.10% |
| an_match_table | 334500ns | 329944ns | 325034ns | 328817ns | 347756ns | base |
| an_match_tree | 337122ns | 339603ns | 325518ns | 336749ns | 343483ns | +0.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_match_pred | 318947ns | 310649ns | 322930ns | -3.89% | 0.051 |
| an_match_prof | 341862ns | 318048ns | 363236ns | +3.02% | 0.048 |
| an_match_seq | 332376ns | 323989ns | 338442ns | +0.16% | 0.049 |
| an_match_table | 331854ns | 322385ns | 345105ns | base | 0.049 |
| an_match_tree | 334597ns | 323276ns | 340871ns | +0.83% | 0.049 |

## Performance model

- Peak throughput: **0.053 Gops/s** (an_match_pred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_match_pred | 0.051 | 97.2% |
| an_match_prof | 0.048 | 91.1% |
| an_match_seq | 0.049 | 93.1% |
| an_match_table | 0.050 | 94.9% |
| an_match_tree | 0.049 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_match_pred | 321485ns | 321485ns | -3.89% |
| an_match_prof | 344522ns | 344522ns | +3.00% |
| an_match_seq | 334832ns | 334832ns | +0.10% |
| an_match_table | 334500ns | 334500ns | base |
| an_match_tree | 337122ns | 337122ns | +0.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_match_table | 327362ns | base | --- | [323096, 345105] | --- | --- | --- | --- |
| an_match_pred | 319667ns | -7430.0ns (-2.3%) | [-28816, -2476]ns | [314243, 322930] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_match_prof | 340995ns | no significant difference | [-1742, +18863]ns | [321353, 363236] | no | 0.4375 | 0.2188 | 0 |
| an_match_seq | 333509ns | no significant difference | [-13968, +12973]ns | [325178, 338442] | no | 0.6875 | 0.6875 | 0 |
| an_match_tree | 336857ns | no significant difference | [-18316, +16542]ns | [326064, 340871] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_match_table | an_match_pred | an_match_prof | an_match_seq | an_match_tree |
|---|---|---|---|---|---|
| 1 | 322385ns | -1.1% | -1.3% | +0.5% | +5.0% |
| 2 | 331437ns | -2.7% | +4.7% | +1.1% | +2.8% |
| 3 | 323806ns | -1.8% | +0.3% | +5.6% | +5.3% |
| 4 | 329988ns | -5.9% | +3.1% | -1.1% | -2.0% |
| 5 | 324736ns | -0.4% | +5.3% | +2.4% | +3.3% |
| 6 | 358772ns | -10.7% | +5.8% | -6.8% | -8.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_match_pred | -0.236 | moderate- |
| an_match_prof | -0.081 | ok |
| an_match_seq | -0.264 | moderate- |
| an_match_table | -0.167 | ok |
| an_match_tree | -0.087 | ok |

**Consistency summary:**

- **an_match_pred**: won 6/6, lost 0/6
- **an_match_prof**: won 1/6, lost 5/6
- **an_match_seq**: won 2/6, lost 4/6
- **an_match_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_match_pred | 9.2ns | 318946.9ns | 0.0% |  |
| an_match_prof | 18.3ns | 341861.7ns | 0.0% |  |
| an_match_seq | 20.5ns | 332376.5ns | 0.0% |  |
| an_match_table | 17.3ns | 331854.2ns | 0.0% |  |
| an_match_tree | 9.8ns | 334597.4ns | 0.0% |  |

## Distribution (algo ns)

```
an_match_pred (n=6, range 310648.7-322930.4 ns)
  310648.7 |########################################
  311262.8 |
  311876.9 |
  312491.0 |
  313105.0 |
  313719.1 |
  314333.2 |
  314947.3 |
  315561.4 |
  316175.5 |
  316789.6 |
  317403.6 |########################################
  318017.7 |
  318631.8 |########################################
  319245.9 |
  319860.0 |
  320474.1 |########################################
  321088.1 |
  321702.2 |
  322316.3 |########################################
  (0 below, 1 above range)

an_match_prof (n=6, range 318047.9-363236.5 ns)
  318047.9 |########################################
  320307.3 |
  322566.8 |########################################
  324826.2 |
  327085.6 |
  329345.1 |
  331604.5 |
  333863.9 |
  336123.3 |
  338382.8 |########################################
  340642.2 |########################################
  342901.6 |
  345161.1 |########################################
  347420.5 |
  349679.9 |
  351939.3 |
  354198.8 |
  356458.2 |
  358717.6 |
  360977.1 |
  (0 below, 1 above range)

an_match_seq (n=6, range 323989.2-338442.5 ns)
  323989.2 |########################################
  324711.9 |
  325434.5 |
  326157.2 |########################################
  326879.9 |
  327602.5 |
  328325.2 |
  329047.9 |
  329770.5 |
  330493.2 |
  331215.8 |
  331938.5 |########################################
  332661.2 |
  333383.8 |
  334106.5 |########################################
  334829.2 |########################################
  335551.8 |
  336274.5 |
  336997.2 |
  337719.8 |
  (0 below, 1 above range)

an_match_table (n=6, range 322385.4-345104.6 ns)
  322385.4 |########################################
  323521.4 |########################################
  324657.3 |########################################
  325793.3 |
  326929.2 |
  328065.2 |
  329201.2 |########################################
  330337.1 |########################################
  331473.1 |
  332609.0 |
  333745.0 |
  334881.0 |
  336016.9 |
  337152.9 |
  338288.8 |
  339424.8 |
  340560.8 |
  341696.7 |
  342832.7 |
  343968.6 |
  (0 below, 1 above range)

an_match_tree (n=6, range 323275.8-340871.4 ns)
  323275.8 |########################################
  324155.6 |
  325035.4 |
  325915.1 |
  326794.9 |
  327674.7 |
  328554.5 |########################################
  329434.3 |
  330314.1 |
  331193.8 |
  332073.6 |
  332953.4 |
  333833.2 |
  334713.0 |########################################
  335592.8 |
  336472.5 |
  337352.3 |
  338232.1 |########################################
  339111.9 |
  339991.7 |########################################
  (0 below, 1 above range)

```

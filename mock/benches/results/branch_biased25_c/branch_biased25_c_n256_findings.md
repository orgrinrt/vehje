# Branch strategies, cheap-arm, biased25: ~25% taken (b<64)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_biased25**

## Key findings

- **Baseline (br_branch_c_biased25) is the fastest** at 342.3 ns median
- 4 variants significantly slower than baseline
- Spread: 1.61x (fastest 342.3 ns, slowest 551.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 2699ns | 2632ns | 2488ns | 2587ns | 2971ns | base |
| br_lut_c_biased25 | 2974ns | 2944ns | 2648ns | 2848ns | 3326ns | +10.20% |
| br_mask_c_biased25 | 2919ns | 2928ns | 2655ns | 2838ns | 3172ns | +8.16% |
| br_predicate_c_biased25 | 3005ns | 2972ns | 2655ns | 2900ns | 3337ns | +11.34% |
| br_profiled_hot_c_biased25 | 2833ns | 2852ns | 2532ns | 2764ns | 3089ns | +4.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_biased25 | 352ns | 322ns | 388ns | base | 0.728 |
| br_lut_c_biased25 | 547ns | 486ns | 612ns | +55.53% | 0.468 |
| br_mask_c_biased25 | 541ns | 492ns | 590ns | +53.81% | 0.473 |
| br_predicate_c_biased25 | 571ns | 491ns | 660ns | +62.28% | 0.448 |
| br_profiled_hot_c_biased25 | 424ns | 371ns | 459ns | +20.57% | 0.604 |

## Performance model

- Peak throughput: **0.796 Gops/s** (br_branch_c_biased25; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_biased25 | 0.748 | 94.0% |
| br_lut_c_biased25 | 0.474 | 59.6% |
| br_mask_c_biased25 | 0.473 | 59.5% |
| br_predicate_c_biased25 | 0.465 | 58.4% |
| br_profiled_hot_c_biased25 | 0.597 | 75.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_biased25 | 2699ns | 2699ns | base |
| br_lut_c_biased25 | 2974ns | 2974ns | +10.20% |
| br_mask_c_biased25 | 2919ns | 2919ns | +8.16% |
| br_predicate_c_biased25 | 3005ns | 3005ns | +11.34% |
| br_profiled_hot_c_biased25 | 2833ns | 2833ns | +4.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 342ns | base | --- | [325, 388] | --- | --- | --- | --- |
| br_lut_c_biased25 | 540ns | +179.8ns (+52.5%) | [+161, +246]ns | [489, 612] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_biased25 | 541ns | +180.5ns (+52.7%) | [+164, +223]ns | [493, 590] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_biased25 | 551ns | +203.4ns (+59.4%) | [+173, +281]ns | [501, 660] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_c_biased25 | 429ns | +74.2ns (+21.7%) | [+51, +92]ns | [385, 459] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_biased25 | br_lut_c_biased25 | br_mask_c_biased25 | br_predicate_c_biased25 | br_profiled_hot_c_biased25 |
|---|---|---|---|---|---|
| 1 | 329ns | +49.5% | +49.4% | +55.2% | +23.5% |
| 2 | 399ns | +47.3% | +47.4% | +82.8% | +13.1% |
| 3 | 355ns | +65.2% | +65.4% | +65.3% | +27.7% |
| 4 | 328ns | +48.3% | +50.7% | +50.0% | +21.6% |
| 5 | 322ns | +53.1% | +53.5% | +60.0% | +15.3% |
| 6 | 378ns | +68.7% | +56.4% | +56.6% | +22.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_biased25 | -0.211 | moderate- |
| br_lut_c_biased25 | -0.220 | moderate- |
| br_mask_c_biased25 | -0.179 | ok |
| br_predicate_c_biased25 | -0.126 | ok |
| br_profiled_hot_c_biased25 | -0.168 | ok |

**Consistency summary:**

- **br_lut_c_biased25**: won 0/6, lost 6/6
- **br_mask_c_biased25**: won 0/6, lost 6/6
- **br_predicate_c_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_c_biased25**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_biased25 | 3.6ns | 351.8ns | 1.0% |  |
| br_lut_c_biased25 | 5.3ns | 547.1ns | 1.0% |  |
| br_mask_c_biased25 | 5.8ns | 541.0ns | 1.1% |  |
| br_predicate_c_biased25 | 4.0ns | 570.8ns | 0.7% |  |
| br_profiled_hot_c_biased25 | 4.7ns | 424.1ns | 1.1% |  |

## Distribution (algo ns)

```
br_branch_c_biased25 (n=6, range 321.7-388.4 ns)
    321.7 |########################################
    325.0 |########################################
    328.4 |########################################
    331.7 |
    335.0 |
    338.4 |
    341.7 |
    345.0 |
    348.4 |
    351.7 |
    355.0 |########################################
    358.4 |
    361.7 |
    365.0 |
    368.4 |
    371.7 |
    375.0 |########################################
    378.4 |
    381.7 |
    385.0 |
  (0 below, 1 above range)

br_lut_c_biased25 (n=6, range 485.8-612.5 ns)
    485.8 |########################################
    492.1 |####################
    498.5 |
    504.8 |
    511.1 |
    517.5 |
    523.8 |
    530.1 |
    536.5 |
    542.8 |
    549.1 |
    555.5 |
    561.8 |
    568.2 |
    574.5 |
    580.8 |####################
    587.2 |####################
    593.5 |
    599.8 |
    606.2 |
  (0 below, 1 above range)

br_mask_c_biased25 (n=6, range 491.7-589.5 ns)
    491.7 |########################################
    496.6 |
    501.5 |
    506.4 |
    511.3 |
    516.2 |
    521.1 |
    525.9 |
    530.8 |
    535.7 |
    540.6 |
    545.5 |
    550.4 |
    555.3 |
    560.2 |
    565.1 |
    570.0 |
    574.9 |
    579.8 |
    584.7 |##########################
  (0 below, 1 above range)

br_predicate_c_biased25 (n=6, range 491.2-660.5 ns)
    491.2 |####################
    499.7 |
    508.1 |########################################
    516.6 |
    525.0 |
    533.5 |
    542.0 |
    550.4 |
    558.9 |
    567.4 |
    575.8 |
    584.3 |########################################
    592.8 |
    601.2 |
    609.7 |
    618.1 |
    626.6 |
    635.1 |
    643.5 |
    652.0 |
  (0 below, 1 above range)

br_profiled_hot_c_biased25 (n=6, range 370.8-458.8 ns)
    370.8 |####################
    375.2 |
    379.6 |
    384.0 |
    388.4 |
    392.8 |
    397.2 |####################
    401.6 |
    406.0 |####################
    410.4 |
    414.8 |
    419.2 |
    423.6 |
    428.0 |
    432.4 |
    436.8 |
    441.2 |
    445.6 |
    450.0 |########################################
    454.4 |
  (0 below, 1 above range)

```

# Branch strategies, cheap-arm, alt: strict alternation (i&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_alt**

## Key findings

- **Baseline (br_branch_c_alt) is the fastest** at 349.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.81x (fastest 349.6 ns, slowest 631.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_alt | 2840ns | 2930ns | 2558ns | 2870ns | 2938ns | base |
| br_lut_c_alt | 2958ns | 2955ns | 2657ns | 2862ns | 3253ns | +4.14% |
| br_mask_c_alt | 3129ns | 3136ns | 2724ns | 3043ns | 3460ns | +10.15% |
| br_predicate_c_alt | 2920ns | 2881ns | 2611ns | 2841ns | 3192ns | +2.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_alt | 336ns | 306ns | 352ns | base | 0.762 |
| br_lut_c_alt | 542ns | 491ns | 586ns | +61.38% | 0.472 |
| br_mask_c_alt | 614ns | 548ns | 651ns | +82.78% | 0.417 |
| br_predicate_c_alt | 540ns | 486ns | 586ns | +60.86% | 0.474 |

## Performance model

- Peak throughput: **0.837 Gops/s** (br_branch_c_alt; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_alt | 0.732 | 87.5% |
| br_lut_c_alt | 0.469 | 56.0% |
| br_mask_c_alt | 0.405 | 48.4% |
| br_predicate_c_alt | 0.478 | 57.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_alt | 2840ns | 2840ns | base |
| br_lut_c_alt | 2958ns | 2958ns | +4.14% |
| br_mask_c_alt | 3129ns | 3129ns | +10.15% |
| br_predicate_c_alt | 2920ns | 2920ns | +2.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_alt | 350ns | base | --- | [306, 352] | --- | --- | --- | --- |
| br_lut_c_alt | 546ns | +212.5ns (+60.8%) | [+170, +236]ns | [494, 586] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_alt | 631ns | +282.3ns (+80.7%) | [+250, +302]ns | [559, 651] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_alt | 535ns | +209.8ns (+60.0%) | [+167, +236]ns | [499, 586] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_alt | br_lut_c_alt | br_mask_c_alt | br_predicate_c_alt |
|---|---|---|---|---|
| 1 | 353ns | +43.9% | +73.6% | +37.7% |
| 2 | 351ns | +67.2% | +85.4% | +67.2% |
| 3 | 349ns | +67.6% | +86.7% | +67.9% |
| 4 | 350ns | +66.8% | +85.8% | +57.5% |
| 5 | 307ns | +62.3% | +78.5% | +67.1% |
| 6 | 306ns | +60.5% | +86.7% | +69.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_alt | 0.418 | moderate+ |
| br_lut_c_alt | 0.232 | moderate+ |
| br_mask_c_alt | 0.305 | moderate+ |
| br_predicate_c_alt | 0.043 | ok |

**Consistency summary:**

- **br_lut_c_alt**: won 0/6, lost 6/6
- **br_mask_c_alt**: won 0/6, lost 6/6
- **br_predicate_c_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_alt | 4.4ns | 335.9ns | 1.3% |  |
| br_lut_c_alt | 5.5ns | 542.1ns | 1.0% |  |
| br_mask_c_alt | 3.8ns | 613.9ns | 0.6% |  |
| br_predicate_c_alt | 4.9ns | 540.3ns | 0.9% |  |

## Distribution (algo ns)

```
br_branch_c_alt (n=6, range 305.8-351.9 ns)
    305.8 |########################################
    308.1 |
    310.4 |
    312.7 |
    315.0 |
    317.3 |
    319.6 |
    321.9 |
    324.2 |
    326.5 |
    328.8 |
    331.1 |
    333.4 |
    335.7 |
    338.0 |
    340.3 |
    342.6 |
    344.9 |
    347.2 |####################
    349.5 |########################################
  (0 below, 1 above range)

br_lut_c_alt (n=6, range 490.8-586.0 ns)
    490.8 |####################
    495.6 |####################
    500.3 |
    505.1 |####################
    509.9 |
    514.6 |
    519.4 |
    524.1 |
    528.9 |
    533.7 |
    538.4 |
    543.2 |
    547.9 |
    552.7 |
    557.5 |
    562.2 |
    567.0 |
    571.8 |
    576.5 |
    581.3 |########################################
  (0 below, 1 above range)

br_mask_c_alt (n=6, range 547.5-651.2 ns)
    547.5 |####################
    552.7 |
    557.9 |
    563.1 |
    568.2 |####################
    573.4 |
    578.6 |
    583.8 |
    589.0 |
    594.2 |
    599.4 |
    604.6 |
    609.8 |####################
    614.9 |
    620.1 |
    625.3 |
    630.5 |
    635.7 |
    640.9 |
    646.1 |########################################
  (0 below, 1 above range)

br_predicate_c_alt (n=6, range 485.8-586.5 ns)
    485.8 |########################################
    490.8 |
    495.9 |
    500.9 |
    505.9 |
    511.0 |########################################
    516.0 |########################################
    521.0 |
    526.1 |
    531.1 |
    536.1 |
    541.2 |
    546.2 |########################################
    551.2 |
    556.3 |
    561.3 |
    566.3 |
    571.4 |
    576.4 |
    581.4 |########################################
  (0 below, 1 above range)

```

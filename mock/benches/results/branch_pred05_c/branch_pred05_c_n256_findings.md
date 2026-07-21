# Branch strategies, cheap-arm, pred05: ~5% taken, predictable (b<13)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred05**

## Key findings

- **Baseline (br_branch_c_pred05) is the fastest** at 407.2 ns median
- 4 variants significantly slower than baseline
- Spread: 1.51x (fastest 407.2 ns, slowest 614.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 2954ns | 2958ns | 2487ns | 2896ns | 3274ns | base |
| br_lut_c_pred05 | 3114ns | 3162ns | 2957ns | 3128ns | 3171ns | +5.42% |
| br_mask_c_pred05 | 3201ns | 3191ns | 2646ns | 3119ns | 3601ns | +8.37% |
| br_predicate_c_pred05 | 3373ns | 3306ns | 3163ns | 3261ns | 3647ns | +14.20% |
| br_profiled_hot_c_pred05 | 3013ns | 3016ns | 2519ns | 3012ns | 3262ns | +2.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred05 | 402ns | 328ns | 450ns | base | 0.637 |
| br_lut_c_pred05 | 575ns | 549ns | 588ns | +43.11% | 0.445 |
| br_mask_c_pred05 | 595ns | 496ns | 670ns | +48.05% | 0.431 |
| br_predicate_c_pred05 | 625ns | 588ns | 671ns | +55.52% | 0.410 |
| br_profiled_hot_c_pred05 | 431ns | 358ns | 469ns | +7.23% | 0.594 |

## Performance model

- Peak throughput: **0.780 Gops/s** (br_branch_c_pred05; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred05 | 0.629 | 80.6% |
| br_lut_c_pred05 | 0.438 | 56.2% |
| br_mask_c_pred05 | 0.434 | 55.6% |
| br_predicate_c_pred05 | 0.417 | 53.5% |
| br_profiled_hot_c_pred05 | 0.585 | 75.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred05 | 2954ns | 2954ns | base |
| br_lut_c_pred05 | 3114ns | 3114ns | +5.42% |
| br_mask_c_pred05 | 3201ns | 3201ns | +8.37% |
| br_predicate_c_pred05 | 3373ns | 3373ns | +14.20% |
| br_profiled_hot_c_pred05 | 3013ns | 3013ns | +2.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 407ns | base | --- | [348, 450] | --- | --- | --- | --- |
| br_lut_c_pred05 | 584ns | +164.1ns (+40.3%) | [+130, +225]ns | [553, 588] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_pred05 | 590ns | +200.0ns (+49.1%) | [+129, +250]ns | [524, 670] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_pred05 | 614ns | +231.5ns (+56.8%) | [+175, +262]ns | [589, 671] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_c_pred05 | 437ns | +30.4ns (+7.5%) | [+17, +40]ns | [386, 469] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred05 | br_lut_c_pred05 | br_mask_c_pred05 | br_predicate_c_pred05 | br_profiled_hot_c_pred05 |
|---|---|---|---|---|---|
| 1 | 328ns | +69.4% | +51.1% | +80.0% | +9.0% |
| 2 | 413ns | +32.9% | +43.5% | +42.1% | +6.6% |
| 3 | 439ns | +32.7% | +59.7% | +59.7% | +7.1% |
| 4 | 460ns | +27.2% | +19.6% | +38.3% | +1.5% |
| 5 | 401ns | +46.0% | +59.2% | +59.5% | +8.2% |
| 6 | 368ns | +60.3% | +60.0% | +61.0% | +12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred05 | 0.154 | ok |
| br_lut_c_pred05 | 0.418 | moderate+ |
| br_mask_c_pred05 | -0.277 | moderate- |
| br_predicate_c_pred05 | -0.100 | ok |
| br_profiled_hot_c_pred05 | 0.139 | ok |

**Consistency summary:**

- **br_lut_c_pred05**: won 0/6, lost 6/6
- **br_mask_c_pred05**: won 0/6, lost 6/6
- **br_predicate_c_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred05**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred05 | 4.8ns | 401.7ns | 1.2% |  |
| br_lut_c_pred05 | 4.2ns | 574.8ns | 0.7% |  |
| br_mask_c_pred05 | 4.6ns | 594.6ns | 0.8% |  |
| br_predicate_c_pred05 | 4.4ns | 624.6ns | 0.7% |  |
| br_profiled_hot_c_pred05 | 4.5ns | 430.7ns | 1.0% |  |

## Distribution (algo ns)

```
br_branch_c_pred05 (n=6, range 328.3-449.8 ns)
    328.3 |########################################
    334.4 |
    340.4 |
    346.5 |
    352.6 |
    358.7 |
    364.8 |########################################
    370.8 |
    376.9 |
    383.0 |
    389.0 |
    395.1 |
    401.2 |########################################
    407.3 |########################################
    413.3 |
    419.4 |
    425.5 |
    431.6 |
    437.6 |########################################
    443.7 |
  (0 below, 1 above range)

br_lut_c_pred05 (n=6, range 549.2-587.5 ns)
    549.2 |########################################
    551.1 |
    553.0 |
    554.9 |########################################
    556.9 |
    558.8 |
    560.7 |
    562.6 |
    564.5 |
    566.4 |
    568.4 |
    570.3 |
    572.2 |
    574.1 |
    576.0 |
    577.9 |
    579.8 |
    581.8 |########################################
    583.7 |########################################
    585.6 |########################################
  (0 below, 1 above range)

br_mask_c_pred05 (n=6, range 496.2-670.0 ns)
    496.2 |########################################
    504.9 |
    513.6 |
    522.3 |
    531.0 |
    539.6 |
    548.3 |########################################
    557.0 |
    565.7 |
    574.4 |
    583.1 |########################################
    591.8 |########################################
    600.5 |
    609.2 |
    617.9 |
    626.5 |
    635.2 |########################################
    643.9 |
    652.6 |
    661.3 |
  (0 below, 1 above range)

br_predicate_c_pred05 (n=6, range 587.5-670.6 ns)
    587.5 |########################################
    591.7 |####################
    595.8 |
    600.0 |
    604.1 |
    608.3 |
    612.4 |
    616.6 |
    620.7 |
    624.9 |
    629.0 |
    633.2 |####################
    637.4 |####################
    641.5 |
    645.7 |
    649.8 |
    654.0 |
    658.1 |
    662.3 |
    666.4 |
  (0 below, 1 above range)

br_profiled_hot_c_pred05 (n=6, range 357.9-468.8 ns)
    357.9 |########################################
    363.4 |
    369.0 |
    374.5 |
    380.1 |
    385.6 |
    391.2 |
    396.7 |
    402.2 |
    407.8 |
    413.3 |########################################
    418.9 |
    424.4 |
    430.0 |########################################
    435.5 |########################################
    441.0 |
    446.6 |
    452.1 |
    457.7 |
    463.2 |########################################
  (0 below, 1 above range)

```

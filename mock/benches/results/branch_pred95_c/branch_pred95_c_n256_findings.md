# Branch strategies, cheap-arm, pred95: ~95% taken, predictable (b<243)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred95**

## Key findings

- **Baseline (br_branch_c_pred95) is the fastest** at 400.0 ns median
- 3 variants significantly slower than baseline
- Spread: 1.58x (fastest 400.0 ns, slowest 633.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 2974ns | 3105ns | 2516ns | 2911ns | 3298ns | base |
| br_lut_c_pred95 | 3201ns | 3435ns | 2694ns | 3214ns | 3436ns | +7.65% |
| br_mask_c_pred95 | 3042ns | 2979ns | 2661ns | 2904ns | 3441ns | +2.30% |
| br_predicate_c_pred95 | 3104ns | 3167ns | 2654ns | 3033ns | 3436ns | +4.38% |
| br_profiled_hot_c_pred95 | 2914ns | 3008ns | 2450ns | 2844ns | 3252ns | -2.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred95 | 397ns | 343ns | 444ns | base | 0.644 |
| br_lut_c_pred95 | 592ns | 495ns | 639ns | +49.03% | 0.432 |
| br_mask_c_pred95 | 565ns | 495ns | 641ns | +42.28% | 0.453 |
| br_predicate_c_pred95 | 579ns | 495ns | 640ns | +45.77% | 0.442 |
| br_profiled_hot_c_pred95 | 398ns | 322ns | 446ns | +0.04% | 0.644 |

## Performance model

- Peak throughput: **0.795 Gops/s** (br_profiled_hot_c_pred95; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred95 | 0.640 | 80.5% |
| br_lut_c_pred95 | 0.404 | 50.9% |
| br_mask_c_pred95 | 0.465 | 58.5% |
| br_predicate_c_pred95 | 0.433 | 54.4% |
| br_profiled_hot_c_pred95 | 0.622 | 78.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred95 | 2974ns | 2974ns | base |
| br_lut_c_pred95 | 3201ns | 3201ns | +7.65% |
| br_mask_c_pred95 | 3042ns | 3042ns | +2.30% |
| br_predicate_c_pred95 | 3104ns | 3104ns | +4.38% |
| br_profiled_hot_c_pred95 | 2914ns | 2914ns | -2.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 400ns | base | --- | [348, 444] | --- | --- | --- | --- |
| br_lut_c_pred95 | 633ns | +193.0ns (+48.2%) | [+156, +235]ns | [505, 639] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred95 | 550ns | +163.3ns (+40.8%) | [+143, +197]ns | [504, 641] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred95 | 592ns | +174.8ns (+43.7%) | [+153, +218]ns | [506, 640] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred95 | 412ns | no significant difference | [-45, +45]ns | [335, 446] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred95 | br_lut_c_pred95 | br_mask_c_pred95 | br_predicate_c_pred95 | br_profiled_hot_c_pred95 |
|---|---|---|---|---|---|
| 1 | 396ns | +61.3% | +39.5% | +61.8% | -18.6% |
| 2 | 450ns | +42.3% | +42.1% | +42.4% | -3.8% |
| 3 | 404ns | +56.3% | +35.8% | +46.2% | +2.1% |
| 4 | 353ns | +40.2% | +40.2% | +46.1% | -1.6% |
| 5 | 343ns | +49.8% | +49.7% | +44.4% | +19.9% |
| 6 | 438ns | +44.7% | +46.9% | +35.2% | +4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred95 | 0.015 | ok |
| br_lut_c_pred95 | 0.195 | ok |
| br_mask_c_pred95 | -0.075 | ok |
| br_predicate_c_pred95 | 0.419 | moderate+ |
| br_profiled_hot_c_pred95 | -0.198 | ok |

**Consistency summary:**

- **br_lut_c_pred95**: won 0/6, lost 6/6
- **br_mask_c_pred95**: won 0/6, lost 6/6
- **br_predicate_c_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred95**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred95 | 5.5ns | 397.3ns | 1.4% |  |
| br_lut_c_pred95 | 4.8ns | 592.2ns | 0.8% |  |
| br_mask_c_pred95 | 4.4ns | 565.3ns | 0.8% |  |
| br_predicate_c_pred95 | 4.6ns | 579.2ns | 0.8% |  |
| br_profiled_hot_c_pred95 | 4.6ns | 397.5ns | 1.1% |  |

## Distribution (algo ns)

```
br_branch_c_pred95 (n=6, range 342.9-444.0 ns)
    342.9 |########################################
    348.0 |
    353.0 |########################################
    358.1 |
    363.1 |
    368.2 |
    373.2 |
    378.3 |
    383.3 |
    388.4 |
    393.4 |########################################
    398.5 |
    403.5 |########################################
    408.6 |
    413.6 |
    418.7 |
    423.7 |
    428.8 |
    433.8 |########################################
    438.9 |
  (0 below, 1 above range)

br_lut_c_pred95 (n=6, range 495.4-639.0 ns)
    495.4 |####################
    502.6 |
    509.8 |####################
    516.9 |
    524.1 |
    531.3 |
    538.5 |
    545.6 |
    552.8 |
    560.0 |
    567.2 |
    574.4 |
    581.5 |
    588.7 |
    595.9 |
    603.1 |
    610.2 |
    617.4 |
    624.6 |####################
    631.8 |########################################
  (0 below, 1 above range)

br_mask_c_pred95 (n=6, range 495.4-641.2 ns)
    495.4 |####################
    502.7 |
    510.0 |####################
    517.3 |
    524.6 |
    531.9 |
    539.2 |
    546.4 |########################################
    553.7 |
    561.0 |
    568.3 |
    575.6 |
    582.9 |
    590.2 |
    597.5 |
    604.8 |
    612.1 |
    619.4 |
    626.7 |
    634.0 |####################
  (0 below, 1 above range)

br_predicate_c_pred95 (n=6, range 495.0-640.4 ns)
    495.0 |####################
    502.3 |
    509.5 |####################
    516.8 |
    524.1 |
    531.4 |
    538.6 |
    545.9 |
    553.2 |
    560.4 |
    567.7 |
    575.0 |
    582.2 |
    589.5 |########################################
    596.8 |
    604.0 |
    611.3 |
    618.6 |
    625.9 |
    633.1 |
  (0 below, 2 above range)

br_profiled_hot_c_pred95 (n=6, range 322.1-445.9 ns)
    322.1 |####################
    328.3 |
    334.5 |
    340.7 |
    346.9 |####################
    353.0 |
    359.2 |
    365.4 |
    371.6 |
    377.8 |
    384.0 |
    390.2 |
    396.4 |
    402.5 |
    408.7 |########################################
    414.9 |
    421.1 |
    427.3 |####################
    433.5 |
    439.7 |
  (0 below, 1 above range)

```

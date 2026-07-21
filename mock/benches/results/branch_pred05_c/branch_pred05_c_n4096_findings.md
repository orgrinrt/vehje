# Branch strategies, cheap-arm, pred05: ~5% taken, predictable (b<13)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred05**

## Key findings

- **Baseline (br_branch_c_pred05) is the fastest** at 6367.3 ns median
- 3 variants significantly slower than baseline
- Spread: 1.41x (fastest 6367.3 ns, slowest 8996.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 8730ns | 8874ns | 7677ns | 8504ns | 9594ns | base |
| br_lut_c_pred05 | 11034ns | 11588ns | 9540ns | 11084ns | 11705ns | +26.40% |
| br_mask_c_pred05 | 11286ns | 11581ns | 9724ns | 11578ns | 11629ns | +29.28% |
| br_predicate_c_pred05 | 11269ns | 11239ns | 9542ns | 11104ns | 12379ns | +29.09% |
| br_profiled_hot_c_pred05 | 8828ns | 9202ns | 7711ns | 8829ns | 9385ns | +1.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred05 | 6269ns | 5507ns | 6890ns | base | 0.653 |
| br_lut_c_pred05 | 8574ns | 7409ns | 9111ns | +36.77% | 0.478 |
| br_mask_c_pred05 | 8753ns | 7546ns | 8999ns | +39.62% | 0.468 |
| br_predicate_c_pred05 | 8766ns | 7410ns | 9669ns | +39.84% | 0.467 |
| br_profiled_hot_c_pred05 | 6362ns | 5541ns | 6786ns | +1.48% | 0.644 |

## Performance model

- Peak throughput: **0.744 Gops/s** (br_branch_c_pred05; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred05 | 0.643 | 86.5% |
| br_lut_c_pred05 | 0.455 | 61.2% |
| br_mask_c_pred05 | 0.456 | 61.2% |
| br_predicate_c_pred05 | 0.470 | 63.1% |
| br_profiled_hot_c_pred05 | 0.619 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred05 | 8730ns | 8730ns | base |
| br_lut_c_pred05 | 11034ns | 11034ns | +26.40% |
| br_mask_c_pred05 | 11286ns | 11286ns | +29.28% |
| br_predicate_c_pred05 | 11269ns | 11269ns | +29.09% |
| br_profiled_hot_c_pred05 | 8828ns | 8828ns | +1.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 6367ns | base | --- | [5550, 6890] | --- | --- | --- | --- |
| br_lut_c_pred05 | 8997ns | +2261.6ns (+35.5%) | [+1910, +2744]ns | [7614, 9111] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred05 | 8992ns | +2318.1ns (+36.4%) | [+1939, +3194]ns | [8267, 8999] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred05 | 8722ns | +2387.2ns (+37.5%) | [+1834, +3271]ns | [7907, 9669] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred05 | 6616ns | no significant difference | [-191, +355]ns | [5684, 6786] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred05 | br_lut_c_pred05 | br_mask_c_pred05 | br_predicate_c_pred05 | br_profiled_hot_c_pred05 |
|---|---|---|---|---|---|
| 1 | 5592ns | +39.8% | +34.9% | +51.2% | +4.2% |
| 2 | 5507ns | +34.5% | +63.4% | +34.6% | +0.6% |
| 3 | 6702ns | +34.3% | +34.1% | +45.6% | +2.0% |
| 4 | 6640ns | +35.6% | +35.4% | +26.6% | +1.4% |
| 5 | 6095ns | +51.2% | +47.6% | +57.2% | +7.8% |
| 6 | 7077ns | +27.1% | +27.2% | +27.0% | -5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred05 | 0.069 | ok |
| br_lut_c_pred05 | 0.387 | moderate+ |
| br_mask_c_pred05 | -0.036 | ok |
| br_predicate_c_pred05 | -0.371 | moderate- |
| br_profiled_hot_c_pred05 | 0.251 | moderate+ |

**Consistency summary:**

- **br_lut_c_pred05**: won 0/6, lost 6/6
- **br_mask_c_pred05**: won 0/6, lost 6/6
- **br_predicate_c_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred05**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred05 | 3.5ns | 6268.8ns | 0.1% |  |
| br_lut_c_pred05 | 3.9ns | 8574.1ns | 0.0% |  |
| br_mask_c_pred05 | 3.3ns | 8752.5ns | 0.0% |  |
| br_predicate_c_pred05 | 3.7ns | 8766.0ns | 0.0% |  |
| br_profiled_hot_c_pred05 | 3.7ns | 6361.9ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_c_pred05 (n=6, range 5506.7-6889.6 ns)
   5506.7 |########################################
   5575.8 |########################################
   5645.0 |
   5714.1 |
   5783.3 |
   5852.4 |
   5921.6 |
   5990.7 |
   6059.9 |########################################
   6129.0 |
   6198.1 |
   6267.3 |
   6336.4 |
   6405.6 |
   6474.7 |
   6543.9 |
   6613.0 |########################################
   6682.2 |########################################
   6751.3 |
   6820.5 |
  (0 below, 1 above range)

br_lut_c_pred05 (n=6, range 7408.8-9111.0 ns)
   7408.8 |#############
   7493.9 |
   7579.0 |
   7664.1 |
   7749.2 |#############
   7834.4 |
   7919.5 |
   8004.6 |
   8089.7 |
   8174.8 |
   8259.9 |
   8345.0 |
   8430.1 |
   8515.2 |
   8600.3 |
   8685.5 |
   8770.6 |
   8855.7 |
   8940.8 |########################################
   9025.9 |
  (0 below, 1 above range)

br_mask_c_pred05 (n=6, range 7545.8-8999.2 ns)
   7545.8 |##########
   7618.5 |
   7691.1 |
   7763.8 |
   7836.5 |
   7909.1 |
   7981.8 |
   8054.5 |
   8127.1 |
   8199.8 |
   8272.5 |
   8345.1 |
   8417.8 |
   8490.5 |
   8563.1 |
   8635.8 |
   8708.5 |
   8781.1 |
   8853.8 |
   8926.5 |########################################
  (0 below, 1 above range)

br_predicate_c_pred05 (n=6, range 7410.0-9669.1 ns)
   7410.0 |########################################
   7523.0 |
   7635.9 |
   7748.9 |
   7861.8 |
   7974.8 |
   8087.7 |
   8200.7 |
   8313.7 |########################################
   8426.6 |########################################
   8539.6 |
   8652.5 |
   8765.5 |
   8878.4 |########################################
   8991.4 |
   9104.4 |
   9217.3 |
   9330.3 |
   9443.2 |
   9556.2 |########################################
  (0 below, 1 above range)

br_profiled_hot_c_pred05 (n=6, range 5540.8-6785.6 ns)
   5540.8 |########################################
   5603.0 |
   5665.3 |
   5727.5 |
   5789.8 |########################################
   5852.0 |
   5914.3 |
   5976.5 |
   6038.7 |
   6101.0 |
   6163.2 |
   6225.5 |
   6287.7 |
   6350.0 |
   6412.2 |
   6474.4 |
   6536.7 |########################################
   6598.9 |
   6661.2 |########################################
   6723.4 |########################################
  (0 below, 1 above range)

```

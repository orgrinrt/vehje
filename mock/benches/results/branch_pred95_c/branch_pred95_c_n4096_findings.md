# Branch strategies, cheap-arm, pred95: ~95% taken, predictable (b<243)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred95**

## Key findings

- **Fastest: br_profiled_hot_c_pred95** at 5680.0 ns median (-9.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.58x (fastest 5680.0 ns, slowest 8987.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 8596ns | 8780ns | 7732ns | 8477ns | 9208ns | base |
| br_lut_c_pred95 | 11854ns | 11554ns | 9698ns | 11314ns | 13743ns | +37.90% |
| br_mask_c_pred95 | 11155ns | 11589ns | 9699ns | 11335ns | 11611ns | +29.76% |
| br_predicate_c_pred95 | 11008ns | 11215ns | 9714ns | 10753ns | 12038ns | +28.06% |
| br_profiled_hot_c_pred95 | 8133ns | 8059ns | 7249ns | 7806ns | 9064ns | -5.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred95 | 6087ns | 5478ns | 6459ns | base | 0.673 |
| br_lut_c_pred95 | 9193ns | 7534ns | 10683ns | +51.02% | 0.446 |
| br_mask_c_pred95 | 8653ns | 7525ns | 9005ns | +42.15% | 0.473 |
| br_predicate_c_pred95 | 8542ns | 7535ns | 9335ns | +40.32% | 0.480 |
| br_profiled_hot_c_pred95 | 5729ns | 5118ns | 6385ns | -5.90% | 0.715 |

## Performance model

- Peak throughput: **0.800 Gops/s** (br_profiled_hot_c_pred95; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred95 | 0.655 | 81.8% |
| br_lut_c_pred95 | 0.459 | 57.3% |
| br_mask_c_pred95 | 0.456 | 56.9% |
| br_predicate_c_pred95 | 0.471 | 58.8% |
| br_profiled_hot_c_pred95 | 0.721 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred95 | 8596ns | 8596ns | base |
| br_lut_c_pred95 | 11854ns | 11854ns | +37.90% |
| br_mask_c_pred95 | 11155ns | 11155ns | +29.76% |
| br_predicate_c_pred95 | 11008ns | 11008ns | +28.06% |
| br_profiled_hot_c_pred95 | 8133ns | 8133ns | -5.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 6255ns | base | --- | [5549, 6459] | --- | --- | --- | --- |
| br_lut_c_pred95 | 8925ns | +2701.9ns (+43.2%) | [+2026, +4589]ns | [7970, 10683] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred95 | 8988ns | +2558.1ns (+40.9%) | [+2008, +3131]ns | [7967, 9005] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred95 | 8703ns | +2374.4ns (+38.0%) | [+1942, +3047]ns | [7588, 9335] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred95 | 5680ns | no significant difference | [-877, +76]ns | [5121, 6385] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred95 | br_lut_c_pred95 | br_mask_c_pred95 | br_predicate_c_pred95 | br_profiled_hot_c_pred95 |
|---|---|---|---|---|---|
| 1 | 6440ns | +92.2% | +30.6% | +30.6% | -20.5% |
| 2 | 6098ns | +47.4% | +47.5% | +47.9% | +1.2% |
| 3 | 6477ns | +38.8% | +38.8% | +48.9% | +1.2% |
| 4 | 5621ns | +57.7% | +59.9% | +34.1% | -7.7% |
| 5 | 5478ns | +37.5% | +37.4% | +39.5% | -6.5% |
| 6 | 6411ns | +31.1% | +40.6% | +40.3% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred95 | -0.089 | ok |
| br_lut_c_pred95 | 0.096 | ok |
| br_mask_c_pred95 | -0.356 | moderate- |
| br_predicate_c_pred95 | -0.042 | ok |
| br_profiled_hot_c_pred95 | -0.148 | ok |

**Consistency summary:**

- **br_lut_c_pred95**: won 0/6, lost 6/6
- **br_mask_c_pred95**: won 0/6, lost 6/6
- **br_predicate_c_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred95**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred95 | 3.2ns | 6087.5ns | 0.1% |  |
| br_lut_c_pred95 | 6.4ns | 9193.1ns | 0.1% |  |
| br_mask_c_pred95 | 2.8ns | 8653.4ns | 0.0% |  |
| br_predicate_c_pred95 | 3.7ns | 8542.0ns | 0.0% |  |
| br_profiled_hot_c_pred95 | 3.8ns | 5728.5ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_c_pred95 (n=6, range 5477.5-6458.8 ns)
   5477.5 |####################
   5526.6 |
   5575.6 |####################
   5624.7 |
   5673.8 |
   5722.8 |
   5771.9 |
   5820.9 |
   5870.0 |
   5919.1 |
   5968.1 |
   6017.2 |
   6066.2 |####################
   6115.3 |
   6164.4 |
   6213.4 |
   6262.5 |
   6311.6 |
   6360.6 |
   6409.7 |########################################
  (0 below, 1 above range)

br_lut_c_pred95 (n=6, range 7534.2-10683.4 ns)
   7534.2 |####################
   7691.7 |
   7849.1 |
   8006.6 |
   8164.0 |
   8321.5 |####################
   8478.9 |
   8636.4 |
   8793.9 |####################
   8951.3 |########################################
   9108.8 |
   9266.2 |
   9423.7 |
   9581.1 |
   9738.6 |
   9896.1 |
  10053.5 |
  10211.0 |
  10368.4 |
  10525.9 |
  (0 below, 1 above range)

br_mask_c_pred95 (n=6, range 7525.4-9005.0 ns)
   7525.4 |#############
   7599.4 |
   7673.4 |
   7747.3 |
   7821.3 |
   7895.3 |
   7969.3 |
   8043.3 |
   8117.2 |
   8191.2 |
   8265.2 |
   8339.2 |#############
   8413.2 |
   8487.1 |
   8561.1 |
   8635.1 |
   8709.1 |
   8783.1 |
   8857.0 |
   8931.0 |########################################
  (0 below, 1 above range)

br_predicate_c_pred95 (n=6, range 7535.0-9334.6 ns)
   7535.0 |####################
   7625.0 |####################
   7715.0 |
   7804.9 |
   7894.9 |
   7984.9 |
   8074.9 |
   8164.9 |
   8254.8 |
   8344.8 |####################
   8434.8 |
   8524.8 |
   8614.8 |
   8704.7 |
   8794.7 |
   8884.7 |
   8974.7 |########################################
   9064.7 |
   9154.6 |
   9244.6 |
  (0 below, 1 above range)

br_profiled_hot_c_pred95 (n=6, range 5117.9-6385.0 ns)
   5117.9 |########################################
   5181.3 |####################
   5244.6 |
   5308.0 |
   5371.3 |
   5434.7 |
   5498.0 |
   5561.4 |
   5624.7 |
   5688.1 |
   5751.4 |
   5814.8 |
   5878.2 |
   5941.5 |
   6004.9 |
   6068.2 |
   6131.6 |####################
   6194.9 |####################
   6258.3 |
   6321.6 |
  (0 below, 1 above range)

```

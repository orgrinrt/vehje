# Branch strategies, heavy-arm, biased25: ~25% taken (b<64)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_biased25**

## Key findings

- **Baseline (br_branch_h_biased25) is the fastest** at 537.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.31x (fastest 537.9 ns, slowest 1241.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 3072ns | 3133ns | 2718ns | 3074ns | 3245ns | base |
| br_predicate_h_biased25 | 3852ns | 3839ns | 3498ns | 3834ns | 4056ns | +25.42% |
| br_profiled_hot_h_biased25 | 3181ns | 3219ns | 2695ns | 3169ns | 3442ns | +3.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_biased25 | 535ns | 473ns | 570ns | base | 0.120 |
| br_predicate_h_biased25 | 1230ns | 1066ns | 1296ns | +129.74% | 0.052 |
| br_profiled_hot_h_biased25 | 627ns | 528ns | 667ns | +17.20% | 0.102 |

## Performance model

- Peak throughput: **0.135 Gops/s** (br_branch_h_biased25; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_biased25 | 0.119 | 87.9% |
| br_predicate_h_biased25 | 0.052 | 38.1% |
| br_profiled_hot_h_biased25 | 0.098 | 72.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_biased25 | 3072ns | 3072ns | base |
| br_predicate_h_biased25 | 3852ns | 3852ns | +25.42% |
| br_profiled_hot_h_biased25 | 3181ns | 3181ns | +3.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 538ns | base | --- | [498, 570] | --- | --- | --- | --- |
| br_predicate_h_biased25 | 1241ns | +695.2ns (+129.2%) | [+627, +761]ns | [1152, 1296] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_biased25 | 651ns | +88.1ns (+16.4%) | [+63, +125]ns | [564, 667] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_biased25 | br_predicate_h_biased25 | br_profiled_hot_h_biased25 |
|---|---|---|---|
| 1 | 473ns | +125.4% | +11.7% |
| 2 | 530ns | +133.4% | +13.2% |
| 3 | 522ns | +137.6% | +27.0% |
| 4 | 559ns | +122.1% | +19.7% |
| 5 | 582ns | +113.7% | +14.3% |
| 6 | 545ns | +147.5% | +17.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_biased25 | 0.236 | moderate+ |
| br_predicate_h_biased25 | 0.015 | ok |
| br_profiled_hot_h_biased25 | 0.344 | moderate+ |

**Consistency summary:**

- **br_predicate_h_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_h_biased25**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_biased25 | 3.5ns | 535.3ns | 0.6% |  |
| br_predicate_h_biased25 | 3.4ns | 1229.8ns | 0.3% |  |
| br_profiled_hot_h_biased25 | 4.7ns | 627.4ns | 0.8% |  |

## Distribution (algo ns)

```
br_branch_h_biased25 (n=6, range 472.9-570.5 ns)
    472.9 |########################################
    477.8 |
    482.7 |
    487.5 |
    492.4 |
    497.3 |
    502.2 |
    507.0 |
    511.9 |
    516.8 |
    521.7 |########################################
    526.6 |########################################
    531.4 |
    536.3 |
    541.2 |########################################
    546.1 |
    550.9 |
    555.8 |########################################
    560.7 |
    565.6 |
  (0 below, 1 above range)

br_predicate_h_biased25 (n=6, range 1065.8-1296.2 ns)
   1065.8 |#############
   1077.3 |
   1088.8 |
   1100.4 |
   1111.9 |
   1123.4 |
   1134.9 |
   1146.5 |
   1158.0 |
   1169.5 |
   1181.0 |
   1192.5 |
   1204.1 |
   1215.6 |
   1227.1 |#############
   1238.6 |########################################
   1250.2 |
   1261.7 |
   1273.2 |
   1284.7 |
  (0 below, 1 above range)

br_profiled_hot_h_biased25 (n=6, range 528.3-666.9 ns)
    528.3 |####################
    535.2 |
    542.2 |
    549.1 |
    556.0 |
    563.0 |
    569.9 |
    576.8 |
    583.7 |
    590.7 |
    597.6 |####################
    604.5 |
    611.5 |
    618.4 |
    625.3 |
    632.2 |####################
    639.2 |
    646.1 |
    653.0 |
    660.0 |########################################
  (0 below, 1 above range)

```

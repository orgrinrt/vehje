# Branch strategies, heavy-arm, pred95: ~95% taken, predictable (b<243)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred95**

## Key findings

- **Baseline (br_branch_h_pred95) is the fastest** at 751.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.66x (fastest 751.2 ns, slowest 1243.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 3343ns | 3329ns | 3143ns | 3284ns | 3531ns | base |
| br_predicate_h_pred95 | 3857ns | 3840ns | 3583ns | 3833ns | 4030ns | +15.38% |
| br_profiled_hot_h_pred95 | 3509ns | 3621ns | 2880ns | 3552ns | 3759ns | +4.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred95 | 758ns | 714ns | 804ns | base | 0.084 |
| br_predicate_h_pred95 | 1258ns | 1158ns | 1330ns | +65.88% | 0.051 |
| br_profiled_hot_h_pred95 | 826ns | 674ns | 916ns | +8.91% | 0.078 |

## Performance model

- Peak throughput: **0.095 Gops/s** (br_profiled_hot_h_pred95; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred95 | 0.085 | 89.7% |
| br_predicate_h_pred95 | 0.051 | 54.2% |
| br_profiled_hot_h_pred95 | 0.075 | 79.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred95 | 3343ns | 3343ns | base |
| br_predicate_h_pred95 | 3857ns | 3857ns | +15.38% |
| br_profiled_hot_h_pred95 | 3509ns | 3509ns | +4.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 751ns | base | --- | [719, 804] | --- | --- | --- | --- |
| br_predicate_h_pred95 | 1244ns | +505.4ns (+67.3%) | [+466, +527]ns | [1200, 1330] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred95 | 852ns | no significant difference | [-10, +165]ns | [709, 916] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred95 | br_predicate_h_pred95 | br_profiled_hot_h_pred95 |
|---|---|---|---|
| 1 | 714ns | +62.1% | -5.6% |
| 2 | 835ns | +61.8% | +4.4% |
| 3 | 774ns | +69.1% | +7.5% |
| 4 | 724ns | +71.6% | +2.8% |
| 5 | 748ns | +66.2% | +18.8% |
| 6 | 755ns | +64.8% | +25.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred95 | -0.248 | moderate- |
| br_predicate_h_pred95 | -0.223 | moderate- |
| br_profiled_hot_h_pred95 | -0.097 | ok |

**Consistency summary:**

- **br_predicate_h_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred95**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred95 | 5.4ns | 758.2ns | 0.7% |  |
| br_predicate_h_pred95 | 3.7ns | 1257.7ns | 0.3% |  |
| br_profiled_hot_h_pred95 | 5.2ns | 825.8ns | 0.6% |  |

## Distribution (algo ns)

```
br_branch_h_pred95 (n=6, range 714.2-804.2 ns)
    714.2 |########################################
    718.7 |
    723.2 |########################################
    727.7 |
    732.2 |
    736.7 |
    741.2 |
    745.7 |########################################
    750.2 |
    754.7 |########################################
    759.2 |
    763.7 |
    768.2 |
    772.7 |########################################
    777.2 |
    781.7 |
    786.2 |
    790.7 |
    795.2 |
    799.7 |
  (0 below, 1 above range)

br_predicate_h_pred95 (n=6, range 1157.5-1329.8 ns)
   1157.5 |####################
   1166.1 |
   1174.7 |
   1183.3 |
   1192.0 |
   1200.6 |
   1209.2 |
   1217.8 |
   1226.4 |
   1235.0 |########################################
   1243.7 |####################
   1252.3 |
   1260.9 |
   1269.5 |
   1278.1 |
   1286.7 |
   1295.3 |
   1304.0 |####################
   1312.6 |
   1321.2 |
  (0 below, 1 above range)

br_profiled_hot_h_pred95 (n=6, range 674.2-916.5 ns)
    674.2 |########################################
    686.3 |
    698.4 |
    710.5 |
    722.7 |
    734.8 |########################################
    746.9 |
    759.0 |
    771.1 |
    783.2 |
    795.3 |
    807.4 |
    819.6 |
    831.7 |########################################
    843.8 |
    855.9 |
    868.0 |########################################
    880.1 |########################################
    892.2 |
    904.3 |
  (0 below, 1 above range)

```

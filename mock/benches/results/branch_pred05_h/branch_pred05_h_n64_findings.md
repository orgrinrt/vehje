# Branch strategies, heavy-arm, pred05: ~5% taken, predictable (b<13)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred05**

## Key findings

- **Baseline (br_branch_h_pred05) is the fastest** at 416.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.62x (fastest 416.9 ns, slowest 1092.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 2791ns | 2583ns | 2573ns | 2581ns | 3215ns | base |
| br_predicate_h_pred05 | 3653ns | 3409ns | 3237ns | 3354ns | 4310ns | +30.88% |
| br_profiled_hot_h_pred05 | 2905ns | 2878ns | 2585ns | 2786ns | 3243ns | +4.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred05 | 446ns | 402ns | 518ns | base | 0.143 |
| br_predicate_h_pred05 | 1158ns | 1039ns | 1342ns | +159.68% | 0.055 |
| br_profiled_hot_h_pred05 | 482ns | 410ns | 553ns | +8.14% | 0.133 |

## Performance model

- Peak throughput: **0.159 Gops/s** (br_branch_h_pred05; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred05 | 0.154 | 96.5% |
| br_predicate_h_pred05 | 0.059 | 36.8% |
| br_profiled_hot_h_pred05 | 0.135 | 85.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred05 | 2791ns | 2791ns | base |
| br_predicate_h_pred05 | 3653ns | 3653ns | +30.88% |
| br_profiled_hot_h_pred05 | 2905ns | 2905ns | +4.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 417ns | base | --- | [404, 518] | --- | --- | --- | --- |
| br_predicate_h_pred05 | 1092ns | +682.5ns (+163.7%) | [+629, +825]ns | [1040, 1342] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_pred05 | 472ns | +28.4ns (+6.8%) | [+9, +72]ns | [422, 553] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred05 | br_predicate_h_pred05 | br_profiled_hot_h_pred05 |
|---|---|---|---|
| 1 | 402ns | +175.5% | +2.0% |
| 2 | 405ns | +156.3% | +6.9% |
| 3 | 416ns | +150.3% | +6.9% |
| 4 | 418ns | +157.7% | +25.7% |
| 5 | 545ns | +165.3% | +6.6% |
| 6 | 490ns | +152.8% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred05 | 0.322 | moderate+ |
| br_predicate_h_pred05 | 0.235 | moderate+ |
| br_profiled_hot_h_pred05 | 0.468 | moderate+ |

**Consistency summary:**

- **br_predicate_h_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred05**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred05 | 3.5ns | 446.0ns | 0.8% |  |
| br_predicate_h_pred05 | 3.3ns | 1158.3ns | 0.3% |  |
| br_profiled_hot_h_pred05 | 4.0ns | 482.4ns | 0.8% |  |

## Distribution (algo ns)

```
br_branch_h_pred05 (n=6, range 402.1-517.5 ns)
    402.1 |########################################
    407.9 |
    413.6 |########################################
    419.4 |
    425.2 |
    431.0 |
    436.7 |
    442.5 |
    448.3 |
    454.0 |
    459.8 |
    465.6 |
    471.3 |
    477.1 |
    482.9 |
    488.6 |####################
    494.4 |
    500.2 |
    506.0 |
    511.7 |
  (0 below, 1 above range)

br_predicate_h_pred05 (n=6, range 1039.2-1342.3 ns)
   1039.2 |########################################
   1054.4 |
   1069.5 |####################
   1084.7 |
   1099.8 |####################
   1115.0 |
   1130.1 |
   1145.3 |
   1160.4 |
   1175.6 |
   1190.8 |
   1205.9 |
   1221.1 |
   1236.2 |####################
   1251.4 |
   1266.5 |
   1281.7 |
   1296.8 |
   1312.0 |
   1327.1 |
  (0 below, 1 above range)

br_profiled_hot_h_pred05 (n=6, range 410.0-552.9 ns)
    410.0 |########################################
    417.1 |
    424.3 |
    431.4 |########################################
    438.6 |########################################
    445.7 |
    452.9 |
    460.0 |
    467.2 |
    474.3 |
    481.4 |
    488.6 |
    495.7 |########################################
    502.9 |
    510.0 |
    517.2 |
    524.3 |########################################
    531.5 |
    538.6 |
    545.8 |
  (0 below, 1 above range)

```

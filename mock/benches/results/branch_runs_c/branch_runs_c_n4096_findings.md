# Branch strategies, cheap-arm, runs: correlated long runs (flip when b<24)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_runs**

## Key findings

- **Baseline (br_branch_c_runs) is the fastest** at 7702.9 ns median
- 3 variants significantly slower than baseline
- Spread: 1.29x (fastest 7702.9 ns, slowest 9908.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_runs | 10413ns | 10295ns | 9115ns | 10246ns | 11313ns | base |
| br_lut_c_runs | 12410ns | 12552ns | 10324ns | 12470ns | 13365ns | +19.18% |
| br_mask_c_runs | 12270ns | 12374ns | 10706ns | 12106ns | 13298ns | +17.83% |
| br_predicate_c_runs | 11934ns | 12354ns | 10444ns | 11893ns | 12740ns | +14.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_runs | 7807ns | 6864ns | 8501ns | base | 0.525 |
| br_lut_c_runs | 9793ns | 8136ns | 10545ns | +25.44% | 0.418 |
| br_mask_c_runs | 9699ns | 8461ns | 10518ns | +24.24% | 0.422 |
| br_predicate_c_runs | 9446ns | 8262ns | 10078ns | +21.00% | 0.434 |

## Performance model

- Peak throughput: **0.597 Gops/s** (br_branch_c_runs; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_runs | 0.532 | 89.1% |
| br_lut_c_runs | 0.413 | 69.3% |
| br_mask_c_runs | 0.419 | 70.2% |
| br_predicate_c_runs | 0.419 | 70.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_runs | 10413ns | 10413ns | base |
| br_lut_c_runs | 12410ns | 12410ns | +19.18% |
| br_mask_c_runs | 12270ns | 12270ns | +17.83% |
| br_predicate_c_runs | 11934ns | 11934ns | +14.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_runs | 7703ns | base | --- | [7216, 8501] | --- | --- | --- | --- |
| br_lut_c_runs | 9909ns | +1892.3ns (+24.6%) | [+1542, +2524]ns | [8925, 10545] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_runs | 9775ns | +1979.0ns (+25.7%) | [+1588, +2110]ns | [8804, 10518] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_runs | 9769ns | +1454.8ns (+18.9%) | [+1097, +2366]ns | [8492, 10078] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_runs | br_lut_c_runs | br_mask_c_runs | br_predicate_c_runs |
|---|---|---|---|---|
| 1 | 6864ns | +18.5% | +23.3% | +20.4% |
| 2 | 7568ns | +28.3% | +20.9% | +15.2% |
| 3 | 7761ns | +25.3% | +25.9% | +25.8% |
| 4 | 8732ns | +20.7% | +22.3% | +11.9% |
| 5 | 8271ns | +22.1% | +25.2% | +18.3% |
| 6 | 7645ns | +38.0% | +27.9% | +35.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_runs | 0.268 | moderate+ |
| br_lut_c_runs | 0.135 | ok |
| br_mask_c_runs | 0.436 | moderate+ |
| br_predicate_c_runs | 0.371 | moderate+ |

**Consistency summary:**

- **br_lut_c_runs**: won 0/6, lost 6/6
- **br_mask_c_runs**: won 0/6, lost 6/6
- **br_predicate_c_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_runs | 3.1ns | 7806.8ns | 0.0% |  |
| br_lut_c_runs | 3.2ns | 9792.8ns | 0.0% |  |
| br_mask_c_runs | 3.3ns | 9699.1ns | 0.0% |  |
| br_predicate_c_runs | 2.6ns | 9446.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_runs (n=6, range 6863.7-8501.5 ns)
   6863.7 |########################################
   6945.6 |
   7027.5 |
   7109.4 |
   7191.2 |
   7273.1 |
   7355.0 |
   7436.9 |
   7518.8 |########################################
   7600.7 |########################################
   7682.6 |########################################
   7764.5 |
   7846.4 |
   7928.2 |
   8010.1 |
   8092.0 |
   8173.9 |
   8255.8 |########################################
   8337.7 |
   8419.6 |
  (0 below, 1 above range)

br_lut_c_runs (n=6, range 8135.8-10545.2 ns)
   8135.8 |####################
   8256.3 |
   8376.7 |
   8497.2 |
   8617.7 |
   8738.1 |
   8858.6 |
   8979.1 |
   9099.6 |
   9220.0 |
   9340.5 |
   9461.0 |
   9581.4 |
   9701.9 |########################################
   9822.4 |
   9942.9 |
  10063.3 |####################
  10183.8 |
  10304.3 |
  10424.7 |####################
  (0 below, 1 above range)

br_mask_c_runs (n=6, range 8460.8-10518.0 ns)
   8460.8 |####################
   8563.7 |
   8666.5 |
   8769.4 |
   8872.2 |
   8975.1 |
   9077.9 |####################
   9180.8 |
   9283.7 |
   9386.5 |
   9489.4 |
   9592.2 |
   9695.1 |########################################
   9797.9 |
   9900.8 |
  10003.7 |
  10106.5 |
  10209.4 |
  10312.2 |####################
  10415.1 |
  (0 below, 1 above range)

br_predicate_c_runs (n=6, range 8262.5-10078.0 ns)
   8262.5 |#############
   8353.3 |
   8444.0 |
   8534.8 |
   8625.6 |
   8716.4 |#############
   8807.1 |
   8897.9 |
   8988.7 |
   9079.5 |
   9170.2 |
   9261.0 |
   9351.8 |
   9442.5 |
   9533.3 |
   9624.1 |
   9714.9 |########################################
   9805.6 |
   9896.4 |
   9987.2 |
  (0 below, 1 above range)

```

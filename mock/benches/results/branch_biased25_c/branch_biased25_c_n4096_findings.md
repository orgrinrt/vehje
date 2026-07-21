# Branch strategies, cheap-arm, biased25: ~25% taken (b<64)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_biased25**

## Key findings

- **Fastest: br_mask_c_biased25** at 8634.4 ns median (-0.7% vs baseline)
- Spread: 1.05x (fastest 8634.4 ns, slowest 9074.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 10927ns | 11266ns | 9304ns | 10995ns | 11636ns | base |
| br_lut_c_biased25 | 11942ns | 11212ns | 9700ns | 10715ns | 14902ns | +9.28% |
| br_mask_c_biased25 | 10858ns | 11131ns | 9712ns | 10675ns | 11704ns | -0.64% |
| br_predicate_c_biased25 | 10923ns | 11222ns | 9747ns | 10831ns | 11649ns | -0.04% |
| br_profiled_hot_c_biased25 | 11405ns | 11431ns | 10383ns | 11102ns | 12371ns | +4.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_biased25 | 8442ns | 7110ns | 9065ns | base | 0.485 |
| br_lut_c_biased25 | 8867ns | 7524ns | 10372ns | +5.04% | 0.462 |
| br_mask_c_biased25 | 8429ns | 7540ns | 9098ns | -0.15% | 0.486 |
| br_predicate_c_biased25 | 8475ns | 7568ns | 9047ns | +0.39% | 0.483 |
| br_profiled_hot_c_biased25 | 9050ns | 8190ns | 9843ns | +7.20% | 0.453 |

## Performance model

- Peak throughput: **0.576 Gops/s** (br_branch_c_biased25; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_biased25 | 0.471 | 81.8% |
| br_lut_c_biased25 | 0.471 | 81.7% |
| br_mask_c_biased25 | 0.474 | 82.3% |
| br_predicate_c_biased25 | 0.471 | 81.7% |
| br_profiled_hot_c_biased25 | 0.451 | 78.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_biased25 | 10927ns | 10927ns | base |
| br_lut_c_biased25 | 11942ns | 11942ns | +9.28% |
| br_mask_c_biased25 | 10858ns | 10858ns | -0.64% |
| br_predicate_c_biased25 | 10923ns | 10923ns | -0.04% |
| br_profiled_hot_c_biased25 | 11405ns | 11405ns | +4.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 8695ns | base | --- | [7565, 9065] | --- | --- | --- | --- |
| br_lut_c_biased25 | 8701ns | no significant difference | [-799, +1941]ns | [7530, 10372] | no | 1.0000 | 1.0000 | 0 |
| br_mask_c_biased25 | 8634ns | no significant difference | [-928, +525]ns | [7556, 9098] | no | 1.0000 | 0.6875 | 0 |
| br_predicate_c_biased25 | 8698ns | no significant difference | [-672, +714]ns | [7681, 9047] | no | 1.0000 | 1.0000 | 0 |
| br_profiled_hot_c_biased25 | 9074ns | no significant difference | [-200, +1446]ns | [8231, 9843] | no | 0.8750 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_biased25 | br_lut_c_biased25 | br_mask_c_biased25 | br_predicate_c_biased25 | br_profiled_hot_c_biased25 |
|---|---|---|---|---|---|
| 1 | 8020ns | +43.1% | +6.8% | +12.1% | +2.1% |
| 2 | 8690ns | -3.2% | +3.5% | +4.7% | +15.2% |
| 3 | 8700ns | +3.3% | +5.8% | -3.4% | +8.8% |
| 4 | 9288ns | -0.2% | -6.3% | -3.2% | +4.2% |
| 5 | 7110ns | +6.0% | +6.1% | +6.4% | +22.1% |
| 6 | 8842ns | -14.9% | -14.4% | -11.8% | -6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_biased25 | -0.501 | HIGH- (thermal bounce) |
| br_lut_c_biased25 | 0.006 | ok |
| br_mask_c_biased25 | 0.490 | moderate+ |
| br_predicate_c_biased25 | 0.177 | ok |
| br_profiled_hot_c_biased25 | -0.038 | ok |

**Consistency summary:**

- **br_lut_c_biased25**: won 3/6, lost 3/6
- **br_mask_c_biased25**: won 2/6, lost 4/6
- **br_predicate_c_biased25**: won 3/6, lost 3/6
- **br_profiled_hot_c_biased25**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_biased25 | 3.8ns | 8441.9ns | 0.0% |  |
| br_lut_c_biased25 | 4.9ns | 8867.3ns | 0.1% |  |
| br_mask_c_biased25 | 3.5ns | 8429.4ns | 0.0% |  |
| br_predicate_c_biased25 | 2.8ns | 8475.3ns | 0.0% |  |
| br_profiled_hot_c_biased25 | 3.5ns | 9049.5ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_biased25 (n=6, range 7110.0-9065.2 ns)
   7110.0 |####################
   7207.8 |
   7305.5 |
   7403.3 |
   7501.0 |
   7598.8 |
   7696.6 |
   7794.3 |
   7892.1 |
   7989.8 |####################
   8087.6 |
   8185.4 |
   8283.1 |
   8380.9 |
   8478.6 |
   8576.4 |
   8674.2 |########################################
   8771.9 |####################
   8869.7 |
   8967.4 |
  (0 below, 1 above range)

br_lut_c_biased25 (n=6, range 7523.7-10371.7 ns)
   7523.7 |########################################
   7666.1 |
   7808.5 |
   7950.9 |
   8093.3 |
   8235.7 |
   8378.1 |####################
   8520.5 |
   8662.9 |
   8805.3 |
   8947.7 |####################
   9090.1 |
   9232.5 |####################
   9374.9 |
   9517.3 |
   9659.7 |
   9802.1 |
   9944.5 |
  10086.9 |
  10229.3 |
  (0 below, 1 above range)

br_mask_c_biased25 (n=6, range 7540.4-9097.9 ns)
   7540.4 |########################################
   7618.3 |
   7696.1 |
   7774.0 |
   7851.9 |
   7929.8 |
   8007.7 |
   8085.5 |
   8163.4 |
   8241.3 |
   8319.2 |
   8397.0 |
   8474.9 |
   8552.8 |####################
   8630.7 |####################
   8708.5 |
   8786.4 |
   8864.3 |
   8942.2 |####################
   9020.0 |
  (0 below, 1 above range)

br_predicate_c_biased25 (n=6, range 7567.9-9046.9 ns)
   7567.9 |####################
   7641.8 |
   7715.8 |
   7789.7 |####################
   7863.7 |
   7937.6 |
   8011.6 |
   8085.5 |
   8159.5 |
   8233.4 |
   8307.4 |
   8381.3 |####################
   8455.3 |
   8529.2 |
   8603.2 |
   8677.1 |
   8751.1 |
   8825.0 |
   8899.0 |
   8972.9 |########################################
  (0 below, 1 above range)

br_profiled_hot_c_biased25 (n=6, range 8190.0-9843.1 ns)
   8190.0 |########################################
   8272.7 |
   8355.3 |
   8438.0 |
   8520.6 |
   8603.3 |####################
   8685.9 |
   8768.6 |
   8851.2 |
   8933.9 |
   9016.5 |
   9099.2 |
   9181.9 |
   9264.5 |
   9347.2 |
   9429.8 |####################
   9512.5 |
   9595.1 |
   9677.8 |####################
   9760.4 |
  (0 below, 1 above range)

```

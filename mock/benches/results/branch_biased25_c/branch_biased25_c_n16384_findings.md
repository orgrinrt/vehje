# Branch strategies, cheap-arm, biased25: ~25% taken (b<64)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_biased25**

## Key findings

- **Fastest: br_mask_c_biased25** at 32767.9 ns median (-20.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.30x (fastest 32767.9 ns, slowest 42473.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 44007ns | 43688ns | 40865ns | 42821ns | 47356ns | base |
| br_lut_c_biased25 | 35295ns | 35128ns | 33549ns | 34775ns | 36948ns | -19.80% |
| br_mask_c_biased25 | 35027ns | 35144ns | 33342ns | 34721ns | 36327ns | -20.41% |
| br_predicate_c_biased25 | 35008ns | 36059ns | 32279ns | 34895ns | 36543ns | -20.45% |
| br_profiled_hot_c_biased25 | 45306ns | 44750ns | 43606ns | 44680ns | 47095ns | +2.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_biased25 | 41592ns | 38466ns | 44793ns | base | 0.394 |
| br_lut_c_biased25 | 32904ns | 31222ns | 34452ns | -20.89% | 0.498 |
| br_mask_c_biased25 | 32543ns | 30764ns | 33791ns | -21.76% | 0.503 |
| br_predicate_c_biased25 | 32645ns | 30105ns | 34069ns | -21.51% | 0.502 |
| br_profiled_hot_c_biased25 | 42998ns | 41347ns | 44705ns | +3.38% | 0.381 |

## Performance model

- Peak throughput: **0.544 Gops/s** (br_predicate_c_biased25; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_biased25 | 0.396 | 72.8% |
| br_lut_c_biased25 | 0.500 | 91.8% |
| br_mask_c_biased25 | 0.500 | 91.9% |
| br_predicate_c_biased25 | 0.487 | 89.5% |
| br_profiled_hot_c_biased25 | 0.386 | 70.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_biased25 | 44007ns | 44007ns | base |
| br_lut_c_biased25 | 35295ns | 35295ns | -19.80% |
| br_mask_c_biased25 | 35027ns | 35027ns | -20.41% |
| br_predicate_c_biased25 | 35008ns | 35008ns | -20.45% |
| br_profiled_hot_c_biased25 | 45306ns | 45306ns | +2.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 41333ns | base | --- | [38648, 44793] | --- | --- | --- | --- |
| br_lut_c_biased25 | 32778ns | -8304.2ns (-20.1%) | [-10592, -7166]ns | [31483, 34452] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_biased25 | 32768ns | -9258.5ns (-22.4%) | [-11811, -6075]ns | [31071, 33791] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_biased25 | 33625ns | -9368.8ns (-22.7%) | [-11045, -6425]ns | [30242, 34069] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_biased25 | 42474ns | no significant difference | [-1289, +3776]ns | [41816, 44705] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_biased25 | br_lut_c_biased25 | br_mask_c_biased25 | br_predicate_c_biased25 | br_profiled_hot_c_biased25 |
|---|---|---|---|---|---|
| 1 | 38466ns | -18.8% | -14.6% | -10.7% | +9.9% |
| 2 | 45255ns | -25.6% | -25.5% | -25.5% | +3.0% |
| 3 | 43445ns | -21.4% | -27.8% | -22.8% | -2.4% |
| 4 | 44332ns | -21.6% | -23.6% | -23.8% | -3.4% |
| 5 | 38831ns | -18.2% | -20.8% | -22.5% | +9.6% |
| 6 | 39220ns | -18.7% | -16.7% | -22.5% | +5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_biased25 | -0.013 | ok |
| br_lut_c_biased25 | 0.096 | ok |
| br_mask_c_biased25 | -0.663 | HIGH- (thermal bounce) |
| br_predicate_c_biased25 | 0.373 | moderate+ |
| br_profiled_hot_c_biased25 | -0.230 | moderate- |

**Consistency summary:**

- **br_lut_c_biased25**: won 6/6, lost 0/6
- **br_mask_c_biased25**: won 6/6, lost 0/6
- **br_predicate_c_biased25**: won 6/6, lost 0/6
- **br_profiled_hot_c_biased25**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_biased25 | 3.6ns | 41591.5ns | 0.0% |  |
| br_lut_c_biased25 | 3.4ns | 32904.3ns | 0.0% |  |
| br_mask_c_biased25 | 6.5ns | 32543.1ns | 0.0% |  |
| br_predicate_c_biased25 | 4.3ns | 32645.3ns | 0.0% |  |
| br_profiled_hot_c_biased25 | 4.2ns | 42998.4ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_biased25 (n=6, range 38466.2-44793.3 ns)
  38466.2 |########################################
  38782.6 |########################################
  39098.9 |########################################
  39415.3 |
  39731.6 |
  40048.0 |
  40364.3 |
  40680.7 |
  40997.1 |
  41313.4 |
  41629.8 |
  41946.1 |
  42262.5 |
  42578.8 |
  42895.2 |
  43211.6 |########################################
  43527.9 |
  43844.3 |
  44160.6 |########################################
  44477.0 |
  (0 below, 1 above range)

br_lut_c_biased25 (n=6, range 31221.7-34451.7 ns)
  31221.7 |########################################
  31383.2 |
  31544.7 |
  31706.2 |########################################
  31867.7 |########################################
  32029.2 |
  32190.7 |
  32352.2 |
  32513.7 |
  32675.2 |
  32836.7 |
  32998.2 |
  33159.7 |
  33321.2 |
  33482.7 |
  33644.2 |########################################
  33805.7 |
  33967.2 |
  34128.7 |########################################
  34290.2 |
  (0 below, 1 above range)

br_mask_c_biased25 (n=6, range 30764.2-33790.8 ns)
  30764.2 |########################################
  30915.5 |
  31066.9 |
  31218.2 |
  31369.5 |########################################
  31520.9 |
  31672.2 |
  31823.5 |
  31974.9 |
  32126.2 |
  32277.5 |
  32428.9 |
  32580.2 |########################################
  32731.5 |########################################
  32882.9 |
  33034.2 |
  33185.5 |
  33336.9 |
  33488.2 |
  33639.5 |########################################
  (0 below, 1 above range)

br_predicate_c_biased25 (n=6, range 30104.6-34069.2 ns)
  30104.6 |####################
  30302.8 |####################
  30501.1 |
  30699.3 |
  30897.5 |
  31095.8 |
  31294.0 |
  31492.2 |
  31690.4 |
  31888.7 |
  32086.9 |
  32285.1 |
  32483.4 |
  32681.6 |
  32879.8 |
  33078.0 |
  33276.3 |
  33474.5 |####################
  33672.7 |########################################
  33871.0 |
  (0 below, 1 above range)

br_profiled_hot_c_biased25 (n=6, range 41346.7-44705.2 ns)
  41346.7 |########################################
  41514.6 |
  41682.5 |
  41850.5 |
  42018.4 |
  42186.3 |########################################
  42354.2 |########################################
  42522.2 |########################################
  42690.1 |########################################
  42858.0 |
  43025.9 |
  43193.9 |
  43361.8 |
  43529.7 |
  43697.6 |
  43865.6 |
  44033.5 |
  44201.4 |
  44369.3 |
  44537.3 |
  (0 below, 1 above range)

```

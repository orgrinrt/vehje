# Branch strategies, cheap-arm, pred05: ~5% taken, predictable (b<13)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred05**

## Key findings

- **Fastest: br_profiled_hot_c_pred05** at 24719.2 ns median (-0.4% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.38x (fastest 24719.2 ns, slowest 33997.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 26506ns | 27288ns | 23815ns | 26465ns | 27914ns | base |
| br_lut_c_pred05 | 35048ns | 36150ns | 32299ns | 35031ns | 36449ns | +32.23% |
| br_mask_c_pred05 | 35936ns | 36492ns | 33205ns | 35458ns | 38019ns | +35.58% |
| br_predicate_c_pred05 | 35486ns | 35969ns | 32376ns | 35195ns | 37477ns | +33.88% |
| br_profiled_hot_c_pred05 | 27111ns | 27139ns | 24351ns | 26639ns | 29197ns | +2.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred05 | 24109ns | 21604ns | 25406ns | base | 0.680 |
| br_lut_c_pred05 | 32653ns | 30123ns | 33904ns | +35.44% | 0.502 |
| br_mask_c_pred05 | 33474ns | 30918ns | 35399ns | +38.85% | 0.489 |
| br_predicate_c_pred05 | 33056ns | 30151ns | 34914ns | +37.11% | 0.496 |
| br_profiled_hot_c_pred05 | 24724ns | 22202ns | 26648ns | +2.55% | 0.663 |

## Performance model

- Peak throughput: **0.758 Gops/s** (br_branch_c_pred05; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred05 | 0.660 | 87.1% |
| br_lut_c_pred05 | 0.486 | 64.1% |
| br_mask_c_pred05 | 0.482 | 63.5% |
| br_predicate_c_pred05 | 0.489 | 64.4% |
| br_profiled_hot_c_pred05 | 0.663 | 87.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred05 | 26506ns | 26506ns | base |
| br_lut_c_pred05 | 35048ns | 35048ns | +32.23% |
| br_mask_c_pred05 | 35936ns | 35936ns | +35.58% |
| br_predicate_c_pred05 | 35486ns | 35486ns | +33.88% |
| br_profiled_hot_c_pred05 | 27111ns | 27111ns | +2.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 24817ns | base | --- | [22103, 25406] | --- | --- | --- | --- |
| br_lut_c_pred05 | 33721ns | +8471.0ns (+34.1%) | [+8074, +9087]ns | [30334, 33904] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred05 | 33998ns | +9529.3ns (+38.4%) | [+8575, +9993]ns | [31027, 35399] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred05 | 33532ns | +8751.9ns (+35.3%) | [+8314, +9777]ns | [30723, 34914] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred05 | 24719ns | no significant difference | [-349, +1321]ns | [22804, 26648] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred05 | br_lut_c_pred05 | br_mask_c_pred05 | br_predicate_c_pred05 | br_profiled_hot_c_pred05 |
|---|---|---|---|---|---|
| 1 | 22602ns | +35.1% | +36.8% | +38.5% | +3.6% |
| 2 | 21604ns | +39.4% | +44.1% | +39.6% | +2.8% |
| 3 | 24913ns | +35.8% | +38.2% | +39.2% | +4.4% |
| 4 | 25360ns | +33.2% | +40.9% | +38.6% | +6.1% |
| 5 | 24720ns | +37.4% | +35.7% | +35.6% | -5.2% |
| 6 | 25452ns | +32.2% | +37.8% | +31.7% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred05 | 0.337 | moderate+ |
| br_lut_c_pred05 | 0.398 | moderate+ |
| br_mask_c_pred05 | 0.302 | moderate+ |
| br_predicate_c_pred05 | 0.263 | moderate+ |
| br_profiled_hot_c_pred05 | -0.111 | ok |

**Consistency summary:**

- **br_lut_c_pred05**: won 0/6, lost 6/6
- **br_mask_c_pred05**: won 0/6, lost 6/6
- **br_predicate_c_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred05**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred05 | 4.2ns | 24108.6ns | 0.0% |  |
| br_lut_c_pred05 | 3.7ns | 32652.8ns | 0.0% |  |
| br_mask_c_pred05 | 4.6ns | 33474.5ns | 0.0% |  |
| br_predicate_c_pred05 | 3.0ns | 33056.2ns | 0.0% |  |
| br_profiled_hot_c_pred05 | 5.0ns | 24723.7ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_pred05 (n=6, range 21604.2-25406.1 ns)
  21604.2 |########################################
  21794.3 |
  21984.4 |
  22174.5 |
  22364.6 |
  22554.7 |########################################
  22744.8 |
  22934.8 |
  23124.9 |
  23315.0 |
  23505.1 |
  23695.2 |
  23885.3 |
  24075.4 |
  24265.5 |
  24455.6 |
  24645.7 |########################################
  24835.8 |########################################
  25025.9 |
  25216.0 |########################################
  (0 below, 1 above range)

br_lut_c_pred05 (n=6, range 30122.9-33903.9 ns)
  30122.9 |####################
  30312.0 |
  30501.0 |####################
  30690.1 |
  30879.1 |
  31068.2 |
  31257.2 |
  31446.3 |
  31635.3 |
  31824.4 |
  32013.4 |
  32202.5 |
  32391.5 |
  32580.6 |
  32769.6 |
  32958.7 |
  33147.7 |
  33336.8 |
  33525.8 |####################
  33714.9 |########################################
  (0 below, 1 above range)

br_mask_c_pred05 (n=6, range 30918.3-35398.9 ns)
  30918.3 |########################################
  31142.3 |
  31366.4 |
  31590.4 |
  31814.4 |
  32038.5 |
  32262.5 |
  32486.5 |
  32710.6 |
  32934.6 |
  33158.6 |
  33382.7 |####################
  33606.7 |
  33830.7 |
  34054.8 |
  34278.8 |####################
  34502.8 |
  34726.9 |
  34950.9 |####################
  35174.9 |
  (0 below, 1 above range)

br_predicate_c_pred05 (n=6, range 30151.2-34913.6 ns)
  30151.2 |####################
  30389.3 |
  30627.4 |
  30865.6 |
  31103.7 |####################
  31341.8 |
  31579.9 |
  31818.0 |
  32056.1 |
  32294.3 |
  32532.4 |
  32770.5 |
  33008.6 |
  33246.7 |
  33484.8 |########################################
  33723.0 |
  33961.1 |
  34199.2 |
  34437.3 |
  34675.4 |####################
  (0 below, 1 above range)

br_profiled_hot_c_pred05 (n=6, range 22201.7-26647.9 ns)
  22201.7 |####################
  22424.0 |
  22646.3 |
  22868.6 |
  23090.9 |
  23313.2 |########################################
  23535.6 |
  23757.9 |
  23980.2 |
  24202.5 |
  24424.8 |
  24647.1 |
  24869.4 |
  25091.7 |
  25314.0 |
  25536.4 |
  25758.7 |
  25981.0 |####################
  26203.3 |####################
  26425.6 |
  (0 below, 1 above range)

```

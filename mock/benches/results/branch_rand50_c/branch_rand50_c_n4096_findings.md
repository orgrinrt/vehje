# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Key findings

- **Fastest: br_predicate_c_rand50** at 7988.8 ns median (-48.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.94x (fastest 7988.8 ns, slowest 15521.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 17135ns | 18066ns | 14228ns | 17254ns | 18409ns | base |
| br_lut_c_rand50 | 10778ns | 11038ns | 9705ns | 10595ns | 11588ns | -37.10% |
| br_mask_c_rand50 | 12157ns | 12166ns | 11151ns | 11854ns | 13116ns | -29.05% |
| br_predicate_c_rand50 | 10467ns | 10381ns | 9569ns | 10158ns | 11382ns | -38.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 14696ns | 12050ns | 15812ns | base | 0.279 |
| br_lut_c_rand50 | 8366ns | 7531ns | 8995ns | -43.07% | 0.490 |
| br_mask_c_rand50 | 9692ns | 8913ns | 10462ns | -34.05% | 0.423 |
| br_predicate_c_rand50 | 8079ns | 7424ns | 8768ns | -45.03% | 0.507 |

## Performance model

- Peak throughput: **0.552 Gops/s** (br_predicate_c_rand50; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.264 | 47.8% |
| br_lut_c_rand50 | 0.478 | 86.6% |
| br_mask_c_rand50 | 0.422 | 76.5% |
| br_predicate_c_rand50 | 0.513 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 17135ns | 17135ns | base |
| br_lut_c_rand50 | 10778ns | 10778ns | -37.10% |
| br_mask_c_rand50 | 12157ns | 12157ns | -29.05% |
| br_predicate_c_rand50 | 10467ns | 10467ns | -38.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 15521ns | base | --- | [12755, 15812] | --- | --- | --- | --- |
| br_lut_c_rand50 | 8571ns | -6548.1ns (-42.2%) | [-7219, -5223]ns | [7532, 8995] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 9698ns | -5342.5ns (-34.4%) | [-5831, -3841]ns | [8914, 10462] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 7989ns | -6846.5ns (-44.1%) | [-7802, -5203]ns | [7480, 8768] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 15782ns | -46.7% | -35.2% | -53.0% |
| 2 | 15798ns | -44.7% | -35.2% | -45.9% |
| 3 | 15826ns | -43.2% | -32.4% | -43.2% |
| 4 | 15261ns | -41.0% | -39.9% | -44.9% |
| 5 | 12050ns | -37.5% | -26.0% | -37.2% |
| 6 | 13461ns | -44.1% | -33.8% | -44.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | 0.388 | moderate+ |
| br_lut_c_rand50 | 0.348 | moderate+ |
| br_mask_c_rand50 | 0.430 | moderate+ |
| br_predicate_c_rand50 | 0.246 | moderate+ |

**Consistency summary:**

- **br_lut_c_rand50**: won 6/6, lost 0/6
- **br_mask_c_rand50**: won 6/6, lost 0/6
- **br_predicate_c_rand50**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 4.0ns | 14696.2ns | 0.0% |  |
| br_lut_c_rand50 | 3.4ns | 8366.4ns | 0.0% |  |
| br_mask_c_rand50 | 3.6ns | 9691.5ns | 0.0% |  |
| br_predicate_c_rand50 | 3.3ns | 8079.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 12049.6-15811.8 ns)
  12049.6 |####################
  12237.7 |
  12425.8 |
  12613.9 |
  12802.0 |
  12990.2 |
  13178.3 |
  13366.4 |####################
  13554.5 |
  13742.6 |
  13930.7 |
  14118.8 |
  14306.9 |
  14495.1 |
  14683.2 |
  14871.3 |
  15059.4 |
  15247.5 |####################
  15435.6 |
  15623.7 |########################################
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 7530.8-8995.4 ns)
   7530.8 |########################################
   7604.0 |
   7677.3 |
   7750.5 |
   7823.7 |
   7896.9 |
   7970.2 |
   8043.4 |
   8116.6 |
   8189.9 |
   8263.1 |
   8336.3 |####################
   8409.6 |
   8482.8 |
   8556.0 |
   8629.2 |
   8702.5 |####################
   8775.7 |
   8848.9 |
   8922.2 |####################
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 8913.3-10461.9 ns)
   8913.3 |########################################
   8990.7 |
   9068.2 |
   9145.6 |####################
   9223.0 |
   9300.4 |
   9377.9 |
   9455.3 |
   9532.7 |
   9610.1 |
   9687.6 |
   9765.0 |
   9842.4 |
   9919.9 |
   9997.3 |
  10074.7 |
  10152.1 |
  10229.6 |########################################
  10307.0 |
  10384.4 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 7423.7-8768.1 ns)
   7423.7 |########################################
   7490.9 |########################################
   7558.1 |########################################
   7625.4 |
   7692.6 |
   7759.8 |
   7827.0 |
   7894.2 |
   7961.5 |
   8028.7 |
   8095.9 |
   8163.1 |
   8230.3 |
   8297.6 |
   8364.8 |########################################
   8432.0 |
   8499.2 |########################################
   8566.4 |
   8633.7 |
   8700.9 |
  (0 below, 1 above range)

```

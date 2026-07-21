# Branch strategies, cheap-arm, alt: strict alternation (i&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_alt**

## Key findings

- **Baseline (br_branch_c_alt) is the fastest** at 4772.9 ns median
- 3 variants significantly slower than baseline
- Spread: 2.02x (fastest 4772.9 ns, slowest 9646.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_alt | 7188ns | 7234ns | 6608ns | 7200ns | 7460ns | base |
| br_lut_c_alt | 10951ns | 10852ns | 10057ns | 10842ns | 11562ns | +52.35% |
| br_mask_c_alt | 12088ns | 12121ns | 10938ns | 12046ns | 12724ns | +68.16% |
| br_predicate_c_alt | 10740ns | 10801ns | 9527ns | 10552ns | 11629ns | +49.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_alt | 4742ns | 4368ns | 4915ns | base | 0.864 |
| br_lut_c_alt | 8495ns | 7806ns | 8963ns | +79.14% | 0.482 |
| br_mask_c_alt | 9606ns | 8690ns | 10109ns | +102.56% | 0.426 |
| br_predicate_c_alt | 8328ns | 7392ns | 9024ns | +75.61% | 0.492 |

## Performance model

- Peak throughput: **0.938 Gops/s** (br_branch_c_alt; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_alt | 0.858 | 91.5% |
| br_lut_c_alt | 0.486 | 51.8% |
| br_mask_c_alt | 0.425 | 45.3% |
| br_predicate_c_alt | 0.489 | 52.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_alt | 7188ns | 7188ns | base |
| br_lut_c_alt | 10951ns | 10951ns | +52.35% |
| br_mask_c_alt | 12088ns | 12088ns | +68.16% |
| br_predicate_c_alt | 10740ns | 10740ns | +49.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_alt | 4773ns | base | --- | [4539, 4915] | --- | --- | --- | --- |
| br_lut_c_alt | 8425ns | +3648.3ns (+76.4%) | [+3291, +4319]ns | [8097, 8963] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_alt | 9647ns | +4774.8ns (+100.0%) | [+4246, +5570]ns | [9061, 10109] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_alt | 8380ns | +3779.1ns (+79.2%) | [+2764, +4214]ns | [7579, 9024] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_alt | br_lut_c_alt | br_mask_c_alt | br_predicate_c_alt |
|---|---|---|---|---|
| 1 | 4710ns | +65.8% | +100.3% | +56.9% |
| 2 | 4920ns | +87.5% | +76.6% | +57.8% |
| 3 | 4910ns | +71.0% | +96.4% | +85.1% |
| 4 | 4711ns | +78.1% | +112.7% | +90.2% |
| 5 | 4368ns | +99.2% | +133.5% | +91.9% |
| 6 | 4835ns | +74.9% | +99.6% | +73.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_alt | -0.020 | ok |
| br_lut_c_alt | -0.555 | HIGH- (thermal bounce) |
| br_mask_c_alt | 0.296 | moderate+ |
| br_predicate_c_alt | 0.283 | moderate+ |

**Consistency summary:**

- **br_lut_c_alt**: won 0/6, lost 6/6
- **br_mask_c_alt**: won 0/6, lost 6/6
- **br_predicate_c_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_alt | 4.3ns | 4742.0ns | 0.1% |  |
| br_lut_c_alt | 3.3ns | 8495.0ns | 0.0% |  |
| br_mask_c_alt | 2.6ns | 9605.5ns | 0.0% |  |
| br_predicate_c_alt | 3.6ns | 8327.6ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_alt (n=6, range 4367.5-4914.6 ns)
   4367.5 |####################
   4394.9 |
   4422.2 |
   4449.6 |
   4476.9 |
   4504.3 |
   4531.6 |
   4559.0 |
   4586.3 |
   4613.7 |
   4641.1 |
   4668.4 |
   4695.8 |########################################
   4723.1 |
   4750.5 |
   4777.8 |
   4805.2 |
   4832.5 |####################
   4859.9 |
   4887.2 |####################
  (0 below, 1 above range)

br_lut_c_alt (n=6, range 7806.2-8962.7 ns)
   7806.2 |####################
   7864.0 |
   7921.9 |
   7979.7 |
   8037.5 |
   8095.3 |
   8153.1 |
   8211.0 |
   8268.8 |
   8326.6 |
   8384.5 |########################################
   8442.3 |####################
   8500.1 |
   8557.9 |
   8615.8 |
   8673.6 |####################
   8731.4 |
   8789.2 |
   8847.1 |
   8904.9 |
  (0 below, 1 above range)

br_mask_c_alt (n=6, range 8690.4-10109.0 ns)
   8690.4 |####################
   8761.3 |
   8832.3 |
   8903.2 |
   8974.1 |
   9045.0 |
   9116.0 |
   9186.9 |
   9257.8 |
   9328.7 |
   9399.7 |####################
   9470.6 |
   9541.5 |
   9612.5 |########################################
   9683.4 |
   9754.3 |
   9825.2 |
   9896.2 |
   9967.1 |####################
  10038.0 |
  (0 below, 1 above range)

br_predicate_c_alt (n=6, range 7391.7-9024.0 ns)
   7391.7 |####################
   7473.3 |
   7554.9 |
   7636.5 |
   7718.1 |####################
   7799.8 |
   7881.4 |
   7963.0 |
   8044.6 |
   8126.2 |
   8207.8 |
   8289.4 |
   8371.1 |########################################
   8452.7 |
   8534.3 |
   8615.9 |
   8697.5 |
   8779.1 |
   8860.7 |
   8942.3 |####################
  (0 below, 1 above range)

```

# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Key findings

- **Baseline (br_branch_c_rand50) is the fastest** at 423.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.57x (fastest 423.8 ns, slowest 665.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 2891ns | 2992ns | 2523ns | 2849ns | 3139ns | base |
| br_lut_c_rand50 | 3077ns | 3165ns | 2744ns | 3031ns | 3313ns | +6.43% |
| br_mask_c_rand50 | 3127ns | 3248ns | 2732ns | 3077ns | 3399ns | +8.14% |
| br_predicate_c_rand50 | 3080ns | 3166ns | 2755ns | 3035ns | 3310ns | +6.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 412ns | 354ns | 451ns | base | 0.622 |
| br_lut_c_rand50 | 567ns | 505ns | 610ns | +37.68% | 0.451 |
| br_mask_c_rand50 | 642ns | 559ns | 701ns | +56.00% | 0.398 |
| br_predicate_c_rand50 | 574ns | 512ns | 618ns | +39.42% | 0.446 |

## Performance model

- Peak throughput: **0.723 Gops/s** (br_branch_c_rand50; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.604 | 83.6% |
| br_lut_c_rand50 | 0.438 | 60.6% |
| br_mask_c_rand50 | 0.384 | 53.2% |
| br_predicate_c_rand50 | 0.433 | 60.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 2891ns | 2891ns | base |
| br_lut_c_rand50 | 3077ns | 3077ns | +6.43% |
| br_mask_c_rand50 | 3127ns | 3127ns | +8.14% |
| br_predicate_c_rand50 | 3080ns | 3080ns | +6.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 424ns | base | --- | [360, 451] | --- | --- | --- | --- |
| br_lut_c_rand50 | 584ns | +157.9ns (+37.3%) | [+125, +183]ns | [506, 610] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 666ns | +241.5ns (+57.0%) | [+200, +250]ns | [560, 701] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 591ns | +164.0ns (+38.7%) | [+153, +170]ns | [514, 618] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 354ns | +43.4% | +58.6% | +45.5% |
| 2 | 367ns | +37.6% | +52.5% | +39.7% |
| 3 | 423ns | +38.3% | +57.4% | +39.6% |
| 4 | 430ns | +47.0% | +56.2% | +37.5% |
| 5 | 424ns | +38.6% | +56.9% | +39.3% |
| 6 | 472ns | +23.6% | +54.6% | +36.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | 0.340 | moderate+ |
| br_lut_c_rand50 | 0.428 | moderate+ |
| br_mask_c_rand50 | 0.353 | moderate+ |
| br_predicate_c_rand50 | 0.334 | moderate+ |

**Consistency summary:**

- **br_lut_c_rand50**: won 0/6, lost 6/6
- **br_mask_c_rand50**: won 0/6, lost 6/6
- **br_predicate_c_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 5.3ns | 411.8ns | 1.3% |  |
| br_lut_c_rand50 | 4.3ns | 567.0ns | 0.8% |  |
| br_mask_c_rand50 | 5.3ns | 642.4ns | 0.8% |  |
| br_predicate_c_rand50 | 4.9ns | 574.2ns | 0.9% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 354.2-451.2 ns)
    354.2 |####################
    359.1 |
    363.9 |####################
    368.8 |
    373.6 |
    378.5 |
    383.3 |
    388.2 |
    393.0 |
    397.9 |
    402.7 |
    407.6 |
    412.4 |
    417.3 |
    422.1 |########################################
    427.0 |####################
    431.8 |
    436.7 |
    441.5 |
    446.4 |
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 504.6-610.4 ns)
    504.6 |########################################
    509.9 |
    515.2 |
    520.5 |
    525.8 |
    531.0 |
    536.3 |
    541.6 |
    546.9 |
    552.2 |
    557.5 |
    562.8 |
    568.1 |
    573.4 |
    578.7 |####################
    584.0 |########################################
    589.2 |
    594.5 |
    599.8 |
    605.1 |
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 559.2-701.0 ns)
    559.2 |########################################
    566.3 |
    573.4 |
    580.5 |
    587.6 |
    594.7 |
    601.8 |
    608.8 |
    615.9 |
    623.0 |
    630.1 |
    637.2 |
    644.3 |
    651.4 |
    658.5 |####################
    665.6 |########################################
    672.7 |
    679.8 |
    686.9 |
    694.0 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 512.1-618.0 ns)
    512.1 |########################################
    517.4 |
    522.7 |
    528.0 |
    533.3 |
    538.6 |
    543.9 |
    549.1 |
    554.4 |
    559.7 |
    565.0 |
    570.3 |
    575.6 |
    580.9 |
    586.2 |########################################
    591.5 |####################
    596.8 |
    602.1 |
    607.4 |
    612.7 |
  (0 below, 1 above range)

```

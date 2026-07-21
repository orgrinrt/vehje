# Branch strategies, cheap-arm, runs: correlated long runs (flip when b<24)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_runs**

## Key findings

- **Baseline (br_branch_c_runs) is the fastest** at 133.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.36x (fastest 133.8 ns, slowest 182.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_runs | 2711ns | 2825ns | 2354ns | 2675ns | 2944ns | base |
| br_lut_c_runs | 2633ns | 2365ns | 2280ns | 2348ns | 3237ns | -2.88% |
| br_mask_c_runs | 2690ns | 2679ns | 2326ns | 2587ns | 3026ns | -0.78% |
| br_predicate_c_runs | 2702ns | 2618ns | 2296ns | 2538ns | 3151ns | -0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_runs | 129ns | 107ns | 140ns | base | 0.497 |
| br_lut_c_runs | 176ns | 155ns | 211ns | +36.84% | 0.363 |
| br_mask_c_runs | 183ns | 157ns | 207ns | +42.18% | 0.349 |
| br_predicate_c_runs | 182ns | 154ns | 215ns | +41.26% | 0.351 |

## Performance model

- Peak throughput: **0.600 Gops/s** (br_branch_c_runs; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_runs | 0.478 | 79.7% |
| br_lut_c_runs | 0.396 | 66.0% |
| br_mask_c_runs | 0.351 | 58.5% |
| br_predicate_c_runs | 0.362 | 60.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_runs | 2711ns | 2711ns | base |
| br_lut_c_runs | 2633ns | 2633ns | -2.88% |
| br_mask_c_runs | 2690ns | 2690ns | -0.78% |
| br_predicate_c_runs | 2702ns | 2702ns | -0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_runs | 134ns | base | --- | [112, 140] | --- | --- | --- | --- |
| br_lut_c_runs | 162ns | +49.2ns (+36.7%) | [+17, +77]ns | [156, 211] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_runs | 182ns | +54.8ns (+40.9%) | [+35, +73]ns | [160, 207] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_runs | 177ns | +52.9ns (+39.6%) | [+20, +86]ns | [155, 215] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_runs | br_lut_c_runs | br_mask_c_runs | br_predicate_c_runs |
|---|---|---|---|---|
| 1 | 145ns | +6.6% | +21.9% | +7.4% |
| 2 | 107ns | +48.4% | +53.0% | +78.4% |
| 3 | 118ns | +39.5% | +32.8% | +30.0% |
| 4 | 134ns | +17.7% | +40.7% | +22.0% |
| 5 | 134ns | +53.5% | +41.1% | +52.6% |
| 6 | 136ns | +60.2% | +66.0% | +65.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_runs | -0.121 | ok |
| br_lut_c_runs | 0.382 | moderate+ |
| br_mask_c_runs | 0.265 | moderate+ |
| br_predicate_c_runs | 0.144 | ok |

**Consistency summary:**

- **br_lut_c_runs**: won 0/6, lost 6/6
- **br_mask_c_runs**: won 0/6, lost 6/6
- **br_predicate_c_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_runs | 3.9ns | 128.9ns | 3.1% |  |
| br_lut_c_runs | 2.9ns | 176.4ns | 1.6% |  |
| br_mask_c_runs | 2.8ns | 183.3ns | 1.5% |  |
| br_predicate_c_runs | 3.5ns | 182.1ns | 1.9% |  |

## Distribution (algo ns)

```
br_branch_c_runs (n=6, range 106.7-140.4 ns)
    106.7 |####################
    108.4 |
    110.1 |
    111.8 |
    113.4 |
    115.1 |
    116.8 |####################
    118.5 |
    120.2 |
    121.9 |
    123.6 |
    125.2 |
    126.9 |
    128.6 |
    130.3 |
    132.0 |
    133.7 |########################################
    135.3 |####################
    137.0 |
    138.7 |
  (0 below, 1 above range)

br_lut_c_runs (n=6, range 154.6-211.4 ns)
    154.6 |####################
    157.4 |########################################
    160.3 |
    163.1 |####################
    166.0 |
    168.8 |
    171.7 |
    174.5 |
    177.3 |
    180.2 |
    183.0 |
    185.9 |
    188.7 |
    191.6 |
    194.4 |
    197.2 |
    200.1 |
    202.9 |####################
    205.8 |
    208.6 |
  (0 below, 1 above range)

br_mask_c_runs (n=6, range 157.1-207.1 ns)
    157.1 |####################
    159.6 |
    162.1 |####################
    164.6 |
    167.1 |
    169.6 |
    172.1 |
    174.6 |####################
    177.1 |
    179.6 |
    182.1 |
    184.6 |
    187.1 |########################################
    189.6 |
    192.1 |
    194.6 |
    197.1 |
    199.6 |
    202.1 |
    204.6 |
  (0 below, 1 above range)

br_predicate_c_runs (n=6, range 153.8-214.6 ns)
    153.8 |########################################
    156.8 |
    159.9 |
    162.9 |####################
    166.0 |
    169.0 |
    172.0 |
    175.1 |
    178.1 |
    181.2 |
    184.2 |
    187.2 |
    190.3 |####################
    193.3 |
    196.4 |
    199.4 |
    202.4 |####################
    205.5 |
    208.5 |
    211.6 |
  (0 below, 1 above range)

```

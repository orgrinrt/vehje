# Branch strategies, cheap-arm, runs: correlated long runs (flip when b<24)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_runs**

## Key findings

- **Baseline (br_branch_c_runs) is the fastest** at 384.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.55x (fastest 384.6 ns, slowest 596.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_runs | 2828ns | 2802ns | 2506ns | 2739ns | 3121ns | base |
| br_lut_c_runs | 3046ns | 2906ns | 2700ns | 2867ns | 3489ns | +7.73% |
| br_mask_c_runs | 3038ns | 3010ns | 2700ns | 2940ns | 3354ns | +7.44% |
| br_predicate_c_runs | 2983ns | 2897ns | 2659ns | 2842ns | 3357ns | +5.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_runs | 391ns | 346ns | 433ns | base | 0.655 |
| br_lut_c_runs | 604ns | 535ns | 691ns | +54.43% | 0.424 |
| br_mask_c_runs | 603ns | 536ns | 668ns | +54.23% | 0.424 |
| br_predicate_c_runs | 591ns | 525ns | 664ns | +50.99% | 0.433 |

## Performance model

- Peak throughput: **0.740 Gops/s** (br_branch_c_runs; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_runs | 0.666 | 89.9% |
| br_lut_c_runs | 0.444 | 60.0% |
| br_mask_c_runs | 0.429 | 58.0% |
| br_predicate_c_runs | 0.446 | 60.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_runs | 2828ns | 2828ns | base |
| br_lut_c_runs | 3046ns | 3046ns | +7.73% |
| br_mask_c_runs | 3038ns | 3038ns | +7.44% |
| br_predicate_c_runs | 2983ns | 2983ns | +5.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_runs | 385ns | base | --- | [356, 433] | --- | --- | --- | --- |
| br_lut_c_runs | 577ns | +195.0ns (+50.7%) | [+176, +268]ns | [544, 691] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_runs | 596ns | +208.4ns (+54.2%) | [+177, +251]ns | [545, 668] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_runs | 574ns | +205.6ns (+53.5%) | [+152, +241]ns | [533, 664] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_runs | br_lut_c_runs | br_mask_c_runs | br_predicate_c_runs |
|---|---|---|---|---|
| 1 | 397ns | +50.6% | +60.0% | +32.3% |
| 2 | 392ns | +41.8% | +41.7% | +51.9% |
| 3 | 469ns | +46.4% | +48.0% | +48.0% |
| 4 | 378ns | +84.1% | +70.0% | +67.9% |
| 5 | 365ns | +51.5% | +52.5% | +48.1% |
| 6 | 346ns | +54.7% | +54.9% | +60.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_runs | 0.056 | ok |
| br_lut_c_runs | 0.109 | ok |
| br_mask_c_runs | -0.056 | ok |
| br_predicate_c_runs | 0.210 | moderate+ |

**Consistency summary:**

- **br_lut_c_runs**: won 0/6, lost 6/6
- **br_mask_c_runs**: won 0/6, lost 6/6
- **br_predicate_c_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_runs | 4.2ns | 391.1ns | 1.1% |  |
| br_lut_c_runs | 4.6ns | 604.0ns | 0.8% |  |
| br_mask_c_runs | 4.1ns | 603.2ns | 0.7% |  |
| br_predicate_c_runs | 5.1ns | 590.6ns | 0.9% |  |

## Distribution (algo ns)

```
br_branch_c_runs (n=6, range 345.8-433.1 ns)
    345.8 |########################################
    350.2 |
    354.5 |
    358.9 |
    363.3 |########################################
    367.6 |
    372.0 |
    376.4 |########################################
    380.7 |
    385.1 |
    389.5 |########################################
    393.8 |########################################
    398.2 |
    402.6 |
    406.9 |
    411.3 |
    415.7 |
    420.0 |
    424.4 |
    428.8 |
  (0 below, 1 above range)

br_lut_c_runs (n=6, range 535.0-691.0 ns)
    535.0 |####################
    542.8 |
    550.6 |########################################
    558.4 |
    566.2 |
    574.0 |
    581.8 |
    589.6 |
    597.4 |####################
    605.2 |
    613.0 |
    620.8 |
    628.6 |
    636.4 |
    644.2 |
    652.0 |
    659.8 |
    667.6 |
    675.4 |
    683.2 |####################
  (0 below, 1 above range)

br_mask_c_runs (n=6, range 535.8-668.0 ns)
    535.8 |########################################
    542.4 |
    549.0 |########################################
    555.6 |########################################
    562.2 |
    568.8 |
    575.4 |
    582.1 |
    588.7 |
    595.3 |
    601.9 |
    608.5 |
    615.1 |
    621.7 |
    628.3 |
    634.9 |########################################
    641.5 |########################################
    648.1 |
    654.7 |
    661.3 |
  (0 below, 1 above range)

br_predicate_c_runs (n=6, range 525.4-664.0 ns)
    525.4 |########################################
    532.3 |
    539.3 |########################################
    546.2 |
    553.1 |########################################
    560.0 |
    567.0 |
    573.9 |
    580.8 |
    587.8 |
    594.7 |########################################
    601.6 |
    608.6 |
    615.5 |
    622.4 |
    629.4 |########################################
    636.3 |
    643.2 |
    650.1 |
    657.1 |
  (0 below, 1 above range)

```

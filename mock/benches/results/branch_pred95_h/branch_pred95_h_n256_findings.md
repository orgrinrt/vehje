# Branch strategies, heavy-arm, pred95: ~95% taken, predictable (b<243)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred95**

## Key findings

- **Baseline (br_branch_h_pred95) is the fastest** at 2852.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.53x (fastest 2852.7 ns, slowest 4360.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 5258ns | 5294ns | 4767ns | 5165ns | 5641ns | base |
| br_predicate_h_pred95 | 6847ns | 6733ns | 6291ns | 6586ns | 7516ns | +30.23% |
| br_profiled_hot_h_pred95 | 5377ns | 5409ns | 4829ns | 5275ns | 5804ns | +2.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred95 | 2835ns | 2579ns | 3037ns | base | 0.090 |
| br_predicate_h_pred95 | 4461ns | 4113ns | 4909ns | +57.36% | 0.057 |
| br_profiled_hot_h_pred95 | 2968ns | 2654ns | 3203ns | +4.68% | 0.086 |

## Performance model

- Peak throughput: **0.099 Gops/s** (br_branch_h_pred95; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred95 | 0.090 | 90.4% |
| br_predicate_h_pred95 | 0.059 | 59.2% |
| br_profiled_hot_h_pred95 | 0.086 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred95 | 5258ns | 5258ns | base |
| br_predicate_h_pred95 | 6847ns | 6847ns | +30.23% |
| br_profiled_hot_h_pred95 | 5377ns | 5377ns | +2.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 2853ns | base | --- | [2616, 3037] | --- | --- | --- | --- |
| br_predicate_h_pred95 | 4360ns | +1646.8ns (+57.7%) | [+1360, +1872]ns | [4115, 4909] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred95 | 2986ns | no significant difference | [-41, +273]ns | [2714, 3203] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred95 | br_predicate_h_pred95 | br_profiled_hot_h_pred95 |
|---|---|---|---|
| 1 | 2579ns | +60.5% | +8.3% |
| 2 | 2652ns | +55.1% | +4.6% |
| 3 | 2858ns | +44.0% | -7.1% |
| 4 | 2848ns | +60.9% | +11.6% |
| 5 | 3036ns | +61.6% | +5.1% |
| 6 | 3038ns | +61.7% | +5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred95 | 0.476 | moderate+ |
| br_predicate_h_pred95 | 0.585 | HIGH+ (drift/warm-up) |
| br_profiled_hot_h_pred95 | 0.407 | moderate+ |

**Consistency summary:**

- **br_predicate_h_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred95**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred95 | 5.0ns | 2835.0ns | 0.2% |  |
| br_predicate_h_pred95 | 4.4ns | 4461.2ns | 0.1% |  |
| br_profiled_hot_h_pred95 | 4.5ns | 2967.7ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_h_pred95 (n=6, range 2579.2-3036.7 ns)
   2579.2 |########################################
   2602.1 |
   2624.9 |
   2647.8 |########################################
   2670.7 |
   2693.6 |
   2716.4 |
   2739.3 |
   2762.2 |
   2785.1 |
   2807.9 |
   2830.8 |########################################
   2853.7 |########################################
   2876.5 |
   2899.4 |
   2922.3 |
   2945.2 |
   2968.0 |
   2990.9 |
   3013.8 |########################################
  (0 below, 1 above range)

br_predicate_h_pred95 (n=6, range 4113.3-4908.8 ns)
   4113.3 |########################################
   4153.1 |
   4192.8 |
   4232.6 |
   4272.4 |
   4312.2 |
   4351.9 |
   4391.7 |
   4431.5 |
   4471.3 |
   4511.0 |
   4550.8 |#############
   4590.6 |
   4630.3 |
   4670.1 |
   4709.9 |
   4749.7 |
   4789.4 |
   4829.2 |
   4869.0 |#############
  (0 below, 1 above range)

br_profiled_hot_h_pred95 (n=6, range 2654.2-3203.3 ns)
   2654.2 |####################
   2681.7 |
   2709.1 |
   2736.6 |
   2764.0 |####################
   2791.5 |####################
   2818.9 |
   2846.4 |
   2873.9 |
   2901.3 |
   2928.8 |
   2956.2 |
   2983.7 |
   3011.1 |
   3038.6 |
   3066.1 |
   3093.5 |
   3121.0 |
   3148.4 |
   3175.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **br_predicate_h_pred95**: autocorrelation=0.59 (measurement drift or warm-up artifact)

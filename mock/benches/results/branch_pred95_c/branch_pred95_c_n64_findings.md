# Branch strategies, cheap-arm, pred95: ~95% taken, predictable (b<243)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred95**

## Key findings

- **Fastest: br_profiled_hot_c_pred95** at 108.0 ns median (-4.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.65x (fastest 108.0 ns, slowest 178.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 2573ns | 2680ns | 2259ns | 2540ns | 2780ns | base |
| br_lut_c_pred95 | 2653ns | 2758ns | 2314ns | 2614ns | 2880ns | +3.09% |
| br_mask_c_pred95 | 2628ns | 2751ns | 2349ns | 2634ns | 2758ns | +2.13% |
| br_predicate_c_pred95 | 2697ns | 2759ns | 2321ns | 2620ns | 3000ns | +4.80% |
| br_profiled_hot_c_pred95 | 2455ns | 2438ns | 2214ns | 2375ns | 2695ns | -4.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred95 | 110ns | 91ns | 125ns | base | 0.580 |
| br_lut_c_pred95 | 171ns | 148ns | 187ns | +54.91% | 0.374 |
| br_mask_c_pred95 | 170ns | 151ns | 179ns | +53.52% | 0.378 |
| br_predicate_c_pred95 | 172ns | 146ns | 192ns | +55.58% | 0.373 |
| br_profiled_hot_c_pred95 | 108ns | 93ns | 124ns | -1.74% | 0.590 |

## Performance model

- Peak throughput: **0.705 Gops/s** (br_branch_c_pred95; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred95 | 0.569 | 80.7% |
| br_lut_c_pred95 | 0.362 | 51.3% |
| br_mask_c_pred95 | 0.359 | 51.0% |
| br_predicate_c_pred95 | 0.364 | 51.6% |
| br_profiled_hot_c_pred95 | 0.593 | 84.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred95 | 2573ns | 2573ns | base |
| br_lut_c_pred95 | 2653ns | 2653ns | +3.09% |
| br_mask_c_pred95 | 2628ns | 2628ns | +2.13% |
| br_predicate_c_pred95 | 2697ns | 2697ns | +4.80% |
| br_profiled_hot_c_pred95 | 2455ns | 2455ns | -4.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 112ns | base | --- | [94, 125] | --- | --- | --- | --- |
| br_lut_c_pred95 | 177ns | +58.8ns (+52.3%) | [+51, +72]ns | [149, 187] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred95 | 178ns | +59.0ns (+52.4%) | [+52, +66]ns | [151, 179] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred95 | 176ns | +56.1ns (+49.8%) | [+48, +80]ns | [147, 192] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred95 | 108ns | no significant difference | [-14, +5]ns | [93, 124] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred95 | br_lut_c_pred95 | br_mask_c_pred95 | br_predicate_c_pred95 | br_profiled_hot_c_pred95 |
|---|---|---|---|---|---|
| 1 | 117ns | +66.5% | +52.7% | +65.1% | -20.7% |
| 2 | 121ns | +48.0% | +47.6% | +45.2% | +2.8% |
| 3 | 97ns | +51.9% | +56.2% | +50.6% | +4.7% |
| 4 | 91ns | +65.6% | +66.5% | +63.3% | +3.3% |
| 5 | 108ns | +62.2% | +64.9% | +76.8% | +5.8% |
| 6 | 129ns | +39.4% | +39.1% | +37.2% | -3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred95 | 0.185 | ok |
| br_lut_c_pred95 | 0.259 | moderate+ |
| br_mask_c_pred95 | 0.183 | ok |
| br_predicate_c_pred95 | 0.113 | ok |
| br_profiled_hot_c_pred95 | -0.239 | moderate- |

**Consistency summary:**

- **br_lut_c_pred95**: won 0/6, lost 6/6
- **br_mask_c_pred95**: won 0/6, lost 6/6
- **br_predicate_c_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred95**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred95 | 4.4ns | 110.4ns | 4.0% |  |
| br_lut_c_pred95 | 3.4ns | 171.1ns | 2.0% |  |
| br_mask_c_pred95 | 3.0ns | 169.5ns | 1.8% |  |
| br_predicate_c_pred95 | 2.9ns | 171.8ns | 1.7% |  |
| br_profiled_hot_c_pred95 | 4.2ns | 108.5ns | 3.9% |  |

## Distribution (algo ns)

```
br_branch_c_pred95 (n=6, range 90.8-124.8 ns)
     90.8 |########################################
     92.5 |
     94.2 |
     95.9 |########################################
     97.6 |
     99.3 |
    101.0 |
    102.7 |
    104.4 |
    106.1 |
    107.8 |########################################
    109.5 |
    111.2 |
    112.9 |
    114.6 |
    116.3 |########################################
    118.0 |
    119.7 |########################################
    121.4 |
    123.1 |
  (0 below, 1 above range)

br_lut_c_pred95 (n=6, range 147.5-187.3 ns)
    147.5 |########################################
    149.5 |########################################
    151.5 |
    153.5 |
    155.5 |
    157.4 |
    159.4 |
    161.4 |
    163.4 |
    165.4 |
    167.4 |
    169.4 |
    171.4 |
    173.4 |########################################
    175.4 |
    177.4 |########################################
    179.3 |########################################
    181.3 |
    183.3 |
    185.3 |
  (0 below, 1 above range)

br_mask_c_pred95 (n=6, range 151.2-179.0 ns)
    151.2 |##########################
    152.6 |
    154.0 |
    155.4 |
    156.8 |
    158.1 |
    159.5 |
    160.9 |
    162.3 |
    163.7 |
    165.1 |
    166.5 |
    167.9 |
    169.3 |
    170.7 |
    172.1 |
    173.4 |
    174.8 |
    176.2 |
    177.6 |########################################
  (0 below, 1 above range)

br_predicate_c_pred95 (n=6, range 146.2-192.1 ns)
    146.2 |########################################
    148.5 |
    150.8 |
    153.1 |
    155.4 |
    157.7 |
    160.0 |
    162.2 |
    164.5 |
    166.8 |
    169.1 |
    171.4 |
    173.7 |####################
    176.0 |####################
    178.3 |
    180.6 |
    182.9 |
    185.2 |
    187.5 |
    189.8 |####################
  (0 below, 1 above range)

br_profiled_hot_c_pred95 (n=6, range 92.9-124.2 ns)
     92.9 |########################################
     94.5 |
     96.0 |
     97.6 |
     99.2 |
    100.7 |####################
    102.3 |
    103.9 |
    105.4 |
    107.0 |
    108.6 |
    110.1 |
    111.7 |
    113.2 |####################
    114.8 |
    116.4 |
    117.9 |
    119.5 |
    121.1 |
    122.6 |
  (0 below, 2 above range)

```

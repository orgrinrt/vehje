# Branch strategies, cheap-arm, pred95: ~95% taken, predictable (b<243)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred95**

## Key findings

- **Fastest: br_profiled_hot_c_pred95** at 21848.3 ns median (-6.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.55x (fastest 21848.3 ns, slowest 33777.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 25618ns | 25766ns | 23688ns | 25165ns | 27261ns | base |
| br_lut_c_pred95 | 35272ns | 36305ns | 32300ns | 35076ns | 37053ns | +37.69% |
| br_mask_c_pred95 | 34511ns | 35096ns | 31833ns | 34224ns | 36279ns | +34.71% |
| br_predicate_c_pred95 | 34697ns | 35323ns | 32244ns | 34382ns | 36395ns | +35.44% |
| br_profiled_hot_c_pred95 | 24510ns | 24091ns | 23313ns | 23852ns | 26094ns | -4.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred95 | 23250ns | 21484ns | 24770ns | base | 0.705 |
| br_lut_c_pred95 | 32850ns | 30116ns | 34498ns | +41.29% | 0.499 |
| br_mask_c_pred95 | 32157ns | 29685ns | 33780ns | +38.31% | 0.510 |
| br_predicate_c_pred95 | 32331ns | 30064ns | 33892ns | +39.06% | 0.507 |
| br_profiled_hot_c_pred95 | 22206ns | 21039ns | 23660ns | -4.49% | 0.738 |

## Performance model

- Peak throughput: **0.779 Gops/s** (br_profiled_hot_c_pred95; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred95 | 0.700 | 89.9% |
| br_lut_c_pred95 | 0.485 | 62.3% |
| br_mask_c_pred95 | 0.501 | 64.3% |
| br_predicate_c_pred95 | 0.498 | 63.9% |
| br_profiled_hot_c_pred95 | 0.750 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred95 | 25618ns | 25618ns | base |
| br_lut_c_pred95 | 35272ns | 35272ns | +37.69% |
| br_mask_c_pred95 | 34511ns | 34511ns | +34.71% |
| br_predicate_c_pred95 | 34697ns | 34697ns | +35.44% |
| br_profiled_hot_c_pred95 | 24510ns | 24510ns | -4.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred95 | 23409ns | base | --- | [21571, 24770] | --- | --- | --- | --- |
| br_lut_c_pred95 | 33778ns | +9023.6ns (+38.5%) | [+8704, +11073]ns | [30275, 34498] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_pred95 | 32713ns | +8632.1ns (+36.9%) | [+7725, +10363]ns | [29977, 33780] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_pred95 | 32911ns | +8653.1ns (+37.0%) | [+8118, +10471]ns | [30189, 33892] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_c_pred95 | 21848ns | -1104.3ns (-4.7%) | [-1708, -318]ns | [21110, 23660] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred95 | br_lut_c_pred95 | br_mask_c_pred95 | br_predicate_c_pred95 | br_profiled_hot_c_pred95 |
|---|---|---|---|---|---|
| 1 | 22367ns | +51.5% | +52.0% | +49.9% | -5.3% |
| 2 | 25078ns | +35.3% | +33.8% | +33.8% | -4.5% |
| 3 | 24451ns | +43.4% | +37.2% | +40.0% | -4.5% |
| 4 | 24461ns | +37.6% | +30.4% | +32.0% | -9.1% |
| 5 | 21484ns | +40.2% | +40.9% | +41.1% | -2.1% |
| 6 | 21658ns | +40.5% | +37.1% | +38.8% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred95 | 0.214 | moderate+ |
| br_lut_c_pred95 | 0.459 | moderate+ |
| br_mask_c_pred95 | 0.548 | HIGH+ (drift/warm-up) |
| br_predicate_c_pred95 | 0.531 | HIGH+ (drift/warm-up) |
| br_profiled_hot_c_pred95 | 0.148 | ok |

**Consistency summary:**

- **br_lut_c_pred95**: won 0/6, lost 6/6
- **br_mask_c_pred95**: won 0/6, lost 6/6
- **br_predicate_c_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred95**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred95 | 5.4ns | 23249.9ns | 0.0% |  |
| br_lut_c_pred95 | 4.8ns | 32850.2ns | 0.0% |  |
| br_mask_c_pred95 | 5.0ns | 32156.5ns | 0.0% |  |
| br_predicate_c_pred95 | 3.6ns | 32330.7ns | 0.0% |  |
| br_profiled_hot_c_pred95 | 4.8ns | 22206.3ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_pred95 (n=6, range 21483.8-24769.5 ns)
  21483.8 |####################
  21648.1 |####################
  21812.4 |
  21976.7 |
  22141.0 |
  22305.2 |####################
  22469.5 |
  22633.8 |
  22798.1 |
  22962.4 |
  23126.7 |
  23291.0 |
  23455.2 |
  23619.5 |
  23783.8 |
  23948.1 |
  24112.4 |
  24276.7 |
  24441.0 |########################################
  24605.3 |
  (0 below, 1 above range)

br_lut_c_pred95 (n=6, range 30115.8-34497.5 ns)
  30115.8 |####################
  30334.9 |####################
  30554.0 |
  30773.1 |
  30992.1 |
  31211.2 |
  31430.3 |
  31649.4 |
  31868.5 |
  32087.6 |
  32306.7 |
  32525.7 |
  32744.8 |
  32963.9 |
  33183.0 |
  33402.1 |
  33621.2 |####################
  33840.2 |########################################
  34059.3 |
  34278.4 |
  (0 below, 1 above range)

br_mask_c_pred95 (n=6, range 29684.6-33779.8 ns)
  29684.6 |####################
  29889.4 |
  30094.1 |####################
  30298.9 |
  30503.6 |
  30708.4 |
  30913.2 |
  31117.9 |
  31322.7 |
  31527.4 |
  31732.2 |####################
  31937.0 |
  32141.7 |
  32346.5 |
  32551.2 |
  32756.0 |
  32960.8 |
  33165.5 |
  33370.3 |########################################
  33575.0 |
  (0 below, 1 above range)

br_predicate_c_pred95 (n=6, range 30064.2-33891.7 ns)
  30064.2 |####################
  30255.6 |####################
  30447.0 |
  30638.3 |
  30829.7 |
  31021.1 |
  31212.5 |
  31403.8 |
  31595.2 |
  31786.6 |
  31977.9 |
  32169.3 |####################
  32360.7 |
  32552.1 |
  32743.4 |
  32934.8 |
  33126.2 |
  33317.6 |
  33508.9 |########################################
  33700.3 |
  (0 below, 1 above range)

br_profiled_hot_c_pred95 (n=6, range 21038.8-23660.2 ns)
  21038.8 |########################################
  21169.9 |########################################
  21300.9 |
  21432.0 |########################################
  21563.1 |
  21694.2 |
  21825.2 |
  21956.3 |
  22087.4 |
  22218.5 |########################################
  22349.5 |
  22480.6 |
  22611.7 |
  22742.7 |
  22873.8 |
  23004.9 |
  23136.0 |
  23267.0 |########################################
  23398.1 |
  23529.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_mask_c_pred95**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **br_predicate_c_pred95**: autocorrelation=0.53 (measurement drift or warm-up artifact)

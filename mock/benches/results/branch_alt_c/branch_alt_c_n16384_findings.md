# Branch strategies, cheap-arm, alt: strict alternation (i&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_alt**

## Key findings

- **Baseline (br_branch_c_alt) is the fastest** at 18958.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.86x (fastest 18958.8 ns, slowest 35242.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_alt | 22011ns | 21484ns | 19078ns | 20990ns | 25008ns | base |
| br_lut_c_alt | 35014ns | 33910ns | 32145ns | 33670ns | 38464ns | +59.08% |
| br_mask_c_alt | 38263ns | 37581ns | 36143ns | 37508ns | 40455ns | +73.84% |
| br_predicate_c_alt | 34006ns | 33192ns | 32211ns | 33098ns | 36265ns | +54.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_alt | 19471ns | 16890ns | 22142ns | base | 0.841 |
| br_lut_c_alt | 32618ns | 29946ns | 35848ns | +67.52% | 0.502 |
| br_mask_c_alt | 35904ns | 33968ns | 37976ns | +84.40% | 0.456 |
| br_predicate_c_alt | 31668ns | 29995ns | 33775ns | +62.65% | 0.517 |

## Performance model

- Peak throughput: **0.970 Gops/s** (br_branch_c_alt; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_alt | 0.864 | 89.1% |
| br_lut_c_alt | 0.519 | 53.5% |
| br_mask_c_alt | 0.465 | 47.9% |
| br_predicate_c_alt | 0.530 | 54.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_alt | 22011ns | 22011ns | base |
| br_lut_c_alt | 35014ns | 35014ns | +59.08% |
| br_mask_c_alt | 38263ns | 38263ns | +73.84% |
| br_predicate_c_alt | 34006ns | 34006ns | +54.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_alt | 18959ns | base | --- | [17311, 22142] | --- | --- | --- | --- |
| br_lut_c_alt | 31541ns | +13154.2ns (+69.4%) | [+10008, +16280]ns | [30465, 35848] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_alt | 35242ns | +16987.7ns (+89.6%) | [+13594, +18718]ns | [34493, 37976] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_alt | 30890ns | +13029.4ns (+68.7%) | [+9357, +14207]ns | [30340, 33775] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_alt | br_lut_c_alt | br_mask_c_alt | br_predicate_c_alt |
|---|---|---|---|---|
| 1 | 24101ns | +31.3% | +45.4% | +27.7% |
| 2 | 18964ns | +65.8% | +86.8% | +63.5% |
| 3 | 17732ns | +74.7% | +91.6% | +73.0% |
| 4 | 16890ns | +77.3% | +107.3% | +77.6% |
| 5 | 20183ns | +76.7% | +86.7% | +69.1% |
| 6 | 18953ns | +90.1% | +101.9% | +76.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_alt | 0.025 | ok |
| br_lut_c_alt | 0.293 | moderate+ |
| br_mask_c_alt | 0.398 | moderate+ |
| br_predicate_c_alt | 0.220 | moderate+ |

**Consistency summary:**

- **br_lut_c_alt**: won 0/6, lost 6/6
- **br_mask_c_alt**: won 0/6, lost 6/6
- **br_predicate_c_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_alt | 6.8ns | 19470.5ns | 0.0% |  |
| br_lut_c_alt | 4.2ns | 32617.8ns | 0.0% |  |
| br_mask_c_alt | 3.8ns | 35903.7ns | 0.0% |  |
| br_predicate_c_alt | 3.7ns | 31668.5ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_alt (n=6, range 16889.6-22142.1 ns)
  16889.6 |####################
  17152.2 |
  17414.8 |
  17677.5 |####################
  17940.1 |
  18202.7 |
  18465.3 |
  18728.0 |########################################
  18990.6 |
  19253.2 |
  19515.8 |
  19778.4 |
  20041.1 |####################
  20303.7 |
  20566.3 |
  20828.9 |
  21091.6 |
  21354.2 |
  21616.8 |
  21879.4 |
  (0 below, 1 above range)

br_lut_c_alt (n=6, range 29945.8-35847.7 ns)
  29945.8 |####################
  30240.9 |
  30536.0 |
  30831.1 |####################
  31126.2 |
  31421.3 |########################################
  31716.4 |
  32011.5 |
  32306.6 |
  32601.7 |
  32896.8 |
  33191.8 |
  33486.9 |
  33782.0 |
  34077.1 |
  34372.2 |
  34667.3 |
  34962.4 |
  35257.5 |
  35552.6 |####################
  (0 below, 1 above range)

br_mask_c_alt (n=6, range 33968.3-37975.6 ns)
  33968.3 |####################
  34168.7 |
  34369.0 |
  34569.4 |
  34769.8 |
  34970.1 |########################################
  35170.5 |
  35370.9 |####################
  35571.2 |
  35771.6 |
  35972.0 |
  36172.3 |
  36372.7 |
  36573.0 |
  36773.4 |
  36973.8 |
  37174.1 |
  37374.5 |
  37574.9 |####################
  37775.2 |
  (0 below, 1 above range)

br_predicate_c_alt (n=6, range 29995.4-33775.2 ns)
  29995.4 |########################################
  30184.4 |
  30373.4 |
  30562.4 |########################################
  30751.4 |########################################
  30940.3 |########################################
  31129.3 |
  31318.3 |
  31507.3 |
  31696.3 |
  31885.3 |
  32074.3 |
  32263.3 |
  32452.3 |
  32641.3 |
  32830.2 |
  33019.2 |
  33208.2 |
  33397.2 |########################################
  33586.2 |
  (0 below, 1 above range)

```

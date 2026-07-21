# Branch strategies, heavy-arm, pred05: ~5% taken, predictable (b<13)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred05**

## Key findings

- **Baseline (br_branch_h_pred05) is the fastest** at 103235.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.61x (fastest 103235.2 ns, slowest 269349.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 106551ns | 105623ns | 105121ns | 105472ns | 108885ns | base |
| br_predicate_h_pred05 | 270241ns | 271940ns | 262300ns | 270603ns | 273669ns | +153.63% |
| br_profiled_hot_h_pred05 | 114530ns | 115037ns | 107693ns | 113646ns | 119273ns | +7.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred05 | 104169ns | 102712ns | 106543ns | base | 0.157 |
| br_predicate_h_pred05 | 267729ns | 259732ns | 271184ns | +157.01% | 0.061 |
| br_profiled_hot_h_pred05 | 112070ns | 105424ns | 116571ns | +7.58% | 0.146 |

## Performance model

- Peak throughput: **0.160 Gops/s** (br_branch_h_pred05; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred05 | 0.159 | 99.5% |
| br_predicate_h_pred05 | 0.061 | 38.1% |
| br_profiled_hot_h_pred05 | 0.145 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred05 | 106551ns | 106551ns | base |
| br_predicate_h_pred05 | 270241ns | 270241ns | +153.63% |
| br_profiled_hot_h_pred05 | 114530ns | 114530ns | +7.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 103235ns | base | --- | [102729, 106543] | --- | --- | --- | --- |
| br_predicate_h_pred05 | 269350ns | +164641.4ns (+159.5%) | [+159728, +166312]ns | [262654, 271184] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_pred05 | 112653ns | +7815.0ns (+7.6%) | [+3228, +12660]ns | [106986, 116571] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred05 | br_predicate_h_pred05 | br_profiled_hot_h_pred05 |
|---|---|---|---|
| 1 | 102747ns | +161.5% | +5.6% |
| 2 | 102712ns | +158.6% | +13.2% |
| 3 | 107012ns | +154.0% | +9.2% |
| 4 | 106074ns | +155.1% | +4.1% |
| 5 | 103329ns | +161.3% | +2.0% |
| 6 | 103141ns | +151.8% | +11.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred05 | 0.148 | ok |
| br_predicate_h_pred05 | -0.111 | ok |
| br_profiled_hot_h_pred05 | -0.095 | ok |

**Consistency summary:**

- **br_predicate_h_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred05**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred05 | 4.2ns | 104169.1ns | 0.0% |  |
| br_predicate_h_pred05 | 7.3ns | 267729.4ns | 0.0% |  |
| br_profiled_hot_h_pred05 | 5.3ns | 112070.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_pred05 (n=6, range 102711.7-106542.9 ns)
  102711.7 |########################################
  102903.3 |
  103094.8 |####################
  103286.4 |####################
  103477.9 |
  103669.5 |
  103861.1 |
  104052.6 |
  104244.2 |
  104435.8 |
  104627.3 |
  104818.9 |
  105010.4 |
  105202.0 |
  105393.6 |
  105585.1 |
  105776.7 |
  105968.3 |####################
  106159.8 |
  106351.4 |
  (0 below, 1 above range)

br_predicate_h_pred05 (n=6, range 259731.7-271184.3 ns)
  259731.7 |########################################
  260304.3 |
  260877.0 |
  261449.6 |
  262022.2 |
  262594.9 |
  263167.5 |
  263740.1 |
  264312.8 |
  264885.4 |
  265458.0 |########################################
  266030.7 |
  266603.3 |
  267175.9 |
  267748.6 |
  268321.2 |########################################
  268893.8 |
  269466.5 |########################################
  270039.1 |########################################
  270611.7 |
  (0 below, 1 above range)

br_profiled_hot_h_pred05 (n=6, range 105423.8-116570.6 ns)
  105423.8 |########################################
  105981.1 |
  106538.5 |
  107095.8 |
  107653.2 |
  108210.5 |########################################
  108767.9 |
  109325.2 |
  109882.5 |########################################
  110439.9 |
  110997.2 |
  111554.6 |
  112111.9 |
  112669.3 |
  113226.6 |
  113783.9 |
  114341.3 |########################################
  114898.6 |
  115456.0 |
  116013.3 |########################################
  (0 below, 1 above range)

```

# Branch strategies, heavy-arm, pred05: ~5% taken, predictable (b<13)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred05**

## Key findings

- **Baseline (br_branch_h_pred05) is the fastest** at 26797.7 ns median
- 1 variant significantly slower than baseline
- Spread: 2.52x (fastest 26797.7 ns, slowest 67655.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 29281ns | 29054ns | 27418ns | 28648ns | 31162ns | base |
| br_predicate_h_pred05 | 70882ns | 69945ns | 67951ns | 69618ns | 74242ns | +142.07% |
| br_profiled_hot_h_pred05 | 29903ns | 29305ns | 28703ns | 29154ns | 31628ns | +2.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred05 | 26993ns | 25280ns | 28715ns | base | 0.152 |
| br_predicate_h_pred05 | 68536ns | 65709ns | 71722ns | +153.90% | 0.060 |
| br_profiled_hot_h_pred05 | 27629ns | 26508ns | 29197ns | +2.35% | 0.148 |

## Performance model

- Peak throughput: **0.162 Gops/s** (br_branch_h_pred05; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred05 | 0.153 | 94.3% |
| br_predicate_h_pred05 | 0.061 | 37.4% |
| br_profiled_hot_h_pred05 | 0.151 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred05 | 29281ns | 29281ns | base |
| br_predicate_h_pred05 | 70882ns | 70882ns | +142.07% |
| br_profiled_hot_h_pred05 | 29903ns | 29903ns | +2.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 26798ns | base | --- | [25468, 28715] | --- | --- | --- | --- |
| br_predicate_h_pred05 | 67655ns | +40890.0ns (+152.6%) | [+39741, +43995]ns | [66229, 71722] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred05 | 27095ns | no significant difference | [-549, +1928]ns | [26594, 29197] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred05 | br_predicate_h_pred05 | br_profiled_hot_h_pred05 |
|---|---|---|---|
| 1 | 26794ns | +164.7% | -1.1% |
| 2 | 28660ns | +153.0% | +6.2% |
| 3 | 28769ns | +135.7% | -2.8% |
| 4 | 26802ns | +151.8% | +0.1% |
| 5 | 25655ns | +160.2% | +4.0% |
| 6 | 25280ns | +159.9% | +8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred05 | 0.451 | moderate+ |
| br_predicate_h_pred05 | 0.418 | moderate+ |
| br_profiled_hot_h_pred05 | -0.136 | ok |

**Consistency summary:**

- **br_predicate_h_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred05**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred05 | 3.7ns | 26993.4ns | 0.0% |  |
| br_predicate_h_pred05 | 3.5ns | 68535.6ns | 0.0% |  |
| br_profiled_hot_h_pred05 | 3.7ns | 27628.7ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_pred05 (n=6, range 25280.4-28714.8 ns)
  25280.4 |####################
  25452.1 |
  25623.8 |####################
  25795.6 |
  25967.3 |
  26139.0 |
  26310.7 |
  26482.4 |
  26654.2 |########################################
  26825.9 |
  26997.6 |
  27169.3 |
  27341.0 |
  27512.8 |
  27684.5 |
  27856.2 |
  28027.9 |
  28199.6 |
  28371.4 |
  28543.1 |####################
  (0 below, 1 above range)

br_predicate_h_pred05 (n=6, range 65709.2-71722.5 ns)
  65709.2 |########################################
  66009.9 |
  66310.5 |
  66611.2 |########################################
  66911.9 |
  67212.5 |########################################
  67513.2 |
  67813.9 |########################################
  68114.5 |
  68415.2 |
  68715.9 |
  69016.5 |
  69317.2 |
  69617.8 |
  69918.5 |
  70219.2 |
  70519.8 |
  70820.5 |########################################
  71121.2 |
  71421.8 |
  (0 below, 1 above range)

br_profiled_hot_h_pred05 (n=6, range 26507.9-29196.7 ns)
  26507.9 |########################################
  26642.3 |########################################
  26776.8 |########################################
  26911.2 |
  27045.7 |
  27180.1 |
  27314.5 |########################################
  27449.0 |
  27583.4 |
  27717.8 |
  27852.3 |########################################
  27986.7 |
  28121.2 |
  28255.6 |
  28390.0 |
  28524.5 |
  28658.9 |
  28793.3 |
  28927.8 |
  29062.2 |
  (0 below, 1 above range)

```

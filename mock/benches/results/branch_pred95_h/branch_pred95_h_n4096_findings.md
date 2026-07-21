# Branch strategies, heavy-arm, pred95: ~95% taken, predictable (b<243)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred95**

## Key findings

- **Baseline (br_branch_h_pred95) is the fastest** at 42673.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.62x (fastest 42673.3 ns, slowest 69284.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 44964ns | 45034ns | 42561ns | 44369ns | 47057ns | base |
| br_predicate_h_pred95 | 71386ns | 71642ns | 67568ns | 71024ns | 73839ns | +58.76% |
| br_profiled_hot_h_pred95 | 46844ns | 46664ns | 44957ns | 46464ns | 48359ns | +4.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred95 | 42639ns | 40390ns | 44634ns | base | 0.096 |
| br_predicate_h_pred95 | 69021ns | 65389ns | 71341ns | +61.87% | 0.059 |
| br_profiled_hot_h_pred95 | 44529ns | 42769ns | 45928ns | +4.43% | 0.092 |

## Performance model

- Peak throughput: **0.101 Gops/s** (br_branch_h_pred95; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred95 | 0.096 | 94.6% |
| br_predicate_h_pred95 | 0.059 | 58.3% |
| br_profiled_hot_h_pred95 | 0.092 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred95 | 44964ns | 44964ns | base |
| br_predicate_h_pred95 | 71386ns | 71386ns | +58.76% |
| br_profiled_hot_h_pred95 | 46844ns | 46844ns | +4.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 42673ns | base | --- | [40610, 44634] | --- | --- | --- | --- |
| br_predicate_h_pred95 | 69284ns | +26557.9ns (+62.2%) | [+24836, +27750]ns | [66437, 71341] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred95 | 44378ns | no significant difference | [-330, +3407]ns | [43282, 45928] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred95 | br_predicate_h_pred95 | br_profiled_hot_h_pred95 |
|---|---|---|---|
| 1 | 40390ns | +61.9% | +5.9% |
| 2 | 43742ns | +56.4% | +6.4% |
| 3 | 42212ns | +66.2% | +7.3% |
| 4 | 45527ns | +58.1% | -2.9% |
| 5 | 43134ns | +63.9% | +1.5% |
| 6 | 40831ns | +65.3% | +9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred95 | -0.199 | ok |
| br_predicate_h_pred95 | 0.252 | moderate+ |
| br_profiled_hot_h_pred95 | -0.239 | moderate- |

**Consistency summary:**

- **br_predicate_h_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred95**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred95 | 3.9ns | 42639.2ns | 0.0% |  |
| br_predicate_h_pred95 | 4.2ns | 69020.8ns | 0.0% |  |
| br_profiled_hot_h_pred95 | 3.0ns | 44529.4ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_pred95 (n=6, range 40389.6-44634.2 ns)
  40389.6 |########################################
  40601.8 |
  40814.1 |########################################
  41026.3 |
  41238.5 |
  41450.8 |
  41663.0 |
  41875.2 |
  42087.4 |########################################
  42299.7 |
  42511.9 |
  42724.1 |
  42936.4 |########################################
  43148.6 |
  43360.8 |
  43573.0 |########################################
  43785.3 |
  43997.5 |
  44209.7 |
  44422.0 |
  (0 below, 1 above range)

br_predicate_h_pred95 (n=6, range 65389.2-71341.1 ns)
  65389.2 |########################################
  65686.8 |
  65984.4 |
  66282.0 |
  66579.6 |
  66877.2 |
  67174.8 |
  67472.3 |########################################
  67769.9 |
  68067.5 |
  68365.1 |########################################
  68662.7 |
  68960.3 |
  69257.9 |
  69555.5 |
  69853.1 |
  70150.7 |########################################
  70448.3 |########################################
  70745.9 |
  71043.5 |
  (0 below, 1 above range)

br_profiled_hot_h_pred95 (n=6, range 42769.2-45927.8 ns)
  42769.2 |########################################
  42927.1 |
  43085.1 |
  43243.0 |
  43400.9 |
  43558.8 |
  43716.8 |########################################
  43874.7 |
  44032.6 |
  44190.5 |########################################
  44348.5 |
  44506.4 |########################################
  44664.3 |
  44822.3 |
  44980.2 |
  45138.1 |
  45296.0 |########################################
  45454.0 |
  45611.9 |
  45769.8 |
  (0 below, 1 above range)

```

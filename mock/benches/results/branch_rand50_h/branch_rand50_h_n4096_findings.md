# Branch strategies, heavy-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_rand50**

## Key findings

- **Baseline (br_branch_h_rand50) is the fastest** at 35316.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.93x (fastest 35316.7 ns, slowest 68155.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 38092ns | 37521ns | 36472ns | 37446ns | 39871ns | base |
| br_predicate_h_rand50 | 69859ns | 70431ns | 67331ns | 69505ns | 71653ns | +83.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_rand50 | 35815ns | 34290ns | 37524ns | base | 0.114 |
| br_predicate_h_rand50 | 67504ns | 65000ns | 69190ns | +88.48% | 0.061 |

## Performance model

- Peak throughput: **0.119 Gops/s** (br_branch_h_rand50; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_rand50 | 0.116 | 97.1% |
| br_predicate_h_rand50 | 0.060 | 50.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_rand50 | 38092ns | 38092ns | base |
| br_predicate_h_rand50 | 69859ns | 69859ns | +83.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 35317ns | base | --- | [34604, 37524] | --- | --- | --- | --- |
| br_predicate_h_rand50 | 68155ns | +31378.6ns (+88.8%) | [+30075, +33614]ns | [65166, 69190] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_rand50 | br_predicate_h_rand50 |
|---|---|---|
| 1 | 34290ns | +89.6% |
| 2 | 35275ns | +92.5% |
| 3 | 34918ns | +87.1% |
| 4 | 36385ns | +88.1% |
| 5 | 38662ns | +76.9% |
| 6 | 35358ns | +97.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_rand50 | 0.093 | ok |
| br_predicate_h_rand50 | -0.049 | ok |

**Consistency summary:**

- **br_predicate_h_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_rand50 | 3.6ns | 35814.8ns | 0.0% |  |
| br_predicate_h_rand50 | 3.5ns | 67503.8ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_rand50 (n=6, range 34290.4-37523.6 ns)
  34290.4 |####################
  34452.1 |
  34613.7 |
  34775.4 |####################
  34937.0 |
  35098.7 |
  35260.3 |########################################
  35422.0 |
  35583.7 |
  35745.3 |
  35907.0 |
  36068.6 |
  36230.3 |####################
  36391.9 |
  36553.6 |
  36715.3 |
  36876.9 |
  37038.6 |
  37200.2 |
  37361.9 |
  (0 below, 1 above range)

br_predicate_h_rand50 (n=6, range 65000.0-69190.0 ns)
  65000.0 |####################
  65209.5 |####################
  65419.0 |
  65628.5 |
  65838.0 |
  66047.5 |
  66257.0 |
  66466.5 |
  66676.0 |
  66885.5 |
  67095.0 |
  67304.5 |
  67514.0 |
  67723.5 |####################
  67933.0 |
  68142.5 |
  68352.0 |########################################
  68561.5 |
  68771.0 |
  68980.5 |
  (0 below, 1 above range)

```

# Branch strategies, heavy-arm, alt: strict alternation (i&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_alt**

## Key findings

- **Baseline (br_branch_h_alt) is the fastest** at 134594.0 ns median
- 1 variant significantly slower than baseline
- Spread: 2.00x (fastest 134594.0 ns, slowest 269224.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_alt | 137003ns | 137044ns | 134859ns | 136718ns | 138501ns | base |
| br_predicate_h_alt | 275229ns | 271558ns | 266265ns | 270510ns | 286791ns | +100.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_alt | 134557ns | 132208ns | 136109ns | base | 0.122 |
| br_predicate_h_alt | 272632ns | 263682ns | 283933ns | +102.61% | 0.060 |

## Performance model

- Peak throughput: **0.124 Gops/s** (br_branch_h_alt; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_alt | 0.122 | 98.2% |
| br_predicate_h_alt | 0.061 | 49.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_alt | 137003ns | 137003ns | base |
| br_predicate_h_alt | 275229ns | 275229ns | +100.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_alt | 134594ns | base | --- | [132969, 136109] | --- | --- | --- | --- |
| br_predicate_h_alt | 269224ns | +134022.8ns (+99.6%) | [+130821, +149381]ns | [264739, 283933] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_alt | br_predicate_h_alt |
|---|---|---|
| 1 | 135373ns | +119.9% |
| 2 | 135629ns | +96.0% |
| 3 | 133815ns | +101.6% |
| 4 | 136588ns | +96.7% |
| 5 | 133730ns | +102.0% |
| 6 | 132208ns | +99.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_alt | -0.092 | ok |
| br_predicate_h_alt | -0.138 | ok |

**Consistency summary:**

- **br_predicate_h_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_alt | 5.8ns | 134557.2ns | 0.0% |  |
| br_predicate_h_alt | 15.4ns | 272632.2ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_alt (n=6, range 132207.9-136108.5 ns)
  132207.9 |########################################
  132402.9 |
  132598.0 |
  132793.0 |
  132988.0 |
  133183.1 |
  133378.1 |
  133573.1 |########################################
  133768.2 |########################################
  133963.2 |
  134158.2 |
  134353.3 |
  134548.3 |
  134743.3 |
  134938.4 |
  135133.4 |
  135328.4 |########################################
  135523.5 |########################################
  135718.5 |
  135913.5 |
  (0 below, 1 above range)

br_predicate_h_alt (n=6, range 263682.5-283932.9 ns)
  263682.5 |####################
  264695.0 |
  265707.5 |####################
  266720.1 |
  267732.6 |####################
  268745.1 |
  269757.6 |########################################
  270770.1 |
  271782.7 |
  272795.2 |
  273807.7 |
  274820.2 |
  275832.7 |
  276845.3 |
  277857.8 |
  278870.3 |
  279882.8 |
  280895.3 |
  281907.9 |
  282920.4 |
  (0 below, 1 above range)

```

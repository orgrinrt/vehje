# Branch strategies, heavy-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_rand50**

## Key findings

- **Baseline (br_branch_h_rand50) is the fastest** at 145865.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.79x (fastest 145865.5 ns, slowest 261322.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 148751ns | 148215ns | 146985ns | 147820ns | 151031ns | base |
| br_predicate_h_rand50 | 266963ns | 263953ns | 260371ns | 263624ns | 275268ns | +79.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_rand50 | 146375ns | 144467ns | 148616ns | base | 0.112 |
| br_predicate_h_rand50 | 264536ns | 258007ns | 272933ns | +80.73% | 0.062 |

## Performance model

- Peak throughput: **0.113 Gops/s** (br_branch_h_rand50; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_rand50 | 0.112 | 99.0% |
| br_predicate_h_rand50 | 0.063 | 55.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_rand50 | 148751ns | 148751ns | base |
| br_predicate_h_rand50 | 266963ns | 266963ns | +79.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 145865ns | base | --- | [144642, 148616] | --- | --- | --- | --- |
| br_predicate_h_rand50 | 261322ns | +116680.4ns (+80.0%) | [+113440, +124363]ns | [259351, 272933] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_rand50 | br_predicate_h_rand50 |
|---|---|---|
| 1 | 150254ns | +83.1% |
| 2 | 146887ns | +84.3% |
| 3 | 144467ns | +80.9% |
| 4 | 146979ns | +77.4% |
| 5 | 144844ns | +78.1% |
| 6 | 144817ns | +80.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_rand50 | 0.055 | ok |
| br_predicate_h_rand50 | 0.455 | moderate+ |

**Consistency summary:**

- **br_predicate_h_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_rand50 | 3.5ns | 146374.6ns | 0.0% |  |
| br_predicate_h_rand50 | 17.9ns | 264535.5ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_rand50 (n=6, range 144466.7-148616.5 ns)
  144466.7 |####################
  144674.2 |########################################
  144881.7 |
  145089.2 |
  145296.7 |
  145504.2 |
  145711.6 |
  145919.1 |
  146126.6 |
  146334.1 |
  146541.6 |
  146749.1 |####################
  146956.6 |####################
  147164.1 |
  147371.6 |
  147579.0 |
  147786.5 |
  147994.0 |
  148201.5 |
  148409.0 |
  (0 below, 1 above range)

br_predicate_h_rand50 (n=6, range 258007.1-272933.3 ns)
  258007.1 |####################
  258753.4 |
  259499.7 |
  260246.0 |####################
  260992.3 |########################################
  261738.6 |
  262485.0 |
  263231.3 |
  263977.6 |
  264723.9 |
  265470.2 |
  266216.5 |
  266962.8 |
  267709.1 |
  268455.4 |
  269201.8 |
  269948.1 |####################
  270694.4 |
  271440.7 |
  272187.0 |
  (0 below, 1 above range)

```

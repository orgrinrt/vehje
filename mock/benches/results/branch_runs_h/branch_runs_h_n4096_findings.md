# Branch strategies, heavy-arm, runs: correlated long runs (flip when b<24)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_runs**

## Key findings

- **Baseline (br_branch_h_runs) is the fastest** at 34149.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.95x (fastest 34149.2 ns, slowest 66708.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_runs | 36714ns | 36334ns | 35514ns | 36290ns | 37949ns | base |
| br_predicate_h_runs | 71850ns | 68923ns | 68355ns | 68743ns | 78258ns | +95.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_runs | 34497ns | 33308ns | 35678ns | base | 0.119 |
| br_predicate_h_runs | 69526ns | 66124ns | 75732ns | +101.54% | 0.059 |

## Performance model

- Peak throughput: **0.123 Gops/s** (br_branch_h_runs; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_runs | 0.120 | 97.5% |
| br_predicate_h_runs | 0.061 | 49.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_runs | 36714ns | 36714ns | base |
| br_predicate_h_runs | 71850ns | 71850ns | +95.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_runs | 34149ns | base | --- | [33665, 35678] | --- | --- | --- | --- |
| br_predicate_h_runs | 66709ns | +32383.4ns (+94.8%) | [+31843, +40860]ns | [66137, 75732] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_runs | br_predicate_h_runs |
|---|---|---|
| 1 | 33308ns | +105.4% |
| 2 | 36437ns | +127.9% |
| 3 | 34023ns | +96.2% |
| 4 | 34128ns | +93.8% |
| 5 | 34918ns | +90.9% |
| 6 | 34171ns | +93.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_runs | -0.575 | HIGH- (thermal bounce) |
| br_predicate_h_runs | -0.105 | ok |

**Consistency summary:**

- **br_predicate_h_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_runs | 2.8ns | 34497.3ns | 0.0% |  |
| br_predicate_h_runs | 8.0ns | 69525.9ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_runs (n=6, range 33307.5-35677.7 ns)
  33307.5 |####################
  33426.0 |
  33544.5 |
  33663.0 |
  33781.5 |
  33900.1 |
  34018.6 |########################################
  34137.1 |####################
  34255.6 |
  34374.1 |
  34492.6 |
  34611.1 |
  34729.6 |
  34848.1 |####################
  34966.6 |
  35085.1 |
  35203.7 |
  35322.2 |
  35440.7 |
  35559.2 |
  (0 below, 1 above range)

br_predicate_h_runs (n=6, range 66124.2-75731.9 ns)
  66124.2 |########################################
  66604.6 |########################################
  67085.0 |
  67565.4 |
  68045.7 |####################
  68526.1 |
  69006.5 |
  69486.9 |
  69967.3 |
  70447.7 |
  70928.0 |
  71408.4 |
  71888.8 |
  72369.2 |
  72849.6 |
  73330.0 |
  73810.4 |
  74290.7 |
  74771.1 |
  75251.5 |
  (0 below, 1 above range)

```

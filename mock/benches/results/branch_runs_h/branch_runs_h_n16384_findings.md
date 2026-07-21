# Branch strategies, heavy-arm, runs: correlated long runs (flip when b<24)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_runs**

## Key findings

- **Baseline (br_branch_h_runs) is the fastest** at 139823.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 139823.8 ns, slowest 266979.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_runs | 141985ns | 142125ns | 139025ns | 141863ns | 143649ns | base |
| br_predicate_h_runs | 271354ns | 269543ns | 262419ns | 268355ns | 280322ns | +91.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_runs | 139567ns | 136584ns | 141128ns | base | 0.117 |
| br_predicate_h_runs | 268822ns | 259805ns | 277917ns | +92.61% | 0.061 |

## Performance model

- Peak throughput: **0.120 Gops/s** (br_branch_h_runs; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_runs | 0.117 | 97.7% |
| br_predicate_h_runs | 0.061 | 51.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_runs | 141985ns | 141985ns | base |
| br_predicate_h_runs | 271354ns | 271354ns | +91.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_runs | 139824ns | base | --- | [137750, 141128] | --- | --- | --- | --- |
| br_predicate_h_runs | 266980ns | +125851.8ns (+90.0%) | [+123819, +138093]ns | [261570, 277917] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_runs | br_predicate_h_runs |
|---|---|---|
| 1 | 139865ns | +105.0% |
| 2 | 141313ns | +88.6% |
| 3 | 136584ns | +90.2% |
| 4 | 138917ns | +89.6% |
| 5 | 140942ns | +89.8% |
| 6 | 139783ns | +92.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_runs | -0.232 | moderate- |
| br_predicate_h_runs | 0.082 | ok |

**Consistency summary:**

- **br_predicate_h_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_runs | 5.3ns | 139567.3ns | 0.0% |  |
| br_predicate_h_runs | 6.7ns | 268822.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_runs (n=6, range 136584.2-141127.7 ns)
  136584.2 |####################
  136811.4 |
  137038.6 |
  137265.7 |
  137492.9 |
  137720.1 |
  137947.2 |
  138174.4 |
  138401.6 |
  138628.8 |
  138856.0 |####################
  139083.1 |
  139310.3 |
  139537.5 |
  139764.7 |########################################
  139991.8 |
  140219.0 |
  140446.2 |
  140673.4 |
  140900.5 |####################
  (0 below, 1 above range)

br_predicate_h_runs (n=6, range 259804.6-277917.1 ns)
  259804.6 |########################################
  260710.2 |
  261615.9 |
  262521.5 |########################################
  263427.1 |
  264332.7 |
  265238.3 |
  266144.0 |########################################
  267049.6 |########################################
  267955.2 |
  268860.8 |########################################
  269766.5 |
  270672.1 |
  271577.7 |
  272483.3 |
  273389.0 |
  274294.6 |
  275200.2 |
  276105.8 |
  277011.5 |
  (0 below, 1 above range)

```

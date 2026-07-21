# Branch strategies, heavy-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_rand50**

## Key findings

- **Baseline (br_branch_h_rand50) is the fastest** at 2503.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.96x (fastest 2503.9 ns, slowest 4900.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 5074ns | 5098ns | 4395ns | 5005ns | 5516ns | base |
| br_predicate_h_rand50 | 7547ns | 7488ns | 6909ns | 7484ns | 7960ns | +48.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_rand50 | 2497ns | 2144ns | 2718ns | base | 0.103 |
| br_predicate_h_rand50 | 4967ns | 4617ns | 5242ns | +98.96% | 0.052 |

## Performance model

- Peak throughput: **0.119 Gops/s** (br_branch_h_rand50; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_rand50 | 0.102 | 85.6% |
| br_predicate_h_rand50 | 0.052 | 43.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_rand50 | 5074ns | 5074ns | base |
| br_predicate_h_rand50 | 7547ns | 7547ns | +48.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 2504ns | base | --- | [2267, 2718] | --- | --- | --- | --- |
| br_predicate_h_rand50 | 4900ns | +2443.9ns (+97.6%) | [+2266, +2702]ns | [4758, 5242] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_rand50 | br_predicate_h_rand50 |
|---|---|---|
| 1 | 2144ns | +115.3% |
| 2 | 2522ns | +94.3% |
| 3 | 2690ns | +107.5% |
| 4 | 2485ns | +97.2% |
| 5 | 2746ns | +78.4% |
| 6 | 2390ns | +105.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_rand50 | -0.151 | ok |
| br_predicate_h_rand50 | -0.096 | ok |

**Consistency summary:**

- **br_predicate_h_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_rand50 | 5.1ns | 2496.5ns | 0.2% |  |
| br_predicate_h_rand50 | 4.4ns | 4967.1ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_h_rand50 (n=6, range 2144.2-2718.3 ns)
   2144.2 |########################################
   2172.9 |
   2201.6 |
   2230.3 |
   2259.0 |
   2287.7 |
   2316.4 |
   2345.1 |
   2373.8 |########################################
   2402.5 |
   2431.2 |
   2460.0 |########################################
   2488.7 |
   2517.4 |########################################
   2546.1 |
   2574.8 |
   2603.5 |
   2632.2 |
   2660.9 |
   2689.6 |########################################
  (0 below, 1 above range)

br_predicate_h_rand50 (n=6, range 4617.1-5242.5 ns)
   4617.1 |##########
   4648.4 |
   4679.6 |
   4710.9 |
   4742.2 |
   4773.5 |
   4804.7 |
   4836.0 |
   4867.3 |
   4898.5 |########################################
   4929.8 |
   4961.1 |
   4992.3 |
   5023.6 |
   5054.9 |
   5086.1 |
   5117.4 |
   5148.7 |
   5180.0 |
   5211.2 |
  (0 below, 1 above range)

```

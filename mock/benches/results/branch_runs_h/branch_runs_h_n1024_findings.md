# Branch strategies, heavy-arm, runs: correlated long runs (flip when b<24)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_runs**

## Key findings

- **Baseline (br_branch_h_runs) is the fastest** at 10273.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.93x (fastest 10273.5 ns, slowest 19872.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_runs | 12388ns | 12873ns | 10354ns | 12379ns | 13419ns | base |
| br_predicate_h_runs | 21496ns | 22479ns | 18375ns | 21218ns | 23474ns | +73.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_runs | 9850ns | 8202ns | 10694ns | base | 0.104 |
| br_predicate_h_runs | 18998ns | 16225ns | 20750ns | +92.87% | 0.054 |

## Performance model

- Peak throughput: **0.125 Gops/s** (br_branch_h_runs; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_runs | 0.100 | 79.8% |
| br_predicate_h_runs | 0.052 | 41.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_runs | 12388ns | 12388ns | base |
| br_predicate_h_runs | 21496ns | 21496ns | +73.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_runs | 10274ns | base | --- | [8583, 10694] | --- | --- | --- | --- |
| br_predicate_h_runs | 19873ns | +9454.1ns (+92.0%) | [+7789, +10201]ns | [16371, 20750] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_runs | br_predicate_h_runs |
|---|---|---|
| 1 | 8963ns | +81.0% |
| 2 | 10496ns | +88.7% |
| 3 | 10342ns | +92.9% |
| 4 | 8202ns | +101.4% |
| 5 | 10205ns | +95.5% |
| 6 | 10892ns | +97.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_runs | -0.238 | moderate- |
| br_predicate_h_runs | -0.164 | ok |

**Consistency summary:**

- **br_predicate_h_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_runs | 4.4ns | 9850.2ns | 0.0% |  |
| br_predicate_h_runs | 2.8ns | 18998.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_runs (n=6, range 8202.5-10694.1 ns)
   8202.5 |########################################
   8327.1 |
   8451.7 |
   8576.2 |
   8700.8 |
   8825.4 |
   8950.0 |########################################
   9074.6 |
   9199.2 |
   9323.7 |
   9448.3 |
   9572.9 |
   9697.5 |
   9822.1 |
   9946.7 |
  10071.2 |
  10195.8 |########################################
  10320.4 |########################################
  10445.0 |########################################
  10569.6 |
  (0 below, 1 above range)

br_predicate_h_runs (n=6, range 16225.0-20750.0 ns)
  16225.0 |####################
  16451.2 |####################
  16677.5 |
  16903.7 |
  17130.0 |
  17356.2 |
  17582.5 |
  17808.7 |
  18035.0 |
  18261.2 |
  18487.5 |
  18713.7 |
  18940.0 |
  19166.2 |
  19392.5 |
  19618.7 |####################
  19845.0 |########################################
  20071.2 |
  20297.5 |
  20523.7 |
  (0 below, 1 above range)

```

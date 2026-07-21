# Branch strategies, heavy-arm, runs: correlated long runs (flip when b<24)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_runs**

## Key findings

- **Baseline (br_branch_h_runs) is the fastest** at 629.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.15x (fastest 629.6 ns, slowest 1354.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_runs | 3337ns | 3446ns | 2892ns | 3343ns | 3549ns | base |
| br_predicate_h_runs | 3957ns | 4166ns | 3158ns | 4076ns | 4178ns | +18.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_runs | 628ns | 472ns | 736ns | base | 0.102 |
| br_predicate_h_runs | 1287ns | 1028ns | 1358ns | +104.96% | 0.050 |

## Performance model

- Peak throughput: **0.136 Gops/s** (br_branch_h_runs; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_runs | 0.102 | 75.0% |
| br_predicate_h_runs | 0.047 | 34.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_runs | 3337ns | 3337ns | base |
| br_predicate_h_runs | 3957ns | 3957ns | +18.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_runs | 630ns | base | --- | [519, 736] | --- | --- | --- | --- |
| br_predicate_h_runs | 1354ns | +667.3ns (+106.0%) | [+541, +769]ns | [1150, 1358] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_runs | br_predicate_h_runs |
|---|---|---|
| 1 | 472ns | +117.7% |
| 2 | 728ns | +86.5% |
| 3 | 647ns | +109.0% |
| 4 | 565ns | +140.3% |
| 5 | 744ns | +70.8% |
| 6 | 612ns | +121.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_runs | -0.458 | moderate- |
| br_predicate_h_runs | -0.130 | ok |

**Consistency summary:**

- **br_predicate_h_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_runs | 4.3ns | 628.1ns | 0.7% |  |
| br_predicate_h_runs | 3.9ns | 1287.3ns | 0.3% |  |

## Distribution (algo ns)

```
br_branch_h_runs (n=6, range 472.1-735.9 ns)
    472.1 |########################################
    485.3 |
    498.5 |
    511.7 |
    524.9 |
    538.0 |
    551.2 |
    564.4 |########################################
    577.6 |
    590.8 |
    604.0 |########################################
    617.2 |
    630.4 |
    643.5 |########################################
    656.7 |
    669.9 |
    683.1 |
    696.3 |
    709.5 |
    722.7 |########################################
  (0 below, 1 above range)

br_predicate_h_runs (n=6, range 1027.9-1357.9 ns)
   1027.9 |#############
   1044.4 |
   1060.9 |
   1077.4 |
   1093.9 |
   1110.4 |
   1126.9 |
   1143.4 |
   1159.9 |
   1176.4 |
   1192.9 |
   1209.4 |
   1225.9 |
   1242.4 |
   1258.9 |#############
   1275.4 |
   1291.9 |
   1308.4 |
   1324.9 |
   1341.4 |########################################
  (0 below, 1 above range)

```

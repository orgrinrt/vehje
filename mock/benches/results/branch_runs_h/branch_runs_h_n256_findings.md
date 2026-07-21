# Branch strategies, heavy-arm, runs: correlated long runs (flip when b<24)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_runs**

## Key findings

- **Baseline (br_branch_h_runs) is the fastest** at 2205.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.09x (fastest 2205.4 ns, slowest 4609.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_runs | 4645ns | 4524ns | 4248ns | 4435ns | 5158ns | base |
| br_predicate_h_runs | 6954ns | 7034ns | 6307ns | 6795ns | 7517ns | +49.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_runs | 2280ns | 2057ns | 2569ns | base | 0.112 |
| br_predicate_h_runs | 4558ns | 4133ns | 4928ns | +99.88% | 0.056 |

## Performance model

- Peak throughput: **0.124 Gops/s** (br_branch_h_runs; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_runs | 0.116 | 93.3% |
| br_predicate_h_runs | 0.056 | 44.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_runs | 4645ns | 4645ns | base |
| br_predicate_h_runs | 6954ns | 6954ns | +49.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_runs | 2205ns | base | --- | [2067, 2569] | --- | --- | --- | --- |
| br_predicate_h_runs | 4610ns | +2231.7ns (+101.2%) | [+2014, +2588]ns | [4137, 4928] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_runs | br_predicate_h_runs |
|---|---|---|
| 1 | 2057ns | +101.3% |
| 2 | 2190ns | +88.7% |
| 3 | 2078ns | +106.7% |
| 4 | 2455ns | +100.7% |
| 5 | 2682ns | +83.7% |
| 6 | 2220ns | +121.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_runs | 0.166 | ok |
| br_predicate_h_runs | 0.559 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **br_predicate_h_runs**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_runs | 4.9ns | 2280.4ns | 0.2% |  |
| br_predicate_h_runs | 5.0ns | 4558.1ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_h_runs (n=6, range 2056.7-2568.8 ns)
   2056.7 |########################################
   2082.3 |
   2107.9 |
   2133.5 |
   2159.1 |
   2184.7 |####################
   2210.3 |####################
   2235.9 |
   2261.5 |
   2287.1 |
   2312.7 |
   2338.3 |
   2363.9 |
   2389.5 |
   2415.1 |
   2440.7 |####################
   2466.3 |
   2491.9 |
   2517.5 |
   2543.1 |
  (0 below, 1 above range)

br_predicate_h_runs (n=6, range 4133.3-4927.5 ns)
   4133.3 |########################################
   4173.0 |
   4212.7 |
   4252.4 |
   4292.1 |####################
   4331.9 |
   4371.6 |
   4411.3 |
   4451.0 |
   4490.7 |
   4530.4 |
   4570.1 |
   4609.8 |
   4649.5 |
   4689.2 |
   4728.9 |
   4768.7 |
   4808.4 |
   4848.1 |
   4887.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **br_predicate_h_runs**: autocorrelation=0.56 (measurement drift or warm-up artifact)

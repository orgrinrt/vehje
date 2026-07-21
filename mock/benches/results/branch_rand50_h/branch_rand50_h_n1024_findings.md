# Branch strategies, heavy-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_rand50**

## Key findings

- **Baseline (br_branch_h_rand50) is the fastest** at 8506.9 ns median
- 1 variant significantly slower than baseline
- Spread: 2.04x (fastest 8506.9 ns, slowest 17318.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 10975ns | 10694ns | 10582ns | 10675ns | 11622ns | base |
| br_predicate_h_rand50 | 19820ns | 20027ns | 18795ns | 19748ns | 20441ns | +80.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_rand50 | 8717ns | 8420ns | 9202ns | base | 0.117 |
| br_predicate_h_rand50 | 17343ns | 16588ns | 17946ns | +98.96% | 0.059 |

## Performance model

- Peak throughput: **0.122 Gops/s** (br_branch_h_rand50; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_rand50 | 0.120 | 99.0% |
| br_predicate_h_rand50 | 0.059 | 48.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_rand50 | 10975ns | 10975ns | base |
| br_predicate_h_rand50 | 19820ns | 19820ns | +80.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 8507ns | base | --- | [8442, 9202] | --- | --- | --- | --- |
| br_predicate_h_rand50 | 17319ns | +8764.2ns (+103.0%) | [+7686, +9428]ns | [16763, 17946] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_rand50 | br_predicate_h_rand50 |
|---|---|---|
| 1 | 9649ns | +75.5% |
| 2 | 8754ns | +101.2% |
| 3 | 8463ns | +107.4% |
| 4 | 8420ns | +102.9% |
| 5 | 8507ns | +95.0% |
| 6 | 8507ns | +114.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_rand50 | 0.186 | ok |
| br_predicate_h_rand50 | -0.346 | moderate- |

**Consistency summary:**

- **br_predicate_h_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_rand50 | 4.1ns | 8716.8ns | 0.0% |  |
| br_predicate_h_rand50 | 4.7ns | 17342.7ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_rand50 (n=6, range 8420.4-9201.5 ns)
   8420.4 |####################
   8459.5 |####################
   8498.5 |########################################
   8537.6 |
   8576.6 |
   8615.7 |
   8654.7 |
   8693.8 |
   8732.8 |####################
   8771.9 |
   8811.0 |
   8850.0 |
   8889.1 |
   8928.1 |
   8967.2 |
   9006.2 |
   9045.3 |
   9084.3 |
   9123.4 |
   9162.4 |
  (0 below, 1 above range)

br_predicate_h_rand50 (n=6, range 16588.3-17946.2 ns)
  16588.3 |########################################
  16656.2 |
  16724.1 |
  16792.0 |
  16859.9 |
  16927.8 |########################################
  16995.7 |
  17063.6 |########################################
  17131.5 |
  17199.4 |
  17267.2 |
  17335.1 |
  17403.0 |
  17470.9 |
  17538.8 |########################################
  17606.7 |########################################
  17674.6 |
  17742.5 |
  17810.4 |
  17878.3 |
  (0 below, 1 above range)

```

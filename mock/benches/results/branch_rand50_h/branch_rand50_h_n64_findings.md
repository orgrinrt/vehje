# Branch strategies, heavy-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_rand50**

## Key findings

- **Baseline (br_branch_h_rand50) is the fastest** at 629.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.97x (fastest 629.5 ns, slowest 1239.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 3138ns | 3227ns | 2799ns | 3170ns | 3261ns | base |
| br_predicate_h_rand50 | 3777ns | 3832ns | 3162ns | 3831ns | 4002ns | +20.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_rand50 | 621ns | 549ns | 658ns | base | 0.103 |
| br_predicate_h_rand50 | 1222ns | 1025ns | 1295ns | +96.89% | 0.052 |

## Performance model

- Peak throughput: **0.117 Gops/s** (br_branch_h_rand50; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_rand50 | 0.102 | 87.2% |
| br_predicate_h_rand50 | 0.052 | 44.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_rand50 | 3138ns | 3138ns | base |
| br_predicate_h_rand50 | 3777ns | 3777ns | +20.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_rand50 | 630ns | base | --- | [575, 658] | --- | --- | --- | --- |
| br_predicate_h_rand50 | 1239ns | +610.0ns (+96.9%) | [+528, +666]ns | [1132, 1295] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_rand50 | br_predicate_h_rand50 |
|---|---|---|
| 1 | 549ns | +86.7% |
| 2 | 658ns | +88.1% |
| 3 | 601ns | +106.3% |
| 4 | 605ns | +104.7% |
| 5 | 657ns | +89.3% |
| 6 | 654ns | +106.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_rand50 | -0.261 | moderate- |
| br_predicate_h_rand50 | 0.006 | ok |

**Consistency summary:**

- **br_predicate_h_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_rand50 | 4.4ns | 620.7ns | 0.7% |  |
| br_predicate_h_rand50 | 3.9ns | 1222.1ns | 0.3% |  |

## Distribution (algo ns)

```
br_branch_h_rand50 (n=6, range 549.2-657.5 ns)
    549.2 |####################
    554.6 |
    560.0 |
    565.4 |
    570.9 |
    576.3 |
    581.7 |
    587.1 |
    592.5 |
    597.9 |####################
    603.4 |####################
    608.8 |
    614.2 |
    619.6 |
    625.0 |
    630.4 |
    635.8 |
    641.3 |
    646.7 |
    652.1 |########################################
  (0 below, 1 above range)

br_predicate_h_rand50 (n=6, range 1025.4-1295.2 ns)
   1025.4 |#############
   1038.9 |
   1052.4 |
   1065.9 |
   1079.4 |
   1092.9 |
   1106.3 |
   1119.8 |
   1133.3 |
   1146.8 |
   1160.3 |
   1173.8 |
   1187.3 |
   1200.8 |
   1214.3 |
   1227.8 |########################################
   1241.2 |#############
   1254.7 |
   1268.2 |
   1281.7 |
  (0 below, 1 above range)

```

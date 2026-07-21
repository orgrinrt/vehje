# Branch strategies, heavy-arm, alt: strict alternation (i&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_alt**

## Key findings

- **Baseline (br_branch_h_alt) is the fastest** at 8876.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.92x (fastest 8876.8 ns, slowest 17008.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_alt | 11148ns | 11177ns | 10504ns | 10988ns | 11710ns | base |
| br_predicate_h_alt | 21399ns | 19237ns | 18626ns | 19160ns | 26144ns | +91.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_alt | 8849ns | 8328ns | 9302ns | base | 0.116 |
| br_predicate_h_alt | 18972ns | 16454ns | 23289ns | +114.40% | 0.054 |

## Performance model

- Peak throughput: **0.123 Gops/s** (br_branch_h_alt; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_alt | 0.115 | 93.8% |
| br_predicate_h_alt | 0.060 | 49.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_alt | 11148ns | 11148ns | base |
| br_predicate_h_alt | 21399ns | 21399ns | +91.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_alt | 8877ns | base | --- | [8368, 9302] | --- | --- | --- | --- |
| br_predicate_h_alt | 17009ns | +8522.3ns (+96.0%) | [+7861, +13987]ns | [16619, 23289] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_alt | br_predicate_h_alt |
|---|---|---|
| 1 | 9282ns | +83.4% |
| 2 | 9281ns | +109.7% |
| 3 | 9323ns | +190.9% |
| 4 | 8408ns | +102.1% |
| 5 | 8328ns | +101.6% |
| 6 | 8473ns | +94.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_alt | 0.504 | HIGH+ (drift/warm-up) |
| br_predicate_h_alt | -0.038 | ok |

**Consistency summary:**

- **br_predicate_h_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_alt | 4.1ns | 8849.0ns | 0.0% |  |
| br_predicate_h_alt | 4.8ns | 18972.2ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_alt (n=6, range 8327.9-9302.3 ns)
   8327.9 |####################
   8376.6 |####################
   8425.3 |####################
   8474.1 |
   8522.8 |
   8571.5 |
   8620.2 |
   8668.9 |
   8717.7 |
   8766.4 |
   8815.1 |
   8863.8 |
   8912.5 |
   8961.3 |
   9010.0 |
   9058.7 |
   9107.4 |
   9156.1 |
   9204.9 |
   9253.6 |########################################
  (0 below, 1 above range)

br_predicate_h_alt (n=6, range 16453.8-23288.5 ns)
  16453.8 |########################################
  16795.5 |########################################
  17137.3 |
  17479.0 |
  17820.8 |
  18162.5 |
  18504.2 |
  18846.0 |
  19187.7 |####################
  19529.4 |
  19871.2 |
  20212.9 |
  20554.7 |
  20896.4 |
  21238.1 |
  21579.9 |
  21921.6 |
  22263.3 |
  22605.1 |
  22946.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_branch_h_alt**: autocorrelation=0.50 (measurement drift or warm-up artifact)

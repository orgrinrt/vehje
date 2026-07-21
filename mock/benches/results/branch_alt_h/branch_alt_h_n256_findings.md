# Branch strategies, heavy-arm, alt: strict alternation (i&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_alt**

## Key findings

- **Baseline (br_branch_h_alt) is the fastest** at 2253.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.97x (fastest 2253.3 ns, slowest 4438.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_alt | 4654ns | 4582ns | 4261ns | 4496ns | 5089ns | base |
| br_predicate_h_alt | 6822ns | 6783ns | 6294ns | 6622ns | 7385ns | +46.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_alt | 2284ns | 2095ns | 2492ns | base | 0.112 |
| br_predicate_h_alt | 4472ns | 4121ns | 4854ns | +95.81% | 0.057 |

## Performance model

- Peak throughput: **0.122 Gops/s** (br_branch_h_alt; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_alt | 0.114 | 93.0% |
| br_predicate_h_alt | 0.058 | 47.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_alt | 4654ns | 4654ns | base |
| br_predicate_h_alt | 6822ns | 6822ns | +46.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_alt | 2253ns | base | --- | [2106, 2492] | --- | --- | --- | --- |
| br_predicate_h_alt | 4439ns | +2134.4ns (+94.7%) | [+1990, +2440]ns | [4123, 4854] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_alt | br_predicate_h_alt |
|---|---|---|
| 1 | 2095ns | +96.7% |
| 2 | 2171ns | +90.0% |
| 3 | 2117ns | +102.5% |
| 4 | 2336ns | +96.5% |
| 5 | 2491ns | +105.4% |
| 6 | 2494ns | +84.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_alt | 0.517 | HIGH+ (drift/warm-up) |
| br_predicate_h_alt | 0.440 | moderate+ |

**Consistency summary:**

- **br_predicate_h_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_alt | 4.8ns | 2283.9ns | 0.2% |  |
| br_predicate_h_alt | 4.2ns | 4472.1ns | 0.1% |  |

## Distribution (algo ns)

```
br_branch_h_alt (n=6, range 2094.6-2492.5 ns)
   2094.6 |########################################
   2114.5 |########################################
   2134.4 |
   2154.3 |########################################
   2174.2 |
   2194.1 |
   2214.0 |
   2233.9 |
   2253.8 |
   2273.7 |
   2293.6 |
   2313.4 |
   2333.3 |########################################
   2353.2 |
   2373.1 |
   2393.0 |
   2412.9 |
   2432.8 |
   2452.7 |
   2472.6 |########################################
  (0 below, 1 above range)

br_predicate_h_alt (n=6, range 4120.8-4854.1 ns)
   4120.8 |########################################
   4157.5 |
   4194.1 |
   4230.8 |
   4267.5 |####################
   4304.1 |
   4340.8 |
   4377.5 |
   4414.1 |
   4450.8 |
   4487.5 |
   4524.1 |
   4560.8 |########################################
   4597.5 |
   4634.1 |
   4670.8 |
   4707.5 |
   4744.1 |
   4780.8 |
   4817.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_branch_h_alt**: autocorrelation=0.52 (measurement drift or warm-up artifact)
